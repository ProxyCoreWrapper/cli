mod cli;
mod client;
mod config;
mod consts;

use clap::Parser;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let config = config::read().expect("configuration error");

    let args = cli::Cli::parse();

    let mut client = match client::PCWClient::connect(&config.server.uds_path).await {
        Ok(client) => client,
        Err(e) => {
            eprintln!("Failed to connect to server: {e}");
            return ExitCode::FAILURE;
        }
    };

    match args.section {
        cli::Sections::Configs { action } => match action {
            cli::ConfigsCommands::List {} => match client.configs.list().await {
                Ok(configs) => println!("{:?}", configs),
                Err(e) => {
                    eprintln!("Error listing configs: {}", e);
                    return ExitCode::FAILURE;
                }
            },
            cli::ConfigsCommands::Get { config_id } => match client.configs.get(config_id).await {
                Ok(config) => println!("{}", config),
                Err(e) => {
                    eprintln!("Error getting config: {}", e);
                    return ExitCode::FAILURE;
                }
            },
            cli::ConfigsCommands::Edit {
                config_id,
                new_config,
            } => {
                let config_content = if new_config == "-" {
                    eprintln!("Reading config from stdin...");
                    let mut buffer = String::new();
                    if let Err(e) = std::io::stdin().read_line(&mut buffer) {
                        eprintln!("Error reading from stdin: {}", e);
                        return ExitCode::FAILURE;
                    }
                    let config_content = buffer.trim().to_string();
                    if let Err(e) = client.configs.edit(config_id.clone(), config_content).await {
                        eprintln!("Error editing config: {}", e);
                        return ExitCode::FAILURE;
                    }

                    buffer
                } else {
                    new_config
                };

                if let Err(e) = client.configs.edit(config_id, config_content).await {
                    eprintln!("Error editing config: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        },
        cli::Sections::Service {} => {}
    }

    ExitCode::SUCCESS
}
