---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/3019-ci-sccache-startup-fallback/ISSUE.md
last-updated-utc: "2026-11-24 17:04"
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

- Status: `IN_PROGRESS`
- First attempt, on a `josecelano-bot` fork branch: branch `3019-verify-red`, upstream
  `develop` at the merge of PR #3020 plus one temporary commit that adds a step to the `unit`
  job of `testing.yaml`. The step writes `server_startup_timeout_ms = 1` to a config file and
  exports `SCCACHE_CONF` before the sccache steps. On 2026-11-24 at 13:05 UTC the push
  succeeded, but no workflow run started: the commit's only check suite belongs to the `claude`
  app, with no GitHub Actions suite. The fork's Actions permissions report `enabled: true` and
  `allowed_actions: all`, and the fork's latest workflow run on any branch is from 2026-10-30.
- The maintainer then chose a throwaway draft PR from the same branch:
  <https://github.com/torrust/torrust-tracker/pull/3033>, waiting for its workflow runs.

### V4 - Hosted green run

- Status: `IN_PROGRESS`
- Throwaway draft PR <https://github.com/torrust/torrust-tracker/pull/3034>: the implementation
  branch plus the same temporary commit as V3. Waiting for its workflow runs.

### V5 - Hosted normal run

- Status: `TODO`

### V6 - Hosted runs of jobs the PR checks do not run

- Status: `TODO` (after the merge)
- `coverage.yaml`: the run for the merge push to `develop`.
- `testing.yaml` → `docker-e2e`: the next Dependabot branch push, the only event that runs its
  `sccache` steps in `torrust/torrust-tracker`.

## Failures and Follow-up

None yet.
