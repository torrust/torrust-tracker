---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2521-1669-remove-unused-udp-stats-traits/ISSUE.md
last-updated-utc: 2026-10-14 13:05
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

`.tmp/` and other git-ignored paths are not part of the repository, so a path
there is not evidence once the run ends. Record everything a reviewer needs in
this issue folder:

- Embed small artifacts, such as a tracker configuration file, verbatim in
  this document.
- Store larger artifacts, such as full logs or captured responses, in an
  `evidence/` subfolder of the issue folder and link them from here.
- Record runtime outputs (database rows, responses, relevant log lines) inline
  in each process.

A local `.tmp/` path may still be mentioned to show where a run wrote its
files, but never as the only copy of something the evidence relies on.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-14, baseline run at about 12:50 and after run at about 12:59.
- Artifact under test:
  - Baseline: `develop` at `c1874facb` (`Merge torrust/torrust-tracker#2526`) plus only the
    spec commit `docs(issues): [#2521] record maintainer decisions before implementation`, so no
    code change.
  - After: the implementation branch with every code patch applied, up to and including
    `docs(issues): [#2521] note the SI-30 trait removal in EPIC #1669`.
- Operating system / environment: Linux workstation; debug builds of `torrust-tracker` and
  `tracker_client` (`cargo build`, nightly 1.101.0 directory override).
- Prerequisites and setup performed: another agent's tracker was already running on this machine
  (UDP `16969`, API `11212`, health check `1313`), so this run used a private configuration on
  distinct loopback ports and a throwaway SQLite database under `.tmp/2521/`.

Tracker configuration (`.tmp/2521/tracker.toml`), derived from
`share/default/config/tracker.development.sqlite3.toml`:

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "./.tmp/2521/db/sqlite3.db"

[[udp_trackers]]
bind_address = "127.0.0.1:26969"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:21212"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "127.0.0.1:21313"
```

Each run used the same steps (`.tmp/2521/run-scenario.sh <baseline|after>`): start from an empty
database, wait for the health check, send one UDP announce, wait one second, capture the four REST
responses, and stop the tracker.

```bash
./target/debug/torrust-tracker --config-toml-path ./.tmp/2521/tracker.toml &
./target/debug/tracker_client udp announce 127.0.0.1:26969 9c38422213e30bff212b30c360d26f9a02136422
curl -sf "http://127.0.0.1:21212/api/v1/stats?token=MyAccessToken&format=json"
curl -sf "http://127.0.0.1:21212/api/v1/stats?token=MyAccessToken&format=prometheus"
curl -sf "http://127.0.0.1:21212/api/v1/metrics?token=MyAccessToken&format=json"
curl -sf "http://127.0.0.1:21212/api/v1/metrics?token=MyAccessToken&format=prometheus"
```

Both runs returned the same announce response:

```json
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
```

The captured responses are stored verbatim in [evidence/baseline/](./evidence/baseline/) and
[evidence/after/](./evidence/after/). A Python comparison then checked, per format, that the key
order, the series (name and labels), the `HELP` and `TYPE` lines, and every value except the
volatile `*processing_time*` values are identical:

```text
M1 json same keys: True | differing non-timing values: {}
M1 json udp4_announces_handled: 1 1
stats.prometheus same series: True (32 series) | differing non-timing values: {}
stats.prometheus HELP/TYPE lines same: True
metrics.prometheus same series: True (17 series) | differing non-timing values: {}
metrics.prometheus HELP/TYPE lines same: True
M2 json same (name,type,labels): True (17 samples) | differing non-timing values: {}
M2 json udp metric names: ['udp_tracker_core_requests_received_total', 'udp_tracker_server_performance_avg_processed_requests_total', 'udp_tracker_server_performance_avg_processing_time_ns', 'udp_tracker_server_requests_accepted_total', 'udp_tracker_server_requests_received_total', 'udp_tracker_server_responses_sent_total']
```

## Verification Processes

### M1 - `/api/v1/stats` Still Reports UDP Server Fields

- Goal: the UDP fields served from the `udp-server` repository are unchanged in JSON and
  Prometheus formats, and the announce is counted.
- Initial state: fresh tracker, empty database, one UDP announce sent.
- Status: `DONE`

#### Steps Performed

1. Ran the baseline and after runs described above.
2. Compared `stats.json` and `stats.prometheus` between the two runs.

#### Observed Result

After run, `stats.prometheus` (the UDP lines; the baseline differs only in the two non-zero
average processing times, `43977` and `67035`):

```text
udp_requests_discarded 0
udp_requests_aborted 0
udp_requests_banned 0
udp_banned_ips_total 0
udp_avg_connect_processing_time_ns 44388
udp_avg_announce_processing_time_ns 61736
udp_avg_scrape_processing_time_ns 0
udp4_requests 3
udp4_connections_handled 2
udp4_announces_handled 1
udp4_scrapes_handled 0
udp4_responses 3
udp4_errors_handled 0
udp6_requests 0
udp6_connections_handled 0
udp6_announces_handled 0
udp6_scrapes_handled 0
udp6_responses 0
udp6_errors_handled 0
```

Full responses: [evidence/after/stats.prometheus](./evidence/after/stats.prometheus) and
[evidence/after/stats.json](./evidence/after/stats.json). The JSON response has the same keys in
the same order in both runs, and `udp4_announces_handled` is `1` in both.

#### Conclusion

Met: both formats expose the same UDP fields as the baseline (including `udp4_requests`,
`udp4_announces_handled`, and `udp_banned_ips_total`), and the announce is counted in
`udp4_announces_handled`.

### M2 - `/api/v1/metrics` Still Reports UDP Core Metrics

- Goal: the merged `udp-core` and `udp-server` metric collections keep their names and labels in
  JSON and Prometheus formats, and the announce is counted.
- Initial state: fresh tracker, empty database, one UDP announce sent.
- Status: `DONE`

#### Steps Performed

1. Ran the baseline and after runs described above.
2. Compared `metrics.json` and `metrics.prometheus` between the two runs.

#### Observed Result

Non-zero `udp_tracker_core_*` series in the after run's `metrics.prometheus` (identical in the
baseline):

```text
udp_tracker_core_requests_received_total{client_address_ip_family="inet",client_address_ip_type="plain",request_kind="announce",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="127.0.0.1",server_binding_port="26969",server_binding_protocol="udp"} 1
udp_tracker_core_requests_received_total{client_address_ip_family="inet",client_address_ip_type="plain",request_kind="connect",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="127.0.0.1",server_binding_port="26969",server_binding_protocol="udp"} 2
```

Both runs report the same 17 series with the same labels, covering
`udp_tracker_core_requests_received_total` and the `udp_tracker_server_*` metrics listed in the
comparison output above (for example `udp_tracker_server_requests_accepted_total` is `1` for
`request_kind="announce"`). Full responses:
[evidence/after/metrics.prometheus](./evidence/after/metrics.prometheus) and
[evidence/after/metrics.json](./evidence/after/metrics.json).

#### Conclusion

Met: both formats contain `udp_tracker_core_requests_received_total` (from `udp-core`) and the
`udp_tracker_server_*` metrics (from `udp-server`) with the same names and labels as the
baseline, and the announce is counted.

## Failures and Follow-up

None. Both scenarios passed on the first after run.
