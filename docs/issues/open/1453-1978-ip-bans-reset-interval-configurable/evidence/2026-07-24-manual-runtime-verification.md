---
semantic-links:
  skill-links:
    - run-tracker-locally
  related-artifacts:
    - issue #1453
    - docs/issues/open/1453-1978-ip-bans-reset-interval-configurable/ISSUE.md
    - src/app.rs
    - src/bootstrap/jobs/udp_tracker_server.rs
    - packages/udp-server/src/server/launcher.rs
    - share/default/config/tracker.development.sqlite3.toml
---

# Manual Runtime Verification — 2026-07-24

## Scope

This record captures manual verification scenario M2 from the issue specification:
start the tracker with two UDP listeners and verify that it starts exactly one
application-owned IP-ban cleanup job.

The temporary configuration and raw terminal log were created under `.tmp/`, which
is git-ignored. This document retains the commands, relevant configuration changes,
and observed output as the durable evidence.

## Environment

- Workspace: `torrust-tracker-agent-01`
- Branch: `1453-ip-bans-reset-interval`
- Implementation commit: `7d7982d0006ff1bb15fe6937392de729d7b4a8fe`
- Configuration baseline: `share/default/config/tracker.development.sqlite3.toml`

## Procedure

1. Created a temporary copy of the development SQLite configuration in `.tmp/`.
2. Changed the UDP listener addresses to `127.0.0.1:16868` and `127.0.0.1:16969`
   to avoid collisions with normal local services. Changed the HTTP and API ports
   similarly.
3. Started the tracker with the temporary configuration and captured its output:

   ```text
   TORRUST_TRACKER_CONFIG_TOML_PATH="$PWD/.tmp/1453-runtime-config-Tg8fjI.toml" \
     RUST_LOG=info cargo run --bin torrust-tracker 2>&1 | tee "$PWD/.tmp/1453-runtime.log"
   ```

4. Stopped the interactive process after startup with `Ctrl+C`.
5. Counted the cleanup-start log entries and displayed the relevant startup lines:

   ```text
   grep -c 'Starting UDP IP-ban cleanup job' .tmp/1453-runtime.log
   grep -E 'Starting UDP IP-ban cleanup job|Started on: udp://127\.0\.0\.1:(16868|16969)' \
     .tmp/1453-runtime.log
   ```

## Relevant Configuration

```toml
[[udp_trackers]]
bind_address = "127.0.0.1:16868"
tracker_usage_statistics = true

[[udp_trackers]]
bind_address = "127.0.0.1:16969"
tracker_usage_statistics = true
```

## Observed Output

```text
1
2026-07-24T15:59:10.445058Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=86400
2026-07-24T15:59:10.445438Z  INFO run_with_graceful_shutdown{cookie_lifetime=120s}: UDP TRACKER: Started on: udp://127.0.0.1:16868
2026-07-24T15:59:10.445542Z  INFO run_with_graceful_shutdown{cookie_lifetime=120s}: UDP TRACKER: Started on: udp://127.0.0.1:16969
```

## Result

**Passed.** Two configured UDP listeners started, while the log contained exactly
one `Starting UDP IP-ban cleanup job` entry. The recorded interval was the expected
current bootstrap default of `86400` seconds. This confirms M2 and supports AC3 and
AC4: cleanup is application-owned rather than spawned once per UDP listener.
