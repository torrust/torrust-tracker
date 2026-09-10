---
doc-type: issue
issue-type: bug
status: open
priority: p1
epic: null
github-issue: 2179
spec-path: docs/issues/open/2179-docker-e2e-job-package-flag/ISSUE.md
branch: "2179-docker-e2e-job-package-flag-spec"
related-pr: null
last-updated-utc: 2026-09-10 08:15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/workflows/testing.yaml
    - .github/workflows/container.yaml
    - .yamllint-ci.yml
    - packages/e2e-tools/Cargo.toml
    - docs/issues/closed/1854-1840-workflow-performance-container-test-gating/ISSUE.md
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2179 - Docker E2E job in testing.yaml fails on every feature-branch push: cargo run lacks the e2e-tools package flag

## Goal

Restore the `Docker E2E` job in `.github/workflows/testing.yaml` to a state where its four E2E runner steps actually execute, by naming the package that owns the binaries on each `cargo run` invocation exactly as `.github/workflows/container.yaml` already does, so that feature-branch pushes and pull requests targeting branches other than `develop` and `main` regain the E2E coverage the job exists to provide.

## Background

The `docker-e2e` job in `.github/workflows/testing.yaml` (lines 124-197) exists to give E2E coverage to the events that `.github/workflows/container.yaml` does not cover. Its `if:` guard (lines 133-139) skips the job on pull requests whose base is `develop` or `main`, and on pushes to `refs/heads/develop`, `refs/heads/main`, and `refs/heads/releases/*`, because `container.yaml` builds the same image and runs the same E2E tests for exactly those events. The job's comment (lines 125-129) states this and cites issue #1854. What remains after the guard is precisely the feature-branch case: a push to any in-repo branch that is not `develop`, `main`, or a release branch, and any pull request targeting some other branch.

That job cannot reach its E2E tests. Its four runner steps invoke the binaries without naming the package that owns them:

- line 185, step `run-tracker-e2e-tests`: `cargo run --bin e2e_tests_runner -- ...`
- line 189, step `run-qbittorrent-e2e-test-sqlite3`: `cargo run --bin qbittorrent_e2e_runner -- ...`
- line 193, step `run-qbittorrent-e2e-test-mysql`: `cargo run --bin qbittorrent_e2e_runner -- ...`
- line 197, step `run-qbittorrent-e2e-test-postgresql`: `cargo run --bin qbittorrent_e2e_runner -- ...`

Both binaries live in the `torrust-tracker-e2e-tools` package, at `packages/e2e-tools/src/bin/e2e_tests_runner.rs` and `packages/e2e-tools/src/bin/qbittorrent_e2e_runner.rs`. The workspace root is itself a package: the root `Cargo.toml` opens with a `[package]` section declaring `name = "torrust-tracker"` and `default-run = "torrust-tracker"`, and the `[workspace]` table sets `members` without setting `default-members`. Run from the workspace root without `-p`, `cargo run --bin <name>` therefore resolves the target against the root package alone, where neither binary exists, and the step fails before any tracker image is exercised. The failure reported from CI logs is `error: no bin target named 'e2e_tests_runner' in default-run packages`, with exit code 101.

The same four invocations in `.github/workflows/container.yaml` (lines 117, 123, 127, 131) carry `-p torrust-tracker-e2e-tools`, which is why that workflow is green while this one is not.

The divergence has a precise origin, and it is not the guard. Commit `c1b6c5466` ("feat(ci): eliminate duplicate E2E tests and add test binary landscape analysis", 2026-06-03) added only the `if:` guard and its explanatory comment to `testing.yaml`, twelve added lines and no deletions; the runner steps already existed there and predate it. The break arrived the next day in commit `c47173f53` ("ci(container): slim nextest archive by extracting e2e and benchmark packages", 2026-06-04), which moved `src/bin/e2e_tests_runner.rs`, `src/bin/qbittorrent_e2e_runner.rs`, and `src/bin/profiling.rs` into the new `packages/e2e-tools` package as pure renames and touched neither workflow. At that moment both workflows were broken in the same way. Commit `2d1ce246b` ("ci(container): address Copilot review issues after T13/T14 extraction", 2026-06-04) then repaired `container.yaml` alone: its own message records fixing the four `cargo run --bin` invocations to include `-p torrust-tracker-e2e-tools` "so Cargo resolves the binaries after they were extracted out of `src/bin/` into the new e2e-tools package", and wrapping the long `e2e_tests_runner` step in a YAML `>-` block scalar to stay inside the yamllint line-length limit. The identical four invocations in `testing.yaml` were not part of that change and have carried the defect ever since.

