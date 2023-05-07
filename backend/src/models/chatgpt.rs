use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct ChatGPTConf {
    pub url: Option<String>,
    pub api_key: String,
    pub org_id: Option<String>,
}

#[allow(dead_code)]
pub struct ChatGPTClient {
    url: String,
    api_key: String,
    org_id: Option<String>,
    client: Client,
}

impl ChatGPTClient {
    pub fn new(conf: &ChatGPTConf) -> Result<Self, &'static str> {
        if conf.api_key.is_empty() {
            return Err("api_key is empty");
        }

        let url = match &conf.url {
            Some(v) => v.clone(),
            None => "https://api.openai.com".to_string(),
        };

        Ok(Self {
            url,
            api_key: conf.api_key.clone(),
            org_id: conf.org_id.clone(),
            client: Client::new(),
        })
    }
}
