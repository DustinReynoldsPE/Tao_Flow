/// Classifies rain content into mineral tags for spring affinity.
///
/// Springs declare affinities. Rain carries minerals. When they match,
/// a spring's relevance rises. The classifier reads the rain and
/// identifies what substance it carries.
pub struct MineralClassifier;

impl MineralClassifier {
    pub fn classify(input: &str) -> Vec<String> {
        let lower = input.to_lowercase();
        let mut minerals = Vec::new();

        if Self::matches(
            &lower,
            &[
                "philosophy",
                "meaning",
                "nature of",
                "existence",
                "consciousness",
                "tao",
                "dao",
                "wisdom",
                "truth",
                "metaphysic",
                "why do",
                "what is the nature",
                "fundamental",
                "purpose of",
            ],
        ) {
            minerals.push("philosophy".into());
        }

        if Self::matches(
            &lower,
            &[
                "architect",
                "design",
                "system",
                "structure",
                "pattern",
                "framework",
                "module",
                "component",
                "interface",
                "abstraction",
            ],
        ) {
            minerals.push("architecture".into());
        }

        if Self::matches(
            &lower,
            &[
                "analyze",
                "analysis",
                "compare",
                "contrast",
                "evaluate",
                "explain why",
                "reason",
                "logic",
                "proof",
                "theorem",
                "cause",
                "effect",
                "implication",
            ],
        ) {
            minerals.push("deep_reasoning".into());
        }

        if Self::matches(
            &lower,
            &[
                "how to",
                "quick",
                "fix",
                "error",
                "bug",
                "what is",
                "define",
                "list",
                "summarize",
            ],
        ) {
            minerals.push("quick_answers".into());
        }

        if Self::matches(
            &lower,
            &["format", "markdown", "table", "json", "csv", "template"],
        ) {
            minerals.push("formatting".into());
        }

        if Self::matches(
            &lower,
            &[
                "story",
                "narrative",
                "creative",
                "write a",
                "poem",
                "fiction",
                "character",
                "imagine",
                "metaphor",
            ],
        ) {
            minerals.push("narrative".into());
        }

        if Self::matches(&lower, &["poem", "poetry", "verse", "haiku", "sonnet"]) {
            minerals.push("poetry".into());
        }

        if Self::matches(
            &lower,
            &[
                "feel",
                "emotion",
                "empathy",
                "support",
                "comfort",
                "grief",
                "anxiety",
                "relationship",
                "personal",
            ],
        ) {
            minerals.push("empathy".into());
        }

        if Self::matches(&lower, &["funny", "humor", "joke", "comedy", "witty"]) {
            minerals.push("humor".into());
        }

        if Self::matches(
            &lower,
            &[
                "creativity",
                "inventive",
                "original",
                "brainstorm",
                "innovate",
            ],
        ) {
            minerals.push("creativity".into());
        }

        minerals
    }

    fn matches(text: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|kw| text.contains(kw))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classifies_philosophical_content() {
        let minerals = MineralClassifier::classify(
            "What is the nature of consciousness and its relationship to the Tao?",
        );
        assert!(minerals.contains(&"philosophy".to_string()));
    }

    #[test]
    fn classifies_technical_content() {
        let minerals = MineralClassifier::classify(
            "Design a distributed system architecture for real-time collaboration",
        );
        assert!(minerals.contains(&"architecture".to_string()));
    }

    #[test]
    fn classifies_creative_content() {
        let minerals = MineralClassifier::classify("Write a poem about the changing of seasons");
        assert!(minerals.contains(&"narrative".to_string()));
        assert!(minerals.contains(&"poetry".to_string()));
    }

    #[test]
    fn classifies_quick_answers() {
        let minerals = MineralClassifier::classify("What is a closure in Rust?");
        assert!(minerals.contains(&"quick_answers".to_string()));
    }

    #[test]
    fn multiple_minerals_for_mixed_content() {
        let minerals =
            MineralClassifier::classify("Analyze the philosophy behind system design patterns");
        assert!(minerals.contains(&"philosophy".to_string()));
        assert!(minerals.contains(&"architecture".to_string()));
        assert!(minerals.contains(&"deep_reasoning".to_string()));
    }

    #[test]
    fn empty_for_unrecognized_content() {
        let minerals = MineralClassifier::classify("hello world");
        assert!(minerals.is_empty());
    }

    #[test]
    fn case_insensitive() {
        let minerals = MineralClassifier::classify("PHILOSOPHY of DESIGN");
        assert!(minerals.contains(&"philosophy".to_string()));
        assert!(minerals.contains(&"architecture".to_string()));
    }
}
