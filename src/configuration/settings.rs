use config::{ConfigBuilder, ConfigError, File, FileFormat};
use config::builder::DefaultState;
use serde::Deserialize;
use crate::configuration::Cli;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    pub log: Log,
    pub llm: LLMClient,
    pub neo4j: Neo4jClient,
    pub prompts: Prompts,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Log {
    pub level: String,
}

#[derive(Debug, Deserialize, Clone)]
pub enum LLMClient {
    Ollama(OllamaLLMClient)
}

#[derive(Debug, Deserialize, Clone)]
pub struct OllamaLLMClient {
    pub uri: String,
    pub model: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Neo4jClient {
    pub uri: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Prompts {
    pub summary: Prompt,
    pub predict: Prompt,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Prompt {
    pub system: Option<String>,
    pub prompt: String,
}

// TODO: change this to /etc/liaisons/default when client is ready to run as a
// job.
const DEFAULT_CFG_PATH: &str = ".dev/config/default";

impl Settings {
    pub fn new(config_path: & Option<String>) -> Result<Self, ConfigError> {
        let builder: ConfigBuilder<DefaultState> = ConfigBuilder::default();

        let mut builder = builder
            .add_source(File::new(DEFAULT_CFG_PATH, FileFormat::Yaml));

        if let Some(ref path) = config_path {
            builder = builder
                .add_source(File::new(path, FileFormat::Yaml));
        }
        
        let config = builder.build()?;
        
        config.try_deserialize()
    }

    pub fn merge_to_cli(& mut self, cli: &Cli) {
        if let Some(ref level) = cli.log.level {
            self.log.level = level.clone();
        }
    }
}
