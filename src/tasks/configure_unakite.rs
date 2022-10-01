use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::script;

script!(ConfigureUnakiteScript {
    file = "configure-unakite"
    args = UnakiteConfig
});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnakiteConfig {
    pub root: PathBuf,
    pub old_root: PathBuf,
    pub efidir: PathBuf,
    pub bootdev: PathBuf,
}
