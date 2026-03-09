use crate::confluence::{ConfluencePool, Decomposer};
use crate::error::FlowError;
use crate::still_lake::StillLake;
use crate::water::rain::Volume;
use crate::water::{Message, Ocean, Rain, Role, Stream, Vapor};
use crate::watershed::{VolumeSensor, Watershed};

pub struct TaoFlow {
    watershed: Watershed,
    confluence: ConfluencePool,
    still_lake: StillLake,
    decomposer: Option<Decomposer>,
    vapor: Vapor,
    max_depth: usize,
}

impl TaoFlow {
    pub fn new(watershed: Watershed, confluence: ConfluencePool, still_lake: StillLake) -> Self {
        Self {
            watershed,
            confluence,
            still_lake,
            decomposer: None,
            vapor: Vapor::default(),
            max_depth: 1,
        }
    }

    pub fn with_decomposer(mut self, decomposer: Decomposer) -> Self {
        self.decomposer = Some(decomposer);
        self
    }

    pub fn with_max_depth(mut self, max_depth: usize) -> Self {
        self.max_depth = max_depth;
        self
    }

    pub async fn flow(&mut self, user_input: &str) -> Result<String, FlowError> {
        let rain = Rain::new(user_input, self.vapor.clone());
        let ocean = self.flow_inner(rain, 0).await?;

        self.vapor.conversation_history.push(Message {
            role: Role::User,
            content: user_input.to_string(),
        });
        self.vapor.conversation_history.push(Message {
            role: Role::Assistant,
            content: ocean.content.clone(),
        });

        Ok(ocean.content)
    }

    /// Routes rain through either recursive or single-pass flow.
    ///
    /// Storm volume at depth < max_depth triggers decomposition.
    /// All other rain, or failed decomposition, uses single pass.
    async fn flow_inner(&self, rain: Rain, depth: usize) -> Result<Ocean, FlowError> {
        let volume = VolumeSensor::new().sense(&rain);

        if volume == Volume::Storm && depth < self.max_depth && self.decomposer.is_some() {
            match self.decompose_and_flow(rain.clone(), depth).await {
                Ok(ocean) => Ok(ocean),
                Err(_) => self.single_pass(rain).await,
            }
        } else {
            self.single_pass(rain).await
        }
    }

    /// The return: decompose, flow each part, reassemble.
    async fn decompose_and_flow(&self, rain: Rain, depth: usize) -> Result<Ocean, FlowError> {
        let decomposer = self.decomposer.as_ref().unwrap();
        let sub_questions = decomposer.decompose(&rain.raw_input).await?;

        // Each sub-question flows through the full journey independently.
        // Sub-questions carry the parent's vapor for context but do not update it.
        let futures: Vec<_> = sub_questions
            .iter()
            .map(|q| self.flow_inner(Rain::new(q.as_str(), rain.vapor.clone()), depth + 1))
            .collect();

        let results = futures::future::join_all(futures).await;

        // Collect successful sub-flows as tributaries for higher confluence
        let sub_streams: Vec<Stream> = results
            .into_iter()
            .enumerate()
            .filter_map(|(i, result)| {
                result
                    .ok()
                    .filter(|o| o.has_substance())
                    .map(|ocean| Stream::new(format!("tributary_{}", i + 1), ocean.content))
            })
            .collect();

        if sub_streams.is_empty() {
            return Err(FlowError::Drought);
        }

        // Higher confluence: the sub-rivers merge
        let river = self.confluence.merge(sub_streams, &rain.raw_input).await?;

        // Final settling
        self.still_lake.settle(river, &rain.raw_input).await
    }

    /// The single-pass journey: watershed → confluence → still lake → ocean.
    async fn single_pass(&self, mut rain: Rain) -> Result<Ocean, FlowError> {
        let streams = self.watershed.receive_rain(&mut rain).await;

        if streams.is_empty() {
            return Err(FlowError::Drought);
        }

        let river = self.confluence.merge(streams, &rain.raw_input).await?;
        self.still_lake.settle(river, &rain.raw_input).await
    }

