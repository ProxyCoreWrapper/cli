use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum ConfigsCommands {
    List,
    Get {
        config_id: String
    },
    Edit {
        config_id: String,
        new_config: String
    }
}

#[derive(Subcommand, Debug)]
pub enum Sections {
    Configs {
        #[command[subcommand]]
        action: ConfigsCommands,
    },
    Service {}
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub section: Sections,
}
