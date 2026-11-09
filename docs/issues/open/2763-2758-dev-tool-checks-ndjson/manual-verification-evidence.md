---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2763-2758-dev-tool-checks-ndjson/ISSUE.md
last-updated-utc: 2026-11-09 12:12
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-09 12:11 to 12:13
- Artifact under test: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet)
- Operating system / environment: Linux; stable Rust toolchain 1.99.1 (`rustc 1.99.1`, `cargo 1.99.1`); binaries built with `cargo build` (debug) and run directly from `target/debug/`, stdout and stderr redirected to files (stdout is not a terminal)
- Prerequisites and setup performed: none beyond the build

## Verification Processes

### R0 - Reproduction of the deviation before the change (agent-review-report-contract and frontmatter-schema)

- Goal: show the current behavior that the issue changes, as `fix-bug` requires, before the code changes.
- Status: `DONE` (reproduction only; the red regression test, the fix and the final like-for-like recheck are recorded when the issue is implemented)

#### Steps Performed

```text
$ target/debug/agent-review-report-contract > out 2> err
exit=0
stdout (43 bytes):
Agent review report contract check passed.
stderr (0 bytes):
(empty)
```

```text
$ target/debug/frontmatter-schema > out 2> err
exit=2
stdout (0 bytes):
(empty)
stderr (83 bytes):
frontmatter-schema: usage: frontmatter-schema <generate|check> [--artifact <path>]
```

#### Observed Result

`agent-review-report-contract` writes a plain-text success line to stdout (a `no-stdout-result` tool must write nothing there). `frontmatter-schema` writes a plain-text usage message to stderr (exit `2` already right, the format is not NDJSON). The failure path of `agent-review-report-contract` was not run (it needs a deliberately broken tree); its plain-text failure lines are read from the source (`src/main.rs`, the `writeln!(stderr, "{failure}")` call).
