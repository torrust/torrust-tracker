---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2536-1669-hide-statistics-repository-read-lock/ISSUE.md
last-updated-utc: 2026-10-15 19:50
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-10-15, baseline run about 17:57 and branch run about 19:42.
- Artifact under test:
  - Baseline: `develop` at `fe5d8264a9898e0a7482f6662e2e0f76365c9547`. The baseline binaries were
    built from this branch before any code change; at that point the branch differed from
    `develop` only by two spec commits.
  - Implementation: the branch patches titled
    - `refactor(udp-core): [#2536] hide the statistics read lock and metric collection field`
    - `refactor(udp-server): [#2536] hide the statistics read lock and metric collection field`
    - `refactor(http-core): [#2536] hide the statistics read lock and metric collection field`
    - `refactor(tracker-core): [#2536] hide the statistics read lock and metric collection field`
    - `refactor(swarm-coordination-registry): [#2536] hide the statistics read lock and metric collection field`
- Operating system / environment: Linux, local development workspace, `dev` profile binaries built
  with `cargo build --bin torrust-tracker` and `cargo build -p torrust-tracker-client --bin tracker_client`.
- Prerequisites and setup performed: each run used a fresh SQLite database and the same isolated
  ports. Both binaries were copied out of `target/` first, so both runs could use them without
  rebuilding. The two runs differed only in the binary and in the database path.

Configuration (the database path was `<run-dir>/database.sqlite3`; the `debug` run below changed
only `trace_filter` to `"debug"`):

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
path = "<run-dir>/database.sqlite3"

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

### V1 - REST metrics unchanged (M1)

- Goal: show that the same UDP and HTTP announces produce the same REST statistics and the same
  samples for every metric family (HTTP core, UDP core, UDP server, tracker core, and swarm
  coordination registry) on `develop` and on the branch, and that debug logging still works.
- Initial state: fresh database; every metric family empty.
- Status: `DONE`

#### Steps Performed

The same command sequence ran with each binary, wrapped in a session-local shell script
(not committed) for exact repetition; its steps were:

1. Start `torrust-tracker --config-toml-path <run-dir>/tracker.toml`, logging to
   `<run-dir>/tracker.log`, and poll `http://127.0.0.1:18212/api/health_check` until it answers.
   The aggregate `/health_check` endpoint was not used, because it sends UDP requests that would
   change the UDP metrics.
2. Read `/api/v1/stats` and `/api/v1/metrics` with
   `curl -fsS -H 'Authorization: Bearer MyAccessToken'`.
3. Send four announces for info-hash `9c38422213e30bff212b30c360d26f9a02136422` with
   `tracker_client`:

   ```sh
   tracker_client http announce http://127.0.0.1:18070 9c38422213e30bff212b30c360d26f9a02136422 --event started --left 1000 --port 6881 '--peer-id=-RC00000000000000001'
   tracker_client udp announce 127.0.0.1:17969 9c38422213e30bff212b30c360d26f9a02136422 --event started --left 1000 --port 6882 '--peer-id=-RC00000000000000002'
   tracker_client http announce http://127.0.0.1:18070 9c38422213e30bff212b30c360d26f9a02136422 --event completed --left 0 --port 6881 '--peer-id=-RC00000000000000001'
   tracker_client udp announce 127.0.0.1:17969 9c38422213e30bff212b30c360d26f9a02136422 --event completed --left 0 --port 6882 '--peer-id=-RC00000000000000002'
   ```

4. Wait one second for the asynchronous statistics event listeners, then read `/api/v1/stats` and
   `/api/v1/metrics` again and stop the tracker.
5. Normalize each output as the spec requires:
   - metrics: one line per sample (`name {sorted labels} value`, or `name <no samples>`), sorted;
     `recorded_at` ignored;
   - in metrics and stats, the values of `*processing_time*` entries replaced with `<timing>`;
   - in metrics, each line of `swarm_coordination_registry_peers_inactive_total` and
     `swarm_coordination_registry_torrents_inactive_total` replaced with `name <activity-job>`;
   - stats: sorted keys, two-space JSON indentation.
6. Compare with `diff -u`.

Before the first comparison, the `develop` run was repeated and its normalized outputs were
identical to the first run's, which shows the normalization is deterministic.

Stored outputs (five per run), all under [`evidence/`](evidence/):

| Output                    | `develop`                                                                     | Branch                                                                      |
| ------------------------- | ----------------------------------------------------------------------------- | --------------------------------------------------------------------------- |
| Client responses          | [`m1-develop-client-responses.txt`](evidence/m1-develop-client-responses.txt) | [`m1-branch-client-responses.txt`](evidence/m1-branch-client-responses.txt) |
| Stats before announces    | [`m1-develop-stats-before.json`](evidence/m1-develop-stats-before.json)       | [`m1-branch-stats-before.json`](evidence/m1-branch-stats-before.json)       |
| Stats after announces     | [`m1-develop-stats-after.json`](evidence/m1-develop-stats-after.json)         | [`m1-branch-stats-after.json`](evidence/m1-branch-stats-after.json)         |
| Metrics before announces  | [`m1-develop-metrics-before.txt`](evidence/m1-develop-metrics-before.txt)     | [`m1-branch-metrics-before.txt`](evidence/m1-branch-metrics-before.txt)     |
| Metrics after announces   | [`m1-develop-metrics-after.txt`](evidence/m1-develop-metrics-after.txt)       | [`m1-branch-metrics-after.txt`](evidence/m1-branch-metrics-after.txt)       |

#### Observed Result

The comparison output:

```text
== diff -u develop/client-responses.txt branch/client-responses.txt
(no differences)
== diff -u develop/stats-before.json branch/stats-before.json
(no differences)
== diff -u develop/stats-after.json branch/stats-after.json
(no differences)
== diff -u develop/metrics-before.txt branch/metrics-before.txt
(no differences)
== diff -u develop/metrics-after.txt branch/metrics-after.txt
(no differences)
```

The two tracker binaries differ (their md5 sums differ), so the branch run did exercise the new code.

Lazy debug logging (AC4), checked at runtime on the branch binary with two announces (one HTTP,
one UDP):

- With `trace_filter = "info"` the run log had `0` `stats: Metrics` lines.
- With `trace_filter = "debug"` the log had `9` `stats: Metrics { metric_collection: ... }` lines
  (the text format is unchanged), from the three handlers
  ([`evidence/m1-debug-log-stats-lines-by-target.txt`](evidence/m1-debug-log-stats-lines-by-target.txt)):

```text
      1 torrust_tracker_http_core::statistics::event::handler: stats:
      2 torrust_tracker_udp_core::statistics::event::handler: stats:
      6 torrust_tracker_udp_server::statistics::event::handler: stats:
```

#### Conclusion

The five normalized outputs are byte-identical between `develop` and the branch, so the REST
`/api/v1/stats` and `/api/v1/metrics` report the same values for the same announces (AC5). The
debug logs are still produced only when `debug` is enabled and keep their text (AC4).

## Failures and Follow-up

None. One debug-run attempt failed for a mistake in my command, not in the tracker: the first
command was rejected before it created the debug configuration file, so the tracker could not load
`debug.toml`. I redid the run with the file in place; the result above is from that rerun.
