pub mod confluence;
pub mod error;
pub mod flow;
pub mod pearl;
pub mod still_lake;
pub mod vessel;
pub mod water;
pub mod watershed;

pub use confluence::{ConfluencePool, Decomposer};
pub use error::FlowError;
pub use flow::TaoFlow;
pub use pearl::Pearl;
pub use still_lake::StillLake;
pub use water::{Ocean, Rain, River, Stream, Vapor};

#[cfg(test)]
pub mod test_springs {
    use crate::watershed::source::mock::MockSource;
    use crate::watershed::spring::SpringConfig;
    use crate::watershed::{DesertSpring, ForestSpring, MountainSpring, Spring};
    use std::collections::HashMap;

    pub fn mountain(response: &str) -> Box<dyn Spring> {
        let mut affinities = HashMap::new();
        affinities.insert("philosophy".into(), 0.9);
        affinities.insert("architecture".into(), 0.8);
        affinities.insert("analysis".into(), 0.7);
        Box::new(MountainSpring::new(
            SpringConfig {
                name: "mountain".into(),
                nature: "deep reasoning, analysis, architecture".into(),
                affinities,
            },
            Box::new(MockSource::new(response)),
        ))
    }

    pub fn desert(response: &str) -> Box<dyn Spring> {
        let mut affinities = HashMap::new();
        affinities.insert("quick_answers".into(), 0.9);
        affinities.insert("formatting".into(), 0.7);
        affinities.insert("code".into(), 0.6);
        Box::new(DesertSpring::new(
            SpringConfig {
                name: "desert".into(),
                nature: "speed, directness, efficiency".into(),
                affinities,
            },
            Box::new(MockSource::new(response)),
        ))
    }

    pub fn forest(response: &str) -> Box<dyn Spring> {
        let mut affinities = HashMap::new();
        affinities.insert("narrative".into(), 0.9);
        affinities.insert("empathy".into(), 0.8);
        affinities.insert("poetry".into(), 0.7);
        Box::new(ForestSpring::new(
            SpringConfig {
                name: "forest".into(),
                nature: "creativity, narrative, empathy".into(),
                affinities,
            },
            Box::new(MockSource::new(response)),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use water::rain::Volume;
    use water::{Message, Role};

    #[test]
    fn water_types_are_distinct() {
        let vapor = Vapor::default();
        let rain = Rain::new("What is the Tao?", vapor);
        let stream = Stream::new("mountain", "The Tao that can be told...");
        let river = River::from_single(stream.source.clone(), stream.content.clone());
        let ocean = Ocean::new(river.content.clone());

        assert_eq!(rain.raw_input, "What is the Tao?");
        assert_eq!(stream.source, "mountain");
        assert_eq!(river.tributary_count(), 1);
        assert!(ocean.has_substance());
    }

    #[test]
    fn water_cycle_completes() {
        let mut vapor = Vapor::default();
        let rain = Rain::new("Hello", vapor.clone());
        let ocean = Ocean::new("Greetings, traveler.");

        vapor.conversation_history.push(Message {
            role: Role::User,
            content: rain.raw_input.clone(),
        });
        vapor.conversation_history.push(Message {
            role: Role::Assistant,
            content: ocean.content.clone(),
        });

        let rain2 = Rain::new("What did I just say?", vapor);
        assert_eq!(rain2.vapor.conversation_history.len(), 2);
        assert_eq!(rain2.vapor.conversation_history[0].content, "Hello");
    }

    #[test]
    fn volume_determines_depth() {
        let sensor = watershed::VolumeSensor::new();

        let droplet = Rain::new("hi", Vapor::default());
        let storm = Rain::new("word ".repeat(200), Vapor::default());

        assert_eq!(sensor.sense(&droplet), Volume::Droplet);
        assert_eq!(sensor.sense(&storm), Volume::Storm);
    }
}
