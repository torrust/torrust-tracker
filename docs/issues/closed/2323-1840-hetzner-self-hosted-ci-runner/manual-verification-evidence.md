---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
last-updated-utc: 2026-09-29
---

# Manual Verification Evidence

<!-- cspell:ignore journalctl -->

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-27
- Artifact under test: PR #2352, with `ci(container): run the container test job on the Hetzner
  runner` and `docs(adrs): cover Dependabot branch pushes in the runner ADR`
- Operating system / environment: `torrust-runner-01` (Hetzner, 8 vCPU, 16 GB RAM, Ubuntu 26.04.1,
  runner `v2.337.0`, labels `self-hosted,Linux,X64,torrust-hetzner`); GitHub-hosted runners for the
  other jobs
- Prerequisites and setup performed: `runner-server-setup.md` steps 1-12 and
  `runner-agent-installation.md` steps 1-8

## Verification Processes

### V1 - PR Run on the Self-Hosted Runner (M1)

- Goal: a PR targeting `develop` runs `Test (Docker)` on the Hetzner runner and passes.
- Initial state: first job ever on the runner; no Rust toolchain, Docker layers, or Cargo caches
  on the host.
- Status: `DONE`

#### Steps Performed

1. Opened PR #2352 from `josecelano:2323-1840-hetzner-self-hosted-ci-runner` into `develop`.
2. Inspected the jobs of `Container` run `36298632207`:
   `gh api repos/torrust/torrust-tracker/actions/runs/36298632207/jobs`.
3. Attempt 1 failed (see Failures and Follow-up). After the fix, re-ran the failed job:
   `gh run rerun 36298632207 --repo torrust/torrust-tracker --failed`.

#### Observed Result

Attempt 2, <https://github.com/torrust/torrust-tracker/actions/runs/36298632207/job/108565752883>:

```text
Test (Docker) (release) | success | runner=torrust-runner-01 | 2026-09-27T06:24:22Z -> 2026-09-27T06:38:53Z
Context | success | runner=GitHub Actions 1000090371
Publish (Release) | skipped
Publish (Development) | skipped
  Build Tracker Image: 06:24:34Z -> 06:34:43Z
  Run Persistence Transition Regression: 06:34:43Z -> 06:35:25Z
  Run E2E Tests: 06:35:25Z -> 06:37:11Z
  Run qBittorrent E2E Test (SQLite): 06:37:11Z -> 06:37:37Z
  Run qBittorrent E2E Test (MySQL): 06:37:37Z -> 06:38:17Z
  Run qBittorrent E2E Test (PostgreSQL): 06:38:17Z -> 06:38:46Z
