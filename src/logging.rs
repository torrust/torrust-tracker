use std::str::FromStr;
use std::sync::Once;

use log::{info, LevelFilter};

use crate::config::Configuration;

static INIT: Once = Once::new();

pub fn setup_logging(cfg: &Configuration) {
    let level = config_level_or_default(&cfg.log_level);

    if level == log::LevelFilter::Off {
        return;
    }

    INIT.call_once(|| {
        stdout_config(level);
    });
}

fn config_level_or_default(log_level: &Option<String>) -> LevelFilter {
    match log_level {
        None => log::LevelFilter::Info,
        Some(level) => LevelFilter::from_str(level).unwrap(),
    }
}

fn stdout_config(level: LevelFilter) {
    if let Err(_err) = fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{} [{}][{}] {}",
                chrono::Local::now().format("%+"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }

    info!("logging initialized.");
}
