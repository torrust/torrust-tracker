---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2855-container-regression-timeout-sigint/ISSUE.md
last-updated-utc: "2026-11-13 11:20"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date (UTC): 2026-11-13
- Artifact under test: release image built from `develop` at the merge of #2841
  (`docker build --target release --tag tt:repro -f Containerfile .`, tagged `tt:repro`)
- OS: Ubuntu 26.04, Docker 28.3.3, `timeout` = uutils coreutils 0.10.0
  (`rust-coreutils 0.10.0-1ubuntu2~26.04.1`); GNU `timeout` reached as `/usr/bin/gnutimeout`
  (`gnu-coreutils 9.7`, a separate binary installed alongside uutils, not a PATH shim)

## Verification Processes

### V1 - Reproduce the failing step

- Status: `DONE` (outcome: **Reproduced**)

Edit applied to a working copy of the script (not committed; `PROJECT_ROOT` pinned to the
working directory, and the unchanged `run_tracker` call instrumented to print the real exit
status before the accept/reject `test`):

```diff
-    timeout --signal=INT --kill-after=3s 10s docker run --rm \
+    timeout --signal=INT --kill-after=3s 10s docker run --rm \
         --env USER_ID="$(id -u)" \
         --volume "${TEST_DIRECTORY}/etc:/etc/torrust/tracker:rw" \
         --volume "${TEST_DIRECTORY}/lib:/var/lib/torrust/tracker:rw" \
         --volume "${TEST_DIRECTORY}/log:/var/log/torrust/tracker:rw" \
         "${IMAGE_TAG}" || exit_status=$?

+    echo "RUN_TRACKER_STATUS=${exit_status}" >&2
     test "${exit_status}" -eq 0 -o "${exit_status}" -eq 124
```

Command: `IMAGE_TAG=tt:repro BUILD_IMAGE=false bash -x ./instrumented-copy.sh`

```text
+ exit_status=137
+ echo RUN_TRACKER_STATUS=137
RUN_TRACKER_STATUS=137
+ test 137 -eq 0 -o 137 -eq 124
```

(script exits 1; `set -e` stops at the failing `test`)

The container log ends at `Tracker shutdown signal handlers installed.` plus health-check
requests, with no `shutting down (SIGINT)` line, as in the failing CI logs (run 43299605132).
CI's script exits 1 because of the `test` on line 39 of
`contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh`, not because
`docker run` itself returned 1.

### V2 - Isolate the cause

- Status: `DONE`

All runs below use the mounted `share/default/config/tracker.container.no-persistence.toml`
(health-check only) and the `tt:repro` image; `<container>` is the one started by a plain
`docker run -d ...` with the same volumes, waited 4s to reach `Started on: http://...1313`
before each signal.

| # | Command | Observed |
| --- | --- | --- |
| 1 | `docker kill --signal=INT <container>` | `exit=0 status=exited`; log has `shutting down (SIGINT) ...` then `successfully shutdown.` |
| 2 | `docker kill --signal=TERM <container>` | `exit=0 status=exited`; log has `shutting down (SIGTERM) ...` then `successfully shutdown.` |
| 3 | `kill -INT <pid>` of a `docker run --rm ...` client running in the background | `client_exit=0`; log has `shutting down (SIGINT) ...` then `successfully shutdown.` |
| 4 | `/usr/bin/gnutimeout --signal=INT --kill-after=3s 6s docker run --rm ...` | `exit=124`; log has `shutting down (SIGINT) ...` then `successfully shutdown.` |
| 5 | `/usr/bin/gnutimeout --foreground --signal=INT --kill-after=3s 6s docker run --rm ...` | `exit=124`; log has `shutting down (SIGINT) ...` then `successfully shutdown.` |
| 6 | `timeout --signal=INT --kill-after=3s 6s docker run --rm ...` (uutils, no flag) | **`exit=137`; no `shutting down` line in the log** |
| 7 | `timeout --foreground --signal=INT --kill-after=3s 6s docker run --rm ...` (uutils) | `exit=124`; log has `shutting down (SIGINT) ...` then `successfully shutdown.` |

