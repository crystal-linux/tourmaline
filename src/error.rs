use std::path::PathBuf;

use miette::Diagnostic;
use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Error, Diagnostic, Debug)]
pub enum AppError {
    #[error("Error while evaluating nu script")]
    #[diagnostic()]
    Nu(miette::Error),

    #[error("Could not find the script file {0}")]
    ScriptNotFound(PathBuf),
}

impl From<miette::Error> for AppError {
    fn from(e: miette::Error) -> Self {
        Self::Nu(e)
    }
}
