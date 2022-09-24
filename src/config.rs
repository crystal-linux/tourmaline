use serde::Serialize;

use crate::tasks::{
    BootloaderConfig, DesktopConfig, LocaleConfig, NetworkConfig, PartitionsConfig, UsersConfig,
};

#[derive(Clone, Debug, Serialize)]
pub struct Config {
    pub local: LocaleConfig,
    pub network: NetworkConfig,
    pub partitions: PartitionsConfig,
    pub bootloader: BootloaderConfig,
    pub desktop: DesktopConfig,
    pub users: UsersConfig,
}
