use std::path::PathBuf;
use tokio::fs;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::clients;
use crate::configuration::settings::{LLMClient, Prompt, Neo4jClient};
use crate::clients::ClientTrait;
use crate::models::Argument;

/// Describe the content of the raw arguments passed through the input JSON
/// file.
#[derive(Deserialize, Serialize, Debug)]
struct InputContentData {
    /// The user-generated web content to extract arguments from as a raw
    /// string.
    content: String,
}

pub async fn summarize_arguments(llm: &LLMClient, prompt: &Prompt, _neo4j: & Neo4jClient, file_path: PathBuf) -> Result<(), ()> {
    let content_inputs: Vec<InputContentData> = {
        let data = fs::read_to_string(file_path).await.unwrap();

        // TODO: CHANGE THIS UNWRAP !!!
        serde_json::from_str(&data).unwrap()
    };
    let client = clients::Client::new(&llm);

    let mut arguments: Vec<Argument> = Vec::with_capacity(content_inputs.len());

    // I've decided to send to send each elements as a separate requests to simplify
    // the development, and the design of the request handling.
    for input in content_inputs {
        match client.summarize(prompt, input.content).await {
            Ok(argument) => {
                log::info!("sucessfully retrieved argument");

                arguments.push(argument);
            },
            Err(e) => {
                log::error!("{}", e);
            },
        };
    }

    let j = serde_json::to_string(&arguments)
        .expect("failed deserialize retrieved arguments");

    fs::write("data.json", j)
        .await
        .expect("failed to write output to document");

    Ok(())
}

#[cfg(test)]
mod tests {
    mod retrieve_arguments {
    }
}
