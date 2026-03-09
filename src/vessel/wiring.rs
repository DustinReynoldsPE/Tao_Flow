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

pub const DEFAULT_MOUNTAIN_MODEL: &str = "claude-sonnet-4-6";
pub const DEFAULT_DESERT_MODEL: &str = "claude-haiku-4-5-20251001";
pub const DEFAULT_FOREST_MODEL: &str = "claude-sonnet-4-6";
pub const DEFAULT_UTILITY_MODEL: &str = "claude-haiku-4-5-20251001";

/// Configuration for a vessel-backed TaoFlow session.
pub struct VesselConfig {
    pub session: String,
    pub mountain_model: String,
    pub desert_model: String,
    pub forest_model: String,
    pub utility_model: String,
}

impl VesselConfig {
    pub fn new(session: impl Into<String>) -> Self {
        Self {
            session: session.into(),
            mountain_model: DEFAULT_MOUNTAIN_MODEL.into(),
            desert_model: DEFAULT_DESERT_MODEL.into(),
            forest_model: DEFAULT_FOREST_MODEL.into(),
            utility_model: DEFAULT_UTILITY_MODEL.into(),
        }
    }
}

/// Wraps TmuxPaneSource so the system prompt travels with each message.
///
/// Confluence and Still Lake change system prompts between calls
/// (detect, yield, weave), so the prompt cannot be baked into the
/// wrapper script. Instead, it is prepended to the user message.
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

        let embedded = if system.is_empty() {
            last_content.to_string()
        } else {
            format!("[System: {system}]\n\n{last_content}")
        };

        let new_messages = vec![ChatMessage {
            role: Role::User,
            content: embedded,
        }];

        self.inner.complete("", &new_messages).await
    }
}

/// Write a wrapper script that pipes single-line input through `claude -p`.
fn write_wrapper_script(session: &str, name: &str, model: &str, system_prompt: &str) -> String {
    let script_path = format!("/tmp/taoflow-{session}-{name}.sh");
    let escaped_prompt = system_prompt.replace('\'', "'\\''");
    let script = format!(
        "#!/bin/bash\nwhile IFS= read -r line; do\n  \
         echo \"$line\" | env -u CLAUDECODE claude -p --model {model} \
         --system-prompt $'{escaped_prompt}'\n  \
         echo \"{SENTINEL}\"\ndone\n"
    );
    std::fs::write(&script_path, &script).expect("failed to write wrapper script");
    script_path
}

/// Write a wrapper script that reads multi-line input terminated by a delimiter.
fn write_multiline_wrapper_script(session: &str, name: &str, model: &str) -> String {
    let script_path = format!("/tmp/taoflow-{session}-{name}.sh");
    let script = format!(
        "#!/bin/bash\nwhile true; do\n  input=\"\"\n  while IFS= read -r line; do\n    \
         [ \"$line\" = \"{INPUT_DELIMITER}\" ] && break\n    \
         if [ -z \"$input\" ]; then\n      input=\"$line\"\n    else\n      \
         input=\"$input\"$'\\n'\"$line\"\n    fi\n  done\n  \
         [ -z \"$input\" ] && continue\n  \
         echo \"$input\" | env -u CLAUDECODE claude -p --model {model}\n  \
         echo \"{SENTINEL}\"\ndone\n"
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
    let script = write_wrapper_script(&config.session, name, model, system_prompt);
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
    let script =
        write_multiline_wrapper_script(&config.session, "confluence", &config.utility_model);
    let vessel = TmuxVessel::new(&config.session, "confluence", &config.utility_model)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL)
        .with_input_delimiter(INPUT_DELIMITER);
    Box::new(SystemPromptSource {
        inner: TmuxPaneSource::new(vessel),
    })
}

fn vessel_lake_source(config: &VesselConfig) -> Box<dyn LlmSource> {
    let script =
        write_multiline_wrapper_script(&config.session, "still-lake", &config.utility_model);
    let vessel = TmuxVessel::new(&config.session, "still-lake", &config.utility_model)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL)
        .with_input_delimiter(INPUT_DELIMITER);
    Box::new(SystemPromptSource {
        inner: TmuxPaneSource::new(vessel),
    })
}

fn vessel_decomposer_source(config: &VesselConfig) -> Box<dyn LlmSource> {
    let script =
        write_multiline_wrapper_script(&config.session, "decomposer", &config.utility_model);
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
