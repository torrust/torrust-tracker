---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2762-2758-package-coverage-check-contract/ISSUE.md
last-updated-utc: 2026-11-10 12:00
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

### R1 - Red regression tests (before the fix)

`cargo test -p package-coverage-check --test cli` on commit `test(package-coverage-check): [#2762] failing tests for the CLI output contract`: 5 failed, 0 passed (usage error exited `1` instead of `2`, stderr was not JSON, `summary` printed Markdown, not JSON).

### V1 - Usage error and runtime error after the change (M1)

Toolchain: stable Rust, `cargo build -p package-coverage-check`, binary run directly with stdout and stderr redirected to files.

```text
$ target/debug/package-coverage-check bogus > out 2> err
exit=2
stdout: 0 bytes
stderr: {"kind":"usage_error","message":"usage: <matrix <base-workspace> ... | summary <discovery-json> <artifacts-directory>>"}

$ target/debug/package-coverage-check summary /nonexistent <dir> > out 2> err
exit=1
stdout: 0 bytes
stderr: {"kind":"runtime_error","message":"/nonexistent: No such file or directory (os error 2)"}

$ script -qec "target/debug/package-coverage-check matrix a b c d; echo exit=$?" /dev/null
{"kind":"tty_refusal","message":"stdout is a TTY; pipe the output to consume result data"}
exit=2
```

### V2 - Three subcommands and the summary round trip (M2)

```text
$ target/debug/package-coverage-check matrix . . HEAD~1 HEAD
{"matrix":{"include":[{"package":"package-coverage-check",...}]},"unavailable":[]}   exit=0
$ target/debug/package-coverage-check compare pkg base.json src head.json src      (synthetic Codecov reports)
{"package":"pkg","base":{"covered_lines":1,"instrumented_lines":2},"head":{"covered_lines":2,"instrumented_lines":2},"warning":false}   exit=0
$ target/debug/package-coverage-check summary disc.json art > sum.json   exit=0
{"markdown":"## Package Coverage Regression\n\nThis report is informational and does not block merging.\n\nNo directly changed workspace package was selected.\n"}
$ jq -r .markdown sum.json
## Package Coverage Regression

This report is informational and does not block merging.

No directly changed workspace package was selected.
```

The `jq -r .markdown` output is identical to the former plain Markdown output (the report text is unchanged; only the envelope is new). `matrix` and `compare` outputs are unchanged. `matrix` ran on two real revisions (`HEAD~1`, `HEAD`); `compare` and `summary` ran on synthetic inputs because a real `cargo llvm-cov` run is the CI job's work.

### V3 - Workflow syntax (M3)

`actionlint` is not installed here; the workflow was parsed with Python `yaml.safe_load` (valid), and the one change is `| jq -r .markdown` on the `summary` step (GitHub Actions bash runs with `pipefail`, so a failing `cargo run` still fails the step; `jq` is preinstalled on `ubuntu-latest`). The workflow runs on this PR itself.