What the guard did contribute is the silence. Because `docker-e2e` never runs on pushes to `develop` or on pull requests targeting `develop`, upstream CI has never executed the broken steps, so `develop` has stayed green while the job has failed on every feature-branch push since June 2026. The failure surfaces only where nobody routinely looks: it has been reported on the dependency-update pull requests #2055 and #2106, whose branch pushes do trigger the job, and it is observed directly on the `da2ce7/torrust-tracker` fork, most recently in the `Testing` run for the push of `2175-merge-tool-symlink-exceptions` at `4bff469e` on 2026-09-10, where the `Docker E2E` job builds the tracker image successfully and then fails at step `Run E2E Tests`, leaving the three qBittorrent steps skipped ([run 34444943476](https://github.com/da2ce7/torrust-tracker/actions/runs/34444943476/job/102767609212)). The #2055 and #2106 observations are carried here as reported evidence and were not re-checked.

One detail of the repair is load-bearing rather than cosmetic. `.yamllint-ci.yml` sets `line-length: max: 200`, and `linter all` runs yamllint over the workflow files. Line 185 is currently 184 characters; adding `-p torrust-tracker-e2e-tools` together with the space separating it from the following argument costs 26 more characters and would take it to 210, over the limit. The other three lines are 153, 151, and 156 characters and land at 179, 177, and 182. The first step must therefore be wrapped in a `>-` block scalar, which is exactly the shape `container.yaml` already uses for the same step and for the same reason.

## Scope

### In Scope

- Add `-p torrust-tracker-e2e-tools` to the four `cargo run` invocations in the `docker-e2e` job of `.github/workflows/testing.yaml`, at lines 185, 189, 193, and 197, keeping every other argument of each step byte-for-byte as it is today.
- Wrap the `run-tracker-e2e-tests` step's command in a YAML `>-` block scalar so the added flag does not push the line past the 200-character yamllint limit, matching the form `container.yaml` uses for the same step.
- Extend the job's existing comment with one sentence recording why the package flag is required: the workspace root is itself a package, so `--bin` alone resolves only against the root package's targets.
- Verify the fix by pushing the branch to a fork or another in-repo feature branch, where the guard admits the job, and recording the passing run URL in this specification.

### Out of Scope

