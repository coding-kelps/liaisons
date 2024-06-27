mod configuration;
mod clients;
mod subcommands;
mod models;

use configuration::*;
use subcommands::{predict, summarize};
use tokio;
use tracing_subscriber;
use tracing;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match load_configuration(&cli) {
        Ok(s) => {
            setup_logger(&s.log);

            match &cli.command {
                Some(Commands::SummarizeArguments { file, system, prompt }) => {
                    let cfg = summarize::SummarizeArgumentCfg {
                        llm_cfg: s.llm,
                        repo_cfg: s.repository,
                        prompt: Prompt {
                            system: system.clone().or(s.prompts.summary.system),
                            prompt: prompt.clone().unwrap_or(s.prompts.summary.prompt),
                        },
                        file_path: file.to_path_buf(),
                    };

                    if let Err(ref e) = summarize::summarize_arguments(cfg)
                        .await {
                            tracing::error!("arguments summarize failed: {}", e);
                    }
                },
                Some(Commands::PredictRelations { args_id, system, prompt }) => {
                    let cfg = predict::PredictRelationCfg {
                        llm_cfg: s.llm,
                        repo_cfg: s.repository,
                        prompt: Prompt {
                            system: system.clone().or(s.prompts.predict.system),
                            prompt: prompt.clone().unwrap_or(s.prompts.predict.prompt),
                        },
                        args_id: args_id.clone(),
                    };

                    if let Err(ref e) = predict::predict_relations(cfg)
                        .await {
                        tracing::error!("arguments relation prediction failed: {}", e);
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
    match tracing_subscriber::EnvFilter::try_new(cfg.level.clone()) {
        Ok(filter) => {
            let subscriber = tracing_subscriber::fmt()
                // Use a more compact, abbrievated log format
                .compact()
                // Display source code file paths
                .with_file(true)
                // Display source code line numbers
                .with_line_number(true)
                // Display the thread ID an event was recorded on
                .with_thread_ids(true)
                // Don't display the event's target (module path)
                .with_target(false)
                // Set level filter
                .with_env_filter(filter)
                // Finish building the subscriber
                .finish();
    
            tracing::subscriber::set_global_default(subscriber).unwrap();
        },
        _ => {
            setup_default_logger();
            
            tracing::warn!("failed to setup logger level");
        },
    };
}

// Setup the env_logger with a default `WARN` filter level.
fn setup_default_logger() {
    let filter = tracing_subscriber::EnvFilter::try_new("liaisons=info")
        .unwrap();

    let subscriber = tracing_subscriber::fmt()
        // Use a more compact, abbrievated log format
        .compact()
        // Display source code file paths
        .with_file(true)
        // Display source code line numbers
        .with_line_number(true)
        // Display the thread ID an event was recorded on
        .with_thread_ids(true)
        // Don't display the event's target (module path)
        .with_target(false)
        // Set level filter
        .with_env_filter(filter)
        // Finish building the subscriber
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();
}
