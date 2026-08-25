---
doc-type: guide
parent-epic: 1978
spec-path: docs/issues/open/1978-configuration-overhaul-epic/configuration-v2-to-v3-migration.md
last-updated-utc: 2026-08-24
---

# Migrating from Configuration v2.0.0 to v3.0.0

This guide helps users migrate their Torrust Tracker configuration from schema
version `2.0.0` to `3.0.0`. Each section covers a breaking change, what to
update, and why.

> **Status**: Partially complete. Sections for completed subissues are filled in.
> Sections for pending subissues are marked with TODOs and will be completed as
> each subissue is implemented.

## Quick reference

| v2 field / section           | v3 equivalent                                                    | Subissue | Status    |
| ---------------------------- | ---------------------------------------------------------------- | -------- | --------- |
| `[core.net]` (global)        | Per-tracker `[http_trackers.network]` / `[udp_trackers.network]` | #1640    | DONE      |
| `tsl_config`                 | `tls_config`                                                     | #1981    | DONE      |
| No public URL field          | `public_url` on HTTP trackers, UDP trackers, and HTTP API        | #1417    | DONE      |
| `on_reverse_proxy` (global)  | Per-HTTP-tracker `network.on_reverse_proxy`                      | #1640    | DONE      |
| No logging style option      | `[logging] trace_style`                                          | #889     | DONE      |
| `threshold`                  | `trace_filter`                                                   | #889     | DONE      |
| No connection ID policy      | `[udp_tracker_server] connection_id_validation`                  | #1136    | DONE      |
| Hardcoded IP bans interval   | `[udp_tracker_server] ip_bans_reset_interval_in_secs`            | #1453    | IN_REVIEW |
| Per-listener UDP error limit | `[udp_tracker_server] max_connection_id_errors_per_ip`           | #2083    | IN_REVIEW |
| Flat `[core.database]`       | Database enum with per-driver config                             | #1490    | DONE      |
| No announce `ip` opt-in      | Per-HTTP-tracker opt-in field (TBD)                              | #1987    | TODO      |

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

| Value       | Description                                  |
| ----------- | -------------------------------------------- |
| `"full"`    | Standard human-readable output (default)     |
| `"pretty"`  | Pretty-printed with colours                  |
| `"compact"` | Compact single-line output                   |
| `"json"`    | Structured JSON output (for log aggregation) |

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

## Step 8: Move the UDP connection-ID error limit to the shared server section

**Subissue**: #2083 — Move UDP connection-ID error limit to shared server configuration

In v2, `max_connection_id_errors_per_ip` appears in every `[[udp_trackers]]`
entry. In v3, it must be declared once in `[udp_tracker_server]`. The tracker
uses one shared ban service for all UDP listeners, so a per-listener value would
misrepresent the effective policy and could make it depend on listener order.

```toml
# v2 — remove this field from every listener
[[udp_trackers]]
bind_address = "0.0.0.0:6969"
max_connection_id_errors_per_ip = 10

[[udp_trackers]]
bind_address = "0.0.0.0:6970"
max_connection_id_errors_per_ip = 10

# v3 — declare the shared policy once
[udp_tracker_server]
max_connection_id_errors_per_ip = 10
```

The default remains `10`. V3 rejects the old listener-scoped field rather than
accepting repeated values. The setting takes effect at runtime when #1980
activates v3 configuration consumers.

<!-- ────────────────────────────────────────────────────────────────────── -->
<!-- SECTIONS BELOW ARE TODO — to be filled as each subissue is implemented -->
<!-- ────────────────────────────────────────────────────────────────────── -->

## Step 9: Update the database configuration

**Subissue**: #1490 — Decompose v3 database configuration

The v3 `path` field is replaced by driver-specific fields. This makes the
database connection explicit and removes the requirement to percent-encode
password characters in a URL.

```toml
# v2 — a filesystem path or credential-bearing URL shared one field name
[core.database]
driver = "mysql"
path = "mysql://db_user:db_user_secret_password@mysql:3306/torrust_tracker"

# v3 — fields match the selected database driver
[core.database]
driver = "mysql"
host = "mysql"
port = 3306 # optional; defaults to 3306 for MySQL and 5432 for PostgreSQL
user = "db_user"
password = "db_user_password" # mandatory and non-empty
database = "torrust_tracker"
```

PostgreSQL uses the same component fields and defaults an omitted `port` to
`5432`:

```toml
# v2
[core.database]
driver = "postgresql"
path = "postgresql://postgres:postgres_password@postgres:5432/torrust_tracker"

# v3
[core.database]
driver = "postgresql"
host = "postgres"
user = "postgres"
password = "postgres_password"
database = "torrust_tracker"
```

SQLite retains its filesystem `path`:

```toml
[core.database]
driver = "sqlite3"
path = "/var/lib/torrust/tracker/database/sqlite3.db"
```

### Optional database representation and staged activation

**Subissue**: #999 — Optional v3 database configuration

V3 accepts an omitted `[core.database]` section and represents it as no
configured database. This is a schema/API change first; the runtime activation
is deliberately staged:

1. #999 introduces the optional representation and optional container
   dependencies while retaining an explicit temporary database bridge.
2. #1980 activates v3 runtime consumers with that bridge, preserving the
   existing effective database behavior during the configuration transition.
3. A small post-#1980 follow-up honors the omitted database at runtime when no
   persistence-required capability is enabled.

Until the activation follow-up is merged, do not interpret an omitted v3
database section as a persistence-free running tracker. The final activation
follow-up will document which capabilities require persistence, startup
diagnostics, and supported container behavior.

This is a breaking configuration change: MySQL and PostgreSQL URLs are not
accepted in v3. Move their URL components into the fields above. Do not use an
empty password: loading rejects missing and empty network database passwords.

The preceding `secrecy` refactor changes API-token Rust value types without
changing TOML syntax. #1490 then represents the isolated v3 database password
as a secret value, also without changing the TOML syntax shown above. Both must
be merged before the configuration crate's v3 public API is published.

## Step 10: Configure HTTP announce IP trust policy

**Subissue**: #1987 — Use peer IP from the HTTP announce `ip` parameter

> **Staged delivery**: #1987 currently supplies the protocol policy and keeps
> production explicitly disabled until #1980 activates v3 configuration at
> runtime. This section will document the final v3 TOML field and its default
> when that consumer migration is complete. The setting will remain opt-in:
> enabling it trusts a client-provided announce `ip` and can therefore allow
> peers to spoof addresses.

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
- [ ] `max_connection_id_errors_per_ip` moved from every `[[udp_trackers]]` entry to `[udp_tracker_server]` (if set)
- [ ] `threshold` renamed to `trace_filter` in `[logging]`
- [ ] `trace_style` added to `[logging]` (optional, defaults to `"full"`)
- [ ] `public_url` added to trackers and API (optional, recommended for reverse proxies)
- [ ] `connection_id_validation` reviewed in `[udp_tracker_server]` (optional, defaults to `"strict"`)
- [ ] `ip_bans_reset_interval_in_secs` reviewed in `[udp_tracker_server]` (optional, defaults to `86400`)
- [ ] Network database URLs replaced with component fields; database passwords are non-empty
- [ ] Review the staged optional-database activation guidance before omitting
      `[core.database]` in a deployed v3 tracker

## References

- [EPIC #1978 — Configuration Overhaul](EPIC.md)
- [ADRs](../../../adrs/README.md)
