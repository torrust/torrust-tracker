//! Helpers shared by the unit tests of this crate.

/// Fails when `message` contains anything that looks like a source path: a workspace path, a
/// registry checkout, the standard library sources, or a `file.rs:line` suffix, written with
/// either path separator. It reports a failure at the caller's line.
///
/// This definition is separate because a dev-dependency on `torrust-tracker-test-helpers` would
/// pull tracker configuration and logging into a protocol crate's tests.
/// Four other definitions exist; change all five together:
///
/// - `packages/test-helpers/src/assertions.rs`
/// - `packages/primitives/src/peer.rs`
/// - `packages/configuration/src/lib.rs`
/// - `packages/axum-server/src/tls.rs`
#[track_caller]
pub fn assert_names_no_source_path(message: &str) {
    for fragment in [
        ".rs:",
        "packages/",
        "packages\\",
        "src/",
        "src\\",
        ".cargo/registry",
        "rustlib/src",
    ] {
        assert!(
            !message.contains(fragment),
            "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
        );
    }
}
