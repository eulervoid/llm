use reqwest::header::{HeaderMap, AUTHORIZATION, CONTENT_TYPE};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use thiserror::Error;

pub struct OpenAI {
    api_key: String,
    client: Client,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    System,
    User,
    Assistant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageChoice {
    pub message: Message,
    pub finish_reason: String,
    pub index: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub usage: Usage,
    pub choices: Vec<MessageChoice>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}

#[derive(Debug, Error)]
pub enum OAIError {
    #[error("deserialization failed")]
    SerdeError(#[from] serde_json::error::Error),
    #[error("request failed")]
    ReqwestError(#[from] reqwest::Error),
}

impl OpenAI {
    pub fn new(api_key: &str) -> OpenAI {
        let client = Client::new();
        OpenAI {
            api_key: api_key.to_owned(),
            client,
        }
    }

    pub async fn get_chat_completion(
        &self,
        model: &str,
        messages: &[Message],
    ) -> Result<Response, OAIError> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers.insert(
            AUTHORIZATION,
            format!("Bearer {}", self.api_key).parse().unwrap(),
        );

        let body = serde_json::json!({
            "model": model,
            "messages": messages
        });

        let response_text = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .headers(headers)
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        let response: Response = serde_json::from_str(&response_text)?;

        Ok(response)
    }
}
