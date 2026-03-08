use async_trait::async_trait;
use tokio::sync::Mutex;

use super::{ChatMessage, LlmSource};
use crate::error::FlowError;
use crate::vessel::TmuxVessel;
use crate::water::Role;

/// An LlmSource backed by a persistent tmux pane.
///
/// The vessel carries the conversation naturally. Each exchange
/// lives in the pane's history. Only the latest message is sent;
/// the pane remembers what came before.
pub struct TmuxPaneSource {
    vessel: Mutex<TmuxVessel>,
}

impl TmuxPaneSource {
    pub fn new(vessel: TmuxVessel) -> Self {
        Self {
            vessel: Mutex::new(vessel),
        }
    }
}

#[async_trait]
impl LlmSource for TmuxPaneSource {
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError> {
        let mut vessel = self.vessel.lock().await;

        // Lazy preparation: the vessel initializes on first use.
        // The system prompt shapes the riverbed once; subsequent
        // calls flow through the same channel.
        vessel.prepare(system).await?;

        let last_user_message = messages
            .iter()
            .rfind(|m| m.role == Role::User)
            .map(|m| m.content.as_str())
            .unwrap_or("");

        if last_user_message.is_empty() {
            return Ok(String::new());
        }

        vessel.send(last_user_message).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The vessel source must be Send + Sync to flow through concurrent springs.
    #[test]
    fn tmux_pane_source_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<TmuxPaneSource>();
    }

    #[test]
    fn construction() {
        let vessel = TmuxVessel::new("test-session", "test-window", "test-model");
        let _source = TmuxPaneSource::new(vessel);
    }
}
