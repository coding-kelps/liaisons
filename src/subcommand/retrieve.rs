use std::path::PathBuf;
use tokio::fs;
use serde::{Deserialize, Serialize};
use serde_json;
use reqwest::Client;
use log;

#[derive(Deserialize, Serialize, Debug)]
struct InputContentData {
    content: String,
}

#[derive(Deserialize)]
struct ErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct SuccessResponse {
    argument: Argument,
}

#[derive(Serialize, Deserialize)]
#[allow(dead_code)]
struct Argument {
    summary: String,
}

pub async fn retrieve_arguments(file_path: PathBuf, model_address: &str) -> Result<(), ()> {
    let content_inputs: Vec<InputContentData> = {
        let data = fs::read_to_string(file_path).await.unwrap();

        serde_json::from_str(&data).unwrap()
    };
    let mut arguments: Vec<Argument> = Vec::with_capacity(content_inputs.len());

    let client = Client::new();

    // I've decided to send to send each elements as a separate requests to simplify
    // the development, and the design of the request handling.
    for input in content_inputs {
        let res = client.post(model_address)
            .json(&input)
            .send()
            .await;

        match res {
            Ok(response) => {
                if !response.status().is_success() {
                    let body = response.json::<ErrorResponse>().await.unwrap();

                    log::error!("bad response: {}", body.error)
                } else {
                    let body: SuccessResponse = response.json::<SuccessResponse>().await.unwrap();

                    arguments.push(body.argument);
                }
            }
            Err(e) => log::error!("requests failed {:?}", e)
        }
    }

    let j = serde_json::to_string(&arguments).unwrap();

    fs::write("data.json", j).await.unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    mod retrieve_arguments {
    }
}
