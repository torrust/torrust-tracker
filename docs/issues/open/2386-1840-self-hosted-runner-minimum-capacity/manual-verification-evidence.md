---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2386-1840-self-hosted-runner-minimum-capacity/ISSUE.md
last-updated-utc: 2026-09-30 15:17
semantic-links:
  related-artifacts:
    - ISSUE.md
    - ../../../self-hosted-runner.md
---

# Manual Verification Evidence

<!-- cspell:ignore cpuset cpus fromdate journalctl startswith vmstat -->

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands
or interactions actually performed against the artifact; do not invent commands, output, logs, or
results.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-30
- Artifact under test: the self-hosted runner `torrust-runner-01` (8 vCPU, 16 GB RAM, 16 GB swap)
  and the `Container` workflow's run history in `torrust/torrust-tracker`.
- Operating system / environment: maintainer desktop with an authenticated GitHub CLI (`gh`) and
  POSIX `awk`, `sort`, and `sed`; V1, V2, and V4 ran on the runner host (Ubuntu 26.04.1, Docker
  29.8.1) over SSH with the maintainer's key.
- Prerequisites and setup performed: V3 commands are read-only. V1 and V2 ran in a
  maintainer-approved window with the runner service stopped (V4 records the window).

## Verification Processes

### V1 - Full-Size Build (M1)

- Goal: time the realistic pull-request build (warm cache, application code changed) at the
  server's full size, 8 vCPU and 16 GB.
- Initial state: runner service stopped (V4); `develop` at `ec80aa0`.
- Status: `DONE`

#### Steps Performed

1. As the `runner` user, added a detached worktree of `develop` (`ec80aa0`) at
   `/home/runner/capacity-bench` and created the builder:
   `docker buildx create --name capacity-bench --driver docker-container`. Git run as `root`
   refuses the Actions checkout, which `runner` owns ("detected dubious ownership").
2. Warmed the builder once without limits:
   `docker buildx build --builder capacity-bench --target release --progress=plain -f Containerfile .`.
   The `-f Containerfile` is required; without it Buildx looks for a `Dockerfile` and fails.
3. Appended a comment line to `src/lib.rs` and ran the same build, with `vmstat -w 1` sampling the
   host.
4. Read the `build 3/3` (`cargo nextest archive --release`) duration and the test summary from the
   build log, the minimum free memory and maximum swap from the `vmstat` samples, and
   out-of-memory kills from `journalctl -k`.

#### Observed Result

```text
build 13:57:49 to 14:08:28 UTC: success, 639 s
build 3/3 workspace compile: 614.1 s
image test stage: 1148 tests passed in 12.3 s
host: minimum free memory 126,192 KiB, maximum swap used 8,649,504 KiB (8.25 GiB)
out-of-memory kills: none
```

Projected job time: 639 s + 162 s of non-build steps = 801 s (13.4 minutes).

The builder's `memory.peak` (14.72 GiB) and `memory.swap.peak` (8.02 GiB) also cover the warm-up
build, so the host samples are the figures for this run.

#### Conclusion

The current size meets the 15-minute target with 1.6 minutes to spare. The workspace compile took
614 s, against 561 s in #2323 run `36310358131`, so the job has grown since the size was chosen.
Memory is also close to its limit: the host was down to about 123 MiB free and used 8.25 GiB of
swap.

### V2 - 4 vCPU / 8 GB Build (M2)

- Goal: time the same build with the resources of the next smaller Hetzner shared-vCPU size.
- Initial state: the builder from V1, warmed by V1's build.
- Status: `DONE`

#### Steps Performed

1. Limited the builder container and checked the limits in its cgroup:

   ```bash
   docker update --cpuset-cpus 0-3 --memory 8g --memory-swap 24g buildx_buildkit_capacity-bench0
   ```

   The cgroup reported `cpuset.cpus.effective` `0-3`, `memory.max` 8589934592 (8 GiB), and
   `memory.swap.max` 17179869184 (16 GiB).
2. Appended a different comment line to `src/lib.rs` and ran the same build as V1, with `vmstat`
   sampling the host.
3. From 14:48 UTC to the end of the build, sampled the builder's `memory.current` and
   `memory.swap.current` every second, and read `memory.events` afterwards.

#### Observed Result

```text
build 14:35:27 to 14:53:12 UTC: success, 1065 s
build 3/3 workspace compile: 1029.7 s
image test stage: 1148 tests passed in 20.6 s
builder memory: at the 8 GiB limit (maximum sample 8,589,811,712 bytes); memory.events max 41922, oom 0, oom_kill 0
builder swap: maximum sample 3.10 GiB (14:48 onwards only)
host: maximum swap used 5,969,952 KiB (5.69 GiB)
out-of-memory kills: none
```