```

The `Testing` workflow's `Docker E2E` job was `skipped`, as expected for a PR targeting `develop`.

#### Conclusion

M1 met: the job ran on `torrust-runner-01` and passed in 14 min 31 s (baseline median 37 min).
The run was not fully cold: attempt 1 had already installed the toolchain, downloaded the crates,
and built about six minutes of Docker layers. T7 records the warm-cache comparison.

### V2 - Develop Push Run (M2)

- Goal: a merge into `develop` runs `Test (Docker)` on the Hetzner runner and publishes the
  development image successfully.
- Initial state: PR #2352 merged into `develop` as `432d4e69`.
- Status: `DONE`

#### Steps Performed

1. Merged PR #2352 into `develop`.
2. Waited for `Container` run `36345987166` with
  `gh run watch 36345987166 --repo torrust/torrust-tracker --interval 60 --exit-status`.
3. Inspected its jobs with
  `gh api repos/torrust/torrust-tracker/actions/runs/36345987166/jobs`.

#### Observed Result

<https://github.com/torrust/torrust-tracker/actions/runs/36345987166>:

```text
Test (Docker) (release) | success | runner=torrust-runner-01 | 19:53:18Z -> 19:55:35Z
Publish (Development) | success | runner=GitHub Actions 1000090528 | 19:55:42Z -> 20:24:46Z
Publish (Release) | skipped
```

#### Conclusion

M2 met: the `develop` push ran the test job on the self-hosted runner and completed the
development-image publish job successfully.

### V3 - Publish Isolation (M3)

- Goal: the development publish job runs on a GitHub-hosted runner and imports no GitHub Actions
  cache.
- Initial state: the successful `develop` push run from V2.
- Status: `DONE`

#### Steps Performed

1. Retrieved the `Publish (Development)` job log:
  `gh run view 36345987166 --repo torrust/torrust-tracker --job 108695500749 --log`.
2. Searched that log for `--cache-from=type=gha` and `--cache-to=type=gha`.
3. Inspected `container.yaml` to confirm both publish jobs use `ubuntu-latest` and omit GitHub
  Actions cache inputs.

#### Observed Result

<https://github.com/torrust/torrust-tracker/actions/runs/36345987166/job/108695500749>:

```text
Publish (Development) | success | runner=GitHub Actions 1000090528
github_runner_environment=github-hosted
No type=gha cache flags found in the Publish (Development) job log.
```

#### Conclusion

M3 met: the publish job ran on a GitHub-hosted runner and its build log contains no GitHub Actions
cache import or export flag.

### V4 - Warm Cache Reuse and Timing Comparison (M4, M6)

- Goal: after application code changes, the `test` job reuses the local Docker layers and Cargo
  caches, and its timing is recorded against the baseline.
- Initial state: the runner had built PR #2352 before; the branch was then rebased onto 43 new
  `develop` commits, some of them changing Rust code.
- Status: `DONE`

#### Steps Performed

1. Pushed the rebased branch; `Container` run `36310358131` started.
2. Waited for it with `gh run watch 36310358131 --repo torrust/torrust-tracker --interval 60 --exit-status`.
3. Read the job's step timings from the jobs API and the BuildKit stage results from its log:
   `gh api repos/torrust/torrust-tracker/actions/jobs/108594972392/logs`.

#### Observed Result

<https://github.com/torrust/torrust-tracker/actions/runs/36310358131/job/108594972392>:

```text
Test (Docker) (release) | success | runner=torrust-runner-01 | 09:45:05Z -> 09:57:08Z
#18 CACHED        (dependencies_thirdparty 3/3, cargo chef cook)
#32 CACHED        (dependencies 4/4)
#62 DONE 561.4s   (build 3/3, workspace compile)
PR checks on the head commit: first start 09:45:05Z, last completion 10:02:54Z (Unit (nightly))
```

#### Conclusion

M4 met: third-party layers came from the local Docker store and only the workspace compile reran.
M6 met: the job took 12 min 3 s and the PR checks 17 min 49 s, recorded in `benchmark-results.md`
against the baseline and the 15-minute target.

### V5 - Runner-Offline Recovery (M5)

Recorded under follow-up issue #2374.

- Goal: prove that `timeout-minutes` does not expire a queued self-hosted job, then prove restart
  recovery and the GitHub-hosted fallback.
- Initial state: `torrust-runner-01` online and idle; no queued or active `Container` or `Testing`
  runs. The disposable PR #2378 changed only `Test (Docker)` from `timeout-minutes: 90` to `2`.
- Status: `DONE`

#### Steps Performed

1. Stopped `actions.runner.torrust-torrust-tracker.torrust-runner-01.service` at 18:03:34 UTC.
  The service was `inactive`; GitHub reported the runner `offline` before the PR was opened.
2. Opened disposable PR #2378 at 18:09 UTC. Its Container run `36610063594` created job
  `109548900765` at 18:09:40 UTC.
3. The job remained queued until 18:14:33 UTC, when the restarted runner picked it up. The queue
  duration was 293 seconds (4 min 53 s), longer than its 2-minute `timeout-minutes`. It was
  cancelled at 18:16:40 UTC only after execution had begun, as expected for the execution limit.
4. A second queued run (`36610612944`, job `109550759376`) also started on
  `torrust-runner-01` after recovery, confirming the service resumed the backlog.
5. Pushed a fallback commit only to #2378 that set the test job's `runs-on` to `ubuntu-latest`.
  Its run `36611356707` started job `109553292706` on `GitHub Actions 1000091271` two seconds
  after creation (18:20:31 UTC -> 18:20:33 UTC). It was cancelled at its 2-minute execution cap.
6. Verified `torrust-runner-01` online and idle, then closed #2378 and deleted its fork branch.

Observed result:

```text
run 36610063594 | self-hosted | created 18:09:40Z | started 18:14:33Z | queue 293 s | cancelled 18:16:40Z
run 36610612944 | self-hosted | created 18:14:13Z | started 18:16:42Z | queue 149 s | cancelled 18:18:49Z
run 36611356707 | ubuntu-latest | created 18:20:31Z | started 18:20:33Z | queue 2 s | cancelled 18:23:12Z
```

Job links:

- <https://github.com/torrust/torrust-tracker/actions/runs/36610063594/job/109548900765>
- <https://github.com/torrust/torrust-tracker/actions/runs/36610612944/job/109550759376>
- <https://github.com/torrust/torrust-tracker/actions/runs/36611356707/job/109553292706>

Conclusion: M5 met. A queued job waited past its two-minute timeout and ran only after the runner
recovered, proving that `timeout-minutes` does not limit queue time. The fallback started on a
GitHub-hosted runner without changing `develop`. The five-minute wait in the plan was not needed:
the 293-second observation already exceeds the two-minute claim under test.

### V7 - Untrusted-Code Routing (M7)

Recorded under follow-up issue #2374, which completes the scenarios this issue left open.

- Goal: Dependabot jobs run on GitHub-hosted runners, also after a maintainer updates the branch,
  and a PR from a non-member fork waits for approval before any job runs.
- Initial state: routing merged in PR #2352 (`432d4e69`); fork-PR approval policy
  `all_external_contributors`.
- Status: `DONE`

#### Part 1 - Dependabot PRs

Steps performed (2026-09-29 16:15 UTC, read-only):

1. Listed the Dependabot PRs opened or re-run after the routing change: #2369 and #2338, both
   authored by `app/dependabot`.
2. Inspected their `Container` and `Testing` runs:
   `gh api repos/torrust/torrust-tracker/actions/runs/<run-id>` and
   `gh api repos/torrust/torrust-tracker/actions/runs/<run-id>/jobs`.

Observed result (`labels` is the job's resolved `runs-on` value):

```text
PR #2369, head 5055ba63, branch dependabot/github_actions/develop/github/codeql-action-4.38.2
  run 36474323310 Container pull_request actor=dependabot[bot] triggering=dependabot[bot]
    Test (Docker) (release) | success | runner=GitHub Actions 1000091160 | labels=ubuntu-latest
  run 36474314420 Testing push actor=dependabot[bot] triggering=dependabot[bot]
    Docker E2E | success | runner=GitHub Actions 1000091147 | labels=ubuntu-latest
