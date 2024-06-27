pub mod cli;
pub mod settings;

pub use cli::*;
pub use settings::*;

use thiserror::Error as ThisError;
use std::fs;
use serde_yaml;
use tracing;
use std::env;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("configuration loading error: {0}")]
    ConfigError(#[from] settings::config::ConfigError),

    #[error("{0}")]
    IOError(#[from] std::io::Error),

    #[error("yaml serialization error: {0}")]
    SerializationError(#[from] serde_yaml::Error),
}

const DEFAULT_CFG: &str = "/etc/liaisons/default/config.yml";

pub fn load_configuration(cli: &Cli) -> Result<Settings, Error> {
    let config_path = format!("/home/{}/.liaisons/config.yml", env::var("USER").unwrap());

    let s = match settings::Settings::load_from_file(config_path) {
        Ok(s) => s,
        Err(_) => {
            tracing::warn!("failed to load custom configuration file, loading default configuration file");

            let s = settings::Settings::load_from_file(String::from(DEFAULT_CFG))?;

            save_configuration(&s)?;

            s
        }
    };

    Ok(s.merge_to_cli(&cli))
}

fn save_configuration(s: &Settings) -> Result<(), Error> {
    let config_dir = format!("/home/{}/.liaisons", env::var("USER").unwrap());

    if fs::metadata(config_dir.clone()).is_err() {
        tracing::warn!("no custom configuration directory found, creating one");

        fs::create_dir("/home/guilhem/.liaisons")?;
    }

    let yaml = serde_yaml::to_string(s)?;

    fs::write(format!("{}/{}", config_dir, "config.yml"), yaml)?;

    Ok(())
}
