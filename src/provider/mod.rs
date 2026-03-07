pub mod anthropic;
pub mod claude_cli;
pub mod tmux;

use async_trait::async_trait;

use crate::error::FlowError;

/// A message sent to an LLM provider.
#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

/// Role in a chat conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatRole {
    User,
    Assistant,
}

/// The empty pot -- any LLM provider can fill this shape.
///
/// "We shape clay into a pot, but it is the emptiness inside
/// that holds whatever we want." -- Tao Te Ching, Chapter 11
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Send a completion request to the LLM.
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError>;
}

#[cfg(test)]
pub mod mock {
    use super::*;

    /// A mock provider that returns a predetermined response.
    /// For testing -- the spring flows without calling any API.
    pub struct MockProvider {
        pub response: String,
    }

    impl MockProvider {
        pub fn new(response: impl Into<String>) -> Self {
            Self {
                response: response.into(),
            }
        }
    }

    #[async_trait]
    impl LlmProvider for MockProvider {
        async fn complete(
            &self,
            _system: &str,
            _messages: &[ChatMessage],
        ) -> Result<String, FlowError> {
            Ok(self.response.clone())
        }
    }

    /// A mock provider that echoes the user's last message.
    /// Useful for testing that input flows through correctly.
    pub struct EchoProvider;

    #[async_trait]
    impl LlmProvider for EchoProvider {
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

    /// A mock provider that always fails -- a dry well.
    pub struct DryProvider;

    #[async_trait]
    impl LlmProvider for DryProvider {
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
        async fn mock_provider_returns_response() {
            let provider = MockProvider::new("The Tao flows.");
            let result = provider.complete("system", &[]).await.unwrap();
            assert_eq!(result, "The Tao flows.");
        }

        #[tokio::test]
        async fn echo_provider_echoes_last_user_message() {
            let provider = EchoProvider;
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
            let result = provider.complete("system", &messages).await.unwrap();
            assert_eq!(result, "Tell me more.");
        }

        #[tokio::test]
        async fn dry_provider_fails() {
            let provider = DryProvider;
            let result = provider.complete("system", &[]).await;
            assert!(result.is_err());
        }
    }
}
