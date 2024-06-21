use std::path::PathBuf;
use tokio::fs;
use serde::{Deserialize, Serialize};
use serde_json;

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

pub async fn summarize_arguments(llm_cfg: &LLMCfg, repo_cfg: &RepositoryCfg, prompt: &Prompt, file_path: PathBuf) -> Result<(), ()> {
    let content_inputs: Vec<InputContentData> = {
        let data = fs::read_to_string(file_path).await.unwrap();

        // TODO: CHANGE THIS UNWRAP !!!
        serde_json::from_str(&data).unwrap()
    };

    let llm_client = llm::Client::new(&llm_cfg);
    // TODO: CHANGE THIS UNWRAP !!!
    let mut repo_client = repository::Repository::new(&repo_cfg)
        .await
        .unwrap();

    // I've decided to send to send each elements as a separate requests to simplify
    // the development, and the design of the request handling.
    for input in content_inputs {
        match llm_client.summarize(prompt, input.content.clone()).await {
            Ok(info) => {
                log::info!("sucessfully summarized argument");

                let argument = Argument::new(info, input.content);

                if let Err(ref e) = repo_client.add_argument(argument).await {
                    log::error!("failed to create argument in Neo4j database: {}", e);
                };
            },
            Err(e) => {
                log::error!("{}", e);
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
