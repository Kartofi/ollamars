use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default)]
pub struct OllamaRequest {
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,

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
}

impl ChatMessage {
    pub fn new(role: ChatRole, content: &str) -> ChatMessage {
        ChatMessage {
            role: role,
            content: content.to_string(),
            thinking: None,
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ChatRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}

#[derive(Serialize, Deserialize, Debug,Default)]
#[serde(rename_all = "lowercase")]
pub enum FieldType {
    #[default]
    String,
    Integer,
    Boolean,
    
}

#[derive(Serialize, Deserialize, Debug,Default)]
pub struct TypeInfo {
    pub r#type: FieldType,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_str: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "enum")]
    pub enum_int: Option<Vec<i32>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputFormat {
    pub r#type: String,
    pub properties: HashMap<String, TypeInfo>,
    pub required: Vec<String>,
}
