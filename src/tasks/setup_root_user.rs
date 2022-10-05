use embed_nu::rusty_value::RustyValue;
use serde::Deserialize;

use crate::script;

script!(SetupRootUserScript {
    file = "setup-root-user"
    args = RootUserConfig
});

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub struct RootUserConfig {
    pub password: String,
}
