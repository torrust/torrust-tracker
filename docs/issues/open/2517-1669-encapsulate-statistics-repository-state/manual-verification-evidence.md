---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
last-updated-utc: 2026-10-14 13:06
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-14, 12:51-13:05
- Artifact under test: `develop` at `c1874facbc5790e8881727f63a17296e24242731`, and the implementation changes titled:
  - `refactor(udp-core): [#2517] make the statistics repository lock private`
  - `refactor(udp-server): [#2517] make the statistics repository lock private`
  - `refactor(http-core): [#2517] make the statistics repository lock private`
  - `refactor(tracker-core): [#2517] make the statistics repository lock private`
  - `refactor(swarm-coordination-registry): [#2517] make the statistics repository lock private`
- Operating system / environment: Linux, local development workspace
- Prerequisites and setup performed: Started the tracker separately on `develop` and the implementation branch. Each run used a fresh SQLite database and the same development configuration with isolated ports. The implementation run used an otherwise identical config with a separate database path.

The isolated config used for the baseline (the implementation config differs only in the database path):

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = ".tmp/manual-2517-baseline/database.sqlite3"

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[core.tracker_policy]
max_peer_timeout = 60
persistent_torrent_completed_stat = true
remove_peerless_torrents = true

[[udp_trackers]]
bind_address = "0.0.0.0:17968"
tracker_usage_statistics = true

[[udp_trackers]]
bind_address = "0.0.0.0:17969"
tracker_usage_statistics = true

[[http_trackers]]
bind_address = "0.0.0.0:18070"
tracker_usage_statistics = true

[[http_trackers]]
bind_address = "0.0.0.0:18171"
tracker_usage_statistics = true

[http_api]
bind_address = "0.0.0.0:18212"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "127.0.0.1:18313"
```

The `develop` and implementation databases were separate and initially empty. The health-check endpoint returned `{"status":"Ok","message":""}` before each announce pair.

## Verification Processes

### V1 - Metrics still update

- Goal: Compare UDP and HTTP announce results and REST statistics/metrics before and after the visibility change.
- Initial state: Fresh database and zero torrents, seeders, leechers, UDP announces, and HTTP announces.
- Status: `DONE`

#### Steps Performed

1. Started each build with `cargo run --bin torrust-tracker -- --config-toml-path ./.tmp/manual-2517-{baseline,implementation}/tracker.toml`.
2. Read the initial stats using `curl --fail --silent --show-error -H 'Authorization: Bearer MyAccessToken' http://127.0.0.1:18212/api/v1/stats`.
3. Sent the same HTTP and UDP started announces to each build:

   ```sh
   cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:18070 9c38422213e30bff212b30c360d26f9a02136422 --event started --left 1000 --port 6881 '--peer-id=-RC00000000000000001'
   cargo run -p torrust-tracker-client --bin tracker_client -- udp announce 127.0.0.1:17969 9c38422213e30bff212b30c360d26f9a02136422 --event started --left 1000 --port 6882 '--peer-id=-RC00000000000000002'
   ```

4. Read `/api/v1/stats` and `/api/v1/metrics` again with the same authorization header.
5. Compared the selected post-announce API fields and announce metric samples using `diff -u`; the baseline and implementation summaries matched.

#### Observed Result

The HTTP client response on both builds was:

```json
{"complete":0,"incomplete":1,"interval":120,"min interval":120,"peers":[]}
```

The UDP client response on both builds was:

```json
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":2,"seeders":0,"peers":["127.0.0.1:6881"]}}
```

REST stats before and after each announce pair:

| Field | `develop` before | `develop` after | Implementation before | Implementation after |
| ----- | ---------------: | --------------: | ---------------------: | --------------------: |
| `torrents` | 0 | 1 | 0 | 1 |
| `seeders` | 0 | 0 | 0 | 0 |
| `leechers` | 0 | 2 | 0 | 2 |
| `udp4_announces_handled` | 0 | 1 | 0 | 1 |
| `tcp4_announces_handled` | 0 | 1 | 0 | 1 |

The `/api/v1/metrics` announce samples after each pair were:

```text
http_tracker_core_requests_received_total request_kind=announce port=18070 value=1
udp_tracker_core_requests_received_total request_kind=announce port=17969 value=1
```

#### Conclusion

M1 passed. The same UDP and HTTP announces produced identical tracker-client responses, REST stats, and protocol announce metric samples on `develop` and after the field became private.

## Failures and Follow-up

- The first start with default development ports could not bind the health-check API because `127.0.0.1:1313` was already occupied by a pre-existing tracker process. That process was left untouched; both M1 runs used the isolated configuration above.
- An initial REST request omitted the `Bearer` prefix and was rejected as an unknown token; the request was corrected and the authenticated reads succeeded.
- An initial client command supplied a 40-character hex string as a peer ID and was rejected client-side because the CLI expects a 20-byte value. The announce was rerun with valid 20-byte peer IDs; the successful response outputs above are from those completed requests.
