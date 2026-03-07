use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{ChatMessage, ChatRole, LlmProvider};
use crate::error::FlowError;

/// Anthropic API provider -- Claude models.
pub struct AnthropicProvider {
    client: Client,
    api_key: String,
    model: String,
    max_tokens: u32,
}

impl AnthropicProvider {
    pub fn new(api_key: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            api_key: api_key.into(),
            model: model.into(),
            max_tokens: 4096,
        }
    }

    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }
}

#[derive(Serialize)]
struct ApiRequest {
    model: String,
    max_tokens: u32,
    system: String,
    messages: Vec<ApiMessage>,
}

#[derive(Serialize)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct ApiResponse {
    content: Vec<ContentBlock>,
}

#[derive(Deserialize)]
struct ContentBlock {
    text: String,
}

impl From<&ChatMessage> for ApiMessage {
    fn from(msg: &ChatMessage) -> Self {
        ApiMessage {
            role: match msg.role {
                ChatRole::User => "user".into(),
                ChatRole::Assistant => "assistant".into(),
            },
            content: msg.content.clone(),
        }
    }
}

#[async_trait]
impl LlmProvider for AnthropicProvider {
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError> {
        let api_messages: Vec<ApiMessage> = messages.iter().map(ApiMessage::from).collect();

        let request = ApiRequest {
            model: self.model.clone(),
            max_tokens: self.max_tokens,
            system: system.to_string(),
            messages: api_messages,
        };

        let response = self
            .client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| FlowError::SpringFailure {
                name: self.model.clone(),
                reason: format!("HTTP error: {e}"),
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(FlowError::SpringFailure {
                name: self.model.clone(),
                reason: format!("API error {status}: {body}"),
            });
        }

        let api_response: ApiResponse =
            response
                .json()
                .await
                .map_err(|e| FlowError::SpringFailure {
                    name: self.model.clone(),
                    reason: format!("Failed to parse response: {e}"),
                })?;

        let text = api_response
            .content
            .into_iter()
            .map(|block| block.text)
            .collect::<Vec<_>>()
            .join("");

        Ok(text)
    }
}
