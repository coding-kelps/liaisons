pub mod cli;
pub mod settings;
pub mod logger;

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

    #[error("environment variable loading error: {0}")]
    VarError(#[from] env::VarError),
}
pub fn load_configuration(cli: &Cli) -> Result<Settings, Error> {
    let s: Settings;

    if let Ok(path) = env::var("LIAISONS_CONFIG") {
        let config_path = path;

        s = settings::Settings::load_from_file(config_path)?;
    } else if let Ok(user) = env::var("USER") {
        let config_path = format!("/home/{}/.liaisons/config.yml", user);

        if fs::metadata(config_path.clone()).is_ok() {
            s = settings::Settings::load_from_file(config_path)?;
        } else {
            let config_path = String::from("/etc/liaisons/default/config.yml");

            tracing::debug!("no custom configuration file found, loading default one");
            s = settings::Settings::load_from_file(config_path)?;
    
            tracing::info!("saving configuration in user \"~/.liaisons\" directory");
            save_configuration(&s)?;
        }
    } else {
        let config_path = String::from("/etc/liaisons/default/config.yml");

        tracing::debug!("no custom configuration file found, loading default one");
        s = settings::Settings::load_from_file(config_path)?;
    }


    Ok(s.merge_to_cli(&cli))
}

fn save_configuration(s: &Settings) -> Result<(), Error> {
    let config_dir = format!("/home/{}/.liaisons", env::var("USER")?);

    if fs::metadata(config_dir.clone()).is_err() {
        tracing::debug!("no custom configuration directory found, creating one");

        fs::create_dir("/home/guilhem/.liaisons")?;
    }

    let yaml = serde_yaml::to_string(s)?;
    
    fs::write(format!("{}/{}", config_dir, "config.yml"), yaml)?;

    Ok(())
}
