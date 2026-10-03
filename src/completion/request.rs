use crate::completion::config::{
    Completion, 
    CompletionError, 
    CompletionRequest, 
    CompletionResponse,
    HEADER, 
    URL,
};

use crate::completion::parse::{ parse_response, };

pub struct RequestSlm {
    url: String,
}

impl RequestSlm {
    pub fn new() -> Self {
        Self {
            url: URL.to_string(),
        }
    }

    pub fn complete(
        &self,
        prefix: &str,
    ) -> Result<Vec<Completion>, CompletionError> {

        let response = self.request(prefix)?;

        parse_response(prefix, response)
    }

    fn request(
        &self,
        prefix: &str,
    ) -> Result<CompletionResponse, CompletionError> {

        let prompt = format!("{HEADER}Input: {prefix}\n1.");

        let request = CompletionRequest {
            prompt,
            temperature: 0.0,
            max_tokens: 48,
            stop: vec!["\n3.", "\n\n", "Input:"],
            cache_prompt: true,
        };

        let response = ureq::post(&self.url)
            .header("Content-Type", "application/json")
            .send_json(&request)
            .map_err(|e| CompletionError::Http(e.to_string()))?;

        response
            .into_body()
            .read_json()
            .map_err(|e| CompletionError::Json(e.to_string()))
    }
}