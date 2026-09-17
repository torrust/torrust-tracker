---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2226-fix-stale-inactivity-cutoff-in-activity-metrics-updater/ISSUE.md
last-updated-utc: 2026-09-17 15:16
---

# Manual Verification Evidence

## Purpose

Record the fixed-build recheck of the original local tracker reproduction in
`evidence.md`.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-17, 15:14-15:16 UTC
- Artifact under test: local `2226-fix-stale-inactivity-cutoff` branch after
  commit `017dddc9`
- Operating system / environment: Linux, local Cargo development build
- Prerequisites and setup performed: isolated SQLite configuration with
  `max_peer_timeout = 20`, `inactive_peer_cleanup_interval = 0`,
  `remove_peerless_torrents = false`, HTTP tracker usage statistics enabled,
  HTTP tracker `127.0.0.1:17071`, and REST API `127.0.0.1:11213`

## Verification Processes

### V1 - Post-Startup Peer Becomes Inactive

- Goal: repeat the pre-fix reproduction against the fixed tracker build.
- Initial state: the isolated tracker had no peers and both inactivity gauges
  reported `0`.
- Status: `DONE`

#### Steps Performed

1. Started the tracker with an isolated configuration and captured its output:

   ```text
   rm -f .tmp/issue-evidence-fixed/activity-metrics-fixed-cutoff.sqlite3 .tmp/issue-evidence-fixed/tracker.log && RUST_LOG=debug cargo run --bin torrust-tracker -- --config-toml-path .tmp/issue-evidence-fixed/activity-metrics-fixed-cutoff.toml > .tmp/issue-evidence-fixed/tracker.log 2>&1
   ```

2. Queried the initial inactive gauges:

   ```text
   curl --fail --silent --show-error -H 'Authorization: Bearer issue-evidence-token' 'http://127.0.0.1:11213/api/v1/metrics?format=prometheus' | grep -E 'swarm_coordination_registry_(peers|torrents)_inactive_total'
   ```

3. Announced one HTTP peer after tracker startup:

   ```text
   cargo run -p torrust-tracker-client --bin tracker_client -- http announce http://127.0.0.1:17071 0123456789abcdef0123456789abcdef01234567 --event started --uploaded 0 --downloaded 0 --left 1000 --port 16881 --peer-id 01234567890123456789 --compact 1
   ```

4. Ran the issue-scoped regression suites while the peer crossed the
   20-second inactivity timeout:

   ```text
   cargo test -p torrust-tracker-swarm-coordination-registry --lib statistics::activity_metrics_updater
   cargo test -p torrust-tracker-swarm-coordination-registry --lib swarm::coordinator
   cargo test -p torrust-tracker-core --lib torrent::manager
   ```

5. Queried the gauges and captured recent updater classifications after the
   timeout and a subsequent 15-second update tick:

   ```text
   date --utc '+%Y-%m-%d %H:%M:%S UTC'
   curl --fail --silent --show-error -H 'Authorization: Bearer issue-evidence-token' 'http://127.0.0.1:11213/api/v1/metrics?format=prometheus' | grep -E 'swarm_coordination_registry_(peers|torrents)_inactive_total'
   grep -E 'Updating peers and torrents activity metrics|active_peers_total|inactive_peers_total' .tmp/issue-evidence-fixed/tracker.log | tail -n 10
   ```

#### Observed Result

```text
Initial gauges:
swarm_coordination_registry_peers_inactive_total 0
swarm_coordination_registry_torrents_inactive_total 0

Announce response:
{"complete":0,"incomplete":1,"interval":120,"min interval":120,"peers":[],"peers6":[]}

Focused suites:
activity_metrics_updater: 4 passed; 0 failed
swarm::coordinator: 41 passed; 0 failed
torrent::manager: 4 passed; 0 failed

2026-09-17 15:16:09 UTC
swarm_coordination_registry_peers_inactive_total 1
swarm_coordination_registry_torrents_inactive_total 1
2026-09-17T15:15:10.194868Z ... Sending TcpAnnounce event ... updated: 1789658110.194543169s
2026-09-17T15:15:25.150427Z ... active_peers_total=1 inactive_peers_total=0 active_torrents_total=1 inactive_torrents_total=0
2026-09-17T15:15:40.151052Z ... active_peers_total=0 inactive_peers_total=1 active_torrents_total=0 inactive_torrents_total=1
2026-09-17T15:15:55.151001Z ... active_peers_total=0 inactive_peers_total=1 active_torrents_total=0 inactive_torrents_total=1
```

#### Conclusion

The result meets M1 and AC5. The peer announced after startup was active at the
first following update, then was counted as inactive at the next update after
more than 20 seconds had elapsed. Both inactive gauges changed from `0` to `1`.

## Failures and Follow-up

- The first readiness probe ran while Cargo was still compiling the tracker and
  could not connect to the REST API. The build completed successfully, the
  repeated probe returned the expected zero baseline, and V1 was then executed.
- The ignored `.tmp/issue-evidence-fixed/` configuration, database, and tracker
  log support local inspection but are not part of this tracked evidence.

<!-- End of manual verification evidence. -->