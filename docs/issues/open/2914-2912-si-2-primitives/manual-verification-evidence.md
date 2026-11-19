---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2914-2912-si-2-primitives/ISSUE.md
last-updated-utc: "2026-11-19 07:51"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

The scratch crate used by V1 and V2 lived outside the repository (an agent scratchpad). Its two
files are embedded verbatim below, and every output it produced is recorded inline. In the
recorded output, the absolute path of the agent's git worktree is replaced with `<worktree>`; the
rest of each line is verbatim.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-18 18:51 to 18:55; the red re-run and the mutations of the
  review of PR #2955 on 2026-11-19 07:39 to 07:42.
- Artifact under test: `packages/primitives` on branch `2914-primitives`, cut from `develop` at
  `7e8df95ec`; `develop` at `f212ae33b` was merged into it on 2026-11-19, which changed no file of
  `packages/primitives`. V1 and V2 "Before" ran on the unchanged tree of `develop` at
  `7e8df95ec`, before any edit on the branch. "After": the working tree that became the commit
  `fix(primitives): [#2914] remove source locations from peer role and peer id errors`; the scratch
  crate builds only the library, so the tests in that tree do not affect its output.
- Toolchain: stable Rust, `rustc 1.100.0 (0ef1bc386 2026-11-09)`, `cargo 1.100.0
  (3acc79c0c 2026-11-05)`. Formatting with the nightly Rust toolchain, `cargo 1.102.0-nightly
  (f93c676e3 2026-11-15)`.
- Operating system / environment: Linux 7.0.0-43-generic.
- Prerequisites and setup performed: a scratch binary crate `si2-repro` that depends on the
  workspace crate by path, so it sees the library as a consumer of the published crate would.

`Cargo.toml` of the scratch crate:

```toml
[package]
name = "si2-repro"
version = "0.1.0"
edition = "2024"
publish = false

[workspace]

[dependencies]
torrust-tracker-primitives = { path = "<worktree>/packages/primitives" }
```

`src/main.rs` of the scratch crate (`peer::Id` does not implement `Debug`, so the `Id` errors are
taken with `.err().unwrap()` instead of `.unwrap_err()`):

```rust
use torrust_tracker_primitives::peer::{self, PeerRole};

fn main() {
    println!("V1: {}", "x".parse::<PeerRole>().unwrap_err());
    println!("V2: {}", peer::Id::try_from(vec![0; 19]).err().unwrap());
    println!("V2b: {}", peer::Id::try_from(vec![0; 21]).err().unwrap());
}
```

## Analysis

Re-checked against the tree before the first change (T1). `grep -rn 'IdConversionError\|InvalidPeerRole\|NotEnoughBytes\|TooManyBytes' packages src --include=*.rs`
finds the two error types only in `packages/primitives/src/peer.rs`. The `NotEnoughBytes` and
`TooManyBytes` hits in `packages/http-protocol/src/percent_encoding.rs` belong to that crate's own
`PeerIdConversionError`, which has no location. So, as the audit's T2 table says, no production
path prints these two errors; the exposure is to library consumers.

The line numbers differ from the spec's table, which was written against an older tree: the
`IdConversionError` captures are at `peer.rs:395` and `peer.rs:401`, not 391 and 397. The
`ParsePeerRoleError` capture is still at `peer.rs:73`.

Hypothesis: each variant's `Display` interpolates a `location` filled with `Location::caller()`
inside a trait impl reached through `str::parse` or `try_into`, so it always prints the same
line of `peer.rs`.

## Verification Processes

### V1 - Parse an invalid peer role as a library consumer (scenario M1)

- Goal: `"x".parse::<PeerRole>()` returns an error whose text names `x` and holds no file path.
- Initial state: the scratch crate above, built against the artifact under test.
- Status: `DONE`

#### Steps Performed

1. Before the fix, at 18:51:43 UTC: `cargo run -q` in the scratch crate (with a scratch
   `CARGO_TARGET_DIR`).
