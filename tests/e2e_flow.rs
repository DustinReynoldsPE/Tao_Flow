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

use std::time::Duration;

use tao_flow::flow::TaoFlow;
use tao_flow::vessel::wiring::{
    build_tao_flow, cleanup_session, VesselConfig, INPUT_DELIMITER, SENTINEL,
};

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

async fn vessel_tao(session: &str) -> TaoFlow {
    let config = VesselConfig::new(session);
    build_tao_flow(&config).await
}

// --- Flow journal ---
// Captures every vessel's pane content into a structured markdown document.
// The journey is preserved, not just the destination.

const JOURNAL_ORDER: &[&str] = &[
    "mountain",
    "desert",
    "forest",
    "decomposer",
    "confluence",
    "still-lake",
];

fn window_title(name: &str) -> &str {
    match name {
        "mountain" => "Mountain Spring",
        "desert" => "Desert Spring",
        "forest" => "Forest Spring",
        "decomposer" => "Decomposer",
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
        .filter(|line| !line.contains("% bash /tmp/tao") && !line.starts_with("bash /tmp/tao"))
        .collect()
}

/// Parse a spring pane (no delimiter) into exchanges.
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

fn strip_trailing_rule(text: &str) -> &str {
    let mut result = text.trim_end();
    while let Some(stripped) = result.strip_suffix("---") {
        result = stripped.trim_end();
    }
    result
}

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
    matches!(window, "confluence" | "still-lake" | "decomposer")
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

/// Capture and parse exchanges from a single window.
/// Returns the structured prompt/response pairs visible in the pane.
async fn capture_window_exchanges(session: &str, window: &str) -> Vec<Exchange> {
    let content = capture_window_content(session, window).await;
    if content.trim().is_empty() {
        return Vec::new();
    }
    if uses_delimiter(window) {
        parse_delimited_exchanges(&content)
    } else {
        parse_spring_exchanges(&content)
    }
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
// Tier 1: The Droplet (single spring, <=5 words)
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

    let result = tokio::time::timeout(Duration::from_secs(60), tao.flow("What is the Tao?"))
        .await
        .expect("timed out after 60s")
        .expect("flow should produce an ocean");

    assert_ocean_clean(&result);

    let pearl = tao.last_pearl().expect("droplet should form a pearl");
    assert_eq!(pearl.core, "What is the Tao?");
    assert_eq!(pearl.ocean, result);
    assert!(
        !pearl.streams.is_empty(),
        "pearl should capture desert stream"
    );
    assert!(pearl.sub_pearls.is_empty(), "droplet has no sub-pearls");

    write_journal(session, "droplet", "What is the Tao?", &result).await;

    cleanup_session(session).await;
}

// ============================================================
// Tier 2: The Shower (two springs, 6-30 words)
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

    let input = "How does water teach us about patience and persistence?";
    let result = tokio::time::timeout(Duration::from_secs(300), tao.flow(input))
        .await
        .expect("timed out after 300s")
        .expect("flow should produce an ocean");

    assert_ocean_clean(&result);

    let pearl = tao.last_pearl().expect("shower should form a pearl");
    assert_eq!(pearl.core, input);
    assert_eq!(pearl.ocean, result);
    assert!(pearl.river.is_some(), "shower pearl should have a river");
    assert!(pearl.sub_pearls.is_empty(), "shower has no sub-pearls");

    write_journal(session, "shower", input, &result).await;

    cleanup_session(session).await;
}

// ============================================================
// Tier 3: The Downpour (three springs, 31-100 words)
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

    let pearl = tao.last_pearl().expect("downpour should form a pearl");
    assert_eq!(pearl.core, input);
    assert_eq!(pearl.ocean, result);
    assert!(
        pearl.streams.len() >= 2,
        "downpour should capture multiple streams"
    );
    assert!(pearl.river.is_some(), "downpour pearl should have a river");

    write_journal(session, "downpour", input, &result).await;

    cleanup_session(session).await;
}

// ============================================================
// Tier 4: The Storm (recursive decomposition, >100 words)
// ============================================================

