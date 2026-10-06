---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md
last-updated-utc: "2026-10-06 11:49"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-06 10:19 (defect reproduction) and 11:45-11:48 (fix verification).
- Artifact under test: `target/debug/torrust-tracker` and `target/debug/tracker_client`. The
  reproduction used `develop` before any #2245 change. The verification used branch
  `2245-2243-review-numeric-protocol-wire-conversions` at commit
  `fix(udp-server): [#2245] clamp announce wire fields instead of wrapping negative`.
- Operating system / environment: Linux; repository root as current working directory.
- Setup: the configuration below, saved as `.tmp/m1-2245/tracker-over.toml`. The
  `.tmp/m1-2245/tracker-max.toml` variant is identical except for `interval = 2147483647`.

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
listed = false
private = false
tracker_usage_statistics = false

[core.announce_policy]
interval = 2147483648
interval_min = 120
max_peers_per_announce = 2

[core.database]
driver = "sqlite3"
path = "./.tmp/m1-2245/sqlite3.db"

[core.tracker_policy]
persistent_torrent_completed_stat = false
remove_peerless_torrents = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:16970"
```

## Verification Processes

### V1 - M1: Announce Interval Bounds

- Goal: Confirm that the UDP announce response encodes `announce_interval` as a valid signed
  32-bit value at and beyond `i32::MAX` (A171).
- Initial state: a fresh tracker for each configuration; one announce on info-hash
  `9c38422213e30bff212b30c360d26f9a02136422`.
- Status: `DONE`

#### Steps Performed

1. Started the tracker with one configuration, for example:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/m1-2245/tracker-over.toml ./target/debug/torrust-tracker \
     > .tmp/m1-2245/tracker-over.log 2>&1
   ```

2. Announced once:

   ```sh
   ./target/debug/tracker_client udp announce 127.0.0.1:16970 9c38422213e30bff212b30c360d26f9a02136422 \
     --peer-id=-qB00000000000000001 --port 7001 --event started --left 100
   ```

3. Stopped the tracker, then repeated steps 1-2 for the other configuration and build.

#### Observed Result

Before the fix, on `develop`, with `interval = 2147483648` (the tracker logged
`"interval": 2147483648` at startup):

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":-2147483648,"leechers":1,"seeders":0,"peers":[]}}
```

After the fix, with `interval = 2147483647`:

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":2147483647,"leechers":1,"seeders":0,"peers":[]}}
```

After the fix, with `interval = 2147483648`:

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":2147483647,"leechers":1,"seeders":0,"peers":[]}}
```

| `interval` | Before the fix | After the fix | Expected |
| ---------- | -------------- | ------------- | -------- |
| 2147483647 (`i32::MAX`) | not run | 2147483647 | Unchanged |
| 2147483648 (`i32::MAX + 1`) | -2147483648 | 2147483647 | Clamped to `i32::MAX` |

#### Conclusion

The fix clamps an out-of-range interval to `i32::MAX` instead of wrapping it to a negative value,
and leaves in-range values unchanged. M1 passes. The tracker still accepts the out-of-range
configuration; rejecting it at load is the follow-up recorded in the issue spec.