2. After the fix, at 18:54:58 UTC: the same command, unchanged scratch crate.

#### Observed Result

Before:

```text
V1: invalid param x in <worktree>/packages/primitives/src/peer.rs:73:27
```

After:

```text
V1: invalid peer role "x", expected "seeder" or "leecher"
```

#### Conclusion

Met. Before the fix the text ended in a source path and line of the library; after it the text
names the rejected input `x` and the accepted names, and holds no path.

### V2 - Convert a short (and a long) peer id (scenario M2)

- Goal: `peer::Id::try_from(vec![0; 19])` returns an error whose text states the byte count and
  holds no file path. The 21-byte case (`V2b`) checks the other variant.
- Initial state: same scratch crate, same runs as V1.
- Status: `DONE`

#### Observed Result

Before (18:51:43 UTC):

```text
V2: not enough bytes for peer id: got 19 bytes, expected 20 <worktree>/packages/primitives/src/peer.rs:395:27
V2b: too many bytes for peer id: got 21 bytes, expected 20 <worktree>/packages/primitives/src/peer.rs:401:27
```

After (18:54:58 UTC):

```text
V2: not enough bytes for peer id: got 19 bytes, expected 20
V2b: too many bytes for peer id: got 21 bytes, expected 20
```

#### Conclusion

Met. The byte counts stay; the source path is gone.

### V3 - Scenario M3 of the audit #2804 (`"x".parse::<PeerRole>()` through `str::parse`)

- Goal: scenario M3 of `docs/issues/closed/2804-location-caller-track-caller-audit/ISSUE.md`,
  handed over to this issue when that spec was closed: run the regression test through
  `str::parse` against the old and the fixed code; old text holds a `peer.rs` path, the fixed text
  holds no path and echoes the input.
- Status: `DONE`, from runs already recorded in this file; no new run was made for V3.

The regression test
`peer::test::peer_role::it_should_describe_a_name_that_is_not_a_role_by_the_rejected_text_and_no_source_path`
calls `"uploader".parse::<PeerRole>()`. Against the old code (the tests-only commit) it failed with
`"invalid param uploader in packages/primitives/src/peer.rs:73:27"` (Red, below); against the fixed
code it passed (Green, below). The scenario's literal input `"x"` is V1: before the fix the text
ended in `peer.rs:73:27`, after it the text is `invalid peer role "x", expected "seeder" or
"leecher"`.

Conclusion: met. The input is echoed and no path remains.

## Regression Tests

### Boundary

Unit tests in `packages/primitives/src/peer.rs`, at the `FromStr for PeerRole` and
`TryFrom<Vec<u8>> for Id` seams, as the spec's Regression Test Strategy says. They call through
`str::parse::<PeerRole>()` and `Vec<u8>::try_into()`, the shapes callers use. No server prints
these errors, so these tests are the whole regression suite.

- `peer::test::peer_role::it_should_describe_a_name_that_is_not_a_role_by_the_rejected_text_and_no_source_path`
- `peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_less_than_20_bytes_by_its_length_and_no_source_path`
- `peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_more_than_20_bytes_by_its_length_and_no_source_path`

