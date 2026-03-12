use crate::confluence::detection::EddyDetector;
use crate::confluence::yielding::YieldingProtocol;
use crate::confluence::Eddy;
use crate::error::FlowError;
use crate::water::{River, Role, Stream};
use crate::watershed::source::{ChatMessage, LlmSource};

/// Where streams merge into a river.
///
/// When one stream flows alone, the pool does nothing — wu wei.
/// When multiple streams arrive, the pool detects eddies,
/// resolves them through yielding, then weaves the river.
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
            _ => self.detect_yield_weave(streams, rain_input).await,
        }
    }

    async fn detect_yield_weave(
        &self,
        streams: Vec<Stream>,
        rain_input: &str,
    ) -> Result<River, FlowError> {
        let detector = EddyDetector::new(self.source.as_ref());
        let mut eddies = detector.detect(&streams, rain_input).await;

        if !eddies.is_empty() {
            let protocol = YieldingProtocol::new(self.source.as_ref());
            protocol.yield_all(&mut eddies).await;
        }

        self.weave(streams, rain_input, eddies).await
    }

    async fn weave(
        &self,
        streams: Vec<Stream>,
        rain_input: &str,
        eddies: Vec<Eddy>,
    ) -> Result<River, FlowError> {
        let tributaries: Vec<String> = streams.iter().map(|s| s.source.clone()).collect();

        let mut prompt = format!(
            "You are the Confluence -- where multiple streams merge into one river.\n\n\
             The user asked: {}\n\n\
             Multiple springs have responded. Your task is to curate, not blend.\n\n\
             FIRST: Read each stream and identify its gems -- the moments where that stream \
             offers something no other stream does. A gem may be a structural insight, a \
             striking metaphor, a unique cross-tradition connection, a table or framework, \
             a concentrated line that does more work than a paragraph, or a specific reference \
             that grounds the response.\n\n\
             THEN: Weave a response that preserves every gem. Build the narrative around them. \
             Where the streams agree, state it once with confidence. Where one stream offers \
             a unique insight, ensure it survives in the final river -- in its original power, \
             not paraphrased into blandness.\n\n\
             Do not mention the springs by name. Do not say \"one perspective says\" or \
             \"another view is.\" The river should read as one voice that carries the depth \
             of all its tributaries.\n\n",
            rain_input
        );

        for stream in &streams {
            prompt.push_str(&format!(
                "--- {} spring ---\n{}\n\n",
                stream.source, stream.content
            ));
        }

        if eddies.iter().any(|e| e.is_resolved()) {
            prompt.push_str("Points of divergence have been resolved through yielding:\n\n");
            for eddy in &eddies {
                if let Some(ref resolution) = eddy.resolution {
                    prompt.push_str(&format!(
                        "- On \"{}\": {}\n",
                        eddy.topic, resolution.synthesis
                    ));
                }
            }
            prompt.push_str("\nHonor these resolutions in the weave.\n\n");
        }

        let messages = vec![ChatMessage {
            role: Role::User,
            content: prompt,
        }];

        let content = self
            .source
            .complete(CONFLUENCE_SYSTEM_PROMPT, &messages)
            .await
            .map_err(|e| FlowError::ConfluenceFailure(e.to_string()))?;

        let clarity = Self::assess_clarity(&eddies);

        Ok(River {
            content,
            tributaries,
            eddies,
            clarity,
        })
    }

    fn assess_clarity(eddies: &[Eddy]) -> f32 {
        if eddies.is_empty() {
            return 0.8;
        }

        let base = 0.8_f32;
        let mut penalty = 0.0_f32;

        for eddy in eddies {
            if eddy.is_resolved() {
                penalty += 0.05;
            } else {
                penalty += 0.1;
            }
        }

        (base - penalty).max(0.3)
    }
}

const CONFLUENCE_SYSTEM_PROMPT: &str = "\
You are a master editor -- not a blender. You curate the best of what multiple \
voices have offered into one coherent response. You do not add your own opinion. \
You do not paraphrase powerful lines into weaker ones. When a stream offers a \
striking metaphor, a unique structural insight, a table, a specific reference, \
or a concentrated line that does more work than a paragraph -- that gem must \
survive intact in your output. The result should read as one voice that carries \
the full mineral content of all its tributaries, not a diluted average.";

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
        // MockSource returns the same response for all calls (detection, yielding, weaving).
        // Detection returns "NONE" format won't match, so no eddies detected — clean merge.
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
        assert!(!river.has_eddies());
        assert_eq!(river.clarity, 0.8);
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

    #[tokio::test]
    async fn clarity_reflects_resolved_eddies() {
        let eddies = vec![{
            let mut e = Eddy::new(
                "topic",
                crate::confluence::EddyNature::Interpretive,
                vec![
                    crate::confluence::Position {
                        source: "mountain".into(),
                        view: "A".into(),
                    },
                    crate::confluence::Position {
                        source: "desert".into(),
                        view: "B".into(),
                    },
                ],
            );
            e.resolve("Both carry truth.");
            e
        }];
        let clarity = ConfluencePool::assess_clarity(&eddies);
        assert!((clarity - 0.75).abs() < 0.01);
    }

    #[tokio::test]
    async fn clarity_reflects_unresolved_eddies() {
        let eddies = vec![Eddy::new(
            "topic",
            crate::confluence::EddyNature::Factual,
            vec![
                crate::confluence::Position {
                    source: "mountain".into(),
                    view: "A".into(),
                },
                crate::confluence::Position {
                    source: "desert".into(),
                    view: "B".into(),
                },
            ],
        )];
        let clarity = ConfluencePool::assess_clarity(&eddies);
        assert!((clarity - 0.7).abs() < 0.01);
    }

    #[tokio::test]
    async fn clarity_has_floor() {
        let eddies: Vec<Eddy> = (0..10)
            .map(|i| {
                Eddy::new(
                    format!("topic {}", i),
                    crate::confluence::EddyNature::Structural,
                    vec![
                        crate::confluence::Position {
                            source: "mountain".into(),
                            view: "A".into(),
                        },
                        crate::confluence::Position {
                            source: "desert".into(),
                            view: "B".into(),
                        },
                    ],
                )
            })
            .collect();
        let clarity = ConfluencePool::assess_clarity(&eddies);
        assert!((clarity - 0.3).abs() < 0.01);
    }
}
