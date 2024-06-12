mod configuration;
mod clients;
mod subcommands;
mod models;

use configuration::*;
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.log.level {
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

    match &cli.command {
        Some(Commands::RetrieveArguments { file, model }) =>
            subcommands::retrieve::retrieve_arguments(file.to_path_buf(), model).await,
        Some(Commands::PredictRelations { file, model: _ }) => subcommands::predict::predict_relations(file.to_path_buf()).await,
        None => Ok(()),
    }.unwrap()
}

fn setup_default_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();
}
