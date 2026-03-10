use std::collections::HashMap;

use async_trait::async_trait;

use crate::confluence::{ConfluencePool, Decomposer};
use crate::error::FlowError;
use crate::flow::TaoFlow;
use crate::still_lake::StillLake;
use crate::vessel::TmuxVessel;
use crate::water::Role;
use crate::watershed::source::{ChatMessage, LlmSource};
use crate::watershed::spring::SpringConfig;
use crate::watershed::springs::{desert, forest, mountain};
use crate::watershed::{
    DesertSpring, ForestSpring, MountainSpring, Spring, TmuxPaneSource, Watershed,
};

pub const SENTINEL: &str = "TAOFLOW_READY";
pub const INPUT_DELIMITER: &str = "TAOFLOW_INPUT_END";
pub const SYSTEM_DELIMITER: &str = "TAOFLOW_SYSTEM_END";

/// The CLI tool backing each spring's LLM invocations.
pub enum CliBackend {
    /// Claude Code CLI: `claude -p --model {model} --system-prompt ...`
    Claude,
    /// Crush (opencode): `crush run --model {provider/model} --quiet ...`
    Crush,
}

impl CliBackend {
    /// Default model IDs: (mountain, desert, forest, utility).
    fn default_models(&self) -> (&str, &str, &str, &str) {
        match self {
            Self::Claude => (
                "claude-sonnet-4-6",
                "claude-haiku-4-5-20251001",
                "claude-sonnet-4-6",
                "claude-haiku-4-5-20251001",
            ),
            Self::Crush => (
                "anthropic/claude-sonnet-4-6",
                "anthropic/claude-haiku-4-5-20251001",
                "anthropic/claude-sonnet-4-6",
                "anthropic/claude-haiku-4-5-20251001",
            ),
        }
    }

    /// Discover available tools and MCP servers for this backend.
    fn discover_tools(&self) -> ToolConfig {
        match self {
            Self::Claude => discover_claude_context_mode().unwrap_or_else(|| ToolConfig {
                allowed_tools: vec!["WebSearch".into(), "WebFetch".into()],
                mcp_config: None,
            }),
            // Crush manages tools and MCP servers via crush.json.
            Self::Crush => ToolConfig::default(),
        }
    }

    /// Convert a ToolConfig into CLI flags for this backend.
    fn tool_cli_flags(&self, config: &ToolConfig) -> String {
        match self {
            Self::Claude => {
                let mut flags = String::new();
                if !config.allowed_tools.is_empty() {
                    let tools = config.allowed_tools.join(",");
                    flags.push_str(&format!(" --allowedTools {tools}"));
                }
                if let Some(ref path) = config.mcp_config {
                    let escaped = path.replace('"', "\\\"");
                    flags.push_str(&format!(" --mcp-config \"{escaped}\""));
                }
                flags
            }
            // Crush manages tools via crush.json, not CLI flags.
            Self::Crush => String::new(),
        }
    }

    /// CLI invocation that reads from stdin with a baked-in system prompt.
    /// The `escaped_system` should already be escaped for $'...' quoting.
    fn piped_with_baked_system(
        &self,
        model: &str,
        escaped_system: &str,
        tool_flags: &str,
    ) -> String {
        match self {
            Self::Claude => format!(
                "env -u CLAUDECODE claude -p --model {model} \
                 --setting-sources ''{tool_flags} \
                 --system-prompt $'{escaped_system}'"
            ),
            Self::Crush => format!(
                "crush run --model {model} --quiet{tool_flags} \
                 --system-prompt $'{escaped_system}'"
            ),
        }
    }

    /// CLI invocation that reads from stdin with a dynamic system prompt
    /// from the shell variable `$system`.
    fn piped_with_system_var(&self, model: &str, tool_flags: &str) -> String {
        match self {
            Self::Claude => format!(
                "env -u CLAUDECODE claude -p --model {model} \
                 --setting-sources ''{tool_flags} --system-prompt \"$system\""
            ),
            Self::Crush => format!(
                "crush run --model {model} --quiet{tool_flags} \
                 --system-prompt \"$system\""
            ),
        }
    }

    /// CLI invocation that reads from stdin with no system prompt.
    fn piped_bare(&self, model: &str, tool_flags: &str) -> String {
        match self {
            Self::Claude => format!(
                "env -u CLAUDECODE claude -p --model {model} \
                 --setting-sources ''{tool_flags}"
            ),
            Self::Crush => format!("crush run --model {model} --quiet{tool_flags}"),
        }
    }
}

