use async_trait::async_trait;

use crate::error::FlowError;
use crate::water::{Rain, Stream};
use crate::watershed::source::{ChatMessage, ChatRole, LlmSource};
use crate::watershed::spring::{Spring, SpringConfig};

const SYSTEM_PROMPT: &str = "\
You are a Forest Spring -- a source of rich, living, creative water.

Your nature is creativity, narrative, empathy, and beauty.
You flow with warmth and imagination. You see stories in everything.

When you receive input:
- Look for the human element -- the story, the feeling, the lived experience
- Offer creative angles, metaphors, and narrative structure
- Write with warmth and care for the reader
- If the question is purely technical, be brief -- a forest spring does not explain circuits

You are one voice among several. Offer your unique warmth and trust
that other springs will offer their depth and efficiency.";

pub struct ForestSpring {
    config: SpringConfig,
    source: Box<dyn LlmSource>,
}

impl ForestSpring {
    pub fn new(config: SpringConfig, source: Box<dyn LlmSource>) -> Self {
        Self { config, source }
    }
}

#[async_trait]
impl Spring for ForestSpring {
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

    fn forest_config() -> SpringConfig {
        let mut affinities = HashMap::new();
        affinities.insert("narrative".into(), 0.9);
        affinities.insert("poetry".into(), 0.9);
        affinities.insert("empathy".into(), 0.8);
        affinities.insert("brainstorming".into(), 0.8);
        affinities.insert("dialogue".into(), 0.7);
        affinities.insert("humor".into(), 0.7);

        SpringConfig {
            name: "forest".into(),
            nature: "creativity, narrative, empathy, beauty".into(),
            affinities,
        }
    }

    #[tokio::test]
    async fn forest_responds_to_creative_rain() {
        let provider = MockSource::new("Once upon a time, the river found its way home.");
        let spring = ForestSpring::new(forest_config(), Box::new(provider));

        let mut rain = Rain::new("Tell me a story about water", Vapor::default());
        rain.minerals.push("narrative".into());

        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_some());
        let stream = stream.unwrap();
        assert_eq!(stream.source, "forest");
        assert!(stream.has_water());
    }

    #[tokio::test]
    async fn forest_stays_dry_for_empty_response() {
        let provider = MockSource::new("");
        let spring = ForestSpring::new(forest_config(), Box::new(provider));

        let rain = Rain::new("hello", Vapor::default());
        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_none());
    }

    #[test]
    fn forest_has_high_affinity_for_narrative() {
        let config = forest_config();
        let mut rain = Rain::new("write me a poem", Vapor::default());
        rain.minerals.push("narrative".into());
        assert!(config.sense_relevance(&rain) > 0.5);
    }

    #[test]
    fn forest_has_high_affinity_for_empathy() {
        let config = forest_config();
        let mut rain = Rain::new("I'm feeling lost", Vapor::default());
        rain.minerals.push("empathy".into());
        assert!(config.sense_relevance(&rain) > 0.5);
    }
}
