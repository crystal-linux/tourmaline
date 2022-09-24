#![allow(unused)]
use std::{
    collections::HashMap,
    mem,
    path::{Path, PathBuf},
};

use nu_cli::gather_parent_env_vars;
use tokio::fs;

use miette::IntoDiagnostic;
use nu_protocol::{
    ast::{Argument, Block, Call, Expr, Expression, Pipeline},
    engine::{EngineState, Stack, StateDelta, StateWorkingSet},
    DeclId, PipelineData, Signature, Span, Type, Value,
};

use crate::error::{AppError, AppResult};

use super::record::RecordValue;

/// An executor for nu scripts
pub struct NuExecutor {
    script_path: PathBuf,
    args: Vec<RecordValue>,
    global_vars: HashMap<String, RecordValue>,
}

impl NuExecutor {
    pub fn new<P: AsRef<Path>>(script_path: P) -> Self {
        Self {
            script_path: script_path.as_ref().to_owned(),
            args: Vec::new(),
            global_vars: HashMap::new(),
        }
    }

    pub fn add_arg<A: Into<RecordValue>>(&mut self, arg: A) -> &mut Self {
        self.args.push(arg.into());

        self
    }

    pub fn add_args<A: Into<RecordValue>, I: IntoIterator<Item = A>>(
        &mut self,
        args: I,
    ) -> &mut Self {
        let mut args = args.into_iter().map(|a| a.into()).collect::<Vec<_>>();
        self.args.append(&mut args);

        self
    }

    /// Adds a global variable to the executor which can
    /// be accessed from within the script
    pub fn add_global_var<S: ToString, V: Into<RecordValue>>(
        &mut self,
        name: S,
        value: V,
    ) -> &mut Self {
        self.global_vars.insert(name.to_string(), value.into());

        self
    }

    /// Adds multiple global variables
    pub fn add_global_vars<S: ToString, R: Into<RecordValue>, I: IntoIterator<Item = (S, R)>>(
        &mut self,
        vars: I,
    ) -> &mut Self {
        self.global_vars
            .extend(&mut vars.into_iter().map(|(k, v)| (k.to_string(), v.into())));

        self
    }

    /// Executes the given script file in a clean nu context.
    #[tracing::instrument(level = "trace", skip_all)]
    pub async fn execute(&mut self) -> AppResult<()> {
        let mut engine_state = nu_command::create_default_context();
        let mut stack = nu_protocol::engine::Stack::new();
        let init_cwd = nu_cli::get_init_cwd();
        gather_parent_env_vars(&mut engine_state, &init_cwd);
        nu_engine::convert_env_values(&mut engine_state, &stack);

        let vars = mem::take(&mut self.global_vars);
        let vars = vars
            .into_iter()
            .map(|(k, v)| (k, v.into_protocol_value()))
            .collect::<HashMap<_, _>>();

        add_variables_to_state(vars, &mut engine_state, &mut stack);
        let (block, main_id) = read_script_file(&self.script_path, &mut engine_state).await?;

        // put everything the script defines into scope
        nu_engine::eval_block(
            &engine_state,
            &mut stack,
            &block,
            empty_pipeline(),
            false,
            false,
        )
        .into_diagnostic()?;

        let args = mem::take(&mut self.args);

        // block in a different thread to be able to execute scripts in parallel
        tokio::task::spawn_blocking(move || {
            // create a call to the main method wit the given arguments and execute it
            let call_block = create_call(main_id, args);

            nu_engine::eval_block(
                &engine_state,
                &mut stack,
                &call_block,
                empty_pipeline(),
                false,
                false,
            )?;

            AppResult::Ok(())
        })
        .await
        .unwrap()?;

        Ok(())
    }
}

/// Adds variables to the nu engine state
/// Note: Calling this function multiple times will override other variables
#[tracing::instrument(level = "trace", skip(state, stack))]
fn add_variables_to_state(
    vars: HashMap<String, Value>,
    state: &mut EngineState,
    stack: &mut Stack,
) {
    let state2 = nu_command::create_default_context();
    let mut working_set = StateWorkingSet::new(&state2);
    vars.into_iter()
        .map(|(name, value)| {
            (
                working_set.add_variable(
                    name.as_bytes().to_vec(),
                    Span::new(0, 0),
                    nu_protocol::Type::String,
                ),
                value,
            )
        })
        .for_each(|(var_id, value)| stack.add_var(var_id, value));
    state.merge_delta(working_set.render());
}

/// Reads the nu script file and
/// returns its root block
#[tracing::instrument(level = "trace", skip(engine_state))]
async fn read_script_file(
    path: &Path,
    engine_state: &mut EngineState,
) -> AppResult<(Block, DeclId)> {
    let script_contents = fs::read(&path).await.into_diagnostic()?;
    let string_path = path.to_string_lossy().into_owned();
    // parse the source file
    let (block, working_set) = parse_nu(engine_state, &script_contents, Some(&string_path))?;
    // check if a main method exists in the block
    if let Some(decl_id) = working_set.find_decl(b"main", &Type::Block) {
        let delta = working_set.render();
        engine_state.merge_delta(delta);

        Ok((block, decl_id))
    } else {
        Err(AppError::MissingMain(PathBuf::from(path)))
    }
}

/// Parses a nu script
#[tracing::instrument(level = "trace", skip_all)]
fn parse_nu<'a>(
    engine_state: &'a mut EngineState,
    script: &[u8],
    script_path: Option<&str>,
) -> AppResult<(Block, StateWorkingSet<'a>)> {
    let mut working_set = StateWorkingSet::new(engine_state);
    let (block, err) = nu_parser::parse(&mut working_set, script_path, script, false, &[]);

    if let Some(err) = err {
        Err(AppError::from(err))
    } else {
        Ok((block, working_set))
    }
}

/// Creates a call nu expression with the given main block declaration ID
/// and arguments in the form of record values
#[tracing::instrument(level = "trace")]
fn create_call(decl_id: DeclId, args: Vec<RecordValue>) -> Block {
    let args = args
        .into_iter()
        .map(|a| Argument::Positional(a.into_expression()))
        .collect();
    let call = Call {
        decl_id,
        head: Span::new(0, 0),
        arguments: args,
        redirect_stdout: true,
        redirect_stderr: false,
    };
    let pipeline = Pipeline {
        expressions: vec![Expression {
            expr: Expr::Call(Box::new(call)),
            span: Span::new(0, 0),
            ty: Type::Any,
            custom_completion: None,
        }],
    };
    Block {
        signature: Box::new(Signature::build("Call to main")),
        pipelines: vec![pipeline],
        captures: Vec::new(),
        redirect_env: false,
        span: None,
    }
}

fn empty_pipeline() -> PipelineData {
    PipelineData::new(Span::new(0, 0))
}