PR #2338, head cd518a79, branch dependabot/cargo/develop/schemars-1.2.2
  run 36408286110 Container pull_request actor=dependabot[bot] triggering=dependabot[bot]
    Test (Docker) (release) | failure | runner=GitHub Actions 1000090762 | labels=ubuntu-latest
  run 36408280318 Testing push actor=dependabot[bot] triggering=dependabot[bot]
    Docker E2E | success | runner=GitHub Actions 1000090777 | labels=ubuntu-latest
```

Job links:

- <https://github.com/torrust/torrust-tracker/actions/runs/36474323310/job/109104118212>
- <https://github.com/torrust/torrust-tracker/actions/runs/36474314420/job/109104090790>
- <https://github.com/torrust/torrust-tracker/actions/runs/36408286110/job/108882141096>
- <https://github.com/torrust/torrust-tracker/actions/runs/36408280318/job/108882121591>

Conclusion: part 1 met. Both Dependabot PRs ran `Test (Docker)` on GitHub-hosted runners, and the
`Docker E2E` jobs of the pushes to their `dependabot/` branches did too. The #2338 failure is in
the job itself, not in routing. The PR `Docker E2E` jobs were `skipped`, as expected for PRs
targeting `develop`.

#### Part 2 - After a Maintainer Updates the Branch

Steps performed (2026-09-29 18:57 UTC):

1. The maintainer selected **Update with merge commit** for Dependabot PR #2369. The PR author
   remained `app/dependabot` and its new head was `7b3a3245`.
2. Inspected the new Container run `36615722101` and its `Test (Docker)` job `109568077764`.

Observed result:

```text
Container pull_request | actor=josecelano | triggering_actor=josecelano | head=7b3a3245
Test (Docker) (release) | in_progress | runner=GitHub Actions 1000091297 |
  labels=ubuntu-latest | 2026-09-29T18:57:03Z -> 2026-09-29T18:57:05Z
