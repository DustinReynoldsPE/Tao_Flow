use serde::{Deserialize, Serialize};

use crate::confluence::Eddy;

/// Merged output from the confluence -- where streams become one.
///
/// The river carries elements from all tributaries but has its own
/// unified character. It may still contain eddies -- areas of
/// unresolved divergence that are natural features, not errors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct River {
    /// The merged content
    pub content: String,
    /// Which streams contributed
    pub tributaries: Vec<String>,
    /// Remaining areas of divergence
    pub eddies: Vec<Eddy>,
    /// How clear the merged output is (0.0 to 1.0)
    pub clarity: f32,
}

impl River {
    /// Create a river from a single stream -- no merging needed.
    /// Wu wei: do nothing when nothing needs doing.
    pub fn from_single(source: String, content: String) -> Self {
        Self {
            content,
            tributaries: vec![source],
            eddies: Vec::new(),
            clarity: 1.0,
        }
    }

    /// Does this river carry water?
    pub fn has_water(&self) -> bool {
        !self.content.trim().is_empty()
    }

    /// How many tributaries fed this river?
    pub fn tributary_count(&self) -> usize {
        self.tributaries.len()
    }

    /// Does this river still have unresolved eddies?
    pub fn has_eddies(&self) -> bool {
        !self.eddies.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
