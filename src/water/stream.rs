use serde::{Deserialize, Serialize};

/// An LLM's natural response -- a stream from a spring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub source: String,
    pub content: String,
}

impl Stream {
    pub fn new(source: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            source: source.into(),
            content: content.into(),
        }
    }

    pub fn has_water(&self) -> bool {
        !self.content.trim().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_from_spring() {
        let stream = Stream::new(
            "mountain",
            "The Tao that can be told is not the eternal Tao.",
        );
        assert_eq!(stream.source, "mountain");
        assert!(stream.has_water());
    }

    #[test]
    fn dry_spring_has_no_water() {
        let stream = Stream::new("desert", "");
        assert!(!stream.has_water());

        let stream = Stream::new("desert", "   ");
        assert!(!stream.has_water());
    }

    #[test]
    fn stream_serializes() {
        let stream = Stream::new("forest", "Once upon a time...");
        let json = serde_json::to_string(&stream).unwrap();
        let restored: Stream = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.source, "forest");
        assert_eq!(restored.content, "Once upon a time...");
    }
}
