---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/3019-ci-sccache-startup-fallback/ISSUE.md
last-updated-utc: "2026-11-25 08:37"
---

# Manual Verification Evidence

## Purpose

Record real verification of the `sccache` startup failure in bare CI builds and of its fallback.
Every command and output below was actually run; nothing is simulated.

## Preserving Verification Artifacts

The local runs used a throw-away Cargo project in an agent scratch directory. Everything they
rely on is embedded here: the config file, the commands and their output.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-24 07:27
- Artifact under test: the `sccache` client and server, with the `develop` workflows at
  b7890cfbf (merge of PR 3001). The hosted runs (V3 to V6) will name the commits they ran on by
  their Conventional Commit subject.
- Operating system / environment: Linux 7.0.0-43-generic, stable Rust toolchain
  (`rustc 1.100.0 (0ef1bc386 2026-11-09)`), `sccache 0.13.0` from the distribution package.
  CI installs `sccache` v0.20.0. The code paths used here (`connect_or_start_server`,
  `Command::StartServer`, `SCCACHE_IGNORE_SERVER_IO_ERROR`) were read in the v0.20.0 source;
  the local 0.13.0 binary prints the same error and behaves the same way.
- Prerequisites and setup performed:
  - A new binary crate made with `cargo new hello`.
  - `SCCACHE_SERVER_PORT=4299`, so the runs never touch a server on the default port.
  - `SCCACHE_CONF` points to this file, which gives the client 1 ms to wait for server startup
    instead of the default 10 s. That forces the same timeout a slow cache service causes in CI,
    without needing the cache service:

    ```toml
    server_startup_timeout_ms = 1
    ```

## Original Symptom (hosted)

From the job logs, fetched with
`gh-bot api repos/torrust/torrust-tracker/actions/jobs/<job-id>/logs`:

| Job id       | Workflow run, attempt                | Job                                | Error line (UTC)         |
| ------------ | ------------------------------------ | ---------------------------------- | ------------------------ |
| 137973679358 | `OS Compatibility` 45253124047, 2    | `Build on ubuntu-latest (nightly)` | 2026-11-23T20:00:51.378Z |
| 138047313315 | `OS Compatibility` 45253124047, 3    | `Build on macos-latest (nightly)`  | 2026-11-23T22:03:48.755Z |
| 138029839665 | `Testing` 45270620775                | `Unit (stable)`                    | 2026-11-23T21:33:48.945Z |
| 138047391738 | `Testing` 45253124156                | `Unit (stable)`                    | 2026-11-23T22:03:50.897Z |

Job 138047391738, `Download Dependencies` step (`cargo fetch --verbose`):

```text
2026-11-23T22:03:39.6281855Z try to setup sccache version: v0.20.0
...
2026-11-23T22:03:40.1638929Z     Updating crates.io index
2026-11-23T22:03:50.8975292Z error: process didn't exit successfully: `sccache /home/runner/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc -vV` (exit status: 2)
2026-11-23T22:03:50.8976325Z --- stderr
2026-11-23T22:03:50.8977210Z sccache: error: Timed out waiting for server startup. Maybe the remote service is unreachable?
2026-11-23T22:03:50.8978123Z Run with SCCACHE_LOG=debug SCCACHE_NO_DAEMON=1 to get more information
2026-11-23T22:03:50.9165945Z ##[error]Process completed with exit code 101.
```

The gap between `Updating crates.io index` and the error is about 10.7 s: the 10 s startup
timeout.

## Verification Processes

The steps ran one after another in one shell, in the throw-away crate.

### V1 - Local reproduction of the startup timeout

- Goal: observe the CI error with a real `sccache`.
- Initial state: no server on port 4299 (`sccache --stop-server` run first), clean target dir.
- Status: `DONE` - **Reproduced**

#### Steps Performed

1. 07:27:46.610 - `RUSTC_WRAPPER=sccache cargo build`

#### Observed Result

```text
error: process didn't exit successfully: `sccache /home/josecelano/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc -vV` (exit status: 2)
--- stderr
sccache: error: Timed out waiting for server startup. Maybe the remote service is unreachable?
Run with SCCACHE_LOG=debug SCCACHE_NO_DAEMON=1 to get more information

cargo exit=101 end 07:27:46.657
```

#### Conclusion

Same command, error and exit codes as the hosted jobs.

### V2 - `SCCACHE_IGNORE_SERVER_IO_ERROR`, then the planned fallback

- Goal: check whether `SCCACHE_IGNORE_SERVER_IO_ERROR` avoids the failure, and whether the
  planned fallback (start the server explicitly, use `sccache` only if it started) lets the
  build pass.
- Initial state: same as V1.
- Status: `DONE`

#### Steps Performed