/// Tool configuration for a spring's CLI invocations.
///
/// Controls which tools and MCP servers are available.
/// Interpretation depends on the CLI backend:
/// - Claude: `allowed_tools` → `--allowedTools`, `mcp_config` → `--mcp-config`
/// - Crush: tools and MCP are configured in `crush.json`
#[derive(Clone, Default)]
pub struct ToolConfig {
    pub allowed_tools: Vec<String>,
    pub mcp_config: Option<String>,
}

/// Discover context-mode MCP plugin for Claude and write a config file.
///
/// Searches the Claude plugin cache for context-mode, finds the latest
/// version, and writes an MCP config to /tmp/ that claude -p can use.
fn discover_claude_context_mode() -> Option<ToolConfig> {
    let home = std::env::var("HOME").ok()?;
    let plugin_base = std::path::PathBuf::from(&home)
        .join(".claude/plugins/cache/claude-context-mode/context-mode");
    let latest = std::fs::read_dir(&plugin_base)
        .ok()?
        .filter_map(|e| e.ok())
        .max_by_key(|e| e.file_name())?;
    let start_mjs = latest.path().join("start.mjs");
    if !start_mjs.exists() {
        return None;
    }

    let mcp_path = "/tmp/taoflow-mcp-config.json";
    let mcp_json = format!(
        r#"{{"mcpServers":{{"context-mode":{{"command":"node","args":["{}"]}}}}}}"#,
        start_mjs.display()
    );
    std::fs::write(mcp_path, mcp_json).ok()?;

    Some(ToolConfig {
        allowed_tools: vec!["WebSearch".into(), "WebFetch".into()],
        mcp_config: Some(mcp_path.into()),
    })
}

/// Configuration for a vessel-backed TaoFlow session.
pub struct VesselConfig {
    pub session: String,
    pub backend: CliBackend,
    pub mountain_model: String,
    pub desert_model: String,
    pub forest_model: String,
    pub utility_model: String,
    /// Tools available to all springs by default.
    pub default_tools: Option<ToolConfig>,
    /// Per-spring tool overrides, keyed by spring name
    /// ("mountain", "desert", "forest", "confluence", "still-lake", "decomposer").
    pub spring_tools: HashMap<String, ToolConfig>,
}

impl VesselConfig {
    /// Create a config with the Claude backend.
    pub fn new(session: impl Into<String>) -> Self {
        Self::for_backend(session, CliBackend::Claude)
    }

    /// Create a config with a specific CLI backend.
    pub fn for_backend(session: impl Into<String>, backend: CliBackend) -> Self {
        let (mountain, desert, forest, utility) = backend.default_models();
        let tools = backend.discover_tools();
        Self {
            session: session.into(),
            mountain_model: mountain.into(),
            desert_model: desert.into(),
            forest_model: forest.into(),
            utility_model: utility.into(),
            default_tools: Some(tools),
            spring_tools: HashMap::new(),
            backend,
        }
    }

    pub fn with_default_tools(mut self, tools: ToolConfig) -> Self {
        self.default_tools = Some(tools);
        self
    }

    pub fn with_spring_tools(mut self, name: impl Into<String>, tools: ToolConfig) -> Self {
        self.spring_tools.insert(name.into(), tools);
        self
    }

    fn tools_for(&self, name: &str) -> Option<&ToolConfig> {
        self.spring_tools.get(name).or(self.default_tools.as_ref())
    }

    fn tool_flags_for(&self, name: &str) -> String {
        self.tools_for(name)
            .map(|t| self.backend.tool_cli_flags(t))
            .unwrap_or_default()
    }
}

/// Wraps TmuxPaneSource so the system prompt travels with each message.
///
/// Confluence and Still Lake change system prompts between calls
/// (detect, yield, weave), so the prompt cannot be baked into the
/// wrapper script. Instead, the system prompt is sent first, followed
/// by TAOFLOW_SYSTEM_END, then the user content. The wrapper script
/// extracts both and passes `--system-prompt` properly.
struct SystemPromptSource {
    inner: TmuxPaneSource,
}

