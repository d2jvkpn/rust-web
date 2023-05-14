use super::chatgpt::{ChatCompletionsRequest, RoleMessage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub sender: String,
    pub role: String,
    pub timestamp_milli: i64,
    pub content: String,
}

#[allow(dead_code)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChatRecord {
    pub request_id: uuid::Uuid,
    pub user_id: i32,
    pub query: String,
    pub query_at: DateTime<Utc>,
    pub response: Option<String>,
    pub response_at: Option<DateTime<Utc>>,
}
