pub mod source;
pub mod spring;
pub mod springs;
pub mod volume_sensor;

pub use source::{ChatMessage, ChatRole, LlmSource};
pub use spring::Spring;
pub use springs::{DesertSpring, MountainSpring};
pub use volume_sensor::VolumeSensor;

use crate::water::rain::Volume;
use crate::water::{Rain, Stream};

/// The watershed does not decide where rain goes.
/// It simply has a shape, and water follows that shape.
///
/// "Man follows the earth. Earth follows the universe.
///  The universe follows the Tao. The Tao follows only itself."
///  -- Tao Te Ching, Chapter 25
pub struct Watershed {
    springs: Vec<Box<dyn Spring>>,
    volume_sensor: VolumeSensor,
}

impl Watershed {
    pub fn new(springs: Vec<Box<dyn Spring>>) -> Self {
        Self {
            springs,
            volume_sensor: VolumeSensor::new(),
        }
    }

    /// All springs receive rain. Each responds according to its nature.
    pub async fn receive_rain(&self, rain: &mut Rain) -> Vec<Stream> {
        // Sense the volume
        rain.volume = self.volume_sensor.sense(rain);

        // Select springs based on volume (wu wei -- minimal intervention)
        let active_springs = self.activate_springs(rain.volume);

        // All active springs flow simultaneously.
        // Rain is shared -- all springs read the same rain.
        let handles: Vec<_> = active_springs
            .iter()
            .map(|spring| spring.respond(rain))
            .collect();

        let results = futures::future::join_all(handles).await;

        // Gather the streams, filtering dry springs
        results
            .into_iter()
            .filter_map(|r| r.ok().flatten())
            .collect()
    }

    fn activate_springs(&self, volume: Volume) -> Vec<&dyn Spring> {
        match volume {
            Volume::Droplet => {
                // Only desert springs -- light rain, quick response
                let desert: Vec<_> = self
                    .springs
                    .iter()
                    .filter(|s| s.name() == "desert")
                    .map(|s| s.as_ref())
                    .collect();
                if desert.is_empty() {
                    // If no desert spring, use the first available
                    self.springs.iter().take(1).map(|s| s.as_ref()).collect()
                } else {
                    desert
                }
            }
            Volume::Shower => {
                // The two most relevant springs
                self.springs.iter().take(2).map(|s| s.as_ref()).collect()
            }
            Volume::Downpour | Volume::Storm => {
                // All springs flow
                self.springs.iter().map(|s| s.as_ref()).collect()
            }
        }
    }

    /// How many springs are in this watershed?
    pub fn spring_count(&self) -> usize {
        self.springs.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::Vapor;
    use crate::watershed::source::mock::MockSource;
    use crate::watershed::spring::SpringConfig;
    use std::collections::HashMap;

    fn test_mountain(response: &str) -> Box<dyn Spring> {
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

    fn test_desert(response: &str) -> Box<dyn Spring> {
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
    async fn watershed_with_two_springs() {
        let watershed = Watershed::new(vec![
            test_mountain("Deep answer."),
            test_desert("Quick answer."),
        ]);
        assert_eq!(watershed.spring_count(), 2);
    }

    #[tokio::test]
    async fn droplet_activates_only_desert() {
        let watershed = Watershed::new(vec![test_mountain("Deep."), test_desert("Quick.")]);
        let mut rain = Rain::new("hi", Vapor::default());
        let streams = watershed.receive_rain(&mut rain).await;

        // Droplet volume should activate only desert
        assert_eq!(rain.volume, Volume::Droplet);
        assert_eq!(streams.len(), 1);
        assert_eq!(streams[0].source, "desert");
    }

    #[tokio::test]
    async fn shower_activates_two_springs() {
        let watershed = Watershed::new(vec![
            test_mountain("Deep analysis of the topic."),
            test_desert("Quick answer."),
        ]);
        let mut rain = Rain::new(
            "Can you explain how async programming works in Rust?",
            Vapor::default(),
        );
        let streams = watershed.receive_rain(&mut rain).await;

        assert_eq!(rain.volume, Volume::Shower);
        assert_eq!(streams.len(), 2);
    }

    #[tokio::test]
    async fn dry_springs_are_filtered() {
        let watershed = Watershed::new(vec![
            test_mountain(""), // dry
            test_desert("Quick answer."),
        ]);
        let mut rain = Rain::new("Can you help me with something?", Vapor::default());
        let streams = watershed.receive_rain(&mut rain).await;

        // Mountain is dry, only desert flows
        assert!(streams.iter().all(|s| s.source == "desert"));
    }
}
