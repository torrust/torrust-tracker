//! Assertions shared by the tests of the workspace packages.

/// Asserts that an error message names no source path.
///
/// Fails when `message` contains anything that looks like a source path: a workspace path, a
/// registry checkout, the standard library sources, or a `file.rs:line` suffix, written with
/// either path separator. It reports a failure at the caller's line.
///
/// Four other definitions of this function exist, because those packages cannot use this one.
/// Change all five together:
///
/// - `packages/http-protocol/src/test_helpers.rs` (adding a dev-dependency on this package would pull
///   tracker configuration and logging into a protocol crate's tests).
/// - `packages/primitives/src/peer.rs` (this package depends on `configuration`, which depends on
///   `primitives`: a dev-dependency would be a cycle).
/// - `packages/configuration/src/lib.rs` (this package depends on `configuration`: a dev-dependency
///   would be a cycle).
/// - `packages/axum-server/src/tls.rs` (no dev-dependency on this package, and none is added).
///
/// # Panics
///
/// Panics when `message` contains a source path fragment.
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

#[cfg(test)]
mod tests {
    use std::panic::catch_unwind;

    use super::assert_names_no_source_path;

    #[test]
    fn it_should_accept_a_message_without_a_source_path() {
        assert_names_no_source_path("the torrent is not in the whitelist");
    }

    #[test]
    fn it_should_reject_a_message_holding_a_source_path_fragment() {
        // Each message holds exactly one fragment, so the panic text names the fragment that
        // was caught, and a fragment removed from the list fails its own case.
        for (fragment, message) in [
            (".rs:", "failed at lib.rs:12"),
            ("packages/", "failed in packages/core/lib"),
            ("packages\\", "failed in packages\\core\\lib"),
            ("src/", "failed in src/lib"),
            ("src\\", "failed in src\\lib"),
            (".cargo/registry", "failed in /home/u/.cargo/registry"),
            ("rustlib/src", "failed in /toolchain/lib/rustlib/src"),
        ] {
            let Err(panic_payload) = catch_unwind(|| assert_names_no_source_path(message)) else {
                panic!("it should reject {message:?}");
            };

            let panic_text = panic_payload
                .downcast_ref::<String>()
                .expect("the panic text should be a String");
            assert!(
                panic_text.contains(&format!("it contains {fragment:?}")),
                "it should name {fragment:?} for {message:?}, but it panicked with {panic_text:?}"
            );
        }
    }
}
