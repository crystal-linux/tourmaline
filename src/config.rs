use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::tasks::{
    BootloaderConfig, BootloaderPreset, DesktopConfig, ExtraPackages, Kernel, KernelConfig,
    LocaleConfig, NetworkConfig, Partitions, PartitionsConfig, RootUserConfig, UnakiteConfig,
    UsersConfig,
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub locale: LocaleConfig,
    pub network: NetworkConfig,
    pub partitions: PartitionsConfig,
    pub bootloader: BootloaderConfig,
    pub kernels: KernelConfig,
    pub desktop: DesktopConfig,
    pub users: UsersConfig,
    pub root_user: RootUserConfig,
    pub unakite: Option<UnakiteConfig>,
    pub extra_packages: ExtraPackages,
    pub enable_timeshift: bool,
    pub enable_flatpak: bool,
    pub enable_zramd: bool,
}

impl Config {
    pub(crate) fn empty() -> Self {
        Self {
            locale: LocaleConfig {
                locale: Vec::new(),
                keymap: String::new(),
                timezone: String::new(),
            },
            network: NetworkConfig {
                hostname: String::new(),
                ipv6_loopback: false,
            },
            partitions: PartitionsConfig {
                device: PathBuf::new(),
                efi_partition: false,
                partitions: Partitions::Auto,
            },
            bootloader: BootloaderConfig {
                preset: BootloaderPreset::GrubEfi,
                location: PathBuf::new(),
            },
            kernels: KernelConfig {
                default: Kernel(String::new()),
                additional: Vec::new(),
            },
            desktop: DesktopConfig::KdePlasma,
            users: UsersConfig { users: Vec::new() },
            root_user: RootUserConfig {
                password: String::new(),
            },
            unakite: None,
            extra_packages: Vec::new(),
            enable_timeshift: false,
            enable_flatpak: false,
            enable_zramd: false,
        }
    }
}
