---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: 1840
github-issue: null
spec-path: docs/issues/drafts/1840-self-hosted-runner-instance-capacity/ISSUE.md
branch: "{issue-number}-1840-self-hosted-runner-instance-capacity"
related-pr: null
last-updated-utc: "2026-09-30 09:26"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/self-hosted-runner.md
    - docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
    - docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/benchmark-results.md
    - .github/workflows/container.yaml
    - .github/workflows/testing.yaml
    - src/console/ci/e2e/runner.rs
---

<!-- skill-link: create-issue -->

<!-- cspell:ignore cpuset cpus mpstat journalctl -->

# Issue #[To be assigned] - Scale Self-Hosted Runner Capacity for Concurrent Container Jobs

Parent EPIC: #1840 - Improve PR Workflow Performance

> **Deferred (2026-09-30).** The maintainer decided to keep the current single runner and not to
> open this issue now. See [Decision (2026-09-30)](#decision-2026-09-30) for the decisions and the
> revisit triggers. The rest of this draft is the plan to resume from; before opening it, align it
> with `docs/templates/ISSUE.md` and follow the `create-issue` skill.

## Decision (2026-09-30)

Decided by the maintainer after #2374 verified the runner's offline fallback:

1. **Keep one server with one runner instance.** `torrust-runner-01` (8 vCPU, 16 GB RAM, 16 GB
   swap) stays as it is. No second instance, second server, or resize now.
2. **Server size.** The current size is taken as the smallest that keeps the realistic PR job under
   15 minutes. This is an estimate from existing data, not a benchmark result: on 4 vCPU the
   workspace compile took 1068 to 1119 s on GitHub-hosted runners, giving about 21 minutes per
   job. Memory is also tight: on 2026-09-30 08:10 UTC a single running job used 14 GiB of 15 GiB
   RAM plus 4.5 GiB of swap. The Server Sizing benchmark below is not run unless a revisit trigger
   fires.
3. **Concurrency is not needed now.** The maintainer merges one pull request at a time, and each
   next pull request must be rebased and rerun anyway, so parallel container jobs save little.
   Demand is low: from 2026-09-28 17:00 to 2026-09-30 08:10 UTC the runner ran two real jobs,
   both queued for 1 to 2 seconds. The 31-minute queue in the Background evidence came from a
   single busy day.
4. **Resilience is covered by the fallback.** When the runner is down, jobs stay queued and a
   `runs-on: ubuntu-latest` change unblocks them; #2374 (M5) verified that the fallback starts on
   a GitHub-hosted runner within seconds. A second server is not needed for resilience.
5. **Capacity, when needed, means another server.** A second instance on the same server stays
   blocked until the Concurrency Blockers below are removed (proposed test-isolation EPIC).

Revisit triggers:

- A load recheck around 2026-10-30: queue time (`started_at - created_at`) of the self-hosted jobs
  since 2026-09-30, using the jobs API as in the Background evidence.
- Earlier, if load is expected to rise, for example when another maintainer starts working
  actively on the project (@da2ce7 was expected to start, with little activity this week).
- Earlier, if pull requests often wait for the runner in normal work.

## Goal

Run more than one self-hosted container job at a time while keeping each job under the 15-minute
target reached in #2323. Two goals must hold together:

1. Keep Docker build times low (do not go back to the 37-minute GitHub-hosted median).
2. Increase the number of jobs that can run concurrently.
3. Add resilience (maintainer, 2026-09-29): today one runner is a single point of failure. A second
   server covers both a failed runner process and a failed server. A second instance on the same
   server covers only a failed runner process. The `runs-on: ubuntu-latest` fallback remains for
   the case where every runner is down.

Find the cheapest runner layout that meets both: extra runner instances on the current server
(blocked today, see Current Limitation), or extra Hetzner servers of the smallest size that still
keeps a job under 15 minutes. Record the method so it can be repeated after a resize.

## Background

The runner work in #2323 deployed one self-hosted runner (`torrust-runner-01`, Hetzner CPX42 class:
8 vCPU, 16 GB RAM, 16 GB swap, 320 GB disk). The maintainer asked how many jobs it can run at once.

How concurrency works (clarified 2026-09-29):

- One runner process (`Runner.Listener`) runs **one job at a time**, on self-hosted and
  GitHub-hosted runners alike. GitHub-hosted jobs only look parallel because GitHub starts a new VM
  for every job.
- A self-hosted server runs jobs in parallel only if several runner instances are installed on it.
  Each instance has its own name, install directory, `_work` directory, and systemd service. They
  can share the `torrust-hetzner` label, and GitHub sends each queued job to an idle instance.
