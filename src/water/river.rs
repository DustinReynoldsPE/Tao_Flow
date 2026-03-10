use serde::{Deserialize, Serialize};

use crate::confluence::Eddy;

/// Merged output from the confluence.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct River {
    pub content: String,
    pub tributaries: Vec<String>,
    pub eddies: Vec<Eddy>,
    pub clarity: f32,
}

impl River {
    pub fn from_single(source: String, content: String) -> Self {
        Self {
            content,
            tributaries: vec![source],
            eddies: Vec::new(),
            clarity: 1.0,
        }
    }

    pub fn has_water(&self) -> bool {
        !self.content.trim().is_empty()
    }

    pub fn tributary_count(&self) -> usize {
        self.tributaries.len()
    }

    pub fn has_eddies(&self) -> bool {
        !self.eddies.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::confluence::{EddyNature, Position};

    #[test]
    fn river_from_single_stream() {
        let river = River::from_single("mountain".into(), "Deep truth.".into());
        assert_eq!(river.tributary_count(), 1);
        assert!(!river.has_eddies());
        assert!(river.has_water());
        assert_eq!(river.clarity, 1.0);
    }

    #[test]
    fn empty_river_has_no_water() {
        let river = River::from_single("desert".into(), String::new());
        assert!(!river.has_water());
    }

    #[test]
    fn river_tracks_eddy_resolution() {
        let mut resolved = Eddy::new(
            "approach",
            EddyNature::Interpretive,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "theory".into(),
                },
                Position {
                    source: "forest".into(),
                    view: "stories".into(),
                },
            ],
        );
        resolved.resolve("Both are needed.");

        let unresolved = Eddy::new(
            "timeline",
            EddyNature::Factual,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "1971".into(),
                },
                Position {
                    source: "desert".into(),
                    view: "1972".into(),
                },
            ],
        );

        let river = River {
            content: "Merged.".into(),
            tributaries: vec!["mountain".into(), "desert".into(), "forest".into()],
            eddies: vec![resolved, unresolved],
            clarity: 0.65,
        };

        assert!(river.has_eddies());
    }
}
