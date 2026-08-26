---
doc-type: guide
parent-epic: 1978
spec-path: docs/issues/open/1978-configuration-overhaul-epic/configuration-v2-to-v3-migration.md
last-updated-utc: 2026-08-26
---

# Migrating from Configuration v2.0.0 to v3.0.0

Torrust Tracker now activates configuration schema `3.0.0` at runtime. A running
tracker accepts v3 configuration only: a file declaring `schema_version = "2.0.0"`
is rejected. V3 also rejects unknown fields, so remove obsolete v2 keys rather
than leaving them in place.

All shipped configuration templates now declare schema v3. Use the template
matching the intended deployment and migrate any separately maintained v2
configuration before loading it with the active runtime.

## Quick reference

| v2 field / section           | v3 equivalent                                                    | Subissue | Status |
| ---------------------------- | ---------------------------------------------------------------- | -------- | ------ |
| `[core.net]` (global)        | Per-tracker `[http_trackers.network]` / `[udp_trackers.network]` | #1640    | Active |
| `tsl_config`                 | `tls_config`                                                     | #1981    | DONE   |
| No public URL field          | `public_url` on HTTP trackers, UDP trackers, and HTTP API        | #1417    | DONE   |
| `on_reverse_proxy` (global)  | Per-HTTP-tracker `network.on_reverse_proxy`                      | #1640    | DONE   |
| No logging style option      | `[logging] trace_style`                                          | #889     | DONE   |
| `threshold`                  | `trace_filter`                                                   | #889     | DONE   |
| No connection ID policy      | `[udp_tracker_server] connection_id_validation`                  | #1136    | DONE   |
| Hardcoded IP bans interval   | `[udp_tracker_server] ip_bans_reset_interval_in_secs`            | #1453    | Active |
| Per-listener UDP error limit | `[udp_tracker_server] max_connection_id_errors_per_ip`           | #2083    | Active |
| Flat `[core.database]`       | Database enum with per-driver config                             | #1490    | DONE   |
| No announce `ip` opt-in      | Per-HTTP-tracker `use_ip_from_query_string`                      | #1987    | Active |

## Practical migration sequence

1. Copy the deployed v2 file and change `metadata.schema_version` to `"3.0.0"`.
2. Rename `logging.threshold` to `logging.trace_filter` and rename every
   `tsl_config` table to `tls_config`.
3. Remove `[core.net]`; add per-listener `network` tables where the old global
   settings or listener `ipv6_v6only` values apply.
4. Move UDP listener error limits into one `[udp_tracker_server]` table.
5. Convert `[core.database]` for its selected driver. Do not copy a network
   database URL into v3.
6. Review each HTTP listener's `use_ip_from_query_string`; leave it disabled
   unless trusting a client-provided peer address is intentional.
7. Add optional public URLs for externally reachable services, validate the
   converted configuration, and deploy it with an explicit SQLite section.

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

The tracker runs the v3 schema at runtime and rejects configs with a schema
version other than `3.0.0`. V2 is not a fallback schema. V3 also rejects
unknown fields, so remove renamed and moved v2 keys instead of retaining them.

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

V3 rejects the misspelled `tsl_config` key. The corrected table remains nested
under the HTTP tracker or API that it configures; it is not a top-level table.
For example, use `[http_api.tls_config]` for API TLS.

## Step 3: Replace the global network block

**Subissue**: #1640 — Per-HTTP-tracker `on_reverse_proxy` setting

The global `[core.net]` section (including `on_reverse_proxy` and
`external_ip`) is **removed** in v3. These settings, and listener
`ipv6_v6only`, move to per-tracker `network` blocks.

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
ipv6_v6only = false

[[udp_trackers]]
bind_address = "0.0.0.0:6969"

