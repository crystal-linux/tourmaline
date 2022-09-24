use error::AppResult;
use scripting::loader::ScriptLoader;
use tasks::{SetupUsersScript, UsersConfig};

pub mod error;
pub(crate) mod scripting;
pub mod tasks;
pub(crate) mod utils;

pub struct TaskExecutor {
    loader: ScriptLoader,
}

impl TaskExecutor {
    /// Sets up user accounts
    #[tracing::instrument(level = "trace", skip(self))]
    pub async fn setup_users(&self, users_cfg: UsersConfig) -> AppResult<()> {
        self.loader
            .load::<SetupUsersScript>()?
            .execute(users_cfg)
            .await
    }
}

impl Default for TaskExecutor {
    fn default() -> Self {
        Self {
            loader: ScriptLoader::new(),
        }
    }
}