1. 07:27:46.659 - `SCCACHE_IGNORE_SERVER_IO_ERROR=1 RUSTC_WRAPPER=sccache cargo build`
2. 07:27:46.702 - the fallback, with an empty file standing in for `$GITHUB_ENV`, then the
   same build with the variables the file holds:

   ```bash
   if SCCACHE_GHA_ENABLED=false SCCACHE_IDLE_TIMEOUT=0 sccache --start-server; then
     echo "RUSTC_WRAPPER=sccache" >> "$GITHUB_ENV"
   else
     sccache --stop-server >/dev/null 2>&1 || true
     echo "::warning title=sccache disabled::The sccache server did not start; building without the compilation cache."
   fi
   set -a; . "$GITHUB_ENV"; set +a
   cargo clean -q; cargo build
   ```

#### Observed Result

```text
## V2a start 07:27:46.659
error: process didn't exit successfully: `sccache /home/josecelano/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin/rustc -vV` (exit status: 2)
--- stderr
sccache: error: Timed out waiting for server startup. Maybe the remote service is unreachable?
Run with SCCACHE_LOG=debug SCCACHE_NO_DAEMON=1 to get more information

cargo exit=101 end 07:27:46.700
## V2b start 07:27:46.702
sccache: Starting the server...
sccache: error: Timed out waiting for server startup
::warning title=sccache disabled::The sccache server did not start; building without the compilation cache.
GITHUB_ENV lines: 0
RUSTC_WRAPPER=<unset>
   Compiling hello v0.1.0 (...)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s
cargo exit=0 end 07:27:46.832
```

An earlier run of the same fallback without the 1 ms config started the server
(`sccache: Listening on address 127.0.0.1:4299`), wrote `RUSTC_WRAPPER=sccache` to the file, and
the build went through `sccache` (`Compile requests 7` in `sccache --show-stats`).

#### Conclusion

`SCCACHE_IGNORE_SERVER_IO_ERROR=1` does not help: the startup timeout is still fatal, as the
v0.20.0 source shows. `sccache --start-server` exits non-zero on the same timeout, so it is a
reliable check, and the build passes without the wrapper.

### V3 - Hosted red run

- Status: `DONE`
- First attempt, on a `josecelano-bot` fork branch: branch `3019-verify-red`, upstream
  `develop` at the merge of PR #3020 plus one temporary commit that adds a step to the `unit`
  job of `testing.yaml`. The step writes `server_startup_timeout_ms = 1` to a config file and
  exports `SCCACHE_CONF` before the sccache steps. On 2026-11-24 at 13:05 UTC the push
  succeeded, but no workflow run started: the commit's only check suite belongs to the `claude`
  app, with no GitHub Actions suite. The fork's Actions permissions report `enabled: true` and
  `allowed_actions: all`, and the fork's latest workflow run on any branch is from 2026-10-30.
- The maintainer then chose a throwaway draft PR from the same branch:
  <https://github.com/torrust/torrust-tracker/pull/3033>. `Testing` run
  <https://github.com/torrust/torrust-tracker/actions/runs/45444358221>:
  - `Unit (nightly)`, job 138630276169: **failure** after 23 s. Its log shows
    `sccache: error: Timed out waiting for server startup. Maybe the remote service is unreachable?`,
    the error of the 2026-11-23 failures, and the post step reports
    `Cache location ghac`, so the GitHub Actions cache backend was selected.
  - `Unit (stable)`, job 138630276239: cancelled when the nightly job failed (matrix fail-fast),
    so it gives no result of its own.
- Conclusion: the forced timeout reproduces the original failure on the hosted runner with the
  current `Enable sccache` step (red).

### V4 - Hosted green run

- Status: `DONE`
- Throwaway draft PR <https://github.com/torrust/torrust-tracker/pull/3034>: the implementation
  branch plus the same temporary commit as V3. `Testing` run
  <https://github.com/torrust/torrust-tracker/actions/runs/45444371578>:
  - `Unit (nightly)`, job 138630252048: **success** (17:16:33 to 17:30:15 UTC).
  - `Unit (stable)`, job 138630251811: **success** (17:17:09 to 17:32:32 UTC).
  - In both jobs the `Enable sccache` step logs
    `sccache: error: Timed out waiting for server startup`, then the job keeps building. The run
    shows the warning annotation `sccache disabled: The sccache server did not start, so this job
    builds without the compilation cache. See issue #3019.`
  - The post step reports `Cache location Local disk` and `0 hits, 0 misses`: no `sccache`
    wrapper was used, and the post-step server did not need the cache service.
- Conclusion: with the fix, the same forced timeout costs the cache, not the job (green).

### V5 - Hosted normal run

- Status: `DONE`
- PR <https://github.com/torrust/torrust-tracker/pull/3040>, first head, no temporary commit.
  All workflow runs passed: `Testing` 45452040750, `OS Compatibility` 45452040716,
  `Database Compatibility` 45452040507, `Database Benchmarking` 45452040796. `Docker E2E` and
  the coverage package jobs were skipped, as expected for a pull request to `develop` (V6).
- No job showed the `sccache disabled` warning. The post step of each job reported these
  `sccache stats` annotations:

