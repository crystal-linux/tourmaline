use tourmaline::{
    tasks::{User, UsersConfig},
    TaskExecutor,
};

#[tokio::main(flavor = "current_thread")]
async fn main() -> miette::Result<()> {
    color_eyre::install().unwrap();
    dotenv::dotenv().unwrap();
    let executor = TaskExecutor::new();
    let user_cfg = UsersConfig {
        users: vec![
            User {
                name: String::from("test"),
                password: String::from("password"),
                sudoer: false,
                shell: String::from("/bin/zsh"),
            },
            User {
                name: String::from("test2"),
                password: String::from("superpassword"),
                sudoer: true,
                shell: String::from("/bin/nu"),
            },
        ],
    };
    executor.setup_users(user_cfg).await?;

    Ok(())
}
