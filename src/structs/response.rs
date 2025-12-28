use serde::{Deserialize, Serialize};

use super::request::ChatMessage;

#[derive(Debug,Serialize, Deserialize)]
pub struct ResponseStream{
    pub model: String,
    pub created_at: String,
   

    pub response: Option<String>,
    
    pub message: ChatMessage,

    pub done: bool,
}
