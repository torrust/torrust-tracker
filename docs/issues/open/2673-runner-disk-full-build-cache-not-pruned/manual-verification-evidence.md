---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2673-runner-disk-full-build-cache-not-pruned/ISSUE.md
last-updated-utc: 2026-10-27 09:50
---

# Manual Verification Evidence

<!-- cspell:ignore ardinghelli inodes journalctl keldysh moby overlayfs -->

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

### V2 - Reproduce the leak: cancel a container job and look for leftovers

- Goal: confirm or reject the hypothesis that a cancelled job leaves a container or build state behind.
- Initial state: to be recorded when run.
- Status: `TODO`

#### Steps Performed

To be recorded when executed. Only run while the runner is idle and with the maintainer's approval.

#### Observed Result

```text
Not yet run.
```

#### Conclusion

Pending.

### V3 - Like-for-like recheck after the fix

- Goal: repeat V1 and V2 after the last production change, and after three daily prune runs.
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

## Failures and Follow-up

None recorded at drafting time. One note from the collection: an earlier chat summary read the container-prune `Total reclaimed space: 0B` lines as the build-cache result. The table above uses the `docker[<pid>]: Total:` lines, which are the build-cache prune, and the health report was written from those.
