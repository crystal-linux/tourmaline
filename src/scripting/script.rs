use core::fmt;
use std::{collections::HashMap, marker::PhantomData, path::PathBuf};

use embed_nu::{
    rusty_value::RustyValue, Argument, CommandGroupConfig, ContextBuilder, IntoArgument, IntoValue,
    RawValue, Value,
};
use tokio::fs;

use crate::error::{AppError, AppResult};

/// A trait implemented for a given nu script type to
/// associate arguments
pub trait Script {
    type Args: ScriptArgs + fmt::Debug + Clone;

    /// Returns the (expected) name of the script file
    /// This function is used by the loader to load the associated file
    /// The name needs to include the file extension
    fn get_name() -> &'static str;

    /// Returns the name of the script file that get's executed before
    /// the actual script. This has to be the full file name including the extension.
    fn get_pre_hook() -> &'static str;

    /// Returns the name of the script file that get's executed after
    /// the actual script. This has to be the full file name including the extension.
    fn get_post_hook() -> &'static str;
}

/// Script arguments that can be collected in a Vec to
/// be passed to the script
pub trait ScriptArgs: RustyValue {
    fn get_args(self) -> Vec<Argument>;
}

impl<T: RustyValue> ScriptArgs for T {
    fn get_args(self) -> Vec<Argument> {
        match self.into_value() {
            Value::List { vals, .. } => vals
                .into_iter()
                .map(|v| RawValue(v).into_argument())
                .collect(),
            val => vec![RawValue(val).into_argument()],
        }
    }
}

/// A nu script instance that can be executed
pub struct NuScript<S: Script> {
    path: PathBuf,
    vars: HashMap<String, Value>,
    __phantom: PhantomData<S>,
}

impl<S: Script> NuScript<S> {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            vars: HashMap::new(),
            __phantom: PhantomData,
        }
    }

    /// Adds a global variable
    pub fn set_global_var<S1: ToString, V: IntoValue>(&mut self, key: S1, value: V) -> &mut Self {
        self.vars.insert(key.to_string(), value.into_value());

        self
    }

    /// Executes the script with the given args
    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn execute(&self, args: S::Args) -> AppResult<()> {
        let mut ctx = ContextBuilder::default()
            .with_command_groups(CommandGroupConfig::default().all_groups(true))
            .add_script(self.read_file().await?)?
            .build()?;

        if ctx.has_fn("main") {
            ctx.call_fn("main", args.get_args())?;
            Ok(())
        } else {
            Err(AppError::MissingMain(self.path.clone()))
        }
    }

    async fn read_file(&self) -> AppResult<String> {
        fs::read_to_string(&self.path).await.map_err(AppError::from)
    }
}

/// Defines a script
/// This macro doesn't accept a file extension for the script name
/// as it is reused for the hook name
#[macro_export]
macro_rules! script {
    ($script:ident {
        file = $name:literal
        args = $argtype:ident
    }) => {
        pub struct $script;

        impl $crate::scripting::script::Script for $script {
            type Args = $argtype;

            fn get_name() -> &'static str {
                concat!($name, ".nu")
            }

            fn get_pre_hook() -> &'static str {
                concat!($name, ".pre.nu")
            }

            fn get_post_hook() -> &'static str {
                concat!($name, ".post.nu")
            }
        }
    };
}
