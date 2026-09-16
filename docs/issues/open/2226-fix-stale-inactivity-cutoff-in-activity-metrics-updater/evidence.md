---
doc-type: reproduction-evidence
issue-spec: docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md
last-updated-utc: 2026-09-15 11:05
---

# Stale Activity-Metrics Cutoff Reproduction Evidence

## Strategy

The defect is observable only for a peer that announces after the updater has
captured its startup-time cutoff. The reproduction therefore uses an isolated
tracker with a 20-second `max_peer_timeout`, disables peer cleanup so the peer
remains available for metrics classification, and keeps the 15-second activity
metrics updater enabled. The reproduction announces one HTTP peer after startup,
waits well beyond the timeout and multiple updater ticks, then reads the
Prometheus metrics endpoint and relevant tracker logs.

If the cutoff were recomputed at each update, the peer announced at 10:06:21
would be inactive by 10:06:41 and the next update tick would report one inactive
peer. The observed output instead reports zero inactive peers through 10:11:08.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-15, 10:04-10:11 UTC
- Artifact under test: current workspace revision, before an inactivity-cutoff fix
- Operating system / environment: Linux, local Cargo development build
- Tracker configuration: `.tmp/issue-evidence/activity-metrics-stale-cutoff.toml`
- Isolated tracker ports: HTTP tracker `127.0.0.1:17070`; REST API `127.0.0.1:11212`
- Relevant configuration: `max_peer_timeout = 20`, `inactive_peer_cleanup_interval = 0`, `remove_peerless_torrents = false`, and HTTP tracker usage statistics enabled

## Verification Processes

### V1 - Post-startup Peer Is Never Marked Inactive

- Goal: reproduce the inactive-gauge defect against a running local tracker.
- Initial state: a fresh isolated SQLite database, no peers, and activity gauges at zero.
- Status: `DONE`

#### Steps Performed

1. Created and displayed the isolated configuration:

   ```text
   mkdir -p .tmp/issue-evidence && cat > .tmp/issue-evidence/activity-metrics-stale-cutoff.toml <<'EOF'
   [metadata]
   app = "torrust-tracker"
   purpose = "configuration"
   schema_version = "3.0.0"

   [logging]
   trace_filter = "debug"
   trace_style = "full"

   [core]
   inactive_peer_cleanup_interval = 0
   listed = false
   private = false

   [core.database]
   driver = "sqlite3"
   path = ".tmp/issue-evidence/activity-metrics-stale-cutoff.sqlite3"

   [core.tracker_policy]
   max_peer_timeout = 20
   persistent_torrent_completed_stat = true
   remove_peerless_torrents = false

   [[http_trackers]]
   bind_address = "127.0.0.1:17070"
   tracker_usage_statistics = true

   [http_api]
   bind_address = "127.0.0.1:11212"

   [http_api.access_tokens]
   admin = "issue-evidence-token"
   EOF
   cat .tmp/issue-evidence/activity-metrics-stale-cutoff.toml
   ```

   Output:

   ```toml
   [metadata]
   app = "torrust-tracker"
   purpose = "configuration"
   schema_version = "3.0.0"

   [logging]
   trace_filter = "debug"
   trace_style = "full"

   [core]
   inactive_peer_cleanup_interval = 0
   listed = false
   private = false

   [core.database]
   driver = "sqlite3"
   path = ".tmp/issue-evidence/activity-metrics-stale-cutoff.sqlite3"

   [core.tracker_policy]
   max_peer_timeout = 20
   persistent_torrent_completed_stat = true
   remove_peerless_torrents = false

   [[http_trackers]]
   bind_address = "127.0.0.1:17070"
   tracker_usage_statistics = true

   [http_api]
   bind_address = "127.0.0.1:11212"

   [http_api.access_tokens]
   admin = "issue-evidence-token"
   ```

