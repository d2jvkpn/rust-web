use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub sender: String,
    pub timestamp_milli: i64,
    pub content: String,
}

impl Message {
    pub fn new(content: String) -> Self {
        Self { sender: "system".into(), timestamp_milli: Utc::now().timestamp_millis(), content }
    }
}
