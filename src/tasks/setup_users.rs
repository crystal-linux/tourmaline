use std::collections::HashMap;

use serde::Serialize;

use crate::scripting::{
    record::RecordValue,
    script::{Script, ScriptArgs},
};

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

impl ScriptArgs for UsersConfig {
    fn get_args(self) -> Vec<RecordValue> {
        let mut user_cfg_map = HashMap::new();
        let mut user_cfgs = Vec::new();

        for user in self.users {
            let mut user_map: HashMap<&'static str, RecordValue> = HashMap::new();
            user_map.insert("name", user.name.into());
            user_map.insert("password", user.password.into());
            user_map.insert("sudoer", user.sudoer.into());
            user_map.insert("shell", user.shell.into());
            user_cfgs.push(user_map);
        }
        user_cfg_map.insert("users", user_cfgs);

        vec![user_cfg_map.into()]
    }
}
