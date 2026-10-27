---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/ISSUE.md
last-updated-utc: 2026-10-27 10:15
---

# Manual Verification Evidence

<!-- cspell:ignore ardinghelli fsverity inodes journalctl keldysh moby overlayfs -->

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands or interactions actually performed against the artifact; nothing here is invented. The outputs below were copied from the collection session on 2026-10-27 and trimmed to the relevant lines. Run ids and container names are public CI data. The runner's IP address is deliberately not recorded.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-27, collected between 09:12 and 09:21.
- Artifact under test: the self-hosted runner `torrust-runner-01` as configured by [docs/self-hosted-runner.md](../../../self-hosted-runner.md). No repository patch is under test at drafting time.
- Operating system / environment: Ubuntu 26.04.1 LTS, kernel 7.0.0-34-generic, Docker 29.8.1 with the containerd overlayfs snapshotter, BuildKit 0.33.0, 8 vCPU, 16 GB RAM.
- Prerequisites and setup performed: SSH to `torrust-runner-01` as `root` through the maintainer's `~/.ssh/config`, opened by the maintainer. Every command below was read-only. Nothing was stopped, pruned, or deleted.

## Verification Processes

### V1 - Reproduce the symptom: full disk and a prune that frees nothing

- Goal: observe the wrong outcome itself, not only the code path.
- Initial state: runner online and not busy; no job running.
- Status: `DONE`
- Reproduction outcome: **Reproduced** for the symptom. The cause is not yet reproduced (see V2).

#### Steps Performed

1. `df -h /; df -i /` on the runner (09:12:40 UTC).
2. `docker system df`.
3. `docker ps -a --format '{{.ID}} {{.Image}} {{.Names}} {{.Status}} {{.CreatedAt}}'`.
4. `docker buildx du --verbose` and counted records by their `Reclaimable` and `Usage count` fields.
5. `journalctl -u docker-ci-prune.service --since 2026-09-26 -o short-iso`, keeping the lines that match `docker[<pid>]: Total:`, which belong to the build-cache prune.
6. `journalctl --since "10 days ago" | grep -i "no space left"` and a per-day count.
7. `gh run list` and `gh api repos/torrust/torrust-tracker/actions/runs/<id>/jobs` from the maintainer's desktop.

#### Observed Result

Disk (steps 1 and again at about 09:19 UTC):

```text
Filesystem      Size  Used Avail Use% Mounted on
/dev/sda1       301G  288G  186M 100% /
/dev/sda1       19793280 1869214 17924066   10% /     (df -i: inodes)

/dev/sda1       301G  288G   93M 100% /
```

Docker (step 2):

```text
TYPE            TOTAL     ACTIVE    SIZE      RECLAIMABLE
Images          7         0         2.724GB   2.724GB (99%)
Containers      4         4         327.7kB   0B (0%)
Local Volumes   3         0         49.78kB   49.78kB (100%)
Build Cache     483       483       253.5GB   0B
```

Containers that were running (step 3):

```text
435bf7e0c09e 8e22809d2be9 nostalgic_einstein Up 4 days (healthy) 2026-10-22 18:48:49 +0000 UTC
151202f4784a 8931a9b8ac8a intelligent_ardinghelli Up 6 days (healthy) 2026-10-20 17:56:41 +0000 UTC
06fc40c52209 3b054882eea1 angry_keldysh Up 10 days (healthy) 2026-10-16 11:56:58 +0000 UTC
d974aa687d71 1548fdc25fbf hungry_mclaren Up 12 days (healthy) 2026-10-14 10:40:07 +0000 UTC
```

All four run `/usr/local/bin/entry.sh [/usr/bin/torrust-tracker]`, are attached to the `bridge` network, have 3 mounts, and publish ports 1212, 1313, 6969, and 7070. Their images are `torrust-tracker:local` by name, but their image ids (`8e22809d2be9`, `8931a9b8ac8a`, `3b054882eea1`, `1548fdc25fbf`) are not in `docker image ls -a`, which lists `torrust-tracker:local` as `7286323317d9`.

Build cache records (step 4):

```text
docker buildx du (summary):  Reclaimable: 0B   Total: 253.6GB
docker buildx du --verbose:  "Reclaimable: false" on 487 entries, "Reclaimable: true" on 27 entries
                             "Usage count: 0" on 117 entries, "1" on 344, "2" on 14, higher counts on a few
ctr -n moby snapshots ls:    121 Active, 491 Committed (and 9 with no kind column)
ctr -n moby leases ls:       507 leases
```

The 27 reclaimable entries are tiny (for example 67 B and 80 B) and were created at 09:17:27 UTC, the time of the command; they do not account for any of the 253.6 GB. The verbose listing has more entries than the 483 `ID:` lines counted in the same session; that mismatch was not investigated.

Build-cache prune result per run (`Total:` lines, step 5):

```text
2026-10-15 88.56GB   2026-10-16 82.6GB   2026-10-17 87.47GB   2026-10-18 7.769GB
2026-10-19 0B        2026-10-20 77.42GB  2026-10-21 60.56GB   2026-10-22 40.27GB
2026-10-23 17.48GB   2026-10-24 943MB    2026-10-25 0B        2026-10-26 0B
2026-10-27 0B
```