2. Started the tracker, capturing its output in `.tmp/issue-evidence/tracker.log`:

   ```text
   rm -f .tmp/issue-evidence/activity-metrics-stale-cutoff.sqlite3 && RUST_LOG=debug cargo run --bin torrust-tracker -- --config-toml-path .tmp/issue-evidence/activity-metrics-stale-cutoff.toml > .tmp/issue-evidence/tracker.log 2>&1
   ```

   Relevant startup output:

   ```text
   2026-09-15T10:04:50.197334Z  INFO ... HTTP TRACKER: Started on: http://127.0.0.1:17070
   2026-09-15T10:04:50.197548Z  INFO ... API: Started tracker API service_binding=http://127.0.0.1:11212/
   ```

3. Queried the inactive gauges before any announce:

   ```text
   curl --fail --silent --show-error -H 'Authorization: Bearer issue-evidence-token' 'http://127.0.0.1:11212/api/v1/metrics?format=prometheus' | grep -E 'swarm_coordination_registry_(peers|torrents)_inactive_total'
   ```

   Output:

   ```text
   # HELP swarm_coordination_registry_peers_inactive_total The total number of inactive peers.
   # TYPE swarm_coordination_registry_peers_inactive_total gauge
   swarm_coordination_registry_peers_inactive_total 0
   # HELP swarm_coordination_registry_torrents_inactive_total The total number of inactive torrents.
   # TYPE swarm_coordination_registry_torrents_inactive_total gauge
   swarm_coordination_registry_torrents_inactive_total 0
   ```

4. Attempted one announce with a 40-byte peer identifier. The client rejected it, so it did not modify tracker state:

   ```text
   cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:17070 0123456789abcdef0123456789abcdef01234567 --event started --uploaded 0 --downloaded 0 --left 1000 --port 16881 --peer-id 0123456789abcdef0123456789abcdef01234567 --compact 1
   ```

   Output:

   ```text
   error: invalid value '0123456789abcdef0123456789abcdef01234567' for '--peer-id <PEER_ID>': peer-id must be exactly 20 bytes, got 40 bytes for `0123456789abcdef0123456789abcdef01234567`
   ```

5. Announced a single peer after tracker startup using a valid 20-byte peer identifier:

   ```text
   cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:17070 0123456789abcdef0123456789abcdef01234567 --event started --uploaded 0 --downloaded 0 --left 1000 --port 16881 --peer-id 01234567890123456789 --compact 1
   ```

   Output:

   ```text
   Finished `dev` profile [optimized + debuginfo] target(s) in 0.26s
   Running `target/debug/tracker_client http announce 'http://127.0.0.1:17070' 0123456789abcdef0123456789abcdef01234567 --event started --uploaded 0 --downloaded 0 --left 1000 --port 16881 --peer-id 01234567890123456789 --compact 1`
   {"complete":0,"incomplete":1,"interval":120,"min interval":120,"peers":[],"peers6":[]}
   ```

   Tracker log evidence records the peer update time as `2026-09-15T10:06:21.041748914Z`:

   ```text
   2026-09-15T10:06:21.041524Z  INFO request{method=GET uri=/announce?...}: HTTP TRACKER: request server_socket_addr=127.0.0.1:17070 ...
   2026-09-15T10:06:21.042379Z DEBUG request{method=GET uri=/announce?...}: torrust_tracker_http_core::services::announce: Sending TcpAnnounce event: ... updated: 1789466781.041748914s, ...
   2026-09-15T10:06:21.042513Z  INFO request{method=GET uri=/announce?...}: HTTP TRACKER: response ... status_code=200 OK ...
   ```

