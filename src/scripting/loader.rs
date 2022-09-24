use std::path::PathBuf;

use crate::error::{AppError, AppResult};

use super::script::{NuScript, Script};

/// A loader for nu script files
pub struct ScriptLoader {
    base_dir: PathBuf,
}

impl ScriptLoader {
    /// Creates a new script loader with the default config dir
    pub fn new() -> Self {
        Self {
            base_dir: crate::utils::SCRIPT_PATH.to_owned(),
        }
    }

    /// Loads the given script file
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn load<S: Script>(&self) -> AppResult<NuScript<S>> {
        let script_path = self.base_dir.join(S::get_name());

        if !script_path.exists() {
            Err(AppError::ScriptNotFound(script_path))
        } else {
            Ok(NuScript::new(script_path))
        }
    }
}