- Whether the server is a VM or bare metal makes no difference.
- Current limit: **1 concurrent self-hosted job**, because one instance is registered. Checked on
  2026-09-29: one `actions.runner.*` unit and one `Runner.Listener` process.

Constraints from the maintainer:

- One task at a time: this starts after #2323's pending work is merged.
- A second runner instance on the same server was the first preference. Because that is blocked
  (see Current Limitation), a second server is now acceptable, possibly with the current server
  replaced by smaller ones, as long as jobs stay under 15 minutes.
- A future server resize is possible, so the method must be repeatable.
- The 8 vCPU size was chosen partly on the assumption that one server could run several jobs at
  once. That assumption does not hold with the current tests, so the size should be re-evaluated.

Queue evidence (2026-09-28 06:53 to 17:00 UTC, 25 jobs on `torrust-runner-01`, from the jobs API,
`started_at - created_at`; raw data in `.tmp/runner-queue-jobs.tsv`):

- Queue time: median 2 s, 75th percentile 377 s, maximum 1861 s (31 min); 12 of 25 jobs waited
  more than 60 s. Jobs already wait for the single runner when several PRs are active.
- Job duration: 8 fully warm jobs of 130 to 140 s; 17 jobs with a workspace compile of 726 to
  1034 s (mean 806 s, about 13.4 min).
- Both goals are therefore real: the runner is a queue on busy days, and most jobs recompile.

Known facts that bound the expectation:

