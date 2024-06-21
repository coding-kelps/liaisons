use reqwest;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;

use crate::models::SummarizedInfo;
use crate::clients::llm;
use crate::configuration::settings;

/// Describe a client to a Large Language Model running via an Ollama server.
pub struct Client {
    /// The underlying http client.
    client: reqwest::Client,
    /// The name of the Large Language Model request inference from. 
    model: String,
    /// The address of Ollama server.
    uri: String,
    /// The options of the inference.
    options: Option<ModelOptions>,
}

/// Describe the configuration of the Ollama client.
#[derive(Deserialize)]
struct Configuration {

}

#[derive(Deserialize, Serialize, Clone)]
struct ModelOptions {
    /// The temperature of the inference.
    temperature: f64,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("api error: {0}")]
    ApiError(String),

    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
}

/// Describe the body for a generation http request to a Large Language
/// Model running via an Ollama server.
#[derive(Serialize)]
struct GenerateRequestBody {
    // the model name.
    model: String,
    // the prompt to generate a response for.
    prompt: String,
    // system message to (overrides what is defined in the Modelfile).
    system: Option<String>,
    // additional model parameters listed in the documentation for the Modelfile
    // (https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)
    // such as temperature
    options: Option<ModelOptions>,
    // if false the response will be returned as a single response object,
    // rather than a stream of objects.
    stream: bool,
}

/// Describe the response body of the Ollama server in case of an error
/// following an generation http request.
#[derive(Deserialize)]
struct GenerateErrorResponseBody {
    /// The error message.
    error: String,
}

/// Describe a successful response body from the Ollama server following a
/// generation http request.
#[derive(Deserialize)]
struct GenerateSuccessResponseBody {
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    created_at: String,
    response: String,
    #[allow(dead_code)]
    context: Vec<u32>,
    #[allow(dead_code)]
    total_duration: u64,
    #[allow(dead_code)]
    load_duration: u64,
    #[allow(dead_code)]
    prompt_eval_count: u32,
    #[allow(dead_code)]
    prompt_eval_duration: u64,
    #[allow(dead_code)]
    eval_count: u32,
    #[allow(dead_code)]
    eval_duration: u64,
}

impl Client {
    pub fn new(cfg: &settings::OllamaCfg) -> Self {
        Client {
            client: reqwest::Client::new(),
            model: cfg.model.clone(),
            uri: cfg.uri.clone(),
            options: None,
        }
    }
}

impl llm::ClientTrait for Client {
    async fn summarize(&self, prompt: &settings::Prompt, raw: String) -> Result<SummarizedInfo, llm::Error> {
        let req_body = GenerateRequestBody {
            model: self.model.clone(),
            prompt: format!("{}\n{}", prompt.prompt.clone(), raw),
            system: prompt.system.clone(),
            options: self.options.clone(),
            stream: false,
        };

        let res: reqwest::Response = self.client
            .post(format!("{}/api/generate", &self.uri))
            .json(&req_body)
            .send()
            .await
            .map_err(Error::from)?;

        if !res.status().is_success() {
            let body_parsing = res
                .json::<GenerateErrorResponseBody>()
                .await;
            
            match body_parsing {
                Ok(body) => Err(llm::Error::Ollama(Error::ApiError(body.error))),
                Err(e) => Err(llm::Error::Ollama(Error::ApiError(format!("failed to parse response body: {}", e)))),
            }
        } else {
            let body_parsing = res
                .json::<GenerateSuccessResponseBody>()
                .await;
            
            match body_parsing {
                Ok(body) => Ok(SummarizedInfo { name: String::from("salut"), summary: body.response }),
                Err(e) => Err(llm::Error::Ollama(Error::ApiError(format!("failed to parse response body: {}", e)))),
            }
        }
    }
}
