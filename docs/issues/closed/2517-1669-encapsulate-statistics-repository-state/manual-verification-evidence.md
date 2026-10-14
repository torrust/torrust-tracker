---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2517-1669-encapsulate-statistics-repository-state/ISSUE.md
last-updated-utc: 2026-10-14 15:59
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-14, 14:12 (baseline) and 14:12 (implementation)
- Artifact under test:
  - Baseline: `develop` at `c1874facbc5790e8881727f63a17296e24242731`.
  - Implementation: the branch patches titled
    - `refactor(udp-core): [#2517] make the statistics repository lock private`
    - `refactor(udp-server): [#2517] make the statistics repository lock private`
    - `refactor(http-core): [#2517] make the statistics repository lock private`
    - `refactor(tracker-core): [#2517] make the statistics repository lock private`
    - `refactor(swarm-coordination-registry): [#2517] make the statistics repository lock private`
    - `docs(statistics): [#2517] introduce the repository privacy doctests`
- Operating system / environment: Linux, local development workspace, `dev` profile binaries
  built with `cargo build --bin torrust-tracker` and
  `cargo build -p torrust-tracker-client --bin tracker_client` on each checkout.
- Prerequisites and setup performed: each run used a fresh SQLite database and the default
  development configuration with isolated ports, because another local tracker process already
  held `127.0.0.1:1313`. The two runs differed only in their database path.

Configuration (the implementation run used `.tmp/m1-2517-implementation/database.sqlite3`):

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
path = ".tmp/m1-2517-baseline/database.sqlite3"

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

## Verification Processes

### V1 - Metrics still update

- Goal: show that the same UDP and HTTP announces produce the same REST statistics and the same
  samples for every metric family (HTTP core, UDP core, UDP server, tracker core, and swarm
  coordination registry) before and after the change.
- Initial state: fresh database; every metric family empty, and the three
  `tracker_core_*_torrents_downloads_total` counters at `0`
  ([`m1-metrics-before.txt`](evidence/m1-metrics-before.txt)).
- Status: `DONE`

#### Steps Performed

The same command sequence ran on both checkouts, so the two runs could differ only by the code
under test. It was wrapped in a session-local shell script for exact repetition; its steps were:

1. Start `./target/debug/torrust-tracker --config-toml-path <run-dir>/tracker.toml`, logging to
   `<run-dir>/tracker.log`, and poll `http://127.0.0.1:18212/api/health_check` until it answers.
   The aggregate `/health_check` endpoint was not used, because it sends UDP requests that would
   change the UDP metrics.
2. Read `/api/v1/stats` and `/api/v1/metrics` with
   `curl -fsS -H 'Authorization: Bearer MyAccessToken'`.
3. Send four announces for info-hash `9c38422213e30bff212b30c360d26f9a02136422`:

   ```sh
   ./target/debug/tracker_client http announce http://127.0.0.1:18070 9c38422213e30bff212b30c360d26f9a02136422 --event started --left 1000 --port 6881 '--peer-id=-RC00000000000000001'
   ./target/debug/tracker_client udp announce 127.0.0.1:17969 9c38422213e30bff212b30c360d26f9a02136422 --event started --left 1000 --port 6882 '--peer-id=-RC00000000000000002'
   ./target/debug/tracker_client http announce http://127.0.0.1:18070 9c38422213e30bff212b30c360d26f9a02136422 --event completed --left 0 --port 6881 '--peer-id=-RC00000000000000001'
   ./target/debug/tracker_client udp announce 127.0.0.1:17969 9c38422213e30bff212b30c360d26f9a02136422 --event completed --left 0 --port 6882 '--peer-id=-RC00000000000000002'
   ```

   The `completed` announces go beyond the spec's single announce per protocol, so that the
   tracker-core download counters and swarm completion metrics also change.
4. Wait one second for the asynchronous statistics event listeners, then read `/api/v1/stats` and
   `/api/v1/metrics` again and stop the tracker.
