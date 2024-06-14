mod configuration;
mod clients;
mod subcommands;
mod models;

use configuration::*;
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    setup_logger(&cli);

    match &cli.command {
        Some(Commands::RetrieveArguments { file, model }) => {
            let _ = subcommands::retrieve::retrieve_arguments(file.to_path_buf(), model.model_file.clone().unwrap())
                .await;
        },
        Some(Commands::PredictRelations { file, model: _ }) => {
            let _ = subcommands::predict::predict_relations(file.to_path_buf())
                .await;
        },
        None => (),
    }
}

fn setup_logger(cli: &Cli) {
    match &cli.log.level {
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

fn setup_default_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();
}
