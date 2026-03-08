use serde::{Deserialize, Serialize};

use super::Vapor;

/// The volume of rain determines how many springs respond.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Volume {
    /// Simple, single-spring sufficient
    Droplet,
    /// Moderate, 2-3 springs
    Shower,
    /// Complex, all springs
    Downpour,
    /// Transformative, multiple passes
    Storm,
}

/// User input -- undifferentiated, natural.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rain {
    pub raw_input: String,
    pub vapor: Vapor,
    pub volume: Volume,
    pub minerals: Vec<String>,
}

impl Rain {
    pub fn new(input: impl Into<String>, vapor: Vapor) -> Self {
        Self {
            raw_input: input.into(),
            vapor,
            volume: Volume::Shower,
            minerals: Vec::new(),
        }
    }

    pub fn weight(&self) -> usize {
        self.raw_input.split_whitespace().count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rain_from_simple_input() {
        let rain = Rain::new("hello", Vapor::default());
        assert_eq!(rain.raw_input, "hello");
        assert_eq!(rain.volume, Volume::Shower);
        assert_eq!(rain.weight(), 1);
    }

    #[test]
    fn rain_weight_reflects_complexity() {
        let light = Rain::new("hi", Vapor::default());
        let heavy = Rain::new(
            "Design a complete distributed system for real-time \
             collaborative document editing with conflict resolution",
            Vapor::default(),
        );
        assert!(heavy.weight() > light.weight());
    }

    #[test]
    fn volume_enum_is_ordered_conceptually() {
        // Droplet < Shower < Downpour < Storm in intensity
        // We verify they are distinct variants
        assert_ne!(Volume::Droplet, Volume::Shower);
        assert_ne!(Volume::Shower, Volume::Downpour);
        assert_ne!(Volume::Downpour, Volume::Storm);
    }

    #[test]
    fn rain_carries_minerals() {
        let mut rain = Rain::new("write a poem about rust", Vapor::default());
        rain.minerals.push("poetry".into());
        rain.minerals.push("code".into());
        assert_eq!(rain.minerals.len(), 2);
    }
}
