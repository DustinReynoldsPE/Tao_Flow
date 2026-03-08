use crate::error::FlowError;
use crate::water::{Message, Ocean, Rain, River, Role, Stream, Vapor};
use crate::watershed::Watershed;

pub struct TaoFlow {
    watershed: Watershed,
    vapor: Vapor,
}

impl TaoFlow {
    pub fn new(watershed: Watershed) -> Self {
        Self {
            watershed,
            vapor: Vapor::default(),
        }
    }

    pub async fn flow(&mut self, user_input: &str) -> Result<String, FlowError> {
        let mut rain = Rain::new(user_input, self.vapor.clone());
        let streams = self.watershed.receive_rain(&mut rain).await;

        if streams.is_empty() {
            return Err(FlowError::Drought);
        }

        let river = Self::simple_merge(streams);
        let ocean = Ocean::new(river.content);
        self.update_vapor(&rain, &ocean);

        Ok(ocean.content)
    }

    /// Pick the deepest stream. Full confluence comes later.
    fn simple_merge(streams: Vec<Stream>) -> River {
        debug_assert!(!streams.is_empty());

        if streams.len() == 1 {
            let stream = streams.into_iter().next().unwrap();
            return River::from_single(stream.source, stream.content);
        }

        // Select the deepest stream
        let best = streams
            .iter()
            .max_by(|a, b| a.depth.partial_cmp(&b.depth).unwrap())
            .unwrap();

        let tributaries: Vec<String> = streams.iter().map(|s| s.source.clone()).collect();

        River {
            content: best.content.clone(),
            tributaries,
            eddies: Vec::new(),
            clarity: best.clarity,
        }
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
    use crate::watershed::{DesertSpring, MountainSpring};
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

    #[tokio::test]
    async fn rain_flows_to_ocean() {
        let watershed = Watershed::new(vec![
            mountain_spring("The Tao is the way."),
            desert_spring("It's the way."),
        ]);
        let mut tao = TaoFlow::new(watershed);
        let result = tao.flow("What is the Tao?").await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn droplet_uses_only_desert() {
        let watershed = Watershed::new(vec![
            mountain_spring("Should not appear."),
            desert_spring("Hello!"),
        ]);
        let mut tao = TaoFlow::new(watershed);
        let result = tao.flow("hi").await.unwrap();
        assert_eq!(result, "Hello!");
    }

    #[tokio::test]
    async fn simple_merge_picks_deepest() {
        // Create two streams with different depths
        let short = Stream::new("desert", "Yes.");
        let long = Stream::new("mountain", "word ".repeat(100));

        let river = TaoFlow::simple_merge(vec![short, long]);
        assert_eq!(river.tributary_count(), 2);
        // The longer response has more depth
        assert!(river.content.contains("word"));
    }

    #[tokio::test]
    async fn vapor_accumulates_across_flows() {
        let watershed = Watershed::new(vec![desert_spring("Response.")]);
        let mut tao = TaoFlow::new(watershed);

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
        let mut tao = TaoFlow::new(watershed);
        let result = tao.flow("hello").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn single_stream_passes_through() {
        let stream = Stream::new("mountain", "Only voice.");
        let river = TaoFlow::simple_merge(vec![stream]);
        assert_eq!(river.tributary_count(), 1);
        assert_eq!(river.content, "Only voice.");
        assert!(!river.has_eddies());
    }
}