- The first single job was killed by the out-of-memory killer (8 parallel `rustc` processes on
  16 GB, no swap). A 16 GB swap file fixed it (#2323 T6 and `runner-server-setup.md` step 12).
  Two concurrent compiles may bring this back.
- Warm-cache `Test (Docker)` took 12 min 3 s, with a workspace compile of 561 s (#2323
  `benchmark-results.md`). This is the single-instance baseline to compare against.
- Idle snapshot, 2026-09-29 12:01 UTC: load 0.24; 14 GiB RAM available; 355 MiB swap used;
  142 GB disk free (51% used); Docker build cache 127.6 GB (122 GB reclaimable); prune timer active.

## Current Limitation

With the current tests, **only one runner instance per server is possible**. Two jobs on the same
Docker host interfere (blockers below). Until the test-isolation work in Related Work is done, the
only way to run jobs concurrently is another server, each server with a single runner instance.
Separate servers have separate Docker daemons, ports, and file systems, so the blockers do not
apply between them.

## Concurrency Blockers Found in the Code (2026-09-29)

These make a second instance on the same server unsafe until they are resolved. #2323 Scenario E
predicted them.

1. **Fixed host ports.** `src/console/ci/e2e/runner.rs` publishes `6969/udp`, `7070`, `1212`, and
   `1313` on the host. Two concurrent `Run E2E Tests` steps would fail to bind. (The qBittorrent
   Compose stacks are fine: they use host port `0` and random project names.)
2. **Shared image tag.** `container.yaml` loads the built image as `torrust-tracker:local`, and the
   E2E steps run that tag. Both instances use the same Docker daemon, so job A can test the image
   built by job B. This is a silent correctness risk, not just a failure risk.
3. **Shared Cargo target directories.** Both instances run as the `runner` user, and the jobs set
   `CARGO_TARGET_DIR=$HOME/.cache/torrust-tracker/container-target` (and `docker-e2e-target`).
   Cargo's lock serializes concurrent builds on one target directory. That is safe, but the jobs
   wait for each other and the measurement is skewed.

Still to check: any other fixed container names, networks, volumes, or `/tmp` paths in the E2E
steps. The persistence regression script uses `mktemp` and `--rm`, but still uses the
`torrust-tracker:local` tag passed by the workflow.

## Related Work

- #1419 - Allow multiple integration tests at the main app level: the same problem for the
  integration tests (per-test config and storage, port `0` with endpoint discovery, no shared
  environment variables).
- Proposed new EPIC (to draft and open later, not part of this issue): **make tests safe to run in
  shared and parallel environments**. Children would be:
  - E2E and workflow tests that can run concurrently on one Docker host (the three blockers
    above);
  - #1419, integration tests that can run in parallel.
- Once that EPIC removes the blockers, adding runner instances on one server becomes possible, and
  it is usually better than several small servers with the same total CPU: a single job gets all
  the cores when it runs alone, and the servers share one cache. The limit then becomes RAM (two
  concurrent compiles on 16 GB) rather than isolation.

## Server Sizing

Question: what is the smallest Hetzner server that keeps a realistic PR job under 15 minutes?

What the evidence already says:

- The realistic PR case is a warm-cache job with application code changed: 723 s on 8 vCPU
  (#2323 run `36310358131`). Of that, the workspace compile is 561 s and CPU-bound; the other
  162 s (E2E, persistence regression, setup) barely depend on CPU count.
- The budget for the compile is therefore about 900 - 162 = 738 s. On fewer cores, the compile may
  slow down by at most about 1.3x.
- The GitHub-hosted runners have 4 vCPU, and their workspace compile took 1068 to 1119 s (#2323
  T1). That suggests a 4 vCPU server would take about 1090 + 162 = 1250 s (about 21 min) per job,
  over the target. The CPU generations differ, so this is an estimate, not a result.
- The 16 GB of RAM matters as much as the cores: the first job hit the OOM killer on 16 GB without
  swap. Smaller types also have less RAM.

Measurement method (cheapest first):

1. **Benchmark protocol.** On a fixed commit: build once to warm the dependency layers, apply a
   small application-code change, then time `docker buildx build --target release` (the
   realistic PR case). Repeat 3 times per size. Record the build time, the `build 3/3` compile
   time, peak memory, and swap use. Add the measured 162 s of non-build steps to get the job time.
   Run the same protocol on the current 8 vCPU server first, so every size is compared the same
   way.
2. **Emulate smaller sizes on the current server (free).** Use a BuildKit builder limited to part
   of the machine, for example
   `docker buildx create --name bench-4cpu --driver docker-container --driver-opt cpuset-cpus=0-3 --driver-opt memory=8g`,
   and repeat for 4 and 6 CPUs. Run it only while the runner is idle (or in an agreed maintenance
   window), because it competes with CI jobs. This narrows the candidates.
3. **Confirm on a real server (a few cents).** Hetzner bills by the hour: create the best
   candidate type, run the protocol, and delete the server. It needs no runner registration or
   GitHub access, so it does not touch the security model. This also captures the effect of a
   different CPU generation and of shared vCPU neighbors.
4. **Decide.** Compare total monthly cost and capacity of the options: one 8 vCPU server (today),
   two servers of the smallest passing size, or two 8 vCPU servers. Check current prices in the
   Hetzner console when deciding.

## Scope

### In Scope

- Benchmark the realistic PR build at several server sizes (Server Sizing) and find the smallest
  size that keeps a job under 15 minutes.
- Choose a runner layout from the evidence: keep one server, add a second server, or replace the
  current server with smaller ones. Each server runs one runner instance.
- If a server is added: set it up with `docs/self-hosted-runner.md` (same `torrust-hetzner` label,
  a new runner name), then measure queue time again against the 2026-09-28 evidence.
- Document the chosen layout, the sizing method, and the one-instance-per-server limit in
  `docs/self-hosted-runner.md`.

### Deferred Until the Test-Isolation EPIC Is Done

The multi-instance plan below (Concurrency Blockers, Measurement Method, and the instance-count
experiment) applies only after the blockers are removed by the proposed test-isolation EPIC. Keep
it here as the plan for that later step.

### Out of Scope

- Removing the concurrency blockers and #1419 (the proposed test-isolation EPIC).
- Autoscaling, ephemeral runners, or runner-controller tooling.
- Changes to the security routing (Dependabot, `main`, `releases/**`, fork approval).
- Optimizing the build itself (other #1840 subissues).
- The unit-test critical path (#2323 Scenario G), which is a separate #1840 subissue.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`
  (one instance for now; add instances only when measured queue time requires it).
- ADRs to create: probably none. Update the ADR if the supported instance count or the
  isolation model changes materially.

## Design and Ownership Review

- Per-job isolation: every host-level resource a job creates (image tags, container names, host
  ports, Compose projects, temporary directories, Cargo target directories) must be unique per
  runner instance or per job. Candidate keys: `runner.name` or `github.run_id`/`github.run_attempt`.
- Shared resources that stay shared on purpose: the Docker daemon, the BuildKit cache, and the
  Cargo registry. Record whether sharing them is safe for concurrent use.
- Ownership: each runner instance has its own systemd unit and its own `Restart=on-failure`
  drop-in (`docs/self-hosted-runner.md` step 12).
- Deadline: no new readiness waits are expected. Keep the existing `timeout-minutes`.

## Bug-Fix Process

Not applicable. This is capacity measurement plus the isolation work it needs.

## Regression Test Strategy

Not applicable. The isolation changes are verified by concurrent runs on the real runner.

## Measurement Method

Deferred: multiple instances on one server, after the test-isolation EPIC. For each instance
count N:

- Trigger N disposable non-documentation PRs (or one PR with N re-runs) so that N `Test (Docker)`
  jobs start within about one minute of each other. Record the start skew.
- Use comparable cache states (warm cache with application code changed) so that runs at
  different N can be compared.
- Sample host metrics during the run: CPU (load and `mpstat`), memory and swap (`free`, `vmstat`,
  swap in and out), disk I/O, disk usage, `docker system df`, and `journalctl -k` for OOM kills.
- Record for each job: runner name, run and job URL, queue time, total duration, `Build Tracker
  Image` duration, workspace compile time, E2E step durations, and the result.

Pass criteria for N (proposed; the maintainer adjusts them before execution):

- All N jobs pass, and there is no OOM kill or runner-service failure.
- No sustained swap thrashing (continuous swap in and out during the compile).
- The median job duration is at most X% slower than the N = 1 baseline (X to be decided, for
  example 25%).
- Disk usage stays under the prune policy's limits.

Rollback: stop, uninstall, and deregister the extra instance
(`gh api -X DELETE repos/torrust/torrust-tracker/actions/runners/<runner-id>`).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                | Notes / Expected Output                                                                                                                 |
| --- | ------ | ----------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Write the benchmark protocol        | A repeatable script for the warm-cache, code-changed build (Server Sizing step 1); disposable unless it proves reusable.                |
| T2  | TODO   | Baseline on the current server      | Protocol results on 8 vCPU / 16 GB, run while the runner is idle.                                                                      |
| T3  | TODO   | Emulate smaller sizes               | Protocol results with limited builders (for example 6 and 4 CPUs, matching RAM limits); narrows the candidate types.                   |
| T4  | TODO   | Confirm on a real candidate server  | Needs maintainer approval: hourly Hetzner server, protocol only, no runner registration; deleted afterwards.                           |
| T5  | TODO   | Choose and apply the layout         | Needs maintainer approval: cost and capacity comparison; set up any new server with `docs/self-hosted-runner.md`; measure queue time again. |
| T6  | TODO   | Document the result                 | `docs/self-hosted-runner.md`: layout, one instance per server limit, sizing method, and how to repeat it after a resize.               |

## Acceptance Criteria

- [ ] AC1: The realistic PR build (warm cache, application code changed) is measured on the current
      server and at each candidate size, with build time, compile time, peak memory, and swap.
- [ ] AC2: The smallest size that keeps the job under 15 minutes is identified, or the evidence
      shows that none smaller than today's does.
- [ ] AC3: The chosen runner layout is justified by cost, job time, and queue time, and any new
      server passes the `docs/self-hosted-runner.md` verification.
- [ ] AC4: `docs/self-hosted-runner.md` records the one-instance-per-server limit, the chosen
      layout, and how to repeat the sizing after a resize.
- [ ] `linter all` exits with code `0`

## Open Questions

1. Is a single job allowed to exceed 15 minutes on a smaller server if the queue time saved makes
   the total PR wait shorter? (The 2026-09-28 maximum queue was 31 minutes.)
2. With two servers, each keeps its own cache, so the first job per server after a dependency
   change pays the cold cost twice. Is that acceptable?
3. Whether `Docker E2E` in `testing.yaml` (feature-branch pushes) must count as load, since it
   shares the label.
4. For the deferred multi-instance step: the accepted slowdown threshold X, how to trigger N
   concurrent runs reproducibly, and whether to cap `CARGO_BUILD_JOBS` per instance.

## Progress Log

- 2026-09-29 12:35 UTC - GitHub Copilot - Local draft created from the 2026-09-29 discussion; deferred until the pending #2323 operational verification is merged - this file
- 2026-09-29 14:07 UTC - GitHub Copilot - Recorded that only one instance per server is possible with the current tests, linked #1419 and the proposed test-isolation EPIC, added the 2026-09-28 queue evidence and the server-sizing method, and refocused the plan on server sizing - this file
- 2026-09-29 15:30 UTC - GitHub Copilot - Added resilience as a third goal: a second server also removes the single point of failure - this file
- 2026-09-30 09:26 UTC - josecelano, GitHub Copilot - Decided to keep one runner on the current server, deferred this issue, and recorded the revisit triggers; moved the draft from `.tmp/` to `docs/issues/drafts/` - [Decision (2026-09-30)](#decision-2026-09-30)
