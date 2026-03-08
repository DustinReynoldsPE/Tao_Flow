use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EddyNature {
    Factual,
    Interpretive,
    Stylistic,
    Structural,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub source: String,
    pub view: String,
}

/// What emerges when positions yield to each other.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resolution {
    pub synthesis: String,
}

impl Resolution {
    pub fn new(synthesis: impl Into<String>) -> Self {
        Self {
            synthesis: synthesis.into(),
        }
    }
}

/// A point of divergence between streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eddy {
    pub topic: String,
    pub positions: Vec<Position>,
    pub nature: EddyNature,
    pub resolution: Option<Resolution>,
}

impl Eddy {
    pub fn new(topic: impl Into<String>, nature: EddyNature, positions: Vec<Position>) -> Self {
        Self {
            topic: topic.into(),
            positions,
            nature,
            resolution: None,
        }
    }

    pub fn stream_count(&self) -> usize {
        self.positions.len()
    }

    pub fn is_verifiable(&self) -> bool {
        self.nature == EddyNature::Factual
    }

    pub fn is_resolved(&self) -> bool {
        self.resolution.is_some()
    }

    pub fn resolve(&mut self, synthesis: impl Into<String>) {
        self.resolution = Some(Resolution::new(synthesis));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eddy_from_two_streams() {
        let eddy = Eddy::new(
            "best language for beginners",
            EddyNature::Interpretive,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "Python".into(),
                },
                Position {
                    source: "forest".into(),
                    view: "JavaScript".into(),
                },
            ],
        );
        assert_eq!(eddy.stream_count(), 2);
        assert!(!eddy.is_verifiable());
        assert!(!eddy.is_resolved());
    }

    #[test]
    fn factual_eddy_is_verifiable() {
        let eddy = Eddy::new(
            "capital of France",
            EddyNature::Factual,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "Paris".into(),
                },
                Position {
                    source: "desert".into(),
                    view: "Lyon".into(),
                },
            ],
        );
        assert!(eddy.is_verifiable());
    }

    #[test]
    fn eddy_resolves_through_yielding() {
        let mut eddy = Eddy::new(
            "best approach",
            EddyNature::Interpretive,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "Start with theory.".into(),
                },
                Position {
                    source: "forest".into(),
                    view: "Start with stories.".into(),
                },
            ],
        );
        assert!(!eddy.is_resolved());

        eddy.resolve("Theory gives the map; stories give the territory. Begin with a story that illuminates the theory.");
        assert!(eddy.is_resolved());
        assert!(eddy.resolution.unwrap().synthesis.contains("story"));
    }

    #[test]
    fn eddy_nature_variants_are_distinct() {
        assert_ne!(EddyNature::Factual, EddyNature::Interpretive);
        assert_ne!(EddyNature::Stylistic, EddyNature::Structural);
    }

    #[test]
    fn eddy_serializes_with_resolution() {
        let mut eddy = Eddy::new(
            "approach",
            EddyNature::Structural,
            vec![Position {
                source: "mountain".into(),
                view: "top-down".into(),
            }],
        );
        eddy.resolve("Both directions serve the whole.");

        let json = serde_json::to_string(&eddy).unwrap();
        let restored: Eddy = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.topic, "approach");
        assert_eq!(restored.nature, EddyNature::Structural);
        assert!(restored.is_resolved());
    }
}
