use serde::{Deserialize, Serialize};

/// An LLM's natural response -- a stream from a spring.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stream {
    pub source: String,
    pub content: String,
    pub clarity: f32,
    pub depth: f32,
}

impl Stream {
    pub fn new(source: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        let depth = Self::assess_depth(&content);
        Self {
            source: source.into(),
            content,
            clarity: 0.8,
            depth,
        }
    }

    pub fn has_water(&self) -> bool {
        !self.content.trim().is_empty()
    }

    fn assess_depth(content: &str) -> f32 {
        match content.split_whitespace().count() {
            0..=49 => 0.2,
            50..=199 => 0.5,
            _ => 0.8,
        }
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
    fn depth_reflects_engagement() {
        let shallow = Stream::new("desert", "Yes.");
        let deep = Stream::new(
            "mountain",
            "word ".repeat(200), // 200 words
        );
        assert!(deep.depth > shallow.depth);
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
