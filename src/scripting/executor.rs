#![allow(unused)]
use nu_protocol::{PipelineData, Span};

/// An executor for nu scripts
pub struct NuExecutor {
    script_path: String,
    args: Vec<String>,
}

impl NuExecutor {
    pub fn new(script_path: String) -> Self {
        Self {
            script_path,
            args: Vec::new(),
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

    pub fn execute(&mut self) {
        let mut engine_state = nu_command::create_default_context();
        let mut stack = nu_protocol::engine::Stack::new();
        let input = PipelineData::new(Span::new(0, 0));
        let init_cwd = nu_cli::get_init_cwd();
        nu_cli::gather_parent_env_vars(&mut engine_state, &init_cwd);

        nu_cli::evaluate_file(
            self.script_path.clone(),
            &self.args,
            &mut engine_state,
            &mut stack,
            input,
            false,
        )
        .unwrap();
    }
}