6. After the 20-second timeout had elapsed and after multiple 15-second updater ticks, queried the gauges and captured the updater logs:

   ```text
   date --utc '+%Y-%m-%d %H:%M:%S UTC'; curl --fail --silent --show-error -H 'Authorization: Bearer issue-evidence-token' 'http://127.0.0.1:11212/api/v1/metrics?format=prometheus' | grep -E 'swarm_coordination_registry_(peers|torrents)_inactive_total'; tail -n 80 .tmp/issue-evidence/tracker.log | grep -E 'Updating peers and torrents activity metrics|Peers and torrents activity metrics updated|active_peers_total|inactive_peers_total'
   ```

   Output (abridged only to omit repeated, identical ticks between 10:06:05 and 10:11:05; the two ticks before the announce reported `active_peers_total=0 inactive_peers_total=0`):

   ```text
   2026-09-15 10:11:08 UTC
   # HELP swarm_coordination_registry_torrents_inactive_total The total number of inactive torrents.
   # TYPE swarm_coordination_registry_torrents_inactive_total gauge
   swarm_coordination_registry_torrents_inactive_total 0
   # HELP swarm_coordination_registry_peers_inactive_total The total number of inactive peers.
   # TYPE swarm_coordination_registry_peers_inactive_total gauge
   swarm_coordination_registry_peers_inactive_total 0
   2026-09-15T10:06:35.198661Z DEBUG ... Updating peers and torrents activity metrics (executed every 15 secs) ...
   2026-09-15T10:06:35.201616Z  INFO ... active_peers_total=1 inactive_peers_total=0 active_torrents_total=1 inactive_torrents_total=0
   2026-09-15T10:06:50.198878Z DEBUG ... Updating peers and torrents activity metrics (executed every 15 secs) ...
   2026-09-15T10:06:50.199687Z  INFO ... active_peers_total=1 inactive_peers_total=0 active_torrents_total=1 inactive_torrents_total=0
   2026-09-15T10:07:20.198618Z DEBUG ... Updating peers and torrents activity metrics (executed every 15 secs) ...
   2026-09-15T10:07:20.198664Z  INFO ... active_peers_total=1 inactive_peers_total=0 active_torrents_total=1 inactive_torrents_total=0
   2026-09-15T10:08:35.198801Z DEBUG ... Updating peers and torrents activity metrics (executed every 15 secs) ...
   2026-09-15T10:08:35.198850Z  INFO ... active_peers_total=1 inactive_peers_total=0 active_torrents_total=1 inactive_torrents_total=0
   2026-09-15T10:09:05.198691Z DEBUG ... Updating peers and torrents activity metrics (executed every 15 secs) ...
   2026-09-15T10:09:05.198736Z  INFO ... active_peers_total=1 inactive_peers_total=0 active_torrents_total=1 inactive_torrents_total=0
   ```

7. Captured the relevant source wiring after the runtime reproduction:

   ```text
   grep -nE 'peer_inactivity_cutoff_timestamp|inactivity_cutoff' src/bootstrap/jobs/activity_metrics_updater.rs packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs packages/swarm-coordination-registry/src/swarm/coordinator.rs
   ```

   Output:

   ```text
   src/bootstrap/jobs/activity_metrics_updater.rs:17:        peer_inactivity_cutoff_timestamp(config.core.tracker_policy.max_peer_timeout),
   src/bootstrap/jobs/activity_metrics_updater.rs:25:fn peer_inactivity_cutoff_timestamp(max_peer_timeout: u32) -> Duration {
   packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:21:    inactivity_cutoff: DurationSinceUnixEpoch,
   packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:41:                        update_activity_metrics(interval_in_secs, &swarms, &stats_repository, inactivity_cutoff).await;
   packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:56:    inactivity_cutoff: DurationSinceUnixEpoch,
   packages/swarm-coordination-registry/src/statistics/activity_metrics_updater.rs:65:    let activity_metadata = swarms.get_activity_metadata(inactivity_cutoff).await;
   ```

8. Stopped the tracker with Ctrl+C after collecting evidence:

   ```text
   ^C
   ```

#### Conclusion

The reproduction confirms the bug. The only peer announced at 10:06:21 remained
reported as active through the 10:11:08 metrics query, about 287 seconds after
the announce and about 267 seconds after its 20-second inactivity timeout
expired at 10:06:41. The tracker ran activity-metrics updates
at least every 15 seconds during that period, yet every recorded update reported
`active_peers_total=1` and `inactive_peers_total=0`. The source wiring confirms
the updater receives one cutoff at job creation and passes that unchanged value
to every update.

## Failures and Follow-up

- The first announce command used a 40-byte peer identifier and was rejected by the client before it sent a request. The next command used a valid 20-byte identifier and produced the peer used for the reproduction.
- The captured tracker log remains in `.tmp/issue-evidence/tracker.log` for local inspection; `.tmp/` is ignored and is not part of this issue evidence.
