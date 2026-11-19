//! The tracked schema artifact must be the one the model generates.

use std::fs;
use std::path::Path;
use std::process::Command;

use agent_reply_validator::schema::{ARTIFACT_PATH, reply_schema_json};

#[test]
fn it_should_track_the_schema_that_the_model_generates() {
    // Arrange
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..");
    let tracked = fs::read_to_string(repository_root.join(ARTIFACT_PATH)).expect("the schema artifact is tracked");

    // Act
    let generated = reply_schema_json();

    // Assert
    assert!(
        tracked == generated,
        "{ARTIFACT_PATH} differs from the model; regenerate it with \
         `cargo run --package agent-reply-validator --bin agent-reply-schema`"
    );
}

#[test]
fn it_should_refuse_any_argument_and_write_nothing() {
    // Arrange
    let artifact = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..").join(ARTIFACT_PATH);
    let modified = |path: &Path| fs::metadata(path).and_then(|metadata| metadata.modified()).ok();
    let before = (fs::read_to_string(&artifact).ok(), modified(&artifact));

    // Act
    let output = Command::new(env!("CARGO_BIN_EXE_agent-reply-schema"))
        .arg("check")
        .output()
        .expect("the binary runs");

    // Assert
    assert_eq!(output.status.code(), Some(2));
    assert_eq!(String::from_utf8_lossy(&output.stdout), "");
    let record: serde_json::Value = serde_json::from_slice(&output.stderr).expect("stderr holds one JSON record");
    assert_eq!(record["kind"], "usage_error");
    assert_eq!((fs::read_to_string(&artifact).ok(), modified(&artifact)), before);
}
