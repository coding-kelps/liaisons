pub use config;
use config::{ConfigBuilder, ConfigError, File, FileFormat};
use config::builder::DefaultState;
use serde::{Serialize, Deserialize};
use crate::configuration::Cli;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub log: Log,
    pub llm: LLMCfg,
    pub repository: RepositoryCfg,
    pub prompts: Prompts,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Log {
    pub level: String,
    pub output: LogOutput,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LogOutput {
    Stdout,
    Stderr,
    Default,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum LLMCfg {
    Ollama{ ollama: OllamaCfg },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OllamaCfg {
    pub uri: String,
    pub model: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(untagged)]
pub enum RepositoryCfg {
    Neo4j{ neo4j: Neo4jCfg },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Neo4jCfg {
    pub uri: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompts {
    pub summary: Prompt,
    pub predict: Prompt,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    pub system: Option<String>,
    pub prompt: String,
}

impl Settings {
    pub fn load_from_file(path: String) -> Result<Self, ConfigError> {
        let builder: ConfigBuilder<DefaultState> = ConfigBuilder::default();

        let builder = builder
            .add_source(File::new(path.as_str(), FileFormat::Yaml));
        
        let config = builder.build()?;
        
        config.try_deserialize()
    }

    pub fn merge_to_cli(&self, cli: &Cli) -> Self {
        Settings {
            log: Log {
                level: match &cli.log.level {
                    Some(level) => level.to_string(),
                    None => self.log.level.clone(),
                },
                output: self.log.output.clone(),
            },
            llm: self.llm.clone(),
            repository: self.repository.clone(),
            prompts: self.prompts.clone(),
        }
    }
}
