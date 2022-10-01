use serde::{Deserialize, Serialize};

use crate::script;

script!(SetupUsersScript {
    file = "setup-users"
    args = UsersConfig
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UsersConfig {
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
    pub sudoer: bool,
    pub shell: String,
}
