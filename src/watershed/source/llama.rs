use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use super::{ChatMessage, LlmSource};
use crate::error::FlowError;
use crate::water::Role;

/// llama.cpp server via its OpenAI-compatible API.
pub struct LlamaSource {
    client: Client,
    base_url: String,
    model: String,
}

impl LlamaSource {
    pub fn new(base_url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into(),
            model: model.into(),
        }
    }
}

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<ApiMessage>,
}

#[derive(Serialize)]
struct ApiMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct CompletionResponse {
    choices: Vec<Choice>,
}

#[derive(Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Deserialize)]
struct ResponseMessage {
    content: String,
}

fn to_api_messages(system: &str, messages: &[ChatMessage]) -> Vec<ApiMessage> {
    let mut out = Vec::new();
    if !system.is_empty() {
        out.push(ApiMessage {
            role: "system".into(),
            content: system.into(),
        });
    }
    for msg in messages {
        out.push(ApiMessage {
            role: match msg.role {
                Role::User => "user".into(),
                Role::Assistant => "assistant".into(),
            },
            content: msg.content.clone(),
        });
    }
    out
}

#[async_trait]
impl LlmSource for LlamaSource {
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError> {
        let url = format!(
            "{}/v1/chat/completions",
            self.base_url.trim_end_matches('/')
        );

        let request = CompletionRequest {
            model: self.model.clone(),
            messages: to_api_messages(system, messages),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await
            .map_err(|e| FlowError::SpringFailure {
                name: format!("llama({})", self.model),
                reason: format!("HTTP error: {e}"),
            })?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(FlowError::SpringFailure {
                name: format!("llama({})", self.model),
                reason: format!("API error {status}: {body}"),
            });
        }

        let parsed: CompletionResponse =
            response
                .json()
                .await
                .map_err(|e| FlowError::SpringFailure {
                    name: format!("llama({})", self.model),
                    reason: format!("Failed to parse response: {e}"),
                })?;

        let text = parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .unwrap_or_default();

        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_messages_include_system() {
        let msgs = vec![ChatMessage {
            role: Role::User,
            content: "hello".into(),
        }];
        let api = to_api_messages("you are wise", &msgs);
        assert_eq!(api.len(), 2);
        assert_eq!(api[0].role, "system");
        assert_eq!(api[1].role, "user");
    }

    #[test]
    fn api_messages_skip_empty_system() {
        let msgs = vec![ChatMessage {
            role: Role::User,
            content: "hello".into(),
        }];
        let api = to_api_messages("", &msgs);
        assert_eq!(api.len(), 1);
        assert_eq!(api[0].role, "user");
    }
}
