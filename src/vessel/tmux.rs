use tokio::process::Command;

use crate::error::FlowError;

/// Manages a persistent process in a tmux window.
///
/// The vessel is the pot, not the water. `with_command` sets
/// the process; `prepare()` creates the tmux window and starts it.
pub struct TmuxVessel {
    session: String,
    window: String,
    model: String,
    command: Option<String>,
    sentinel: Option<String>,
    input_delimiter: Option<String>,
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
            command: None,
            sentinel: None,
            input_delimiter: None,
            initialized: false,
        }
    }

    /// Override the process started in this vessel's window.
    /// By default, the vessel starts claude. Use this for testing
    /// with echo processes or other programs.
    pub fn with_command(mut self, command: impl Into<String>) -> Self {
        self.command = Some(command.into());
        self
    }

    /// Set a sentinel pattern that signals the process is ready for input.
    /// When set, the vessel waits for this pattern to appear after the
    /// input line instead of polling for content stability.
    pub fn with_sentinel(mut self, sentinel: impl Into<String>) -> Self {
        self.sentinel = Some(sentinel.into());
        self
    }

    /// Set a delimiter that marks the end of multi-line input.
    /// When set, the vessel sends this string after the input text,
    /// and uses it as the boundary between input and output in the pane.
    pub fn with_input_delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.input_delimiter = Some(delimiter.into());
        self
    }

    fn target(&self) -> String {
        format!("{}:{}", self.session, self.window)
    }

    /// Ensure the tmux session and window exist with the configured process.
    ///
    /// Requires `with_command()` to have been called — the vessel
    /// must know what process to start before it can prepare.
    pub async fn prepare(&mut self) -> Result<(), FlowError> {
        if self.initialized {
            return Ok(());
        }

        let cmd = self.command.as_ref().ok_or_else(|| {
            FlowError::VesselFailure(
                "Vessel has no command configured. Call with_command() before prepare().".into(),
            )
        })?;
        let cmd = cmd.clone();

        // Check if tmux is available
        let tmux_check = Command::new("tmux").arg("-V").output().await;
        if tmux_check.is_err() || !tmux_check.unwrap().status.success() {
            return Err(FlowError::VesselFailure(
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
                    FlowError::VesselFailure(format!("Failed to create tmux session: {e}"))
                })?;

            if !status.success() {
                return Err(FlowError::VesselFailure(
                    "Failed to create tmux session".into(),
                ));
            }

            self.start_process(&cmd).await?;
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
                        FlowError::VesselFailure(format!("Failed to create tmux window: {e}"))
                    })?;

                self.start_process(&cmd).await?;
            }
        }

        self.initialized = true;
        Ok(())
    }

    /// Start a process inside this vessel's window.
    async fn start_process(&self, cmd: &str) -> Result<(), FlowError> {
        Command::new("tmux")
            .args(["send-keys", "-t", &self.target(), cmd, "Enter"])
            .output()
            .await
            .map_err(|e| {
                FlowError::VesselFailure(format!("Failed to start process in tmux: {e}"))
            })?;

        // Give the process a moment to start
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        Ok(())
    }

    /// Capture the full pane content including scrollback history.
    ///
    /// Uses `-S -` and `-E -` for complete scrollback (pattern adopted
    /// from tmux-lib, without the dependency). `-J` joins wrapped lines.
    async fn capture_pane(&self) -> Result<String, FlowError> {
        let output = Command::new("tmux")
            .args([
                "capture-pane",
                "-t",
                &self.target(),
                "-p", // output to stdout
                "-J", // join wrapped lines
                "-S",
                "-", // from start of scrollback
                "-E",
                "-", // to end of scrollback
            ])
            .output()
            .await
            .map_err(|e| {
                FlowError::VesselFailure(format!("[{}] Failed to capture pane: {e}", self.window))
            })?;

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Send a message through the vessel and capture the response.
    ///
    /// Two modes of waiting:
    /// - **Sentinel**: if set, waits for the sentinel pattern to appear
    ///   after the input line (the process signals readiness).
    /// - **Stability**: if no sentinel, waits for pane content to stop
    ///   changing (the mud settles on its own).
    pub async fn send(&self, input: &str) -> Result<String, FlowError> {
        if input.is_empty() {
            return Ok(String::new());
        }

        if self.input_delimiter.is_some() {
            // Multiline mode: write content to a temp file and send the path.
            // paste-buffer hits pty buffer limits (~4096 bytes on macOS),
            // truncating large inputs. File-based transport has no such limit.
            self.send_via_file(input).await?;
        } else {
            let output = Command::new("tmux")
                .args(["send-keys", "-l", "-t", &self.target(), input])
                .output()
                .await
                .map_err(|e| {
                    FlowError::VesselFailure(format!(
                        "[{}] Failed to send to tmux: {e}",
                        self.window
                    ))
                })?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(FlowError::VesselFailure(format!(
                    "[{}] tmux send-keys failed: {}",
                    self.window,
                    stderr.trim()
                )));
            }
        }

        // Send Enter key separately (not literal, so "Enter" is the key)
        Command::new("tmux")
            .args(["send-keys", "-t", &self.target(), "Enter"])
            .output()
            .await
            .ok();

        // Send input delimiter if set (marks end of multi-line input)
        if let Some(ref delimiter) = self.input_delimiter {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
            let output = Command::new("tmux")
                .args(["send-keys", "-t", &self.target(), delimiter, "Enter"])
                .output()
                .await
                .map_err(|e| {
                    FlowError::VesselFailure(format!(
                        "[{}] Failed to send delimiter to tmux: {e}",
                        self.window
                    ))
                })?;
            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(FlowError::VesselFailure(format!(
                    "[{}] tmux send-keys (delimiter) failed: {}",
                    self.window,
                    stderr.trim()
                )));
            }
        }

        // The boundary separates input from output in the pane.
        // With a delimiter, it's the delimiter (handles multi-line input).
        // Without, it's the input itself (single-line).
        let boundary = self.input_delimiter.as_deref().unwrap_or(input);
        let use_last_occurrence = self.input_delimiter.is_some();

        // Wait for response
        let mut last_content = String::new();
        let mut stable_count = 0;
        let max_wait = 60; // seconds
        let poll_interval = tokio::time::Duration::from_millis(500);

        for _ in 0..(max_wait * 2) {
            tokio::time::sleep(poll_interval).await;

            let content = self.capture_pane().await?;

            if let Some(ref sentinel) = self.sentinel {
                // Sentinel mode: look for readiness signal after input.
                // With delimiter: last occurrence (handles repeated calls).
                // Without: first occurrence (response may echo input).
                let all_lines: Vec<&str> = content.lines().collect();
                let start = if use_last_occurrence {
                    all_lines.iter().rposition(|line| line.contains(boundary))
                } else {
                    all_lines.iter().position(|line| line.contains(boundary))
                }
                .map(|i| i + 1)
                .unwrap_or(0);

                let sentinel_found = all_lines[start..]
                    .iter()
                    .rev()
                    .find(|l| !l.trim().is_empty())
                    .map(|line| line.contains(sentinel.as_str()))
                    .unwrap_or(false);

                last_content = content;
                if sentinel_found {
                    break;
                }
            } else {
                // Stability mode: content unchanged for 2 seconds
                if content == last_content {
                    stable_count += 1;
                    if stable_count >= 4 {
                        break;
                    }
                } else {
                    stable_count = 0;
                    last_content = content;
                }
            }
        }

        // Extract response: everything after the boundary
        let all_lines: Vec<&str> = last_content.lines().collect();
        let start = if use_last_occurrence {
            all_lines.iter().rposition(|line| line.contains(boundary))
        } else {
            all_lines.iter().position(|line| line.contains(boundary))
        }
        .map(|i| i + 1)
        .unwrap_or(0);
        let mut lines: Vec<&str> = all_lines[start..].to_vec();

        // Strip sentinel line from the end if present
        if let Some(ref sentinel) = self.sentinel {
            while let Some(last) = lines.last() {
                if last.contains(sentinel.as_str()) || last.trim().is_empty() {
                    lines.pop();
                } else {
                    break;
                }
            }
        }

        Ok(lines.join("\n").trim().to_string())
    }

    /// Write input to a temp file and send the file path via send-keys.
    /// The multiline wrapper script reads from the file instead of stdin,
    /// avoiding pty buffer limits that truncate paste-buffer content.
    async fn send_via_file(&self, input: &str) -> Result<(), FlowError> {
        let file_path = format!("/tmp/taoflow-{}-{}-input.txt", self.session, self.window);
        std::fs::write(&file_path, input).map_err(|e| {
            FlowError::VesselFailure(format!("[{}] Failed to write input file: {e}", self.window))
        })?;

        let output = Command::new("tmux")
            .args(["send-keys", "-l", "-t", &self.target(), &file_path])
            .output()
            .await
            .map_err(|e| {
                FlowError::VesselFailure(format!(
                    "[{}] Failed to send file path to tmux: {e}",
                    self.window
                ))
            })?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FlowError::VesselFailure(format!(
                "[{}] tmux send-keys failed: {}",
                self.window,
                stderr.trim()
            )));
        }
        Ok(())
    }

    /// Kill the tmux session this vessel belongs to.
    pub async fn teardown(&self) -> Result<(), FlowError> {
        Command::new("tmux")
            .args(["kill-session", "-t", &self.session])
            .status()
            .await
            .map_err(|e| FlowError::VesselFailure(format!("Failed to kill tmux session: {e}")))?;
        Ok(())
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
