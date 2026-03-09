use async_trait::async_trait;
use tokio::process::Command;

use super::{ChatMessage, LlmSource};
use crate::error::FlowError;
use crate::water::Role;

/// Uses `claude -p` (print mode). No API keys needed.
pub struct ClaudeCliSource {
    model: String,
}

impl ClaudeCliSource {
    pub fn new(model: impl Into<String>) -> Self {
        Self {
            model: model.into(),
        }
    }

    /// Format conversation history into the prompt.
    /// In stateless `-p` mode, vapor must be carried explicitly.
    fn format_conversation(system: &str, messages: &[ChatMessage]) -> String {
        let mut parts = Vec::new();

        if !system.is_empty() {
            parts.push(format!("[System context: {system}]"));
        }

        // Include prior conversation as context
        if messages.len() > 1 {
            parts.push("Prior conversation:".to_string());
            for msg in &messages[..messages.len() - 1] {
                let role = match msg.role {
                    Role::User => "User",
                    Role::Assistant => "Assistant",
                };
                parts.push(format!("{role}: {}", msg.content));
            }
            parts.push(String::new()); // blank line before current
        }

        // The current message (last user message) is the actual prompt
        if let Some(last) = messages.last() {
            parts.push(last.content.clone());
        }

        parts.join("\n")
    }
}

#[async_trait]
impl LlmSource for ClaudeCliSource {
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError> {
        let prompt = Self::format_conversation(system, messages);

        let mut cmd = Command::new("claude");
        cmd.arg("-p")
            .arg("--model")
            .arg(&self.model)
            .arg("--system-prompt")
            .arg(system)
            .env_remove("CLAUDECODE");

        // Pass the prompt via stdin
        cmd.stdin(std::process::Stdio::piped());
        cmd.stdout(std::process::Stdio::piped());
        cmd.stderr(std::process::Stdio::piped());

        let mut child = cmd.spawn().map_err(|e| FlowError::SpringFailure {
            name: format!("claude-cli({})", self.model),
            reason: if e.kind() == std::io::ErrorKind::NotFound {
                "claude CLI not found. Install Claude Code: https://docs.anthropic.com/en/docs/claude-code".to_string()
            } else {
                format!("Failed to spawn claude process: {e}")
            },
        })?;

        // Write prompt to stdin
        if let Some(mut stdin) = child.stdin.take() {
            use tokio::io::AsyncWriteExt;
            stdin
                .write_all(prompt.as_bytes())
                .await
                .map_err(|e| FlowError::SpringFailure {
                    name: format!("claude-cli({})", self.model),
                    reason: format!("Failed to write to stdin: {e}"),
                })?;
            // Drop stdin to close it, signaling EOF
        }

        let output = child
            .wait_with_output()
            .await
            .map_err(|e| FlowError::SpringFailure {
                name: format!("claude-cli({})", self.model),
                reason: format!("Failed to read output: {e}"),
            })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FlowError::SpringFailure {
                name: format!("claude-cli({})", self.model),
                reason: format!("claude exited with {}: {}", output.status, stderr.trim()),
            });
        }

        let text = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(text)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_simple_prompt() {
        let messages = vec![ChatMessage {
            role: Role::User,
            content: "What is the Tao?".into(),
        }];
        let formatted = ClaudeCliSource::format_conversation("You are wise.", &messages);
        assert!(formatted.contains("What is the Tao?"));
    }

    #[test]
    fn format_conversation_with_history() {
        let messages = vec![
            ChatMessage {
                role: Role::User,
                content: "My name is River.".into(),
            },
            ChatMessage {
                role: Role::Assistant,
                content: "Hello, River.".into(),
            },
            ChatMessage {
                role: Role::User,
                content: "What is my name?".into(),
            },
        ];
        let formatted = ClaudeCliSource::format_conversation("system", &messages);
        assert!(formatted.contains("Prior conversation:"));
        assert!(formatted.contains("User: My name is River."));
        assert!(formatted.contains("Assistant: Hello, River."));
        assert!(formatted.contains("What is my name?"));
    }

    #[test]
    fn format_empty_messages() {
        let formatted = ClaudeCliSource::format_conversation("system", &[]);
        assert!(formatted.contains("System context"));
    }
}