5. Normalize each metrics response to one sorted line per sample:
   `name {sorted labels} value`, or `name <no samples>`. Ignore `recorded_at`, and replace the
   values of time-dependent `*processing_time*` metrics and stats fields with `<timing>`.
6. Compare the two runs with `diff -u` for the client responses, the normalized metrics before
   and after, and the normalized stats before and after.

#### Observed Result

The comparison output:

```text
== diff -u m1-2517-baseline/client.txt m1-2517-implementation/client.txt
(no differences)
== diff -u m1-2517-baseline/metrics-before.txt m1-2517-implementation/metrics-before.txt
(no differences)
== diff -u m1-2517-baseline/metrics-after.txt m1-2517-implementation/metrics-after.txt
(no differences)
== diff -u m1-2517-baseline/stats-before.normalized.json m1-2517-implementation/stats-before.normalized.json
(no differences)
== diff -u m1-2517-baseline/stats-after.normalized.json m1-2517-implementation/stats-after.normalized.json
(no differences)
```

Because the outputs are identical, one copy of each is stored, except the stats-before reading:
it was compared (the `stats-before.normalized.json` line above) but not copied into `evidence/`,
and its run directory no longer exists, so that reading rests only on the `diff -u` transcript.
In the stored client responses, each UDP response is followed by a newline that the raw output
lacks, because the UDP client prints no trailing newline; nothing else differs:

- Client responses: [`m1-client-responses.txt`](evidence/m1-client-responses.txt)
- Metrics before the announces: [`m1-metrics-before.txt`](evidence/m1-metrics-before.txt)
- Metrics after the announces: [`m1-metrics-after.txt`](evidence/m1-metrics-after.txt)
- Stats after the announces: [`m1-stats-after.json`](evidence/m1-stats-after.json)

Selected post-announce metric samples, one or more per repository changed by this issue (labels
abbreviated):

```text
http_tracker_core_requests_received_total {request_kind=announce, port=18070} 2
udp_tracker_core_requests_received_total {request_kind=announce, port=17969} 2
udp_tracker_core_requests_received_total {request_kind=connect, port=17969} 2
udp_tracker_server_requests_received_total {port=17969} 4
udp_tracker_server_responses_sent_total {request_kind=announce, result=ok} 2
tracker_core_in_session_torrents_downloads_total {} 2
tracker_core_persisted_torrents_downloads_total {} 2
swarm_coordination_registry_peers_added_total {peer_role=leecher} 2
swarm_coordination_registry_peers_updated_total {peer_role=seeder} 2
swarm_coordination_registry_torrents_downloads_total {peer_role=seeder} 2
swarm_coordination_registry_torrents_total {} 1.0
```

Selected post-announce stats: `torrents=1`, `seeders=2`, `leechers=0`, `completed=2`,
`tcp4_announces_handled=2`, `udp4_announces_handled=2`, `udp4_requests=4`.

Tracker logs: each run wrote 220 lines with no `WARN` or `ERROR` entries, and both ended with:

```text
INFO torrust_tracker: Torrust tracker successfully shutdown.
```

At the `info` trace level the tracker does not log individual announces, so the logs show only
startup, the shutdown of each statistics event listener, and successful termination.

#### Conclusion

M1 passed. On `develop` and on the implementation branch, the same announces produced identical
client responses, identical REST stats, and identical samples for all five metric families,
apart from the masked processing times.

## Failures and Follow-up

- A first M1 run (recorded in commit
  `docs(issues): [#2517] record implementation verification`) compared only some REST stats
  fields and the HTTP and UDP core announce samples. The independent review found that this did
  not cover the tracker-core and swarm metrics required by the expected result. M1 was rerun as
  above. The superseded content remains in the branch history.
- The default development ports could not be used because a pre-existing tracker process held
  the health-check port `127.0.0.1:1313`. That process was left untouched.
