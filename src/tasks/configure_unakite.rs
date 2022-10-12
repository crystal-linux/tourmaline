use std::path::PathBuf;

use embed_nu::rusty_value::*;
use serde::Deserialize;

use crate::script;

script!(ConfigureUnakiteScript {
    file = "configure-unakite"
    args = UnakiteConfig
});

#[derive(Clone, Debug, RustyValue, Deserialize)]
pub struct UnakiteConfig {
    pub root: PathBuf,
    pub old_root: PathBuf,
    pub efidir: PathBuf,
    pub bootdev: PathBuf,
}
