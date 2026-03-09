use std::io::{self, Write};

use tao_flow::vessel::wiring::{build_tao_flow, cleanup_session, VesselConfig};

const SESSION: &str = "tao-flow";

#[tokio::main]
async fn main() {
    let config = VesselConfig::new(SESSION);
    let mut tao = build_tao_flow(&config).await;

    eprintln!("Three springs flow in tmux session '{SESSION}'.");
    eprintln!("  tmux attach -t {SESSION}");
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

    cleanup_session(SESSION).await;
}
