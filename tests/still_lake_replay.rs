//! Replay test for the Still Lake refusal.
//!
//! Sends the exact river content that previously triggered a "prompt injection"
//! refusal through the Still Lake system prompt 5 times, verifying each response
//! is substantive and not a refusal.
//!
//! Requires: claude CLI with valid authentication.
//! Run: cargo test --test still_lake_replay -- --ignored --nocapture

use std::time::Duration;

use tao_flow::still_lake::LAKE_SYSTEM_PROMPT;
use tao_flow::Pearl;

const REFUSAL_PATTERNS: &[&str] = &[
    "prompt injection",
    "not a legitimate task",
    "appears to be a prompt",
    "I need to be direct",
    "designed to prevent me",
    "What should we actually do",
];

const STORM_PEARL_PATH: &str =
    ".storms/examine-the-relationship-between-three-ancient-20260309-181545/pearl.json";

async fn claude_available() -> bool {
    tokio::process::Command::new("claude")
        .arg("--version")
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Reconstruct the Still Lake prompt from a Pearl's river, matching StillLake::build_prompt.
fn build_lake_prompt(pearl: &Pearl) -> String {
    let river = pearl.river.as_ref().expect("pearl must have a river");

    let depth_label = if river.clarity >= 0.75 {
        "This response was woven from multiple perspectives with good agreement.\n\
         Apply a light touch. Polish for clarity and simplicity. \
         Remove any roughness from the weaving.\n\n"
    } else if river.clarity >= 0.5 {
        "This response carries some unresolved tension \
         from the merging of perspectives.\n\
         Settle the remaining turbulence. \
         Ensure the response is whole and true.\n\n"
    } else {
        "This response carries significant unresolved disagreement \
         from multiple perspectives.\n\
         The water is muddy. Settle it deeply. \
         The reader deserves clarity.\n\n"
    };

    let mut prompt = format!(
        "The user asked: {}\n\n\
         The river carries this response:\n\n{}\n\n{}",
        pearl.core, river.content, depth_label
    );

    let unresolved: Vec<_> = river.eddies.iter().filter(|e| !e.is_resolved()).collect();
    if !unresolved.is_empty() {
        prompt.push_str("Unresolved points of divergence that need your attention:\n\n");
        for eddy in &unresolved {
            prompt.push_str(&format!("- {} ({:?}): ", eddy.topic, eddy.nature));
            for (i, pos) in eddy.positions.iter().enumerate() {
                if i > 0 {
                    prompt.push_str(" vs ");
                }
                prompt.push_str(&format!("\"{}\"", pos.view));
            }
            prompt.push('\n');
        }
        prompt.push_str(
            "\nResolve these where possible. For factual disagreements, \
             determine the most likely correct answer. For interpretive ones, \
             honor both perspectives naturally.\n\n",
        );
    }

    prompt.push_str(
        "Apply the five questions:\n\
         1. Clarity — can the reader understand without effort?\n\
         2. Wholeness — is the full question addressed?\n\
         3. Kindness — is this respectful and compassionate?\n\
         4. Truth — is it honest about what it knows and doesn't?\n\
         5. Simplicity — can anything be removed?\n\n\
         Return only the settled response. No commentary. No meta-text.",
    );

    prompt
}

/// Call `claude -p` with the lake system prompt and user prompt.
async fn call_claude(system_prompt: &str, user_prompt: &str) -> Result<String, String> {
    let escaped_system = system_prompt.replace('\'', "'\\''");

    let mut child = tokio::process::Command::new("claude")
        .args([
            "-p",
            "--model",
            "claude-haiku-4-5-20251001",
            "--setting-sources",
            "",
            "--system-prompt",
            &escaped_system,
        ])
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .env_remove("CLAUDECODE")
        .spawn()
        .map_err(|e| format!("Failed to spawn claude: {e}"))?;

    use tokio::io::AsyncWriteExt;
    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(user_prompt.as_bytes())
            .await
            .map_err(|e| format!("Failed to write to stdin: {e}"))?;
    }

    let output = child
        .wait_with_output()
        .await
        .map_err(|e| format!("Failed to wait for claude: {e}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("claude exited with error: {stderr}"));
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[tokio::test]
#[ignore]
async fn still_lake_settles_self_referential_content() {
    if !claude_available().await {
        eprintln!("claude CLI not available, skipping");
        return;
    }

    let pearl_json = match std::fs::read_to_string(STORM_PEARL_PATH) {
        Ok(json) => json,
        Err(_) => {
            eprintln!("Storm pearl not found at {STORM_PEARL_PATH}");
            eprintln!(
                "Run the storm e2e test first: cargo test --test e2e_flow storm -- --ignored"
            );
            return;
        }
    };

    let pearl: Pearl = serde_json::from_str(&pearl_json).expect("pearl.json should deserialize");
    let prompt = build_lake_prompt(&pearl);

    eprintln!(
        "Replaying Still Lake with river content ({} chars, clarity {:.2})",
        pearl.river.as_ref().unwrap().content.len(),
        pearl.river.as_ref().unwrap().clarity
    );

    for attempt in 1..=5 {
        let result = tokio::time::timeout(
            Duration::from_secs(120),
            call_claude(LAKE_SYSTEM_PROMPT, &prompt),
        )
        .await
        .unwrap_or_else(|_| panic!("attempt {attempt}: timed out after 120s"));

        let response =
            result.unwrap_or_else(|e| panic!("attempt {attempt}: claude call failed: {e}"));

        // Check for refusal patterns
        for pattern in REFUSAL_PATTERNS {
            assert!(
                !response.to_lowercase().contains(&pattern.to_lowercase()),
                "attempt {attempt}: Still Lake refused with pattern \"{pattern}\"\n\
                 Response starts with: {}",
                &response[..response.len().min(200)]
            );
        }

        assert!(
            response.len() > 100,
            "attempt {attempt}: response too short ({} chars), likely not substantive",
            response.len()
        );

        eprintln!(
            "Attempt {attempt}/5: {} chars — {}...",
            response.len(),
            &response[..response.len().min(80)]
        );
    }

    eprintln!("All 5 attempts produced substantive, non-refusal responses.");
}