Projected job time: 1065 s + 162 s = 1227 s (20.5 minutes).

#### Conclusion

4 vCPU / 8 GB misses the 15-minute target by 5.5 minutes, outside the 13.5 to 16.5 minute band
that would call for a real candidate server (Open Question 2). The result matches the 1250 s
estimate from GitHub-hosted 4 vCPU runners. The build ran at its memory limit throughout, so
swapping added to the loss of CPU.

Measurement limits: this is an approximation of a smaller server, not one (see the specification's
Measurement Method). Writing `0` to `memory.peak` and `memory.swap.peak` before the run did not
change the values read afterwards, so the per-second sampling was added part-way through; the
swap peak covers only its last 5 minutes.

### V3 - Queue-Time Recheck and Workload Replay (M3)

- Goal: measure how long jobs wait for the single runner, both as observed and for a normal
  month's workload.
- Initial state: one runner instance; no workflow cancels superseded runs (`container.yaml` and
  `testing.yaml` have no `concurrency` group).
- Status: `DONE`

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
6. After V1, reran step 1 for the whole of 2026-09-30 and steps 2 to 4 with `S` = 801 s, the job
   time projected from V1.

#### Observed Result

Queue-time recheck, 2026-09-30 08:03 to 10:27 UTC:

```text
9 Test (Docker) jobs on torrust-runner-01
queued 1, 1, 2, 2, 2, 124, 194, 216, 760 s (4 of 9 waited more than 60 s)
```

Queue-time recheck, whole of 2026-09-30 (last job 13:27 UTC, before the V1 window):

```text
17 Test (Docker) jobs on torrust-runner-01
6 waited more than 60 s: 124, 194, 216, 249, 350, 760 s
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

Replay at the V1 job time, 554 runs from 2026-08-31 to 2026-09-30 13:27 UTC:

```text
job 801 s, 1 runner:              waited >60 s 59%, >15 min 206 runs, p90 140 min, max 239 min, 20 days
job 801 s, 2 runners:             waited >60 s 26%, >15 min  50 runs, p90  13 min, max  63 min,  4 days
job 801 s, 1 runner, cancelling:  395 runs, waited >60 s 32%, >15 min 28 runs, p90 12 min, max 79 min, 9 days
job 801 s, 2 runners, cancelling: 395 runs, waited >60 s  3%, >15 min  0 runs, p90  0 min, max 11 min
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
cancelled, so the cancellation rows are a lower bound on load. At the V1 job time of 801 s the
picture is the same: cancelling cuts the one-runner 90th-percentile wait from 140 to 12 minutes,
and a second runner alone cuts it to 13 minutes.

### V4 - Runner Restored (M4)

- Goal: take the runner out of service for V1 and V2 without losing jobs, then restore it and
  leave nothing of the benchmark behind.
- Status: `DONE`

#### Steps Performed

1. Before the window, checked in separate commands that the runners API showed
   `torrust-runner-01` `online` and `busy=false`, that no `Container` run was queued, and that the
   host ran no containers and no `Runner.Worker`. The last self-hosted job (run `36721823660`)
   finished at 13:30:18 UTC.
2. Stopped `actions.runner.torrust-torrust-tracker.torrust-runner-01.service`; the runners API
   reported `offline` before the benchmark setup began.
3. After V2, removed the builder with its cache (`docker buildx rm capacity-bench`) and the
   worktree (`git worktree remove --force /home/runner/capacity-bench`), and checked the Actions
   checkout.
4. Started the service and polled the runners API until it reported `online`.
5. Listed queued and in-progress runs.

#### Observed Result

```text
service stopped 13:34:31 UTC, started 14:58:54 UTC, "Listening for Jobs" 14:58:58 UTC
runners API at 14:59:02 UTC: online, busy=false
builder container and worktree removed; Actions checkout clean at ec80aa0 on develop
root file system: 201 GB used after the warm-up build, 189 GB after removing the builder
queued or in-progress runs after the restart: none
```

#### Conclusion

The window lasted 84 minutes. No job arrived during it, so none was delayed, and the host is back
to its previous state.

## Failures and Follow-up

- The first attempts to monitor the V1 and V2 builds matched their own command lines with
  `pgrep -f`, so a watch loop never ended and the V2 sampler kept writing after the build finished.
  Both were stopped by hand; the sampler's maximum still reflects the build, because memory drops
  when the build ends. Watch from the desktop with short SSH checks instead of a loop on the host.
- `memory.peak` could not be reset for V2 (see V2), so its swap peak covers only the last
  5 minutes of the build.
