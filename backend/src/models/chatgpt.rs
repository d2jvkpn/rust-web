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
    client: reqwest::Client,

    auth: String,
    url_chat_completions: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionsRequest {
    pub model: String,
    pub temperature: f32,
    pub messages: Vec<RoleMessage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ChatCompletionsResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<Choice>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    pub message: RoleMessage,
    pub finish_reason: String,
    pub index: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RoleMessage {
    pub role: String,
    pub content: String,
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
            url: url.clone(),
            api_key: conf.api_key.clone(),
            org_id: conf.org_id.clone(),
            client: reqwest::Client::new(),

            url_chat_completions: url + "/v1/chat/completions",
            auth: "Bearer ".to_owned() + &conf.api_key,
        })
    }

    // TODO: handle error
    pub async fn chat_completions(
        &self,
        ccr: &ChatCompletionsRequest,
    ) -> Result<ChatCompletionsResponse, reqwest::Error> {
        /*
        let res = self
            .client
            .post(&self.url_chat_completions)
            .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await?
            .json::<ChatCompletionsResponse>()
            .await?;
         */

        let mut req =
            self.client.post(&self.url_chat_completions).header("Authorization", &self.auth);

        if let Some(v) = &self.org_id {
            req = req.header("OpenAI-Organization", v);
        }

        // .header("Content-Type", "application/json")
        let res = req.json(ccr).send().await?;

        if !res.status().is_success() {
            // TODO: return an reqwest error
            println!("!!! chat_completions status: {}", res.status());
        }

        res.json::<ChatCompletionsResponse>().await
    }
}
