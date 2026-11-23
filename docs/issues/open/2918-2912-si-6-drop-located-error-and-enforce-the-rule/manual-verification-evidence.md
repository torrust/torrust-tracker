---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2918-2912-si-6-drop-located-error-and-enforce-the-rule/ISSUE.md
last-updated-utc: "2026-11-23 21:40"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands
actually performed against the artifact; nothing here is invented.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-23, about 21:35.
- Artifact under test: the branch `2918-drop-located-error-and-enforce-the-rule` at the commit
  `feat(source-location-check): [#2918] add the check that errors record no source locations`.
- Operating system / environment: Linux, stable Rust toolchain of the repository.
- Prerequisites and setup performed: none beyond the checkout. The probes edited
  `packages/primitives/src/lib.rs` and were reverted with `git checkout` (no commit contains them).

## Verification Processes

### V1 - A contributor reintroduces a location (M1, T4)

The pre-commit step is `cargo run --quiet --package source-location-check`; the probe ran that
command directly, not the whole hook.

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

Result: the check names the file, the line and the rule, and passes after the revert.

### V2 - Test helpers stay allowed (M2)

The tree has `#[track_caller]` helpers in `#[cfg(test)]` modules (`packages/configuration/src/lib.rs`,
`packages/primitives/src/peer.rs`, `packages/axum-server/src/tls.rs`,
`packages/axum-http-server/src/lib.rs`), in files declared `#[cfg(test)] mod ...;`
(`packages/http-protocol/src/test_helpers.rs`, `packages/tracker-core/src/test_helpers.rs`) and in
`tests/` directories. `cargo run -q -p source-location-check` on the tree printed the `checked` line
above and exited 0.
