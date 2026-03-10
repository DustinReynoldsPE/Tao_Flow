use std::io::{self, Write};

use tao_flow::vessel::config::load_config_or_default;
use tao_flow::vessel::wiring::{build_tao_flow, cleanup_session};

#[tokio::main]
async fn main() {
    let config = load_config_or_default();
    let session = config.session.clone();
    let mut tao = build_tao_flow(&config).await;

    eprintln!("Three springs flow in tmux session '{session}'.");
    eprintln!("  tmux attach -t {session}");
    eprintln!();

    let stdin = io::stdin();
    loop {
        print!("> ");
        io::stdout().flush().ok();

        let mut input = String::new();
        match stdin.read_line(&mut input) {
            Ok(0) => break,
            Ok(_) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed == "/quit" || trimmed == "/exit" {
                    break;
                }
                match tao.flow(trimmed).await {
                    Ok(ocean) => println!("\n{ocean}\n"),
                    Err(e) => eprintln!("\nThe water could not reach the ocean: {e}\n"),
                }
            }
            Err(e) => {
                eprintln!("Failed to read input: {e}");
                break;
            }
        }
    }

    cleanup_session(&session).await;
}
