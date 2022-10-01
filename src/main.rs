use args::{Args, Command, GenerateScriptsArgs, InstallFromConfigArgs};
use clap::Parser;
use miette::{Context, IntoDiagnostic};
use tokio::{fs::OpenOptions, io::AsyncReadExt};
use tourmaline::{config::Config, error::AppResult, generate_script_files, TaskExecutor};

mod args;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    color_eyre::install().unwrap();
    dotenv::dotenv().unwrap();
    let args = Args::parse();

    match args.command {
        Command::InstallFromConfig(args) => install_from_config(args).await,
        Command::GenerateScripts(args) => generate_scripts(args).await,
    }
    .unwrap();
}

async fn install_from_config(args: InstallFromConfigArgs) -> AppResult<()> {
    let mut file = OpenOptions::new()
        .read(true)
        .open(args.path)
        .await
        .into_diagnostic()
        .context("Could not read file")?;
    let mut cfg_contents = String::new();
    file.read_to_string(&mut cfg_contents)
        .await
        .into_diagnostic()
        .context("Could not read file")?;
    let config: Config = serde_json::from_str(&cfg_contents)
        .into_diagnostic()
        .context("Could not parse config as JSON")?;

    TaskExecutor::with_config(config)
        .install_from_config()
        .await
}

async fn generate_scripts(args: GenerateScriptsArgs) -> AppResult<()> {
    generate_script_files(args.path).await
}
