---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md
last-updated-utc: "2026-11-24 17:06"
---

# Manual Verification Evidence

Real commands run on 2026-11-24 (UTC) on Linux, stable Rust `rustc 1.100.0 (0ef1bc386 2026-11-09)`, on the
implementation branch before the commits of T1 to T5 were pushed.

## V1 - Count the definitions (M1)

```console
$ grep -rn "fn assert_names_no_source_path" packages
packages/test-helpers/src/assertions.rs:24:pub fn assert_names_no_source_path(message: &str) {
packages/configuration/src/lib.rs:396:    fn assert_names_no_source_path(message: &str) {
packages/primitives/src/peer.rs:673:    fn assert_names_no_source_path(message: &str) {
packages/axum-server/src/tls.rs:70:    fn assert_names_no_source_path(message: &str) {
packages/http-protocol/src/test_helpers.rs:15:pub fn assert_names_no_source_path(message: &str) {
```

Result: exactly five definitions, in `test-helpers`, `configuration`, `primitives`, `axum-server` and
`http-protocol`. `git diff --stat -- '*.toml' Cargo.lock` shows no change. Status: DONE.

## V2 - The shared helper still catches a path (M2)

Scratch file `packages/tracker-core/tests/scratch_m2.rs` (a `tracker-core` integration test), deleted after the run:

```rust
use torrust_tracker_test_helpers::assertions::assert_names_no_source_path;

#[test]
fn scratch_m2() {
    assert_names_no_source_path("failed at packages/x.rs:1");
}
```

```console
$ cargo test -p torrust-tracker-core --test scratch_m2
running 1 test
test scratch_m2 ... FAILED
thread 'scratch_m2' panicked at packages/tracker-core/tests/scratch_m2.rs:5:5:
the error message must not hold a source path, but it contains ".rs:": "failed at packages/x.rs:1"
```

Result: the test fails with the unified message at the calling test line (`#[track_caller]`). The scratch file was
removed (`git status` shows no trace of it). Status: DONE.

## Automatic checks

`linter all` exits 0 (after shortening the first doc paragraph of the shared function, clippy
`too_long_first_doc_paragraph`). `cargo test` passes for `torrust-tracker-test-helpers`, `-core`, `-axum-http-server`,
`-axum-rest-api-server`, `-udp-server`, `-http-protocol`, `-primitives`, `-configuration` and `-axum-server`.