| Job                                       | Hit rate | Hits | Misses | Errors |
| ----------------------------------------- | -------- | ---- | ------ | ------ |
| `Unit (nightly)`                          | 36 %     | 1144 | 1434   | 607    |
| `Unit (stable)`                           | 45 %     | 1348 | 1134   | 491    |
| `Build on ubuntu-latest (nightly)`        | 59 %     | 623  | 307    | 124    |
| `Build on ubuntu-latest (stable)`         | 48 %     | 572  | 356    | 262    |
| `Build on macos-latest (nightly)`         | 21 %     | 243  | 687    | 222    |
| `Build on macos-latest (stable)`          | 30 %     | 309  | 619    | 113    |
| `Build on windows-latest (nightly)`       | 15 %     | 139  | 653    | 122    |
| `Build on windows-latest (stable)`        | 0 %      | 0    | 790    | 244    |
| `Database Compatibility MySQL (8.0)`      | 65 %     | 392  | 106    | 102    |
| `Database Compatibility MySQL (8.4)`      | 27 %     | 190  | 308    | 208    |
| `Database Compatibility PostgreSQL (14)`  | 89 %     | 466  | 32     | 28     |
| `Database Compatibility PostgreSQL (15)`  | 68 %     | 400  | 98     | 94     |
| `Database Compatibility PostgreSQL (16)`  | 67 %     | 399  | 99     | 99     |
| `Database Compatibility PostgreSQL (17)`  | 65 %     | 384  | 114    | 96     |
| `Persistence Benchmark SQLite3`           | 26 %     | 184  | 311    | 203    |
| `Persistence Benchmark MySQL`             | 50 %     | 289  | 206    | 79     |
| `Persistence Benchmark PostgreSQL`        | 29 %     | 201  | 294    | 206    |

- The full stats of `Unit (nightly)` report `cache_location: ghac` (the GitHub Actions cache
  backend) and `version: 0.20.1`: `mozilla-actions/sccache-action` now installs sccache v0.20.1,
  not the v0.20.0 read for the analysis. The errors are `cache_read_errors` (607) and
  `cache_write_errors` (789): the cache service failed requests.
- Comparison with runs that do not have this change, read the same way at 18:4x UTC:
  - `Unit` jobs of other pull requests between 17:40 and 18:08 UTC show the same pattern:
    34 % to 61 % hits with 349 to 867 errors. A pull request run at 14:45 UTC had 98 % hits and
    0 errors. The low hit rate and the errors come from the cache service at that time, not
    from this change.
  - On `windows-latest`, two other pull requests at 18:23 and 18:27 UTC report
    `0 hits, 0 misses, 0 errors`: before this change no compilation went through `sccache` on
    Windows. With the composite action (`shell: bash`) the Windows jobs use it. The cold cache
    explains the low hit rates there. The old step was a `run:` step with the default Windows
    shell, PowerShell; this was not investigated further.
- Conclusion: when the server starts, the jobs use `sccache` with the GitHub Actions cache
  backend, and every bare job that enabled `sccache` now uses the shared action and passes,
  including the `windows-latest` jobs.

### V6 - Hosted runs of jobs the PR checks do not run

- Status: `PARTIAL`: `coverage.yaml` done; `docker-e2e` not observed when the spec was archived.
- `coverage.yaml`, run <https://github.com/torrust/torrust-tracker/actions/runs/45575925023>
  for the merge push of PR #3040 to `develop` (2026-11-25):
  - `Generate Coverage Report`, job 139073721007: **success** (08:24:20 to 08:35:05 UTC). No
    `sccache disabled` warning annotation, so the server started.
  - Its stats report `cache_location: ghac` and `version: 0.20.1`, but
    `0 hits, 624 misses, 275 errors`, with `cache_read_errors` 275, `cache_write_errors` 624 and
    `cache_writes` 0: the cache service failed every write and many reads.
  - The cache service was degraded for the other jobs of the same push too: the
    `OS Compatibility` builds had 0 % to 35 % hits with 135 to 441 errors each. The two
    `coverage` runs before the change had 61 % and 66 % hits. The 0 hits come from the cache
    service, not from the change: the backend, key inputs and `RUSTFLAGS` of the job did not
    change.
- `testing.yaml` → `docker-e2e`: its `sccache` steps run only for a Dependabot branch push in
  `torrust/torrust-tracker`. The last ones (2026-11-24 17:04 UTC and earlier) ran before the
  merge, and none ran between the merge and the archive. It runs the same composite action as
  the `unit` job on the same `ubuntu-latest` image, which V4 and V5 verified.
- Windows note: on the merge push, `Build on windows-latest (stable)` again had 0 hits, with
  435 successful `cache_writes`. GitHub Actions caches written by a pull request branch are not
  readable from `develop`, and `develop` had no Windows `sccache` entries before this push, so
  a cold cache is expected there.

## Failures and Follow-up

None yet.
