use reqwest;
use serde::{Deserialize, Serialize};
use serde_yaml;
use thiserror::Error as ThisError;

use crate::models::Argument;
use crate::clients;

#[derive(Serialize)]
struct GenerateRequestBody {
    // the model name
    model: String,
    // the prompt to generate a response for
    prompt: String,
    // system message to (overrides what is defined in the Modelfile)
    system: Option<String>,
    // additional model parameters listed in the documentation for the Modelfile
    // (https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)
    // such as temperature
    options: Option<ModelOptions>,
    // if false the response will be returned as a single response object,
    // rather than a stream of objects
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateErrorResponseBody {
    error: String,
}

#[derive(Deserialize)]
struct GenerateSuccessResponseBody {
//    model: String,
//    created_at: String,
    response: String,
//    context: Vec<u32>,
//    total_duration: u64,
//    load_duration: u64,
//    prompt_eval_count: u32,
//    prompt_eval_duration: u64,
//    eval_count: u32,
//    eval_duration: u64,
}

pub struct Client {
    client: reqwest::Client,
    configuration: Configuration,
}

#[derive(Deserialize)]
struct Configuration {
    model_name: String,
    address: String,
    prompt: String,
    system: Option<String>,
    options: Option<ModelOptions>,
}

#[derive(Deserialize, Serialize, Clone)]
struct ModelOptions {
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

impl Client {
    pub fn from_value(value: & serde_yaml::Value) -> Result<Self, Error> {
        Ok(Client {
            client: reqwest::Client::new(),
            configuration: serde_yaml::from_value::<Configuration>(value.to_owned())?,
        })
    }
}

impl clients::ClientTrait for Client {
    async fn retrieve(&self, content: String) -> Result<Argument, clients::Error> {
        let req_body = GenerateRequestBody {
            model: self.configuration.model_name.clone(),
            prompt: format!("{} {}", self.configuration.prompt.clone(), content),
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
                Ok(body) => Ok(Argument{ summary: body.response }),
                Err(e) => Err(clients::Error::Ollama(Error::ApiError(format!("failed to parse response body: {}", e)))),
            }
        }
    }
}
