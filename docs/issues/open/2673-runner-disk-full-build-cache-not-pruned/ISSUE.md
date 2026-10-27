---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: null
github-issue: 2673
spec-path: docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/ISSUE.md
branch: "2673-runner-disk-full-build-cache-not-pruned-spec"
related-pr: null
last-updated-utc: "2026-10-27 09:50"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/self-hosted-runner.md
    - docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md
    - .github/workflows/container.yaml
---

<!-- skill-link: create-issue -->

<!-- cspell:ignore journalctl retag -->

# Issue #2673 - Self-hosted runner disk fills because the daily prune cannot free the Docker build cache

## Goal

The self-hosted runner `torrust-runner-01` keeps its disk below a safe level without manual intervention, and the maintainers are told when it does not.

## Background

The container test jobs run on a self-hosted Hetzner runner (see [docs/self-hosted-runner.md](../../../self-hosted-runner.md)). Its documented cleanup is a daily systemd timer, `docker-ci-prune.timer`, that runs `docker builder prune --force --max-used-space 120GB` plus container, image, and volume prunes. The guide says the disk is sized to absorb the cache growth between runs.

What happens, step by step, as observed on 2026-10-27 (details in [manual-verification-evidence.md](manual-verification-evidence.md)):

1. CI jobs build the tracker image on the runner, and the Docker build cache grows well past 120 GB between daily runs.
2. Until about 2026-10-22 the 04:00 UTC prune freed tens of GB per day (for example 88.56 GB on 2026-10-15). Its freed amount then fell (17.48 GB on 2026-10-23, 943 MB on 2026-10-24) and has been 0 B on 2026-10-25, 2026-10-26, and 2026-10-27.
3. On 2026-10-27 the build cache is 253.6 GB in 487 records, and Docker reports none of it reclaimable. The root filesystem `/` is 100% full (288 GB of 301 GB used, under 200 MB free).
4. From about 07:42 UTC the system logs "No space left on device". Container jobs on the runner fail within seconds: the `develop` push run created at 07:28 UTC and the run for pull request 2637 created at 08:09 UTC both ended with `Test (Docker) (release)` failed and the publish jobs skipped.

The timer itself is healthy: it fired on every day from 2026-09-26 and exits with status 0. The failure is silent. Nothing in the setup fails or alerts when a prune frees nothing, and the first visible sign was failed CI jobs.

Contract violated: the setup guide states that the daily prune trims the cache to 120 GiB and that the disk is sized to absorb the growth. Neither holds today.

Impact: container tests, and on `develop` the development-image publish, cannot complete on the runner. Until the disk is freed, every pull request that needs `Test (Docker)` fails or waits.

### Local hypothesis (not proven)

Four containers started from `torrust-tracker:local` images have run for 4 to 12 days (started 2026-10-14, 10-16, 10-20, and 10-22) and all show healthy. Their image ids no longer appear in `docker image ls -a`, because newer builds retag `torrust-tracker:local`. The containerd snapshotter reports 121 Active snapshots. The prune freed less each time one of these containers was started, so they probably keep build-cache layers in use. They were most likely orphaned when an E2E run was cancelled or killed, but the origin is not confirmed. An alternative explanation, such as leaked build leases from cancelled builds, is not ruled out. The diagnosis task decides between them with evidence.

A second, smaller gap: the host-side Cargo target cache (23 GB under `/home/runner/.cache/torrust-tracker/`) has no automatic cleanup.

## Scope

### In Scope

- Diagnose why the build cache is no longer reclaimable and record the evidence.
- Restore free disk space on the runner, with the maintainer's explicit approval for each step that changes the runner.
- Fix the cause so containers or build state left by a cancelled or failed job cannot keep the cache pinned.
- Make the daily cleanup observable: a prune that frees nothing while the cache is above target, or a disk above 85%, must be visible to the maintainers.
- Decide and document cleanup for the host Cargo target cache.
- Update [docs/self-hosted-runner.md](../../../self-hosted-runner.md) so it matches the new behavior, and replace its claim that the disk is sized to absorb the cache with the measured facts.

### Out of Scope

- Moving jobs back to GitHub-hosted runners or adding runner capacity.
- Resizing the Hetzner server or disk, unless the diagnosis shows the cache cannot fit under any prune policy. That would be a separate decision.
- Changing which jobs run on the runner.
- The pending reboot and the runner agent update; both are recorded in the health report.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
- ADRs to create: None known. Create one if the fix changes how jobs own and clean up Docker resources on the runner.

