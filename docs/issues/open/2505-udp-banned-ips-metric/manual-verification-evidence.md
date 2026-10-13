---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
last-updated-utc: 2026-10-13 17:05
---

# Manual Verification Evidence

<!-- cspell:ignore cooki -->

## Purpose

Record the real-artifact reproduction of the UDP banned-IPs metric bug before the fix, and later
the like-for-like recheck after the fix. Every command and output below was actually run and
observed; nothing is simulated.

## Summary

Classified outcome: **Reproduced** (fix-bug skill classification).

| Process | What was done                                       | Reported "banned IPs" | IPs actually banned | Status |
| ------- | --------------------------------------------------- | --------------------- | ------------------- | ------ |
| V1      | One invalid connection ID from `127.0.0.1`          | 1                     | 0                   | DONE   |
| V2      | One invalid connection ID from `.2` and `.3` too    | 3                     | 0                   | DONE   |
| V3      | 11 invalid connection IDs from `127.0.0.4`          | 4                     | 1                   | DONE   |
| V4      | First periodic ban reset (3600 s after start)       | 4 (stale)             | 0                   | DONE   |

The three read surfaces (REST JSON stats, REST Prometheus stats, REST Prometheus metrics) always
agreed, because they all read the same `udp-server` gauge.

## Preserving Verification Artifacts

- The tracker configuration is embedded verbatim below.
- The probe script is kept in this folder: [udp-banned-ips-probe.sh](udp-banned-ips-probe.sh).
- The full tracker log of the run is kept in [evidence/tracker.log](evidence/tracker.log).
- The run used the local, git-ignored working folder `.tmp/udp-banned-ips-metric/repro/` for the
  configuration, the SQLite database, and the live log. Nothing in this document depends on it.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-13, tracker started at 15:09:06 UTC.
- Artifact under test: `torrust-tracker` binary built from `develop` at commit `1c5939c76`
  (`Merge torrust/torrust-tracker#2366`), with no local changes to code.
- Toolchain: stable Rust toolchain, `rustc 1.99.0 (b940084d7 2026-09-28)`,
  `cargo 1.99.0 (5f94df478 2026-08-27)`; `dev` profile build (`cargo build --bin torrust-tracker`).
- Operating system: Ubuntu 26.04.1 LTS, Linux 7.0.0-38-generic x86_64.
- Probe tools: OpenBSD netcat 1.234-1 (Debian package `netcat-openbsd`), `xxd` 2025-11-26, `curl` 8.18.0,
  `jq` 1.8.1.
- Network: all traffic on loopback. Linux accepts any `127.0.0.0/8` source address, so `nc -s`
  simulates distinct client IPs (`127.0.0.1` to `127.0.0.4`).
- Ports: UDP tracker `16969`, REST API `127.0.0.1:11212` (chosen to avoid collisions with other
  trackers on the same machine). The health check API used its default `127.0.0.1:1313`.

### Tracker configuration

Saved locally as `.tmp/udp-banned-ips-metric/repro/tracker.toml`:

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

[core.database]
driver = "sqlite3"
path = "./.tmp/udp-banned-ips-metric/repro/storage/sqlite3.db"

[udp_tracker_server]
# 3600 is the minimum allowed value.
ip_bans_reset_interval_in_secs = 3600
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "0.0.0.0:16969"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:11212"

[http_api.access_tokens]
admin = "MyAccessToken"
```

With `max_connection_id_errors_per_ip = 10`, an IP is banned once it has sent **11** invalid
connection IDs (`is_banned` is `count > limit`).

### Starting the tracker

Each command below is reproduced exactly as it was passed to the agent's Bash tool: a fresh,
non-interactive shell started at the repository root
(`/home/josecelano/Documents/git/committer/me/github/torrust/torrust-tracker-agent-08`). Each
output block is verbatim. In the probe commands, the `echo '$ ...'` calls only print labels into
the output; they are not separate commands.

Write the configuration and build (command):

```bash
mkdir -p .tmp/udp-banned-ips-metric/repro/storage && cat > .tmp/udp-banned-ips-metric/repro/tracker.toml <<'EOF'
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

[core.database]
driver = "sqlite3"
path = "./.tmp/udp-banned-ips-metric/repro/storage/sqlite3.db"

[udp_tracker_server]
# 3600 is the minimum allowed value.
ip_bans_reset_interval_in_secs = 3600
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "0.0.0.0:16969"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:11212"