The full series from 2026-09-26 is in the first runner health report from PR #2672.

Disk-full evidence (step 6):

```text
First "No space left on device" (rsyslogd, /var/log/syslog): Oct 27 07:42:47
Count per day, Oct 20 to Oct 26: 0 each. Oct 27: 28592.
```

CI impact (step 7):

```text
40467307852  Container  failure  2026-10-27T07:28:16Z  push     develop
40471601395  Container  failure  2026-10-27T08:09:48Z  pull_request  2637-1347-add-tests-axum-rest-api-server-auth
  job "Test (Docker) (release)": failure, runner torrust-runner-01, 09:08:23 to 09:08:42, no failed step recorded
  jobs "Context", "Publish (Release)", "Publish (Development)": skipped
40464969167  Container  success  2026-10-27T07:05:05Z  (Test (Docker) 07:05:08 to 07:22:25)
```

The log of the failed jobs could not be retrieved, so the failure cause is not proven to be the full disk.

The timer itself: `docker-ci-prune.service` last ran 2026-10-27 04:00:10 UTC with `Result=success` and `ExecMainStatus=0`; the timer is `active (waiting)` and fires next at 2026-10-28 04:00 UTC.

#### Conclusion

The wrong outcome was observed: `/` is 100% full, the build cache is 253.6 GB with 0 B reclaimable, the prune freed 0 B on three consecutive days, and container jobs on the runner failed after the disk filled. The prune is not failing; it simply cannot reclaim the cache. Why it cannot is the open question for T1.

### V2 - Find what pins the cache, and try to reproduce the leak (T1)

- Goal: confirm or reject the hypothesis that the four long-running containers pin the cache, and find what does.
- Initial state: the runner state recorded in V1, unchanged; read-only commands only for the analysis steps.
- Status: `IN_PROGRESS` (the pinned state is identified; how a cancelled job leaves it is not reproduced)
- Reproduction outcome for the leak trigger: **Infeasible on the runner** (no free disk and a production host), with a negative local substitute.

#### Steps Performed

1. Between about 10:05 and 10:07 UTC on 2026-10-27, copied `docker buildx du --verbose`, `ctr -n moby snapshots ls`, `ctr -n moby leases ls`, `ctr -n moby containers ls`, and `docker system df -v` output from the runner into a local git-ignored folder, and analyzed it offline with a short Python script (not kept).
2. Read the daemon journal: `journalctl -u docker --since 2026-10-01`, counting `/moby.buildkit.v1.Control/Solve` errors by day and by `desc =` text.
3. Counted Container workflow runs per day and conclusion with `gh run list --workflow container.yaml`.
4. Local substitute experiment on the maintainer's desktop (Docker 28.3.3, overlay2, BuildKit 0.23.2, buildx 0.26.1; not the runner's stack): `timeout -s INT -k 8 45 docker buildx build --load -f Containerfile .` on a three-step image whose last `RUN` writes 64 MB and sleeps 600 s, so the build is cancelled mid-step. Compared `docker buildx du` and the in-use asterisks of `docker system df -v` before and after.

#### Observed Result

Build-cache graph (step 1), 2026-10-27 about 10:05 UTC:

```text
487 build-cache records, 253.6 GB; none reclaimable; no record has "Mutable: true" or "Shared: true"
117 leaf records (no child), all with Usage count 0 and no "Last used"; every one of the 487 records is an ancestor-or-self of a leaf
82 records larger than 1 GB, 187.6 GB in total; the largest leaves are 2.2 to 3.0 GB (intermediate build stages, not the 260 MB final image)
docker system df -v: all 487 records marked "*" (in use)
ctr -n moby snapshots ls: 121 Active and 500 Committed (the 500 include 9 base layers with no parent); the 4 containers account for the 4 extra Active snapshots, leaving 117 Active after their removal
```

Leaf records by creation day (all still pinned): 09-27 1, 09-29 2, 10-02 2, 10-03 4, 10-05 2, 10-06 2, 10-07 4, 10-08 4, 10-13 3, 10-14 12, 10-15 5, 10-16 7, 10-19 12, 10-20 4, 10-21 12, 10-22 9, 10-23 14, 10-24 4, 10-25 3, 10-26 4, 10-27 7.

The four containers (step 1): each has its own 28-layer image chain plus an Active snapshot, from a 260 MB image, so together they cannot hold 253 GB. The 121 Active snapshots at that time were the 117 above plus the 4 container snapshots.

Daemon journal (step 2), since 2026-10-01:

```text
553 level=error msg=/moby.buildkit.v1.Control/Solve   (551 with "desc = context canceled")
439 level=error msg=/moby.buildkit.v1.frontend.LLBBridge/StatFile
dockerd had run for 32 days without a restart
```

Container workflow runs (step 3), conclusion per day: 2026-10-23 13 cancelled and 16 success; 2026-10-24 11 cancelled and 16 success; 2026-10-25 3 cancelled and 9 success; 2026-10-26 5 cancelled and 19 success.