#[async_trait]
impl LlmSource for SystemPromptSource {
    async fn complete(&self, system: &str, messages: &[ChatMessage]) -> Result<String, FlowError> {
        let last_content = messages
            .iter()
            .rfind(|m| m.role == Role::User)
            .map(|m| m.content.as_str())
            .unwrap_or("");

        let content = if system.is_empty() {
            last_content.to_string()
        } else {
            format!("{system}\n{SYSTEM_DELIMITER}\n{last_content}")
        };

        let new_messages = vec![ChatMessage {
            role: Role::User,
            content,
        }];

        self.inner.complete("", &new_messages).await
    }
}

/// Write a wrapper script that pipes single-line input through the CLI backend.
fn write_wrapper_script(
    session: &str,
    name: &str,
    model: &str,
    system_prompt: &str,
    tool_flags: &str,
    backend: &CliBackend,
) -> String {
    let script_path = format!("/tmp/taoflow-{session}-{name}.sh");
    let escaped_prompt = system_prompt.replace('\'', "'\\''");
    let invocation = backend.piped_with_baked_system(model, &escaped_prompt, tool_flags);
    let script = format!(
        "#!/bin/bash\ncd /tmp\nwhile IFS= read -r line; do\n  \
         echo \"$line\" | {invocation}\n  \
         echo \"{SENTINEL}\"\ndone\n"
    );
    std::fs::write(&script_path, &script).expect("failed to write wrapper script");
    script_path
}

/// Write a wrapper script that reads input from temp files.
///
/// The script reads file paths from stdin (one per line). Each file
/// contains an optional system prompt followed by user content:
///   system prompt lines...
///   TAOFLOW_SYSTEM_END
///   user content lines...
///
/// If no TAOFLOW_SYSTEM_END appears, all content is treated as user input.
/// The file is deleted after processing.
///
/// This avoids the pty buffer limit (~4096 bytes) that truncates large
/// inputs sent via paste-buffer. Only the short file path travels
/// through the pty; the content travels through the filesystem.
fn write_multiline_wrapper_script(
    session: &str,
    name: &str,
    model: &str,
    tool_flags: &str,
    backend: &CliBackend,
) -> String {
    let script_path = format!("/tmp/taoflow-{session}-{name}.sh");
    let invoke_with_system = backend.piped_with_system_var(model, tool_flags);
    let invoke_bare = backend.piped_bare(model, tool_flags);
    let script = format!(
        r#"#!/bin/bash
cd /tmp
while IFS= read -r filepath; do
  [ "$filepath" = "{INPUT_DELIMITER}" ] && continue
  [ -z "$filepath" ] && continue
  [ ! -f "$filepath" ] && continue
  system=""
  input=""
  in_system=true
  found_system=false
  while IFS= read -r line; do
    if $in_system && [ "$line" = "{SYSTEM_DELIMITER}" ]; then
      in_system=false
      found_system=true
      continue
    fi
    if $in_system; then
      if [ -z "$system" ]; then system="$line"; else system="$system"$'\n'"$line"; fi
    else
      if [ -z "$input" ]; then input="$line"; else input="$input"$'\n'"$line"; fi
    fi
  done < "$filepath"
  if ! $found_system; then
    input="$system"
    system=""
  fi
  [ -z "$input" ] && continue
  if [ -n "$system" ]; then
    echo "$input" | {invoke_with_system}
  else
    echo "$input" | {invoke_bare}
  fi
  rm -f "$filepath"
  echo "{SENTINEL}"
done
"#
    );
    std::fs::write(&script_path, &script).expect("failed to write wrapper script");
    script_path
}

fn vessel_source(
    config: &VesselConfig,
    name: &str,
    model: &str,
    system_prompt: &str,
) -> Box<dyn LlmSource> {
    let tool_flags = config.tool_flags_for(name);
    let script = write_wrapper_script(
        &config.session,
        name,
        model,
        system_prompt,
        &tool_flags,
        &config.backend,
    );
    let vessel = TmuxVessel::new(&config.session, name, model)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL);
    Box::new(TmuxPaneSource::new(vessel))
}

fn vessel_mountain(config: &VesselConfig) -> Box<dyn Spring> {
    let mut affinities = HashMap::new();
    affinities.insert("philosophy".into(), 0.9);
    affinities.insert("architecture".into(), 0.8);
    affinities.insert("analysis".into(), 0.7);
    Box::new(MountainSpring::new(
        SpringConfig {
            name: "mountain".into(),
            nature: "deep reasoning, analysis, architecture".into(),
            affinities,
        },
        vessel_source(
            config,
            "mountain",
            &config.mountain_model,
            mountain::SYSTEM_PROMPT,
        ),
    ))
}

