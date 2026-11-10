---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2762-2758-package-coverage-check-contract/ISSUE.md
last-updated-utc: 2026-11-10 18:36
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): R0: 2026-11-09 12:11 to 12:13; R1, V1 to V3 and the round-1 rechecks: 2026-11-10 16:40 to 17:35
- Artifact under test: R0: the unchanged tool, built from the tree of the commit `Merge torrust/develop into 2758-cli-output-contract-tools-migration-spec` (develop with the EPIC specs; no code of this issue yet). R1: the commit `test(package-coverage-check): [#2762] failing tests for the CLI output contract`. V1, V2 and the green run: the build of the commit `fix(package-coverage-check): [#2762] follow the CLI output contract`, rebuilt with `cargo build -p package-coverage-check`; the TTY and mutation rechecks below ran on the tree of the commit `fix(package-coverage-check): [#2762] keep pipefail on the summary step and correct the evidence`, whose Rust source equals the first fix commit apart from one comment in `tests/cli.rs`
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

`cargo test -p package-coverage-check --test cli` at the commit `test(package-coverage-check): [#2762] failing tests for the CLI output contract` (tests only, binary unchanged): 5 failed, 0 passed.

```text
it_should_report_an_unknown_subcommand_as_a_usage_error_with_exit_code_two
  assertion `left == right` failed
    left: Some(1)
   right: Some(2)
it_should_report_missing_arguments_as_a_usage_error_with_exit_code_two      (same: Some(1) vs Some(2))
it_should_report_an_unreadable_input_as_a_runtime_error_with_exit_code_one
  the stderr record is JSON: Error("expected value", line: 1, column: 1)
it_should_report_a_missing_subcommand_as_a_usage_error_with_exit_code_two   (failed)
it_should_print_the_summary_markdown_in_the_markdown_field_of_one_json_object (failed)
test result: FAILED. 0 passed; 5 failed
```

The unit test `it_should_map_each_failure_to_its_record_kind_and_exit_code` arrived with the fix (the `Failure` type does not exist before it). Mutation check, recorded in round 1 of the review: changing the `Usage` and `TtyRefusal` exit code in `Failure::exit_code` from `ExitCode::from(2)` to `ExitCode::FAILURE` makes it fail:

```text
tests::it_should_map_each_failure_to_its_record_kind_and_exit_code --- FAILED
  left: ExitCode(unix_exit_status(1))
 right: ExitCode(unix_exit_status(2))
test result: FAILED. 2 passed; 1 failed
```

The mutation was reverted (`git checkout`); the tree equals the fix again.

### G1 - Green tests

`cargo test -p package-coverage-check` after the fix: lib 16 passed, `main.rs` unit tests 3 passed, `tests/cli.rs` 5 passed, `tests/summary.rs` 1 passed, doc-tests 0; 0 failed in every target.

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

$ script -qec 'target/debug/package-coverage-check matrix a b c d; echo exit=$?' /dev/null
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

`actionlint` is not installed here, so it did not run. What ran: `linter all` (which includes `linter yaml`) exited `0` on the round-1 tree, and the workflow parses with Python `yaml.safe_load`. The step runs `summary` through `| jq -r .markdown >> "$GITHUB_STEP_SUMMARY"`. Review finding F1 showed that a step without an explicit `shell:` runs `bash -e {0}` without `pipefail`, so a failing `summary` (empty stdout, exit `1`) would have left the step green with an empty report; the step now sets `shell: bash` (bash with `-e` and `-o pipefail`), so the pipeline fails with the tool's status.

### H1 - Hosted run of the summary step

Job `Package Coverage Regression` (the job that runs the `Write Report-Only Summary` step) in the pull-request run https://github.com/torrust/torrust-tracker/actions/runs/42867272734, on the PR head after review round 1 (the commit `docs(pr-reviews): [#2762] record the PR 2810 review round 1 audit`; it contains the `shell: bash` fix): job conclusion `success`, step `Write Report-Only Summary` `success`. The log shows the step ran `cargo run -p package-coverage-check -- summary ... | jq -r .markdown >> "$GITHUB_STEP_SUMMARY"` with the shell options `-e -o pipefail`. The rendered step summary is not part of the job log or of the run page text, and it was not readable through the API or the browser here, so what it showed was not observed; the byte-identity of the Markdown is established locally in V2. The `Package Coverage (package-coverage-check)` job, which runs `compare` through the new binary, also succeeded in the same run.

### L1 - `linter all`

`linter all` on the round-1 tree: `All linters passed` (exit `0`).
