---
doc-type: guide
parent-epic: 1978
spec-path: docs/issues/open/1978-configuration-overhaul-epic/migration-guide.md
last-updated-utc: 2026-07-28
---

# Migrating from Configuration v2.0.0 to v3.0.0

This guide helps users migrate their Torrust Tracker configuration from schema
version `2.0.0` to `3.0.0`. Each section covers a breaking change, what to
update, and why.

> **Status**: Partially complete. Sections for completed subissues are filled in.
> Sections for pending subissues are marked with TODOs and will be completed as
> each subissue is implemented.

## Quick reference

| v2 field / section | v3 equivalent | Subissue | Status |
| --- | --- | --- | --- |
| `[core.net]` (global) | Per-tracker `[http_trackers.network]` / `[udp_trackers.network]` | #1640 | DONE |
| `tsl_config` | `tls_config` | #1981 | DONE |
| No public URL field | `public_url` on HTTP trackers, UDP trackers, and HTTP API | #1417 | DONE |
| `on_reverse_proxy` (global) | Per-HTTP-tracker `network.on_reverse_proxy` | #1640 | DONE |
| No logging style option | `[logging] trace_style` | #889 | DONE |
| `threshold` | `trace_filter` | #889 | DONE |
| No connection ID policy | `[udp_tracker_server] connection_id_validation` | #1136 | DONE |
| Hardcoded IP bans interval | `[udp_tracker_server] ip_bans_reset_interval_in_secs` | #1453 | IN_REVIEW |
| Flat `[core.database]` | Database enum with per-driver config | #1490 | TODO |
| No announce `ip` opt-in | Per-HTTP-tracker `use_ip_from_announce` | #1987 | TODO |

## Step 1: Update the schema version

Change the `schema_version` in your config file:

```toml
# v2
[metadata]
schema_version = "2.0.0"

# v3
[metadata]
schema_version = "3.0.0"
```

The tracker will reject configs with a schema version other than `3.0.0`.

## Step 2: Fix the TLS config typo

**Subissue**: #1981 — `tsl_config` → `tls_config`

The v2 schema contained a typo: `tsl_config`. This is corrected to `tls_config`
in v3. If your config has a `[http_trackers.tsl_config]` or
`[http_trackers.tls_config]` section, use the corrected name:

```toml
# v2 (typo)
[http_trackers.tsl_config]
ssl_cert_path = "..."
ssl_key_path = "..."

# v3 (corrected)
[http_trackers.tls_config]
ssl_cert_path = "..."
ssl_key_path = "..."
```

