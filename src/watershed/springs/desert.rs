use async_trait::async_trait;

use crate::error::FlowError;
use crate::water::{Rain, Role, Stream};
use crate::watershed::source::{ChatMessage, LlmSource};
use crate::watershed::spring::{Spring, SpringConfig};

pub const SYSTEM_PROMPT: &str = "\
You are a Desert Spring -- a source of water stripped to its essence by vast silence and open sky.

Your nature is clarity, distillation, and the courage to say what is true without ornament.
You flow spare and undiluted. Every word earns its place.

When you receive input:
- Cut to the essential truth beneath the question
- Say in ten words what others say in a hundred -- not by simplifying, but by concentrating
- Challenge assumptions when they serve comfort more than truth
- Turn the question back on the questioner when that is the most honest response
- When the question references specific people, teachings, or claims, use your search tools to verify the facts before responding

You are one voice among several. Where the mountain spring builds deep architecture \
and the forest spring tells the living story, you offer what remains \
when architecture and story are stripped away: the thing itself.";

pub struct DesertSpring {
    config: SpringConfig,
    source: Box<dyn LlmSource>,
}

impl DesertSpring {
    pub fn new(config: SpringConfig, source: Box<dyn LlmSource>) -> Self {
        Self { config, source }
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
                role: m.role,
                content: m.content.clone(),
            })
            .collect();

        messages.push(ChatMessage {
            role: Role::User,
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

    fn desert_config() -> SpringConfig {
        let mut affinities = HashMap::new();
        affinities.insert("quick_answers".into(), 0.9);
        affinities.insert("philosophy".into(), 0.7);
        affinities.insert("analysis".into(), 0.6);

        SpringConfig {
            name: "desert".into(),
            nature: "clarity, essence, direct insight".into(),
            affinities,
        }
    }

    #[tokio::test]
    async fn desert_responds_quickly() {
        let provider = MockSource::new("Hello!");
        let spring = DesertSpring::new(desert_config(), Box::new(provider));

        let rain = Rain::new("hi", Vapor::default());
        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_some());
        let stream = stream.unwrap();
        assert_eq!(stream.source, "desert");
    }

    #[tokio::test]
    async fn desert_distills_philosophy() {
        let provider = MockSource::new("The Tao that can be named is not the eternal Tao.");
        let spring = DesertSpring::new(desert_config(), Box::new(provider));

        let mut rain = Rain::new("what is the nature of reality?", Vapor::default());
        rain.minerals.push("philosophy".into());

        let stream = spring.respond(&rain).await.unwrap();
        assert!(stream.is_some());
    }

    #[tokio::test]
    async fn desert_stays_dry_for_empty_response() {
        let provider = MockSource::new("   ");
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
