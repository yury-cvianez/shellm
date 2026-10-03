use serde::{Deserialize, Serialize};

pub const URL: &str = "http://127.0.0.1:7070/v1/completions";

pub const HEADER: &str = r#"Complete each partial Linux command.
Give 2 different valid, complete commands that start exactly with the input,
one per line, most likely first.

Input: systemctl sta
1. systemctl status
2. systemctl start

Input: tar -xz
1. tar -xzf
2. tar -xzvf

Input: cat /etc/host
1. cat /etc/hosts
2. cat /etc/hostname

Input: pip inst
1. pip install
2. pip install -r requirements.txt

"#;

#[derive(Debug)]
pub struct Completion {
    pub text: String,
}

#[derive(Debug)]
pub enum CompletionError {
    Http(String),
    Json(String),
    InvalidResponse(String),
}


#[derive(Serialize)]
pub struct CompletionRequest<'a> {
    pub prompt: String,
    pub temperature: f32,
    pub max_tokens: u32,
    pub stop: Vec<&'a str>,
    pub cache_prompt: bool,
}

#[derive(Deserialize)]
pub struct CompletionResponse {
    pub choices: Vec<Choice>,
}

#[derive(Deserialize)]
pub struct Choice {
    pub text: String,
}