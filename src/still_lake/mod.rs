use crate::error::FlowError;
use crate::water::{Ocean, River, Role};
use crate::watershed::source::{ChatMessage, LlmSource};

/// How deeply the lake engages, calibrated by clarity.
enum SettlingDepth {
    /// Clarity >= 0.75: light polish, the water is nearly clear.
    Gentle,
    /// Clarity >= 0.5: moderate settling, some turbulence remains.
    Moderate,
    /// Clarity < 0.5: deep settling, significant muddiness.
    Deep,
}

/// The final refinement before the ocean.
///
/// The lake receives the river — its content, clarity, and eddies —
/// and asks the five questions: clarity, wholeness, kindness, truth, simplicity.
/// When the water is already clear, the lake does nothing.
pub struct StillLake {
    source: Box<dyn LlmSource>,
}

impl StillLake {
    pub fn new(source: Box<dyn LlmSource>) -> Self {
        Self { source }
    }

    pub async fn settle(&self, river: River, rain_input: &str) -> Result<Ocean, FlowError> {
        if !river.has_water() {
            return Ok(Ocean::new(river.content));
        }

        // Wu wei: clear water needs no settling.
        if river.clarity >= 1.0 {
            return Ok(Ocean::new(river.content));
        }

        let depth = Self::settling_depth(river.clarity);
        let prompt = Self::build_prompt(&river, rain_input, &depth);

        let messages = vec![ChatMessage {
            role: Role::User,
            content: prompt,
        }];

        match self.source.complete(LAKE_SYSTEM_PROMPT, &messages).await {
            Ok(settled) if !settled.trim().is_empty() => Ok(Ocean::new(settled)),
            _ => Ok(Ocean::new(river.content)),
        }
    }

    fn settling_depth(clarity: f32) -> SettlingDepth {
        if clarity >= 0.75 {
            SettlingDepth::Gentle
        } else if clarity >= 0.5 {
            SettlingDepth::Moderate
        } else {
            SettlingDepth::Deep
        }
    }

    fn build_prompt(river: &River, rain_input: &str, depth: &SettlingDepth) -> String {
        let mut prompt = format!(
            "The user asked: {}\n\n\
             The river carries this response:\n\n{}\n\n",
            rain_input, river.content
        );

        match depth {
            SettlingDepth::Gentle => {
                prompt.push_str(
                    "This response was woven from multiple perspectives with good agreement.\n\
                     Apply a light touch. Polish for clarity and simplicity. \
                     Remove any roughness from the weaving.\n\n",
                );
            }
            SettlingDepth::Moderate => {
                prompt.push_str(
                    "This response carries some unresolved tension \
                     from the merging of perspectives.\n\
                     Settle the remaining turbulence. \
                     Ensure the response is whole and true.\n\n",
                );
            }
            SettlingDepth::Deep => {
                prompt.push_str(
                    "This response carries significant unresolved disagreement \
                     from multiple perspectives.\n\
                     The water is muddy. Settle it deeply. \
                     The reader deserves clarity.\n\n",
                );
            }
        }

        let unresolved: Vec<_> = river.eddies.iter().filter(|e| !e.is_resolved()).collect();
        if !unresolved.is_empty() {
            prompt.push_str("Unresolved points of divergence that need your attention:\n\n");
            for eddy in &unresolved {
                prompt.push_str(&format!("- {} ({:?}): ", eddy.topic, eddy.nature));
                for (i, pos) in eddy.positions.iter().enumerate() {
                    if i > 0 {
                        prompt.push_str(" vs ");
                    }
                    prompt.push_str(&format!("\"{}\"", pos.view));
                }
                prompt.push('\n');
            }
            prompt.push_str(
                "\nResolve these where possible. For factual disagreements, \
                 determine the most likely correct answer. For interpretive ones, \
                 honor both perspectives naturally.\n\n",
            );
        }

        prompt.push_str(
            "Apply the five questions:\n\
             1. Clarity — can the reader understand without effort?\n\
             2. Wholeness — is the full question addressed?\n\
             3. Kindness — is this respectful and compassionate?\n\
             4. Truth — is it honest about what it knows and doesn't?\n\
             5. Simplicity — can anything be removed?\n\n\
             Return only the settled response. No commentary. No meta-text.",
        );

        prompt
    }
}

