use clap::{Parser, Args, Subcommand, ValueEnum};

pub mod commands {
    pub mod add;
    pub mod init;
    pub mod install;
    pub mod update;
    pub mod remove;
}

pub mod utils {
    use std::time::Instant;

    use crate::utils::prompter::Prompter;

    pub mod tracing;
    pub mod helpers;
    pub mod errors;
    pub mod prompter;
    pub async fn time_it<F, Fut>(label: &str, f: F)
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future,
    {
        let start = Instant::now();
        f().await;

        let duration = start.elapsed();
        Prompter::normal(format!("{} took {:.2?}", label, duration).as_str());
    }
}

/// Protobuf Package Manager
#[derive(Parser)]
#[command(name = "ppm")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// A subcommand for ppm
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

    // Lists installed packages
    // List(List),
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
    pub path: String,
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
