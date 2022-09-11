use log::info;

use crate::Configuration;

pub fn setup_logging(cfg: &Configuration) {
    let log_level = match &cfg.log_level {
        None => log::LevelFilter::Info,
        Some(level) => match level.as_str() {
            "off" => log::LevelFilter::Off,
            "trace" => log::LevelFilter::Trace,
            "debug" => log::LevelFilter::Debug,
            "info" => log::LevelFilter::Info,
            "warn" => log::LevelFilter::Warn,
            "error" => log::LevelFilter::Error,
            _ => {
                panic!("Unknown log level encountered: '{}'", level.as_str());
            }
        },
    };

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
        .level(log_level)
        .chain(std::io::stdout())
        .apply()
    {
        panic!("Failed to initialize logging.")
    }
    info!("logging initialized.");
}
