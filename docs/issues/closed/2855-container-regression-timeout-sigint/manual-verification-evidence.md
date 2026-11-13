---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2855-container-regression-timeout-sigint/ISSUE.md
last-updated-utc: "2026-11-13 22:55"
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

Setup (not committed; the instrumented copy is not retained as an issue-local artifact because the
complete patch below reconstructs it exactly from the pre-fix script). The working tree's script
now carries the fix (PR #2857, merged into `develop`), so the pre-fix state is named by its
`develop` commit id, `ec1f58ec9` (the last commit before that merge), a durable reference that a
rebase cannot move:

```bash
git show ec1f58ec9:contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh > ./instrumented-copy.sh
patch ./instrumented-copy.sh <<'PATCH'
--- a/test-mounted-no-persistence-configuration.sh
+++ b/instrumented-copy.sh
@@ -16 +16 @@
-PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/../../../.." && pwd)
+PROJECT_ROOT=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
@@ -37,3 +37,4 @@ run_tracker() {
         "${IMAGE_TAG}" || exit_status=$?

+    echo "RUN_TRACKER_STATUS=${exit_status}" >&2
     test "${exit_status}" -eq 0 -o "${exit_status}" -eq 124
PATCH
```

`PROJECT_ROOT` is pinned because the `git show` redirect on the first line runs from, and so puts
the copy in, the repository root instead of the script's nested path under that root; the
`run_tracker` call itself is unchanged, only instrumented to print the real exit status before the
accept/reject `test`.

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
CI's script exits 1 because of the `test` on line 39 at `ec1f58ec9` (the last `develop` commit
before PR #2857's fix merged) of
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

Ran the script with the fix from `fix(container): [#2855] stop the tracker with timeout
--foreground` (PR #2857, `timeout --foreground --signal=INT --kill-after=3s 10s ...`), against the
same `tt:repro` image, once under each `timeout`:

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

Capture time: 2026-11-13 22:18:22 UTC (`date -u`, run immediately before the commands below).

```text
$ docker ps -a --format '{{.ID}} {{.Status}} {{.CreatedAt}}'
d7d7f841aec2 Up 3 hours (healthy) 2026-11-13 19:37:32 +0000 UTC
70cc80bca209 Up 7 hours (healthy) 2026-11-13 15:32:30 +0000 UTC
5e006c603754 Up 9 hours (healthy) 2026-11-13 13:46:16 +0000 UTC
[... 29 more lines, newest to oldest ...]
a040534e2abb Up 38 hours (healthy) 2026-11-12 08:37:16 +0000 UTC
2212f3f2e8ef Up 2 days (healthy)   2026-11-11 09:45:50 +0000 UTC

$ docker ps -a --filter status=exited -q | wc -l
0

$ docker ps -a -q | wc -l
34

$ for c in $(docker ps -aq); do
    echo "$c $(docker inspect "$c" --format '{{.Config.Image}} {{range .Mounts}}{{.Source}} {{end}}')"
  done
d7d7f841aec2 torrust-tracker:local /tmp/test-mounted-no-persistence-configuration.xwVxMj/etc /tmp/test-mounted-no-persistence-configuration.xwVxMj/lib /tmp/test-mounted-no-persistence-configuration.xwVxMj/log
[... 32 more lines, each torrust-tracker:local with a
     /tmp/test-mounted-no-persistence-configuration.<random>/{etc,lib,log} mount set ...]
2212f3f2e8ef torrust-tracker:local /tmp/test-mounted-no-persistence-configuration.5G4agY/etc /tmp/test-mounted-no-persistence-configuration.5G4agY/lib /tmp/test-mounted-no-persistence-configuration.5G4agY/log
```

All 34 running containers (none stopped) carry the `torrust-tracker:local` image and a
`/tmp/test-mounted-no-persistence-configuration.<random>/{etc,lib,log}` mount set, so every one of
them was started by this script, by its `mktemp -d` temp-directory naming pattern (script line
17). The oldest, `2212f3f2e8ef`, was created 2026-11-11 09:45:50 UTC — before the last known-green
Container run (2026-11-11 16:30 UTC, V4) — so this leftover alone predates the regression and is
not explained by the SIGINT-forwarding bug fixed here; it must come from an earlier, separate
failure of the same script. The remaining 33 containers were created between 2026-11-12 08:37 UTC
(after the first red run, V4) and 2026-11-13 19:37 UTC, consistent with (but, for any individual
container, not proven to be caused by) this regression. The fix (`--foreground`), merged in
PR #2857, stops new leftovers of this kind; this capture, taken after that merge, still shows the
pre-fix backlog because the fix does not clean up containers already running. Cleaning the
existing backlog is left to the maintainer or a follow-up, since it changes state on the shared
runner and was not requested as part of this fix.
