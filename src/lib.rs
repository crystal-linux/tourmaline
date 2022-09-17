use scripting::executor::NuExecutor;

mod scripting;

pub fn test_execute(script: String) {
    NuExecutor::new(script).execute();
}
