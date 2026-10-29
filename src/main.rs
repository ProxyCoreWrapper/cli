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
            eprintln!("Failed to connect to server: {e:#}");
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
                args:
                    cli::ConfigArgs {
                        new_config,
                        core_tag,
                    },
            } => {
                let config_content = match new_config {
                    Some(new_config) => {
                        if new_config == "-" {
                            println!("Reading config from stdin...");
                            let mut buffer = String::new();
                            if let Err(e) = std::io::stdin().read_line(&mut buffer) {
                                eprintln!("Error reading from stdin: {}", e);
                                return ExitCode::FAILURE;
                            }
                            Some(buffer.trim().to_string())
                        } else {
                            match std::fs::read_to_string(new_config) {
                                Ok(content) => Some(content),
                                Err(e) => {
                                    eprintln!("Error reading config file: {}", e);
                                    return ExitCode::FAILURE;
                                }
                            }
                        }
                    }
                    None => None,
                };

                if let Err(e) = client
                    .configs
                    .edit(config_id, config_content, core_tag)
                    .await
                {
                    eprintln!("Error editing config: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        },
        cli::Sections::Management { action } => match action {
            cli::ManagementCommands::Start { config_id } => {
                if let Err(e) = client.management.start(config_id).await {
                    eprintln!("Error starting service: {}", e);
                    return ExitCode::FAILURE;
                }
            }
            cli::ManagementCommands::Restart { config_id } => {
                if let Err(e) = client.management.restart(config_id).await {
                    eprintln!("Error restarting service: {}", e);
                    return ExitCode::FAILURE;
                }
            }
            cli::ManagementCommands::Stop { config_id } => {
                if let Err(e) = client.management.stop(config_id).await {
                    eprintln!("Error stopping service: {}", e);
                    return ExitCode::FAILURE;
                }
            }
            cli::ManagementCommands::Logs { config_id } => {
                if let Err(e) = client.management.print_log_stream(config_id).await {
                    eprintln!("Error receiving log stream: {}", e);
                    return ExitCode::FAILURE;
                }
            }
        },
    }

    ExitCode::SUCCESS
}
