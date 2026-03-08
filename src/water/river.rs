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
