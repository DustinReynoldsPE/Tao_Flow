//! Capture 5 Still Lake responses for comparison.
//!
//! Saves each response to `.storms/lake-comparison/attempt-{N}.md`
//! and writes a comparison summary.
//!
//! Run: cargo test --test still_lake_comparison -- --ignored --nocapture

use std::time::Duration;

use tao_flow::still_lake::LAKE_SYSTEM_PROMPT;
use tao_flow::Pearl;

const STORM_PEARL_PATH: &str =
    ".storms/examine-the-relationship-between-three-ancient-20260309-181545/pearl.json";
const OUTPUT_DIR: &str = ".storms/lake-comparison";

async fn claude_available() -> bool {
    tokio::process::Command::new("claude")
        .arg("--version")
        .output()
        .await
        .map(|o| o.status.success())
        .unwrap_or(false)
}

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

async fn call_claude(system_prompt: &str, user_prompt: &str) -> Result<String, String> {
    let mut child = tokio::process::Command::new("claude")
        .args([
            "-p",
            "--model",
            "claude-haiku-4-5-20251001",
            "--setting-sources",
            "",
            "--system-prompt",
            system_prompt,
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
async fn capture_five_lake_responses() {
    if !claude_available().await {
        eprintln!("claude CLI not available, skipping");
        return;
    }

    let pearl_json = match std::fs::read_to_string(STORM_PEARL_PATH) {
        Ok(json) => json,
        Err(_) => {
            eprintln!("Storm pearl not found at {STORM_PEARL_PATH}");
            return;
        }
    };

    let pearl: Pearl = serde_json::from_str(&pearl_json).unwrap();
    let prompt = build_lake_prompt(&pearl);

    std::fs::create_dir_all(OUTPUT_DIR).unwrap();

    let mut responses: Vec<String> = Vec::new();

    for i in 1..=5 {
        eprintln!("Sending attempt {i}/5...");
        let response = tokio::time::timeout(
            Duration::from_secs(120),
            call_claude(LAKE_SYSTEM_PROMPT, &prompt),
        )
        .await
        .expect("timed out")
        .expect("claude call failed");

        let path = format!("{OUTPUT_DIR}/attempt-{i}.md");
        std::fs::write(&path, &response).unwrap();
        eprintln!("  Saved to {path} ({} chars)", response.len());

        responses.push(response);
    }

    eprintln!("\nAll 5 responses captured in {OUTPUT_DIR}/");
}
