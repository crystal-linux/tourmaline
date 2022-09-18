#![allow(unused)]
use std::{
    collections::HashMap,
    fs, mem,
    path::{Path, PathBuf},
};

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
    script_path: String,
    args: Vec<String>,
    global_vars: HashMap<String, VarValue>,
}

impl NuExecutor {
    pub fn new<P: AsRef<Path>>(script_path: P) -> Self {
        Self {
            script_path: script_path.as_ref().to_string_lossy().into_owned(),
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
    pub fn execute(&mut self) -> AppResult<()> {
        let mut engine_state = nu_command::create_default_context();
        let mut stack = nu_protocol::engine::Stack::new();
        let input = PipelineData::new(Span::new(0, 0));
        let init_cwd = nu_cli::get_init_cwd();
        nu_engine::convert_env_values(&mut engine_state, &mut stack);

        let vars = mem::take(&mut self.global_vars);
        let vars = vars
            .into_iter()
            .map(|(k, v)| (k, map_var_to_value(v)))
            .collect::<HashMap<_, _>>();

        add_variables_to_state(vars, &mut engine_state, &mut stack);
        let block = read_script_file(&self.script_path, &mut engine_state)?;

        nu_engine::eval_block(
            &engine_state,
            &mut stack,
            &block,
            PipelineData::new(Span::new(0, 0)),
            false,
            false,
        )
        .into_diagnostic()?;

        // TODO: Create the AST for the call here instead of parsing it from a string
        let args = format!("main {}", self.args.join(" "));
        nu_cli::eval_source(
            &mut engine_state,
            &mut stack,
            args.as_bytes(),
            "<commandline>",
            input,
        );

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
    for (name, value) in vars {
        let var_id = working_set.add_variable(
            name.as_bytes().to_vec(),
            Span::new(0, 0),
            nu_protocol::Type::String,
        );
        stack.add_var(var_id, value);
    }
    state.merge_delta(working_set.render());
}

/// Reads the nu script file and
/// returns its root block
fn read_script_file(path: &str, engine_state: &mut EngineState) -> AppResult<Block> {
    let mut working_set = StateWorkingSet::new(engine_state);
    let script_contents = fs::read(&path).into_diagnostic()?;
    // parse the source file
    let (block, err) = nu_parser::parse(&mut working_set, Some(path), &script_contents, false, &[]);

    if let Some(err) = err {
        return Err(AppError::from(err));
    }
    // check if a main method exists in the block
    if !working_set.find_decl(b"main", &Type::Block).is_some() {
        return Err(AppError::MissingMain(PathBuf::from(path)));
    }
    engine_state.merge_delta(working_set.render());

    Ok(block)
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
