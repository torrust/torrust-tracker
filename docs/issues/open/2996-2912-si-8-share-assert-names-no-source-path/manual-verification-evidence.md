---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2996-2912-si-8-share-assert-names-no-source-path/ISSUE.md
last-updated-utc: "2026-11-25 11:37"
---

# Manual Verification Evidence

Real commands run on 2026-11-24 and 2026-11-25 (UTC) on Linux, stable Rust `rustc 1.100.0 (0ef1bc386 2026-11-09)`, on the
implementation branch: V1 and V2 before the commits of T1 to T5 were pushed, the V1 re-run, V3 and V4 in review round 2
of PR 3041.

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

Re-run on 2026-11-25, after the fix of review finding F4 of PR 3041 wrapped one doc line above four of the kept
copies, so their line numbers moved by one:

```console
$ grep -rn "fn assert_names_no_source_path" packages
packages/test-helpers/src/assertions.rs:24:pub fn assert_names_no_source_path(message: &str) {
packages/http-protocol/src/test_helpers.rs:16:pub fn assert_names_no_source_path(message: &str) {
packages/configuration/src/lib.rs:397:    fn assert_names_no_source_path(message: &str) {
packages/primitives/src/peer.rs:674:    fn assert_names_no_source_path(message: &str) {
packages/axum-server/src/tls.rs:71:    fn assert_names_no_source_path(message: &str) {
```

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

## V3 - The panic wording is the same in the five definitions (AC4)

Run on 2026-11-25 (review finding F2 of PR 3041):

```console
$ grep -rn 'must not hold a source path' packages
packages/test-helpers/src/assertions.rs:36:            "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
packages/http-protocol/src/test_helpers.rs:28:            "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
packages/configuration/src/lib.rs:409:                "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
packages/primitives/src/peer.rs:686:                "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
packages/axum-server/src/tls.rs:83:                "the error message must not hold a source path, but it contains {fragment:?}: {message:?}"
```

Result: one line per definition, the same text after indentation. Status: DONE.

## V4 - Each fragment of the shared function is pinned by its own test case (AC1)

Run on 2026-11-25 (review finding F1 of PR 3041). The test `it_should_reject_a_message_holding_a_source_path_fragment`
gives each fragment a message that holds only that fragment and checks that the panic text names it. On a scratch
change that removed `"rustlib/src"` from the fragment list of `assert_names_no_source_path`:

```console
$ cargo test -q -p torrust-tracker-test-helpers assertions
assertions::tests::it_should_reject_a_message_holding_a_source_path_fragment --- FAILED
it should reject the message: ()
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s
```

The file was restored from a copy taken before the change, and `cargo test -q -p torrust-tracker-test-helpers` then
passed (10 tests). Status: DONE.

## Automatic checks

`linter all` exits 0 (after shortening the first doc paragraph of the shared function, clippy
`too_long_first_doc_paragraph`). `cargo test` passes for `torrust-tracker-test-helpers`, `-core`, `-axum-http-server`,
`-axum-rest-api-server`, `-udp-server`, `-http-protocol`, `-primitives`, `-configuration` and `-axum-server`.
