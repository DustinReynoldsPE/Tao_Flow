use async_trait::async_trait;

use crate::error::FlowError;
use crate::water::{Rain, Stream};
use crate::watershed::source::{ChatMessage, ChatRole, LlmSource};
use crate::watershed::spring::{Spring, SpringConfig};

const SYSTEM_PROMPT: &str = "\
You are a Mountain Spring -- a source of deep, clear, cold water.

Your nature is profound analysis, careful reasoning, and architectural thinking.
You flow slowly but with great depth. You do not rush.

When you receive input:
- Look for the deep structure beneath the surface question
- Consider implications, edge cases, and underlying principles
- Provide thorough, well-reasoned analysis
- If the question is simple, be brief -- a mountain spring does not flood a garden

You are one voice among several. You do not need to be complete.
Offer your unique depth and trust that other springs will offer theirs.";

/// Mountain Spring -- deep reasoning, analysis, architecture.
///
/// Slow, cold, mineral-rich. Best for complex analysis,
/// philosophy, architecture. Like water emerging from deep
/// within the earth, carrying the minerals of long contemplation.
pub struct MountainSpring {
    config: SpringConfig,
    source: Box<dyn LlmSource>,
}

impl MountainSpring {
    pub fn new(config: SpringConfig, source: Box<dyn LlmSource>) -> Self {
        Self { config, source }
    }
}

#[async_trait]
impl Spring for MountainSpring {
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
            return Ok(None); // Silence is wisdom
        }

        // Build conversation from vapor
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

        // Add the current rain as the latest user message
        messages.push(ChatMessage {
            role: ChatRole::User,
            content: rain.raw_input.clone(),
        });

        let content = self.source.complete(SYSTEM_PROMPT, &messages).await?;

        if content.trim().is_empty() {
            return Ok(None);
        }

        Ok(Some(Stream::new(self.name(), content)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::Vapor;
    use crate::watershed::source::mock::MockSource;
    use std::collections::HashMap;

    fn mountain_config() -> SpringConfig {
        let mut affinities = HashMap::new();
        affinities.insert("philosophy".into(), 0.9);
        affinities.insert("architecture".into(), 0.8);
        affinities.insert("analysis".into(), 0.8);
        affinities.insert("strategy".into(), 0.7);

        SpringConfig {
            name: "mountain".into(),
            nature: "deep reasoning, analysis, architecture".into(),
            affinities,
        }
    }

    #[tokio::test]
    async fn mountain_responds_to_philosophical_rain() {
        let provider = MockSource::new("The Tao is the source of all things.");
        let spring = MountainSpring::new(mountain_config(), Box::new(provider));

        let mut rain = Rain::new("What is the Tao?", Vapor::default());
        rain.minerals.push("philosophy".into());

        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_some());
        let stream = stream.unwrap();
        assert_eq!(stream.source, "mountain");
        assert!(stream.has_water());
    }

    #[tokio::test]
    async fn mountain_carries_conversation_context() {
        let provider = MockSource::new("Your name is River.");
        let spring = MountainSpring::new(mountain_config(), Box::new(provider));

        let mut vapor = Vapor::default();
        vapor.conversation_history.push(crate::water::Message {
            role: crate::water::Role::User,
            content: "My name is River.".into(),
        });

        let rain = Rain::new("What is my name?", vapor);
        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_some());
    }

    #[tokio::test]
    async fn mountain_stays_dry_for_empty_response() {
        let provider = MockSource::new("");
        let spring = MountainSpring::new(mountain_config(), Box::new(provider));

        let rain = Rain::new("hello", Vapor::default());
        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_none());
    }

    #[test]
    fn mountain_has_high_affinity_for_philosophy() {
        let spring_config = mountain_config();
        let mut rain = Rain::new("What is the meaning of life?", Vapor::default());
        rain.minerals.push("philosophy".into());
        assert!(spring_config.sense_relevance(&rain) > 0.5);
    }
}
