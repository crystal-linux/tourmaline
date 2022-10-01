use config::Config;
use error::{AppError, AppResult};
use scripting::{
    loader::{HookType, ScriptLoader},
    record::RecordValue,
    script::{NuScript, Script, ScriptArgs},
};
use tasks::*;

pub mod config;
pub mod error;
pub(crate) mod scripting;
pub mod tasks;
pub(crate) mod utils;
pub use utils::generate_script_files;

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

pub struct TaskExecutor {
    config: Option<Config>,
    loader: ScriptLoader,
}

impl TaskExecutor {
    /// Creates a new task executor with a given config
    pub fn with_config(config: Config) -> Self {
        Self {
            config: Some(config),
            loader: ScriptLoader::new(),
        }
    }

    tasks!(
        setup_users => SetupUsersScript,
        configure_network => ConfigureNetworkScript,
        configure_unakite => ConfigureUnakiteScript,
        create_partitions => CreatePartitionsScript,
        install_base => InstallBaseScript,
        install_bootloader => InstallBootloaderScript,
        install_desktop => InstallDesktopScript,
        install_extra_packages => InstallExtraPackagesScript,
        install_flatpak => InstallFlatpakScript,
        install_kernels => InstallKernelsScript,
        install_timeshift => InstallTimeshiftScript,
        install_zramd => InstallZRamDScript,
        setup_root_user => SetupRootUserScript,
        configure_locale => ConfigureLocaleScript
    );

    /// Installs the system from the given system configuration
    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn install_from_config(&self) -> AppResult<()> {
        let config = self
            .config
            .as_ref()
            .ok_or_else(|| AppError::MissingConfig)?;
        self.create_partitions(&config.partitions).await?;
        self.install_base(&()).await?;
        self.install_kernels(&config.kernels).await?;
        self.install_bootloader(&config.bootloader).await?;
        self.configure_locale(&config.locale).await?;
        self.configure_network(&config.network).await?;

        if config.enable_zramd {
            self.install_zramd(&()).await?;
        }
        if config.enable_timeshift {
            self.install_timeshift(&()).await?;
        }
        if config.enable_flatpak {
            self.install_flatpak(&()).await?;
        }
        self.setup_users(&config.users).await?;
        self.setup_root_user(&config.root_user).await?;
        self.install_desktop(&config.desktop).await?;
        self.install_extra_packages(&config.extra_packages).await?;

        if let Some(unakite) = &config.unakite {
            self.configure_unakite(unakite).await?;
        }

        Ok(())
    }

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
            script.set_global_var("TRM_CONFIG", RecordValue::from(Config::empty().get_args()))
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
