use error::AppResult;
use scripting::{
    loader::ScriptLoader,
    script::{Script, ScriptArgs},
};

pub mod error;
pub(crate) mod scripting;
pub(crate) mod utils;

pub struct TestScript;

impl Script for TestScript {
    type Args = TestScriptArgs;

    fn get_name() -> &'static str {
        "test.nu"
    }
}

pub struct TestScriptArgs {
    pub msg: String,
}

impl ScriptArgs for TestScriptArgs {
    fn get_args(self) -> Vec<String> {
        vec![self.msg]
    }
}

pub async fn test_execute() -> AppResult<()> {
    let loader = ScriptLoader::new();
    let test_script = loader.load::<TestScript>()?;

    test_script
        .execute(TestScriptArgs {
            msg: "'Hello World'".to_string(),
        })
        .await
}
