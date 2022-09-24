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
    ast::{Block, Call},
    engine::{EngineState, Stack, StateDelta, StateWorkingSet},
    PipelineData, Span, Type, Value,
};

use crate::error::{AppError, AppResult};

#[derive(Clone, Debug)]
pub enum VarValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl VarValue {
    pub fn string<S: ToString>(s: S) -> Self {
        Self::String(s.to_string())
    }
}

/// An executor for nu scripts
pub struct NuExecutor {
    script_path: PathBuf,
    args: Vec<String>,
    global_vars: HashMap<String, VarValue>,
}

impl NuExecutor {
    pub fn new<P: AsRef<Path>>(script_path: P) -> Self {
        Self {
            script_path: script_path.as_ref().to_owned(),
            args: Vec::new(),
            global_vars: HashMap::new(),
        }
    }

    pub fn add_arg<S: ToString>(&mut self, arg: S) -> &mut Self {
        self.args.push(arg.to_string());

        self
    }

    pub fn add_args<S: ToString, I: IntoIterator<Item = S>>(&mut self, args: I) -> &mut Self {
        let mut args = args.into_iter().map(|a| a.to_string()).collect::<Vec<_>>();
        self.args.append(&mut args);

        self
    }

    /// Adds a global variable to the executor which can
    /// be accessed from within the script
    pub fn add_global_var<S: ToString>(&mut self, name: S, value: VarValue) -> &mut Self {
        self.global_vars.insert(name.to_string(), value);

        self
    }

    /// Executes the given script file in a clean nu context.
    pub async fn execute(&mut self) -> AppResult<()> {
        let mut engine_state = nu_command::create_default_context();
        let mut stack = nu_protocol::engine::Stack::new();
        let init_cwd = nu_cli::get_init_cwd();
        gather_parent_env_vars(&mut engine_state, &init_cwd);
        nu_engine::convert_env_values(&mut engine_state, &mut stack);

        let vars = mem::take(&mut self.global_vars);
        let vars = vars
            .into_iter()
            .map(|(k, v)| (k, map_var_to_value(v)))
            .collect::<HashMap<_, _>>();

        add_variables_to_state(vars, &mut engine_state, &mut stack);
        let block = read_script_file(&self.script_path, &mut engine_state).await?;

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
            // TODO: Create the AST for the call here instead of parsing it from a string
            let args = format!("main {}", args.join(" "));
            let (call_block, working_set) = parse_nu(&mut engine_state, args.as_bytes(), None)?;
            let delta = working_set.render();
            engine_state.merge_delta(delta);

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
async fn read_script_file(path: &Path, engine_state: &mut EngineState) -> AppResult<Block> {
    let script_contents = fs::read(&path).await.into_diagnostic()?;
    let string_path = path.to_string_lossy().into_owned();
    // parse the source file
    let (block, working_set) = parse_nu(engine_state, &script_contents, Some(&string_path))?;
    // check if a main method exists in the block
    if !working_set.find_decl(b"main", &Type::Block).is_some() {
        return Err(AppError::MissingMain(PathBuf::from(path)));
    }
    let delta = working_set.render();
    engine_state.merge_delta(delta);

    Ok(block)
}

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

fn empty_pipeline() -> PipelineData {
    PipelineData::new(Span::new(0, 0))
}

fn map_var_to_value(var: VarValue) -> Value {
    let span = Span::new(0, 0);

    match var {
        VarValue::String(val) => Value::String { val, span },
        VarValue::Integer(val) => Value::Int { val, span },
        VarValue::Float(val) => Value::Float { val, span },
        VarValue::Boolean(val) => Value::Bool { val, span },
    }
}
