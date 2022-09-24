use std::path::PathBuf;

use serde::Serialize;

use crate::script;

script!(InstallBootloaderScript {
    file = "install-bootloader"
    args = BootloaderConfig
});

#[derive(Clone, Debug, Serialize)]
pub struct BootloaderConfig {
    preset: BootloaderPreset,
    location: PathBuf,
}

#[derive(Clone, Debug, Serialize)]
pub enum BootloaderPreset {
    GrubEfi,
    Legacy,
}
