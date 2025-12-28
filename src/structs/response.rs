use serde::{Deserialize, Serialize};

use super::request::ChatMessage;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub model: String,
    pub created_at: String,

    pub response: Option<String>,
    pub thinking: Option<String>,

    pub total_duration: Option<i32>,
    pub load_duration: Option<i32>,

    pub message: Option<ChatMessage>,

    pub done: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ResponseStreamToken {
    pub model: String,
    pub created_at: String,

    pub message: Option<ChatMessage>,
    pub response: Option<String>,
    pub thinking: Option<String>,

    pub done: bool,
}
