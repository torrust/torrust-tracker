---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2764-2758-http-health-check-no-stdout/ISSUE.md
last-updated-utc: 2026-11-09 12:12
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

### R0 - Reproduction of the deviation before the change (http_health_check)

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction only; the red regression test, the fix and the final like-for-like recheck are recorded when the issue is implemented)

#### Steps Performed

```text
$ target/debug/http_health_check
exit=1
stdout (0 bytes):
(empty)
stderr (134 bytes):
Usage:   cargo run --bin http_health_check <HEALTH_URL>
Example: cargo run --bin http_health_check http://127.0.0.1:1313/health_check
```

```text
$ target/debug/http_health_check http://127.0.0.1:9/health_check > out 2> err
exit=1
stdout (88 bytes):
Health check ...
ERROR: error sending request for url (http://127.0.0.1:9/health_check)
stderr (0 bytes):
(empty)
```

#### Observed Result

Usage error: plain text on stderr and exit `1` (the contract requires JSON and exit `2`). Unreachable endpoint: plain text on stdout (`Health check ...`, `ERROR: ...`), nothing on stderr, exit `1`. The healthy case was not run here (it needs a running tracker). Running the same probe inside a pseudo-terminal (`script -qec`) gives the same output and `rc=1`.
