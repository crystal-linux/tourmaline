use std::{io, path::PathBuf};

use miette::Diagnostic;
use nu_protocol::ShellError;
use thiserror::Error;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Error, Diagnostic, Debug)]
pub enum AppError {
    #[error("Miette error")]
    #[diagnostic()]
    Miette(miette::Error),

    #[error("Error while evaluating nu script")]
    #[diagnostic()]
    Nu(#[from] ShellError),

    #[error("Could not find the script file")]
    ScriptNotFound(PathBuf),

    #[diagnostic()]
    #[error("Could not parse the source file: {0}")]
    ParseError(#[from] nu_parser::ParseError),

    #[diagnostic()]
    #[error("Could not find the main mehod in the script file {0}")]
    MissingMain(PathBuf),

    #[error("Failed to execute script")]
    FailedToExecuteScript,

    #[error("Missing config")]
    MissingConfig,

    #[error("IO Error: {0}")]
    Io(#[from] io::Error),
}

impl From<miette::Error> for AppError {
    fn from(e: miette::Error) -> Self {
        Self::Miette(e)
    }
}
