use thiserror::Error;

/// Errors that arise as eddies in the flow.
///
/// "Failure is an opportunity." -- Tao Te Ching, Chapter 79
#[derive(Error, Debug)]
pub enum FlowError {
    #[error("Spring '{name}' failed to respond: {reason}")]
    SpringFailure { name: String, reason: String },

    #[error("No springs produced water for this rain")]
    Drought,

    #[error("Confluence failed to merge streams: {0}")]
    ConfluenceFailure(String),

    #[error("Still Lake failed to clarify: {0}")]
    ClarityFailure(String),

    #[error("LLM provider error: {0}")]
    ProviderError(#[from] reqwest::Error),

    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn errors_display_clearly() {
        let err = FlowError::Drought;
        assert_eq!(err.to_string(), "No springs produced water for this rain");

        let err = FlowError::SpringFailure {
            name: "mountain".into(),
            reason: "timeout".into(),
        };
        assert!(err.to_string().contains("mountain"));
        assert!(err.to_string().contains("timeout"));
    }
}
