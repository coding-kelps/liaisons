use std::path::PathBuf;
use clap::{Parser, Subcommand};
use tokio;
use env_logger;
mod subcommand;

// The structure that defines the parsed arguments
#[derive(Parser)]
#[command(version = "0.1.0", about, long_about = None)]
#[command(about = "A CLI client to mine arguments and their relations from social media posts", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(alias("retrieve"), alias("r"))]
    #[clap(about = "Retrieve arguments from a given JSON source file gathering social media posts")]
    RetrieveArguments {
        #[arg(short, long)]
        #[arg(help = "The file path to the JSON source file of social media posts")]
        file: PathBuf,
        #[arg(short, long)]
        #[arg(help = "The actual address of the Large Language Model to use to make inferences with")]
        #[clap(default_value = "http://localhost")]
        model_address: String,
    },

    #[clap(alias("predict"), alias("p"))]
    #[clap(about = "Predicts argument relations from a given JSON source file of retrieved arguments")]
    PredictRelations {
        #[arg(short, long)]
        #[arg(help = "The file path to the JSON source file of retrieved arguments")]
        file: PathBuf,
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::RetrieveArguments { file, model_address }) =>
            subcommand::retrieve::retrieve_arguments(file.to_path_buf(), model_address).await,
        Some(Commands::PredictRelations { file }) => subcommand::predict::predict_relations(file.to_path_buf()).await,
        None => Ok(()),
    }.unwrap()
}