    pub fn vapor(&self) -> &Vapor {
        &self.vapor
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_springs;
    use crate::watershed::source::mock::{DrySource, MockSource};
    use crate::watershed::spring::SpringConfig;
    use crate::watershed::MountainSpring;
    use std::collections::HashMap;

    fn test_confluence(response: &str) -> ConfluencePool {
        ConfluencePool::new(Box::new(MockSource::new(response)))
    }

    fn test_lake(response: &str) -> StillLake {
        StillLake::new(Box::new(MockSource::new(response)))
    }

    fn test_decomposer(response: &str) -> Decomposer {
        Decomposer::new(Box::new(MockSource::new(response)))
    }

    // --- Existing single-pass tests ---

    #[tokio::test]
    async fn rain_flows_to_ocean() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("The Tao is the way."),
            test_springs::desert("It's the way."),
        ]);
        let confluence = test_confluence("The Tao is the way.");
        let lake = test_lake("The Tao is the way, settled.");
        let mut tao = TaoFlow::new(watershed, confluence, lake);
        let result = tao.flow("What is the Tao?").await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn droplet_uses_only_desert() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Should not appear."),
            test_springs::desert("Hello!"),
        ]);
        // Single stream: clarity 1.0, lake does nothing (wu wei)
        let confluence = test_confluence("unused");
        let lake = test_lake("should not be called");
        let mut tao = TaoFlow::new(watershed, confluence, lake);
        let result = tao.flow("hi").await.unwrap();
        assert_eq!(result, "Hello!");
    }

    #[tokio::test]
    async fn three_springs_merge_and_settle() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Deep analysis of the question."),
            test_springs::desert("Quick, direct answer."),
            test_springs::forest("A story about the answer."),
        ]);
        // MockSource returns same response for all calls (detection, weaving, settling)
        let woven = "A woven response from three perspectives.";
        let confluence = test_confluence(woven);
        let lake = test_lake(woven);
        let mut tao = TaoFlow::new(watershed, confluence, lake);

        let result = tao
            .flow("Explain the nature of water in philosophy and storytelling and practice")
            .await
            .unwrap();

        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn vapor_accumulates_across_flows() {
        let watershed = Watershed::new(vec![test_springs::desert("Response.")]);
        let confluence = test_confluence("unused");
        let lake = test_lake("should not be called");
        let mut tao = TaoFlow::new(watershed, confluence, lake);

        tao.flow("First").await.unwrap();
        tao.flow("Second").await.unwrap();

        assert_eq!(tao.vapor().conversation_history.len(), 4);
        assert_eq!(tao.vapor().conversation_history[0].content, "First");
        assert_eq!(tao.vapor().conversation_history[2].content, "Second");
    }

    #[tokio::test]
    async fn drought_when_all_springs_dry() {
        let watershed = Watershed::new(vec![Box::new(MountainSpring::new(
            SpringConfig {
                name: "mountain".into(),
                nature: "deep".into(),
                affinities: HashMap::new(),
            },
            Box::new(DrySource),
        )) as Box<dyn crate::watershed::Spring>]);
        let confluence = test_confluence("unused");
        let lake = test_lake("unused");
        let mut tao = TaoFlow::new(watershed, confluence, lake);
        let result = tao.flow("hello").await;
        assert!(result.is_err());
    }

    // --- Phase 6: The Return ---

    #[tokio::test]
    async fn storm_with_decomposer_flows() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Mountain's wisdom on the sub-topic."),
            test_springs::desert("Desert's speed on the sub-topic."),
        ]);
        let confluence = test_confluence("Woven response.");
        let lake = test_lake("Settled response.");
        let decomposer = test_decomposer(
            "Q: What is the philosophical foundation?\nQ: What are the practical applications?",
        );

        let mut tao = TaoFlow::new(watershed, confluence, lake).with_decomposer(decomposer);

        // Storm-level input (>100 words)
        let storm_input = "word ".repeat(101);
        let result = tao.flow(&storm_input).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn storm_without_decomposer_single_passes() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Mountain's response."),
            test_springs::desert("Desert's response."),
        ]);
        let confluence = test_confluence("Woven.");
        let lake = test_lake("Settled.");
        let mut tao = TaoFlow::new(watershed, confluence, lake);

        let storm_input = "word ".repeat(101);
        let result = tao.flow(&storm_input).await.unwrap();
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn decomposition_failure_falls_back_to_single_pass() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Mountain."),
            test_springs::desert("Desert."),
        ]);
        let confluence = test_confluence("Woven.");
        let lake = test_lake("Settled.");
        // DrySource causes decomposition to fail
        let decomposer = Decomposer::new(Box::new(DrySource));

        let mut tao = TaoFlow::new(watershed, confluence, lake).with_decomposer(decomposer);

        let storm_input = "word ".repeat(101);
        let result = tao.flow(&storm_input).await.unwrap();
        // Graceful fallback: single-pass still produces output
        assert!(!result.is_empty());
    }

    #[tokio::test]
    async fn vapor_updated_once_after_recursive_flow() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Response."),
            test_springs::desert("Response."),
        ]);
        let confluence = test_confluence("Woven.");
        let lake = test_lake("Settled.");
        let decomposer = test_decomposer("Q: Sub one?\nQ: Sub two?");

        let mut tao = TaoFlow::new(watershed, confluence, lake).with_decomposer(decomposer);

        let storm_input = "word ".repeat(101);
        tao.flow(&storm_input).await.unwrap();

        // Only the top-level exchange is recorded, not sub-flows
        assert_eq!(tao.vapor().conversation_history.len(), 2);
        assert_eq!(tao.vapor().conversation_history[0].role, Role::User);
        assert_eq!(tao.vapor().conversation_history[1].role, Role::Assistant);
    }

    #[tokio::test]
    async fn non_storm_ignores_decomposer() {
        let watershed = Watershed::new(vec![test_springs::desert("Quick.")]);
        let confluence = test_confluence("unused");
        let lake = test_lake("unused");
        // Decomposer would fail if called (single question = error)
        let decomposer = test_decomposer("Q: Only one question.");

        let mut tao = TaoFlow::new(watershed, confluence, lake).with_decomposer(decomposer);

        // Shower-level input — should not trigger decomposition
        let result = tao.flow("What is the Tao?").await.unwrap();
        assert_eq!(result, "Quick.");
    }

    #[tokio::test]
    async fn max_depth_prevents_infinite_recursion() {
        let watershed = Watershed::new(vec![
            test_springs::mountain("Response."),
            test_springs::desert("Response."),
        ]);
        let confluence = test_confluence("Woven.");
        let lake = test_lake("Settled.");
        // Decomposer returns long sub-questions that would also be Storm volume
        let long_sub = "word ".repeat(110);
        let decomposer = test_decomposer(&format!("Q: {}\nQ: {}", long_sub, long_sub));

        let mut tao = TaoFlow::new(watershed, confluence, lake)
            .with_decomposer(decomposer)
            .with_max_depth(1);

        let storm_input = "word ".repeat(120);
        // Should complete without infinite recursion — sub-questions at depth 1
        // exceed max_depth so they single-pass
        let result = tao.flow(&storm_input).await.unwrap();
        assert!(!result.is_empty());
    }
}
