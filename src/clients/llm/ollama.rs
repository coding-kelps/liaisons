use reqwest;
use serde::{Deserialize, Serialize};
use thiserror::Error as ThisError;
use regex::Regex;

use crate::models::{self, SummarizedInfo};
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

    #[error("response parsing error: {0}")]
    ResponseParsingError(String),
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
//    #[allow(dead_code)]
//    model: String,
//    #[allow(dead_code)]
//    created_at: String,
    response: String,
//    #[allow(dead_code)]
//    context: Vec<u32>,
//    #[allow(dead_code)]
//    total_duration: u64,
//    #[allow(dead_code)]
//    load_duration: u64,
//    #[allow(dead_code)]
//    prompt_eval_count: u32,
//    #[allow(dead_code)]
//    prompt_eval_duration: u64,
//    #[allow(dead_code)]
//    eval_count: u32,
//    #[allow(dead_code)]
//    eval_duration: u64,
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

    fn parse_summarize_response(&self, response: String) -> Result<SummarizedInfo, Error> {
        let re = Regex::new(r"Title: (?<title>.*)\nSummary: (?<summary>.*)").unwrap();

        let Some(ref caps) = re.captures(&response) else {
            return Err(Error::ResponseParsingError(String::from("no element found in LLM response")))
        };

        let Some(title) = caps.name("title") else {
            return Err(Error::ResponseParsingError(String::from("no \"Title\" element found in LLM response")))
        };

        let Some(summary) = caps.name("summary") else {
            return Err(Error::ResponseParsingError(String::from("no \"Summary\" element found in LLM response")))
        };

        Ok(SummarizedInfo {
            title: String::from(title.as_str()),
            summary: String::from(summary.as_str())
        })
    }

    fn parse_predict_response(&self, response: String) -> Result<models::RelationType, Error> {
        let re = Regex::new(r"Relation: (?<relation>.*)").unwrap();

        let Some(ref caps) = re.captures(&response) else {
            return Err(Error::ResponseParsingError(String::from("no element found in LLM response")))
        };

        let Some(relation_type) = caps.name("relation") else {
            return Err(Error::ResponseParsingError(String::from("no \"Relation\" element found in LLM response")))
        };

        Ok(models::RelationType::from(relation_type.as_str()))
    }
}

impl llm::ClientTrait for Client {
    async fn summarize(&self, prompt: &settings::Prompt, raw: String) -> Result<SummarizedInfo, llm::Error> {
        let req_body = GenerateRequestBody {
            model: self.model.clone(),
            prompt: format!("{}\n\nArg:{}\n", prompt.prompt.clone(), raw),
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
                Ok(body) => Ok(self.parse_summarize_response(body.response)?),
                Err(e) => Err(llm::Error::Ollama(Error::ApiError(format!("failed to parse response body: {}", e)))),
            }
        }
    }

    async fn predict(&self, prompt: &settings::Prompt, arg_a: &models::Argument, arg_b: &models::Argument) -> Result<models::Relation, llm::Error> {
        let req_body = GenerateRequestBody {
            model: self.model.clone(),
            prompt: format!("{}\n\nArg1:{}\nArg2:{}\n", prompt.prompt.clone(), arg_a.raw, arg_b.raw),
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
                Ok(body) => {
                    let relation_type = self.parse_predict_response(body.response)?;

                    Ok(models::Relation {
                        arg_a_id: arg_a.id.unwrap(),
                        arg_b_id: arg_b.id.unwrap(),
                        relation_type: relation_type,
                        confidence: 1.0_f32,
                        explanation: String::from(""),
                    })
                },
                Err(e) => Err(llm::Error::Ollama(Error::ApiError(format!("failed to parse response body: {}", e)))),
            }
        }
    }
}
