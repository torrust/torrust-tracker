---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
last-updated-utc: 2026-09-30 11:04
semantic-links:
  related-artifacts:
    - ISSUE.md
    - ../../../self-hosted-runner.md
---

# Manual Verification Evidence

<!-- cspell:ignore fromdate startswith -->

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands
or interactions actually performed against the artifact; do not invent commands, output, logs, or
results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-30
- Artifact under test: the self-hosted runner `torrust-runner-01` (8 vCPU, 16 GB RAM, 16 GB swap)
  and the `Container` workflow's run history in `torrust/torrust-tracker`.
- Operating system / environment: maintainer desktop with an authenticated GitHub CLI (`gh`) and
  POSIX `awk`, `sort`, and `sed`.
- Prerequisites and setup performed: none; every command in V3 is read-only.

## Verification Processes

### V1 - Full-Size Build (M1)

- Status: `TODO` (waiting for the maintainer-approved measurement window)

### V2 - 4 vCPU / 8 GB Build (M2)

- Status: `TODO` (waiting for the maintainer-approved measurement window)

### V3 - Queue-Time Recheck and Workload Replay (M3)

- Goal: measure how long jobs wait for the single runner, both as observed and for a normal
  month's workload.
- Initial state: one runner instance; no workflow cancels superseded runs (`container.yaml` and
  `testing.yaml` have no `concurrency` group).
- Status: `IN_PROGRESS` (the observed window is extended to the measurement date after V1 and V2)

#### Steps Performed

1. Ran the guide's queue-time recheck command ("Add Runner Capacity") with `since=2026-09-30`.
2. Collected the arrival time, event, and branch of every `Container` run created from 2026-08-31
   that would use the self-hosted runner (pull requests and pushes to `develop`, except
   Dependabot's):

   ```bash
   gh api --paginate "repos/torrust/torrust-tracker/actions/workflows/container.yaml/runs?created=>=2026-08-31&per_page=100" -q '.workflow_runs[] | select(.actor.login != "dependabot[bot]" and (.head_branch | startswith("dependabot/") | not)) | select(.event == "pull_request" or (.event == "push" and .head_branch == "develop")) | [(.created_at | fromdate), .event, .head_branch, .created_at] | @tsv' | sort -n > arrivals.tsv
   ```

3. Replayed those arrivals, in order, through a first-in-first-out queue served by `C` runners,
   each job taking a fixed `S` seconds. `S` = 723 s is the realistic pull-request job measured in
   #2323 (run `36310358131`); `S` = 968 s is the 2026-09-28 mean workspace compile (806 s) plus the
   162 s of non-build steps. The replay prints each job's wait:

   ```bash
   awk -v S=723 -v C=1 -F'\t' '{ t=$1; best=1; for (i=2;i<=C;i++) if (free[i] < free[best]) best=i; start=(free[best]>t)?free[best]:t; free[best]=start+S; print start-t "\t" substr($4,1,10) }' arrivals.tsv
   ```

4. Repeated step 3 after removing each pull-request run followed by a newer run on the same branch
   within `S` seconds, as a `cancel-in-progress` policy would:

   ```bash
   awk -v S=723 -F'\t' '{ t[NR]=$1; e[NR]=$2; b[NR]=$3 } END { for (i=1;i<=NR;i++) { keep=1; if (e[i]=="pull_request") for (j=i+1;j<=NR && t[j]-t[i] < S;j++) if (b[j]==b[i]) { keep=0; break } if (keep) print t[i] } }' arrivals.tsv
   ```

5. Calibrated the model by replaying only 2026-09-28 and comparing it with the queue observed that
   day.

#### Observed Result

Queue-time recheck, 2026-09-30 08:03 to 10:27 UTC:

```text
9 Test (Docker) jobs on torrust-runner-01
queued 1, 1, 2, 2, 2, 124, 194, 216, 760 s (4 of 9 waited more than 60 s)
```

Workload, 2026-08-31 to 2026-09-30:

```text
546 runs on 29 active days: median 14 per day, 75th percentile 23, maximum 53
78 pull-request branches: median 4 runs per branch, mean 6.0, maximum 42
155 of 471 pull-request runs (33%) were followed within 723 s by a newer run on the same branch
```

Replay, all runs (waits rounded to the nearest minute):

```text
job 723 s, 1 runner:  waited >60 s 56%, >15 min 187 runs, p90 99 min, max 196 min, 18 days with a >15 min wait
job 723 s, 2 runners: waited >60 s 23%, >15 min  40 runs, p90 10 min, max  48 min,  3 days
job 968 s, 1 runner:  waited >60 s 64%, >15 min 242 runs, p90 232 min, max 330 min, 23 days
job 968 s, 2 runners: waited >60 s 30%, >15 min  81 runs, p90 28 min, max  93 min,  6 days
```

Replay, superseded pull-request runs cancelled:

```text
job 723 s, 1 runner:  391 runs, waited >60 s 31%, >15 min 20 runs, p90 10 min, max 62 min
job 723 s, 2 runners: 391 runs, waited >60 s  2%, >15 min  0 runs, p90  0 min, max 11 min
job 968 s, 1 runner:  370 runs, waited >60 s 32%, >15 min 36 runs, p90 14 min, max 53 min
job 968 s, 2 runners: 370 runs, waited >60 s  3%, >15 min  0 runs, p90  0 min, max 11 min
```

Calibration, 2026-09-28 (25 jobs, one runner):

```text
observed:        12 waited >60 s, max 31 min
replay at 723 s: 13 waited >60 s, max 25 min
replay at 968 s: 15 waited >60 s, max 42 min
```

#### Conclusion

The replay brackets the observed day, so it is a usable model. With a normal month's workload, one
runner queues heavily. Cancelling superseded pull-request runs removes about a third of the jobs
and cuts the 90th-percentile wait from 99 to 10 minutes at no cost; a second runner on top brings
waits close to zero. The model assumes every job recompiles; warm jobs (130 to 140 s) are faster,
so it overstates waits somewhat. A cancelled run may also have used part of its time before being
cancelled, so the cancellation rows are a lower bound on load.

### V4 - Runner Restored (M4)

- Status: `TODO` (after V1 and V2)

## Failures and Follow-up

None yet.
