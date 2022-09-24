use serde::Serialize;

use crate::scripting::script::Script;

pub struct SetupUsersScript;

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

impl Script for SetupUsersScript {
    type Args = UsersConfig;

    fn get_name() -> &'static str {
        "setup-users.nu"
    }
}
