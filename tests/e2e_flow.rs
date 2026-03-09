//! End-to-end flow tests -- Level 3.
//!
//! Real rain through real springs, each visible in its own tmux window.
//! `tmux attach -t tao-e2e` to watch the water flow.
//!
//! Requirements:
//! - tmux
//! - claude CLI with valid authentication
//!
//! Run all:  cargo test --test e2e_flow -- --ignored
//! Run one:  cargo test --test e2e_flow -- --ignored droplet_flows_through_desert

use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use tao_flow::confluence::ConfluencePool;
use tao_flow::error::FlowError;
use tao_flow::flow::TaoFlow;
use tao_flow::still_lake::StillLake;
use tao_flow::vessel::TmuxVessel;
use tao_flow::water::Role;
use tao_flow::watershed::source::{ChatMessage, LlmSource};
use tao_flow::watershed::spring::SpringConfig;
use tao_flow::watershed::springs::{desert, forest, mountain};
use tao_flow::watershed::{
    DesertSpring, ForestSpring, MountainSpring, Spring, TmuxPaneSource, Watershed,
};

// --- Models ---
// Adjust these to match your subscription and speed preference.

const MOUNTAIN_MODEL: &str = "claude-sonnet-4-6";
const DESERT_MODEL: &str = "claude-haiku-4-5-20251001";
const FOREST_MODEL: &str = "claude-sonnet-4-6";
const UTILITY_MODEL: &str = "claude-haiku-4-5-20251001";

const SENTINEL: &str = "TAOFLOW_READY";

// --- Session management ---

