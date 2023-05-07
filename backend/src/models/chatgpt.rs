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

    url_chat_completions: String,
    token: String,
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

            url_chat_completions: url.clone() + "/v1/chat/completions",
            token: "Bearer ".to_owned() + &conf.api_key,
        })
    }

    // TODO: handle error
    pub async fn chat_completions(
        &self,
        req: &ChatCompletionsRequest,
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

        let res = match self
            .client
            .post(&self.url_chat_completions)
            .header("authorization", &self.token)
            // .header("Content-Type", "application/json")
            .json(req)
            .send()
            .await
        {
            Ok(v) => v,
            Err(e) => {
                println!("!!! chat_completions request: {:?}\n", e);
                return Err(e);
            }
        };

        if res.status().is_success() {
            println!("!!! chat_completions status: {}", res.status());
        }

        let res = match res.json::<ChatCompletionsResponse>().await {
            Ok(v) => v,
            Err(e) => {
                println!("!!! chat_completions unmarshal: {:?}\n", e);
                return Err(e);
            }
        };

        Ok(res)
    }
}
