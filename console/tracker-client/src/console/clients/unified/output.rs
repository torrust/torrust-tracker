//! The single place where `tracker_client` writes to stdout and stderr.
//!
//! Follows the global CLI output contract
//! (`docs/adrs/20260519000000_define_global_cli_output_contract.md`): stdout carries the result
//! object (one JSON value and a newline) and nothing else; stderr carries one JSON object per
//! line. Nothing else in the crate writes to either channel through a print macro, so none is needed.
//! The one other writer is the `tracing` subscriber, which `app` points at `std::io::stderr` with
//! its JSON formatter, so its events are JSON lines on stderr too.

use std::io::{self, IsTerminal as _, Write as _};

use serde::Serialize;

/// Message of the `tty_refusal` record, as the ADR shows it.
const TTY_REFUSAL_MESSAGE: &str = "stdout is a TTY; pipe the output to consume result data";

/// The one-line record written to stderr when stdout is a terminal.
#[must_use]
pub fn tty_refusal_record() -> String {
    serde_json::json!({ "kind": "tty_refusal", "message": TTY_REFUSAL_MESSAGE }).to_string()
}

/// Whether the result channel is a terminal, which the contract refuses.
#[must_use]
pub fn stdout_is_terminal() -> bool {
    io::stdout().is_terminal()
}

/// Writes the result to stdout: the rendered JSON and a newline.
///
/// # Errors
///
/// Returns an error if stdout cannot be written.
pub fn write_result(rendered: &str) -> io::Result<()> {
    writeln!(io::stdout().lock(), "{rendered}")
}

/// Renders `value` as the JSON result, pretty-printed when `pretty` is set.
///
/// # Errors
///
/// Returns an error if the value cannot be serialized.
pub fn render_json<T: Serialize>(value: &T, pretty: bool) -> serde_json::Result<String> {
    if pretty {
        serde_json::to_string_pretty(value)
    } else {
        serde_json::to_string(value)
    }
}

/// Writes one NDJSON record to stderr. The line holds one JSON object: any newline in a string
/// is escaped by the serializer.
///
/// A failure to write to stderr cannot be reported anywhere else, so it is ignored.
pub fn write_diagnostic(record: &str) {
    drop(writeln!(io::stderr().lock(), "{record}"));
}

/// Replaces the default panic hook, which writes plain text to stderr, with one that writes a
/// `panic` JSON record. The default hook is not chained.
pub fn install_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let message = info
            .payload()
            .downcast_ref::<&str>()
            .map(|message| (*message).to_string())
            .or_else(|| info.payload().downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "panic with a non-string payload".to_string());
        let location = info
            .location()
            .map(|location| format!("{}:{}", location.file(), location.line()));
        write_diagnostic(&serde_json::json!({ "kind": "panic", "message": message, "location": location }).to_string());
    }));
}

#[cfg(test)]
mod tests {
    use super::tty_refusal_record;

    #[test]
    fn the_tty_refusal_record_is_one_json_line_with_the_adr_kind() {
        let record = tty_refusal_record();

        assert!(!record.contains('\n'));
        let value: serde_json::Value = serde_json::from_str(&record).expect("record should be JSON");
        assert_eq!(value["kind"], "tty_refusal");
    }
}
