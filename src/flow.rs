use crate::confluence::ConfluencePool;
use crate::error::FlowError;
use crate::still_lake::StillLake;
use crate::water::{Message, Ocean, Rain, Role, Vapor};
use crate::watershed::Watershed;

pub struct TaoFlow {
    watershed: Watershed,
    confluence: ConfluencePool,
    still_lake: StillLake,
    vapor: Vapor,
}

impl TaoFlow {
    pub fn new(watershed: Watershed, confluence: ConfluencePool, still_lake: StillLake) -> Self {
        Self {
            watershed,
            confluence,
            still_lake,
            vapor: Vapor::default(),
        }
    }

    pub async fn flow(&mut self, user_input: &str) -> Result<String, FlowError> {
        let mut rain = Rain::new(user_input, self.vapor.clone());
        let streams = self.watershed.receive_rain(&mut rain).await;

        if streams.is_empty() {
            return Err(FlowError::Drought);
        }

        let river = self.confluence.merge(streams, &rain.raw_input).await?;
        let ocean = self.still_lake.settle(river, &rain.raw_input).await?;
        self.update_vapor(&rain, &ocean);

        Ok(ocean.content)
    }

    fn update_vapor(&mut self, rain: &Rain, ocean: &Ocean) {
        self.vapor.conversation_history.push(Message {
            role: Role::User,
            content: rain.raw_input.clone(),
        });
        self.vapor.conversation_history.push(Message {
            role: Role::Assistant,
            content: ocean.content.clone(),
        });
    }

    pub fn vapor(&self) -> &Vapor {
        &self.vapor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::watershed::source::mock::{DrySource, MockSource};
    use crate::watershed::spring::SpringConfig;
    use crate::watershed::{DesertSpring, ForestSpring, MountainSpring};
    use std::collections::HashMap;

    fn mountain_spring(response: &str) -> Box<dyn crate::watershed::Spring> {
        let mut affinities = HashMap::new();
        affinities.insert("philosophy".into(), 0.9);
        let config = SpringConfig {
            name: "mountain".into(),
            nature: "deep reasoning".into(),
            affinities,
        };
        Box::new(MountainSpring::new(
            config,
            Box::new(MockSource::new(response)),
        ))
    }

    fn desert_spring(response: &str) -> Box<dyn crate::watershed::Spring> {
        let mut affinities = HashMap::new();
        affinities.insert("quick_answers".into(), 0.9);
        let config = SpringConfig {
            name: "desert".into(),
            nature: "speed, efficiency".into(),
            affinities,
        };
        Box::new(DesertSpring::new(
            config,
            Box::new(MockSource::new(response)),
        ))
    }

    fn forest_spring(response: &str) -> Box<dyn crate::watershed::Spring> {
        let mut affinities = HashMap::new();
        affinities.insert("narrative".into(), 0.9);
        affinities.insert("empathy".into(), 0.8);
        let config = SpringConfig {
            name: "forest".into(),
            nature: "creativity, narrative, empathy".into(),
            affinities,
        };
        Box::new(ForestSpring::new(
            config,
            Box::new(MockSource::new(response)),
        ))
    }

    fn test_confluence(response: &str) -> ConfluencePool {
        ConfluencePool::new(Box::new(MockSource::new(response)))
    }

    fn test_lake(response: &str) -> StillLake {
        StillLake::new(Box::new(MockSource::new(response)))
    }

    #[tokio::test]
    async fn rain_flows_to_ocean() {
        let watershed = Watershed::new(vec![
            mountain_spring("The Tao is the way."),
            desert_spring("It's the way."),
        ]);
        let confluence = test_confluence("The Tao is the way.");
        let lake = test_lake("The Tao is the way, settled.");
        let mut tao = TaoFlow::new(watershed, confluence, lake);
        let result = tao.flow("What is the Tao?").await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn droplet_uses_only_desert() {
        let watershed = Watershed::new(vec![
            mountain_spring("Should not appear."),
            desert_spring("Hello!"),
        ]);
        // Single stream: clarity 1.0, lake does nothing (wu wei)
        let confluence = test_confluence("unused");
        let lake = test_lake("should not be called");
        let mut tao = TaoFlow::new(watershed, confluence, lake);
        let result = tao.flow("hi").await.unwrap();
        assert_eq!(result, "Hello!");
    }

    #[tokio::test]
    async fn three_springs_merge_and_settle() {
        let watershed = Watershed::new(vec![
            mountain_spring("Deep analysis of the question."),
            desert_spring("Quick, direct answer."),
            forest_spring("A story about the answer."),
        ]);
        // MockSource returns same response for all calls (detection, weaving, settling)
        let woven = "A woven response from three perspectives.";
        let confluence = test_confluence(woven);
        let lake = test_lake(woven);
        let mut tao = TaoFlow::new(watershed, confluence, lake);

        let result = tao
            .flow("Explain the nature of water in philosophy and storytelling and practice")
            .await
            .unwrap();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn vapor_accumulates_across_flows() {
        let watershed = Watershed::new(vec![desert_spring("Response.")]);
        let confluence = test_confluence("unused");
        let lake = test_lake("should not be called");
        let mut tao = TaoFlow::new(watershed, confluence, lake);

        tao.flow("First").await.unwrap();
        tao.flow("Second").await.unwrap();

        assert_eq!(tao.vapor().conversation_history.len(), 4);
        assert_eq!(tao.vapor().conversation_history[0].content, "First");
        assert_eq!(tao.vapor().conversation_history[2].content, "Second");
    }

    #[tokio::test]
    async fn drought_when_all_springs_dry() {
        let watershed = Watershed::new(vec![Box::new(MountainSpring::new(
            SpringConfig {
                name: "mountain".into(),
                nature: "deep".into(),
                affinities: HashMap::new(),
            },
            Box::new(DrySource),
        )) as Box<dyn crate::watershed::Spring>]);
        let confluence = test_confluence("unused");
        let lake = test_lake("unused");
        let mut tao = TaoFlow::new(watershed, confluence, lake);
        let result = tao.flow("hello").await;
        assert!(result.is_err());
    }
}
