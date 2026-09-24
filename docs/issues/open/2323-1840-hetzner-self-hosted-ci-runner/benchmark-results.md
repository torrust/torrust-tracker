# Benchmark Results - Container Workflow on GitHub-Hosted Runners

Baseline (T1) for issue #2323, before the `Test (Docker)` job moves to the self-hosted runner.
See [ISSUE.md](ISSUE.md) for the plan and the Post-Switch Scenarios table.

## Capture

| Property        | Value                                                                                           |
| --------------- | ----------------------------------------------------------------------------------------------- |
| Captured        | 2026-09-24 16:10 UTC                                                                            |
| Source          | GitHub REST API through `gh` (runs, jobs, job logs, check runs, cache usage)                    |
| Workflow        | `.github/workflows/container.yaml`, job `Test (Docker) (release)`                               |
| Runner          | `ubuntu-latest` (standard GitHub-hosted runner)                                                 |
| Sample (timing) | 25 successful runs, 2026-09-23 15:12 UTC to 2026-09-24 08:41 UTC (22 `pull_request`, 3 `push`) |
| Sample (logs)   | 6 of those runs, with BuildKit step timings taken from the job logs                            |

Raw API responses were kept in the git-ignored `.tmp/` directory and are not committed.

## Test (Docker) Job, 25 Runs

| Metric                                           | Min    | Median | Mean   | Max    |
| ------------------------------------------------ | ------ | ------ | ------ | ------ |
| Queue time (job created to started)              | 1 s    | 2 s    | 149 s  | 1036 s |
| Job duration                                     | 1794 s | 2239 s | 2311 s | 2958 s |
| `Build Tracker Image` step                       | 1448 s | 1925 s | 1958 s | 2500 s |
| E2E steps (persistence regression, E2E, qBittorrent x3) | 203 s  | 218 s  | 311 s  | 450 s  |

In minutes: the job takes 30 to 49 minutes (median 37), and `Build Tracker Image` takes 24 to 42
minutes (median 32, about 86% of the job). The E2E steps take 3.4 to 7.5 minutes (median 3.6).
The remaining setup and cleanup steps take about one minute (59 s in run `35972794173`).

## Build Step Breakdown, 6 Runs

BuildKit timings from the job logs. "Third-party cook" is the
`dependencies_thirdparty 3/3` stage (`cargo chef cook` of external dependencies). "Workspace
compile" is the `build 3/3` stage (`cargo nextest archive` of the workspace). Cache export is
split into BuildKit's local "preparing build cache for export" phase and the "sending cache
export" upload to the GitHub Actions cache.

| Run           | Event          | Build step | Third-party cook | Workspace compile | Export: prepare | Export: upload | Cache import |
| ------------- | -------------- | ---------- | ---------------- | ----------------- | --------------- | -------------- | ------------ |
| `35922675851` | `pull_request` | 1448 s     | cached           | 1068 s            | 160 s           | 136 s          | 0.1 s        |
| `35962976925` | `pull_request` | 1925 s     | cached           | 1084 s            | 160 s           | 566 s          | 0.6 s        |
| `35968093266` | `pull_request` | 2280 s     | 283 s (miss)     | 1095 s            | 176 s           | 376 s          | 0.2 s        |
| `35972579291` | `push`         | 2067 s     | 295 s (miss)     | 1119 s            | 169 s           | 176 s          | 0.1 s        |
| `35972794173` | `pull_request` | 1974 s     | 221 s (miss)     | 932 s             | 157 s           | 384 s          | 11.5 s       |
| `35976748146` | `push`         | 2408 s     | 294 s (miss)     | 1115 s            | 168 s           | 522 s          | 0.5 s        |

## GitHub Actions Cache State

| Property                    | Value                                                              |
| --------------------------- | ------------------------------------------------------------------ |
| Active cache size           | 10.8 GB (10,813,432,132 bytes), above the 10 GB repository allowance |
| Active cache entries        | 3074                                                               |
| Largest entries             | BuildKit blobs on `refs/heads/develop`, 0.7 to 1.3 GB each         |