## Design and Ownership Review

Applies, because the likely fix concerns resource cleanup for child processes (containers) started by CI jobs, including when a job is cancelled.

- Responsibility and ownership map: to be written when the diagnosis names the owner of the leaked containers (a workflow step, the `qbittorrent_e2e_runner` or `e2e_tests_runner` binaries in `packages/e2e-tools`, or a test script). The owner of a container is the component that starts it and must remove it on the normal, failure, and cancellation paths.
- Lifetime invariants: a job leaves no running container and no unreleased build state after it ends, however it ends. A janitor on the host is a safety net, not the owner.
- Deadlines: any cleanup step added to a workflow or a script bounds every await with an explicit timeout so cleanup cannot hang a job.
- Design-review checkpoint: after the first passing vertical slice of the fix, review ownership before adding more cleanup paths.

## Bug-Fix Process

This issue is substantively a bug, so it follows [fix-bug](../../../../.github/skills/dev/debugging/fix-bug/SKILL.md). Planned sequence:

1. Analysis and hypothesis: the section above. The reproduction at drafting time observed the symptom; the cause is open.
2. Real-artifact reproduction: **Reproduced** for the symptom (full disk, prune frees 0 B, jobs fail) on 2026-10-27, recorded in [manual-verification-evidence.md](manual-verification-evidence.md). The cause is not yet reproduced; T1 does that with a controlled reproduction, for example cancelling an E2E run and observing a leftover container.
3. Regression-test boundary: see the next section.
4. Red evidence: the failing run before the fix.
5. Production fix.
6. Green evidence and a like-for-like recheck of the original symptom.

## Regression Test Strategy

Selection is deferred until T1 names the owner of the leak, because the smallest deterministic boundary depends on it:

- If a Rust runner in `packages/e2e-tools` leaves containers behind when cancelled, the boundary is a unit or collaboration test at that cleanup seam, written with the `write-unit-test` skill and proven red against the current behavior.
- If the leak is in a workflow step or a shell script, the preferred boundary is a Rust test or a script with a fake `docker` command that records calls, in line with the repository rule that non-trivial logic should not live only in workflow shell.
- The host-level behavior of the systemd units and of the disk alert cannot be covered by a deterministic maintained test inside this repository. For those, the evidence is manual, recorded in [manual-verification-evidence.md](manual-verification-evidence.md), and the follow-up that would make them testable is a documented script for the alert logic.

The red and green output and the final recheck are recorded in the same evidence file.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                  | Notes / Expected Output                                                                                                                                       |
| --- | ------ | ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Diagnose what pins the build cache and reproduce the leak in a controlled way                         | Read-only evidence that names the owner of the held state, and a reproduction of how a cancelled job leaves it. Updates the hypothesis section.                |
| T2  | TODO   | Restore free disk space on the runner, with the maintainer's approval for each step                   | `/` is below 70% used and a manual run of the prune frees the cache toward 120 GiB. The state before and after is recorded. No job is running during the change. |
| T3  | TODO   | Select the regression-test boundary and write the maintained test                                     | Boundary rationale recorded. The test fails against the current behavior (red) and the failing output is recorded.                                           |
| T4  | TODO   | Fix the cause of the leaked state                                                                     | Containers or build state of a cancelled or failed job are removed by their owner. The smallest production change.                                           |
| T5  | TODO   | Make the cleanup observable and cover the host Cargo target cache                                     | A check that reports a prune freeing 0 B above target and a disk above 85%, and a documented policy or prune for the Cargo target cache.                    |
| T6  | TODO   | Update the runner guide and add the next health report                                                | `docs/self-hosted-runner.md` matches the behavior. A follow-up health report shows the recovered state.                                                      |
| T7  | TODO   | Verify green and recheck like-for-like                                                                | Regression test green. The original symptom is rechecked: prune frees space and the disk stays below the alert level across at least three daily runs.        |

Follow the Required Sequence of the `fix-bug` skill: T3 red before T4, and T7 after the last production change.

## Commit Points

