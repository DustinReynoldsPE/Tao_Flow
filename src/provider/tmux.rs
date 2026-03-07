use async_trait::async_trait;
use tokio::process::Command;

use super::{ChatMessage, LlmProvider};
use crate::error::FlowError;

/// Tmux-managed persistent Claude session.
///
/// Each spring can be a persistent `claude` process running in a
/// tmux window. The session remembers the conversation naturally --
/// vapor flows without explicit management.
///
/// tmux session layout:
/// ```text
/// tao-flow (session)
///   mountain (window) — claude --model opus, persistent
///   desert   (window) — claude --model haiku, persistent
///   forest   (window) — claude --model sonnet, persistent
/// ```
///
/// "Returning is the motion of the Tao." -- Chapter 40
/// The conversation cycles naturally, each exchange deepening
/// the riverbed without the system carrying the water's memory.
pub struct TmuxProvider {
    session: String,
    window: String,
    model: String,
    initialized: bool,
}

impl TmuxProvider {
    pub fn new(
        session: impl Into<String>,
        window: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            session: session.into(),
            window: window.into(),
            model: model.into(),
            initialized: false,
        }
    }

    fn target(&self) -> String {
        format!("{}:{}", self.session, self.window)
    }

    /// Ensure the tmux session and window exist with a claude process.
    pub async fn initialize(&mut self, system_prompt: &str) -> Result<(), FlowError> {
        if self.initialized {
            return Ok(());
        }

        // Check if tmux is available
        let tmux_check = Command::new("tmux").arg("-V").output().await;
        if tmux_check.is_err() || !tmux_check.unwrap().status.success() {
            return Err(FlowError::ConfigError(
                "tmux is not installed or not available".into(),
            ));
        }

        // Check if session exists, create if not
        let has_session = Command::new("tmux")
            .args(["has-session", "-t", &self.session])
            .output()
            .await
            .map(|o| o.status.success())
            .unwrap_or(false);

        if !has_session {
            let status = Command::new("tmux")
                .args(["new-session", "-d", "-s", &self.session, "-n", &self.window])
                .status()
                .await
                .map_err(|e| {
                    FlowError::ConfigError(format!("Failed to create tmux session: {e}"))
                })?;

            if !status.success() {
                return Err(FlowError::ConfigError(
                    "Failed to create tmux session".into(),
                ));
            }

            // Start claude in the window
            let claude_cmd = format!(
                "claude --model {} --system-prompt '{}'",
                self.model,
                system_prompt.replace('\'', "'\\''")
            );
            Command::new("tmux")
                .args(["send-keys", "-t", &self.target(), &claude_cmd, "Enter"])
                .status()
                .await
                .map_err(|e| {
                    FlowError::ConfigError(format!("Failed to start claude in tmux: {e}"))
                })?;

            // Give claude a moment to start
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        } else {
            // Session exists; check if window exists
            let has_window = Command::new("tmux")
                .args(["list-windows", "-t", &self.session, "-F", "#{window_name}"])
                .output()
                .await
                .map(|o| {
                    String::from_utf8_lossy(&o.stdout)
                        .lines()
                        .any(|l| l.trim() == self.window)
                })
                .unwrap_or(false);

            if !has_window {
                Command::new("tmux")
                    .args(["new-window", "-t", &self.session, "-n", &self.window])
                    .status()
                    .await
                    .map_err(|e| {
                        FlowError::ConfigError(format!("Failed to create tmux window: {e}"))
                    })?;

                let claude_cmd = format!(
                    "claude --model {} --system-prompt '{}'",
                    self.model,
                    system_prompt.replace('\'', "'\\''")
                );
                Command::new("tmux")
                    .args(["send-keys", "-t", &self.target(), &claude_cmd, "Enter"])
                    .status()
                    .await
                    .map_err(|e| {
                        FlowError::ConfigError(format!("Failed to start claude in tmux: {e}"))
                    })?;

                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            }
        }

        self.initialized = true;
        Ok(())
    }
}

#[async_trait]
impl LlmProvider for TmuxProvider {
    async fn complete(&self, _system: &str, messages: &[ChatMessage]) -> Result<String, FlowError> {
        // For tmux sessions, the system prompt was set at initialization.
        // We only send the latest user message -- the session remembers
        // the conversation naturally.
        let prompt = messages
            .last()
            .map(|m| m.content.clone())
            .unwrap_or_default();

        if prompt.is_empty() {
            return Ok(String::new());
        }

        // Clear the pane to isolate the new response
        Command::new("tmux")
            .args(["send-keys", "-t", &self.target(), "", ""])
            .status()
            .await
            .ok();

        // Send the prompt
        Command::new("tmux")
            .args(["send-keys", "-t", &self.target(), &prompt, "Enter"])
            .status()
            .await
            .map_err(|e| FlowError::SpringFailure {
                name: self.window.clone(),
                reason: format!("Failed to send to tmux: {e}"),
            })?;

        // Wait for response -- this is the hard part.
        // We poll the pane content until it stops changing.
        // This is a simple approach; a more sophisticated one would
        // watch for the prompt marker.
        let mut last_content = String::new();
        let mut stable_count = 0;
        let max_wait = 60; // seconds
        let poll_interval = tokio::time::Duration::from_millis(500);

        for _ in 0..(max_wait * 2) {
            tokio::time::sleep(poll_interval).await;

            let output = Command::new("tmux")
                .args(["capture-pane", "-t", &self.target(), "-p"])
                .output()
                .await
                .map_err(|e| FlowError::SpringFailure {
                    name: self.window.clone(),
                    reason: format!("Failed to capture pane: {e}"),
                })?;

            let content = String::from_utf8_lossy(&output.stdout).to_string();

            if content == last_content {
                stable_count += 1;
                if stable_count >= 4 {
                    // Content has been stable for 2 seconds
                    break;
                }
            } else {
                stable_count = 0;
                last_content = content;
            }
        }

        // Extract the response (everything after our prompt)
        let response = last_content
            .lines()
            .skip_while(|line| !line.contains(&prompt))
            .skip(1) // skip the prompt line itself
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmux_target_format() {
        let provider = TmuxProvider::new("tao-flow", "mountain", "claude-opus-4-6");
        assert_eq!(provider.target(), "tao-flow:mountain");
    }
}
