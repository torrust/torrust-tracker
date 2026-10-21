---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
    - docs/issues/closed/2507-stop-requiring-pull-request-rebases/baseline-evidence.md
---

<!-- skill-link: create-issue -->

# Baseline Evidence: Pull-Request CI Runner Time Before the CI Gate

Task T8(a) of [#2525](ISSUE.md). This records what the pull-request CI costs in runner time **before**
the `CI gate` orchestrator (T6) changes the workflows, per documentation-only pull request and per code
pull request. GitHub deletes old workflow runs, so the per-run data is committed next to this file and
every figure below can be recomputed from it without GitHub.

Collected on 2026-10-21 at about 07:10 UTC from the GitHub REST and GraphQL APIs for
`torrust/torrust-tracker`, with `develop` at `0a7145528` (`Merge torrust/torrust-tracker#2580`).

## 1. What Was Measured

- **Population:** every pull request merged into `develop` from 2026-10-07 onwards (98 pull requests),
  and every `pull_request` workflow run attributed to one of them (1,189 runs, created from
  2026-10-07 06:04 UTC to 2026-10-20 20:59 UTC). A run belongs to a pull request when its head
  repository and branch match and it was created between the pull request's creation and its merge.
- **Runner time of a run:** the sum, over all jobs of the run (every attempt), of `completed_at` minus
  `started_at`, in minutes. Skipped jobs count as zero. It is **not** wall-clock time: queue time and the
  time jobs wait for each other are not included, and jobs of one run overlap in wall-clock time.
  Cancelled runs count for the time they ran.
- **Class of a pull request:** the pull request's final file list piped, NUL-separated, to
  `ci-gate classify --base-branch develop`. It is `code` when the `testing` group is needed, and
  `docs-only` when it is not. 57 pull requests are `docs-only` and 41 are `code`.
- **Pushed head:** a distinct head commit of a pull request that triggered at least one run. A pull
  request that is pushed to several times has several pushed heads, and each one costs its own runs.

## 2. Result

### 2.1 Cost per pushed head

This is the figure to compare a change of the per-push CI cost against, because it does not depend on
how many times a pull request was pushed.

| Pull requests of class | Window                                | Pushed heads | Runs | Mean runner minutes per head |
| ---------------------- | ------------------------------------- | -----------: | ---: | ---------------------------: |
| docs-only              | whole window                          |          265 |  270 |                          1.1 |
| code                   | whole window                          |          173 |  919 |                         87.8 |
| docs-only              | before #2526 (until 14 Oct 12:18 UTC) |          177 |  181 |                          1.0 |
| code                   | before #2526 (until 14 Oct 12:18 UTC) |           48 |  238 |                         80.2 |
| docs-only              | after #2526                           |           88 |   89 |                          1.3 |
| code                   | after #2526                           |          125 |  681 |                         90.8 |

Pull request #2526 made rebasing optional, so the "after" rows are the current policy. A documentation-only head runs
one workflow, `Docs Lint`, and a code head runs the four large workflows plus `Docs Lint`, and the
database and security workflows when the change needs them.

### 2.2 Cost per pull request

Only the 87 pull requests created on or after 2026-10-07 are used here, because the other 11 had pushes
before the window whose runs are not in the data.

| Class     | Pull requests | Pushed heads per PR (mean) | Mean runner minutes | Median | 90th percentile |  Max |
| --------- | ------------: | -------------------------: | ------------------: | -----: | --------------: | ---: |
| docs-only |            47 |                       4.87 |                 5.4 |    3.8 |            11.0 | 30.5 |
| code      |            40 |                       4.30 |               377.3 |  266.5 |           780.5 | 1584 |

The per-pull-request figure is dominated by the number of pushes. The docs-only maximum is #2550, which
had nine review rounds and 17 pushed heads: 14 of its `Docs Lint` runs took about one minute and 4 took
3.6 to 4.5 minutes (see 2.4).

### 2.3 Cost per workflow

Whole window, per run, by class.

| Class     | Workflow                       | Runs | Mean runner minutes per run | Runner hours |
| --------- | ------------------------------ | ---: | --------------------------: | -----------: |
| docs-only | Docs Lint                      |  270 |                         1.1 |          4.7 |
| code      | OS Compatibility               |  172 |                        27.9 |         80.0 |
| code      | Generate Coverage Reports (PR) |  172 |                        25.6 |         73.4 |
| code      | Testing                        |  172 |                        19.7 |         56.5 |
| code      | Container                      |  172 |                         9.3 |         26.5 |
| code      | Database Compatibility         |   27 |                        17.3 |          7.8 |
| code      | Docs Lint                      |  174 |                         1.5 |          4.4 |
| code      | Database Benchmarking          |   27 |                         7.4 |          3.3 |
| code      | Security Scan                  |    3 |                        26.7 |          1.3 |
| **Total**                      |                                | 1189 |                             |    **258.0** |

### 2.4 The `Docs Lint` run time is bimodal

`Docs Lint` is the only workflow a documentation-only pull request runs, so it is what the gate's added
cost will be compared with. Across all 444 `Docs Lint` runs the median is 0.88 minutes and the mean is
1.2. The runs form two groups: 394 take under 1.5 minutes, and 48 (11%) take 3.0 to 4.9 minutes, 17 of
them on documentation-only pull requests; only 2 fall in between. The slow runs occur on 7 of the 12
days that have runs. The workflow installs the linter with `cargo install --locked torrust-linting`
behind a Rust cache, so the slow runs are probably cache misses; this was not confirmed. A documentation-only
head therefore costs about 0.85 minutes typically, and about 4.5 minutes one time in nine.

## 3. Cross-Check Against the #2507 Baseline

[`baseline-evidence.md`](../../closed/2507-stop-requiring-pull-request-rebases/baseline-evidence.md) section 6
estimated the same quantity for 1 to 13 October from a random sample of six runs per workflow. This
measurement uses every run, so it has no sampling error, but its window is different.

| Workflow                       | #2507 sample mean | This measurement, whole window |
| ------------------------------ | ----------------: | -----------------------------: |
| OS Compatibility               |              28.2 |                           27.9 |
| Generate Coverage Reports (PR) |              27.6 |                           25.6 |
| Testing                        |              22.5 |                           19.7 |
| Container                      |              13.4 |                            9.3 |
| Database Compatibility         |              17.5 |                           17.3 |
| Docs Lint                      |               1.4 |                            1.2 |
| Database Benchmarking          |               5.9 |                            7.4 |
| Security Scan                  |              30.5 |                           26.7 |

The two agree within sampling noise for most workflows. `Container` differs most (13.4 against 9.3), and
a sample of six runs of a workflow whose cost varies with caching can easily be off by that much.

## 4. Limits

- **Runner time, not money and not wall-clock time.** It says how long jobs occupied runners. It does not
  say what a minute costs on the self-hosted runner, and it does not measure how long a pull request waits.
- **A window of two weeks that straddles #2526.** The per-head figures are given before and after it. A
  window this short has few pull requests of the rarer kinds: 3 runs of `Security Scan`, 27 of each
  database workflow.
- **The class is the pull request's final file list.** A pull request that changed from code to
  documentation-only, or the reverse, is counted in its final class for all of its heads.
- **Excluded runs.** 57 `pull_request` runs (4.6% of the 1,246 in the window) belong to pull requests that
  were not merged into `develop`: closed Dependabot pull requests, open pull requests, and branches
  without a pull request into `develop`. `push` runs are not included.
- **Reruns.** Only one run of the 1,189 has a second attempt, and the jobs of every attempt are counted.
- **Cancelled runs.** 151 of the 1,189 runs (12.7%) ended `cancelled`, 81 of them `Container` and 70
  `Testing`, the two workflows whose concurrency group cancels an in-progress pull-request run when a
  newer commit arrives. Their time up to the cancellation is counted.

## 5. How to Reproduce

The figures came from a temporary collection script that is not committed, because it only calls the
commands below and writes the CSV; the CSV is the durable evidence. The commands it ran, all through
`gh`:

```sh
# 1. The merged pull requests, with their final file lists.
gh pr list --repo torrust/torrust-tracker --state merged --base develop \
  --search "merged:>=2026-10-07" --limit 500 \
  --json number,headRefName,headRepository,headRepositoryOwner,createdAt,mergedAt,files

# 2. Their class: the file list, NUL-separated, into the change-detection tool.
printf '%s\0' <the file paths of one pull request> | ci-gate classify --base-branch develop

# 3. The runs, one query per day because a listing is capped at 1000 results.
gh api --paginate \
  "repos/torrust/torrust-tracker/actions/runs?event=pull_request&created=2026-10-07..2026-10-07&per_page=100"

# 4. The jobs of each attributed run, every attempt.
gh api --paginate "repos/torrust/torrust-tracker/actions/runs/<run id>/jobs?filter=all&per_page=100"
```

The first attempt listed the runs of the whole window in one query and silently returned only 1,000 of
1,245; comparing the count with the response's `total_count` showed the gap, and the per-day queries
fixed it. The script now asserts that each day's listing is complete.

To recompute the tables, group the rows of
[`evidence/pr-ci-runs-2026-10-07-to-2026-10-21.csv`](evidence/pr-ci-runs-2026-10-07-to-2026-10-21.csv)
by `pr_class` and by `workflow`, and by distinct `head_sha` per `pr`, and sum `job_minutes`. The class and
the changed groups of each pull request are in
[`evidence/pr-ci-prs-2026-10-07-to-2026-10-21.json`](evidence/pr-ci-prs-2026-10-07-to-2026-10-21.json).
