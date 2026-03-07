use serde::{Deserialize, Serialize};

/// A message in the conversation history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

/// The role of a message sender.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
}

/// User preferences -- the atmospheric conditions.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Preferences {
    pub style: Option<String>,
    pub verbosity: Option<f32>,
}

/// Session context -- the weather pattern.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SessionContext {
    pub session_id: Option<String>,
    pub metadata: std::collections::HashMap<String, String>,
}

/// Context -- the atmosphere before rain falls.
///
/// Vapor is the invisible precursor to rain. It represents
/// conversation history, user preferences, and session state
/// that exist before any new input arrives.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vapor {
    pub conversation_history: Vec<Message>,
    pub user_preferences: Preferences,
    pub session_context: SessionContext,
    /// -1.0 cold/analytical, +1.0 warm/emotional
    pub emotional_temperature: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vapor_defaults_to_neutral() {
        let vapor = Vapor::default();
        assert!(vapor.conversation_history.is_empty());
        assert_eq!(vapor.emotional_temperature, 0.0);
    }

    #[test]
    fn vapor_accumulates_history() {
        let mut vapor = Vapor::default();
        vapor.conversation_history.push(Message {
            role: Role::User,
            content: "What is the Tao?".into(),
        });
        vapor.conversation_history.push(Message {
            role: Role::Assistant,
            content: "The Tao that can be told is not the eternal Tao.".into(),
        });
        assert_eq!(vapor.conversation_history.len(), 2);
        assert_eq!(vapor.conversation_history[0].role, Role::User);
        assert_eq!(vapor.conversation_history[1].role, Role::Assistant);
    }

    #[test]
    fn vapor_serializes_to_json() {
        let vapor = Vapor {
            emotional_temperature: 0.5,
            ..Default::default()
        };
        let json = serde_json::to_string(&vapor).unwrap();
        let restored: Vapor = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.emotional_temperature, 0.5);
    }
}
