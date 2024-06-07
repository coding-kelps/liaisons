use reqwest;
use serde::Deserialize;

use crate::clients::ollama::Error;
use crate::models::Argument;

#[derive(Deserialize)]
struct RetrieveErrorResponse {
    error: String,
}

#[derive(Deserialize)]
struct RetrieveSuccessResponse {
    argument: Argument,
}

pub struct Client {
    client: reqwest::Client,
    endpoint: String,
}

impl From<&str> for Client {
    fn from(endpoint: &str) -> Self {
        Client {
            client: reqwest::Client::new(),
            endpoint: endpoint.to_string(),
        }
    }
}

impl Client {
    pub async fn retrieve(&self, content: String) -> Result<Argument, Error> {
        let res: Result<reqwest::Response, reqwest::Error> = self.client.post(&self.endpoint)
            .json(&content)
            .send()
            .await;

        match res {
            Ok(response) => {
                if !response.status().is_success() {
                    let body_parsing = response.json::<RetrieveErrorResponse>()
                        .await;
                    
                    match body_parsing {
                        Ok(body) => Err(Error::ApiError(body.error)),
                        Err(_) => Err(Error::ApiError(String::from("failed to parse response body"))),
                    }
                } else {
                    let body_parsing = response
                        .json::<RetrieveSuccessResponse>()
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
