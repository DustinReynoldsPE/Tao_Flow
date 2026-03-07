use serde::{Deserialize, Serialize};

/// The nature of an eddy -- how streams disagree.
///
/// Different natures resolve differently, the way different
/// kinds of turbulence settle in different ways.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EddyNature {
    /// One stream is right, one is wrong. Can be verified.
    Factual,
    /// Both streams may be valid. Richness, not conflict.
    Interpretive,
    /// Different tone or voice. Diversity, not disagreement.
    Stylistic,
    /// Different organization of the same truth.
    Structural,
}

/// A position within an eddy -- one stream's view.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    /// Which spring holds this view
    pub source: String,
    /// The view itself
    pub view: String,
}

/// A point of divergence between streams.
///
/// When two rivers meet at an angle, turbulence is natural.
/// The turbulence is not wrong. It is the process by which
/// two waters become one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Eddy {
    /// What the disagreement is about
    pub topic: String,
    /// The different positions held by streams
    pub positions: Vec<Position>,
    /// The nature of the divergence
    pub nature: EddyNature,
}

impl Eddy {
    /// Create a new eddy from two diverging streams.
    pub fn new(topic: impl Into<String>, nature: EddyNature, positions: Vec<Position>) -> Self {
        Self {
            topic: topic.into(),
            positions,
            nature,
        }
    }

    /// How many streams are involved in this eddy?
    pub fn stream_count(&self) -> usize {
        self.positions.len()
    }

    /// Is this a factual disagreement that can be verified?
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
