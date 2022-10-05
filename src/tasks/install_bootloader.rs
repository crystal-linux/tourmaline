use std::path::PathBuf;

use embed_nu::rusty_value::RustyValue;
use serde::Deserialize;

use crate::script;

script!(InstallBootloaderScript {
    file = "install-bootloader"
    args = BootloaderConfig
});

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub struct BootloaderConfig {
    pub preset: BootloaderPreset,
    pub location: PathBuf,
}

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub enum BootloaderPreset {
    GrubEfi,
    Legacy,
}
