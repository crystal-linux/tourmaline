use std::path::PathBuf;

use serde::Serialize;

use crate::script;

script!(InstallBootloaderScript {
    file = "install-bootloader.nu"
    args = BooloaderConfig
});

#[derive(Clone, Debug, Serialize)]
pub struct BooloaderConfig {
    preset: BootloaderPreset,
    location: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub enum BootloaderPreset {
    GrubEfi,
    Legacy,
}
