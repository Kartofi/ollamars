use std::sync::mpsc;

use async_stream::stream;
use futures_util::{Stream, pin_mut};

use crate::structs::{
    error::OllamaError,
    request::{ChatMessage, OllamaRequest},
    response::ResponseStream,
};

use super::http::check_url;

#[derive(Debug)]
pub struct Ollama {
    pub url: String,
}

/// A stream of `ChatMessageResponse` objects
pub type ChatMessageResponseStream =
    std::pin::Pin<Box<dyn tokio_stream::Stream<Item = Result<ResponseStream, ()>> + Send>>;

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
        prompt: &str,
        think: bool,
    ) -> ChatMessageResponseStream {
        let client = reqwest::Client::new();

        let request = OllamaRequest {
            model: model.to_string(),
            messages: Some(vec![ChatMessage::new(
                crate::structs::request::ChatRole::User,
                prompt,
            )]),
            prompt: None,
            stream: true,
            think: think,
        };

        let mut response = client
            .post(format!("{}/api/chat", self.url))
            .body(serde_json::to_string(&request).unwrap())
            .send()
            .await
            .unwrap();

        let s = stream! {
               while let Some(chunk) = response.chunk().await.unwrap() {


               let json_res = serde_json::from_slice::<ResponseStream>(chunk.iter().as_slice());


                match json_res {
                   Ok(json) => {
                       yield Ok(json);
                   },
                   Err(e) => {println!("{:?}",e)}
               }
           }
        };

        Box::pin(s)
    }
}
