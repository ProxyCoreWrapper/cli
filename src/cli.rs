use clap::{Args, Parser, Subcommand};

#[derive(Args, Debug)]
#[group(required = true, multiple = true)]
pub struct ConfigArgs {
    #[arg(short, long)]
    pub new_config: Option<String>,
    #[arg(short, long)]
    pub core_tag: Option<String>,
}

#[derive(Subcommand, Debug)]
pub enum ConfigsCommands {
    List,
    Get {
        config_id: String,
    },
    Edit {
        config_id: String,
        #[command(flatten)]
        args: ConfigArgs,
    },
    Delete {
        config_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum ManagementCommands {
    Start {
        config_id: String,
    },
    Restart {
        config_id: String,
    },
    Stop {
        config_id: String,
    },
    Logs {
        config_id: String,
    },
}

#[derive(Subcommand, Debug)]
pub enum Sections {
    Configs {
        #[command(subcommand)]
        action: ConfigsCommands,
    },
    Management {
        #[command(subcommand)]
        action: ManagementCommands,
    },
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub section: Sections,
}
