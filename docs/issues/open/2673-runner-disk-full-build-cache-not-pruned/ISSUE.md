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
last-updated-utc: "2026-10-27 10:57"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/self-hosted-runner.md
    - docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md
    - .github/workflows/container.yaml
    - contrib/dev-tools/self-hosted-runner/docker-ci-prune.sh
    - contrib/dev-tools/self-hosted-runner/tests/test-docker-ci-prune.sh
    - docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/deferred-proposals.md
---

<!-- skill-link: create-issue -->

<!-- cspell:ignore journalctl moby retag -->

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

### Cause (revised after T1 and T2; the leak trigger is still unproven)

The first draft of this specification guessed that four orphaned `torrust-tracker:local` containers pinned the cache. T1 and T2 rejected that: removing them changed nothing, and their image chains are about 28 small layers each. What pins the cache is 117 leaf records, each backed by an Active snapshot, that BuildKit inside dockerd still treats as in use although no job owns them. All 487 records descend from them, so the prune cannot remove any. Restarting `docker.service` released them (cache 253.6 GB to 100 GB, `/` from 100% to 42% used), so the pinned state lives in the daemon until it restarts; see [manual-verification-evidence.md](manual-verification-evidence.md) sections V2 and V4.

Still a hypothesis: the trigger is a cancelled or killed build. Evidence for it is a correlation (551 `context canceled` Solve errors since 2026-10-01, leaves created mostly on heavy-cancellation days, dockerd up 32 days). A cancelled build on a desktop with Docker 28.3.3, overlay2, and BuildKit 0.23.2 did not reproduce it; the runner uses Docker 29.8.1, the containerd snapshotter, and BuildKit 0.33.0.

A second, smaller gap: the host-side Cargo target cache (23 GB under `/home/runner/.cache/torrust-tracker/`) has no automatic cleanup. It was deleted once during T2, and the next job rebuilds it.

## Scope

### In Scope

- Diagnose why the build cache is no longer reclaimable and record the evidence.
- Restore free disk space on the runner, with the maintainer's explicit approval for each step that changes the runner.
- Make the daily cleanup release the orphaned build state itself: restart `docker.service` while the runner is idle when the prune cannot keep the disk below 70%, with a maintained shell test.
- Make a cleanup that cannot keep the disk below 85% visible as a failed `docker-ci-prune.service` unit in `systemctl --failed`.
- Decide and document cleanup for the host Cargo target cache.
- Update [docs/self-hosted-runner.md](../../../self-hosted-runner.md) so it matches the new behavior, and replace its claim that the disk is sized to absorb the cache with the measured facts.
- Record the alerting and prevention proposals that were deferred, in [deferred-proposals.md](deferred-proposals.md).

### Out of Scope

- Notifications to the maintainers and scheduled workflows; the maintainer deferred them. The proposals are in [deferred-proposals.md](deferred-proposals.md) and may be implemented later if a broken runner becomes a recurring problem.
- Preventing the leak at its source: the trigger is unproven (see V2), so reproducing it and any upstream fix or builder change are deferred to the same document.
- Moving jobs back to GitHub-hosted runners or adding runner capacity.
- Resizing the Hetzner server or disk, unless the diagnosis shows the cache cannot fit under any prune policy. That would be a separate decision.
- Changing which jobs run on the runner.
- The pending reboot and the runner agent update; both are recorded in the health report.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
- ADRs to create: None known. Create one if the fix changes how jobs own and clean up Docker resources on the runner.

## Design and Ownership Review

Applies, because the likely fix concerns resource cleanup for child processes (containers) started by CI jobs, including when a job is cancelled.

- Responsibility and ownership map: the pinned state is held by BuildKit inside dockerd, not by a workflow step or a Rust runner (T1). What is still open is which component should guarantee it is released when a job is cancelled or killed: the workflow (a cleanup step that runs on cancellation), a host-side janitor that restarts `docker.service` when the runner is idle, or an upstream fix. That decision is made before T3.
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

T1 showed the pinned state lives in BuildKit inside dockerd, not in a job-owned container, so the boundary is the cleanup script itself.

