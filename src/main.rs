mod configuration;
mod clients;
mod subcommands;
mod models;

use configuration::*;
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    setup_logger(&cli.log);

    match &cli.command {
        Some(Commands::SummarizeArguments { file, model }) => {
            let _ = subcommands::summarize::summarize_arguments(file.to_path_buf(), model.model_file.clone().unwrap())
                .await;
        },
        Some(Commands::PredictRelations { file, model: _ }) => {
            let _ = subcommands::predict::predict_relations(file.to_path_buf())
                .await;
        },
        None => (),
    }
}

// Setup the env_logger logger from a Log configuration
fn setup_logger(cfg: &Log) {
    match &cfg.level {
        Some(level_str) => {
            match level_str.parse::<log::LevelFilter>() {
                Ok(level) => {
                    env_logger::builder()
                        .filter_level(level)
                        .init();
                },
                _ => {
                    setup_default_logger();
                    
                    log::warn!("failed to setup logger level");
                },
            };
        },
        _ => setup_default_logger(),
    };
}

// Setup the env_logger with a default `WARN` filter level.
fn setup_default_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();
}
