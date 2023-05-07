use super::chatgpt::{ChatCompletionsRequest, RoleMessage};
use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub sender: String,
    pub role: String,
    pub timestamp_milli: i64,
    pub content: String,
}

impl Message {
    pub fn new(content: String) -> Self {
        Self {
            sender: "".into(),
            role: "system".into(),
            timestamp_milli: Utc::now().timestamp_millis(),
            content,
        }
    }
}

impl Into<ChatCompletionsRequest> for Message {
    fn into(self) -> ChatCompletionsRequest {
        ChatCompletionsRequest {
            model: "gpt-3.5-turbo".to_string(),
            temperature: 1.0,
            messages: vec![RoleMessage { role: self.role, content: self.content }],
        }
    }
}
