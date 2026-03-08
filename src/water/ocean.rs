use serde::{Deserialize, Serialize};

/// What the user receives.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ocean {
    pub content: String,
}

impl Ocean {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
        }
    }

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
