use std::path::PathBuf;

const CONFIG_DIR: &str = "/etc";

lazy_static::lazy_static! {
    pub static ref CFG_PATH: PathBuf = PathBuf::from(CONFIG_DIR).join("tourmaline");
    pub static ref SCRIPT_PATH: PathBuf = CFG_PATH.join("scripts");
}
