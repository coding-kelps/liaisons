mod ollama;
use std::fs;
use std::path::PathBuf;
use thiserror::Error as ThisError;
use serde_yaml::Mapping;
use crate::models;

pub enum Client {
    Ollama(ollama::Client),
}

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
    pub fn new(path: & PathBuf) -> Result<Self, Error> {
        let data = fs::read_to_string(path)?;

        let mapping = serde_yaml::from_str::<Mapping>(&data)?;

        let mut loaded_client: Option<Self> = None;
        let client_keys = ["ollama"];


        for key in client_keys {
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
            None => Err(Error::Configuration(String::from(""))),
        }
    }
}

pub trait ClientTrait {
    async fn retrieve(&self, content: String) -> Result<models::Argument, Error>;
}

impl ClientTrait for Client {
    async fn retrieve(&self, content: String) -> Result<models::Argument, Error> {
        match self {
            Client::Ollama(client) => client.retrieve(content).await,
        }
    }
}
