---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2855-container-regression-timeout-sigint/ISSUE.md
last-updated-utc: "2026-11-13 10:10"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date (UTC): 2026-11-13
- Artifact under test: release image built from `develop` at the merge of #2841
  (`docker build --target release -f Containerfile .`)
- OS: Ubuntu 26.04, Docker 28.3.3, `timeout` = uutils coreutils 0.10.0
  (`rust-coreutils 0.10.0-1ubuntu2~26.04.1`); GNU `timeout` as `/usr/bin/gnutimeout`
  (`gnu-coreutils 9.7`)

## Verification Processes

### V1 - Reproduce the failing step

- Status: `DONE` (outcome: **Reproduced**)

Ran the script with `BUILD_IMAGE=false`, printing the exit status of each `run_tracker`:

```text
+ echo RUN_TRACKER_STATUS=137
RUN_TRACKER_STATUS=137
+ test 137 -eq 0 -o 137 -eq 124
(script exits 1)
```

The container log ends at `Tracker shutdown signal handlers installed.` plus health-check
requests, with no `shutting down (SIGINT)` line, as in the failing CI logs (run 43299605132).
CI's script exits 1 because of the final `test`, not because the tracker returned 1.

### V2 - Isolate the cause

- Status: `DONE`

| Command                                                        | Result                                        |
| -------------------------------------------------------------- | --------------------------------------------- |
| `docker kill --signal=INT <container>`                         | exit 0, `successfully shutdown`               |
| `docker kill --signal=TERM <container>`                        | exit 0, `successfully shutdown`               |
| `kill -INT <docker client pid>` (`docker run --rm` in the bg)  | `shutting down (SIGINT)`, `successfully shutdown` |
| `/usr/bin/gnutimeout --signal=INT --kill-after=3s 6s docker run ...`              | status 124, shutdown logged |
| `/usr/bin/gnutimeout --foreground --signal=INT ...`            | status 124, shutdown logged                   |
| `timeout --signal=INT --kill-after=3s 6s docker run ...` (uutils) | **status 137, no shutdown line**           |
| `timeout --foreground --signal=INT ...` (uutils)               | status 124, shutdown logged                   |

Conclusion: the tracker handles SIGINT; uutils `timeout` without `--foreground` does not get the
signal to the docker client.

### V3 - Recheck after the fix

- Status: `TODO`

### V4 - Runner package history

- Status: `DONE`

`ssh torrust-runner-01 'dpkg -l | grep coreutils; grep -h -B2 -A3 rust-coreutils /var/log/apt/history.log*'`:

```text
ii  rust-coreutils  0.10.0-1ubuntu2~26.04.1
/usr/bin/timeout -> ../lib/cargo/bin/coreutils/timeout   (timeout (uutils coreutils) 0.10.0)
Start-Date: 2026-11-12  06:36:55
Upgrade: rust-coreutils:amd64 (0.8.0-0ubuntu3, 0.10.0-1ubuntu2~26.04.1)
```
