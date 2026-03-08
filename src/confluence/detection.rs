use crate::confluence::{Eddy, EddyNature, Position};
use crate::water::Role;
use crate::water::Stream;
use crate::watershed::source::{ChatMessage, LlmSource};

/// Analyzes streams to find where they diverge.
///
/// When multiple springs respond, they may disagree. The detector
/// finds these eddies — not to judge them, but to name them
/// so yielding can begin.
pub struct EddyDetector<'a> {
    source: &'a dyn LlmSource,
}

impl<'a> EddyDetector<'a> {
    pub fn new(source: &'a dyn LlmSource) -> Self {
        Self { source }
    }

    pub async fn detect(&self, streams: &[Stream], rain_input: &str) -> Vec<Eddy> {
        if streams.len() < 2 {
            return Vec::new();
        }

        let prompt = self.build_detection_prompt(streams, rain_input);
        let messages = vec![ChatMessage {
            role: Role::User,
            content: prompt,
        }];

        let response = match self
            .source
            .complete(DETECTION_SYSTEM_PROMPT, &messages)
            .await
        {
            Ok(r) => r,
            Err(_) => return Vec::new(),
        };

        self.parse_eddies(&response, streams)
    }

    fn build_detection_prompt(&self, streams: &[Stream], rain_input: &str) -> String {
        let mut prompt = format!(
            "The user asked: {}\n\n\
             Multiple springs responded. Analyze their responses for points of divergence.\n\n",
            rain_input
        );

        for stream in streams {
            prompt.push_str(&format!(
                "--- {} spring ---\n{}\n\n",
                stream.source, stream.content
            ));
        }

        prompt.push_str(
            "Identify points where these responses diverge. For each divergence, output \
             exactly one line in this format:\n\
             EDDY|<nature>|<topic>|<source1>:<view1>|<source2>:<view2>\n\n\
             Where <nature> is one of: Factual, Interpretive, Stylistic, Structural\n\n\
             If there are no meaningful divergences, output: NONE\n\
             Output only EDDY lines or NONE. No other text.",
        );

        prompt
    }

    fn parse_eddies(&self, response: &str, streams: &[Stream]) -> Vec<Eddy> {
        let valid_sources: Vec<&str> = streams.iter().map(|s| s.source.as_str()).collect();

        response
            .lines()
            .filter(|line| line.starts_with("EDDY|"))
            .filter_map(|line| self.parse_eddy_line(line, &valid_sources))
            .collect()
    }

    fn parse_eddy_line(&self, line: &str, valid_sources: &[&str]) -> Option<Eddy> {
        let parts: Vec<&str> = line.splitn(4, '|').collect();
        if parts.len() < 4 {
            return None;
        }

        let nature = match parts[1].trim() {
            "Factual" => EddyNature::Factual,
            "Interpretive" => EddyNature::Interpretive,
            "Stylistic" => EddyNature::Stylistic,
            "Structural" => EddyNature::Structural,
            _ => return None,
        };

        let topic = parts[2].trim().to_string();
        if topic.is_empty() {
            return None;
        }

        let positions: Vec<Position> = parts[3]
            .split('|')
            .filter_map(|p| {
                let (source, view) = p.split_once(':')?;
                let source = source.trim();
                if !valid_sources.contains(&source) {
                    return None;
                }
                Some(Position {
                    source: source.to_string(),
                    view: view.trim().to_string(),
                })
            })
            .collect();

        if positions.len() < 2 {
            return None;
        }

        Some(Eddy::new(topic, nature, positions))
    }
}

const DETECTION_SYSTEM_PROMPT: &str = "\
You analyze multiple responses to find where they diverge. \
You do not judge which is right. You name the divergence and classify it. \
Factual: disagreement on verifiable facts. \
Interpretive: different valid readings of the same thing. \
Stylistic: different tones, approaches, or framings. \
Structural: different ways to organize or prioritize.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::watershed::source::mock::MockSource;

    #[tokio::test]
    async fn detects_eddies_from_divergent_streams() {
        let response =
            "EDDY|Interpretive|best approach|mountain:Start with theory|forest:Start with stories";
        let source = MockSource::new(response);
        let detector = EddyDetector::new(&source);

        let streams = vec![
            Stream::new("mountain", "Start with theory and build up."),
            Stream::new("forest", "Start with stories and discover."),
        ];

        let eddies = detector.detect(&streams, "How should I learn?").await;
        assert_eq!(eddies.len(), 1);
        assert_eq!(eddies[0].nature, EddyNature::Interpretive);
        assert_eq!(eddies[0].topic, "best approach");
        assert_eq!(eddies[0].stream_count(), 2);
    }

    #[tokio::test]
    async fn detects_multiple_eddies() {
        let response = "\
            EDDY|Factual|year invented|mountain:1971|desert:1972\n\
            EDDY|Stylistic|tone|mountain:formal analysis|forest:narrative warmth";
        let source = MockSource::new(response);
        let detector = EddyDetector::new(&source);

        let streams = vec![
            Stream::new("mountain", "Invented in 1971, formally..."),
            Stream::new("desert", "Created in 1972, quickly..."),
            Stream::new("forest", "The story begins in the early 70s..."),
        ];

        let eddies = detector.detect(&streams, "Tell me about Rust").await;
        assert_eq!(eddies.len(), 2);
        assert_eq!(eddies[0].nature, EddyNature::Factual);
        assert_eq!(eddies[1].nature, EddyNature::Stylistic);
    }

    #[tokio::test]
    async fn no_eddies_when_streams_agree() {
        let source = MockSource::new("NONE");
        let detector = EddyDetector::new(&source);

        let streams = vec![
            Stream::new("mountain", "Water flows downhill."),
            Stream::new("desert", "Water flows downhill."),
        ];

        let eddies = detector
            .detect(&streams, "Which way does water flow?")
            .await;
        assert!(eddies.is_empty());
    }

    #[tokio::test]
    async fn no_eddies_from_single_stream() {
        let source = MockSource::new("should not be called");
        let detector = EddyDetector::new(&source);

        let streams = vec![Stream::new("desert", "Just me.")];
        let eddies = detector.detect(&streams, "hello").await;
        assert!(eddies.is_empty());
    }

    #[tokio::test]
    async fn graceful_on_unparseable_response() {
        let source = MockSource::new("I couldn't find any divergences in these responses.");
        let detector = EddyDetector::new(&source);

        let streams = vec![Stream::new("mountain", "A"), Stream::new("desert", "B")];

        let eddies = detector.detect(&streams, "test").await;
        assert!(eddies.is_empty());
    }

    #[tokio::test]
    async fn graceful_on_source_failure() {
        let source = crate::watershed::source::mock::DrySource;
        let detector = EddyDetector::new(&source);

        let streams = vec![Stream::new("mountain", "A"), Stream::new("desert", "B")];

        let eddies = detector.detect(&streams, "test").await;
        assert!(eddies.is_empty());
    }

    #[tokio::test]
    async fn rejects_invalid_source_names() {
        let response = "EDDY|Factual|topic|unknown:view1|mountain:view2";
        let source = MockSource::new(response);
        let detector = EddyDetector::new(&source);

        let streams = vec![Stream::new("mountain", "A"), Stream::new("desert", "B")];

        // "unknown" is not a valid source, so only 1 valid position → eddy rejected
        let eddies = detector.detect(&streams, "test").await;
        assert!(eddies.is_empty());
    }
}
