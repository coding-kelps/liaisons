mod configuration;
mod clients;
mod subcommands;
mod models;

use configuration::*;
use subcommands::{predict, summarize};
use tokio;
use tracing;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match load_configuration(&cli) {
        Ok(s) => {
            let _guard = logger::setup(&s.log)
                .unwrap_or_else(|e| {
                    panic!("Exiting due to an error at the logger setup {}", e);
                });

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
