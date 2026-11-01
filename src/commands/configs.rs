use crate::{cli, client::{self, PCWClient}, messages};
use std::process::ExitCode;

pub async fn execute(client: &mut PCWClient, action: cli::ConfigsCommands) -> ExitCode {
    match action {
        cli::ConfigsCommands::List => match client.configs.list().await {
            Ok(configs) => print_configs(&configs),
            Err(error) => {
                eprintln!(
                    "{}: {}",
                    messages::LIST_CONFIGS_ERROR,
                    messages::format_status(&error)
                );
                return ExitCode::FAILURE;
            }
        },
        cli::ConfigsCommands::Get { config_id } => match client.configs.get(config_id).await {
            Ok(config) => println!("{config}"),
            Err(error) => {
                eprintln!(
                    "{}: {}",
                    messages::GET_CONFIG_ERROR,
                    messages::format_status(&error)
                );
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
            let config_content = match read_config(new_config) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::FAILURE;
                }
            };

            if let Err(error) = client
                .configs
                .edit(config_id, config_content, core_tag)
                .await
            {
                eprintln!(
                    "{}: {}",
                    messages::EDIT_CONFIG_ERROR,
                    messages::format_status(&error)
                );
                return ExitCode::FAILURE;
            }

            println!("{}", messages::CONFIG_EDITED);
        }
        cli::ConfigsCommands::Delete { config_id } => {
            if let Err(error) = client.configs.delete(config_id).await {
                eprintln!(
                    "{}: {}",
                    messages::DELETE_CONFIG_ERROR,
                    messages::format_status(&error)
                );
                return ExitCode::FAILURE;
            }

            println!("{}", messages::CONFIG_DELETED);
        }
    }

    ExitCode::SUCCESS
}

fn print_configs(configs: &[client::proto::Config]) {
    if configs.is_empty() {
        println!("{}", messages::NO_CONFIGS);
        return;
    }

    let id_width = configs
        .iter()
        .map(|config| config.config_id.len())
        .max()
        .unwrap_or(0)
        .max(messages::CONFIG_LIST_ID_HEADER.len());
    let core_width = configs
        .iter()
        .map(|config| config.core_tag.len())
        .max()
        .unwrap_or(0)
        .max(messages::CONFIG_LIST_CORE_HEADER.len());

    println!(
        "{:<id_width$}  {:<core_width$}  {}",
        messages::CONFIG_LIST_ID_HEADER,
        messages::CONFIG_LIST_CORE_HEADER,
        messages::CONFIG_LIST_STATE_HEADER,
        id_width = id_width,
        core_width = core_width
    );
    println!(
        "{:<id_width$}  {:<core_width$}  {}",
        messages::CONFIG_LIST_SEPARATOR.repeat(id_width),
        messages::CONFIG_LIST_SEPARATOR.repeat(core_width),
        messages::CONFIG_LIST_SEPARATOR.repeat(messages::CONFIG_LIST_STATE_HEADER.len()),
        id_width = id_width,
        core_width = core_width
    );

    for config in configs {
        let state = client::proto::config::State::try_from(config.state)
            .map(|state| state.as_str_name())
            .unwrap_or(messages::CONFIG_LIST_UNKNOWN_STATE);
        println!(
            "{:<id_width$}  {:<core_width$}  {state}",
            config.config_id,
            config.core_tag,
            id_width = id_width,
            core_width = core_width
        );
    }
}

fn read_config(new_config: Option<String>) -> Result<Option<String>, String> {
    match new_config {
        Some(path) if path == "-" => {
            println!("{}", messages::READING_CONFIG_STDIN);
            let mut buffer = String::new();
            std::io::stdin()
                .read_line(&mut buffer)
                .map_err(|error| format!("{}: {error}", messages::READ_STDIN_ERROR))?;
            Ok(Some(buffer.trim().to_string()))
        }
        Some(path) => std::fs::read_to_string(path)
            .map(Some)
            .map_err(|error| format!("{}: {error}", messages::READ_CONFIG_FILE_ERROR)),
        None => Ok(None),
    }
}
