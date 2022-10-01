use std::path::Path;
use std::{env, path::PathBuf};

use tokio::fs;

use crate::error::AppResult;

use crate::tasks::all_tasks;

const DEFAULT_CONFIG_DIR: &str = "/etc";

lazy_static::lazy_static! {
    pub static ref CFG_PATH: PathBuf = env::var("TRM_CFG_PATH").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(DEFAULT_CONFIG_DIR).join("tourmaline"));
    pub static ref SCRIPT_PATH: PathBuf = CFG_PATH.join("scripts");
    pub static ref HOOK_PATH: PathBuf = CFG_PATH.join("hooks");
}

pub async fn generate_script_files<P: AsRef<Path>>(output: P) -> AppResult<()> {
    let tasks = all_tasks();

    for task in tasks {
        fs::write(
            task.script_path(output.as_ref()),
            r#"
def main [cfg] {
    echo "Executing Task with config" $cfg                
}
        "#
            .to_string(),
        )
        .await?;
        fs::write(
            task.pre_hook_path(output.as_ref()),
            r#"
def main [cfg] {
    echo "Executing before Task with task config: " $cfg                
    echo "The global config is: " $TRM_CONFIG
}
        "#
            .to_string(),
        )
        .await?;
        fs::write(
            task.post_hook_path(output.as_ref()),
            r#"
def main [cfg] {
    echo "Executing after Task with task config: " $cfg                
    echo "The global config is: " $TRM_CONFIG
}
        "#
            .to_string(),
        )
        .await?;
    }

    Ok(())
}
