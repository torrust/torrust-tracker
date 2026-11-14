---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p2
epic: null
github-issue: 2853
spec-path: docs/issues/open/2853-container-build-jobs-limit/ISSUE.md
branch: "2853-container-build-jobs-limit"
related-pr: null
last-updated-utc: "2026-11-14 08:26"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - Containerfile
    - contrib/dev-tools/containers/docker-build.sh
    - contrib/dev-tools/experiments/container-build-jobs/report.md
    - docs/containers.md
    - docs/self-hosted-runner.md
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2853 - Limit the Cargo parallelism of the container image build by default

## Goal

Building the container image must not exhaust the RAM of an ordinary developer machine. The `Containerfile` caps the number of parallel Cargo jobs by default, with a measured default, and `docs/containers.md` explains how to raise or lower it.

## Background

Cargo starts one `rustc` job per CPU core unless told otherwise. Several `Containerfile` steps have no limit: `cargo chef cook --tests --workspace --all-features` (debug and release chains), `cargo chef cook` of the shipped binaries, `cargo nextest archive` and `cargo build`. BuildKit also runs independent stages at the same time: in the release chain, `dependencies` (test cook) and `dependencies_shipped_thirdparty` (shipped cook) both stack on `dependencies_thirdparty`, and `build` and `build_shipped` both follow. So the number of concurrent `rustc` processes is roughly the core count times the number of overlapping stages, and each holds about 1.2-2.2 GB (LTO and many generics).

Field evidence from a maintainer workstation (32 threads, 62 GB RAM, 8 GB swap): the kernel OOM killer fired twice (2026-11-11 11:32 and 2026-11-12 21:10). Each time the kernel log showed 31 `rustc` processes running as root (inside Docker/BuildKit) holding 33-38 GB of RSS, RAM exhausted and swap full. The kernel killed the desktop application (the largest unprotected process; Docker is protected) and every session in it. The same failure already happened on the self-hosted CI runner (8 vCPU, 16 GB): `docs/self-hosted-runner.md` records that eight parallel `rustc` processes exhausted 16 GB on the first run, and the fix there was swap (a machine-level workaround, not available to contributors).

The goal is to lower the default so contributors with smaller machines can build the image without friction. A bigger machine only loses some time and can raise the limit with one flag.

## Scope

### In Scope

- Measure peak memory and wall time of the release image build at `CARGO_BUILD_JOBS` = 4, 8 and 16, and record it in `contrib/dev-tools/experiments/container-build-jobs/report.md` (same format as `contrib/dev-tools/experiments/sccache-docker/`), together with the measurement script.
- Choose the default from that evidence.
- `Containerfile`: one `ARG CARGO_BUILD_JOBS` plus `ENV CARGO_BUILD_JOBS` in the `chef` base stage (every stage that compiles Rust and derives from it), and the same declaration in the separate `tester` base stage (its `cargo binstall` of `cargo-nextest` can fall back to a source compile), so every compiling stage is capped.
- `contrib/dev-tools/containers/docker-build.sh`: pass the value as a build argument when the caller sets it.
- `docs/containers.md`: a "Memory and CPU Use" subsection (default and why, raising and lowering it, memory rule of thumb, effect on plain `cargo build` and `cargo test`).
- Decide and record whether BuildKit stage overlap should also be addressed in the `Containerfile`.
- Check the effect on the self-hosted CI runner and on GitHub-hosted runners.

### Out of Scope

Related context only; these belong to the maintainer's machine setup and tooling, not to the Tracker:

