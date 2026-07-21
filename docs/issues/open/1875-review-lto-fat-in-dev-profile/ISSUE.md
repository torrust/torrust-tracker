---
doc-type: issue
issue-type: task
status: in-review
priority: p2
github-issue: 1875
spec-path: docs/issues/open/1875-review-lto-fat-in-dev-profile/ISSUE.md
branch: "1875-review-lto-fat-in-dev-profile"
related-pr: null
last-updated-utc: 2026-07-21 10:18
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - Cargo.toml
    - Containerfile
    - docs/issues/open/1875-review-lto-fat-in-dev-profile/research.md
    - docs/skills/semantic-skill-link-convention.md
---

# Issue #1875 - Review and fix `lto = "fat"` in `[profile.dev]`

## Goal

Optimize development builds for compilation speed and production builds for execution speed.
Remove the obsolete `lto = "fat"` setting from `[profile.dev]`, allowing Cargo's development-profile default (`lto = false`) to apply. Keep `lto = "fat"` in `[profile.release]` for production binary optimization.

## Background

Commit `3c715fbb` changed `[profile.dev]` from `lto = "thin"` to `lto = "fat"` as a workaround for a `failed to load bitcode` error involving Criterion in a Docker build with Rust 1.79/1.81-nightly in mid-2024.

The investigation is recorded in [research.md](research.md). It found an important discrepancy: the recorded failing command used `--release`, which selects `[profile.release]`; changing `[profile.dev]` could not have directly affected that invocation. The release profile already used fat LTO before the workaround. Therefore, this issue removes the unsupported development-profile workaround while retaining the independently appropriate release setting.

## Scope

### In Scope

- Remove `lto = "fat"` from `[profile.dev]` in `Cargo.toml`.
- Preserve `lto = "fat"` in `[profile.release]`.
- Verify development-profile tests and the Docker debug image build.
- Verify the Docker release image build continues to succeed.
- Record evidence in this issue spec and its research document.

### Out of Scope

- Changing `[profile.release]` LTO settings.
- Introducing a non-default development LTO setting such as `"thin"` or `"off"`.
- Restructuring the `Containerfile` beyond what is necessary to verify the change.
- Reproducing the historic Rust 1.79/1.81-nightly failure.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                    | Notes / Expected Output                                                                                                                          |
| --- | ------ | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Collect user decision and research current LTO behavior | User selected Cargo's default development LTO setting. Findings are in [research.md](research.md).                                               |
| T2  | DONE   | Remove `lto = "fat"` from `[profile.dev]`               | Removed the key; Cargo uses its default `lto = false` development-profile behavior.                                                              |
| T3  | DONE   | Run the full local test suite                           | Passed: `cargo test --tests --benches --examples --workspace --all-targets --all-features`.                                                      |
| T4  | DONE   | Build the Docker debug image                            | Passed: `docker build --target debug --tag torrust-tracker:debug --file Containerfile .` completed in 120.9 seconds without a bitcode error.     |
| T5  | DONE   | Build the Docker release image                          | Passed: `docker build --target release --tag torrust-tracker:release --file Containerfile .` completed in 214.4 seconds without a bitcode error. |
| T6  | DONE   | Run pre-commit checks                                   | Passed: `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh` exited 0.                                                   |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Local implementation branch created
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-06-03 00:00 UTC - GitHub Copilot - Spec drafted after investigating git history for `lto = "fat"` in `[profile.dev]`; root cause traced to commit `3c715fbb`.
- 2026-07-21 10:18 UTC - User - Confirmed the policy: prioritize development compilation speed and production execution speed. Approved the folder format with `ISSUE.md` as the normal-issue specification file.
- 2026-07-21 10:18 UTC - GitHub Copilot - Created branch `1875-review-lto-fat-in-dev-profile`, converted the specification to folder format, and recorded research findings.
- 2026-07-21 10:18 UTC - GitHub Copilot - Removed development-profile fat LTO. The full local test suite, Docker debug and release image builds, and pre-commit checks all passed.
- 2026-07-21 10:18 UTC - GitHub Copilot - Removed the empty continued line in `Containerfile` that produced Docker's `NoEmptyContinuation` warning. `docker build --target recipe --file Containerfile .` passed without the warning.

## Acceptance Criteria

- [x] AC1: `[profile.dev]` in `Cargo.toml` has no explicit `lto` setting and therefore uses Cargo's default `lto = false` behavior.
- [x] AC2: `[profile.release]` retains `lto = "fat"`.
- [x] AC3: `cargo test --tests --benches --examples --workspace --all-targets --all-features` exits with code 0.
- [x] AC4: Docker debug build (`docker build --target debug`) completes without a `failed to load bitcode` error.
- [x] AC5: Docker release build (`docker build --target release`) completes without a `failed to load bitcode` error.
- [x] AC6: `linter all` exits with code 0.
- [x] AC7: Manual verification scenarios are executed and documented (status + evidence).
- [x] AC8: Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --tests --benches --examples --workspace --all-targets --all-features`
- `./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                        | Command/Steps                                                                        | Expected Result                        | Status | Evidence                                                                                     |
| --- | ------------------------------- | ------------------------------------------------------------------------------------ | -------------------------------------- | ------ | -------------------------------------------------------------------------------------------- |
| M1  | Local development-profile tests | `cargo test --tests --benches --examples --workspace --all-targets --all-features`   | All tests pass with no bitcode error.  | DONE   | Passed.                                                                                      |
| M2  | Docker debug image              | `docker build --target debug --tag torrust-tracker:debug --file Containerfile .`     | Build completes with no bitcode error. | DONE   | Passed in 120.9 seconds. The unrelated `NoEmptyContinuation` warning was subsequently fixed. |
| M3  | Docker release image            | `docker build --target release --tag torrust-tracker:release --file Containerfile .` | Build completes with no bitcode error. | DONE   | Passed in 214.4 seconds. The unrelated `NoEmptyContinuation` warning was subsequently fixed. |

## Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                             |
| ----- | ---------------------- | ------------------------------------------------------------------------------------ |
| AC1   | DONE                   | `[profile.dev]` contains only `debug = 1` and `opt-level = 1`; no `lto` key remains. |
| AC2   | DONE                   | `[profile.release]` still contains `lto = "fat"`.                                    |
| AC3   | DONE                   | Full command passed.                                                                 |
| AC4   | DONE                   | Docker debug image build passed.                                                     |
| AC5   | DONE                   | Docker release image build passed.                                                   |
| AC6   | DONE                   | Pre-commit's `linter all` step passed.                                               |
| AC7   | DONE                   | M1 through M3 passed and are recorded above.                                         |
| AC8   | DONE                   | This table was reviewed and updated after all verification completed.                |

## References

- Commit `3c715fbb` — original workaround: "fix: [#898] docker build error: failed to load bitcode of module criterion"
- [Cargo reference — profiles](https://doc.rust-lang.org/cargo/reference/profiles.html#lto)
- [Rustc codegen option — LTO](https://doc.rust-lang.org/rustc/codegen-options/index.html#lto)
