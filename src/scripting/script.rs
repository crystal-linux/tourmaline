use core::fmt;
use std::{marker::PhantomData, path::PathBuf};

use serde::Serialize;

use crate::error::AppResult;

use super::{
    executor::{NuExecutor, VarValue},
    record::RecordValue,
    record_serializer::RecordSerializer,
};

/// A trait implemented for a given nu script type to
/// associate arguments
pub trait Script {
    type Args: ScriptArgs + fmt::Debug;

    /// Returns the (expected) name of the script file
    /// This function is used by the loader to load the associated file
    /// The name needs to include the file extension
    fn get_name() -> &'static str;
}

/// Script arguments that can be collected in a Vec to
/// be passed to the script
pub trait ScriptArgs {
    fn get_args(self) -> Vec<RecordValue>;
}

impl<T: Serialize> ScriptArgs for T {
    fn get_args(self) -> Vec<RecordValue> {
        let mut serializer = RecordSerializer::default();
        let val = self.serialize(&mut serializer).unwrap();
        match val {
            RecordValue::List(entries) => entries,
            val => vec![val],
        }
    }
}

/// A nu script instance that can be executed
pub struct NuScript<S: Script> {
    path: PathBuf,
    __phantom: PhantomData<S>,
}

impl<S: Script> NuScript<S> {
    pub(crate) fn new(path: PathBuf) -> Self {
        Self {
            path,
            __phantom: PhantomData,
        }
    }

    /// Executes the script with the given args
    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn execute(&self, args: S::Args) -> AppResult<()> {
        NuExecutor::new(&self.path)
            .add_args(args.get_args())
            .add_global_var("BY_TOURMALINE", VarValue::string("Hello from Tourmaline!"))
            .add_global_var(
                "ANOTHER_ONE",
                VarValue::string("This variable was provided by tourmaline"),
            )
            .execute()
            .await
    }
}

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
                $name
            }
        }
    };
}