- Changing when the `docker-e2e` job runs. The `if:` guard and the trigger design stay exactly as they are.
- Deduplicating `docker-e2e` against the E2E steps in `container.yaml`, or deciding which of the two workflows should own E2E coverage.
- Any change to the E2E runners themselves, to `packages/e2e-tools`, or to the tracker image build.
- Any change to `.github/workflows/container.yaml`. It is already correct and is the reference this fix matches.
- Repairing the stale `src/bin/` inventory in `AGENTS.md`, which still lists `e2e_tests_runner` and `profiling` as root-package binaries and omits `e2e-tools` from the package catalog. It is the same drift from `c47173f53`, and #2190 has since taken ownership of the five live references that carry it.
- Any general CI policy about jobs that can only fail where nobody looks. The Design and Ownership Review notes the question; this issue does not answer it.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md` documents the E2E and unit-test split these workflows implement; it is context, not a constraint this change touches.
- ADRs to create: None required. The defect is a missing package flag on four command lines, and the correct value is already fixed by the package that owns the binaries; there is no design choice to record.

## Design and Ownership Review

Not applicable in the template's usual sense: this issue adds no child processes, asynchronous I/O, readiness waits, resource cleanup, or test fixtures. Every change is confined to one workflow file, and the behavior it restores is entirely the behavior `container.yaml` already demonstrates with the same commands against the same binaries.

Two ownership observations are worth carrying forward without widening this issue. First, the four invocations exist twice, in `testing.yaml` and in `container.yaml`, with no mechanism keeping them in step; this defect is what that duplication costs, and a later issue could decide whether one workflow should own the commands. Second, the job's silence on `develop` is a property of its `if:` guard rather than of this fix: the guard is doing what it was designed to do, and it is also what let a broken job stay broken for three months. Whether any job should be allowed to fail only on events that upstream CI never runs is a question worth asking separately; it is noted here, not scoped here.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                        | Notes / Expected Output                                                                                                                                                                                                                     |
| --- | ------ | ------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Add the package flag and amend the comment  | The four `cargo run` lines in the `docker-e2e` job of `.github/workflows/testing.yaml` carry `-p torrust-tracker-e2e-tools`; the `run-tracker-e2e-tests` step uses a `>-` block scalar; the job comment gains one sentence explaining the flag; `linter all` exits `0`. |
| T2  | TODO   | Verify on a feature-branch push             | The branch is pushed to a fork or another in-repo feature branch, the `Docker E2E` job of `testing.yaml` runs, its four runner steps execute and pass, and the run URL is recorded as evidence for M1 and AC2.                                |

## Commit Points

| Task | Coherent change set                                                                                             | Commit policy                                                                                              |
| ---- | --------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| T1   | The workflow edit: four `cargo run` lines plus the block scalar and the comment sentence, in one file.           | Commit after `linter all` passes. One signed Conventional Commit, scope `ci`.                              |
| T2   | Evidence only: progress-log entry and manual-scenario rows updated with the run URL, no workflow change.        | Commit after the run completes, or fold into the pull request description if no spec update is warranted.   |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and moved to `docs/issues/open/2179-docker-e2e-job-package-flag/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue [#2179](https://github.com/torrust/torrust-tracker/issues/2179) created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-08 21:30 UTC - Spec author - Drafted this specification against `develop` at `e4db63d5`. Verified the four invocation line numbers in both workflows, the `if:` guard range, the binaries' location under `packages/e2e-tools/src/bin/`, and the root `Cargo.toml` `[package]`/`default-run` declaration. Traced the divergence to `c47173f53` (package extraction) and `2d1ce246b` (which repaired `container.yaml` only), not to `c1b6c5466`, which added only the guard. Measured the four line lengths against the 200-character limit in `.yamllint-ci.yml` and found that line 185 needs a `>-` block scalar. CI observations for #2055, #2106, and the `merge-tool-symlink-exceptions-spec` push are recorded as reported evidence; no GitHub access was available while drafting.
- 2026-09-10 08:15 UTC - Spec author - Re-verified both evidence facts before opening the issue. Against `develop` at `89d45145` the four `cargo run` invocations in the `docker-e2e` job still carry no `-p` flag at lines 185, 189, 193, and 197, measuring 184, 153, 151, and 156 characters, so the block-scalar requirement on the first step holds unchanged. The symptom is live on the `da2ce7/torrust-tracker` fork: in the `Testing` run for `4bff469e` the `Docker E2E` job fails at step `Run E2E Tests` after a successful image build, with the three qBittorrent steps skipped - https://github.com/da2ce7/torrust-tracker/actions/runs/34444943476/job/102767609212
- 2026-09-10 08:15 UTC - Spec author - GitHub issue #2179 created from the reviewed draft; specification moved to `docs/issues/open/2179-docker-e2e-job-package-flag/ISSUE.md` - https://github.com/torrust/torrust-tracker/issues/2179

## Acceptance Criteria

- [ ] AC1: The four `cargo run` invocations in the `docker-e2e` job of `.github/workflows/testing.yaml` each carry `-p torrust-tracker-e2e-tools`, and every other argument of each step is unchanged.
- [ ] AC2: On a push to a feature branch, the `Docker E2E` job of `testing.yaml` runs, its four runner steps reach the E2E runners rather than failing on target resolution, and the job completes successfully. Evidence is the run URL.
- [ ] AC3: `.github/workflows/container.yaml` is unchanged by this issue.
- [ ] AC4: The `docker-e2e` job comment records why the package flag is required.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all` — this covers the YAML change through yamllint (`.yamllint-ci.yml`), including the 200-character line-length rule that motivates the `>-` block scalar on the first step, and covers this specification through markdownlint, cspell, and lychee.
- No Rust tests are affected; the change touches no code.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

A pull request targeting `develop` cannot verify this fix: the `if:` guard skips `docker-e2e` for exactly that event, so the job never runs and produces no evidence. Verification must come from an event the guard admits.

| ID  | Scenario                                          | Command/Steps                                                                                                                                                | Expected Result                                                                                                                                                    | Status | Evidence                                              |
| --- | ------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ----------------------------------------------------- |
| M1  | The job runs and passes on a feature-branch push  | Push the fix branch to a fork or to an in-repo branch that is not `develop`, `main`, or `releases/*`, then open the `Testing` workflow run for that push.     | The `Docker E2E` job runs; steps `run-tracker-e2e-tests`, `run-qbittorrent-e2e-test-sqlite3`, `run-qbittorrent-e2e-test-mysql`, and `run-qbittorrent-e2e-test-postgresql` all execute the runners and succeed; the job is green. | TODO   | Run URL of the `Docker E2E` job                       |
| M2  | Behavior on `develop` is unchanged                 | Open the pull request for this fix against `develop` and inspect its `Testing` workflow run.                                                                  | The `Docker E2E` job is skipped, as it is today, because `container.yaml` covers the event. No new job appears and no existing job changes status.                     | TODO   | Run URL of the pull request's `Testing` workflow      |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                            |
| ----- | ---------------------- | --------------------------------------------------- |
| AC1   | TODO                   | Diff of `.github/workflows/testing.yaml`             |
| AC2   | TODO                   | M1 run URL                                          |
| AC3   | TODO                   | Pull request file list                              |
| AC4   | TODO                   | Diff of `.github/workflows/testing.yaml`             |

## Risks and Trade-offs

- No material risk to the change itself. The flag value is not a guess: `container.yaml` runs the same four binaries with the same flag against the same package on every push to `develop`, so the invocation form is already proven in CI.
- The line-length rule is the only way this edit can go wrong mechanically, and `linter all` catches it before the change leaves the working tree.
- Verification depends on an event upstream CI does not produce, so the passing run URL must be captured deliberately rather than expected to appear on the pull request. M1 exists for that reason.
- Restoring the job means feature-branch pushes will start spending E2E time they have not spent since June 2026. That is the job's intended cost, not a regression, and the job already carries `timeout-minutes: 90`.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- A retrospective is likely unnecessary for a four-line workflow fix. It becomes warranted if the feature-branch run reveals that the runner steps fail for some further reason once target resolution succeeds, since that would mean the job has been hiding more than one defect.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Origin of the job's `if:` guard: #1854, spec at `docs/issues/closed/1854-1840-workflow-performance-container-test-gating/ISSUE.md`
- Related EPIC: #1840, spec at `docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md`
- Reported failing runs: #2055, #2106
- Observed failing run: the `Docker E2E` job of the fork's `Testing` run for `4bff469e`, 2026-09-10 - https://github.com/da2ce7/torrust-tracker/actions/runs/34444943476/job/102767609212
- Commit `c1b6c5466` — added the `if:` guard and comment to the `docker-e2e` job
- Commit `c47173f53` — moved the E2E binaries into `packages/e2e-tools`, breaking both workflows
- Commit `2d1ce246b` — added `-p torrust-tracker-e2e-tools` to `container.yaml` only
- `.github/workflows/testing.yaml` — the `docker-e2e` job, lines 124-197
- `.github/workflows/container.yaml` — the correct invocations, lines 117, 123, 127, 131
- `packages/e2e-tools/` — package `torrust-tracker-e2e-tools`, owner of both binaries
- `.yamllint-ci.yml` — the 200-character line-length rule
