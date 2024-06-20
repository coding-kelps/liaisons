mod configuration;
mod clients;
mod subcommands;
mod models;

use configuration::*;
use tokio;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let mut settings: Settings;

    match Settings::new(&cli.cfg_file_path) {
        Ok(loaded_settings) => {
            settings = loaded_settings;

            settings.merge_to_cli(&cli);

            setup_logger(&settings.log);

            match &cli.command {
                Some(Commands::SummarizeArguments { file, system, prompt }) => {
                    settings.prompts.summary = Prompt {
                        system: system.clone().or(settings.prompts.summary.system),
                        prompt: prompt.clone().unwrap_or(settings.prompts.summary.prompt),
                    };

                    let _ = subcommands::summarize::summarize_arguments(&settings.llm, &settings.prompts.summary, &&settings.neo4j, file.to_path_buf())
                        .await;
                },
                Some(Commands::PredictRelations { file, system, prompt }) => {
                    settings.prompts.predict = Prompt {
                        system: system.clone().or(settings.prompts.predict.system),
                        prompt: prompt.clone().unwrap_or(settings.prompts.predict.prompt),
                    };

                    let _ = subcommands::predict::predict_relations(file.to_path_buf())
                        .await;
                },
                None => (),
            };
        }
        Err(e) =>
            println!("failed to load configuration file: {}", e),
    };
}

// Setup the env_logger logger from a Log configuration
fn setup_logger(cfg: &configuration::settings::Log) {
    match cfg.level.parse::<log::LevelFilter>() {
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
}

// Setup the env_logger with a default `WARN` filter level.
fn setup_default_logger() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Warn)
        .init();
}
