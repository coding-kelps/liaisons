use reqwest;
use serde::{Deserialize, Serialize};

use crate::clients::ollama::Error;
use crate::models::Argument;

#[derive(Serialize)]
struct ModelOptions {
    temperature: f64,
}

#[derive(Serialize)]
struct GenerateRequestBody {
    // the model name
    model: String,
    // the prompt to generate a response for
    prompt: String,
    // system message to (overrides what is defined in the Modelfile)
    system: String,
    // template: the prompt template to use (overrides what is defined in the
    // Modelfile)
    template: String,
    // additional model parameters listed in the documentation for the Modelfile
    // (https://github.com/ollama/ollama/blob/main/docs/modelfile.md#valid-parameters-and-values)
    // such as temperature
    options: ModelOptions,
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
    argument: Argument,
}

pub struct Client {
    client: reqwest::Client,
    endpoint: String,
    model_name: String,
}

impl Client {
    pub fn new(endpoint: &str, model_name: &str) -> Self {
        Client {
            client: reqwest::Client::new(),
            endpoint: endpoint.to_string(),
            model_name: model_name.to_string(),
        } 
    }

    pub async fn retrieve(&self, _content: String) -> Result<Argument, Error> {
        let req_body = GenerateRequestBody {
            model: self.model_name.clone(),
            prompt: String::from("Hi gemma!"),
            system: String::from("Respond Kindly"),
            template: String::from("Hi charming person!"),
            options: ModelOptions {
                temperature: 0.7_f64,
            },
            stream: false,
        };

        let res: Result<reqwest::Response, reqwest::Error> = self.client.post(&self.endpoint)
            .json(&req_body)
            .send()
            .await;

        match res {
            Ok(response) => {
                if !response.status().is_success() {
                    let body_parsing = response.json::<GenerateErrorResponseBody>()
                        .await;
                    
                    match body_parsing {
                        Ok(body) => Err(Error::ApiError(body.error)),
                        Err(_) => Err(Error::ApiError(String::from("failed to parse response body"))),
                    }
                } else {
                    let body_parsing = response
                        .json::<GenerateSuccessResponseBody>()
                        .await;
                
                    match body_parsing {
                        Ok(body) => Ok(body.argument),
                        Err(_) => Err(Error::ApiError(String::from("failed to parse response body"))),
                    }
                }
            }
            Err(e) => Err(Error::RequestError(e))
        }
    }
}
