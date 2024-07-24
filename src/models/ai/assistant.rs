use serde::{Deserialize, Serialize};

use super::chatgpt::Message;

const THREAD_ENDPOINT: &str = "openai/threads";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssistantRequest {
    pub model: String,
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub temperature: f64,
    pub top_p: f64,
    pub response_format: String,
}

impl AssistantRequest {
    pub fn new(model: String, name: String, instructions: String) -> Self {
        Self {
            model,
            name: name.clone(),
            description: name,
            instructions,
            temperature: 1.0,
            top_p: 1.0,
            response_format: String::from("auto"),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssistantResponse {
    pub id: String,
    pub object: String,
    pub created_at: u64,
    pub name: String,
    pub description: String,
    pub model: String,
    pub instructions: String,
    pub tools: Vec<String>,
    pub top_p: f64,
    pub temperature: f64,
    pub file_ids: Vec<String>,
    pub response_format: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Thread {
    pub messages: Vec<Message>,

}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadRunRequest {
    pub thread: Thread,
    pub assistant_id: String
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadRunResponse {
    pub id: String,
    pub status: String,
    pub thread_id: String,
    pub assistant_id: String,
    pub object: String,
    pub created_at: u64,
    pub expires_at: u64,
    pub cancelled_at: Option<u64>,
    pub failed_at: Option<u64>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThreadMessageResponse {
    id: String,
    content: String
}