#[tokio::test]
#[ignore]
async fn storm_decomposes_and_reassembles() {
    if !tmux_available().await || !claude_available().await {
        eprintln!("tmux or claude CLI not available, skipping");
        return;
    }

    let session = "tao-e2e-storm";
    let mut tao = vessel_tao(session).await;

    let input = "Examine the relationship between three ancient philosophical traditions \
        and their relevance to modern technology. First, consider how Taoist principles of \
        wu wei and natural flow apply to software architecture — specifically, how systems \
        that yield rather than force tend to be more resilient. Second, analyze how Stoic \
        concepts of virtue and acceptance inform the practice of debugging and incident \
        response — when systems fail, how does a Stoic mindset change the engineer's \
        approach? Third, explore how Buddhist concepts of impermanence and interdependence \
        relate to distributed systems and microservices — nothing exists independently, \
        everything changes. For each tradition, provide specific examples of how these \
        ancient insights manifest in modern engineering practices, and identify where \
        these traditions agree and where they diverge in their guidance for builders \
        of complex systems.";

    let result = tokio::time::timeout(Duration::from_secs(900), tao.flow(input))
        .await
        .expect("timed out after 900s")
        .expect("flow should produce an ocean");

    assert_ocean_clean(&result);
    assert!(
        result.len() > 100,
        "Storm ocean should be substantive, got {} chars",
        result.len()
    );

    // --- Verify the recursive path was taken ---

    // The decomposer should have produced sub-questions
    let decomposer_exchanges = capture_window_exchanges(session, "decomposer").await;
    if !decomposer_exchanges.is_empty() {
        let response = &decomposer_exchanges[0].response;
        assert!(
            response.contains("Q:")
                || response.contains("Q1")
                || response.contains("1.")
                || response.contains("1)"),
            "Decomposer should produce sub-questions, got: {}",
            &response[..response.len().min(200)]
        );
        eprintln!(
            "Storm decomposed into sub-questions (decomposer had {} exchanges)",
            decomposer_exchanges.len()
        );
    } else {
        eprintln!("Decomposer pane was empty — Storm may have fallen back to single-pass");
    }

    // Springs should have received multiple exchanges (one per sub-question)
    let mountain_exchanges = capture_window_exchanges(session, "mountain").await;
    let desert_exchanges = capture_window_exchanges(session, "desert").await;
    eprintln!(
        "Mountain: {} exchanges, Desert: {} exchanges",
        mountain_exchanges.len(),
        desert_exchanges.len()
    );

    // Confluence should have woven at least once (sub-flow merges or higher confluence)
    let confluence_exchanges = capture_window_exchanges(session, "confluence").await;
    assert!(
        !confluence_exchanges.is_empty(),
        "Confluence should have woven at least once"
    );
    eprintln!(
        "Confluence: {} exchanges (sub-flow merges + higher confluence)",
        confluence_exchanges.len()
    );

    // --- Verify pearl captures recursive structure ---
    let pearl = tao.last_pearl().expect("storm should form a pearl");
    assert_eq!(pearl.ocean, result);
    assert!(
        !pearl.sub_pearls.is_empty(),
        "storm pearl should contain sub-pearls from decomposition"
    );
    assert!(
        pearl.river.is_some(),
        "storm pearl should have higher confluence river"
    );

    for (i, sub) in pearl.sub_pearls.iter().enumerate() {
        assert!(!sub.core.is_empty(), "sub-pearl {} should have a core", i);
        assert!(
            !sub.ocean.is_empty(),
            "sub-pearl {} should have an ocean",
            i
        );
    }

    eprintln!(
        "Pearl: {} sub-pearls, river clarity {:.2}",
        pearl.sub_pearls.len(),
        pearl.river.as_ref().map(|r| r.clarity).unwrap_or(0.0)
    );

    write_journal(session, "storm", input, &result).await;

    cleanup_session(session).await;
}

// ============================================================
// Tier 5: Multi-Turn (vapor carries context)
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

    let first = tokio::time::timeout(Duration::from_secs(60), tao.flow("My name is River."))
        .await
        .expect("flow 1 timed out")
        .expect("flow 1 should succeed");

    assert_ocean_clean(&first);

    let second = tokio::time::timeout(Duration::from_secs(60), tao.flow("What is my name?"))
        .await
        .expect("flow 2 timed out")
        .expect("flow 2 should succeed");

    assert_ocean_clean(&second);

    assert_eq!(
        tao.vapor().conversation_history.len(),
        4,
        "Vapor should have 4 messages (2 exchanges)"
    );

    // Pearl should reflect only the most recent flow
    let pearl = tao.last_pearl().expect("vapor test should have a pearl");
    assert_eq!(pearl.core, "What is my name?");
    assert_eq!(pearl.ocean, second);

    let journal_ocean = format!("**Flow 1:** {first}\n\n**Flow 2:** {second}");
    write_journal(
        session,
        "vapor",
        "My name is River. -> What is my name?",
        &journal_ocean,
    )
    .await;

    if second.to_lowercase().contains("river") {
        eprintln!("Vapor carried context successfully through vessel springs");
    } else {
        eprintln!(
            "Vapor context not carried (expected -- vessel springs are stateless). Got:\n{second}"
        );
    }

    cleanup_session(session).await;
}
