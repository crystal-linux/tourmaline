use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::script;

script!(InstallBootloaderScript {
    file = "install-bootloader"
    args = BootloaderConfig
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BootloaderConfig {
    pub preset: BootloaderPreset,
    pub location: PathBuf,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum BootloaderPreset {
    GrubEfi,
    Legacy,
}
