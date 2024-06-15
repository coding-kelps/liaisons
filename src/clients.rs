mod ollama;
use std::fs;
use std::path::PathBuf;
use thiserror::Error as ThisError;
use serde_yaml::Mapping;
use crate::models;

/// An abstraction of a Large Language Model client (e.g., Ollama), to provide 
/// an interface to requests arguments summarization and relations prediction.
/// 
/// # Examples
/// 
/// ```rs
/// let client = Client::new("/path/to/client/config.yml");
/// 
/// let summarized_argument = client.summarize("An example argument").unwrap();
/// ```
pub enum Client {
    Ollama(ollama::Client),
}

/// The different kinds of error expected from a client.
#[derive(Debug, ThisError)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("serialization error: {0}")]
    Serde(#[from] serde_yaml::Error),


    #[error("ollama client error: {0}")]
    Ollama(#[from] ollama::Error),

    #[error("configuration error: {0}")]
    Configuration(String),
}

impl Client {
    /// Create a new Large Language Model client from a YAML configuration file.
    /// For now the only accepted configuration is a configuration for an Ollama
    /// client.
    /// 
    /// # Arguments
    /// 
    /// * `path` - The path of the client YAML configuration file.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// 
    /// * The configuration file does not exist.
    /// * The file cannot be read.
    /// * The file contains invalid data (e.g., wrong format)
    /// * There is no or more than one client configuration in the file.
    /// 
    /// # Examples
    /// 
    /// ```rs
    /// let client = Client::new("/path/to/client/config.yml");
    /// 
    /// let summarized_argument = client.summarize("An example argument").unwrap();
    /// ```
    pub fn new(path: & PathBuf) -> Result<Self, Error> {
        // Retrieve file as mapping to identify which type client configuration
        // is loaded from the available client keys.
        let mapping = {
            let data = fs::read_to_string(path)?;

            serde_yaml::from_str::<Mapping>(&data)?
        };
        let mut loaded_client: Option<Self> = None;
        let accepted_client_keys = ["ollama"];

        for key in accepted_client_keys {
            if mapping.get(key).is_some() {
                let value = mapping.get(key).unwrap();

                match ollama::Client::from_value(value) {
                    Ok(client) => {
                        if loaded_client.is_none() {
                            loaded_client = Some(Client::Ollama(client));
                        } else {
                            return Err(Error::Configuration(
                                String::from("more than one client definition in configuration file")))
                        }
                    },
                    Err(e) => return Err(Error::Configuration(format!("{}", e))),
                };
            }
        }

        match loaded_client {
            Some(client) => Ok(client),
            None => Err(Error::Configuration(String::from("no client configuration found"))),
        }
    }
}

/// Trait defining all the Large Language Model client expected features for
/// this program.
pub trait ClientTrait {
    /// Summarize the underlying argument of a user generated web-content
    /// (e.g., Twitter Post)
    async fn summarize(&self, content: String) -> Result<models::Argument, Error>;
}

impl ClientTrait for Client {
    async fn summarize(&self, content: String) -> Result<models::Argument, Error> {
        match self {
            Client::Ollama(client) => client.summarize(content).await,
        }
    }
}
