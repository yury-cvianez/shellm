use crate::completion::config::{
    Completion, 
    CompletionError, 
    CompletionResponse,
};

use crate::completion::response::{ valid, };

pub fn parse_response(
    prefix: &str,
    response: CompletionResponse,
) -> Result<Vec<Completion>, CompletionError> {

    let choice = response
        .choices
        .first()
        .ok_or_else(|| {
            CompletionError::InvalidResponse(
                "response contains no choices".into()
            )
        })?;

    let raw = format!("1.{}", choice.text);

    let mut completions: Vec<Completion> = Vec::with_capacity(2);

    for line in raw.lines() {

        let Some((number, candidate)) = line.split_once('.') else {
            continue;
        };

        if number != "1" && number != "2" {
            continue;
        }

        let candidate = candidate.trim();

        if !valid(prefix, candidate) {
            continue;
        }

        let suffix = &candidate[prefix.len()..];

        if completions.iter().any(|c| c.text == suffix) {
            continue;
        }

        completions.push(Completion {
            text: suffix.to_string(),
        });
    }

    Ok(completions)
}