- **Selected boundary:** a bash test, [test-docker-ci-prune.sh](../../../../contrib/dev-tools/self-hosted-runner/tests/test-docker-ci-prune.sh), that runs `docker-ci-prune.sh` with stub `docker`, `systemctl`, `pgrep`, `df`, and `sleep` commands. It follows the precedent in `contrib/dev-tools/checks/tests/`. The logic is a few decisions (prune, restart or skip, fail), so the repository rule that non-trivial logic should prefer Rust does not require a Rust tool here; revisit if the logic grows.
- **Red first:** the script was first committed with exactly the behavior of the old systemd unit; five of the eight tests fail against it. The failing output is in [manual-verification-evidence.md](manual-verification-evidence.md) section V5.
- **Guard tests proven:** the two tests that already passed against the old behavior (no restart while a job runs, no restart below the threshold) were proven by mutate-then-restore: removing the idle guard and removing the readiness wait each fail two tests.
- **Not covered by a maintained test:** the real systemd units, the real Docker restart, and the pinned state itself cannot be tested deterministically in this repository. They are covered by manual evidence (V4 for recovery, V6 for deployment and the first runs) and by the stub-based test for the decisions.

The red and green output and the final recheck are recorded in the same evidence file.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                                  | Notes / Expected Output                                                                                                                                       |
| --- | ------ | ----------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Diagnose what pins the build cache and reproduce the leak in a controlled way                         | Done: the pinned state is named (117 leaf records with Active snapshots held by BuildKit in dockerd) and the four-container hypothesis is rejected. Not done, deferred: reproducing the trigger on the runner stack (R1 in `deferred-proposals.md`). |
| T2  | DONE   | Restore free disk space on the runner, with the maintainer's approval for each step                   | Done 2026-10-27 10:10 to 10:11 UTC: `/` from 100% to 42% used, build cache 253.6 GB to 100 GB with 0 in use. States recorded in V4. No job was running. |
| T3  | DONE   | Select the regression-test boundary and write the maintained test                                     | Done: `test-docker-ci-prune.sh`, 5 of 8 tests red against the old behavior (commit `test(self-hosted-runner): [#2673] add failing tests for the runner Docker cleanup`). Output in V5. |
| T4  | DONE   | Make the daily cleanup release the orphaned build state                                               | Done: restart `docker.service` when the disk is still at or above 70% after the prune and no `Runner.Worker` runs, wait for Docker, prune again. 8 of 8 tests green (commit `fix(self-hosted-runner): [#2673] restart dockerd when the daily prune cannot free the disk`). |
| T5  | DONE   | Make a failed cleanup visible and decide the host Cargo target cache policy                           | Done: the script exits non-zero at or above 85%, so `systemctl --failed` lists the unit. Policy: not pruned automatically, last-resort deletion documented in the guide. Notifications deferred. |
| T6  | IN_PROGRESS | Update the runner guide, record the deferred proposals, and add the next health report            | Guide updated and `deferred-proposals.md` written. Open: the follow-up health report after deployment. |
| T7  | IN_PROGRESS | Deploy the script and verify green and recheck like-for-like                                          | Script installed on the runner with the maintainer's approval, `docker-ci-prune.service` run, and the first scheduled runs recorded in V6. Done: installed and run once by hand (V6). Open: the restart path on the runner and at least three scheduled daily runs, the first on 2026-10-28 04:00 UTC. The disk stays below 70% (or the restart path runs and works) across at least three daily runs. |

Follow the Required Sequence of the `fix-bug` skill: T3 red before T4, and T7 after the last production change.

## Commit Points

