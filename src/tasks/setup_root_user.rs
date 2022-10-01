use serde::{Deserialize, Serialize};

use crate::script;

script!(SetupRootUserScript {
    file = "setup-root-user"
    args = RootUserConfig
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RootUserConfig {
    pub password: String,
}
