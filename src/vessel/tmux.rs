use tokio::process::Command;

use crate::error::FlowError;

/// Manages a persistent Claude process in a tmux window.
pub struct TmuxVessel {
    session: String,
    window: String,
    model: String,
    initialized: bool,
}

impl TmuxVessel {
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
    pub async fn prepare(&mut self, system_prompt: &str) -> Result<(), FlowError> {
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

            self.start_claude(system_prompt).await?;
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

                self.start_claude(system_prompt).await?;
            }
        }

        self.initialized = true;
        Ok(())
    }

    /// Start a claude process inside this vessel's window.
    async fn start_claude(&self, system_prompt: &str) -> Result<(), FlowError> {
        let claude_cmd = format!(
            "claude --model {} --system-prompt '{}'",
            self.model,
            system_prompt.replace('\'', "'\\''")
        );
        Command::new("tmux")
            .args(["send-keys", "-t", &self.target(), &claude_cmd, "Enter"])
            .status()
            .await
            .map_err(|e| FlowError::ConfigError(format!("Failed to start claude in tmux: {e}")))?;

        // Give claude a moment to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        Ok(())
    }

    /// Send a message through the vessel and capture the response.
    ///
    /// The vessel sends input to the tmux pane and waits for the
    /// output to stabilize -- the water settles in the vessel.
    pub async fn send(&self, input: &str) -> Result<String, FlowError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        // Clear the pane to isolate the new response
        Command::new("tmux")
            .args(["send-keys", "-t", &self.target(), "", ""])
            .status()
            .await
            .ok();

        // Send the input
        Command::new("tmux")
            .args(["send-keys", "-t", &self.target(), input, "Enter"])
            .status()
            .await
            .map_err(|e| FlowError::SpringFailure {
                name: self.window.clone(),
                reason: format!("Failed to send to tmux: {e}"),
            })?;

        // Wait for response -- poll until the pane content stabilizes.
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

        // Extract the response (everything after the input)
        let response = last_content
            .lines()
            .skip_while(|line| !line.contains(input))
            .skip(1) // skip the input line itself
            .collect::<Vec<_>>()
            .join("\n")
            .trim()
            .to_string();

        Ok(response)
    }

    /// The name of this vessel's window.
    pub fn window_name(&self) -> &str {
        &self.window
    }

    /// The model running inside this vessel.
    pub fn model(&self) -> &str {
        &self.model
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tmux_target_format() {
        let vessel = TmuxVessel::new("tao-flow", "mountain", "claude-opus-4-6");
        assert_eq!(vessel.target(), "tao-flow:mountain");
    }

    #[test]
    fn vessel_knows_its_window() {
        let vessel = TmuxVessel::new("tao-flow", "desert", "claude-haiku-4-5-20251001");
        assert_eq!(vessel.window_name(), "desert");
        assert_eq!(vessel.model(), "claude-haiku-4-5-20251001");
    }
}
