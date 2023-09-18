use clap::{Args, Parser, Subcommand, ValueEnum};

pub mod commands {
    pub mod add;
    pub mod init;
    pub mod install;
    pub mod publish;
    pub mod remove;
    pub mod update;
}

pub mod registry {
    pub mod client;
}

pub mod utils {
    use std::time::Instant;

    use crate::utils::prompter::Prompter;

    pub mod configs;
    pub mod errors;
    pub mod helpers;
    pub mod prompter;
    pub mod tracing;
    pub async fn time_it<F, Fut, T, E>(label: &str, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<T, E>>,
    {
        let start = Instant::now();
        let result = f().await;

        let duration = start.elapsed();
        let millis = duration.as_millis();
        let secs = duration.as_secs();

        if millis < 1000 {
            Prompter::normal(format!("{} took {:.2?}ms", label, millis).as_str());
        } else {
            Prompter::normal(format!("{} took {:.2?}s", label, secs).as_str());
        }
        result
    }
}

/// Protobuf Package Manager
#[derive(Parser)]
#[command(name = "plm")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// A subcommand for plm
#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Initialize a new workspace
    Init(Init),

    /// Installs a package
    Install(Install),

    /// Uninstalls a package
    Uninstall(Uninstall),

    /// Publishes a package
    Publish(Publish),

    /// Saving login creds for the registry
    Login(Login),

    /// Saving login creds for the registry
    Config(ConfigCommand),
    // Lists installed packages
    // List(List),
}

/// Create a new workspace
#[derive(Debug, Args)]
pub struct ConfigCommand {
    /// An action to take on .plmrc global file
    pub action: ConfigAction,

    /// The config key
    pub key: String,

    /// The new config value
    pub value: Option<String>,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum ConfigAction {
    Get,
    Set,
}

/// Create a new workspace
#[derive(Debug, Args)]
pub struct Init {
    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

/// Create a new workspace
#[derive(Debug, Args)]
pub struct Login {
    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

/// Installs a package
#[derive(Debug, Args)]
#[command(arg_required_else_help = true, args_conflicts_with_subcommands = true)]
pub struct Install {
    /// The name of the package to install
    pub name: String,

    /// Verbose mode
    #[arg(short, long)]
    pub global: bool,

    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

/// Uninstalls a package
#[derive(Debug, Args)]
pub struct Uninstall {
    /// The name of the package to uninstall
    pub name: String,
    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

/// Publishes a package
#[derive(Debug, Args)]
pub struct Publish {
    /// The path to the package directory
    pub path: Option<String>,
    /// Verbose mode
    #[arg(short, long)]
    pub verbose: bool,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
// Re-export types for easier use in dependent code.
pub use crate::{
    commands::{add, init, install, remove, update},
    utils::{helpers, tracing},
};