fn vessel_desert(config: &VesselConfig) -> Box<dyn Spring> {
    let mut affinities = HashMap::new();
    affinities.insert("quick_answers".into(), 0.9);
    affinities.insert("formatting".into(), 0.7);
    affinities.insert("code".into(), 0.6);
    Box::new(DesertSpring::new(
        SpringConfig {
            name: "desert".into(),
            nature: "speed, directness, efficiency".into(),
            affinities,
        },
        vessel_source(
            config,
            "desert",
            &config.desert_model,
            desert::SYSTEM_PROMPT,
        ),
    ))
}

fn vessel_forest(config: &VesselConfig) -> Box<dyn Spring> {
    let mut affinities = HashMap::new();
    affinities.insert("narrative".into(), 0.9);
    affinities.insert("empathy".into(), 0.8);
    affinities.insert("poetry".into(), 0.7);
    Box::new(ForestSpring::new(
        SpringConfig {
            name: "forest".into(),
            nature: "creativity, narrative, empathy".into(),
            affinities,
        },
        vessel_source(
            config,
            "forest",
            &config.forest_model,
            forest::SYSTEM_PROMPT,
        ),
    ))
}

fn vessel_confluence_source(config: &VesselConfig) -> Box<dyn LlmSource> {
    let tool_flags = config.tool_flags_for("confluence");
    let script = write_multiline_wrapper_script(
        &config.session,
        "confluence",
        &config.utility_model,
        &tool_flags,
        &config.backend,
    );
    let vessel = TmuxVessel::new(&config.session, "confluence", &config.utility_model)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL)
        .with_input_delimiter(INPUT_DELIMITER);
    Box::new(SystemPromptSource {
        inner: TmuxPaneSource::new(vessel),
    })
}

fn vessel_lake_source(config: &VesselConfig) -> Box<dyn LlmSource> {
    let tool_flags = config.tool_flags_for("still-lake");
    let script = write_multiline_wrapper_script(
        &config.session,
        "still-lake",
        &config.utility_model,
        &tool_flags,
        &config.backend,
    );
    let vessel = TmuxVessel::new(&config.session, "still-lake", &config.utility_model)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL)
        .with_input_delimiter(INPUT_DELIMITER);
    Box::new(SystemPromptSource {
        inner: TmuxPaneSource::new(vessel),
    })
}

fn vessel_decomposer_source(config: &VesselConfig) -> Box<dyn LlmSource> {
    let tool_flags = config.tool_flags_for("decomposer");
    let script = write_multiline_wrapper_script(
        &config.session,
        "decomposer",
        &config.utility_model,
        &tool_flags,
        &config.backend,
    );
    let vessel = TmuxVessel::new(&config.session, "decomposer", &config.utility_model)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL)
        .with_input_delimiter(INPUT_DELIMITER);
    Box::new(SystemPromptSource {
        inner: TmuxPaneSource::new(vessel),
    })
}

/// Pre-create the tmux session so concurrent vessel.prepare() calls
/// don't race on session creation.
pub async fn create_session(session: &str) {
    cleanup_session(session).await;
    tokio::process::Command::new("tmux")
        .args(["new-session", "-d", "-s", session])
        .status()
        .await
        .ok();
}

/// Kill the tmux session and clean up wrapper scripts.
pub async fn cleanup_session(session: &str) {
    tokio::process::Command::new("tmux")
        .args(["kill-session", "-t", session])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .await
        .ok();
}

/// Assemble a vessel-backed TaoFlow with three springs,
/// confluence, and still lake — each visible in its own tmux pane.
///
/// `tmux attach -t {session}` to watch the water flow.
pub async fn build_tao_flow(config: &VesselConfig) -> TaoFlow {
    create_session(&config.session).await;
    TaoFlow::new(
        Watershed::new(vec![
            vessel_mountain(config),
            vessel_desert(config),
            vessel_forest(config),
        ]),
        ConfluencePool::new(vessel_confluence_source(config)),
        StillLake::new(vessel_lake_source(config)),
    )
    .with_decomposer(Decomposer::new(vessel_decomposer_source(config)))
}
