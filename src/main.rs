use core::time;
use std::{collections::HashMap, hash::Hash, time::Instant};

use bytes::Bytes;
use ollama::ollama::Ollama;

use reqwest::{self, StatusCode};
use structs::{
    output_format::{FieldType, FieldTypeInfo, OutputFormat},
    request::{ChatMessage, ChatRole},
};
use tokio_stream::StreamExt;

pub mod ollama;
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ollama = Ollama::new("http://192.168.0.13:11434").await.unwrap();

    println!("{:?}",ollama.get_loaded_models().await);

    let history = vec![
        ChatMessage::new(
            ChatRole::System,
            "You are 100 years old! Use the response to response to the user!",
        ),
    
        ChatMessage::new(ChatRole::User, "What is the weather in Plovdiv Bulgaria?"),
    ];
    let mut properties: HashMap<String, FieldTypeInfo> = HashMap::new();
    properties.insert(
        "response".to_string(),
        FieldTypeInfo {
            field_type: FieldType::String,
            ..Default::default()
        },
    );
    properties.insert(
        "tools".to_string(),
        FieldTypeInfo {
            field_type: FieldType::Boolean,
            ..Default::default()
        },
    );
    let output_format = OutputFormat::new(properties, Vec::new());

    let mut res = ollama
        .chat_stream("orieg/gemma3-tools:1b", history, false)
        .await
        .unwrap();

    let mut content = "".to_string();

    while let Some(opa) = res.next().await {
        let opa = opa.unwrap();
        println!("{:?}", opa);
        content.push_str(&opa.message.unwrap().content);
    }
    println!("{}", content);
    // Make a request and get the response

    Ok(())
}