> **Note**: v2 compatibility is retained until the final cleanup (#1980). New
> installations should use `tls_config` directly.

## Step 3: Replace the global network block

**Subissue**: #1640 — Per-HTTP-tracker `on_reverse_proxy` setting

The global `[core.net]` section (including `on_reverse_proxy` and
`external_ip`) is **removed** in v3. These settings move to per-tracker
`network` blocks.

```toml
# v2
[core.net]
on_reverse_proxy = true
external_ip = "1.2.3.4"

# v3 — each tracker gets its own network block
[[http_trackers]]
bind_address = "0.0.0.0:7070"

[http_trackers.network]
on_reverse_proxy = true
external_ip = "1.2.3.4"

[[udp_trackers]]
bind_address = "0.0.0.0:6969"

[udp_trackers.network]
external_ip = "1.2.3.4"
```

If you had `on_reverse_proxy = false` (the default), you can omit the
`network` block entirely — the v3 defaults match the v2 defaults.

## Step 4: Add public URL fields (optional)

**Subissue**: #1417 — Include public service URL in configuration

If your tracker is behind a reverse proxy or load balancer, you can now
declare its public-facing URL. This is optional but recommended for
metrics, health checks, and API discoverability.

```toml
[[http_trackers]]
bind_address = "0.0.0.0:7070"
public_url = "https://tracker.example.com:443/announce"

[[udp_trackers]]
bind_address = "0.0.0.0:6969"
public_url = "udp://tracker.example.com:6969/announce"

[http_api]
bind_address = "127.0.0.1:1212"
public_url = "https://api.example.com:443"
```

The `public_url` field is typed — scheme validation is enforced at
deserialization. HTTP trackers require `http` or `https`; UDP trackers
require `udp`.

## Step 5: Update the logging configuration

**Subissue**: #889 — New config option for logging style

Two changes in the `[logging]` section:

1. **Rename** `threshold` → `trace_filter`
2. **Add** `trace_style` (optional, defaults to `"full"`)

```toml
# v2
[logging]
threshold = "info"

# v3
[logging]
trace_filter = "info"
trace_style = "full"
```

Supported `trace_style` values:

| Value | Description |
| --- | --- |
| `"full"` | Standard human-readable output (default) |
| `"pretty"` | Pretty-printed with colours |
| `"compact"` | Compact single-line output |
| `"json"` | Structured JSON output (for log aggregation) |

> **Breaking**: The old `threshold` key is rejected by v3. If you omit
> `trace_filter`, the default is `info`.

## Step 6: Configure UDP connection ID validation

**Subissue**: #1136 — Add configurable UDP connection ID validation policy

The v3 schema adds an optional `connection_id_validation` field to
`[udp_tracker_server]`. If omitted, the default is `"strict"` (same as
v2 behaviour).

```toml
# v2 — no equivalent; always strict

# v3 — explicit policy (optional)
[udp_tracker_server]
connection_id_validation = "strict"
```

Supported values: `"strict"`, `"disabled"`. Use `"disabled"` only for
isolated compatibility listeners that accept non-compliant clients.

## Step 7: Configure IP bans reset interval

**Subissue**: #1453 — IP bans reset interval configurable

The v3 schema adds `ip_bans_reset_interval_in_secs` to
`[udp_tracker_server]`. The default is `86400` (24 hours), matching the
previous hardcoded value.

```toml
# v2 — no equivalent; hardcoded to 24 hours

# v3 — explicit (optional)
[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
```

> **Note**: This setting is validated in the v3 schema but will not take
> effect at runtime until the final consumer migration (#1980). The value
> is currently read from the v3 default constant.

<!-- ────────────────────────────────────────────────────────────────────── -->
<!-- SECTIONS BELOW ARE TODO — to be filled as each subissue is implemented -->
<!-- ────────────────────────────────────────────────────────────────────── -->

## Step 8: Update the database configuration

**Subissue**: #1490 — Decompose database config and overhaul secrets

> **TODO**: This section will be filled after #1490 is implemented.
> The flat `[core.database]` block will be replaced with a per-driver
> enum structure (`sqlite3`, `mysql`, `postgresql`) and secrets will use
> the `secrecy` crate. The exact v3 syntax depends on the implementation.

## Step 9: Configure HTTP announce IP trust policy

**Subissue**: #1987 — Use peer IP from the HTTP announce `ip` parameter

> **TODO**: This section will be filled after #1987 is implemented.
> A new per-HTTP-tracker opt-in field will control whether the tracker
> trusts the client-provided `ip` parameter in announce requests.

## Final cleanup

**Subissue**: #1980 — Remove global re-exports, migrate consumers

> **TODO**: This section will document the internal cleanup steps.
> For end users, the main effect is that the tracker now ships with v3
> as the default schema and v2 configs are no longer accepted at runtime.

## Runtime observability

**Subissue**: #2023 — Expose configured public URLs in runtime observability

> **TODO**: This section will document how `public_url` values appear in
> health checks, metrics, and logs after the final cleanup.

## Migration checklist

Use this checklist to verify your configuration is ready for v3:

- [ ] `schema_version` set to `"3.0.0"`
- [ ] `tsl_config` renamed to `tls_config` (if applicable)
- [ ] Global `[core.net]` replaced with per-tracker `network` blocks
- [ ] `on_reverse_proxy` moved to per-HTTP-tracker `network` block (if `true`)
- [ ] `external_ip` moved to per-tracker `network` blocks (if set)
- [ ] `threshold` renamed to `trace_filter` in `[logging]`
- [ ] `trace_style` added to `[logging]` (optional, defaults to `"full"`)
- [ ] `public_url` added to trackers and API (optional, recommended for reverse proxies)
- [ ] `connection_id_validation` reviewed in `[udp_tracker_server]` (optional, defaults to `"strict"`)
- [ ] `ip_bans_reset_interval_in_secs` reviewed in `[udp_tracker_server]` (optional, defaults to `86400`)

## References

- [EPIC #1978 — Configuration Overhaul](EPIC.md)
- [ADRs](../../../adrs/README.md)