- BuildKit `max-parallelism` and Docker daemon memory limits.
- A cap on how many container builds run at the same time on one machine.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md` (unit tests stay in the image build), `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md` (self-hosted runner).
- ADRs to create: None known. A build-argument default is a routine implementation choice recorded in the experiment report and the `Containerfile` comment; revisit if the stage-overlap question below leads to a structural change of the `Containerfile`.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Following `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- Analysis: unbounded Cargo jobs times overlapping BuildKit stages exhaust RAM (see Background).
- Reproduction: infeasible to repeat safely on the 32-thread machine (it crashed the desktop twice). The evidence is the kernel OOM log of both incidents; the capped builds are measured instead, with a watchdog, and the uncapped build is never run.
- Regression-test boundary: the `Containerfile` build itself (manual measurement). There is no practical maintained automatic test for peak memory; see the next section.
- Production fix: the `Containerfile` default.
- Final recheck: the release image build with the chosen default, at its measured peak.

## Regression Test Strategy

No deterministic automatic test can fail for "peak memory too high" without running a multi-gigabyte build. The boundary is the manual measurement in the experiment report plus the CI image build (`container.yaml`), which must still pass and must not become meaningfully slower. A cheap automatic guard is a check that the built image environment of a build stage carries `CARGO_BUILD_JOBS` (`docker build --target chef` then `docker run ... printenv`), recorded as manual scenario M3.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                                                         | Notes / Expected Output                                                         |
| --- | ----------- | ------------------------------------------------------------ | ------------------------------------------------------------------------------- |
| T1  | DONE        | Measure the release image build at 4, 8 and 16 jobs          | `report.md` with peak memory, peak `rustc` count and wall time per job count    |
| T2  | DONE        | Choose the default and decide on stage overlap               | Decision and reasoning in `report.md`                                           |
| T3  | DONE        | `Containerfile` `ARG`/`ENV` in the `chef` and `tester` stages | Every cook, archive and build step, and `tester`'s own tool install, capped    |
| T4  | DONE        | `docker-build.sh` passes `CARGO_BUILD_JOBS`                  | Build argument only when set                                                    |
| T5  | DONE        | `docs/containers.md` "Memory and CPU Use"                    | Default, raise, lower, rule of thumb, plain-cargo note                          |
| T6  | DONE        | Check the self-hosted and GitHub-hosted runners              | Reasoning in `report.md`; self-hosted build step measured (M4)                   |

## Commit Points

