use embed_nu::rusty_value::*;
use serde::Deserialize;

use crate::script;

script!(SetupUsersScript {
    file = "setup-users"
    args = UsersConfig
});

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub struct UsersConfig {
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Deserialize, RustyValue)]
pub struct User {
    pub name: String,
    pub password: String,
    pub sudoer: bool,
    pub shell: String,
}
