---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2466-1978-announce-interval-upper-bound/ISSUE.md
last-updated-utc: "2026-10-07 10:18"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-07 10:18 (pre-review defect reproduction).
- Artifact under test: `target/debug/torrust-tracker` and `target/debug/tracker_client`, built
  with the stable Rust toolchain `rustc 1.99.0 (b940084d7 2026-09-28)` from `develop` at
  `7836471b3` plus this specification PR's documentation-only commits (no code change).
- Operating system / environment: Linux; repository root as current working directory.
- Setup: the configuration below, saved as `.tmp/repro-2466/tracker-over.toml`.

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
path = "./.tmp/repro-2466/sqlite3.db"

[core.tracker_policy]
persistent_torrent_completed_stat = false
remove_peerless_torrents = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:16971"

[[http_trackers]]
bind_address = "127.0.0.1:17071"
```

## Verification Processes

### V0 - Pre-Review Reproduction (M1 "Before")

- Goal: Observe that the tracker accepts `interval = 2147483648` and that UDP and HTTP announces
  report different intervals.
- Initial state: a fresh tracker with an empty SQLite database; one announce per protocol on
  info-hash `9c38422213e30bff212b30c360d26f9a02136422`.
- Status: `DONE`
- Outcome: **Reproduced**.

#### Steps Performed

1. Started the tracker:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/repro-2466/tracker-over.toml ./target/debug/torrust-tracker \
     > .tmp/repro-2466/tracker-over.log 2>&1 &
   ```

2. Announced over UDP:

   ```sh
   ./target/debug/tracker_client udp announce 127.0.0.1:16971 9c38422213e30bff212b30c360d26f9a02136422 \
     --peer-id=-qB00000000000000001 --port 7001 --event started --left 100
   ```

3. Announced over HTTP:

   ```sh
   ./target/debug/tracker_client http announce http://127.0.0.1:17071 9c38422213e30bff212b30c360d26f9a02136422
   ```

4. Stopped the tracker.

#### Observed Result

The tracker started and logged the configured value:

```text
"interval": 2147483648
```

UDP announce (clamped by #2245):

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":2147483647,"leechers":1,"seeders":0,"peers":[]}}
```

HTTP announce (configured value):

```text
{"complete":1,"incomplete":1,"interval":2147483648,"min interval":120,"peers":[{"ip":"127.0.0.1","peer id":[45,113,66,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,48,49],"port":7001}]}
```

#### Conclusion

Reproduced: the configuration is accepted, UDP clients receive `2147483647`, and HTTP clients
receive `2147483648` for the same tracker. M1's "after" half and M2-M3 run after the fix.

## Failures and Follow-up

None.
