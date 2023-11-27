//! Minimal `curl` or `wget` to be used for container health checks.
//!
//! It's convenient to avoid using third-party libraries because:
//!
//! - They are harder to maintain.
//! - They introduce new attack vectors.
use std::{env, process};

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage:   cargo run --bin http_health_check <HEALTH_URL>");
        eprintln!("Example: cargo run --bin http_health_check http://127.0.0.1:1212/health_check");
        std::process::exit(1);
    }

    println!("Health check ...");

    let url = &args[1].clone();

    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                println!("STATUS: {}", response.status());
                process::exit(0);
            } else {
                println!("Non-success status received.");
                process::exit(1);
            }
        }
        Err(err) => {
            println!("ERROR: {err}");
            process::exit(1);
        }
    }
}
