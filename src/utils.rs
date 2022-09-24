use std::{env, path::PathBuf};

const DEFAULT_CONFIG_DIR: &str = "/etc";

lazy_static::lazy_static! {
    pub static ref CFG_PATH: PathBuf = env::var("TRM_CFG_PATH").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(DEFAULT_CONFIG_DIR).join("tourmaline"));
    pub static ref SCRIPT_PATH: PathBuf = CFG_PATH.join("scripts");
    pub static ref HOOK_PATH: PathBuf = CFG_PATH.join("hooks");
}
