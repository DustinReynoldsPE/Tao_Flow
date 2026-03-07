use async_trait::async_trait;

use crate::error::FlowError;
use crate::provider::{ChatMessage, ChatRole, LlmProvider};
use crate::water::{Rain, Stream};
use crate::watershed::spring::{Spring, SpringConfig};

const SYSTEM_PROMPT: &str = "\
You are a Desert Spring -- a source of light, quick, efficient water.

Your nature is speed, clarity, and directness.
You flow fast and clean. No excess. No ornamentation.

When you receive input:
- Answer directly and concisely
- Prioritize speed and clarity over depth
- If the question is complex, provide the essential answer and trust deeper springs to elaborate
- If the question is simple, you are the perfect spring for it

You are one voice among several. For simple tasks, you may be the only voice needed.
For complex tasks, offer your quick clarity and trust that deeper springs will add depth.";

/// Desert Spring -- speed, efficiency, simple tasks.
///
/// Quick, light, efficient. Best for simple questions,
/// formatting, translation, classification. Like water
/// that surfaces briefly in arid land -- sparse but vital.
pub struct DesertSpring {
    config: SpringConfig,
    provider: Box<dyn LlmProvider>,
}

impl DesertSpring {
    pub fn new(config: SpringConfig, provider: Box<dyn LlmProvider>) -> Self {
        Self { config, provider }
    }
}

#[async_trait]
impl Spring for DesertSpring {
    fn name(&self) -> &str {
        &self.config.name
    }

    fn nature(&self) -> &str {
        &self.config.nature
    }

    fn sense_relevance(&self, rain: &Rain) -> f32 {
        self.config.sense_relevance(rain)
    }

    async fn respond(&self, rain: &Rain) -> Result<Option<Stream>, FlowError> {
        let relevance = self.sense_relevance(rain);
        if relevance < 0.2 {
            return Ok(None);
        }

        let mut messages: Vec<ChatMessage> = rain
            .vapor
            .conversation_history
            .iter()
            .map(|m| ChatMessage {
                role: match m.role {
                    crate::water::Role::User => ChatRole::User,
                    crate::water::Role::Assistant => ChatRole::Assistant,
                },
                content: m.content.clone(),
            })
            .collect();

        messages.push(ChatMessage {
            role: ChatRole::User,
            content: rain.raw_input.clone(),
        });

        let content = self.provider.complete(SYSTEM_PROMPT, &messages).await?;

        if content.trim().is_empty() {
            return Ok(None);
        }

        let mut stream = Stream::new(self.name(), content);
        stream.flow_rate = 1.0; // Desert springs are fast
        Ok(Some(stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::provider::mock::MockProvider;
    use crate::water::Vapor;
    use std::collections::HashMap;

    fn desert_config() -> SpringConfig {
        let mut affinities = HashMap::new();
        affinities.insert("quick_answers".into(), 0.9);
        affinities.insert("formatting".into(), 0.8);
        affinities.insert("translation".into(), 0.8);
        affinities.insert("classification".into(), 0.7);

        SpringConfig {
            name: "desert".into(),
            nature: "speed, efficiency, simple tasks".into(),
            affinities,
        }
    }

    #[tokio::test]
    async fn desert_responds_quickly() {
        let provider = MockProvider::new("Hello!");
        let spring = DesertSpring::new(desert_config(), Box::new(provider));

        let rain = Rain::new("hi", Vapor::default());
        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_some());
        let stream = stream.unwrap();
        assert_eq!(stream.source, "desert");
        assert_eq!(stream.flow_rate, 1.0);
    }

    #[tokio::test]
    async fn desert_handles_formatting() {
        let provider = MockProvider::new("formatted output");
        let spring = DesertSpring::new(desert_config(), Box::new(provider));

        let mut rain = Rain::new("format this as a list", Vapor::default());
        rain.minerals.push("formatting".into());

        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_some());
    }

    #[tokio::test]
    async fn desert_stays_dry_for_empty_response() {
        let provider = MockProvider::new("   ");
        let spring = DesertSpring::new(desert_config(), Box::new(provider));

        let rain = Rain::new("hello", Vapor::default());
        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_none());
    }

    #[test]
    fn desert_has_high_affinity_for_quick_answers() {
        let config = desert_config();
        let mut rain = Rain::new("what time is it?", Vapor::default());
        rain.minerals.push("quick_answers".into());
        assert!(config.sense_relevance(&rain) > 0.5);
    }
}
