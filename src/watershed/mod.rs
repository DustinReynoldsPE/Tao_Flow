pub mod mineral_classifier;
pub mod source;
pub mod spring;
pub mod springs;
pub mod volume_sensor;

pub use mineral_classifier::MineralClassifier;
pub use source::tmux_pane::TmuxPaneSource;
pub use source::{ChatMessage, LlmSource};
pub use spring::Spring;
pub use springs::{DesertSpring, ForestSpring, MountainSpring};
pub use volume_sensor::VolumeSensor;

use crate::water::rain::Volume;
use crate::water::{Rain, Stream};

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

    pub async fn receive_rain(&self, rain: &mut Rain) -> Vec<Stream> {
        rain.volume = self.volume_sensor.sense(rain);

        if rain.minerals.is_empty() {
            rain.minerals = MineralClassifier::classify(&rain.raw_input);
        }

        let active_springs = self.activate_springs(rain);

        let handles: Vec<_> = active_springs
            .iter()
            .map(|spring| spring.respond(rain))
            .collect();

        let results = futures::future::join_all(handles).await;
        results
            .into_iter()
            .filter_map(|r| r.ok().flatten())
            .collect()
    }

    fn activate_springs(&self, rain: &Rain) -> Vec<&dyn Spring> {
        match rain.volume {
            Volume::Droplet => {
                let desert: Vec<_> = self
                    .springs
                    .iter()
                    .filter(|s| s.name() == "desert")
                    .map(|s| s.as_ref())
                    .collect();
                if desert.is_empty() {
                    self.springs.iter().take(1).map(|s| s.as_ref()).collect()
                } else {
                    desert
                }
            }
            Volume::Shower => {
                // Select the 2 most relevant springs.
                // Stable tiebreak by insertion order preserves existing behavior
                // when all springs have equal relevance (no mineral matches).
                let mut scored: Vec<_> = self
                    .springs
                    .iter()
                    .enumerate()
                    .map(|(i, s)| (s.sense_relevance(rain), i, s.as_ref()))
                    .collect();
                scored.sort_by(|a, b| {
                    b.0.partial_cmp(&a.0)
                        .unwrap_or(std::cmp::Ordering::Equal)
                        .then(a.1.cmp(&b.1))
                });
                scored.into_iter().take(2).map(|(_, _, s)| s).collect()
            }
            Volume::Downpour | Volume::Storm => self.springs.iter().map(|s| s.as_ref()).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_springs;
    use crate::water::Vapor;

    #[tokio::test]
    async fn droplet_activates_only_desert() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Deep."),
            test_springs::desert("Quick."),
        ]);
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
            test_springs::mountain("Deep analysis of the topic."),
            test_springs::desert("Quick answer."),
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
            test_springs::mountain(""), // dry
            test_springs::desert("Quick answer."),
        ]);
        let mut rain = Rain::new("Can you help me with something?", Vapor::default());
        let streams = watershed.receive_rain(&mut rain).await;

        // Mountain is dry, only desert flows
        assert!(streams.iter().all(|s| s.source == "desert"));
    }
}
