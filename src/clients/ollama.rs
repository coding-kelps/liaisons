use reqwest;
use serde::{Deserialize, Serialize};
use serde_yaml;
use thiserror::Error as ThisError;

use crate::models::Argument;
use crate::clients;

/// Describe a client to a Large Language Model running via an Ollama server.
pub struct Client {
    /// The underlying http client.
    client: reqwest::Client,
    /// The configuration of the client.
    configuration: Configuration,
}

/// Describe the configuration of the Ollama client.
#[derive(Deserialize)]
struct Configuration {
    /// The name of the Large Language Model request inference from. 
    model_name: String,
    /// The address of Ollama server.
    address: String,
    /// The prompt 
    prompt: String,
    /// The system prompt.
    system: Option<String>,
    /// The options of the inference.
    options: Option<ModelOptions>,
}

#[derive(Deserialize, Serialize, Clone)]
struct ModelOptions {
    /// The temperature of the inference.
    temperature: f64,
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("deserialization error: {0}")]
    DeserializationError(#[from] serde_yaml::Error),

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
    /// Create a new Ollama client from a `serde_yaml::Value`.
    /// 
    /// # Arguments
    /// 
    /// * `value` - A reference to an `serde_yaml::Value` describing the
    ///     configuration of the Ollama client.
    /// 
    /// # Errors
    /// 
    /// This function will return an error if:
    /// 
    /// * The passed value is invalid.
    /// 
    pub fn from_value(value: & serde_yaml::Value) -> Result<Self, Error> {
        Ok(Client {
            client: reqwest::Client::new(),
            configuration: serde_yaml::from_value::<Configuration>(value.to_owned())?,
        })
    }
}

impl clients::ClientTrait for Client {
    async fn summarize(&self, raw: String) -> Result<Argument, clients::Error> {
        let req_body = GenerateRequestBody {
            model: self.configuration.model_name.clone(),
            prompt: format!("{}\n{}", self.configuration.prompt.clone(), raw),
            system: self.configuration.system.clone(),
            options: self.configuration.options.clone(),
            stream: false,
        };

        let res: reqwest::Response = self.client
            .post(format!("{}/api/generate", &self.configuration.address))
            .json(&req_body)
            .send()
            .await
            .map_err(Error::from)?;

        if !res.status().is_success() {
            let body_parsing = res
                .json::<GenerateErrorResponseBody>()
                .await;
            
            match body_parsing {
                Ok(body) => Err(clients::Error::Ollama(Error::ApiError(body.error))),
                Err(e) => Err(clients::Error::Ollama(Error::ApiError(format!("failed to parse response body: {}", e)))),
            }
        } else {
            let body_parsing = res
                .json::<GenerateSuccessResponseBody>()
                .await;
            
            match body_parsing {
                Ok(body) => Ok(Argument{ summary: body.response, raw: raw }),
                Err(e) => Err(clients::Error::Ollama(Error::ApiError(format!("failed to parse response body: {}", e)))),
            }
        }
    }
}
