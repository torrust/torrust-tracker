---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2246-2243-review-domain-numeric-conversions/ISSUE.md
last-updated-utc: "2026-10-05 17:57"
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-05 17:48-17:50. A first run at 15:00-15:04 gave the same peer
  counts; it was repeated with exactly the commands below so this record quotes verbatim output.
- Artifact under test: `target/debug/torrust-tracker` and `target/debug/tracker_client`, built
  from branch `2246-2243-review-domain-numeric-conversions` at commit
  `docs(issues): [#2246] stamp evidence update time to the minute` (all code changes applied).
- Operating system / environment: Linux; repository root as current working directory.
- Setup: the configuration below, saved as `.tmp/m1-2246/tracker.toml`. It runs a single UDP
  tracker on `127.0.0.1:16969` and sets `max_peers_per_announce = 2` so the cap is observable with
  a small swarm.

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
interval = 120
interval_min = 120
max_peers_per_announce = 2

[core.database]
driver = "sqlite3"
path = "./.tmp/m1-2246/sqlite3.db"

[core.tracker_policy]
persistent_torrent_completed_stat = false
remove_peerless_torrents = false

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "127.0.0.1:16969"
```

## Verification Processes

### V1 - M1: Announce `numwant` Boundary

- Goal: Confirm that `PeersWanted::from_client_request` (A129) still resolves UDP `numwant`
  values end to end: non-positive means "as many as possible", positive values are capped at the
  tracker limit.
- Initial state: tracker running; four leechers (ports 7001-7004) announced on info-hash
  `9c38422213e30bff212b30c360d26f9a02136422`.
- Status: `DONE`

#### Steps Performed

1. Started the tracker:

   ```sh
   TORRUST_TRACKER_CONFIG_TOML_PATH=.tmp/m1-2246/tracker.toml ./target/debug/torrust-tracker \
     > .tmp/m1-2246/tracker-rerun.log 2>&1
   ```

2. Seeded four peers:

   ```sh
   for i in 1 2 3 4; do
     ./target/debug/tracker_client udp announce 127.0.0.1:16969 9c38422213e30bff212b30c360d26f9a02136422 \
       --peer-id=-qB0000000000000000$i --port 700$i --event started --left 100
     echo
   done
   ```

   Output:

   ```text
   {"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":1,"seeders":0,"peers":[]}}
   {"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":2,"seeders":0,"peers":["127.0.0.1:7001"]}}
   {"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":3,"seeders":0,"peers":["127.0.0.1:7001"]}}
   {"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":4,"seeders":0,"peers":["127.0.0.1:7001"]}}
   ```

   Each seeding announce returns at most one peer because the client defaults `peers_wanted` to
   1 when `--peers-wanted` is omitted.

3. Announced from a fifth peer with each `numwant` value and counted the returned peers:

   ```sh
   for w in 0 -1 1 2 3 2147483647; do
     ./target/debug/tracker_client udp announce 127.0.0.1:16969 9c38422213e30bff212b30c360d26f9a02136422 \
       --peer-id=-qB00000000000000009 --port 7009 --left 100 --peers-wanted=$w \
       | jq -c '.AnnounceIpv4 | {peers: (.peers | length), leechers, seeders}'
   done
   ```

#### Observed Result

Step 3 output, one line per `numwant` value in loop order:

```text
{"peers":2,"leechers":5,"seeders":0}
{"peers":2,"leechers":5,"seeders":0}
{"peers":1,"leechers":5,"seeders":0}
{"peers":2,"leechers":5,"seeders":0}
{"peers":2,"leechers":5,"seeders":0}
{"peers":2,"leechers":5,"seeders":0}
```

| `numwant` | Returned peers | Expected |
| --------- | -------------- | -------- |
| 0 | 2 | As many as possible, capped at 2 |
| -1 | 2 | As many as possible, capped at 2 |
| 1 | 1 | Exactly 1 |
| 2 | 2 | Exactly 2 |
| 3 | 2 | Capped at 2 |
| 2147483647 (`i32::MAX`) | 2 | Capped at 2 |

#### Conclusion

Positive values cap at the tracker limit; zero and negative values return as many peers as
possible. M1 passes.
