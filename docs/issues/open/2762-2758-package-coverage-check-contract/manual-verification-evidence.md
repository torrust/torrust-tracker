---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2762-2758-package-coverage-check-contract/ISSUE.md
last-updated-utc: 2026-11-09 12:12
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

### R0 - Reproduction of the deviation before the change (package-coverage-check)

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction only; the red regression test, the fix and the final like-for-like recheck are recorded when the issue is implemented)

#### Steps Performed

```text
$ target/debug/package-coverage-check bogus > out 2> err
exit=1
stdout (0 bytes):
(empty)
stderr (215 bytes):
package-coverage-check: usage: <matrix <base-workspace> <head-workspace> <base-sha> <head-sha> | compare <package> <base-report> <base-src> <head-report> <head-src> | summary <discovery-json> <artifacts-directory>>
```

#### Observed Result

A usage error is plain text on stderr (not JSON) and the exit code is `1`, not `2`. The TTY refusal and the Markdown `summary` were not run here: the TTY case needs a pseudo-terminal and `summary` needs a discovery file and result folder; both are exercised when the issue is implemented. The deviation of the error channel and exit code is reproduced.
