use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Role {
    User,
    Assistant,
}

/// Context carried between flows.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Vapor {
    pub conversation_history: Vec<Message>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vapor_starts_empty() {
        let vapor = Vapor::default();
        assert!(vapor.conversation_history.is_empty());
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
    }

    #[test]
    fn vapor_serializes() {
        let mut vapor = Vapor::default();
        vapor.conversation_history.push(Message {
            role: Role::User,
            content: "hello".into(),
        });
        let json = serde_json::to_string(&vapor).unwrap();
        let restored: Vapor = serde_json::from_str(&json).unwrap();
        assert_eq!(restored.conversation_history.len(), 1);
    }
}
