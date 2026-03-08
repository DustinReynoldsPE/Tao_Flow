pub mod anthropic;
pub mod claude_cli;

use async_trait::async_trait;

use crate::error::FlowError;

/// A message sent to an underground source.
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

/// The underground source -- the aquifer that feeds each spring.
///
/// Every spring draws from an underground source. The source
/// is hidden, deep, formless. What matters is the water that
/// emerges, not the rock it passes through.
///
/// "We shape clay into a pot, but it is the emptiness inside
/// that holds whatever we want." -- Tao Te Ching, Chapter 11
#[async_trait]
pub trait LlmSource: Send + Sync {
    /// Draw water from the source.
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError>;
}

#[cfg(test)]
pub mod mock {
    use super::*;

    /// A mock source that returns a predetermined response.
    /// For testing -- the spring flows without calling any API.
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

    /// A mock source that echoes the user's last message.
    /// Useful for testing that input flows through correctly.
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

    /// A mock source that always fails -- a dry well.
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
