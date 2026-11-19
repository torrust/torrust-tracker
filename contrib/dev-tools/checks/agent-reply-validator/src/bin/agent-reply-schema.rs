//! Writes the `agent-reply-v1` JSON Schema to its tracked path.
//!
//! Run `cargo run --package agent-reply-validator --bin agent-reply-schema`. The file is written
//! in the repository this crate was built from, whatever the working directory. The test
//! `tests/schema.rs` fails when the tracked file differs from the model, so a model change cannot
//! be pushed with a stale schema.
//!
//! Output class `no-stdout-result`: stdout stays empty; one NDJSON record goes to stderr; exit
//! codes are `0` when the file is written, `1` when it cannot be, and `2` for any argument.

use std::io::{self, Write};
use std::path::Path;
use std::process::ExitCode;
use std::{env, fs};

use agent_reply_validator::schema::{ARTIFACT_PATH, reply_schema_json};
use serde_json::json;

fn main() -> ExitCode {
    let (record, exit_code) = if env::args_os().len() > 1 {
        (
            json!({
                "kind": "usage_error",
                "message": "agent-reply-schema takes no arguments",
                "exit_code": 2
            }),
            2,
        )
    } else {
        let artifact = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..").join(ARTIFACT_PATH);
        match fs::write(artifact, reply_schema_json()) {
            Ok(()) => (json!({ "kind": "written", "path": ARTIFACT_PATH }), 0),
            Err(error) => (
                json!({
                    "kind": "runtime_error",
                    "path": ARTIFACT_PATH,
                    "message": error.to_string(),
                    "exit_code": 1
                }),
                1,
            ),
        }
    };
    let mut stderr = io::stderr().lock();
    match writeln!(stderr, "{record}") {
        Ok(()) => ExitCode::from(exit_code),
        Err(_) => ExitCode::FAILURE,
    }
}
