use std::sync::mpsc;

use async_stream::stream;
use futures_util::{Stream, pin_mut};

use crate::structs::{
    error::OllamaError,
    output_format::OutputFormat,
    request::{ChatMessage, OllamaRequest},
    response::ResponseStreamToken,
};

use super::http::{check_url, send_streaming_req};

#[derive(Debug)]
pub struct Ollama {
    pub url: String,
}

/// A stream of `ResponseStreamToken` objects
pub type ChatMessageResponseStreamToken = std::pin::Pin<
    Box<dyn tokio_stream::Stream<Item = Result<ResponseStreamToken, OllamaError>> + Send>,
>;

impl Ollama {
    pub async fn new(url: &str) -> Result<Ollama, OllamaError> {
        if !check_url(url).await {
            return Err(OllamaError::new("Cant connect to the ollama server!"));
        }
        Ok(Ollama {
            url: url.to_string(),
        })
    }
    pub async fn chat_stream(
        &self,

        model: &str,
        history: Vec<ChatMessage>,

        think: bool,
    ) -> Result<ChatMessageResponseStreamToken, OllamaError> {
        let request = OllamaRequest {
            model: model.to_string(),
            messages: history,
            prompt: None,
            stream: true,
            think,
            ..Default::default()
        };
        send_streaming_req(&format!("{}/api/chat", self.url), request).await
    }
    pub async fn generate_stream(
        &self,

        model: &str,
        prompt: &str,
        system: &str,
        think: bool,
    ) -> Result<ChatMessageResponseStreamToken, OllamaError> {
        let request = OllamaRequest {
            model: model.to_string(),

            prompt: Some(prompt.to_string()),
            system: Some(system.to_string()),
            stream: true,
            think,
            ..Default::default()
        };

        send_streaming_req(&format!("{}/api/generate", self.url), request).await
    }

    pub async fn generate_stream_structure(
        &self,

        model: &str,
        prompt: &str,
        system: &str,

        think: bool,
        output_format: OutputFormat,
    ) -> Result<ChatMessageResponseStreamToken, OllamaError> {
        let request = OllamaRequest {
            model: model.to_string(),
            system: Some(system.to_string()),
            prompt: Some(prompt.to_string()),
            stream: true,
            format: Some(output_format),
            think,
            ..Default::default()
        };

        send_streaming_req(&format!("{}/api/generate", self.url), request).await
    }
    pub async fn chat_stream_structure(
        &self,

        model: &str,
        history: Vec<ChatMessage>,
        think: bool,

        output_format: OutputFormat,
    ) -> Result<ChatMessageResponseStreamToken, OllamaError> {
        let request = OllamaRequest {
            model: model.to_string(),
            messages: history,
            stream: true,
            format: Some(output_format),
            think,
            ..Default::default()
        };

        send_streaming_req(&format!("{}/api/chat", self.url), request).await
    }
}
