//! Vessel integration tests -- Level 2.
//!
//! These tests verify the tmux plumbing works using echo processes
//! instead of real LLMs. They require tmux but not Claude or llama.cpp.
//!
//! Run with: cargo test --test vessel_integration -- --ignored

use tao_flow::vessel::TmuxVessel;
use tao_flow::watershed::source::{ChatMessage, ChatRole, LlmSource};
use tao_flow::watershed::TmuxPaneSource;

fn echo_vessel(session: &str, window: &str) -> TmuxVessel {
    TmuxVessel::new(session, window, "unused").with_command("cat")
}

/// Kill a test session, ignoring errors if it doesn't exist.
async fn cleanup(session: &str) {
    tokio::process::Command::new("tmux")
        .args(["kill-session", "-t", session])
        .status()
        .await
        .ok();
}

async fn tmux_available() -> bool {
    tokio::process::Command::new("tmux")
        .arg("-V")
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

async fn session_exists(session: &str) -> bool {
    tokio::process::Command::new("tmux")
        .args(["has-session", "-t", session])
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}

#[tokio::test]
#[ignore]
async fn vessel_creates_and_tears_down_session() {
    if !tmux_available().await {
        return;
    }
    let session = "tao-test-lifecycle";
    cleanup(session).await;

    let mut vessel = echo_vessel(session, "test");
    vessel.prepare("unused").await.unwrap();

    assert!(
        session_exists(session).await,
        "session should exist after prepare"
    );

    vessel.teardown().await.unwrap();

    assert!(
        !session_exists(session).await,
        "session should be gone after teardown"
    );
}

#[tokio::test]
#[ignore]
async fn vessel_send_and_capture_with_cat() {
    if !tmux_available().await {
        return;
    }
    let session = "tao-test-echo";
    cleanup(session).await;

    let mut vessel = echo_vessel(session, "echo");
    vessel.prepare("unused").await.unwrap();

    let response = vessel.send("hello from the vessel").await.unwrap();

    assert!(
        response.contains("hello from the vessel"),
        "expected echo of input, got: {response:?}"
    );

    vessel.teardown().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn vessel_prepare_is_idempotent() {
    if !tmux_available().await {
        return;
    }
    let session = "tao-test-idempotent";
    cleanup(session).await;

    let mut vessel = echo_vessel(session, "test");
    vessel.prepare("first").await.unwrap();
    vessel.prepare("second").await.unwrap();

    let response = vessel.send("still working").await.unwrap();
    assert!(
        response.contains("still working"),
        "vessel should still work after double prepare, got: {response:?}"
    );

    vessel.teardown().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn vessel_empty_input_returns_empty() {
    if !tmux_available().await {
        return;
    }
    let session = "tao-test-empty";
    cleanup(session).await;

    let mut vessel = echo_vessel(session, "test");
    vessel.prepare("unused").await.unwrap();

    let response = vessel.send("").await.unwrap();
    assert!(
        response.is_empty(),
        "empty input should produce empty output"
    );

    vessel.teardown().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn vessel_adds_window_to_existing_session() {
    if !tmux_available().await {
        return;
    }
    let session = "tao-test-multi";
    cleanup(session).await;

    let mut mountain = echo_vessel(session, "mountain");
    mountain.prepare("unused").await.unwrap();

    let mut desert = echo_vessel(session, "desert");
    desert.prepare("unused").await.unwrap();

    let m_response = mountain.send("deep thought").await.unwrap();
    let d_response = desert.send("quick answer").await.unwrap();

    assert!(
        m_response.contains("deep thought"),
        "mountain should echo, got: {m_response:?}"
    );
    assert!(
        d_response.contains("quick answer"),
        "desert should echo, got: {d_response:?}"
    );

    mountain.teardown().await.unwrap();
}

#[tokio::test]
#[ignore]
async fn tmux_pane_source_flows_as_llm_source() {
    if !tmux_available().await {
        return;
    }
    let session = "tao-test-source";
    cleanup(session).await;

    let vessel = echo_vessel(session, "test");
    let source = TmuxPaneSource::new(vessel);

    let messages = vec![ChatMessage {
        role: ChatRole::User,
        content: "water finds its path".into(),
    }];

    let response = source.complete("unused", &messages).await.unwrap();
    assert!(
        response.contains("water finds its path"),
        "TmuxPaneSource should return vessel response, got: {response:?}"
    );

    cleanup(session).await;
}

#[tokio::test]
#[ignore]
async fn tmux_pane_source_sends_only_last_user_message() {
    if !tmux_available().await {
        return;
    }
    let session = "tao-test-lastmsg";
    cleanup(session).await;

    let vessel = echo_vessel(session, "test");
    let source = TmuxPaneSource::new(vessel);

    let messages = vec![
        ChatMessage {
            role: ChatRole::User,
            content: "first question".into(),
        },
        ChatMessage {
            role: ChatRole::Assistant,
            content: "first answer".into(),
        },
        ChatMessage {
            role: ChatRole::User,
            content: "second question".into(),
        },
    ];

    let response = source.complete("unused", &messages).await.unwrap();
    assert!(
        response.contains("second question"),
        "should send only the last user message, got: {response:?}"
    );

    cleanup(session).await;
}
