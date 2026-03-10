use thiserror::Error;

#[derive(Error, Debug)]
pub enum FlowError {
    #[error("Spring '{name}' failed to respond: {reason}")]
    SpringFailure { name: String, reason: String },

    #[error("No springs produced water for this rain")]
    Drought,

    #[error("Confluence failed to merge streams: {0}")]
    ConfluenceFailure(String),

    #[error("Decomposition failed: {0}")]
    DecompositionFailure(String),

    #[error("Vessel failed: {0}")]
    VesselFailure(String),

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
