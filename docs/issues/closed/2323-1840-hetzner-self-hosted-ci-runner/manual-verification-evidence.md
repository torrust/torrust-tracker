---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2323-1840-hetzner-self-hosted-ci-runner/ISSUE.md
last-updated-utc: 2026-09-28
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
