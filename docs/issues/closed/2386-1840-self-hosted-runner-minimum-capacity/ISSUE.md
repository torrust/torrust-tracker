---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 1840
github-issue: 2386
spec-path: docs/issues/closed/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
branch: "2386-1840-runner-capacity-close-out"
related-pr: 2403
last-updated-utc: "2026-10-02 13:30"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/self-hosted-runner.md
    - docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/benchmark-results.md
    - .github/workflows/container.yaml
    - docs/issues/closed/2374-1840-verify-self-hosted-runner-operational-controls/ISSUE.md
    - docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md
    - .github/workflows/testing.yaml
    - src/console/ci/e2e/runner.rs
---

<!-- skill-link: create-issue -->

<!-- cspell:ignore cpuset cpus journalctl -->

# Issue #2386 - Determine the Minimum Self-Hosted Runner Capacity

Parent EPIC: #1840 - Improve PR Workflow Performance

## Goal

Decide the minimum capacity of the self-hosted runner setup for the current workload: the smallest
server size that keeps the realistic pull-request `Test (Docker)` job under 15 minutes, and how
many runners are needed. Measure the size claim, record the decision and when to recheck it, and
update the operations guide. The output is documentation; no workflow or server change is planned.

## Background

Issue #2323 deployed one self-hosted runner, `torrust-runner-01` (Hetzner CPX42 class: 8 vCPU,
16 GB RAM, 16 GB swap, 320 GB disk, 69.49 EUR per month). The 8 vCPU size was chosen partly on the
assumption that one server could run several jobs at once. That assumption does not hold with the
current tests (see Concurrency Limits), so the size needs its own justification.

How concurrency works:

- One runner process runs one job at a time, on self-hosted and GitHub-hosted runners alike.
  GitHub-hosted jobs only look parallel because GitHub starts a new VM for every job.
- A server runs jobs in parallel only with several runner instances. One instance is registered
  today, so at most one self-hosted job runs at a time.

Existing evidence:

