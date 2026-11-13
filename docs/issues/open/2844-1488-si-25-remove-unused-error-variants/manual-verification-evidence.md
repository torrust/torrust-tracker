---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2844-1488-si-25-remove-unused-error-variants/ISSUE.md
last-updated-utc: "2026-11-13 22:30"
---

# Verification Evidence - Remove Unused Shutdown Error Variants (#2844)

> **Status**: Verified on 2026-11-13 (M1 to M3).

## Environment

- OS: Linux 7.0.0-43-generic
- Tracker commit/branch: `2844-remove-unused-error-variants` (based on `torrust/develop` at `24230dd01`)

## M1 - Variants are gone

```text
$ git grep -nE "StartupNotification|FailedToStartOrStopServer|UdpError::Launcher|Error::Stop\b" -- '*.rs' '*.md' ':!docs/issues' ':!docs/pr-reviews'
(exit 1)
```

The `enum UdpError` and each package's `enum Error` were read after the edit: they hold only `Bind`, `Listener`
(HTTP and REST) and `Registration`. The `Launcher` structs and `Stopped` states are unrelated and untouched.

## M2 - Startup and shutdown unchanged

Development sqlite config (HTTP, REST, UDP), temporary database. `SIGTERM` sent to the binary PID: exit code 0.
A second run on the same ports bound all five listeners and also exited 0 on `SIGTERM`.

## M3 - Startup errors still reported

With another process holding port 1212: exit code 1, error
`could not bind tracker API listener: Address already in use (os error 98)` (the `Bind` variant).

## Automatic checks

`cargo check --workspace --all-targets` warning-free; `cargo test --workspace`: 4024 passed, 0 failed; `linter all` passed.