```

<https://github.com/torrust/torrust-tracker/actions/runs/36615722101/job/109568077764>

Conclusion: part 2 met. Although the maintainer triggered the merge commit, #2369 remained a
Dependabot PR and its container test started on a GitHub-hosted runner.

#### Part 3 - PR From a Non-Member Fork

Steps performed (2026-09-29 16:50 UTC):

1. `josecelano-bot` opened #2376 from its fork, `josecelano-bot/torrust-tracker`.
  GitHub classified the author as `FIRST_TIME_CONTRIBUTOR`. The PR adds only `test.md` (three
  lines), and no workflow or executable file changed.
2. Before approval, inspected the PR head `696b0f63`: the combined status was `pending`, with no
  check runs and no Actions workflow runs. The maintainer then approved the workflows in GitHub.
3. After approval, Docs Lint run `36599032618` started at 16:38:45 UTC on `GitHub Actions
  1000091230`, showing that work began only after approval.

Observed result:

```text
Before approval: status=pending; check_runs=[]; workflow_runs=[]
After approval: Docs Lint | failure | runner=GitHub Actions 1000091230 |
           2026-09-29T16:38:45Z -> 2026-09-29T16:39:24Z
```

<https://github.com/torrust/torrust-tracker/pull/2376>
<https://github.com/torrust/torrust-tracker/actions/runs/36599032618/job/109511966141>

The job failed only because the test file contains a spelling error identified by cspell. It is
unrelated to the approval gate. As a documentation-only PR, #2376 did not trigger `Test (Docker)`
and is not evidence of self-hosted-runner routing after approval.

Conclusion: part 3 met. No workflow run existed before the maintainer approved the first-time
external contributor's PR; the approved workflow then started. This proves the approval gate.

Conclusion: M7 met. Dependabot jobs routed to GitHub-hosted runners both before and after a
maintainer updated a Dependabot branch, and the first-time external contributor could not start a
workflow before approval.

## Failures and Follow-up

- Attempt 1, <https://github.com/torrust/torrust-tracker/actions/runs/36298632207/job/108562020495>,
  failed at `Build Tracker Image` after six minutes (05:56:17Z to 06:02:25Z) with
  "The runner has received a shutdown signal". Diagnosis from the server's kernel log: a global
  out-of-memory kill at 06:02 UTC while about eight `rustc` processes compiled the workspace on
  16 GB of RAM with no swap; the kernel killed processes in the runner's service cgroup
  (`docker-buildx`, `Runner.Worker`, `Runner.Listener`), and the service stayed `failed`.
- Remediation (`runner-server-setup.md` step 12): a 16 GB swap file and a `Restart=on-failure`
  drop-in for the runner service; the runner was restarted and reported `online` before the
  re-run. Attempt 2 had no out-of-memory kill (`journalctl -k` since 06:23 reports 0 `Killed
  process` lines) and used 143 MiB of swap.