| Task   | Coherent change set                                            | Commit policy                                        |
| ------ | -------------------------------------------------------------- | ---------------------------------------------------- |
| T1, T2 | Experiment script and report with the measured evidence        | Commit after focused validation and required review. |
| T3, T4 | `Containerfile` and `docker-build.sh`                          | Commit after focused validation and required review. |
| T5     | `docs/containers.md`                                           | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2853-container-build-jobs-limit/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-13 08:50 UTC - Claude Code (Claude Sonnet 5.5) - Drafted the specification while the measurement runs described in the next entry were already under way (started 08:31 UTC) - `contrib/dev-tools/experiments/container-build-jobs/`
- 2026-11-13 09:00 UTC - Claude Code (Claude Sonnet 5.5) - Measured the release image build (uncached, one at a time, watchdog at 8 GiB): 4 jobs (08:31-08:45 UTC) 836 s / 10.4 GiB peak `rustc` RSS; 8 jobs (08:45-08:53 UTC) 499 s / 19.8 GiB; 16 jobs (08:53-08:59 UTC) 346 s / 35.5 GiB. Chose default 4; stage overlap left unchanged - `contrib/dev-tools/experiments/container-build-jobs/results/`
- 2026-11-13 09:48 UTC - Claude Code (Claude Sonnet 5.5) - Spec approved by the maintainer; issue #2853 created; implemented Containerfile, script and docs (`docs(issues): [#2853] spec and measured evidence for the container build jobs limit`, `fix(container): [#2853] cap the Cargo jobs of the image build with CARGO_BUILD_JOBS`, `docs(containers): [#2853] document the memory and CPU use of the image build`); opened implementation PR (no spec-only PR) - https://github.com/torrust/torrust-tracker/issues/2853, https://github.com/torrust/torrust-tracker/pull/2854
- 2026-11-13 10:38 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the Copilot review's R1/R2 (tester-stage cap, measure.sh signal trap), pushed (`fix(container): [#2853] address Copilot review: cap tester stage, trap measure.sh signals`) and replied on both threads.
- 2026-11-13 10:46 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's review (F1-F11): corrected report.md's per-process-memory arithmetic and stage-overlap claim (F4, F5), fixed the `tester`/`chef` risk statement (F6), documented the build-cache invalidation (F7), renamed `REPORT.md` to `report.md` (F10) and fixed the heading-case and script-path nits (F9, F11), added the evidence file's reproduction classification (F2) and the no-flag default verification (F3) (`fix(container): [#2853] address da2ce7 review: fix report arithmetic, evidence, docs nits`). AC6/T6 held at IN_PROGRESS pending the real self-hosted run at this head.
- 2026-11-13 10:52 UTC - Claude Code (Claude Sonnet 5.5) - Fixed the remaining `tester`/`chef` wording contradictions (F13) and added the review audit record at `docs/pr-reviews/pr-2854-review/PR-REVIEW.md` (F12) (`fix(container): [#2853] fix tester/chef wording contradiction, add audit record`).
- 2026-11-13 10:59 UTC - Claude Code (Claude Sonnet 5.5) - Fixed the audit record's table formatting and filled in its Reply URLs so `validate-audit-record.py` passes (`docs(pr-reviews): [#2853] fix audit record table formatting and reply URLs`).
- 2026-11-13 11:12 UTC - Claude Code (Claude Sonnet 5.5) - Restored the three raw run summaries da2ce7's F14 found had been overwritten with a wrong recomputed figure, and corrected report.md's 16-jobs delta to 33.0 (`fix(container): [#2853] restore raw run summaries, correct the 16-jobs delta to 33.0`).
- 2026-11-13 11:28 UTC - Claude Code (Claude Sonnet 5.5) - Addressed R3/R4 (an earlier Copilot review), da2ce7's F15-F17/F19/F20, and Copilot's F21-F24 (locale, PID-anchor, stage-overlap scope, audit-record accuracy, and small doc fixes) (`fix(container): [#2853] address review round 2: locale, PID anchor, stage overlap, audit accuracy`).
- 2026-11-13 11:43 UTC - Claude Code (Claude Sonnet 5.5) - Addressed da2ce7's next round (F25-F27, reassigned from its reused F21-F23 under the audit's collision rule): this entry's own 08:50 claim was itself one of the findings, so it was corrected above rather than here; also fixed report.md's remaining Method/Stage-overlap overstatements and the audit record's two mistimed Processing Log entries (left as written, with an appended correction) (`fix(container): [#2853] address review round 3: audit timestamps, report accuracy`).
- 2026-11-13 13:26 UTC - Claude Code (Claude Fable 5.2) - Appended a correction to the audit log and removed two quoted commit hashes (`docs(pr-reviews): [#2853] correct the audit log and drop quoted commit hashes`, committed 13:17 UTC); all 31 review threads are resolved. Recorded the self-hosted runner build step times (M4, evidence V4): 827 s at 4 jobs against 731 s uncapped. AC6 stays TODO until the maintainer decides whether that is acceptable. Added `implementation-retrospective.md`.
- 2026-11-13 22:33 UTC - Claude Code (Claude Fable 5.2) - The maintainer accepted the 13% slower build step on the self-hosted runner, so AC6 is DONE. Rebased the branch onto `develop` to take the fix of the container regression script (issue #2855), which made the `Test (Docker) (release)` check fail on this pull request. Stated in `implementation-retrospective.md` the time its review counts were taken (review finding F30).
- 2026-11-14 08:26 UTC - Claude Code (Claude Opus 5.5) - Addressed review findings F31 to F33: named where and when the maintainer accepted AC6, pointed the F30 audit row at the reply that reports its fix, and replaced the pre-rebase failing-check statement in evidence V4 with the passing run at the rebased head.

## Acceptance Criteria

- [ ] AC1: `report.md` records peak memory and wall time of the release image build at 4, 8 and 16 jobs, and the reproducible command.
- [ ] AC2: `docker build --target release` on a machine with many cores keeps the Cargo jobs at the chosen default without any flag; `--build-arg CARGO_BUILD_JOBS=N` overrides it for Docker and Podman.
- [ ] AC3: `docker-build.sh` honours `CARGO_BUILD_JOBS` from the environment.
- [ ] AC4: `docs/containers.md` documents the default, how to raise and lower it, and the memory rule of thumb.
- [ ] AC5: The decision on BuildKit stage overlap is recorded with its reasoning.
- [ ] AC6: The effect on the self-hosted and GitHub-hosted runners is recorded and shows no meaningful slowdown, or the workflow passes a different value.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `hadolint` on the `Containerfile` (the `container.yaml` workflow runs it)
- Pre-push checks

### Manual Verification Scenarios

| ID  | Scenario                    | Human-oriented command/steps                                                                                   | Expected Result                                               | Status | Evidence                                     |
| --- | --------------------------- | -------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Measurement at 4, 8, 16     | `contrib/dev-tools/experiments/container-build-jobs/measure.sh <N>` one at a time, watchdog at 8 GiB available | Peak memory and wall time recorded                            | DONE   | `report.md`                                  |
| M2  | Release build, implicit default | `docker build --target chef\|tester ...` and `--target release --build-arg CARGO_BUILD_JOBS=4 ...` (the jobs-4 measurement) | No-flag build reaches `CARGO_BUILD_JOBS=4`; full build passes at that default | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Override and script         | `docker build --target chef\|tester --build-arg CARGO_BUILD_JOBS=2 ...` then `printenv`; `CARGO_BUILD_JOBS=2 ./contrib/dev-tools/containers/docker-build.sh` | `CARGO_BUILD_JOBS=2` visible in both stages and used by the build | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | Self-hosted runner | Read the `Build Tracker Image` step time of the `Test (Docker) (release)` job on `torrust-runner-01` for runs of this PR and of `develop` | Build step time recorded and compared with the uncapped baseline | DONE | `manual-verification-evidence.md` section V4 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `report.md` |
| AC2   | DONE                   | M2, M3   |
| AC3   | DONE                   | M3       |
| AC4   | DONE                   | `docs/containers.md` |
| AC5   | DONE                   | `report.md` |
| AC6   | DONE                   | Measured in V4: the build step is about 13% slower (827 s against 731 s). The maintainer accepted that on 2026-11-13, given in the agent session chat, not on GitHub, before 15:17 UTC (the time GitHub's edit history of the pull request description shows it changed to `Closes #2853`); the workflow passes no different value |

## Risks and Trade-offs

- A lower default makes builds slower on big machines. Mitigation: one flag raises it; the measured slowdown is documented.
- The self-hosted runner (8 vCPU, 16 GB RAM) may build slower with a cap below 8. Mitigation: checked in T6 against the measured times; the workflow can pass a different value through `build-args` if needed.
- `test` and `test_debug` derive from `tester`, not `chef`, so the `chef` stage's `ARG`/`ENV` does not reach them. `tester` now has its own `CARGO_BUILD_JOBS` `ARG`/`ENV` (added after review, since its `cargo binstall` of `cargo-nextest` can fall back to a source compile), but `cargo nextest run` itself compiles nothing — the test binaries it runs were already compiled by the preceding `build`/`build_debug`/`build_shipped*` stages — so the variable has no effect there either way.

## Implementation Completion Review

- Retrospective: [implementation-retrospective.md](implementation-retrospective.md). It covers the review of pull request #2854.

## References

- Related ADRs: `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`
- Related docs: `docs/self-hosted-runner.md`, `docs/containers.md`
