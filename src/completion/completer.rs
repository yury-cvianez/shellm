use crate::completion::config::{
    Completion, 
    CompletionError, 
};

pub trait Completer {
    fn complete(
        &self,
        input: &str,
    ) -> Result<Vec<Completion>, CompletionError>;
}