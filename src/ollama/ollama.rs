use std::sync::mpsc;

use async_stream::stream;
use futures_util::{Stream, pin_mut};
use serde_json::Value;

use crate::structs::{
    error::OllamaError,
    model::Model,
    output_format::OutputFormat,
    request::{ChatMessage, OllamaRequest},
    response::ResponseStreamToken,
};

use super::http::{check_url, send_get, send_streaming_req};

#[derive(Debug, Clone)]
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
    pub async fn get_models(&self) -> Result<Vec<Model>, OllamaError> {
        let json_text = send_get(&format!("{}/api/tags", self.url)).await?;

        let json: Value =
            serde_json::from_str(&json_text).map_err(|_| OllamaError::new("Invalid JSON!"))?;

        let models: Vec<Model> = serde_json::from_value(json.get("models").unwrap().to_owned())
            .map_err(|_| OllamaError::new("Invalid JSON!"))?;

        Ok(models)
    }
    pub async fn get_loaded_models(&self) -> Result<Vec<Model>, OllamaError> {
        let json_text = send_get(&format!("{}/api/ps", self.url)).await?;

        let json: Value =
            serde_json::from_str(&json_text).map_err(|_| OllamaError::new("Invalid JSON!"))?;

        let models: Vec<Model> = serde_json::from_value(json.get("models").unwrap().to_owned())
            .map_err(|_| OllamaError::new("Invalid JSON!"))?;

        Ok(models)
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
