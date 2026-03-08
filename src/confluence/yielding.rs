use crate::confluence::Eddy;
use crate::watershed::source::{ChatMessage, ChatRole, LlmSource};

/// Resolves eddies through yielding — each position finds truth in the other.
///
/// This is not voting. No position is discarded. The soft overcomes the hard;
/// the gentle overcomes the rigid. Each side yields, and from the yielding
/// a synthesis emerges that carries both truths.
pub struct YieldingProtocol<'a> {
    source: &'a dyn LlmSource,
}

impl<'a> YieldingProtocol<'a> {
    pub fn new(source: &'a dyn LlmSource) -> Self {
        Self { source }
    }

    pub async fn yield_eddy(&self, eddy: &mut Eddy) {
        let prompt = self.build_yielding_prompt(eddy);
        let messages = vec![ChatMessage {
            role: ChatRole::User,
            content: prompt,
        }];

        if let Ok(synthesis) = self
            .source
            .complete(YIELDING_SYSTEM_PROMPT, &messages)
            .await
        {
            if !synthesis.trim().is_empty() {
                eddy.resolve(synthesis);
            }
        }
    }

    pub async fn yield_all(&self, eddies: &mut [Eddy]) {
        for eddy in eddies.iter_mut() {
            self.yield_eddy(eddy).await;
        }
    }

    fn build_yielding_prompt(&self, eddy: &Eddy) -> String {
        let mut prompt = format!(
            "An eddy has formed around: {}\n\
             Nature: {:?}\n\n\
             The positions:\n",
            eddy.topic, eddy.nature
        );

        for pos in &eddy.positions {
            prompt.push_str(&format!("- {} says: {}\n", pos.source, pos.view));
        }

        prompt.push_str(
            "\nDo not choose a winner. Instead, let each position yield to the other — \
             find what is true in each view. What does each position see that the other missed? \
             Produce a synthesis that carries both truths forward. \
             Respond with only the synthesis, nothing else.",
        );

        prompt
    }
}

const YIELDING_SYSTEM_PROMPT: &str = "\
You resolve disagreements through yielding, not voting. \
Nothing in the world is as soft and yielding as water, \
yet for dissolving the hard and inflexible, nothing can surpass it. \
Find truth in every position. Let each side yield to the other. \
The synthesis should carry what is true from all sides.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::confluence::{EddyNature, Position};
    use crate::watershed::source::mock::MockSource;

    #[tokio::test]
    async fn yielding_resolves_eddy() {
        let synthesis = "Theory provides the map; stories make the territory real. \
                         Both are needed — begin where the student's curiosity lives.";
        let source = MockSource::new(synthesis);
        let protocol = YieldingProtocol::new(&source);

        let mut eddy = Eddy::new(
            "teaching approach",
            EddyNature::Interpretive,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "Start with theory.".into(),
                },
                Position {
                    source: "forest".into(),
                    view: "Start with stories.".into(),
                },
            ],
        );

        protocol.yield_eddy(&mut eddy).await;

        assert!(eddy.is_resolved());
        let resolution = eddy.resolution.unwrap();
        assert!(resolution.synthesis.contains("map"));
    }

    #[tokio::test]
    async fn yield_all_resolves_multiple_eddies() {
        let source = MockSource::new("Both perspectives carry truth.");
        let protocol = YieldingProtocol::new(&source);

        let mut eddies = vec![
            Eddy::new(
                "style",
                EddyNature::Stylistic,
                vec![
                    Position {
                        source: "mountain".into(),
                        view: "Formal.".into(),
                    },
                    Position {
                        source: "forest".into(),
                        view: "Narrative.".into(),
                    },
                ],
            ),
            Eddy::new(
                "structure",
                EddyNature::Structural,
                vec![
                    Position {
                        source: "mountain".into(),
                        view: "Top-down.".into(),
                    },
                    Position {
                        source: "desert".into(),
                        view: "Bottom-up.".into(),
                    },
                ],
            ),
        ];

        protocol.yield_all(&mut eddies).await;

        assert!(eddies.iter().all(|e| e.is_resolved()));
    }

    #[tokio::test]
    async fn yielding_graceful_on_failure() {
        let source = crate::watershed::source::mock::DrySource;
        let protocol = YieldingProtocol::new(&source);

        let mut eddy = Eddy::new(
            "topic",
            EddyNature::Factual,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "A".into(),
                },
                Position {
                    source: "desert".into(),
                    view: "B".into(),
                },
            ],
        );

        protocol.yield_eddy(&mut eddy).await;
        assert!(!eddy.is_resolved());
    }

    #[tokio::test]
    async fn yielding_rejects_empty_synthesis() {
        let source = MockSource::new("   ");
        let protocol = YieldingProtocol::new(&source);

        let mut eddy = Eddy::new(
            "topic",
            EddyNature::Interpretive,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "A".into(),
                },
                Position {
                    source: "forest".into(),
                    view: "B".into(),
                },
            ],
        );

        protocol.yield_eddy(&mut eddy).await;
        assert!(!eddy.is_resolved());
    }
}
