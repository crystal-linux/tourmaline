use std::{io, path::PathBuf};

use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Could not find the script file")]
    ScriptNotFound(PathBuf),

    #[error("Nu error {0}")]
    NuError(#[from] embed_nu::Error),

    #[error("Could not find the main mehod in the script file {0}")]
    MissingMain(PathBuf),

    #[error("Failed to execute script")]
    FailedToExecuteScript,

    #[error("Missing config")]
    MissingConfig,

    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON deserialization error {0}")]
    JSON(#[from] serde_json::Error),
}
