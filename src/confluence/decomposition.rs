use crate::error::FlowError;
use crate::water::Role;
use crate::watershed::source::{ChatMessage, LlmSource};

/// Breaks Storm-level rain into independent sub-questions.
///
/// Like a watershed ridge that divides a single downpour
/// into streams flowing through separate valleys.
pub struct Decomposer {
    source: Box<dyn LlmSource>,
}

impl Decomposer {
    pub fn new(source: Box<dyn LlmSource>) -> Self {
        Self { source }
    }

    pub async fn decompose(&self, input: &str) -> Result<Vec<String>, FlowError> {
        let messages = vec![ChatMessage {
            role: Role::User,
            content: format!(
                "Break this complex request into 2-5 independent sub-questions. \
                 Each sub-question will be answered by a separate system with NO \
                 access to the other sub-questions. Each must name all subjects \
                 explicitly — never use \"all three\", \"these\", or \"together\". \
                 Do NOT include synthesis or comparison questions — synthesis \
                 happens automatically after all sub-questions are answered.\n\n\
                 Original request: {}\n\n\
                 Return ONLY the sub-questions, one per line, prefixed with \"Q: \"",
                input
            ),
        }];

        let response = self
            .source
            .complete(DECOMPOSER_SYSTEM_PROMPT, &messages)
            .await
            .map_err(|e| FlowError::DecompositionFailure(e.to_string()))?;

        let questions = Self::parse_questions(&response);
        if questions.len() < 2 {
            return Err(FlowError::DecompositionFailure(
                "Could not decompose into multiple sub-questions".into(),
            ));
        }

        Ok(questions)
    }

    fn parse_questions(response: &str) -> Vec<String> {
        let questions: Vec<String> = response
            .lines()
            .filter_map(|line| {
                // Strip markdown bold/italic wrapping (e.g. **Q: text**)
                let trimmed = line
                    .trim()
                    .trim_start_matches('*')
                    .trim_end_matches('*')
                    .trim();
                trimmed
                    .strip_prefix("Q: ")
                    .or_else(|| trimmed.strip_prefix("Q:"))
                    .or_else(|| {
                        // Handle Q1:, Q2:, Q10:, etc.
                        let rest = trimmed.strip_prefix('Q')?;
                        let rest = rest.strip_prefix(|c: char| c.is_ascii_digit())?;
                        let rest = rest.trim_start_matches(|c: char| c.is_ascii_digit());
                        rest.strip_prefix(": ").or_else(|| rest.strip_prefix(':'))
                    })
                    .map(|rest| rest.trim().to_string())
                    .filter(|q| !q.is_empty())
            })
            .collect();

        if !questions.is_empty() {
            return questions;
        }

        // Fallback: numbered list format (1. question, 2) question)
        response
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                let after_digit = trimmed.strip_prefix(|c: char| c.is_ascii_digit())?;
                let rest = after_digit
                    .strip_prefix(". ")
                    .or_else(|| after_digit.strip_prefix(") "))?;
                let q = rest.trim().to_string();
                if q.is_empty() {
                    None
                } else {
                    Some(q)
                }
            })
            .collect()
    }
}

const DECOMPOSER_SYSTEM_PROMPT: &str = "\
You decompose complex requests into independent sub-questions. \
Each sub-question will be answered by a separate system that has NO access \
to the other sub-questions or their answers. Therefore: \
1. Each sub-question must be fully self-contained — name all subjects, \
   traditions, concepts, or entities explicitly. Never use references like \
   \"all three\", \"these traditions\", \"the above\", or \"together\". \
2. Do NOT include synthesis, comparison, or convergence questions. \
   A separate stage handles synthesis after all sub-questions are answered. \
3. Focus each sub-question on a distinct analytical angle. \
Return 2-5 sub-questions, no more. Prefix each with \"Q: \".";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::watershed::source::mock::MockSource;

    #[tokio::test]
    async fn parses_q_prefix_format() {
        let response =
            "Q: What is the philosophical foundation?\nQ: How does it work in practice?\nQ: What are the key examples?";
        let source = MockSource::new(response);
        let decomposer = Decomposer::new(Box::new(source));

        let questions = decomposer.decompose("complex request").await.unwrap();
        assert_eq!(questions.len(), 3);
        assert_eq!(questions[0], "What is the philosophical foundation?");
        assert_eq!(questions[1], "How does it work in practice?");
        assert_eq!(questions[2], "What are the key examples?");
    }

    #[tokio::test]
    async fn parses_numbered_list_fallback() {
        let response =
            "1. What is the foundation?\n2. How does it work?\n3. What are the examples?";
        let source = MockSource::new(response);
        let decomposer = Decomposer::new(Box::new(source));

        let questions = decomposer.decompose("complex request").await.unwrap();
        assert_eq!(questions.len(), 3);
    }

    #[tokio::test]
    async fn fails_on_single_question() {
        let response = "Q: Just one question here.";
        let source = MockSource::new(response);
        let decomposer = Decomposer::new(Box::new(source));

        let result = decomposer.decompose("simple request").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn fails_on_empty_response() {
        let response = "";
        let source = MockSource::new(response);
        let decomposer = Decomposer::new(Box::new(source));

        let result = decomposer.decompose("request").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn skips_non_question_lines() {
        let response = "Here are the sub-questions:\n\nQ: First question?\nQ: Second question?";
        let source = MockSource::new(response);
        let decomposer = Decomposer::new(Box::new(source));

        let questions = decomposer.decompose("request").await.unwrap();
        assert_eq!(questions.len(), 2);
    }

    #[test]
    fn parse_handles_various_formats() {
        let result = Decomposer::parse_questions("Q: First\nQ: Second");
        assert_eq!(result.len(), 2);

        let result = Decomposer::parse_questions("Q:First\nQ:Second");
        assert_eq!(result.len(), 2);

        let result = Decomposer::parse_questions("1. First\n2. Second");
        assert_eq!(result.len(), 2);

        let result = Decomposer::parse_questions("1) First\n2) Second");
        assert_eq!(result.len(), 2);

        let result = Decomposer::parse_questions("Q1: First\nQ2: Second\nQ3: Third");
        assert_eq!(result.len(), 3);

        let result = Decomposer::parse_questions("Q1:First\nQ2:Second");
        assert_eq!(result.len(), 2);

        let result = Decomposer::parse_questions("**Q: First question**\n**Q: Second question**");
        assert_eq!(result.len(), 2);
        assert_eq!(result[0], "First question");
    }
}
