use crate::{cli, client::PCWClient, messages};
use std::process::ExitCode;

pub async fn execute(client: &mut PCWClient, action: cli::ManagementCommands) -> ExitCode {
    match action {
        cli::ManagementCommands::Start { config_id } => {
            if let Err(error) = client.management.start(config_id).await {
                eprintln!(
                    "{}: {}",
                    messages::START_SERVICE_ERROR,
                    messages::format_status(&error)
                );
                return ExitCode::FAILURE;
            }

            println!("{}", messages::SERVICE_STARTED);
        }
        cli::ManagementCommands::Restart { config_id } => {
            if let Err(error) = client.management.restart(config_id).await {
                eprintln!(
                    "{}: {}",
                    messages::RESTART_SERVICE_ERROR,
                    messages::format_status(&error)
                );
                return ExitCode::FAILURE;
            }

            println!("{}", messages::SERVICE_RESTARTED);
        }
        cli::ManagementCommands::Stop { config_id } => {
            if let Err(error) = client.management.stop(config_id).await {
                eprintln!(
                    "{}: {}",
                    messages::STOP_SERVICE_ERROR,
                    messages::format_status(&error)
                );
                return ExitCode::FAILURE;
            }

            println!("{}", messages::SERVICE_STOPPED);
        }
        cli::ManagementCommands::Logs { config_id } => {
            if let Err(error) = client.management.print_log_stream(config_id).await {
                eprintln!(
                    "{}: {}",
                    messages::RECEIVE_LOG_STREAM_ERROR,
                    messages::format_status(&error)
                );
                return ExitCode::FAILURE;
            }
        }
    }

    ExitCode::SUCCESS
}
