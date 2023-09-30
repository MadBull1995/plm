use clap::{Args, Parser, Subcommand, ValueEnum};

pub mod commands {
    pub mod add;
    pub mod init;
    pub mod install;
    pub mod login;
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
    pub mod auth;
    pub mod configs;
    pub mod errors;
    pub mod helpers;
    pub mod lock;
    pub mod prompter;
    pub mod tracing;
    pub async fn time_it<F, Fut, T, E>(label: &str, f: F) -> Result<T, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = anyhow::Result<T, E>>,
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
#[derive(Parser, Clone, Debug)]
#[command(name = "plm")]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Turn prompter information off
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub quiet: bool,

    /// Turn debugging information on (-d, -dd, -ddd -> for tx/rx debug)
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,
}

/// A subcommand for plm
#[derive(Debug, Subcommand, Clone)]
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
    Config(ConfigArgs),
    // Lists installed packages
    // List(List),
}

#[derive(Debug, Args, Clone)]
#[command(args_conflicts_with_subcommands = true)]
pub struct ConfigArgs {
    #[command(subcommand)]
    pub command: Option<ConfigCommand>,
    // #[command(flatten)]
    // push: StashPushArgs,
}

/// Create a new workspace
#[derive(Debug, Subcommand, Clone)]
pub enum ConfigCommand {
    #[clap(about = "Get a value from the config")]
    Get {
        /// The config key to get
        key: String,
    },
    #[clap(about = "Set a value in the config")]
    Set {
        /// The config key to set
        key: String,
        /// The new value to set
        value: String,
    },
    #[clap(about = "Show the entire config")]
    Show {
        /// Show the configs in JSON format
        #[arg(long, action = clap::ArgAction::SetTrue)]
        json: bool,
    },
}

#[derive(Debug, ValueEnum, Clone)]
pub enum ConfigAction {
    Get,
    Set,
}

/// Create a new library
#[derive(Debug, Args, Clone)]
pub struct Init {
    /// The library name to initialize
    #[arg(long)]
    pub library_name: Option<String>,

    /// The library src directory where the protobuf files are nested
    #[arg(long)]
    pub src_dir: Option<String>,

    /// The library description
    #[arg(long)]
    pub description: Option<String>,

    /// The library version in semver (excluding suffix only major.minor.patch supported currently)
    #[arg(long)]
    pub version: Option<String>,

    /// The library dependencies space separated list, in format: <library>:<x.y.z>
    #[arg(long)]
    pub dependencies: Option<Vec<String>>,

    /// The library license
    #[arg(long)]
    pub license: Option<License>,

    /// The library exclude directories from publishing, comma-delimited list
    #[arg(long)]
    pub exclude: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum License {
    /// APACHE-2.0
    APACHE2,

    /// MIT
    MIT,

    /// GPL-3.0
    GPL,

    /// Unlicense
    UNLICENSE,
}

/// Create a new workspace
#[derive(Debug, Args, Clone)]
pub struct Login {
    /// The username to login/signup with
    pub user: String,

    /// The password to login/signup with
    pub password: String,
}

/// Installs a package
#[derive(Debug, Args, Clone)]
#[command(arg_required_else_help = false, args_conflicts_with_subcommands = true)]
pub struct Install {
    /// The name of the package to install
    pub name: Option<String>,

    /// Verbose mode
    #[arg(short, long)]
    pub global: bool,
}

/// Uninstalls a package
#[derive(Debug, Args, Clone)]
pub struct Uninstall {
    /// The name of the package to uninstall
    pub name: String,
}

/// Publishes a package
#[derive(Debug, Args, Clone)]
pub struct Publish {
    /// The path to the package directory
    pub path: Option<String>,

    /// This would preserve the original import paths to ensure that the original structure is emulated within proto_modules/
    #[arg(long, action = clap::ArgAction::SetTrue)]
    pub preserve_imports: bool,
}

pub fn parse_cli() -> Cli {
    Cli::parse()
}
// Re-export types for easier use in dependent code.
pub use crate::{
    commands::{add, init, install, remove, update},
    utils::{helpers, tracing},
};
