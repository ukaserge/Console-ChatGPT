use openai::OpenAI;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct Request {
    prompt: String,
    temperature: f64,
    max_tokens: usize,
    top_p: f64,
    frequency_penalty: f64,
    presence_penalty: f64,
}

#[derive(Debug, Deserialize)]
struct Response {
    id: String,
    prompt: String,
    completions: Vec<String>,
}

pub struct OpenAI {
    token: String,
}

impl OpenAI {
    pub fn new() -> Self {
        let token = std::env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN not set");

        OpenAI { token }
    }

    pub fn get_response(&self, prompt: &str) -> Result<String, Box<dyn Error>> {
        let request = Request {
            prompt: prompt.to_string(),
            temperature: 0.5,
            max_tokens: 256,
            top_p: 1.0,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        };

        let client = reqwest::Client::new();
        let response: Response = client
            .post("https://api.openai.com/v1/completions")
            .bearer_auth(&self.token)
            .json(&request)
            .send()?
            .json()?;

        Ok(response.completions[0].clone())
    }
}