## End-to-End PR Check Time, 4 PR Heads

Wall-clock time from the first check start to the last check completion on the PR head commit.

| Run           | Checks | Wall clock | Slowest check                 | Second slowest         |
| ------------- | ------ | ---------- | ----------------------------- | ---------------------- |
| `35922675851` | 23     | 32 min     | `Test (Docker) (release)` 31 min | `Security Scan` 28 min |
| `35962976925` | 23     | 36 min     | `Test (Docker) (release)` 36 min | `Security Scan` 28 min |
| `35968093266` | 22     | 46 min     | `Test (Docker) (release)` 46 min | `Security Scan` 29 min |
| `35972794173` | 20     | 37 min     | `Test (Docker) (release)` 37 min | `Unit (stable)` 14 min |

## Findings

1. **`Test (Docker)` sets the PR wall-clock time.** It was the slowest check in all four PRs, and
   the PR wall clock was within a minute of its duration.
2. **Workspace compile is the largest fixed cost.** 15.5 to 18.6 minutes in every sampled run.
   Application code changes on every PR, so this stage is never a cache hit; only a faster CPU
   (or a smaller build) reduces it.
3. **Exporting the cache to GitHub costs 5 to 12 minutes per job, already on GitHub-hosted
   runners.** Preparing takes 2.6 to 2.9 minutes and uploading takes 2.3 to 9.4 minutes, with
   high variance. This is the network cost the issue was concerned about, and it is present even
   inside GitHub's own network.
4. **The third-party dependency layer was a cache miss in 4 of 6 runs**, costing 3.7 to 4.9
   minutes each time. The cause is not yet known: those PRs may have changed dependencies, or the
   layer may have been evicted.
5. **The cache is over its 10 GB allowance**, so GitHub evicts entries. With `mode=max` exporting
   every intermediate layer (single blobs up to 1.3 GB), eviction is a plausible cause of finding 4.
6. **Queue time is usually negligible (1 to 3 s) but reached 17 minutes** during a burst of
   runs on 2026-09-23 between 17:57 and 18:37 UTC.
7. **`Security Scan` is the next critical path for PRs that change the `Containerfile`.** It runs
   only when the `Containerfile` or its own workflow changes, rebuilds the image with a plain
   `docker build` without cache, and took 28 to 29 minutes. It ran on three of the four sampled PR
   heads, so those PRs changed one of its trigger paths. For such PRs, the 15-minute target is not
   reachable by moving `Test (Docker)` alone (scenario G).

## Implications for the Plan

- Keeping caches on the persistent server would remove the 5 to 12 minute export and probably
  most third-party cache misses. Based on this data, local caches are part of the workflow change
  (T5) instead of a conditional follow-up.
- An upload from Hetzner to GitHub's cache is expected to be slower than the upload measured here,
  so the runner switch is not measured separately with the GitHub cache.
- `Security Scan` should be measured in T7 and, if it remains the critical path, handled as a
  follow-up subissue of #1840.

## Reproduction

```bash
# Successful Container runs and their jobs
gh run list --repo torrust/torrust-tracker --workflow container.yaml --limit 30 \
  --json databaseId,conclusion -q '.[] | select(.conclusion=="success") | .databaseId'
gh api "repos/torrust/torrust-tracker/actions/runs/<run-id>/jobs"

# BuildKit timings for one job
gh api "repos/torrust/torrust-tracker/actions/jobs/<job-id>/logs" \
  | grep -E 'dependencies_thirdparty 3/3|build 3/3|DONE|CACHED|preparing build cache|sending cache export'

# End-to-end check time for a PR head commit
gh api "repos/torrust/torrust-tracker/commits/<head-sha>/check-runs?per_page=100"

# Cache usage
gh api repos/torrust/torrust-tracker/actions/cache/usage
```

Durations are computed as `completed_at - started_at` (queue time: `started_at - created_at`).