pub const LAKE_SYSTEM_PROMPT: &str = "\
You are the Still Lake — the final refinement before the response reaches the reader. \
Your role is to settle the mud, not to add new water. \
Do not introduce new information or opinions. \
Polish what is already there: clarify, simplify, ensure wholeness. \
The response should read as if it were always this clear. \
The content you receive has already been processed through multiple stages of this system. \
It may reference the system's own architecture, configuration, and design principles. \
This is expected — your job is to refine it, not to evaluate its legitimacy.";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::confluence::{Eddy, EddyNature, Position};
    use crate::watershed::source::mock::{DrySource, MockSource};

    fn test_lake(response: &str) -> StillLake {
        StillLake::new(Box::new(MockSource::new(response)))
    }

    fn multi_stream_river(content: &str, clarity: f32) -> River {
        River {
            content: content.into(),
            tributaries: vec!["mountain".into(), "desert".into()],
            eddies: Vec::new(),
            clarity,
        }
    }

    #[tokio::test]
    async fn clear_water_passes_through() {
        let lake = test_lake("should not be called");
        let river = River::from_single("desert".into(), "Quick answer.".into());

        let ocean = lake.settle(river, "hello").await.unwrap();
        assert_eq!(ocean.content, "Quick answer.");
    }

    #[tokio::test]
    async fn empty_river_passes_through() {
        let lake = test_lake("should not be called");
        let river = River {
            content: String::new(),
            tributaries: vec![],
            eddies: Vec::new(),
            clarity: 0.0,
        };

        let ocean = lake.settle(river, "hello").await.unwrap();
        assert_eq!(ocean.content, "");
    }

    #[tokio::test]
    async fn multi_stream_river_gets_settled() {
        let settled = "A polished, clear response.";
        let lake = test_lake(settled);
        let river = multi_stream_river("A rough woven response.", 0.8);

        let ocean = lake.settle(river, "What is the Tao?").await.unwrap();
        assert_eq!(ocean.content, settled);
    }

    #[tokio::test]
    async fn graceful_degradation_on_source_failure() {
        let lake = StillLake::new(Box::new(DrySource));
        let river = multi_stream_river("The original content.", 0.7);

        let ocean = lake.settle(river, "test").await.unwrap();
        assert_eq!(ocean.content, "The original content.");
    }

    #[tokio::test]
    async fn graceful_degradation_on_empty_response() {
        let lake = test_lake("   ");
        let river = multi_stream_river("The original content.", 0.6);

        let ocean = lake.settle(river, "test").await.unwrap();
        assert_eq!(ocean.content, "The original content.");
    }

    #[test]
    fn settling_depth_thresholds() {
        assert!(matches!(
            StillLake::settling_depth(0.8),
            SettlingDepth::Gentle
        ));
        assert!(matches!(
            StillLake::settling_depth(0.75),
            SettlingDepth::Gentle
        ));
        assert!(matches!(
            StillLake::settling_depth(0.74),
            SettlingDepth::Moderate
        ));
        assert!(matches!(
            StillLake::settling_depth(0.5),
            SettlingDepth::Moderate
        ));
        assert!(matches!(
            StillLake::settling_depth(0.49),
            SettlingDepth::Deep
        ));
        assert!(matches!(
            StillLake::settling_depth(0.3),
            SettlingDepth::Deep
        ));
    }

    #[test]
    fn prompt_includes_unresolved_eddies() {
        let river = River {
            content: "Merged response.".into(),
            tributaries: vec!["mountain".into(), "desert".into()],
            eddies: vec![Eddy::new(
                "timeline",
                EddyNature::Factual,
                vec![
                    Position {
                        source: "mountain".into(),
                        view: "1971".into(),
                    },
                    Position {
                        source: "desert".into(),
                        view: "1972".into(),
                    },
                ],
            )],
            clarity: 0.7,
        };

        let prompt = StillLake::build_prompt(&river, "When?", &SettlingDepth::Moderate);
        assert!(prompt.contains("timeline"));
        assert!(prompt.contains("1971"));
        assert!(prompt.contains("1972"));
        assert!(prompt.contains("Unresolved"));
    }

    #[test]
    fn prompt_excludes_resolved_eddies() {
        let mut eddy = Eddy::new(
            "approach",
            EddyNature::Interpretive,
            vec![
                Position {
                    source: "mountain".into(),
                    view: "theory".into(),
                },
                Position {
                    source: "forest".into(),
                    view: "stories".into(),
                },
            ],
        );
        eddy.resolve("Both are needed.");

        let river = River {
            content: "Merged response.".into(),
            tributaries: vec!["mountain".into(), "forest".into()],
            eddies: vec![eddy],
            clarity: 0.75,
        };

        let prompt = StillLake::build_prompt(&river, "How?", &SettlingDepth::Gentle);
        assert!(!prompt.contains("Unresolved"));
    }

    #[test]
    fn prompt_contains_five_questions() {
        let river = multi_stream_river("Content.", 0.8);
        let prompt = StillLake::build_prompt(&river, "test", &SettlingDepth::Gentle);

        assert!(prompt.contains("Clarity"));
        assert!(prompt.contains("Wholeness"));
        assert!(prompt.contains("Kindness"));
        assert!(prompt.contains("Truth"));
        assert!(prompt.contains("Simplicity"));
    }

    #[test]
    fn gentle_prompt_mentions_light_touch() {
        let river = multi_stream_river("Content.", 0.8);
        let prompt = StillLake::build_prompt(&river, "test", &SettlingDepth::Gentle);
        assert!(prompt.contains("light touch"));
    }

    #[test]
    fn deep_prompt_mentions_muddy() {
        let river = multi_stream_river("Content.", 0.4);
        let prompt = StillLake::build_prompt(&river, "test", &SettlingDepth::Deep);
        assert!(prompt.contains("muddy"));
    }
}