async fn tmux_available() -> bool {
    tokio::process::Command::new("tmux")
        .arg("-V")
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

async fn claude_available() -> bool {
    tokio::process::Command::new("claude")
        .arg("--version")
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

async fn cleanup(session: &str) {
    tokio::process::Command::new("tmux")
        .args(["kill-session", "-t", session])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .await
        .ok();
}

/// Pre-create the session so concurrent vessel.prepare() calls
/// don't race on session creation.
async fn create_session(session: &str) {
    cleanup(session).await;
    tokio::process::Command::new("tmux")
        .args(["new-session", "-d", "-s", session])
        .status()
        .await
        .ok();
}

// --- Flow journal ---
// Captures every vessel's pane content into a structured markdown document.
// The journey is preserved, not just the destination.

const JOURNAL_ORDER: &[&str] = &["mountain", "desert", "forest", "confluence", "still-lake"];

fn window_title(name: &str) -> &str {
    match name {
        "mountain" => "Mountain Spring",
        "desert" => "Desert Spring",
        "forest" => "Forest Spring",
        "confluence" => "Confluence",
        "still-lake" => "Still Lake",
        other => other,
    }
}

async fn list_session_windows(session: &str) -> Vec<String> {
    tokio::process::Command::new("tmux")
        .args(["list-windows", "-t", session, "-F", "#{window_name}"])
        .output()
        .await
        .ok()
        .filter(|o| o.status.success())
        .map(|o| {
            String::from_utf8_lossy(&o.stdout)
                .lines()
                .map(String::from)
                .collect()
        })
        .unwrap_or_default()
}

async fn capture_window_content(session: &str, window: &str) -> String {
    let target = format!("{session}:{window}");
    tokio::process::Command::new("tmux")
        .args([
            "capture-pane",
            "-t",
            &target,
            "-p",
            "-J",
            "-S",
            "-",
            "-E",
            "-",
        ])
        .output()
        .await
        .ok()
        .filter(|o| o.status.success())
        .map(|o| String::from_utf8_lossy(&o.stdout).to_string())
        .unwrap_or_default()
}

// --- Pane parsing: separate what the program sent from what the agent returned ---

struct Exchange {
    prompt: String,
    response: String,
}

/// Strip bash machinery lines from pane content.
fn strip_bash_lines(content: &str) -> Vec<&str> {
    content
        .lines()
        .filter(|line| {
            !line.contains("% bash /tmp/tao-e2e") && !line.starts_with("bash /tmp/tao-e2e")
        })
        .collect()
}

/// Parse a spring pane (no delimiter) into exchanges.
/// First line is the prompt (echoed input), rest until sentinel is the response.
fn parse_spring_exchanges(content: &str) -> Vec<Exchange> {
    let lines = strip_bash_lines(content);
    let mut exchanges = Vec::new();
    let mut prompt = String::new();
    let mut response_lines: Vec<&str> = Vec::new();
    let mut seen_prompt = false;

    for line in &lines {
        if line.trim() == SENTINEL {
            if seen_prompt {
                exchanges.push(Exchange {
                    prompt: prompt.clone(),
                    response: response_lines.join("\n").trim().to_string(),
                });
                prompt.clear();
                response_lines.clear();
                seen_prompt = false;
            }
            continue;
        }
        if !seen_prompt {
            prompt = line.to_string();
            seen_prompt = true;
        } else {
            response_lines.push(line);
        }
    }

    if seen_prompt {
        let response = response_lines.join("\n").trim().to_string();
        if !prompt.trim().is_empty() || !response.is_empty() {
            exchanges.push(Exchange { prompt, response });
        }
    }

    exchanges
}

/// Parse a delimited pane (confluence/still-lake) into exchanges.
/// Content before delimiter = prompt (program input),
/// between delimiter and sentinel = response (agent output).
fn parse_delimited_exchanges(content: &str) -> Vec<Exchange> {
    let lines = strip_bash_lines(content);
    let mut exchanges = Vec::new();
    let mut prompt_lines: Vec<&str> = Vec::new();
    let mut response_lines: Vec<&str> = Vec::new();
    let mut in_response = false;

    for line in &lines {
        if line.trim() == INPUT_DELIMITER {
            in_response = true;
            continue;
        }
        if line.trim() == SENTINEL {
            exchanges.push(Exchange {
                prompt: prompt_lines.join("\n").trim().to_string(),
                response: response_lines.join("\n").trim().to_string(),
            });
            prompt_lines.clear();
            response_lines.clear();
            in_response = false;
            continue;
        }
        if in_response {
            response_lines.push(line);
        } else {
            prompt_lines.push(line);
        }
    }

    exchanges
}

/// Detect the confluence phase from the system prompt embedded in the exchange.
/// Order matters: more specific patterns first (merging contains "yielding" in
/// resolution text, so merging must be checked before yielding).
fn detect_phase(prompt: &str) -> &str {
    if prompt.contains("merge multiple perspectives") || prompt.contains("weave") {
        "Merging"
    } else if prompt.contains("find where they diverge")
        || prompt.contains("analyze multiple responses")
    {
        "Eddy Detection"
    } else if prompt.contains("resolve disagreements") || prompt.contains("yielding") {
        "Yielding"
    } else if prompt.contains("settle the mud") || prompt.contains("Still Lake") {
        "Settling"
    } else if prompt.contains("decompose") || prompt.contains("sub-questions") {
        "Decomposition"
    } else {
        "Processing"
    }
}

/// Strip trailing horizontal rules from agent output so they don't
/// collide with the journal's own section separators.
fn strip_trailing_rule(text: &str) -> &str {
    let mut result = text.trim_end();
    while let Some(stripped) = result.strip_suffix("---") {
        result = stripped.trim_end();
    }
    result
}

/// Format a spring section: blockquoted prompt, then the agent's response.
fn format_spring_section(title: &str, content: &str) -> String {
    let exchanges = parse_spring_exchanges(content);
    if exchanges.is_empty() {
        return String::new();
    }

    let mut section = format!("## {title}\n\n");
    for (i, exchange) in exchanges.iter().enumerate() {
        if !exchange.prompt.is_empty() {
            section.push_str(&format!("> **Prompt:** {}\n\n", exchange.prompt));
        }
        if !exchange.response.is_empty() {
            section.push_str(&format!("{}\n\n", strip_trailing_rule(&exchange.response)));
        }
        if i < exchanges.len() - 1 {
            section.push_str("---\n\n");
        }
    }
    section
}

/// Format a confluence/still-lake section with labeled phases.
/// Prompts in blockquotes, responses in plain text.
fn format_delimited_section(title: &str, content: &str) -> String {
    let exchanges = parse_delimited_exchanges(content);
    if exchanges.is_empty() {
        return String::new();
    }

    let mut section = format!("## {title}\n\n");
    for (i, exchange) in exchanges.iter().enumerate() {
        let phase = detect_phase(&exchange.prompt);
        section.push_str(&format!("### Phase {}: {phase}\n\n", i + 1));

        if !exchange.prompt.is_empty() {
            let quoted: String = exchange
                .prompt
                .lines()
                .map(|l| format!("> {l}"))
                .collect::<Vec<_>>()
                .join("\n");
            section.push_str(&format!("{quoted}\n\n"));
        }

        if !exchange.response.is_empty() {
            section.push_str(&format!(
                "**Response:**\n\n{}\n\n",
                strip_trailing_rule(&exchange.response)
            ));
        }

        if i < exchanges.len() - 1 {
            section.push_str("---\n\n");
        }
    }
    section
}

fn uses_delimiter(window: &str) -> bool {
    matches!(window, "confluence" | "still-lake")
}

async fn write_journal(session: &str, test_name: &str, input: &str, ocean: &str) {
    let journal_dir = "target/e2e-journals";
    std::fs::create_dir_all(journal_dir).ok();

    let windows = list_session_windows(session).await;
    let mut journal = format!("# Flow Journal: {test_name}\n\n**Input:** {input}\n\n");

    for &name in JOURNAL_ORDER {
        if !windows.iter().any(|w| w == name) {
            continue;
        }
        let raw = capture_window_content(session, name).await;
        if raw.trim().is_empty() {
            continue;
        }

        let title = window_title(name);
        let section = if uses_delimiter(name) {
            format_delimited_section(title, &raw)
        } else {
            format_spring_section(title, &raw)
        };

        if !section.is_empty() {
            journal.push_str(&format!("---\n\n{section}"));
        }
    }

    journal.push_str(&format!("---\n\n## Ocean\n\n{ocean}\n"));

    let path = format!("{journal_dir}/{test_name}.md");
    std::fs::write(&path, &journal).ok();
    eprintln!("Journal written to {path}");
}

// --- Vessel-backed spring construction ---

/// Write a wrapper script that pipes input through `claude -p` with
/// the spring's system prompt and echoes a sentinel when done.
/// The pane shows every exchange -- the journey is visible.
fn write_wrapper_script(name: &str, model: &str, system_prompt: &str) -> String {
    let script_path = format!("/tmp/tao-e2e-{name}.sh");
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

fn vessel_source(
    session: &str,
    name: &str,
    model: &str,
    system_prompt: &str,
) -> Box<dyn LlmSource> {
    let script = write_wrapper_script(name, model, system_prompt);
    let vessel = TmuxVessel::new(session, name, model)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL);
    Box::new(TmuxPaneSource::new(vessel))
}

fn vessel_mountain(session: &str) -> Box<dyn Spring> {
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
        vessel_source(session, "mountain", MOUNTAIN_MODEL, mountain::SYSTEM_PROMPT),
    ))
}

