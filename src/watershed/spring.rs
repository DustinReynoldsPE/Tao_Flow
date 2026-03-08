use std::collections::HashMap;

use async_trait::async_trait;

use crate::error::FlowError;
use crate::water::{Rain, Stream};

#[async_trait]
pub trait Spring: Send + Sync {
    fn name(&self) -> &str;
    fn nature(&self) -> &str;
    fn sense_relevance(&self, rain: &Rain) -> f32;
    async fn respond(&self, rain: &Rain) -> Result<Option<Stream>, FlowError>;
}

#[derive(Debug, Clone)]
pub struct SpringConfig {
    pub name: String,
    pub nature: String,
    pub affinities: HashMap<String, f32>,
}

impl SpringConfig {
    pub fn sense_relevance(&self, rain: &Rain) -> f32 {
        let mut score: f32 = 0.3; // Base -- every spring has something to offer
        for mineral in &rain.minerals {
            if let Some(&affinity) = self.affinities.get(mineral) {
                score += affinity;
            }
        }
        score.min(1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::water::Vapor;

    fn test_config() -> SpringConfig {
        let mut affinities = HashMap::new();
        affinities.insert("philosophy".into(), 0.9);
        affinities.insert("architecture".into(), 0.8);
        affinities.insert("code".into(), 0.2);

        SpringConfig {
            name: "mountain".into(),
            nature: "deep reasoning".into(),
            affinities,
        }
    }

    #[test]
    fn base_relevance_without_minerals() {
        let config = test_config();
        let rain = Rain::new("hello", Vapor::default());
        // No minerals in rain, so only base score
        assert_eq!(config.sense_relevance(&rain), 0.3);
    }

    #[test]
    fn relevance_increases_with_matching_minerals() {
        let config = test_config();
        let mut rain = Rain::new("what is the nature of being?", Vapor::default());
        rain.minerals.push("philosophy".into());
        let relevance = config.sense_relevance(&rain);
        assert!(relevance > 0.3);
        assert!((relevance - 1.0).abs() < f32::EPSILON); // 0.3 + 0.9 = 1.2, capped at 1.0
    }

    #[test]
    fn relevance_lower_for_weak_affinity() {
        let config = test_config();
        let mut rain = Rain::new("fix this bug", Vapor::default());
        rain.minerals.push("code".into());
        let relevance = config.sense_relevance(&rain);
        assert_eq!(relevance, 0.5); // 0.3 + 0.2
    }

    #[test]
    fn relevance_caps_at_one() {
        let config = test_config();
        let mut rain = Rain::new("deep thoughts on system design", Vapor::default());
        rain.minerals.push("philosophy".into());
        rain.minerals.push("architecture".into());
        // 0.3 + 0.9 + 0.8 = 2.0, should cap at 1.0
        assert_eq!(config.sense_relevance(&rain), 1.0);
    }

    #[test]
    fn unknown_minerals_do_not_affect_relevance() {
        let config = test_config();
        let mut rain = Rain::new("cook me dinner", Vapor::default());
        rain.minerals.push("cooking".into());
        assert_eq!(config.sense_relevance(&rain), 0.3);
    }
}
