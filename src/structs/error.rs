#[derive(Debug)]
pub struct OllamaError{
    pub message: String,
}
impl OllamaError{
    pub fn new(message: &str) -> OllamaError{
        OllamaError {
            message: message.to_string(),
        }
    }
}
