//! Setup for the application logging.
//!
//! It redirects the log info to the standard output with the log level defined in the configuration.
//!
//! - `Off`
//! - `Error`
//! - `Warn`
//! - `Info`
//! - `Debug`
//! - `Trace`
//!
//! Refer to the [configuration crate documentation](https://docs.rs/torrust-tracker-configuration) to know how to change log settings.
use std::sync::Once;

use log::{info, LevelFilter};
use torrust_tracker_configuration::{Configuration, LogLevel};

static INIT: Once = Once::new();

/// It redirects the log info to the standard output with the log level defined in the configuration
pub fn setup(cfg: &Configuration) {
    let level = config_level_or_default(&cfg.core.log_level);

    if level == log::LevelFilter::Off {
        return;
    }

    INIT.call_once(|| {
        stdout_config(level);
    });
}

fn config_level_or_default(log_level: &Option<LogLevel>) -> LevelFilter {
    match log_level {
        None => log::LevelFilter::Info,
        Some(level) => match level {
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        },
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
            ));
        })
        .level(level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }

    info!("logging initialized.");
}
