mod configuration;
mod clients;
mod subcommands;
mod models;

use configuration::*;
use subcommands::{predict, summarize};
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

                    let cfg = summarize::SummarizeArgumentCfg {
                        llm_cfg: settings.llm,
                        repo_cfg: settings.repository,
                        prompt: settings.prompts.summary,
                        file_path: file.to_path_buf(),
                    };

                    if let Err(ref e) = summarize::summarize_arguments(cfg)
                        .await {
                            log::error!("arguments summarize failed: {}", e);
                    }
                },
                Some(Commands::PredictRelations { system, prompt }) => {
                    settings.prompts.predict = Prompt {
                        system: system.clone().or(settings.prompts.predict.system),
                        prompt: prompt.clone().unwrap_or(settings.prompts.predict.prompt),
                    };

                    let cfg = predict::PredictRelationCfg {
                        llm_cfg: settings.llm,
                        repo_cfg: settings.repository,
                        prompt: settings.prompts.predict,
                    };

                    if let Err(ref e) = predict::predict_relations(cfg)
                        .await {
                        log::error!("arguments relation prediction failed: {}", e);
                    }
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
