use std::sync::Arc;

use super::config::Configuration;
use super::console::Console;
use crate::checker::config::parse_from_json;
use crate::checker::service::Service;

pub const NUMBER_OF_ARGUMENTS: usize = 2;

/// # Panics
///
/// Will panic if:
///
/// - It can't read the json configuration file.
/// - The configuration file is invalid.
pub async fn run() {
    let args = parse_arguments();
    let config = setup_config(&args);
    let console_printer = Console {};
    let service = Service {
        config: Arc::new(config),
        console: console_printer,
    };

    service.run_checks().await;
}

pub struct Arguments {
    pub config_path: String,
}

fn parse_arguments() -> Arguments {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < NUMBER_OF_ARGUMENTS {
        eprintln!("Usage:       cargo run --bin tracker_checker <PATH_TO_CONFIG_FILE>");
        eprintln!("For example: cargo run --bin tracker_checker ./share/default/config/tracker_checker.json");
        std::process::exit(1);
    }

    let config_path = &args[1];

    Arguments {
        config_path: config_path.to_string(),
    }
}

fn setup_config(args: &Arguments) -> Configuration {
    let file_content = std::fs::read_to_string(args.config_path.clone())
        .unwrap_or_else(|_| panic!("Can't read config file {}", args.config_path));

    parse_from_json(&file_content).expect("Invalid config format")
}
