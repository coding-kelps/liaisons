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

    #[error("no client found in configuration")]
    NoClient,
}

impl Client {
    pub fn new(path: & PathBuf) -> Result<Self, Error> {
        let data = fs::read_to_string(path)?;

        let mapping = serde_yaml::from_str::<Mapping>(&data)?;

        let value = mapping.get("ollama").unwrap();

        match ollama::Client::from_value(value) {
            Ok(client) => Ok(Client::Ollama(client)),
            Err(_) => Err(Error::NoClient),
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
