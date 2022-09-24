use serde::Serialize;

use crate::script;

script!(SetupUsersScript {
    file = "setup-users"
    args = UsersConfig
});

#[derive(Clone, Debug, Serialize)]
pub struct UsersConfig {
    pub users: Vec<User>,
}

#[derive(Clone, Debug, Serialize)]
pub struct User {
    pub name: String,
    pub password: String,
    pub sudoer: bool,
    pub shell: String,
}
