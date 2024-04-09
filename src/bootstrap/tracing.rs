//! Setup for the application tracing.
//!
//! It redirects the tracing info to the standard output with the tracing level defined in the configuration.
//!
//! (case is ignored)
//!
//! - `Off` (i.e. don't load any subscriber...)
//! - `Error`
//! - `Warn`
//! - `Info`
//! - `Debug`
//! - `Trace`
//!
//! Refer to the [configuration crate documentation](https://docs.rs/torrust-tracker-configuration) to know how to change tracing settings.
use std::sync::Once;

use torrust_tracker_configuration::{Configuration, TraceLevel};
use tracing::debug;
use tracing::level_filters::LevelFilter;

static INIT: Once = Once::new();

#[derive(Debug)]
enum TraceStyle {
    Default,
    Pretty(bool),
    Compact,
    Json,
}

impl TraceStyle {
    fn new(style: &torrust_tracker_configuration::TraceStyle, filter: LevelFilter) -> Self {
        match style.to_string().as_str() {
            "full" => Self::Default,
            "pretty" => {
                // TRACE < DEBUG < INFO < ERROR < OFF
                let default = LevelFilter::DEBUG <= filter;
                Self::Pretty(default)
            }
            "pretty_with_paths" => Self::Pretty(true),
            "pretty_without_paths" => Self::Pretty(false),
            "compact" => Self::Compact,
            "json" => Self::Json,

            _ => panic!(
                "
Error! Unrecognized `trace_style`!

            Found:    \"{filter}\"

But, the possible values are:
                    - `full`
    (default)  ---> - `pretty`
                    - `pretty_with_paths`
                    - `pretty_without_paths`
                    - `compact`
                    - `json`
"
            ),
        }
    }
}

impl std::fmt::Display for TraceStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let style = match self {
            TraceStyle::Default => "Default Style",
            TraceStyle::Pretty(path) => match path {
                true => "Pretty Style with File Paths",
                false => "Pretty Style without File Paths",
            },

            TraceStyle::Compact => "Compact Style",
            TraceStyle::Json => "Json Format",
        };

        f.write_str(style)
    }
}

/// It redirects the tracing info to the standard output with the tracing level defined in the configuration
pub fn setup(cfg: &Configuration) {
    let filter = get_level_filter(&cfg.tracing_max_verbosity_level);
    let style = TraceStyle::new(&cfg.tracing_format_style, filter);

    if filter == LevelFilter::OFF {
        return;
    }

    INIT.call_once(|| {
        stdout_init(filter, &style);
    });
}

/// Gets the [`LevelFilter`] from the String in [`TraceLevel`].
///
/// # Panics
///
/// If the string cannot be parsed into a valid filter level.
///
fn get_level_filter(filter: &TraceLevel) -> LevelFilter {
    filter.to_string().parse().unwrap_or_else(|_| {
        panic!(
            "
Error! Unrecognized `log_level` (or alias) `trace_filter`!

           Found:     \"{filter}\"

But, the possible values are:
                    - `off`
                    - `error` (strongest)
    (default)  ---> - `info`
                    - `debug`
                    - `trace` (weakest)
"
        )
    })
}

fn stdout_init(filter: LevelFilter, style: &TraceStyle) {
    let builder = tracing_subscriber::fmt().with_max_level(filter);

    let () = match style {
        TraceStyle::Default => builder.init(),
        TraceStyle::Pretty(display_filename) => builder.pretty().with_file(*display_filename).init(),
        TraceStyle::Compact => builder.compact().init(),
        TraceStyle::Json => builder.json().init(),
    };

    debug!("tracing initialized.");
}
