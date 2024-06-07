use std::path::PathBuf;
use tokio::fs;
use serde::{Deserialize, Serialize};
use serde_json;

use crate::clients::ollama;
use crate::models::Argument;

#[derive(Deserialize, Serialize, Debug)]
struct InputContentData {
    content: String,
}

pub async fn retrieve_arguments(file_path: PathBuf, endpoint: &str) -> Result<(), ()> {
    let content_inputs: Vec<InputContentData> = {
        let data = fs::read_to_string(file_path).await.unwrap();

        serde_json::from_str(&data).unwrap()
    };
    let ollama_client = ollama::Client::from(endpoint);

    let mut arguments: Vec<Argument> = Vec::with_capacity(content_inputs.len());

    // I've decided to send to send each elements as a separate requests to simplify
    // the development, and the design of the request handling.
    for input in content_inputs {
        match ollama_client.retrieve(input.content).await {
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