[udp_trackers.network]
external_ip = "1.2.3.4"
ipv6_v6only = false
```

`on_reverse_proxy` is an HTTP address-resolution policy: enable it only for an
HTTP listener behind a trusted proxy, because the listener then trusts the
proxy-provided `X-Forwarded-For` address. `external_ip` is per listener and is
used when a loopback peer needs the tracker's reachable address; wildcard
addresses (`0.0.0.0` and `::`) are invalid. `ipv6_v6only = true` requires a
separate IPv4 listener if IPv4 traffic must be accepted. If the v2 defaults
were suitable, you can omit the `network` block entirely.

## Step 4: Add public URL fields (optional)

**Subissue**: #1417 — Include public service URL in configuration

You can declare each service's externally reachable URL. This is optional and
does not change its bind address, TLS configuration, reverse-proxy policy, or
routing. Use the public scheme, host, port, and path rather than an internal
bind address.

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
deserialization. HTTP trackers and the HTTP API require `http` or `https`;
UDP trackers require `udp`. Configuring a public URL does not expose a new
listener. Runtime observability of configured public URLs is delivered
separately.

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
> `trace_filter`, its value defaults to `info` in the schema, but the v3 loader
> requires an explicit value in a deployed configuration.

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

The setting is active at runtime. It must be at least `3600` seconds.

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
accepting repeated values. All UDP listeners share this limit and one ban
service, so listener declaration order cannot change the effective policy.

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

V3 permits an omitted `[core.database]` table. This is deliberately staged:
Issue #1980 activates v3 consumers while retaining a named fixed-SQLite compatibility
bridge so the tracker remains persistence-enabled. A follow-up will activate
the configured optional value and define the effective omitted-database runtime
behaviour.

**Do not treat an omitted `[core.database]` as persistence-free startup.** It
does not currently disable persistence, select no database, or make container
startup independent of persistent storage. Keep an explicit SQLite database
configuration for deployed v3 trackers until the follow-up is implemented and
documented.

This is a breaking configuration change: MySQL and PostgreSQL URLs are not
accepted in v3. Move their URL components into the fields above. Do not use an
empty password: loading rejects missing and empty network database passwords.

## Step 10: Configure HTTP announce IP trust policy

**Subissue**: #1987 — Use peer IP from the HTTP announce `ip` parameter

V3 adds `use_ip_from_query_string` to each `[[http_trackers]]` entry. It
defaults to `false`. With the default, absent or empty `ip` parameters use the
normal address-resolution path and a non-empty `ip` value is rejected. When
enabled, a non-empty `ip` must be an IPv4 or IPv6 literal and becomes the peer
address; DNS names and invalid values are always rejected.

```toml
[[http_trackers]]
bind_address = "127.0.0.1:7070"
use_ip_from_query_string = true
```

Enabling this setting trusts a client-supplied address and allows a remote
client to register an arbitrary IP in the peer list. Leave it disabled for
public or untrusted deployments; use it only in a controlled deployment that
requires this BEP 3 compatibility behaviour.

For an accepted non-empty query IP, the precedence is:

1. The query `ip` literal when the setting is enabled.
2. The listener `network.external_ip` for a loopback connection.
3. The rightmost `X-Forwarded-For` address when
   `network.on_reverse_proxy = true`.
4. The direct connection address.

An absent or empty `ip` preserves steps 2–4.

## Complete representative v3 configuration

This configuration shows a direct TLS HTTP tracker, one UDP listener, an HTTP
API, per-listener topology, shared UDP policies, and explicit SQLite
persistence. Replace paths, names, tokens, and addresses before production use.

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "json"

[core]
inactive_peer_cleanup_interval = 600
listed = false
private = false
tracker_usage_statistics = true

[core.announce_policy]
interval = 120
interval_min = 120
max_peers_per_announce = 74

[core.tracker_policy]
max_peer_timeout = 900
persistent_torrent_completed_stat = false
remove_peerless_torrents = true

# Keep this explicit while the fixed-SQLite compatibility bridge is active.
[core.database]
driver = "sqlite3"
path = "/var/lib/torrust/tracker/database/sqlite3.db"

[udp_tracker_server]
connection_id_validation = "strict"
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10

[[udp_trackers]]
bind_address = "0.0.0.0:6969"
tracker_usage_statistics = true
public_url = "udp://tracker.example.com:6969"

[udp_trackers.network]
external_ip = "203.0.113.10"
ipv6_v6only = false

[[http_trackers]]
bind_address = "0.0.0.0:7070"
tracker_usage_statistics = true
use_ip_from_query_string = false
public_url = "https://tracker.example.com/announce"

[http_trackers.network]
external_ip = "203.0.113.10"
on_reverse_proxy = false
ipv6_v6only = false

[http_trackers.tls_config]
ssl_cert_path = "/etc/torrust/tracker/tls/tracker.crt"
ssl_key_path = "/etc/torrust/tracker/tls/tracker.key"

[http_api]
bind_address = "127.0.0.1:1212"
public_url = "https://api.tracker.example.com"

[http_api.access_tokens]
admin = "replace-with-a-secret"

[health_check_api]
bind_address = "127.0.0.1:1313"
```

## Migration checklist

Use this checklist to verify your configuration is ready for v3:

- [ ] `schema_version` set to `"3.0.0"`
- [ ] `tsl_config` renamed to `tls_config` (if applicable)
- [ ] Global `[core.net]` replaced with per-tracker `network` blocks
- [ ] `on_reverse_proxy` moved to per-HTTP-tracker `network` block (if `true`)
- [ ] `external_ip` moved to per-tracker `network` blocks (if set)
- [ ] Listener `ipv6_v6only` moved to `network.ipv6_v6only` (if set)
- [ ] `max_connection_id_errors_per_ip` moved from every `[[udp_trackers]]` entry to `[udp_tracker_server]` (if set)
- [ ] `threshold` renamed to `trace_filter` in `[logging]`
- [ ] `trace_style` added to `[logging]` (optional, defaults to `"full"`)
- [ ] `public_url` added to trackers and API (optional, recommended for reverse proxies)
- [ ] `connection_id_validation` reviewed in `[udp_tracker_server]` (optional, defaults to `"strict"`)
- [ ] `ip_bans_reset_interval_in_secs` reviewed in `[udp_tracker_server]` (optional, defaults to `86400`)
- [ ] `use_ip_from_query_string` left disabled unless client-supplied peer IPs are trusted
- [ ] Network database URLs replaced with component fields; database passwords are non-empty
- [ ] Explicit SQLite configuration retained during the fixed-SQLite bridge period

## References

- [EPIC #1978 — Configuration Overhaul](EPIC.md)
- [Issue #1980 — Runtime activation and final cleanup](../1980-1978-configuration-overhaul-final-cleanup.md)
- [Issue #1987 — HTTP announce query-IP policy](../1987-add-config-option-to-use-ip-from-announce-query-string/ISSUE.md)
- [ADRs](../../../adrs/README.md)
