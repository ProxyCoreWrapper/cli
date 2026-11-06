use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct ConfigArgs {
    /// Path to the new configuration file, or `-` to read from stdin.
    #[arg(short, long, value_name = "PATH")]
    pub new_config: Option<String>,
    /// Core tag used to start the configuration.
    #[arg(short, long)]
    pub core_tag: Option<String>,
    /// Start this configuration automatically when the backend starts.
    #[arg(long, value_name = "BOOL")]
    pub auto_restart: Option<bool>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigsCommands {
    /// List all configurations and their current state.
    List,
    /// Print the contents of a configuration.
    Get {
        /// Configuration identifier.
        config_id: String,
    },
    /// Create or update a configuration.
    Edit {
        /// Configuration identifier.
        config_id: String,
        #[command(flatten)]
        args: ConfigArgs,
    },
    /// Delete a configuration.
    Delete {
        /// Configuration identifier.
        config_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ManagementCommands {
    /// Start a configuration.
    Start {
        /// Configuration identifier.
        config_id: String,
    },
    /// Restart a configuration.
    Restart {
        /// Configuration identifier.
        config_id: String,
    },
    /// Stop a configuration.
    Stop {
        /// Configuration identifier.
        config_id: String,
    },
    /// Stream logs from a running configuration.
    Logs {
        /// Configuration identifier.
        config_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum Sections {
    /// Create, inspect, update, and delete configurations.
    Configs {
        #[command(subcommand)]
        action: ConfigsCommands,
    },
    /// Start, stop, restart, and inspect configuration processes.
    Management {
        #[command(subcommand)]
        action: ManagementCommands,
    },
}

#[derive(Parser, Debug)]
#[command(
    version,
    about = "Command-line client for the ProxyCoreWrapper backend"
)]
pub struct Cli {
    #[command(subcommand)]
    pub section: Sections,
}
