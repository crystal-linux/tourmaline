use std::path::PathBuf;

use clap::Parser;
use clap::Subcommand;

const VERSION: &str = concat!(
    env!("CARGO_PKG_VERSION"),
    " (",
    env!("TOURMALINE_CODENAME"),
    ")",
);

#[derive(Debug, Clone, Parser)]
#[clap(bin_name = "trm", name = "Tourmaline", version=VERSION, about= env!("CARGO_PKG_DESCRIPTION"), infer_subcommands = true)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Installs the system from the given config
    #[command()]
    InstallFromConfig(InstallFromConfigArgs),

    /// Generates empty script files for the installation
    #[command()]
    GenerateScripts(GenerateScriptsArgs),
}

#[derive(Debug, Clone, Parser)]
pub struct InstallFromConfigArgs {
    /// The path to the json config file
    #[arg()]
    pub path: PathBuf,
}

#[derive(Debug, Clone, Parser)]
pub struct GenerateScriptsArgs {
    /// The path to the folder where the scripts should be generated in
    #[arg()]
    pub path: PathBuf,
}
