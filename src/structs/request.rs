use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize,Default)]
pub struct OllamaRequest {
    pub model: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub messages: Option<Vec<ChatMessage>>,

    pub stream: bool,
    pub think: bool,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

impl ChatMessage{
    pub fn new(role: ChatRole,content: &str) -> ChatMessage{
        ChatMessage{
            role: role,
            content: content.to_string()
        }
    }
}
#[derive(Serialize, Deserialize,Debug)]
pub enum ChatRole {
    #[serde(rename = "user")]
    User,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "system")]
    System,
}