| Task | Coherent change set                                                                  | Commit policy                                        |
| ---- | ------------------------------------------------------------------------------------ | ---------------------------------------------------- |
| T1   | Diagnosis and reproduction evidence in `manual-verification-evidence.md`             | Commit after the evidence is complete.               |
| T3   | The failing regression test, with a design review recorded                           | Commit after focused validation and required review. |
| T4   | The production fix that makes the regression test pass                               | Commit after focused validation and required review. |
| T5   | The observability check and the Cargo target cache policy                            | Commit after focused validation and required review. |
| T6   | The updated runner guide and the follow-up health report                             | Commit after `linter all` passes.                    |

T2 and T7 change the runner, not the repository; their results are recorded as evidence, and they create no commit of their own unless the evidence changes. For test-producing work, use the `write-unit-test` skill and complete the prose-first Arrange-Act-Assert design review for each test increment before maintainer review and commit. Use a Conventional Commit message with the narrow scope and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-27 09:45 UTC - GitHub Copilot (Claude Opus 5.5) - Drafted the specification from the first runner health report and recorded the symptom reproduction - [manual-verification-evidence.md](manual-verification-evidence.md)
- 2026-10-27 09:50 UTC - GitHub Copilot (Claude Opus 5.5) - Maintainer approved the draft as a standalone issue (no parent EPIC) with a spec-only PR first; created GitHub issue 2673 and moved the specification to `docs/issues/open/`

## Acceptance Criteria

- [ ] AC1: The cause that keeps the Docker build cache from being reclaimed is identified with evidence, and the owner of the leaked state is named.
- [ ] AC2: A job that is cancelled or fails leaves no running container and no unreleased build state on the runner; a maintained regression test or documented manual evidence shows this for a cancelled run.
- [ ] AC3: After the fix, the daily prune brings the build cache back toward its 120 GiB target, shown across at least three consecutive daily runs.
- [ ] AC4: `/` on the runner stays below 85% used across those runs, and a prune that frees 0 B while the cache is above target, or a disk above 85%, is reported to the maintainers.
- [ ] AC5: The host Cargo target cache has a documented cleanup policy.
- [ ] AC6: `docs/self-hosted-runner.md` matches the behavior and no longer states that the disk absorbs the cache growth without evidence.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- The maintained regression test selected in T3
- Pre-push checks when the repository changes
- `cargo +nightly fmt --all -- --check` on the nightly Rust toolchain when Rust code changes

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                       | Human-oriented command/steps                                                                                                                                  | Expected Result                                                                          | Status | Evidence                                     |
| --- | -------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Reproduce the symptom: full disk and a prune that frees nothing | On `torrust-runner-01`: `df -h /`, `docker system df`, `docker ps -a`, and `journalctl -u docker-ci-prune.service`; `gh run list` for failed container runs | `/` is full, the cache is not reclaimable, and the prune frees 0 B                       | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Reproduce the leak: cancel a container job and look for leftovers | Cancel a running `Test (Docker)` job, then `docker ps -a` and `docker buildx du` on the runner                                                              | A container or build state remains after cancellation (confirms the hypothesis)          | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Like-for-like recheck after the fix                            | Repeat M1 and M2 after the last production change; wait for three daily prune runs                                                                            | The cancelled job leaves nothing; the prune frees space; the disk stays below 85%        | TODO   | `manual-verification-evidence.md` section V3 |

Notes:

- Every runner command is read-only except the ones in T2, which need the maintainer's explicit approval and run only while the runner is idle.
- Record the toolchain or runtime for each result that depends on one.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   | T1       |
| AC2   | TODO                   | T3, T4   |
| AC3   | TODO                   | T7       |
| AC4   | TODO                   | T5, T7   |
| AC5   | TODO                   | T5       |
| AC6   | TODO                   | T6       |

## Risks and Trade-offs

- Stopping containers or pruning while a job runs can break that job. Mitigation: do these steps only while the runner is idle, and check `gh api repos/torrust/torrust-tracker/actions/runners` for `busy: false` first.
- A broad prune (`docker builder prune --all`) throws away the warm cache and makes the next builds slower. Mitigation: prefer the targeted removal first, and measure build time after.
- The hypothesis may be wrong. Mitigation: T1 comes first and the plan changes with its result.
- Cleanup added to a workflow can itself fail or hang. Mitigation: bound it with a timeout and run it with `if: always()` so it does not hide the job result.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related PRs: #2672 (adds the runner health report this issue is based on, including its first report, `docs/self-hosted-runner-reports/20261027-091200-runner-report.md`)
- Related docs: [docs/self-hosted-runner.md](../../../self-hosted-runner.md)
- Related ADRs: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
