use std::path::PathBuf;

use crate::error::{AppError, AppResult};

use super::script::{NuScript, Script};

/// A loader for nu script files
pub struct ScriptLoader {
    script_dir: PathBuf,
    hook_dir: PathBuf,
}

pub enum HookType {
    Pre,
    Post,
}

impl ScriptLoader {
    /// Creates a new script loader with the default config dir
    pub fn new() -> Self {
        Self {
            script_dir: crate::utils::SCRIPT_PATH.to_owned(),
            hook_dir: crate::utils::HOOK_PATH.to_owned(),
        }
    }

    /// Loads the given script file
    #[tracing::instrument(level = "trace", skip_all)]
    pub fn load<S: Script>(&self) -> AppResult<NuScript<S>> {
        let script_path = self.script_dir.join(S::get_name());

        if !script_path.exists() {
            Err(AppError::ScriptNotFound(script_path))
        } else {
            Ok(NuScript::new(script_path))
        }
    }

    pub fn load_hook<S: Script>(&self, hook_type: HookType) -> Option<NuScript<S>> {
        let script_name = match hook_type {
            HookType::Pre => S::get_pre_hook(),
            HookType::Post => S::get_post_hook(),
        };
        let script_path = self.hook_dir.join(script_name);

        if !script_path.exists() {
            None
        } else {
            Some(NuScript::new(script_path))
        }
    }
}