Row 6 is the one case that does not shut the tracker down; it is the invocation the script used
before the fix (row 7's flag is the fix).

Conclusion: the tracker receives and handles SIGINT and SIGTERM correctly in every case where a
signal reaches it (rows 1-5, 7); only row 6 — uutils `timeout` without `--foreground` — ends
without the tracker ever logging a shutdown. No signal-delivery probe (for example
`/proc/<pid>/status` on the `docker` client under `timeout`) was recorded, so whether the SIGINT
reaches the client and is not forwarded, or never reaches the client at all, remains a hypothesis;
the fix (`--foreground`) is correct under either reading, since row 7 shows it resolves the
symptom.

### V3 - Recheck after the fix

- Status: `DONE`

Ran the unmodified, fixed script (`timeout --foreground --signal=INT --kill-after=3s 10s ...`)
against the same `tt:repro` image, once under each `timeout`:

```text
$ IMAGE_TAG=tt:repro BUILD_IMAGE=false bash contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh
(exit 0)
mounted-no-persistence-config-preserved-without-sqlite-artifacts
unselected-sqlite-targets-remain-unchanged-across-transitions

$ PATH=<dir with a timeout symlink to /usr/bin/gnutimeout>:$PATH IMAGE_TAG=tt:repro BUILD_IMAGE=false bash contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh
(exit 0)
mounted-no-persistence-config-preserved-without-sqlite-artifacts
unselected-sqlite-targets-remain-unchanged-across-transitions
```

The second run points the script's bare `timeout` at GNU `timeout` through a directory prepended
to `PATH` containing only a `timeout -> /usr/bin/gnutimeout` symlink, so the unmodified script
(which calls `timeout` by bare name) resolves to the GNU implementation without editing the
script.

### V4 - Runner package history

- Status: `DONE`

Three separate commands, run over `ssh torrust-runner-01`, each shown with its own output
(read-only; maintainer-approved per session):

```text
$ dpkg -l | grep coreutils
ii  coreutils                      9.5-1ubuntu2+0.0.0~ubuntu25                all     coreutils meta package
ii  coreutils-from-uutils          0.0.0~ubuntu25                             all     coreutils from the uutils project
ii  gnu-coreutils                  9.7-3ubuntu2.1                             amd64   GNU core utilities
ii  rust-coreutils                 0.10.0-1ubuntu2~26.04.1                    amd64   Universal coreutils utils, written in Rust

$ readlink -f $(command -v timeout); timeout --version
/usr/lib/cargo/bin/coreutils/timeout
timeout (uutils coreutils) 0.10.0

$ grep -h -B2 -A3 "rust-coreutils" /var/log/apt/history.log*
Start-Date: 2026-11-12  06:36:55
Commandline: /usr/bin/unattended-upgrade
Upgrade: rust-coreutils:amd64 (0.8.0-0ubuntu3, 0.10.0-1ubuntu2~26.04.1)
End-Date: 2026-11-12  06:36:57

Start-Date: 2026-11-12  06:37:00
```

The `rust-coreutils` upgrade (0.8.0 to 0.10.0) ran unattended at 2026-11-12 06:36-06:37 UTC,
between the last green Container run (2026-11-11 16:30) and the first red one (2026-11-12 08:36).
Whether 0.8.0 passed the unmodified script was not tested; only the inference from the green runs
supports it.

### V5 - Leftover containers on the runner

- Status: `DONE`

`docker container prune` (in `contrib/dev-tools/self-hosted-runner/docker-ci-prune.sh`) only
removes **stopped** containers; `docker run --rm`'s container keeps running when its attached
client is SIGKILLed (row 6 above), so `--rm` never fires and the prune filter never matches it.
On the runner:

```text
$ docker ps -a --format '{{.ID}} {{.Status}} {{.CreatedAt}}' | head -5
783e39c1bd04 Up 22 minutes (healthy)  2026-11-13 10:52:04 +0000 UTC
114539ac5359 Up 49 minutes (healthy)  2026-11-13 10:25:01 +0000 UTC
a4ecdde3c538 Up About an hour (healthy) 2026-11-13 10:11:37 +0000 UTC
...
2212f3f2e8ef Up 2 days (healthy)       2026-11-11 09:45:50 +0000 UTC

$ docker ps -a -q | wc -l
26
```

26 containers were running (none stopped), the oldest since 2026-11-11 09:45 UTC, a time before
this regression's first red run; these containers' age range is consistent with, but not proven
to be entirely caused by, the bug fixed here (the runner also runs release-image containers from
other jobs that are not `--rm`'d by this script). The fix (`--foreground`) stops new leftovers
from this script; it does not clean up the ones already running. Cleaning the existing backlog is
left to the maintainer or a follow-up, since it changes state on the shared runner and was not
requested as part of this fix.