- Job time: the realistic pull-request case (warm cache, application code changed) took 723 s on
  8 vCPU: 561 s for the CPU-bound workspace compile and 162 s for the E2E, regression, and setup
  steps (#2323 run `36310358131`, `benchmark-results.md`).
- 4 vCPU estimate: GitHub-hosted runners have 4 vCPU, and their workspace compile took 1068 to
  1119 s (#2323 T1), suggesting about 1250 s (21 minutes) per job. The CPU generations differ, so
  this is an estimate, not a measurement.
- Memory: the first job was killed by the out-of-memory killer on 16 GB without swap; 16 GB of swap
  fixed it. On 2026-09-30 08:10 UTC a single running job used 14 GiB of 15 GiB RAM plus 4.5 GiB of
  swap. Smaller server types also have less RAM.
- Queue time (`started_at - created_at`, jobs API): on 2026-09-28 from 06:53 to 17:00 UTC, 25 jobs
  had a median of 2 s and a maximum of 31 minutes, and 12 waited more than 60 s. From 2026-09-28
  17:00 to 2026-09-30 08:10 UTC, the runner ran two real jobs, both queued for 1 to 2 s.
- Resilience: when the runner is down, jobs stay queued and a `runs-on: ubuntu-latest` change
  unblocks them; #2374 (M5) verified that the fallback starts on a GitHub-hosted runner within
  seconds.
- Cost: the server is a flat monthly cost, paid while idle. Against #2323's larger-runner estimate
  (about 0.77 USD per `Container` run), it is cheaper above about 100 runs per month; the 30 days
  before 2026-09-30 had 591 (see the Cost section of `docs/self-hosted-runner.md`).

Maintainer inputs (2026-09-30):

- The maintainer merges one pull request at a time, and each next pull request is rebased and
  rerun anyway.
- Correction, same day: 2026-09-29 and 2026-09-30 were not normal days. Normally the maintainer
  works with 6 AI agents and has about 4 pull requests open, and review rework (Copilot and
  @da2ce7) runs each pull request's checks 3 to 5 times. Merging one at a time therefore does not
  serialize the checks: every push to any open pull request starts a `Test (Docker)` job. The
  30-day data agrees (`manual-verification-evidence.md` V3): median 4 `Container` runs per branch.
- The verified fallback is enough resilience; a second server is not needed for availability.
- Recheck the load in about a month, or earlier if it is expected to rise, for example when another
  maintainer works actively on the project (@da2ce7 was expected to start, with little activity
  this week).

## Concurrency Limits

With the current tests, only one runner instance per server is safe. Two jobs on one Docker host
interfere:

1. **Fixed host ports.** `src/console/ci/e2e/runner.rs` publishes `6969/udp`, `7070`, `1212`, and
   `1313` on the host, so two concurrent `Run E2E Tests` steps cannot both bind. The qBittorrent
   Compose stacks use host port `0` and random project names, so they do not conflict.
2. **Shared image tag.** `container.yaml` loads the built image as `torrust-tracker:local`, and the
   E2E steps run that tag, so one job can test another job's image.
3. **Shared Cargo target directories.** Both instances would run as the `runner` user with the same
   `CARGO_TARGET_DIR`, so their builds serialize on Cargo's lock.

Other fixed container names, networks, or `/tmp` paths were not audited. Separate servers have
separate Docker daemons, ports, and file systems, so these limits do not apply between them.
Removing them belongs to the test-isolation EPIC #2392, together with #1419 for the integration
tests.

## Scope

### In Scope

- Measure the realistic pull-request build on the current server at full size (8 vCPU, 16 GB) and
  constrained to the next smaller Hetzner shared-vCPU size (4 vCPU, 8 GB), with one protocol.
- Record the load evidence and the maintainer's workflow.
- Decide and record the minimum capacity: server size, number of runners, resilience, and recheck
  triggers.
- Update `docs/self-hosted-runner.md`: one runner per server and why, how to add capacity, and how
  to recheck the load.
- Add the missing EPIC #1840 row for #2374 and a row for this issue.

### Out of Scope

- Resizing the server, adding a server, or adding a runner instance. If the measurement favors
  another size, the change is a follow-up that uses the guide's rebuild procedure.
- Removing the concurrency limits (test-isolation EPIC #2392, and #1419).
- Autoscaling, ephemeral runners, or runner-controller tooling.
- Changes to the security routing (Dependabot, `main`, `releases/**`, fork approval).
- Optimizing the build itself, and the unit-test critical path (#2323 Scenario G), which are other
  #1840 work.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
  (one runner instance; add instances only when measured queue time requires it).
- ADRs to create: none expected. The ADR's rationale for one instance covers CPU and memory but not
  the test-isolation limits; the guide records those. Revisit the ADR only if the decision changes
  the server choice materially.

## Design and Ownership Review

No code is involved. The measurement uses the shared runner host:

- The maintainer approves the measurement window and stops and restarts the runner service. The
  agent runs the commands through the maintainer's SSH agent and records the results.
- The runner is stopped during the window so CI jobs do not compete with the measurement. Pull
  requests opened meanwhile queue and resume on restart, as #2374 (M5) verified. The window ends
  with the runner `online` and idle.
- The benchmark builder and its cache are removed afterwards.

## Bug-Fix Process

Not applicable. This is a capacity decision, not a defect.

## Regression Test Strategy

Not applicable. No code or test changes.

## Measurement Method

One BuildKit builder is used for both sizes, so both builds start from the same warm cache:

1. On the runner host, check out a fixed `develop` commit and create a builder:
   `docker buildx create --name capacity-bench --driver docker-container`.
2. Warm it by building the `release` target once, without limits.
3. Full size: apply a small application-code change, then time
   `docker buildx build --builder capacity-bench --target release .`. Record the total build time,
   the `build 3/3` workspace compile time, peak memory and swap (sampled with `vmstat`), and any
   out-of-memory kill (`journalctl -k`).
4. 4 vCPU, 8 GB: limit the builder container with
   `docker update --cpuset-cpus 0-3 --memory 8g --memory-swap 24g <builder-container>` (8 GB of RAM
   plus the guide's 16 GB of swap), apply a different small change, and time the same build.
5. Projected job time for each size: build time plus the 162 s of non-build steps.

The constrained run approximates a 4 vCPU server but does not reproduce its CPU generation or
shared-vCPU neighbors. A real candidate server is used only if the result is close to the limit
(Open Question 2).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                          | Notes / Expected Output                                                                                                             |
| --- | ------ | ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Measure both sizes            | Measurement Method in a maintainer-approved window; results in `manual-verification-evidence.md` V1 and V2; runner restored (V4).   |
| T2  | DONE   | Record the load evidence      | `manual-verification-evidence.md` V3: observed self-hosted queue times, and a replay of the last 30 days of real `Container` run arrivals through 1 and 2 runners, with and without cancelling superseded runs. The observed window is extended to the measurement date after T1. |
| T3  | DONE   | Record the capacity decision  | Maintainer decision on server size, runner count, resilience, and recheck triggers, in a Decision section of this specification.    |
| T4  | DONE   | Update the operations guide   | `docs/self-hosted-runner.md`: current server facts, the cost trade-off, one runner per server and why, how to add capacity, and recheck commands for queue time and run volume. |
| T5  | DONE   | Update EPIC #1840             | Row 16 for #2374 (DONE) and row 17 for #2386 (IN_PROGRESS).                                                                                      |

## Commit Points

| Task   | Coherent change set                                   | Commit policy                                   |
| ------ | ----------------------------------------------------- | ----------------------------------------------- |
| T1, T2 | `manual-verification-evidence.md` V1 to V4             | Commit after the maintainer reviews the results. |
| T3     | Decision section of this specification                | Commit after maintainer approval.               |
| T4     | `docs/self-hosted-runner.md` capacity section          | Commit after maintainer review.                 |
| T5     | EPIC #1840 rows and progress log                      | Commit after focused validation.                |

Use Conventional Commits (`docs(issues)`, `docs(self-hosted-runner)`) and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/1840-self-hosted-runner-minimum-capacity/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Issue linked as a GitHub sub-issue of EPIC #1840
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, pre-commit checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-29 12:35 UTC - GitHub Copilot - Local draft created from the 2026-09-29 discussion; deferred until the pending #2323 operational verification is merged - this file
- 2026-09-29 14:07 UTC - GitHub Copilot - Recorded that only one instance per server is possible with the current tests, linked #1419 and the proposed test-isolation EPIC, added the 2026-09-28 queue evidence and the server-sizing method, and refocused the plan on server sizing - this file
- 2026-09-29 15:30 UTC - GitHub Copilot - Added resilience as a third goal: a second server also removes the single point of failure - this file
- 2026-09-30 09:26 UTC - josecelano, GitHub Copilot - Decided to keep one runner on the current server, deferred this issue, and recorded the revisit triggers; moved the draft from `.tmp/` to `docs/issues/drafts/` - PR #2383
- 2026-09-30 09:41 UTC - josecelano, GitHub Copilot - Changed the scope to determining the minimum capacity, with documentation as the output: the maintainer chose to open and close an issue instead of keeping a deferred draft; added a measurement of the 4 vCPU size; the multi-instance plan moves to a future test-isolation EPIC - this file
- 2026-09-30 10:26 UTC - josecelano, GitHub Copilot - Maintainer approved the spec; created #2386, linked it as a sub-issue of #1840, and moved the spec to `docs/issues/open/`. The work continues in PR #2383, which already carries this spec and the guide changes; its branch keeps the name `1840-record-runner-capacity-decision` because it was opened before the issue existed - #2386, PR #2383
- 2026-09-30 10:35 UTC - GitHub Copilot - Checked the daily Docker prune, suspected of not bounding the build cache. It works as configured: `--max-used-space 120GB` means 120 GiB, the 04:00 run on 2026-09-29 reclaimed 50.29 GB, and a manual run of the same command reclaimed 62.08 GB (187.6 GB to 125.5 GB). The 0 B run on 2026-09-30 found the cache below the cap. The guide said "caps" and now says the timer trims once a day and the cache overshoots between runs. The manual run started while a Container job for PR #2382 was building: the idle check and the prune ran in one command, and the runner picked up that job between them - guide "Prune Docker Storage Daily", run `36702472251`
- 2026-09-30 10:52 UTC - GitHub Copilot - T4 DONE: added the queue-time recheck command to the guide's "Add Runner Capacity" section. Its first run shows a busier morning than the Background window: from 08:03 to 10:27 UTC, 9 self-hosted jobs, 4 of them queued for 124 to 760 s; T2 records the full window after T1 - guide "Add Runner Capacity"
- 2026-09-30 11:04 UTC - josecelano, GitHub Copilot - The maintainer corrected the workload assumption: the last two days were not normal (normally 6 agents, about 4 open pull requests, checks run 3 to 5 times each). Replaying the last 30 days of real `Container` run arrivals shows one runner queues heavily (p90 wait 99 min at a 723 s job) and that cancelling superseded runs helps more than a second runner alone; the "concurrency is not needed" conclusion is withdrawn, and the choice is Open Question 4 - `manual-verification-evidence.md` V3
- 2026-09-30 12:25 UTC - josecelano, GitHub Copilot - Moved the work to branch `2386-1840-self-hosted-runner-minimum-capacity`, rebased onto `develop`, and opened PR #2389, which supersedes #2383: the old branch was named after the EPIC, and GitHub closes a pull request when its head branch is renamed. The #2383 review audit carries over - PR #2389
- 2026-09-30 12:33 UTC - josecelano, GitHub Copilot - PR #2389 merged the spec, guide, and V3 evidence. GitHub closed #2386 on the merge: the merge tool copies the pull request body into the merge commit, and the body's sentence "The link will change to `Closes #2386` when the remaining tasks are done" contains a closing keyword. The issue was reopened because T1 and T3 remain. The rest continues on branch `2386-1840-runner-capacity-measurements` - PR #2389, #2386
- 2026-09-30 15:17 UTC - josecelano, GitHub Copilot - T1 and T2 DONE. In a window from 13:34 to 14:58 UTC the realistic build took 639 s at 8 vCPU / 16 GB (projected job 801 s, 13.4 minutes) and 1065 s constrained to 4 vCPU / 8 GB (projected 1227 s, 20.5 minutes); no out-of-memory kills. The replay rerun at 801 s gives a one-runner 90th-percentile wait of 140 minutes, 12 with superseded runs cancelled. The maintainer kept the current 8 vCPU / 16 GB server as the minimum; the queueing remedy is still open - `manual-verification-evidence.md` V1 to V4, Decision
- 2026-10-01 14:52 UTC - josecelano, GitHub Copilot - T3 DONE: the maintainer chose to cancel superseded pull-request runs before adding a second server, to be delivered in #2402, and kept the recheck trigger of about a month or another active maintainer - Decision, #2402
- 2026-10-01 16:26 UTC - josecelano, GitHub Copilot - Addressed the Copilot review of PR #2403. The first cancelling replay only dropped runs superseded within one job time of their arrival and treated cancelled running jobs as free; an event-driven replay corrects the one-runner figures at 801 s to 11 minutes (90th percentile) and 32 minutes (maximum), from 12 and 79, which strengthens the decision. The V2 memory claim is narrowed to the sampled last 5 minutes, and the Decision dates its parts - `manual-verification-evidence.md` V2, V3, `docs/pr-reviews/pr-2403-review/PR-REVIEW.md`
- 2026-10-02 07:11 UTC - GitHub Copilot - PR #2403 merged. Started the close-out on branch `2386-1840-runner-capacity-close-out`: recorded the implementation retrospective, checked the implementation and verification checkpoints, and added the closing-keyword rule it identified to the `open-pull-request` skill; the independent review follows - `implementation-retrospective.md`
- 2026-10-02 07:47 UTC - GitHub Copilot - The Task Reviewer passed the close-out with four Low findings, all fixed: PR #2403 is now cited in the acceptance verification, References, and EPIC row 17; the evidence `last-updated-utc` is corrected; and the guide's "Add Runner Capacity" now puts cancelling superseded runs (#2402) before a second server and links #2386 by issue instead of by its `docs/issues/open/` path - `agent-review-reports.md`
- 2026-10-02 13:30 UTC - GitHub Copilot - PR #2405 merged and closed #2386. Archived this spec to `docs/issues/closed/` and updated the references in EPIC #1840 and in the frontmatter of EPIC #2392 and its sub-issues #2393 to #2396 - this folder

## Acceptance Criteria

- [x] AC1: The realistic pull-request build is measured with one protocol at 8 vCPU / 16 GB and at
      4 vCPU / 8 GB, recording total build time, workspace compile time, peak memory and swap, and
      any out-of-memory kill.
- [x] AC2: The projected job time for each size is compared with the 15-minute target, and the
      minimum size is stated with its evidence.
- [x] AC3: The capacity decision (server size, number of runners, resilience, and recheck triggers)
      is recorded with the maintainer's rationale.
- [x] AC4: `docs/self-hosted-runner.md` records the current server's facts (without its IP address),
      the cost trade-off against GitHub larger runners with the break-even volume, the
      one-runner-per-server limit and its causes, how to add capacity, and commands to recheck
      queue time and run volume.
- [x] AC5: EPIC #1840 lists #2374 and this issue.
- [x] `linter all` exits with code `0`
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario             | Human-oriented command/steps                                  | Expected Result                                                   | Status | Evidence                                     |
| --- | -------------------- | ------------------------------------------------------------- | ----------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Full-size build      | Measurement Method steps 1 to 3                               | Build and compile times, peak memory and swap recorded            | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | 4 vCPU / 8 GB build  | Measurement Method step 4                                     | Same metrics recorded; any out-of-memory kill noted               | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Queue-time recheck   | Run the recheck command added to `docs/self-hosted-runner.md` | Lists self-hosted jobs with their queue times                     | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | Runner restored      | Runners API after the window                                  | `torrust-runner-01` `online` and idle; benchmark builder removed | DONE   | `manual-verification-evidence.md` section V4 |

### Disposable Verification Scripts

None. The commands are recorded directly in `manual-verification-evidence.md`.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | DONE                   | `manual-verification-evidence.md` V1 and V2, PR #2403 |
| AC2   | DONE                   | V1, V2, and the Decision section, PR #2403 |
| AC3   | DONE                   | Decision section, PR #2403 |
| AC4   | DONE                   | `docs/self-hosted-runner.md`, PR #2389 |
| AC5   | DONE                   | EPIC #1840 rows 16 and 17, PR #2389 |

## Decision

Measurements from 2026-09-30 in `manual-verification-evidence.md`. The maintainer decided the server
size on 2026-09-30 and the runner count and queueing on 2026-10-01.

- **Server size (2026-09-30): keep the current 8 vCPU / 16 GB server (CPX42 class).** It is the smallest size
  that meets the target: its projected job is 801 s (13.4 minutes), while 4 vCPU / 8 GB projects
  1227 s (20.5 minutes). The maintainer's rationale: the job is dominated by heavy work, the Docker
  build and the Rust compilation, so a smaller server is not an option. The margin is small, 1.6
  minutes and about 123 MiB of free memory, and the compile has grown from 561 s to 614 s since
  #2323.
- **Resilience:** the verified `ubuntu-latest` fallback is enough; see Background.
- **Runner count and queueing (2026-10-01): one server; cancel superseded pull-request runs first.**
  At 801 s on one runner, cancelling cuts the 90th-percentile wait from 140 to 11 minutes and the
  longest from 239 to 32, against 13 and 63 minutes for a second server alone (V3), at no cost. It
  is a workflow change, to be delivered in #2402 under EPIC #1840. A second server is added only if
  the recheck after that shows waits over 15 minutes are still common.
- **Recheck triggers:** about a month after cancellation is in place, or earlier if the load is
  expected to rise, for example when another maintainer works actively on the project (maintainer
  input in Background). Use the queue-time and run-volume commands in the guide.

## Risks and Trade-offs

- The constrained build is an approximation of a real 4 vCPU server. Mitigation: a result near the
  limit is treated as inconclusive (Open Question 2).
- Stopping the runner delays pull requests during the window. Mitigation: a quiet window approved
  by the maintainer; queued jobs resume on restart.
- One run per size can be noisy. Mitigation: the 4 vCPU estimate from GitHub-hosted runners gives
  an independent cross-check.

## Open Questions

1. If 4 vCPU / 8 GB stays under 15 minutes, downsize (about half the monthly cost, in a follow-up)
   or keep 8 vCPU for headroom? Answered by V2: it does not stay under 15 minutes (20.5 minutes).
2. How close to 15 minutes counts as inconclusive and needs a real candidate server? Proposed:
   within 10% (13.5 to 16.5 minutes). Not needed: V2 is well outside that band.
3. When is the measurement window? 2026-09-30, 13:34 to 14:58 UTC (V4).
4. The 30-day replay (`manual-verification-evidence.md` V3) shows one runner queues a normal
   workload heavily. Which remedy, decided in T3 and delivered as a follow-up: cancel superseded
   pull-request runs (a `concurrency` group with `cancel-in-progress`, no extra cost), add a second
   server (about 69 EUR per month more, and less cost-efficient at low volume), or both? Decided:
   cancel superseded runs first, in #2402; see Decision.
5. Does a second server change the answer to question 1, for example two 4 vCPU servers instead
   of one 8 vCPU server? No: V2 shows a 4 vCPU server misses the 15-minute job target on its own,
   whatever the number of servers.

## Implementation Completion Review

- Retrospective: [implementation-retrospective.md](implementation-retrospective.md) (material
  discoveries: workload model, queue-replay model, memory evidence, window mechanics, and the
  closing keyword in a PR body)
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise add a progress-log
  entry explaining why not.
- An independent reviewer records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #1840
- Related issues: #2323, #2374, #1419
- Related PRs: #2389 (spec, guide, first queue evidence); #2403 (measurements and decision); #2383
  (superseded by #2389, branch named after the EPIC)
- Related ADRs: `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
