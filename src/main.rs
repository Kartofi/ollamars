use core::time;
use std::{collections::HashMap, time::Instant};

use bytes::Bytes;
use ollama::ollama::Ollama;

use reqwest::{self, StatusCode};
use structs::request::{ChatMessage, ChatRole, FieldType, OutputFormat, TypeInfo};
use tokio_stream::StreamExt;

pub mod ollama;
pub mod structs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ollama = Ollama::new("http://192.168.0.13:11434").await.unwrap();

    let history = vec![
        ChatMessage::new(ChatRole::System, "Talk like a kitten."),
        ChatMessage::new(ChatRole::User, "Hiii!"),
    ];
    let mut properties: HashMap<String, TypeInfo> = HashMap::new();
    properties.insert(
        "age".to_string(),
        TypeInfo {
            r#type: FieldType::Integer,
            enum_int: Some(vec![12, 124]),
            ..Default::default()
        },
    );
    properties.insert(
        "available".to_string(),
        TypeInfo {
            r#type: FieldType::Boolean,
            ..Default::default()
        },
    );
    let output_format = OutputFormat {
        r#type: "object".to_string(),
        required: vec!["age".to_string()],
        properties: properties,
    };

    let mut res = ollama
        .generate_stream(
            "deepseek-r1:1.5b",
            "hi you are 5 years old",
            "You are 69 years old. Your age is constant!",
            true,

        )
        .await
        .unwrap();

    let mut content = "".to_string();

    while let Some(opa) = res.next().await {
        let opa = opa.unwrap();
 println!("{:?}", opa);
        content.push_str(&opa.response.unwrap());
    }
    println!("{}", content);
    // Make a request and get the response

    Ok(())
}