fn vessel_desert(session: &str) -> Box<dyn Spring> {
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
        vessel_source(session, "desert", DESERT_MODEL, desert::SYSTEM_PROMPT),
    ))
}

fn vessel_forest(session: &str) -> Box<dyn Spring> {
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
        vessel_source(session, "forest", FOREST_MODEL, forest::SYSTEM_PROMPT),
    ))
}

const INPUT_DELIMITER: &str = "TAOFLOW_INPUT_END";

/// Wrapper script that reads multi-line input terminated by a delimiter.
/// No baked-in system prompt -- the system instructions travel with each message.
fn write_multiline_wrapper_script(name: &str, model: &str) -> String {
    let script_path = format!("/tmp/tao-e2e-{name}.sh");
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

/// Wraps TmuxPaneSource to embed system prompts into messages.
/// Confluence and Still Lake change system prompts between calls
/// (detect, yield, merge), so the prompt travels with the message.
struct EmbeddingVesselSource {
    inner: TmuxPaneSource,
}

#[async_trait]
impl LlmSource for EmbeddingVesselSource {
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

fn vessel_confluence_source(session: &str) -> Box<dyn LlmSource> {
    let script = write_multiline_wrapper_script("confluence", UTILITY_MODEL);
    let vessel = TmuxVessel::new(session, "confluence", UTILITY_MODEL)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL)
        .with_input_delimiter(INPUT_DELIMITER);
    Box::new(EmbeddingVesselSource {
        inner: TmuxPaneSource::new(vessel),
    })
}

fn vessel_lake_source(session: &str) -> Box<dyn LlmSource> {
    let script = write_multiline_wrapper_script("still-lake", UTILITY_MODEL);
    let vessel = TmuxVessel::new(session, "still-lake", UTILITY_MODEL)
        .with_command(format!("bash {script}"))
        .with_sentinel(SENTINEL)
        .with_input_delimiter(INPUT_DELIMITER);
    Box::new(EmbeddingVesselSource {
        inner: TmuxPaneSource::new(vessel),
    })
}

fn real_confluence(session: &str) -> ConfluencePool {
    ConfluencePool::new(vessel_confluence_source(session))
}

fn real_lake(session: &str) -> StillLake {
    StillLake::new(vessel_lake_source(session))
}

async fn vessel_tao(session: &str) -> TaoFlow {
    create_session(session).await;
    TaoFlow::new(
        Watershed::new(vec![
            vessel_mountain(session),
            vessel_desert(session),
            vessel_forest(session),
        ]),
        real_confluence(session),
        real_lake(session),
    )
}

/// System prompt fragments that should never leak into ocean output.
const SYSTEM_FRAGMENTS: &[&str] = &[
    "You are a Mountain Spring",
    "You are a Desert Spring",
    "You are a Forest Spring",
    "You are one voice among several",
    "trust that other springs",
];

fn assert_ocean_clean(content: &str) {
    assert!(
        !content.trim().is_empty(),
        "Ocean should have substance (non-empty)"
    );
    for fragment in SYSTEM_FRAGMENTS {
        assert!(
            !content.contains(fragment),
            "Ocean leaks system prompt: {fragment:?}"
        );
    }
}

// ============================================================
// Tier 1: The Droplet (single spring, ≤5 words)
//
// tmux attach -t tao-e2e-droplet to watch
// ============================================================

#[tokio::test]
#[ignore]
async fn droplet_flows_through_desert() {
    if !tmux_available().await || !claude_available().await {
        eprintln!("tmux or claude CLI not available, skipping");
        return;
    }

    let session = "tao-e2e-droplet";
    let mut tao = vessel_tao(session).await;

    // "What is the Tao?" = 5 words = Droplet → only desert responds
    let result = tokio::time::timeout(Duration::from_secs(60), tao.flow("What is the Tao?"))
        .await
        .expect("timed out after 60s")
        .expect("flow should produce an ocean");

    assert_ocean_clean(&result);
    write_journal(session, "droplet", "What is the Tao?", &result).await;

    cleanup(session).await;
}

// ============================================================
// Tier 2: The Shower (two springs, 6-30 words)
//
// tmux attach -t tao-e2e-shower to watch
// ============================================================

#[tokio::test]
#[ignore]
async fn shower_weaves_two_perspectives() {
    if !tmux_available().await || !claude_available().await {
        eprintln!("tmux or claude CLI not available, skipping");
        return;
    }

    let session = "tao-e2e-shower";
    let mut tao = vessel_tao(session).await;

    // 10 words = Shower → top 2 springs by affinity respond
    let input = "How does water teach us about patience and persistence?";
    let result = tokio::time::timeout(Duration::from_secs(300), tao.flow(input))
        .await
        .expect("timed out after 300s")
        .expect("flow should produce an ocean");

    assert_ocean_clean(&result);
    write_journal(session, "shower", input, &result).await;

    cleanup(session).await;
}

// ============================================================
// Tier 3: The Downpour (three springs, 31-100 words)
//
// tmux attach -t tao-e2e-downpour to watch
// ============================================================

#[tokio::test]
#[ignore]
async fn downpour_three_springs_merge() {
    if !tmux_available().await || !claude_available().await {
        eprintln!("tmux or claude CLI not available, skipping");
        return;
    }

    let session = "tao-e2e-downpour";
    let mut tao = vessel_tao(session).await;

    // ~45 words = Downpour → all three springs respond
    let input = "Compare and contrast the philosophical traditions of Taoism and Stoicism. \
        How do their approaches to acceptance, virtue, and the nature of reality differ? \
        Which tradition offers more practical guidance for navigating uncertainty in modern life, \
        and why might someone choose one path over the other?";
    let result = tokio::time::timeout(Duration::from_secs(600), tao.flow(input))
        .await
        .expect("timed out after 600s")
        .expect("flow should produce an ocean");

    assert_ocean_clean(&result);
    assert!(
        result.len() > 50,
        "Downpour ocean should be substantive, got {} chars",
        result.len()
    );
    write_journal(session, "downpour", input, &result).await;

    cleanup(session).await;
}

// ============================================================
// Tier 5: Multi-Turn (vapor carries context)
//
// tmux attach -t tao-e2e-vapor to watch
// ============================================================

#[tokio::test]
#[ignore]
async fn vapor_carries_context_across_real_flows() {
    if !tmux_available().await || !claude_available().await {
        eprintln!("tmux or claude CLI not available, skipping");
        return;
    }

    let session = "tao-e2e-vapor";
    let mut tao = vessel_tao(session).await;

    // Flow 1: establish context
    let first = tokio::time::timeout(Duration::from_secs(60), tao.flow("My name is River."))
        .await
        .expect("flow 1 timed out")
        .expect("flow 1 should succeed");

    assert_ocean_clean(&first);

    // Flow 2: follow-up that requires context from flow 1
    let second = tokio::time::timeout(Duration::from_secs(60), tao.flow("What is my name?"))
        .await
        .expect("flow 2 timed out")
        .expect("flow 2 should succeed");

    assert_ocean_clean(&second);

    // Vapor should track the conversation even though the vessel
    // springs are stateless (each claude -p call is independent).
    // Context carriage through vessel springs requires multi-line
    // input with conversation history -- a future enhancement.
    assert_eq!(
        tao.vapor().conversation_history.len(),
        4,
        "Vapor should have 4 messages (2 exchanges)"
    );

    let journal_ocean = format!("**Flow 1:** {first}\n\n**Flow 2:** {second}");
    write_journal(
        session,
        "vapor",
        "My name is River. → What is my name?",
        &journal_ocean,
    )
    .await;

    // Context carriage check: the vessel springs currently use stateless
    // claude -p calls, so the second flow may not recall the name.
    // This assertion validates when vapor-through-vessel is implemented.
    if second.to_lowercase().contains("river") {
        eprintln!("Vapor carried context successfully through vessel springs");
    } else {
        eprintln!(
            "Vapor context not carried (expected -- vessel springs are stateless). Got:\n{second}"
        );
    }

    cleanup(session).await;
}
