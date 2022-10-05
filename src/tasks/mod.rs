mod configure_locale;
mod configure_network;
mod configure_unakite;
mod create_partitions;
mod install_base;
mod install_bootloader;
mod install_desktop;
mod install_extra_packages;
mod install_flatpak;
mod install_kernels;
mod install_timeshift;
mod install_zramd;
mod setup_root_user;
mod setup_users;

use std::path::{Path, PathBuf};

pub use configure_locale::*;
pub use configure_network::*;
pub use configure_unakite::*;
pub use create_partitions::*;
pub use install_base::*;
pub use install_bootloader::*;
pub use install_desktop::*;
pub use install_extra_packages::*;
pub use install_flatpak::*;
pub use install_kernels::*;
pub use install_timeshift::*;
pub use install_zramd::*;
pub use setup_root_user::*;
pub use setup_users::*;

use crate::scripting::script::Script;

pub struct TaskFiles {
    script: String,
    pre_hook: String,
    post_hook: String,
}

impl TaskFiles {
    pub fn script_path(&self, base: &Path) -> PathBuf {
        base.join("scripts").join(&self.script)
    }

    pub fn pre_hook_path(&self, base: &Path) -> PathBuf {
        base.join("hooks").join(&self.pre_hook)
    }

    pub fn post_hook_path(&self, base: &Path) -> PathBuf {
        base.join("hooks").join(&self.post_hook)
    }
}

macro_rules! __all_tasks {
    ($($task:ident),+) => {
        {
            vec![$(
                TaskFiles {
                    script: $task::get_name().into(),
                    pre_hook: $task::get_pre_hook().into(),
                    post_hook: $task::get_post_hook().into(),
                },
            )+]
        }
    };
}

pub fn all_tasks() -> Vec<TaskFiles> {
    __all_tasks!(
        ConfigureLocaleScript,
        ConfigureNetworkScript,
        ConfigureUnakiteScript,
        CreatePartitionsScript,
        InstallBaseScript,
        InstallBootloaderScript,
        InstallDesktopScript,
        InstallExtraPackagesScript,
        InstallFlatpakScript,
        InstallKernelsScript,
        InstallTimeshiftScript,
        InstallZRamDScript,
        SetupRootUserScript,
        SetupUsersScript
    )
}
