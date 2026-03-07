use serde::{Deserialize, Serialize};

/// What the user receives -- the final output.
///
/// The ocean is vast, deep, and unified. The user does not see
/// the rain, the streams, or the river. They see only the ocean.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ocean {
    /// What the user receives
    pub content: String,
    /// How profound the response is (0.0 to 1.0)
    pub depth: f32,
    /// Emotional quality (-1.0 to 1.0)
    pub warmth: f32,
}

impl Ocean {
    /// Create an ocean from refined content.
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            depth: 0.5,
            warmth: 0.0,
        }
    }

    /// Does this ocean carry substance?
    pub fn has_substance(&self) -> bool {
        !self.content.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ocean_carries_content() {
        let ocean = Ocean::new("The Tao flows.");
        assert!(ocean.has_substance());
        assert_eq!(ocean.content, "The Tao flows.");
    }

    #[test]
    fn empty_ocean_lacks_substance() {
        let ocean = Ocean::new("");
        assert!(!ocean.has_substance());
    }
}
