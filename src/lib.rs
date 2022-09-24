use error::AppResult;
use scripting::{loader::ScriptLoader, script::Script};
use tasks::*;

pub mod error;
pub(crate) mod scripting;
pub mod tasks;
pub(crate) mod utils;

pub struct TaskExecutor {
    loader: ScriptLoader,
}

macro_rules! tasks {
    ($($function:ident => $script:ident),+) => {
       $(
            #[tracing::instrument(level = "trace", skip(self))]
            pub async fn $function(&self, cfg: <$script as crate::scripting::script::Script>::Args) -> AppResult<()> {
                self.execute::<$script>(cfg).await
            }
        )+
    }
}

impl TaskExecutor {
    tasks!(
        setup_users => SetupUsersScript,
        configure_network => ConfigureNetworkScript,
        create_partitions => CreatePartitionsScript,
        install_base => InstallBaseScript,
        install_bootloader => InstallBootloaderScript,
        install_desktop => InstallDesktopScript,
        configure_local => ConfigureLocaleScript
    );

    #[inline]
    async fn execute<S: Script>(&self, args: S::Args) -> AppResult<()> {
        self.loader.load::<S>()?.execute(args).await
    }
}

impl Default for TaskExecutor {
    fn default() -> Self {
        Self {
            loader: ScriptLoader::new(),
        }
    }
}
