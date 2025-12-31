use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::output_format::OutputFormat;

#[derive(Serialize, Deserialize, Default)]
pub struct OllamaRequest {
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<ChatMessage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<OutputFormat>,

    pub stream: bool,
    pub think: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub thinking: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
}

impl ChatMessage {
    pub fn new(role: ChatRole, content: &str) -> ChatMessage {
        ChatMessage {
            role,
            content: content.to_string(),
            thinking: None,
            images: Vec::new(),
        }
    }
    pub fn new_with_images(role: ChatRole, content: &str, images: Vec<String>) -> ChatMessage {
        ChatMessage {
            role,
            content: content.to_string(),
            thinking: None,
            images,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    User,
    Assistant,
    System,
}
