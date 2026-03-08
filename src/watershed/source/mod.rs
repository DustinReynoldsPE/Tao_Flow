pub mod claude_cli;
pub mod llama;
pub mod tmux_pane;

use async_trait::async_trait;

use crate::error::FlowError;

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatRole {
    User,
    Assistant,
}

#[async_trait]
pub trait LlmSource: Send + Sync {
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError>;
}

#[cfg(test)]
pub mod mock {
    use super::*;

    pub struct MockSource {
        pub response: String,
    }

    impl MockSource {
        pub fn new(response: impl Into<String>) -> Self {
            Self {
                response: response.into(),
            }
        }
    }

    #[async_trait]
    impl LlmSource for MockSource {
        async fn complete(
            &self,
            _system: &str,
            _messages: &[ChatMessage],
        ) -> Result<String, FlowError> {
            Ok(self.response.clone())
        }
    }

    pub struct EchoSource;

    #[async_trait]
    impl LlmSource for EchoSource {
        async fn complete(
            &self,
            _system: &str,
            messages: &[ChatMessage],
        ) -> Result<String, FlowError> {
            let last = messages
                .iter()
                .rfind(|m| m.role == ChatRole::User)
                .map(|m| m.content.clone())
                .unwrap_or_default();
            Ok(last)
        }
    }

    pub struct DrySource;

    #[async_trait]
    impl LlmSource for DrySource {
        async fn complete(
            &self,
            _system: &str,
            _messages: &[ChatMessage],
        ) -> Result<String, FlowError> {
            Err(FlowError::SpringFailure {
                name: "dry".into(),
                reason: "The well is dry.".into(),
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[tokio::test]
        async fn mock_source_returns_response() {
            let source = MockSource::new("The Tao flows.");
            let result = source.complete("system", &[]).await.unwrap();
            assert_eq!(result, "The Tao flows.");
        }

        #[tokio::test]
        async fn echo_source_echoes_last_user_message() {
            let source = EchoSource;
            let messages = vec![
                ChatMessage {
                    role: ChatRole::User,
                    content: "What is water?".into(),
                },
                ChatMessage {
                    role: ChatRole::Assistant,
                    content: "Water is life.".into(),
                },
                ChatMessage {
                    role: ChatRole::User,
                    content: "Tell me more.".into(),
                },
            ];
            let result = source.complete("system", &messages).await.unwrap();
            assert_eq!(result, "Tell me more.");
        }

        #[tokio::test]
        async fn dry_source_fails() {
            let source = DrySource;
            let result = source.complete("system", &[]).await;
            assert!(result.is_err());
        }
    }
}
