mod cli;
mod client;
mod commands;
mod config;
mod consts;
mod messages;

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
        cli::Sections::Configs { action } => commands::configs::execute(&mut client, action).await,
        cli::Sections::Management { action } => {
            commands::management::execute(&mut client, action).await
        }
    }
}
