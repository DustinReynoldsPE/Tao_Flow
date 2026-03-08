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

/// A point of divergence between streams.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eddy {
    pub topic: String,
    pub positions: Vec<Position>,
    pub nature: EddyNature,
}

impl Eddy {
    pub fn new(topic: impl Into<String>, nature: EddyNature, positions: Vec<Position>) -> Self {
        Self {
            topic: topic.into(),
            positions,
            nature,
        }
    }

    pub fn stream_count(&self) -> usize {
        self.positions.len()
    }

    pub fn is_verifiable(&self) -> bool {
        self.nature == EddyNature::Factual
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
    fn eddy_nature_variants_are_distinct() {
        assert_ne!(EddyNature::Factual, EddyNature::Interpretive);
        assert_ne!(EddyNature::Stylistic, EddyNature::Structural);
    }

    #[test]
    fn eddy_serializes() {
        let eddy = Eddy::new(
            "approach",
            EddyNature::Structural,
            vec![Position {
                source: "mountain".into(),
                view: "top-down".into(),
            }],
        );
        let json = serde_json::to_string(&eddy).unwrap();
        let restored: Eddy = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.topic, "approach");
        assert_eq!(restored.nature, EddyNature::Structural);
    }
}
