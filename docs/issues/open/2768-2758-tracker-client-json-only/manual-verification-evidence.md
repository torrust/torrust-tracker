---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2768-2758-tracker-client-json-only/ISSUE.md
last-updated-utc: 2026-11-09 12:39
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

### R0 - Reproduction of the deviation before the change (tracker_client)

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction only; the red regression test, the fix and the final like-for-like recheck are recorded when the issue is implemented)

#### Steps Performed

```text
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":[],"health_checks":[]}' target/debug/tracker_client check > out 2> err
exit=0
stdout (3 bytes):
[]
stderr (0 bytes):
(empty)
```

```text
$ TORRUST_CHECKER_CONFIG='{"udp_trackers":[],"http_trackers":[],"health_checks":[]}' script -qec 'target/debug/tracker_client check; rc=$?; echo "rc=$rc"; exit "$rc"' /dev/null
[]
rc=0
script exit status: 0
```

#### Observed Result

`check` writes a top-level JSON array, not a single JSON object (Copilot's and da2ce7's finding), and with stdout on a pseudo-terminal it runs and exits `0` instead of refusing with exit `2`. The plain-text `tracing` on stdout and the direct prints of `http`/`udp` were not run here (they need a tracker); they are read from `unified/app.rs` (`tracing_subscriber::fmt()` writes to stdout) and the `print_*` allows in the console layer.
