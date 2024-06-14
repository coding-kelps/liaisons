pub use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;
use serde::Deserialize;

// The structure that defines the parsed arguments
#[derive(Parser)]
#[command(version = "0.1.0", about, long_about = None)]
#[command(about = "A CLI client to mine arguments and their relations from social media posts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[command(flatten)]
    pub log: Log,
}

#[derive(Args)]
pub struct Log {
    #[arg(long = "log-level")]
    #[arg(help = "The logging level of the client (DEBUG, INFO, ERROR...)")]
    pub level: Option<String>,
}

#[derive(Subcommand)]
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

#[derive(Args, Clone, Deserialize)]
pub struct Model {
    #[arg(long = "model-name")]
    #[arg(help = "The name and tag of the Large Language Model used")]
    pub name: Option<String>,
    #[arg(long = "model-endpoint")]
    #[arg(help = "The actual address of the Large Language Model used")]
    pub endpoint: Option<String>,
    #[arg(long)]
    #[arg(help = "The path to a configuration to load the model configuration from")]
    pub model_file: Option<PathBuf>,
}
