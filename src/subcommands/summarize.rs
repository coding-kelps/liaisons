use std::path::PathBuf;
use tokio::fs;
use serde::{Deserialize, Serialize};
use serde_json;
use thiserror::Error as ThisError;

use crate::clients::{llm, repository, llm::ClientTrait, repository::RepositoryTrait};
use crate::configuration::settings::{LLMCfg, RepositoryCfg, Prompt};
use crate::models::Argument;

/// Describe the content of the raw arguments passed through the input JSON
/// file.
#[derive(Deserialize, Serialize, Debug)]
struct InputContentData {
    /// The user-generated web content to extract arguments from as a raw
    /// string.
    content: String,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("repository error: {0}")]
    RepositoryError(#[from] repository::Error),
}

pub struct SummarizeArgumentCfg {
    pub llm_cfg: LLMCfg,
    pub repo_cfg: RepositoryCfg,
    pub prompt: Prompt,
    pub file_path: PathBuf,
}

pub async fn summarize_arguments(cfg: SummarizeArgumentCfg) -> Result<(), Error> {
    let content_inputs: Vec<InputContentData> = {
        let data = fs::read_to_string(cfg.file_path).await?;

        serde_json::from_str(&data)?
    };

    let llm_client = llm::Client::new(&cfg.llm_cfg);
    let mut repo_client = repository::Repository::new(&cfg.repo_cfg)
        .await?;

    // I've decided to send to send each elements as a separate requests to
    // simplify the development, and the design of the request handling.
    for input in content_inputs {
        match llm_client.summarize(&cfg.prompt, input.content.clone()).await {
            Ok(info) => {
                tracing::info!("sucessfully summarized argument");

                let argument = Argument::new(info, input.content);

                if let Err(ref e) = repo_client.add_argument(argument).await {
                    tracing::error!("failed to create argument in Neo4j database: {}", e);
                };
            },
            Err(e) => {
                tracing::error!("{}", e);
            },
        };
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    mod retrieve_arguments {
    }
}
