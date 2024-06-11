mod settings;
mod clients;
mod subcommands;
mod models;

use settings::*;
use tokio;
use env_logger;
use log;

fn setup_default_logger() {
    env_logger::builder()
    .filter_level(log::LevelFilter::Warn)
    .init();

    log::warn!("failed to load logging level, setup logger to default \"WARN\" level");
}

#[tokio::main]
async fn main() {
    let cfg = Settings::new()
        .expect("failed to load configuration");

    match cfg.log.level {
        Some(level_str) => {
            match level_str.parse::<log::LevelFilter>() {
                Ok(level) => {
                    env_logger::builder()
                        .filter_level(level)
                        .init();
                },
                _ => setup_default_logger(),
            };
        },
        _ => setup_default_logger(),
    }

    match &cfg.command {
        Some(Commands::RetrieveArguments { file, model }) =>
            subcommands::retrieve::retrieve_arguments(file.to_path_buf(), model).await,
        Some(Commands::PredictRelations { file, model: _ }) => subcommands::predict::predict_relations(file.to_path_buf()).await,
        None => Ok(()),
    }.unwrap()
}