Local substitute experiment (step 4), 2026-10-27 10:08 to 10:10 UTC:

```text
before:        14 records, 157.6 MB, 0 in use
build:         step 7 CANCELED, "ERROR: failed to build: failed to solve: Canceled: context canceled", client exit 137
after cancel:  18 records, 224.7 MB, 0 in use, all reclaimable
```

#### Conclusion

- The hypothesis that the four containers pin the cache is **rejected**. Removing them (V4) left the cache unchanged at 253.6 GB with 0 B reclaimable, and their image chains are far too small.
- What pins the cache is 117 leaf records that BuildKit inside dockerd still considers in use, each backed by an Active snapshot that no maintained job owns. All 487 records descend from them, so none can be pruned.
- Creation dates of the leaves and the heavy cancellation days of 2026-10-14 and 2026-10-19 to 2026-10-23 point at cancelled builds as the trigger, but that is a correlation. The local experiment did **not** reproduce it: on Docker 28.3.3 with overlay2 a cancelled build left nothing in use. The runner uses a different stack (Docker 29.8.1, containerd snapshotter, BuildKit 0.33.0, `docker/build-push-action` with `load: true`, multi-stage `cargo chef` builds), so the negative result is not a like-for-like refutation. Reproducing the trigger on the runner stack is still open.

### V3 - Like-for-like recheck after the fix

- Goal: repeat V1 after the last production change, and after three daily prune runs.
- Initial state: to be recorded when run. Name the commit under test by its Conventional Commit subject.
- Status: `TODO`

#### Steps Performed

To be recorded when executed.

#### Observed Result

```text
Not yet run.
```

#### Conclusion

Pending.

### V4 - Recover the runner (T2)

- Goal: free the disk and make the build cache reclaimable again, with the maintainer's explicit approval for each step.
- Initial state: the state in V1. Before each step the GitHub API reported `torrust-runner-01` `online` and `busy: false`, and no Container or Testing run was queued or in progress. Other jobs had run on the runner at about 09:17 and 09:23 UTC, so the check was repeated before every step.
- Status: `DONE`

#### Steps Performed

The maintainer approved, in chat, steps 1 to 4 including the dockerd restart. All on 2026-10-27 UTC:

1. 10:10:19 `rm -rf /home/runner/.cache/torrust-tracker/container-target` (the documented cleanup of the host Cargo target cache).
2. 10:10:28 `docker stop -t 20` and `docker rm` of the four `torrust-tracker:local` containers.
3. 10:10:58 `systemctl restart docker.service` (live restore is disabled, so it stops running containers; none remained).
4. 10:11:37 `systemctl start docker-ci-prune.service`.

#### Observed Result

```text
                          df / (used, avail, use%)   docker system df: Build Cache (total, active, reclaimable)
before (10:10:09)         288G, 49M,  100%           487, 487, 253.6GB, 0B
after step 1 (10:10:21)   265G, 23G,   93%           unchanged
after step 2 (10:10:46)   263G, 26G,   92%           487, 487, 253.6GB, 0B   (117 Active snapshots left)
after step 3 (10:11:27)   119G, 170G,  42%           137, 0, 100GB, 100GB    (no Active snapshots; 198 Committed)
after step 4 (10:11:38)   119G, 170G,  42%           137, 0, 100GB, 100GB
```

The prune in step 4 reported `Total: 0B` for the build cache. That is expected: the cache, 100 GB, is under the 120 GB `--max-used-space` target. Docker, containerd, and the runner service were `active` after the restart, GitHub reported the runner `online` and not busy, and the daemon log shows `Initializing buildkit` at 10:10:59 and `Completed buildkit initialization` at 10:10:59, with no errors apart from an unrelated fsverity warning. `docker.service` had consumed 6 h 36 min CPU and a 1.9 GB memory peak over 32 days.

#### Conclusion

The disk is recovered: 42% used with 170 GB free, and the cache is reclaimable again. Removing the containers (step 2) changed nothing in the cache. Restarting dockerd (step 3) released the pinned state: the 117 Active snapshots disappeared, the build cache fell by 153.6 GB (253.6 GB to 100 GB), and `df` used space fell by 144 GiB, all without any prune. That identifies the pinned state as orphaned references that BuildKit inside dockerd holds until the daemon restarts. This restores service only; it does not stop the state from accumulating again.

## Failures and Follow-up

- An earlier chat summary read the container-prune `Total reclaimed space: 0B` lines as the build-cache result. V1 uses the `docker[<pid>]: Total:` lines, which are the build-cache prune, and the health report was written from those.
- The four-container hypothesis recorded in the first draft of this specification and in the first health report was wrong. The health report is not edited because reports are immutable; the next report states the corrected cause.
- A permission gate denied two batched collection commands (the cause was not identified) and, later, one local Docker command. The collection commands were re-run one at a time, and the local Docker commands ran after the maintainer approved the local experiment.
- Not yet known: what makes a build leave an Active snapshot behind (V2), and how fast the state accumulates again. Both are inputs to T3 to T5.
