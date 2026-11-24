---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2918-2912-si-6-drop-located-error-and-enforce-the-rule/ISSUE.md
last-updated-utc: "2026-11-24 00:44"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands
actually performed against the artifact; nothing here is invented.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-23. V1 steps 1 to 3 ran shortly before 21:26 UTC (stamped afterwards from `date -u`; the first version of this file gave estimated times); V1 step 4 ran at 22:25:45 UTC.
- Artifact under test: the branch `2918-drop-located-error-and-enforce-the-rule` at the commit
  `feat(source-location-check): [#2918] add the check that errors record no source locations`.
- Operating system / environment: Linux, stable Rust toolchain of the repository.
- Prerequisites and setup performed: none beyond the checkout. The probes edited
  `packages/primitives/src/lib.rs` and were reverted with `git checkout` (no commit contains them).

## Verification Processes

### V1 - A contributor reintroduces a location (M1, T4)

Steps 1 to 3 ran the pre-commit step's command directly,
`cargo run --quiet --package source-location-check`; step 4 ran the real hook through `git commit`.

1. Appended a function that calls `std::panic::Location::caller()` to `packages/primitives/src/lib.rs`:

   ```text
   {"kind":"validation_error","message":"`Location::caller` records the caller's source location; errors do not record source locations (ADR 20261118045443_errors_do_not_record_source_locations)","file":"packages/primitives/src/lib.rs","line":84,"exit_code":1}
   exit 1
   ```

2. `git checkout packages/primitives/src/lib.rs`, then the check again:

   ```text
   {"kind":"checked","message":"no non-test Rust file records a source location (678 files read)","exit_code":0}
   exit 0
   ```

3. Appended a `#[track_caller]` function instead: exit 1, `file` `packages/primitives/src/lib.rs`,
   `line` 83, then reverted.

4. Appended the same `Location::caller()` function to `packages/primitives/src/lib.rs`, ran
   `git add` and `git commit -m "chore: temporary probe, must be rejected by the hook"` at the commit
   `fix(source-location-check): [#2918] mask comments and strings, balance delimiters and track module ancestry`.
   The `pre-commit.sh` hook rejected the commit at 22:25:45 UTC (`date -u` before and after the
   commit command): `"status": "fail"`, `"failed_step": "Checking that errors record no source locations"`,
   and the step's failure tail:

   ```text
   {"kind":"validation_error","message":"`Location::caller` records the caller's source location; errors do not record source locations (ADR 20261118045443_errors_do_not_record_source_locations)","file":"packages/primitives/src/lib.rs","line":84,"exit_code":1}
   ```

   The two steps before it (dictionary, Clippy allows) passed. `git log --oneline -1` still printed
   the `fix(source-location-check)` commit, so no commit was created. Then
   `git restore --staged packages/primitives/src/lib.rs` and `git checkout packages/primitives/src/lib.rs`
   left `git status --short` empty.

5. After review round 5 the hook step runs `cargo run --quiet --package source-location-check -- --staged`
   (it reads the Git index). The same probe was staged and committed at the commit
   `fix(source-location-check): [#2918] check the staged content, resolve path attributes and mask raw C strings`;
   the hook rejected it between 00:44:24 and 00:44:27 UTC (`date -u` before and after the commit
   command) with `"failed_step": "Checking that errors record no source locations"` and the same
   `validation_error` for `packages/primitives/src/lib.rs` line 84. `git log --oneline -1` still printed that
   fix commit, and `git restore --staged` plus `git checkout` of the file left `git status --short` empty.

Result: the check names the file, the line and the rule, passes after the revert, and the hook
rejects a commit that adds a location, before and after the `--staged` change.

### V2 - Test helpers stay allowed (M2)

The tree has `#[track_caller]` helpers in `#[cfg(test)]` modules (`packages/configuration/src/lib.rs`,
`packages/primitives/src/peer.rs`, `packages/axum-server/src/tls.rs`,
`packages/axum-http-server/src/lib.rs`), in files declared `#[cfg(test)] mod ...;`
(`packages/http-protocol/src/test_helpers.rs`, `packages/tracker-core/src/test_helpers.rs`) and in
`tests/` directories. `cargo run -q -p source-location-check` on the tree printed the `checked` line
above and exited 0.
