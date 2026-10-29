use crate::consts;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, io};

#[derive(Deserialize, Serialize)]
pub struct Server {
    pub uds_path: PathBuf,
}

impl Default for Server {
    fn default() -> Self {
        Self { uds_path: PathBuf::from(consts::DEFAULT_UDS_PATH) }
    }
}

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub server: Server
}

impl Config {
    pub fn read() -> Result<Self> {
        let config_path = env::var_os("HOME")
            .map(PathBuf::from)
            .map(|home| home.join(consts::USER_CONFIG_PATH))
            .unwrap_or_else(|| PathBuf::from(consts::CONFIG_PATH));

        let mut config_file = match File::open(&config_path) {
            Ok(file) => file,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                let global_config_path = PathBuf::from(consts::CONFIG_PATH);
                match File::open(&global_config_path) {
                    Ok(file) => file,
                    Err(error) if error.kind() == io::ErrorKind::NotFound => {
                        return Ok(Default::default());
                    }
                    Err(error) => {
                        return Err(error).with_context(|| {
                            format!("Opening configuration file {}", global_config_path.display())
                        });
                    }
                }
            }
            Err(error) => {
                return Err(error).with_context(|| {
                    format!("Opening configuration file {}", config_path.display())
                });
            }
        };

        let mut config = String::new();
        config_file
            .read_to_string(&mut config)
            .context("Reading error")?;

        Ok(toml::from_str(config.as_str())?)
    }
}
