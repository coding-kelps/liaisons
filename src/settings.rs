pub use clap::{Parser, Subcommand, Args};
pub use config::{Config, ConfigError, File};
use serde::Deserialize;
use std::path::PathBuf;

// The structure that defines the parsed arguments
#[derive(Parser, Deserialize, Clone)]
#[command(version = "0.1.0", about, long_about = None)]
#[command(about = "A CLI client to mine arguments and their relations from social media posts", long_about = None)]
pub struct Settings {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[command(flatten)]
    pub log: Log,
}

#[derive(Args, Deserialize, Clone)]
pub struct Log {
    #[arg(long = "log-level")]
    #[arg(help = "The logging level of the client (DEBUG, INFO, ERROR...)")]
    #[clap(default_value = "INFO")]
    pub level: Option<String>,
}

#[derive(Subcommand, Deserialize, Clone)]
pub enum Commands {
    #[clap(alias("retrieve"))]
    #[clap(about = "Retrieve arguments from a given JSON source file gathering social media posts")]
    RetrieveArguments {
        #[arg(short, long)]
        #[arg(help = "The file path to the JSON source file of social media posts")]
        file: PathBuf,
        #[command(flatten)]
        model: Model,
    },

    #[clap(alias("predict"))]
    #[clap(about = "Predicts argument relations from a given JSON source file of retrieved arguments")]
    PredictRelations {
        #[arg(short, long)]
        #[arg(help = "The file path to the JSON source file of retrieved arguments")]
        file: PathBuf,
        #[command(flatten)]
        model: Model,
    }
}

#[derive(Args, Deserialize, Clone)]
pub struct Model {
    #[arg(long = "model-name")]
    #[arg(help = "The name and tag of the Large Language Model used")]
    #[clap(default_value = "gemma:2b")]
    pub name: String,
    #[arg(long = "model-endpoint")]
    #[arg(help = "The actual address of the Large Language Model used")]
    #[clap(default_value = "http://localhost:8080")]
    pub endpoint: String,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("./.dev/config/dev.yml")
                .required(false))
            .build()?;

        let cfg_file_settings: Settings = s.try_deserialize()?;

        let cmd_settings = Settings::parse();

        Ok(cfg_file_settings.merge(cmd_settings))
    }

    pub fn merge(self, other: Settings) -> Self {
        Self {
            command: other.command.or(self.command),
            log: Log {
                level: other.log.level.or(self.log.level),
            },
        }
    }
}
