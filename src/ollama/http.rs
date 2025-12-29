use std::fmt::format;

use async_stream::stream;
use reqwest::{Client, Response, StatusCode};

use crate::structs::{error::OllamaError, request::OllamaRequest, response::ResponseStreamToken};

use super::ollama::ChatMessageResponseStreamToken;

/// Checks if url loads and returns code 200
pub async fn check_url(url: &str) -> bool {
    let client = Client::new();

    let req = client.get(url).send().await;
    match req {
        Ok(res) => res.status() == StatusCode::OK,
        Err(_) => false,
    }
}

/// Sends Streaming request
pub async fn send_streaming_req(
    url: &str,
    body: OllamaRequest,
) -> Result<ChatMessageResponseStreamToken, OllamaError> {
    let client = reqwest::Client::new();

    let mut response = client
        .post(url)
        .body(serde_json::to_string(&body).unwrap())
        .send()
        .await
        .map_err(|_| OllamaError::new("Error while requesting /api/chat!"))?;

    let s = stream! {
        while let Some(chunk) = response.chunk().await.map_err(|_| OllamaError::new("Invalid chunk!"))? {

            let response_stream_res = serde_json::from_slice::<ResponseStreamToken>(chunk.iter().as_slice());


            match response_stream_res {
               Ok(response_stream) => {
                   yield Ok(response_stream);
               },
               Err(e) => {
                    yield Err(OllamaError::new("Invalid JSON response!"));
                    break;
                }

           }
       }
    };

    Ok(Box::pin(s))
}

/// Send GET request
pub async fn send_get(url: &str) -> Result<String, OllamaError> {
    let client = reqwest::Client::new();

    let response = client
        .get(url)
        .send()
        .await
        .map_err(|_| OllamaError::new(&format!("Error while requesting {}!", url)))?;

    Ok(response.text().await.unwrap())
}
