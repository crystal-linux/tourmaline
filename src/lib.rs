use error::AppResult;
use scripting::{loader::ScriptLoader, script::JSONArgs};
use tasks::{SetupUsersScript, UsersConfig};

pub mod error;
pub(crate) mod scripting;
pub mod tasks;
pub(crate) mod utils;

pub struct TaskExecutor {
    loader: ScriptLoader,
}

impl TaskExecutor {
    pub fn new() -> Self {
        Self {
            loader: ScriptLoader::new(),
        }
    }

    /// Sets up user accounts
    #[tracing::instrument(level = "debug", skip(self))]
    pub async fn setup_users(&self, users_cfg: UsersConfig) -> AppResult<()> {
        self.loader
            .load::<SetupUsersScript>()?
            .execute(JSONArgs(users_cfg))
            .await
    }
}
