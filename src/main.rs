use bytes::Bytes;
use ollama::ollama::Ollama;
use reqwest::{self, StatusCode};
use structs::response::ResponseStream;
use tokio_stream::StreamExt;

pub mod ollama;
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ollama = Ollama::new("http://192.168.0.13:11434").await.unwrap();
  
    let mut res = ollama.chat_stream("gemma3:4b", "How are you?", false).await;

    while let Some(opa) = res.next().await {
        println!("{:?}", opa);
    }
    // Make a request and get the response

    Ok(())
}
