pub use clap::{Parser, Subcommand, Args};
use std::path::PathBuf;

/// Describe the Command Line Interface arguments. 
#[derive(Parser)]
#[command(version = "0.1.0", about, long_about = None)]
#[command(about = "A CLI client to mine arguments and their relations from social media posts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
    #[command(flatten)]
    pub log: Log,
    #[arg(long = "config")]
    #[arg(help = "The path to a configuration file for the client")]
    pub cfg_file_path: Option<String>,
}

#[derive(Args)]
pub struct Log {
    #[arg(long = "log-level")]
    #[arg(help = "The logging level of the client (DEBUG, INFO, ERROR...)")]
    pub level: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(alias("summarize"))]
    #[clap(about = "Summarize arguments from a given JSON source file gathering social media posts")]
    SummarizeArguments {
        #[arg(short, long)]
        #[arg(help = "The file path to the JSON source file of social media posts")]
        file: PathBuf,
        #[arg(short, long)]
        #[arg(help = "")]
        system: Option<String>,
        #[arg(short, long)]
        #[arg(help = "")]
        prompt: Option<String>,
    },

    #[clap(alias("predict"))]
    #[clap(about = "")]
    PredictRelations {
        #[arg(long, num_args = 2..)]
        #[arg(help = "")]
        args_id: Vec<u32>,
        #[arg(long)]
        #[arg(help = "")]
        system: Option<String>,
        #[arg(long)]
        #[arg(help = "")]
        prompt: Option<String>,
    }
}
