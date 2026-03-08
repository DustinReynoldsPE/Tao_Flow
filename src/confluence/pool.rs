use crate::error::FlowError;
use crate::water::{River, Stream};
use crate::watershed::source::{ChatMessage, ChatRole, LlmSource};

/// Where streams merge into a river.
///
/// When one stream flows alone, the pool does nothing — wu wei.
/// When multiple streams arrive, the pool weaves them together.
pub struct ConfluencePool {
    source: Box<dyn LlmSource>,
}

impl ConfluencePool {
    pub fn new(source: Box<dyn LlmSource>) -> Self {
        Self { source }
    }

    pub async fn merge(&self, streams: Vec<Stream>, rain_input: &str) -> Result<River, FlowError> {
        match streams.len() {
            0 => Ok(River {
                content: String::new(),
                tributaries: vec![],
                eddies: Vec::new(),
                clarity: 0.0,
            }),
            1 => {
                let stream = streams.into_iter().next().unwrap();
                Ok(River::from_single(stream.source, stream.content))
            }
            _ => self.weave(streams, rain_input).await,
        }
    }

    async fn weave(&self, streams: Vec<Stream>, rain_input: &str) -> Result<River, FlowError> {
        let tributaries: Vec<String> = streams.iter().map(|s| s.source.clone()).collect();

        let mut prompt = format!(
            "You are the Confluence -- where multiple streams merge into one river.\n\n\
             The user asked: {}\n\n\
             Multiple springs have responded. Weave their perspectives into a single, \
             coherent response. Preserve what is unique from each voice. Where they agree, \
             state it once with confidence. Where they differ, include both perspectives \
             naturally.\n\n\
             Do not mention the springs by name. Do not say \"one perspective says\" or \
             \"another view is.\" Simply weave the river.\n\n",
            rain_input
        );

        for stream in &streams {
            prompt.push_str(&format!(
                "--- {} spring ---\n{}\n\n",
                stream.source, stream.content
            ));
        }

        let messages = vec![ChatMessage {
            role: ChatRole::User,
            content: prompt,
        }];

        let content = self
            .source
            .complete(CONFLUENCE_SYSTEM_PROMPT, &messages)
            .await
            .map_err(|e| FlowError::ConfluenceFailure(e.to_string()))?;

        Ok(River {
            content,
            tributaries,
            eddies: Vec::new(),
            clarity: 0.8,
        })
    }
}

const CONFLUENCE_SYSTEM_PROMPT: &str = "\
You merge multiple perspectives into one clear, coherent response. \
You do not add your own opinion. You weave what the streams have given you. \
Where they agree, state it once. Where they offer different angles, \
include both naturally. The result should read as one voice, not a committee.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::watershed::source::mock::MockSource;

    #[tokio::test]
    async fn single_stream_passes_through() {
        let source = MockSource::new("should not be called");
        let pool = ConfluencePool::new(Box::new(source));

        let streams = vec![Stream::new("mountain", "Deep truth about the Tao.")];
        let river = pool.merge(streams, "What is the Tao?").await.unwrap();

        assert_eq!(river.tributary_count(), 1);
        assert_eq!(river.content, "Deep truth about the Tao.");
        assert!(!river.has_eddies());
        assert_eq!(river.clarity, 1.0);
    }

    #[tokio::test]
    async fn empty_streams_produce_empty_river() {
        let source = MockSource::new("should not be called");
        let pool = ConfluencePool::new(Box::new(source));

        let river = pool.merge(vec![], "hello").await.unwrap();
        assert_eq!(river.tributary_count(), 0);
        assert!(!river.has_water());
    }

    #[tokio::test]
    async fn multiple_streams_are_woven() {
        let woven = "The Tao is both deep and immediate.";
        let source = MockSource::new(woven);
        let pool = ConfluencePool::new(Box::new(source));

        let streams = vec![
            Stream::new("mountain", "The Tao is the source of all depth."),
            Stream::new("desert", "The Tao is simple and direct."),
            Stream::new("forest", "The Tao is alive in every story."),
        ];

        let river = pool.merge(streams, "What is the Tao?").await.unwrap();

        assert_eq!(river.tributary_count(), 3);
        assert_eq!(river.content, woven);
        assert!(river.has_water());
    }

    #[tokio::test]
    async fn two_streams_merge() {
        let woven = "A balanced perspective.";
        let source = MockSource::new(woven);
        let pool = ConfluencePool::new(Box::new(source));

        let streams = vec![
            Stream::new("mountain", "Deep analysis here."),
            Stream::new("desert", "Quick take here."),
        ];

        let river = pool.merge(streams, "Explain Rust").await.unwrap();

        assert_eq!(river.tributary_count(), 2);
        assert_eq!(river.content, woven);
    }

    #[tokio::test]
    async fn tributaries_track_sources() {
        let source = MockSource::new("merged");
        let pool = ConfluencePool::new(Box::new(source));

        let streams = vec![
            Stream::new("mountain", "A"),
            Stream::new("forest", "B"),
            Stream::new("desert", "C"),
        ];

        let river = pool.merge(streams, "test").await.unwrap();

        assert!(river.tributaries.contains(&"mountain".to_string()));
        assert!(river.tributaries.contains(&"forest".to_string()));
        assert!(river.tributaries.contains(&"desert".to_string()));
    }
}