| Task | Coherent change set                                                                  | Commit policy                                        |
| ---- | ------------------------------------------------------------------------------------ | ---------------------------------------------------- |
| T1   | Diagnosis and reproduction evidence in `manual-verification-evidence.md`             | Commit after the evidence is complete.               |
| T3   | The failing regression test, with a design review recorded                           | Committed: `test(self-hosted-runner): [#2673] add failing tests for the runner Docker cleanup`. |
| T4   | The production fix that makes the regression test pass                               | Committed: `fix(self-hosted-runner): [#2673] restart dockerd when the daily prune cannot free the disk`. |
| T5   | The failure exit status and the Cargo target cache policy                            | Delivered with T4 (script) and T6 (policy text in the guide), because the exit status is part of the same script. |
| T6   | The updated runner guide, the deferred proposals, and the follow-up health report    | Commit after `linter all` passes.                    |

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
- 2026-10-27 10:12 UTC - GitHub Copilot (Claude Opus 5.5) - T1 diagnosis rejected the four-container hypothesis and named the pinned state; T2 recovered the runner with the maintainer's approval (`/` 100% to 42%, cache 253.6 GB to 100 GB, restart of `docker.service` released the pinned state); the leak trigger and the fix design remain open - [manual-verification-evidence.md](manual-verification-evidence.md)
- 2026-10-27 10:28 UTC - GitHub Copilot (Claude Opus 5.5) - Maintainer decided: fix the problem, no notifications and no scheduled workflows for now, write the proposals down. Implemented `docker-ci-prune.sh` with an idle-guarded dockerd restart and a failure exit above 85% (red tests first, then the fix), updated the runner guide, and wrote `deferred-proposals.md`. Not yet deployed to the runner - [manual-verification-evidence.md](manual-verification-evidence.md) V5
- 2026-10-27 10:56 UTC - GitHub Copilot (Claude Opus 5.5) - Deployed the script and the new unit to the runner with the maintainer's approval while it was idle, and ran the service once (success, disk 49%). The `Test (Docker)` job of the pull request for this issue passed on the runner. Pending: the restart path on the runner and three scheduled daily runs, the first on 2026-10-28 04:00 UTC - V6

## Acceptance Criteria

- [ ] AC1: The cause that keeps the Docker build cache from being reclaimed is identified with evidence: BuildKit inside dockerd holds orphaned build state that only a daemon restart releases (V2, V4). The trigger that creates that state is not proven and is deferred (see [deferred-proposals.md](deferred-proposals.md), R1).
- [ ] AC2: The daily cleanup releases that state without a person: when the disk is still at or above 70% after the prune and no job runs, it restarts `docker.service` and prunes again; it never restarts while a job runs. Shown by the maintained test and by a run on the runner (V5, V6).
- [ ] AC3: After deployment, the daily prune keeps the build cache near its 120 GiB target and the disk below 70%, or the restart path runs and brings it back, across at least three consecutive daily runs (V6).
- [ ] AC4: A cleanup that cannot keep the disk below 85% makes `docker-ci-prune.service` fail, so `systemctl is-failed docker-ci-prune.service` prints `failed`, and the runner guide says where to look. Notifications and scheduled workflows are out of scope.
- [ ] AC5: The host Cargo target cache has a documented cleanup policy.
- [ ] AC6: `docker-ci-prune.sh` has a maintained test, and `docs/self-hosted-runner.md` matches the behavior and no longer states that the disk absorbs the cache growth without evidence.
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
| M2  | Find what pins the cache and reproduce the trigger | Offline analysis of `docker buildx du --verbose` and `ctr -n moby snapshots ls`; a local cancelled build; a cancelled build on the runner | The pinned state is named, and a cancelled build on the runner stack leaves in-use state | BLOCKED | `manual-verification-evidence.md` section V2: state named, local substitute negative; the runner-stack reproduction is deferred (R1 in `deferred-proposals.md`) because the maintainer did not approve a test build on the production runner |
| M3  | Deploy the script and recheck after the last production change (T7) | Copy `docker-ci-prune.sh` to the runner with the maintainer's approval, update `docker-ci-prune.service`, run `systemctl start docker-ci-prune.service`, and read the first scheduled runs with `journalctl -u docker-ci-prune.service` and `systemctl --failed` | The unit succeeds, the disk stays below 70% or the restart path brings it back, and nothing is listed as failed, across three daily runs | IN_PROGRESS | `manual-verification-evidence.md` section V6: deployed and run once; three scheduled runs pending |
| M4  | Recover the runner with the maintainer's approval (T2)         | While the runner is idle: delete the host Cargo target cache, remove the four containers, `systemctl restart docker.service`, `systemctl start docker-ci-prune.service` | `/` is below 70% used and the build cache is reclaimable                                 | DONE   | `manual-verification-evidence.md` section V4 |

Notes:

- Every runner command is read-only except the ones in T2, which need the maintainer's explicit approval and run only while the runner is idle.
- Record the toolchain or runtime for each result that depends on one.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   | V2, V4: state named; trigger unproven and deferred |
| AC2   | TODO                   | V5 (tests); V6 (runner) pending |
| AC3   | TODO                   | V6       |
| AC4   | TODO                   | V5 (exit status tests); V6 pending |
| AC5   | DONE                   | `docs/self-hosted-runner.md`, Cache Cleanup |
| AC6   | TODO                   | Tests in V5 and the guide updated; the follow-up health report is pending |

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
