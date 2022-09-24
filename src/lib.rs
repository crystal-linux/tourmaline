use config::Config;
use error::AppResult;
use scripting::{
    loader::{HookType, ScriptLoader},
    script::{NuScript, Script, ScriptArgs},
};
use tasks::*;

pub mod config;
pub mod error;
pub(crate) mod scripting;
pub mod tasks;
pub(crate) mod utils;

pub struct TaskExecutor {
    config: Option<Config>,
    loader: ScriptLoader,
}

macro_rules! tasks {
    ($($function:ident => $script:ident),+) => {
       $(
            #[tracing::instrument(level = "trace", skip(self))]
            pub async fn $function(&self, cfg: &<$script as crate::scripting::script::Script>::Args) -> AppResult<()> {
                self.execute_task::<$script>(cfg).await
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

    async fn execute_task<S: Script>(&self, args: &S::Args) -> AppResult<()> {
        if let Some(pre_hook) = self.loader.load_hook::<S>(HookType::Pre) {
            self.execute(pre_hook, args).await?;
        }
        let script = self.loader.load::<S>()?;
        self.execute(script, args).await?;

        if let Some(post_hook) = self.loader.load_hook::<S>(HookType::Post) {
            self.execute(post_hook, args).await?;
        }

        Ok(())
    }

    #[inline]
    async fn execute<S: Script>(&self, mut script: NuScript<S>, args: &S::Args) -> AppResult<()> {
        if let Some(cfg) = self.config.as_ref() {
            script.set_global_var("TRM_CONFIG", cfg.get_args())
        } else {
            &mut script
        }
        .set_global_var("TRM_VERSION", env!("CARGO_PKG_VERSION"))
        .execute(args)
        .await
    }
}

impl Default for TaskExecutor {
    fn default() -> Self {
        Self {
            loader: ScriptLoader::new(),
            config: None,
        }
    }
}