The helper `assert_names_no_source_path` forbids `.rs:`, `packages/`, `packages\`, `src/`, `src\`,
`.cargo/registry` and `rustlib/src`.

The rejected peer role in the test is `uploader`, not `peer`: a first draft used `peer`, and its
"names the rejected text" assertion passed against the bug only because the path `peer.rs`
contains `peer`. With `uploader` the assertion can only pass on the input.

### Red (before the fix)

Command: `cargo test -q -p torrust-tracker-primitives --lib -- no_source_path` (stable Rust, as
above).

**First run, 2026-11-18 18:52:30 UTC, on the working tree before `cargo +nightly fmt` and before
the commit.** This run is not of the committed bytes: in it the helper's fragment array was on one
line, so the panics name `peer.rs:680:13`. `cargo +nightly fmt` then split the array, and the
commit `test(primitives): [#2914] add red tests that peer role and peer id errors name no source path`
was made at 18:52:43 UTC (review finding F2 of PR #2955 found the mismatch). Output, with thread ids
removed:

```text
running 3 tests
peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_less_than_20_bytes_by_its_length_and_no_source_path --- FAILED
peer::test::peer_role::it_should_describe_a_name_that_is_not_a_role_by_the_rejected_text_and_no_source_path --- FAILED
peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_more_than_20_bytes_by_its_length_and_no_source_path --- FAILED
failures:
---- peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_less_than_20_bytes_by_its_length_and_no_source_path stdout ----
thread '...' panicked at packages/primitives/src/peer.rs:680:13:
the error message must not hold a source path, but it contains ".rs:": "not enough bytes for peer id: got 19 bytes, expected 20 packages/primitives/src/peer.rs:395:27"
---- peer::test::peer_role::it_should_describe_a_name_that_is_not_a_role_by_the_rejected_text_and_no_source_path stdout ----
thread '...' panicked at packages/primitives/src/peer.rs:680:13:
the error message must not hold a source path, but it contains ".rs:": "invalid param uploader in packages/primitives/src/peer.rs:73:27"
---- peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_more_than_20_bytes_by_its_length_and_no_source_path stdout ----
thread '...' panicked at packages/primitives/src/peer.rs:680:13:
the error message must not hold a source path, but it contains ".rs:": "too many bytes for peer id: got 21 bytes, expected 20 packages/primitives/src/peer.rs:401:27"
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 125 filtered out; finished in 0.00s
```

**Re-run, 2026-11-19 07:39:10.863 to 07:39:11.867 UTC, at the commit itself.** A detached
temporary worktree checked out
`test(primitives): [#2914] add red tests that peer role and peer id errors name no source path`
(a local rebase of that commit onto `7a4c16eaf`, later replaced by a merge of `develop`; `git diff --quiet` shows its `packages/primitives` byte-identical to the commit on the PR branch) and
ran the same command, with the branch's `target` directory. Output, with thread ids removed:

```text
running 3 tests
peer::test::peer_role::it_should_describe_a_name_that_is_not_a_role_by_the_rejected_text_and_no_source_path --- FAILED
peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_more_than_20_bytes_by_its_length_and_no_source_path --- FAILED
peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_less_than_20_bytes_by_its_length_and_no_source_path --- FAILED
failures:
---- peer::test::peer_role::it_should_describe_a_name_that_is_not_a_role_by_the_rejected_text_and_no_source_path stdout ----
thread '...' panicked at packages/primitives/src/peer.rs:688:13:
the error message must not hold a source path, but it contains ".rs:": "invalid param uploader in packages/primitives/src/peer.rs:73:27"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
---- peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_more_than_20_bytes_by_its_length_and_no_source_path stdout ----
thread '...' panicked at packages/primitives/src/peer.rs:688:13:
the error message must not hold a source path, but it contains ".rs:": "too many bytes for peer id: got 21 bytes, expected 20 packages/primitives/src/peer.rs:401:27"
---- peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_less_than_20_bytes_by_its_length_and_no_source_path stdout ----
thread '...' panicked at packages/primitives/src/peer.rs:688:13:
the error message must not hold a source path, but it contains ".rs:": "not enough bytes for peer id: got 19 bytes, expected 20 packages/primitives/src/peer.rs:395:27"
failures:
    peer::test::peer_role::it_should_describe_a_name_that_is_not_a_role_by_the_rejected_text_and_no_source_path
    peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_less_than_20_bytes_by_its_length_and_no_source_path
    peer::test::torrent_peer_id::it_should_describe_a_byte_vector_with_more_than_20_bytes_by_its_length_and_no_source_path
test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 125 filtered out; finished in 0.00s
error: test failed, to rerun pass `-p torrust-tracker-primitives --lib`
```

The three tests fail on the committed bytes; the panics name the helper's `assert!` line, 688.

### Green (after the fix)

Same command at 18:54:52 UTC, on the fixed working tree before `cargo +nightly fmt` and the commit
(formatting changed no test result; the pre-push hook ran all tests on the committed bytes):

```text
running 3 tests
...
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 125 filtered out; finished in 0.00s
```

`cargo test -q -p torrust-tracker-primitives` then passed: 128 unit tests and 8 doctests.
`grep -rn 'Location' packages/primitives/src` finds nothing.

### Prose-first Arrange-Act-Assert review

Each test was written with temporary prose and compared with the code:

- Peer role. Arrange: a text that is not a role name and appears nowhere in a source path
  (`uploader`). Act: parse it as a `PeerRole`, as callers do, and render the error. Assert: the
  message names the rejected text and holds no source path. The code says each part directly;
  the Arrange is the inline literal, so no prose was kept.
- Peer id, 19 and 21 bytes. Arrange: a byte vector one shorter (or longer) than a peer id. Act:
  convert it with `try_into` and render the error. Assert: the message states the received and
  the expected byte counts and holds no source path. The conversion and rendering mechanics live
  in the single-use helper `conversion_error_message`, which names the action ("the message of the
  conversion error") and hides only the `Result` type annotation and the unwrap; the input length
  stays visible in each test. No prose was kept.
- The forbidden-fragment list sits in one named helper, `assert_names_no_source_path`, whose doc
  comment states what counts as a source path; the assertion message prints the fragment found
  and the whole message, so a failure is readable without opening the file.

### Review fixes of PR #2955 (tests only)

Both commits below change only test code; production code is unchanged since the fix commit, so
V1 and V2 "After" still describe the final library.

- **F5**, `test(primitives): [#2914] report source-path assertion failures at the test line`:
  `assert_names_no_source_path` is `#[track_caller]`. Mutation, at 07:40:15.494 to 07:40:16.401
  UTC on that commit: the `NotEnoughBytes` format temporarily became
  `"not enough bytes for peer id: {message} src/x.rs:1"` (unstaged), and
  `cargo test -q -p torrust-tracker-primitives --lib -- no_source_path` failed one test with
  `panicked at packages/primitives/src/peer.rs:1449:13`, the `assert_names_no_source_path(&message);`
  line of the 19-byte test, not the helper's line. `git checkout -- packages/primitives/src/peer.rs`
  restored the file, and `git diff --quiet HEAD -- packages/primitives/src/peer.rs` exited 0 at
  07:40:16.414.
- **F4**, `test(primitives): [#2914] pin the full message of a rejected peer role`: the new test
  `peer::test::peer_role::it_should_name_the_accepted_roles_when_it_rejects_a_name` compares the
  whole `Display` of `"x".parse::<PeerRole>()` with
  `invalid peer role "x", expected "seeder" or "leecher"`. Mutation, at 07:41:32.980 to
  07:41:33.311 UTC on that commit: the `InvalidPeerRole` format temporarily became
  `"invalid param {raw_param}"` (unstaged), and `cargo test -q -p torrust-tracker-primitives --lib
  -- peer_role` failed only the new test (8 passed, 1 failed) with `left: "invalid param x"`;
  the three no-source-path tests stayed green, as the finding said. The restore was checked as for
  F5 (`git diff --quiet` exited 0 at 07:41:33.323). An earlier attempt at 07:41:25 restored with
  `git checkout` before the test was committed, which also removed the uncommitted test; its
  timestamps were inconsistent too, so it is discarded and not used here.
- Afterwards `cargo test -q -p torrust-tracker-primitives` passed: 129 unit tests and 8 doctests.

## Failures and Follow-up

None. The first-draft weakness of the peer-role test input is described under Boundary; it was
fixed before the red run recorded above.
