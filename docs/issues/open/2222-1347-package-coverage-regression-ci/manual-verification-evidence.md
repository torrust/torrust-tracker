---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2222-1347-package-coverage-regression-ci/ISSUE.md
last-updated-utc: 2026-09-22 08:43
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of Issue #2222's completed report-only coverage
comparison. No commands, output, logs, or results are recorded until the corresponding scenario
is actually executed.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-22 06:10.
- Artifact under test: Local implementation then named `package-coverage-regression` (renamed
  later to `package-coverage-check`) and `.github/workflows/generate_coverage_pr.yaml`.
- Operating system / environment: Linux local development workspace.
- Prerequisites and setup performed: Nightly Rust formatter and the repository pre-commit gate.

## Local Automated Evidence

- `cargo clippy -p package-coverage-regression --all-targets --all-features -- -D warnings` passed.
- `cargo test -p package-coverage-regression` passed with 15 tests covering report filtering,
  tolerance comparison, base/head pairing, new/removed/moved/unchanged packages, and Git rename
  and deletion status parsing.
- `cargo run -p package-coverage-regression -- matrix "$PWD" "$PWD" HEAD~3 HEAD` selected only
  `package-coverage-regression` and emitted an empty unavailable list.
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json` passed.

The commands and package name above are historical evidence captured before the rename. Reproduce
them by checking out the recorded commits; use `package-coverage-check` for current commands.

Hosted GitHub Actions runtime, artifact transfer, matrix rendering, and fork execution are not
represented by this local evidence. Their scenarios remain pending below.

## Verification Processes

### M1 Base/Head Double-Build Proof of Concept

- Goal: Verify a controlled changed-package comparison at the pull request base and head
  revisions without a persisted baseline artifact.
- Initial state: `88822c5c` as base and `aa026584` as head, with the
  `package-coverage-regression` package present in both revisions.
- Status: `DONE` (local proof only)

#### Steps Performed

1. Created a detached base checkout with `git worktree add --detach
  .tmp/package-coverage-base 88822c5c`.
2. Ran `cargo llvm-cov -p package-coverage-regression --all-features --codecov`
  at the base and head revisions with `CARGO_INCREMENTAL=0`,
  `RUSTFLAGS=-Cinstrument-coverage`, separate `CARGO_TARGET_DIR` values, and
  separate Codecov JSON output paths.
3. Ran the repository helper's `compare` command over each revision's JSON report and `src/`
  directory.

#### Observed Result

- The base build passed all 15 package tests and took 2.98 seconds.
- The head build passed all 15 package tests and took 2.98 seconds.
- The helper emitted
  `{"package":"package-coverage-regression","base":{"covered_lines":290,"instrumented_lines":461},"head":{"covered_lines":290,"instrumented_lines":461},"warning":false}`.

#### Conclusion

The local double build used isolated artifacts, retained exact source-only counts, and reported no
warning for equal coverage. It does not establish GitHub-hosted-runner runtime, artifact-download
behavior, rendered job-summary output, or fork safety; those remain M2-M5 rollout checks.

### M2 Dynamic Changed-Package Matrix

- Goal: Verify that the workflow matrix includes only directly changed comparable packages while
  the final report job remains present for every run.
- Initial state: Pending implementation.
- Status: `TODO`

#### Steps Performed

Pending implementation.

#### Observed Result

Pending implementation.

#### Conclusion

Pending implementation.

### M3 Equal, Improved, and Reduced Coverage

- Goal: Verify ratio calculation reports equal, increased, and decreased coverage; warn only for a
  decrease greater than five percentage points without blocking a merge.
- Initial state: Pending implementation.
- Status: `TODO`

#### Steps Performed

Pending implementation.

#### Observed Result

Pending implementation.

#### Conclusion

Pending implementation.

### M4 Exceptional Package Outcomes

- Goal: Verify documentation/workspace-config-only changes and new, deleted, or unmatched renamed
  package outcomes.
- Initial state: Fork PR [#2293](https://github.com/torrust/torrust-tracker/pull/2293) added
  `package-coverage-check`, which is absent from its `develop` base revision.
- Status: `DONE` (new-package outcome)

#### Steps Performed

1. Opened fork PR #2293 against `torrust/torrust-tracker:develop`.
2. Observed the successful
   [Generate Coverage Report (PR) run](https://github.com/torrust/torrust-tracker/actions/runs/35705895905).

#### Observed Result

- `Discover Package Coverage` completed successfully in 21 seconds.
- Discovery emitted `{"matrix":{"include":[]},"unavailable":[{"package":"package-coverage-check","outcome":"new package"}]}`.
- The matrix job was skipped, and the always-present `Package Coverage Regression` job completed
  successfully in 60 seconds.

#### Conclusion

The workflow does not fabricate a base/head comparison for a new package and still produces the
report-only summary. Documentation-only, removed-package, and unmatched-rename outcomes remain
unverified in GitHub Actions.

### M5 Fork Pull Request

- Goal: Verify the report-only check stays in the unprivileged `pull_request` workflow and the
  trusted uploader remains artifact-only.
- Initial state: Fork PR [#2293](https://github.com/torrust/torrust-tracker/pull/2293) from
  `josecelano:2222-1347-package-coverage-regression-ci` to `torrust:develop`.
- Status: `DONE`

#### Steps Performed

1. Opened the fork PR and inspected its successful coverage workflow run.
2. Reviewed the `Package Coverage Regression` job log for its checkout, toolchain, and summary
   command steps.

#### Observed Result

- The workflow ran successfully from the fork PR without accessing the trusted uploader workflow.
- The summary job checked out the immutable pull request head revision, configured the nightly
  toolchain, and ran `package-coverage-check summary` with GitHub-provided discovery JSON and
  downloaded-artifact paths.

#### Conclusion

The report-only coverage check executes in the unprivileged fork pull-request workflow. This run
did not alter or execute the trusted `workflow_run` uploader, preserving its artifact-only role.

## Failures and Follow-up

None recorded.