[http_api.access_tokens]
admin = "MyAccessToken"
EOF
cargo build --bin torrust-tracker 2>&1 | tail -3
```

Output:

```text
   Compiling torrust-tracker v3.0.0-develop (/home/josecelano/Documents/git/committer/me/github/torrust/torrust-tracker-agent-08)
    Finished `dev` profile [optimized + debuginfo] target(s) in 2.45s
```

Start the tracker with an empty database (command, run as a background shell; it prints nothing
because all output goes to `tracker.log`):

```bash
cd /home/josecelano/Documents/git/committer/me/github/torrust/torrust-tracker-agent-08 && rm -rf .tmp/udp-banned-ips-metric/repro/storage/* && date -u '+%Y-%m-%d %H:%M:%S UTC' > .tmp/udp-banned-ips-metric/repro/start-time.txt && ./target/debug/torrust-tracker --config-toml-path ./.tmp/udp-banned-ips-metric/repro/tracker.toml > .tmp/udp-banned-ips-metric/repro/tracker.log 2>&1
```

Relevant startup log lines (from [evidence/tracker.log](evidence/tracker.log)):

```text
2026-10-13T15:09:06.832160Z  INFO initialize: METRICS: type="gauge" name="udp_tracker_server_ips_banned_total" unit=Some(Count) description=Some(MetricDescription("Total number of IPs banned from UDP requests"))
2026-10-13T15:09:06.832316Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=3600
2026-10-13T15:09:06.832320Z  INFO UDP TRACKER: Starting UDP tracker server event listener (banning)
```

Invalid connection IDs are not logged at the `info` level, so the evidence below relies on the
protocol responses and the API reads.

### Probe script

[udp-banned-ips-probe.sh](udp-banned-ips-probe.sh) has three commands:

- `invalid-announce <source-ip>`: sends a 98-byte BEP 15 announce with connection ID
  `0000000000000000` (an invalid cookie) and transaction ID `0000abcd`, and prints the response.
- `connect <source-ip>`: sends a 16-byte BEP 15 connect request (protocol ID `0x41727101980`) and
  prints the returned connection ID, or `<no response>` after a 2-second timeout.
- `stats`: reads `udp_banned_ips_total` from `/api/v1/stats` (JSON and Prometheus) and
  `udp_tracker_server_ips_banned_total` and `udp_tracker_server_requests_banned_total` from
  `/api/v1/metrics?format=prometheus`.

The probe was run from `.tmp/udp-banned-ips-metric/repro/` as `./probe.sh`. The committed script
differs from it in two ways, neither of which changes the behavior of a valid invocation: a longer
header comment, and argument validation added afterwards for PR #2506 review finding F5 (a missing
command or source IP now prints the usage and exits 2 instead of failing with an "unbound variable"
error).

## Verification Processes

### Baseline

The baseline and V1 ran as one command; its first part prints the recorded start time, checks that
the cleanup job started (`1` matching log line), and reads the baseline. See V1 for the command and
the full output.

### V1 - One Invalid Connection ID, Not Banned

- Goal: show that one invalid connection ID is reported as one banned IP although the client is
  not banned (scenario M1).
- Initial state: fresh tracker, baseline above.
- Status: `DONE` (before the fix).

#### Steps Performed

Command:

```bash
sleep 3; cd .tmp/udp-banned-ips-metric/repro && cat start-time.txt && grep -c "Starting UDP IP-ban cleanup job" tracker.log && date -u '+%F %T UTC' && echo '$ ./probe.sh stats' && ./probe.sh stats && echo && echo '$ ./probe.sh invalid-announce 127.0.0.1' && ./probe.sh invalid-announce 127.0.0.1 && sleep 1 && echo '$ ./probe.sh stats' && ./probe.sh stats && echo '$ ./probe.sh connect 127.0.0.1' && ./probe.sh connect 127.0.0.1
```

Output:

```text
2026-10-13 15:09:06 UTC
1
2026-10-13 15:09:13 UTC
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 0
Prom  /stats   udp_banned_ips_total 0
Prom  /metrics udp_tracker_server_ips_banned_total <absent>
Prom  /metrics udp_tracker_server_requests_banned_total <absent>

$ ./probe.sh invalid-announce 127.0.0.1
response (hex): 000000030000abcd747261636b657220616e6e6f756e6365206572726f723a20436f6e6e656374696f6e20636f6f6b6965206572726f723a20636f6f6b69652076616c756520697320657870697265643a20302e303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303032323831373130363936363630373935362c206578706563746564203e20313739313930343033322e39383234373732
response (error message): tracker announce error: Connection cookie error: cookie value is expired: 0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000022817106966607956, expected > 1791904032.9824772
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 1
Prom  /stats   udp_banned_ips_total 1
Prom  /metrics udp_tracker_server_ips_banned_total 1
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.1
response (hex): 000000000000123455608a159e7a26fb
response (connect): connection_id=55608a159e7a26fb
```

#### Observed Result

The tracker answered with a connection-cookie error (action 3, transaction `0000abcd`), which is the
event the banning handler counts. All three surfaces then reported **1** banned IP. The same client
then got a normal connect response, so it was **not** banned.

#### Conclusion

Wrong outcome observed: a client with one invalid connection ID (limit 10) is reported as banned.

### V2 - Several IPs With One Invalid Connection ID, None Banned

- Goal: show that the value grows with every distinct IP that sent one invalid connection ID
  (scenario M2).
- Initial state: after V1 (`127.0.0.1` has 1 error).
- Status: `DONE` (before the fix).

#### Steps Performed

Command (the error-message lines are filtered with `grep` and cut at 80 characters, and the connect
responses are filtered to the decoded line):

```bash
cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; for ip in 127.0.0.2 127.0.0.3; do echo "\$ ./probe.sh invalid-announce $ip"; ./probe.sh invalid-announce $ip | grep "error message" | cut -c1-80; done; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats; for ip in 127.0.0.1 127.0.0.2 127.0.0.3; do echo "\$ ./probe.sh connect $ip"; ./probe.sh connect $ip | grep "connect\|no response"; done
```

Output:

```text
2026-10-13 15:09:26 UTC
$ ./probe.sh invalid-announce 127.0.0.2
response (error message): tracker announce error: Connection cookie error: cooki
$ ./probe.sh invalid-announce 127.0.0.3
response (error message): tracker announce error: Connection cookie error: cooki
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 3
Prom  /stats   udp_banned_ips_total 3
Prom  /metrics udp_tracker_server_ips_banned_total 3
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.1
response (connect): connection_id=db1c18db05a6e1b6
$ ./probe.sh connect 127.0.0.2
response (connect): connection_id=2ee2b78ebb610b2b
$ ./probe.sh connect 127.0.0.3
response (connect): connection_id=c6125d8cc2451d22
```

#### Observed Result

All three surfaces reported **3** banned IPs. All three clients got normal connect responses, and
`udp_tracker_server_requests_banned_total` was still absent (no request was ever dropped).

#### Conclusion

Wrong outcome observed: 3 reported, 0 actually banned.

### V3 - One IP Really Banned

- Goal: show the reported value when exactly one IP is really banned (scenario M3), and confirm
  enforcement itself is correct.
- Initial state: after V2 (`.1`, `.2`, `.3` have 1 error each).
- Status: `DONE` (before the fix).

#### Steps Performed

Command. Its first `echo` prints a simplified label for the loop; the loop actually run is the one
that follows it, which prefixes each line with a counter and cuts the error message at 90
characters:

```bash
cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; echo '$ for i in $(seq 1 11); do ./probe.sh invalid-announce 127.0.0.4; done'; for i in $(seq 1 11); do printf "%2d: " $i; ./probe.sh invalid-announce 127.0.0.4 | grep "error message" | cut -c1-90; done; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats; echo '$ ./probe.sh connect 127.0.0.4'; ./probe.sh connect 127.0.0.4; echo '$ ./probe.sh connect 127.0.0.1'; ./probe.sh connect 127.0.0.1 | grep "connect"; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats
```

Output:

```text
2026-10-13 15:09:42 UTC
$ for i in $(seq 1 11); do ./probe.sh invalid-announce 127.0.0.4; done
 1: response (error message): tracker announce error: Connection cookie error: cookie value is
 2: response (error message): tracker announce error: Connection cookie error: cookie value is
 3: response (error message): tracker announce error: Connection cookie error: cookie value is
 4: response (error message): tracker announce error: Connection cookie error: cookie value is
 5: response (error message): tracker announce error: Connection cookie error: cookie value is
 6: response (error message): tracker announce error: Connection cookie error: cookie value is
 7: response (error message): tracker announce error: Connection cookie error: cookie value is
 8: response (error message): tracker announce error: Connection cookie error: cookie value is
 9: response (error message): tracker announce error: Connection cookie error: cookie value is
10: response (error message): tracker announce error: Connection cookie error: cookie value is
11: response (error message): tracker announce error: Connection cookie error: cookie value is
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 4
Prom  /stats   udp_banned_ips_total 4
Prom  /metrics udp_tracker_server_ips_banned_total 4
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.4
<no response>
$ ./probe.sh connect 127.0.0.1
response (connect): connection_id=87478fba936fc829
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 4
Prom  /stats   udp_banned_ips_total 4
Prom  /metrics udp_tracker_server_ips_banned_total 4
Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
```

#### Observed Result

All 11 invalid announces were answered with a cookie error, so `127.0.0.4` reached 11 errors. Its
next connect got **no response** (dropped), and `udp_tracker_server_requests_banned_total` became 1:
exactly one IP is banned. `127.0.0.1` was still served. The "banned IPs" value was **4** on all
three surfaces.

#### Conclusion

Wrong outcome observed: 4 reported, 1 actually banned. Enforcement itself is correct.

### V4 - Gauge After the Periodic Reset

- Goal: show that the gauge keeps its old value after the periodic reset clears all counters
  (scenario M4).
- Initial state: after V3 (`.1`, `.2`, `.3` have 1 error each; `.4` has 11 and is banned). The
  tracker had been running since 15:09:06 UTC with `ip_bans_reset_interval_in_secs = 3600`.
- Status: `DONE` (before the fix).

#### Steps Performed

1. Waited for the first reset by polling the tracker log. Command:

   ```bash
   cd .tmp/udp-banned-ips-metric/repro && until grep -q "filter cleared" tracker.log; do sleep 5; done; date -u '+%F %T UTC'; grep "filter cleared" tracker.log
   ```

   Output:

   ```text
   2026-10-13 16:09:08 UTC
   2026-10-13T16:09:06.833590Z  INFO UDP TRACKER: Udp::run_udp_server::loop (connection id errors filter cleared)
   ```

2. Read the metric and checked that the banned IP is served again. Command:

   ```bash
   cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; echo '$ ./probe.sh stats'; ./probe.sh stats; echo '$ ./probe.sh connect 127.0.0.4'; ./probe.sh connect 127.0.0.4; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats
   ```

   Output:

   ```text
   2026-10-13 16:09:12 UTC
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 4
   Prom  /stats   udp_banned_ips_total 4
   Prom  /metrics udp_tracker_server_ips_banned_total 4
   Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
   $ ./probe.sh connect 127.0.0.4
   response (hex): 00000000000012341342b281da97bf46
   response (connect): connection_id=1342b281da97bf46
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 4
   Prom  /stats   udp_banned_ips_total 4
   Prom  /metrics udp_tracker_server_ips_banned_total 4
   Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
   ```

3. Sent one invalid connection ID from a new IP to show when the gauge is next written. Command
   (the error message is cut at 80 characters and the stats output limited to its first three
   lines):

   ```bash
   cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; echo '$ ./probe.sh invalid-announce 127.0.0.5'; ./probe.sh invalid-announce 127.0.0.5 | grep "error message" | cut -c1-80; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats | head -3
   ```

   Output:

   ```text
   2026-10-13 16:09:21 UTC
   $ ./probe.sh invalid-announce 127.0.0.5
   response (error message): tracker announce error: Connection cookie error: cooki
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 1
   Prom  /stats   udp_banned_ips_total 1
   Prom  /metrics udp_tracker_server_ips_banned_total 1
   ```

#### Observed Result

The reset ran at 16:09:06 UTC and cleared the counters: `127.0.0.4`, banned in V3, got a normal
connect response again. Nothing was banned any more, yet all three surfaces still reported **4**.
The value only changed when the next connection-cookie error arrived, and then it reported **1**
for a client with a single invalid connection ID, which is again not banned.

#### Conclusion

Wrong outcome observed for the second defect: the reset does not update the gauge, so it reports a
stale total until the next connection-cookie error. Step 3 also shows the first defect again on the
fresh state.

The tracker was stopped (`SIGTERM` from the agent shell) after this step.

## Failures and Follow-up

- First attempt (tracker started at 15:07:57 UTC) used `socat` to send the probes. Each probe
  produced an extra "Couldn't parse action" error response besides the expected cookie error,
  because `socat` sent an extra datagram. That error is not a connection-cookie error, so it does
  not change the ban counter (the reported value was 1 after one probe, as in V1), but it made the
  evidence noisy. The probe was switched to OpenBSD netcat, the tracker was restarted with an empty
  database at 15:09:06 UTC, and all processes above come from that clean run.
