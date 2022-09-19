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

    #[diagnostic()]
    #[error("Could not parse the source file {0}")]
    ParseError(#[from] nu_parser::ParseError),

    #[error("Could not find the main mehod in the script file {0}")]
    MissingMain(PathBuf),

    #[error("Failed to execute script")]
    FailedToExecuteScript,
}

impl From<miette::Error> for AppError {
    fn from(e: miette::Error) -> Self {
        Self::Nu(e)
    }
}
