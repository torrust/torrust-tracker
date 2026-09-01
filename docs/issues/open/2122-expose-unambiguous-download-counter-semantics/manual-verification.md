# Manual Verification Evidence

**Date:** 2026-09-01 11:35 UTC
**Tracker revision:** implementation worktree, pending signed commit
**Issue:** #2122

## Safety

Do not record API tokens, passwords, private keys, connection strings, or other
secrets. Replace each secret with `{REDACTED}` in commands, configuration, and
HTTP requests.

## Local Environment

- Operating system: Linux
- Tracker command: `cargo run -- --config {CONFIG_PATH}`
- Working directory: repository root
- Configuration source: `TORRUST_TRACKER_CONFIG_TOML_PATH={CONFIG_PATH}`
- Configuration: use the documented v3 configuration with SQLite. For M1 omit
  `[core.database]` and set `persistent_torrent_completed_stat = false`; for
  M2 retain the same SQLite `path` across both starts and set it to `true`.

## M1 - Disabled Persistence

**Status:** `BLOCKED`

### Commands

```text
TORRUST_TRACKER_CONFIG_TOML_PATH={M1_CONFIG_PATH} cargo run
{complete one download with tracker-client}
curl -H 'Authorization: Bearer {REDACTED}' http://127.0.0.1:1212/api/v1/stats
curl -H 'Authorization: Bearer {REDACTED}' 'http://127.0.0.1:1212/api/v1/metrics?format=prometheus'
{stop and rerun the same command, then repeat both requests}
```

### Requests And Responses

```text
Pending local tracker execution. Automated server coverage proves the disabled
configuration response and persisted-metric omission.
```

### Result

Blocked pending an operator-provided local v3 configuration and a complete
download event. Expected: `completed_in_session` resets after restart,
`completed_persisted` is zero, `completed_persisted_enabled` is false, and the
persisted Prometheus sample is absent.

## M2 - Enabled Persistence

**Status:** `BLOCKED`

### Commands

```text
TORRUST_TRACKER_CONFIG_TOML_PATH={M2_CONFIG_PATH} cargo run
{complete one download with tracker-client}
curl -H 'Authorization: Bearer {REDACTED}' http://127.0.0.1:1212/api/v1/stats
curl -H 'Authorization: Bearer {REDACTED}' 'http://127.0.0.1:1212/api/v1/metrics?format=prometheus'
{stop and restart using the identical SQLite database path, then repeat requests}
```

### Requests And Responses

```text
Pending local tracker execution. Automated server coverage proves that an
enabled zero persisted count is exported.
```

### Result

Blocked pending a local SQLite run. Expected: `completed_persisted_enabled` is
true; the persisted count survives restart; and its Prometheus sample is
present even when the observed count is zero.

## M3 - Legacy Migration

**Status:** `BLOCKED`

### Commands

```text
Repeat the authenticated stats and Prometheus metrics requests from M1 and M2.
```

### Requests And Responses

```text
Pending M1/M2 execution.
```

### Result

Blocked pending M1/M2 execution. Verify the deprecated legacy metric remains,
the new `in_session` sample remains process-lifetime, and the `persisted`
sample is capability-aware.
