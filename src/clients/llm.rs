mod ollama;
use thiserror::Error as ThisError;
use crate::configuration::settings;
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
    #[error("ollama client error: {0}")]
    Ollama(#[from] ollama::Error),
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
    pub fn new(cfg: &settings::LLMCfg) -> Self {
        match cfg {
            settings::LLMCfg::Ollama { ollama } =>
                Client::Ollama(ollama::Client::new(ollama)),
        }
    }
}

/// Trait defining all the Large Language Model client expected features for
/// this program.
pub trait ClientTrait {
    /// Summarize the underlying argument of a user generated web-content
    /// (e.g., Twitter Post)
    async fn summarize(&self, prompt: &settings::Prompt, content: String) -> Result<models::SummarizedInfo, Error>;

    async fn predict(&self, prompt: &settings::Prompt, arg_a: &models::Argument, arg_b: &models::Argument) -> Result<models::Relation, Error>;
}

impl ClientTrait for Client {
    async fn summarize(&self, prompt: &settings::Prompt, content: String) -> Result<models::SummarizedInfo, Error> {
        match self {
            Client::Ollama(client) => client.summarize(prompt, content).await,
        }
    }

    async fn predict(&self, prompt: &settings::Prompt, arg_a: &models::Argument, arg_b: &models::Argument) -> Result<models::Relation, Error> {
        match self {
            Client::Ollama(client) => client.predict(prompt, arg_a, arg_b).await,
        }
    }
}
