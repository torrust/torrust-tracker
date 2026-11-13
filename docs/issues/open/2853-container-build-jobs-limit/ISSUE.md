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
last-updated-utc: "2026-11-13 08:50"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - Containerfile
    - contrib/dev-tools/containers/docker-build.sh
    - contrib/dev-tools/experiments/container-build-jobs/REPORT.md
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

- Measure peak memory and wall time of the release image build at `CARGO_BUILD_JOBS` = 4, 8 and 16, and record it in `contrib/dev-tools/experiments/container-build-jobs/REPORT.md` (same format as `contrib/dev-tools/experiments/sccache-docker/`), together with the measurement script.
- Choose the default from that evidence.
- `Containerfile`: one `ARG CARGO_BUILD_JOBS` plus `ENV CARGO_BUILD_JOBS` in the `chef` base stage, so every Rust stage is capped in one place.
- `contrib/dev-tools/containers/docker-build.sh`: pass the value as a build argument when the caller sets it.
- `docs/containers.md`: a "Memory and CPU use" subsection (default and why, raising and lowering it, memory rule of thumb, effect on plain `cargo build` and `cargo test`).
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
| T1  | DONE        | Measure the release image build at 4, 8 and 16 jobs          | `REPORT.md` with peak memory, peak `rustc` count and wall time per job count    |
| T2  | DONE        | Choose the default and decide on stage overlap               | Decision and reasoning in `REPORT.md`                                           |
| T3  | DONE        | `Containerfile` `ARG`/`ENV` in the `chef` stage              | Every cook, archive and build step capped; stages outside `chef` need no change |
| T4  | DONE        | `docker-build.sh` passes `CARGO_BUILD_JOBS`                  | Build argument only when set                                                    |
| T5  | DONE        | `docs/containers.md` "Memory and CPU use"                    | Default, raise, lower, rule of thumb, plain-cargo note                          |
| T6  | DONE        | Check the self-hosted and GitHub-hosted runners              | Recorded in `REPORT.md` (see Risks)                                             |

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
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-13 09:50 UTC - Claude Code (Claude Sonnet 5.5) - Spec approved by the maintainer; issue #2853 created; implementation PR with spec, evidence and changes (no spec-only PR) - https://github.com/torrust/torrust-tracker/issues/2853
- 2026-11-13 09:05 UTC - Claude Code (Claude Sonnet 5.5) - Measured the release image build (uncached, one at a time, watchdog at 8 GiB): 4 jobs 836 s / 10.4 GiB peak `rustc` RSS (12 `rustc`); 8 jobs 499 s / 19.8 GiB (16); 16 jobs 346 s / 35.5 GiB (24). Chose default 4; stage overlap left unchanged; implemented Containerfile, script and docs on the draft branch (not committed) - `contrib/dev-tools/experiments/container-build-jobs/REPORT.md`
- 2026-11-13 08:50 UTC - Claude Code (Claude Sonnet 5.5) - Drafted the specification; measurement of the release image build started at 4 jobs - `contrib/dev-tools/experiments/container-build-jobs/`

## Acceptance Criteria

- [ ] AC1: `REPORT.md` records peak memory and wall time of the release image build at 4, 8 and 16 jobs, and the reproducible command.
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
| M1  | Measurement at 4, 8, 16     | `contrib/dev-tools/experiments/container-build-jobs/measure.sh <N>` one at a time, watchdog at 8 GiB available | Peak memory and wall time recorded                            | DONE   | `REPORT.md`                                  |
| M2  | Release build, default      | `docker build --no-cache --target release --tag torrust-tracker:release --file Containerfile .`               | Build passes; peak memory matches the measured default        | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Override and script         | `docker build --target chef --build-arg CARGO_BUILD_JOBS=2 ...` then `printenv`; `CARGO_BUILD_JOBS=2 ./contrib/dev-tools/containers/docker-build.sh` | `CARGO_BUILD_JOBS=2` visible in the stage and used by the build | DONE   | `manual-verification-evidence.md` section V3 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `REPORT.md` |
| AC2   | DONE                   | M2, M3   |
| AC3   | DONE                   | M3       |
| AC4   | DONE                   | `docs/containers.md` |
| AC5   | DONE                   | `REPORT.md` |
| AC6   | DONE                   | `REPORT.md` |

## Risks and Trade-offs

- A lower default makes builds slower on big machines. Mitigation: one flag raises it; the measured slowdown is documented.
- The self-hosted runner (8 vCPU, 16 GB RAM) may build slower with a cap below 8. Mitigation: checked in T6 against the measured times; the workflow can pass a different value through `build-args` if needed.
- `CARGO_BUILD_JOBS` set in the image `ENV` also reaches `cargo nextest run` in the `test` stages (they derive from `tester`, not `chef`, so they are not capped; the test binaries are already compiled by then).

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related ADRs: `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`
- Related docs: `docs/self-hosted-runner.md`, `docs/containers.md`
