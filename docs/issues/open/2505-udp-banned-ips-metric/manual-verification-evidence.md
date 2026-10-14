---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2505-udp-banned-ips-metric/ISSUE.md
last-updated-utc: 2026-10-14 08:48
---

# Manual Verification Evidence

<!-- cspell:ignore cooki -->

## Purpose

Record the real-artifact reproduction of the UDP banned-IPs metric bug before the fix, and later
the like-for-like recheck after the fix. Every command and output below was actually run and
observed; nothing is simulated.

## Summary

Classified outcome: **Reproduced** (fix-bug skill classification). Final recheck: **fixed**.

| Process | What was done                                    | Before the fix (reported / banned) | After the fix (reported / banned) |
| ------- | ------------------------------------------------ | ---------------------------------- | --------------------------------- |
| V1      | One invalid connection ID from `127.0.0.1`       | 1 / 0                              | 0 / 0                             |
| V2      | One invalid connection ID from `.2` and `.3` too | 3 / 0                              | 0 / 0                             |
| V3      | 11 invalid connection IDs from `127.0.0.4`       | 4 / 1                              | 1 / 1                             |
| V4      | First periodic ban reset (3600 s after start)    | 4 (stale) / 0                      | 0 / 0                             |

The "before" column comes from the reproduction (V1-V4); the "after" column from the like-for-like
recheck on the finished code ("Final Recheck on the Finished Code (T10)"), which matches the
earlier recheck on the build before the variant-B commit.

The three read surfaces (REST JSON stats, REST Prometheus stats, REST Prometheus metrics) always
agreed, because they all read the same `udp-server` gauge.

## Preserving Verification Artifacts

- The tracker configuration is embedded verbatim below.
- The probe script is kept in this folder: [udp-banned-ips-probe.sh](udp-banned-ips-probe.sh).
- The full tracker log of the run is kept in [evidence/tracker.log](evidence/tracker.log); the
  recheck logs are [evidence/recheck-tracker.log](evidence/recheck-tracker.log) and
  [evidence/final-recheck-tracker.log](evidence/final-recheck-tracker.log). The logs keep the
  tracker's ANSI color codes; the lines quoted in this document have them removed.
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

## Recheck After the Fix (T10)

Like-for-like rerun of V1 to V4 on the fixed build, with the same configuration (rewritten to
`.tmp/udp-banned-ips-metric/repro/tracker.toml` from the block in "Tracker configuration"), the same
probe (the committed [udp-banned-ips-probe.sh](udp-banned-ips-probe.sh), copied to
`.tmp/udp-banned-ips-metric/repro/probe.sh`), and the same commands as the reproduction.

- Code: `docs(udp-server): [#2505] describe the banned-IPs metric as IPs over the limit`, which
  carries `fix(udp-core): [#2505] count only IPs over the limit as banned` and
  `fix(udp-server): [#2505] publish the banned-IPs total when bans are reset`. This is **not** the
  finished code: the later `perf(udp-core): [#2505] detect the ban crossing with one comparison`
  changed the crossing check, so the whole recheck was repeated in "Final Recheck on the Finished
  Code (T10)".
- Build: `cargo build --bin torrust-tracker` (`Finished dev profile [optimized + debuginfo]
  target(s) in 6.92s`), stable `rustc 1.99.0 (b940084d7 2026-09-28)`. UDP 16969 and TCP 11212 and
  1313 were free before the start.
- Tracker start (background shell), as in the reproduction:

  ```bash
  cd /home/josecelano/Documents/git/committer/me/github/torrust/torrust-tracker-agent-08 && rm -rf .tmp/udp-banned-ips-metric/repro/storage/* && date -u '+%Y-%m-%d %H:%M:%S UTC' > .tmp/udp-banned-ips-metric/repro/start-time.txt && ./target/debug/torrust-tracker --config-toml-path ./.tmp/udp-banned-ips-metric/repro/tracker.toml > .tmp/udp-banned-ips-metric/repro/tracker.log 2>&1
  ```

The cookie error text differs from the reproduction because each tracker start uses a new cookie
key, so the decoded value of connection ID 0 differs; it is still a connection-cookie error.

### R-V1 - One Invalid Connection ID, Not Banned

Command (the same as V1):

```bash
sleep 3; cd .tmp/udp-banned-ips-metric/repro && cat start-time.txt && grep -c "Starting UDP IP-ban cleanup job" tracker.log && date -u '+%F %T UTC' && echo '$ ./probe.sh stats' && ./probe.sh stats && echo && echo '$ ./probe.sh invalid-announce 127.0.0.1' && ./probe.sh invalid-announce 127.0.0.1 && sleep 1 && echo '$ ./probe.sh stats' && ./probe.sh stats && echo '$ ./probe.sh connect 127.0.0.1' && ./probe.sh connect 127.0.0.1
```

Output:

```text
2026-10-13 20:05:55 UTC
1
2026-10-13 20:06:03 UTC
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 0
Prom  /stats   udp_banned_ips_total 0
Prom  /metrics udp_tracker_server_ips_banned_total <absent>
Prom  /metrics udp_tracker_server_requests_banned_total <absent>

$ ./probe.sh invalid-announce 127.0.0.1
response (hex): 000000030000abcd747261636b657220616e6e6f756e6365206572726f723a20436f6e6e656374696f6e20636f6f6b6965206572726f723a20636f6f6b69652076616c756520697320657870697265643a202d302e303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030343838353939343339373131333234312c206578706563746564203e20313739313932313834332e30343532383938
response (error message): tracker announce error: Connection cookie error: cookie value is expired: -0.0000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000004885994397113241, expected > 1791921843.0452898
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 0
Prom  /stats   udp_banned_ips_total 0
Prom  /metrics udp_tracker_server_ips_banned_total 0
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.1
response (hex): 00000000000012347f7c1d4122ebfb4c
response (connect): connection_id=7f7c1d4122ebfb4c
```

Result: the gauge is now published, with **0** (V1: 1). The client is still served.

### R-V2 - Several IPs With One Invalid Connection ID, None Banned

Command (the same as V2):

```bash
cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; for ip in 127.0.0.2 127.0.0.3; do echo "\$ ./probe.sh invalid-announce $ip"; ./probe.sh invalid-announce $ip | grep "error message" | cut -c1-80; done; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats; for ip in 127.0.0.1 127.0.0.2 127.0.0.3; do echo "\$ ./probe.sh connect $ip"; ./probe.sh connect $ip | grep "connect\|no response"; done
```

Output:

```text
2026-10-13 20:06:15 UTC
$ ./probe.sh invalid-announce 127.0.0.2
response (error message): tracker announce error: Connection cookie error: cooki
$ ./probe.sh invalid-announce 127.0.0.3
response (error message): tracker announce error: Connection cookie error: cooki
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 0
Prom  /stats   udp_banned_ips_total 0
Prom  /metrics udp_tracker_server_ips_banned_total 0
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.1
response (connect): connection_id=108b1f796545a549
$ ./probe.sh connect 127.0.0.2
response (connect): connection_id=4800fcfef3e4ae3f
$ ./probe.sh connect 127.0.0.3
response (connect): connection_id=a193110e27054e80
```

Result: **0** banned IPs (V2: 3); all three clients served.

### R-V3 - One IP Really Banned

Command (the same as V3; its first `echo` prints a simplified label for the loop that follows):

```bash
cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; echo '$ for i in $(seq 1 11); do ./probe.sh invalid-announce 127.0.0.4; done'; for i in $(seq 1 11); do printf "%2d: " $i; ./probe.sh invalid-announce 127.0.0.4 | grep "error message" | cut -c1-90; done; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats; echo '$ ./probe.sh connect 127.0.0.4'; ./probe.sh connect 127.0.0.4; echo '$ ./probe.sh connect 127.0.0.1'; ./probe.sh connect 127.0.0.1 | grep "connect"; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats
```

Output:

```text
2026-10-13 20:06:31 UTC
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
JSON  /stats   udp_banned_ips_total: 1
Prom  /stats   udp_banned_ips_total 1
Prom  /metrics udp_tracker_server_ips_banned_total 1
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.4
<no response>
$ ./probe.sh connect 127.0.0.1
response (connect): connection_id=3bbfacdca9ea2555
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 1
Prom  /stats   udp_banned_ips_total 1
Prom  /metrics udp_tracker_server_ips_banned_total 1
Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
```

Result: **1** banned IP (V3: 4), matching the one IP whose request was dropped
(`udp_tracker_server_requests_banned_total` 1). Enforcement is unchanged.

### R-V4 - Gauge After the Periodic Reset

1. Waited for the first reset (the job started at 20:05:55 UTC). Command:

   ```bash
   cd .tmp/udp-banned-ips-metric/repro && until grep -q "filter cleared" tracker.log; do sleep 5; done; date -u '+%F %T UTC'; grep "filter cleared" tracker.log
   ```

   Output:

   ```text
   2026-10-13 21:05:59 UTC
   2026-10-13T21:05:55.966298Z  INFO UDP TRACKER: Udp::run_udp_server::loop (connection id errors filter cleared)
   ```

2. Read the metric and checked that the banned IP is served again. Command (the same as V4 step 2):

   ```bash
   cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; echo '$ ./probe.sh stats'; ./probe.sh stats; echo '$ ./probe.sh connect 127.0.0.4'; ./probe.sh connect 127.0.0.4; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats
   ```

   Output:

   ```text
   2026-10-13 21:06:03 UTC
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 0
   Prom  /stats   udp_banned_ips_total 0
   Prom  /metrics udp_tracker_server_ips_banned_total 0
   Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
   $ ./probe.sh connect 127.0.0.4
   response (hex): 0000000000001234d7218f2a4a3a5fb3
   response (connect): connection_id=d7218f2a4a3a5fb3
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 0
   Prom  /stats   udp_banned_ips_total 0
   Prom  /metrics udp_tracker_server_ips_banned_total 0
   Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
   ```

3. Sent one invalid connection ID from a new IP. Command (the same as V4 step 3):

   ```bash
   cd .tmp/udp-banned-ips-metric/repro && date -u '+%F %T UTC'; echo '$ ./probe.sh invalid-announce 127.0.0.5'; ./probe.sh invalid-announce 127.0.0.5 | grep "error message" | cut -c1-80; sleep 1; echo '$ ./probe.sh stats'; ./probe.sh stats | head -3
   ```

   Output:

   ```text
   2026-10-13 21:06:10 UTC
   $ ./probe.sh invalid-announce 127.0.0.5
   response (error message): tracker announce error: Connection cookie error: cooki
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 0
   Prom  /stats   udp_banned_ips_total 0
   Prom  /metrics udp_tracker_server_ips_banned_total 0
   ```

Result: right after the reset all three surfaces report **0** (V4: a stale 4), and `127.0.0.4` is
served again. A new client with one invalid connection ID still reports **0** (V4: 1).

The tracker was then stopped (`SIGTERM` from the agent shell). Its full log is in
[evidence/recheck-tracker.log](evidence/recheck-tracker.log); the relevant lines:

```text
2026-10-13T20:05:55.964017Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=3600
2026-10-13T21:05:55.966298Z  INFO UDP TRACKER: Udp::run_udp_server::loop (connection id errors filter cleared)
2026-10-13T21:06:19.016030Z  INFO UDP TRACKER: Stopping UDP IP-ban cleanup job ...
```

### Recheck Conclusion

The like-for-like recheck no longer shows either defect: the reported value equals the number of
IPs over the limit in every step, and it is updated when the periodic reset clears the bans.

## Final Recheck on the Finished Code (T10)

The first recheck above ran on the build of
`docs(udp-server): [#2505] describe the banned-IPs metric as IPs over the limit`. The independent
pre-PR review (finding B1 in `agent-review-reports.md`) noted that the later
`perf(udp-core): [#2505] detect the ban crossing with one comparison` changed the counting logic,
so the finished artifact had not been checked by hand. At the maintainer's choice, the whole
recheck was repeated on the final code.

- Code: `test(udp-server): [#2505] make the handler test tell banned IPs from tracked IPs`, the
  last commit (it changes only test code after the variant-B commit).
- Build: `cargo build --bin torrust-tracker` (`Finished dev profile [optimized + debuginfo]
  target(s) in 2.62s`), stable `rustc 1.99.0 (b940084d7 2026-09-28)`. UDP 16969 and TCP 11212 and
  1313 were free and no other tracker was running.
- Configuration and probe: byte-identical to the earlier runs (`diff` of
  `.tmp/udp-banned-ips-metric/repro/tracker.toml` against the "Tracker configuration" block, and of
  `.tmp/udp-banned-ips-metric/repro/probe.sh` against the committed probe, printed nothing).
- Tracker start: the same command as the reproduction, at 2026-10-14 06:53:20 UTC. A first start
  at 06:52:12 UTC was discarded because V2 and V3 ran concurrently by mistake (see "Failures and
  Follow-up").

### F-V1 - One Invalid Connection ID, Not Banned

Command: the same as V1. Output:

```text
2026-10-14 06:53:20 UTC
1
2026-10-14 06:53:28 UTC
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 0
Prom  /stats   udp_banned_ips_total 0
Prom  /metrics udp_tracker_server_ips_banned_total <absent>
Prom  /metrics udp_tracker_server_requests_banned_total <absent>

$ ./probe.sh invalid-announce 127.0.0.1
response (hex): 000000030000abcd747261636b657220616e6e6f756e6365206572726f723a20436f6e6e656374696f6e20636f6f6b6965206572726f723a20636f6f6b69652076616c756520697320657870697265643a202d32303135343638393731393538313034373030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030303030302c206578706563746564203e20313739313936303638372e36313330353132
response (error message): tracker announce error: Connection cookie error: cookie value is expired: -20154689719581047000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000, expected > 1791960687.6130512
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 0
Prom  /stats   udp_banned_ips_total 0
Prom  /metrics udp_tracker_server_ips_banned_total 0
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.1
response (hex): 0000000000001234f8a69dfe88215934
response (connect): connection_id=f8a69dfe88215934
```

### F-V2 - Several IPs With One Invalid Connection ID, None Banned

Command: the same as V2, run after F-V1 finished. Output:

```text
2026-10-14 06:53:38 UTC
$ ./probe.sh invalid-announce 127.0.0.2
response (error message): tracker announce error: Connection cookie error: cooki
$ ./probe.sh invalid-announce 127.0.0.3
response (error message): tracker announce error: Connection cookie error: cooki
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 0
Prom  /stats   udp_banned_ips_total 0
Prom  /metrics udp_tracker_server_ips_banned_total 0
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.1
response (connect): connection_id=728b39f69f44a159
$ ./probe.sh connect 127.0.0.2
response (connect): connection_id=211ff600d45e2b4e
$ ./probe.sh connect 127.0.0.3
response (connect): connection_id=ae3050d3a9068e81
```

### F-V3 - One IP Really Banned

Command: the same as V3, run after F-V2 finished. Output:

```text
2026-10-14 06:53:56 UTC
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
JSON  /stats   udp_banned_ips_total: 1
Prom  /stats   udp_banned_ips_total 1
Prom  /metrics udp_tracker_server_ips_banned_total 1
Prom  /metrics udp_tracker_server_requests_banned_total <absent>
$ ./probe.sh connect 127.0.0.4
<no response>
$ ./probe.sh connect 127.0.0.1
response (connect): connection_id=375d219f0d7ee63a
$ ./probe.sh stats
JSON  /stats   udp_banned_ips_total: 1
Prom  /stats   udp_banned_ips_total 1
Prom  /metrics udp_tracker_server_ips_banned_total 1
Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
```

### F-V4 - Gauge After the Periodic Reset

1. Waited for the first reset (the job started at 06:53:20 UTC). Command: the same as V4 step 1.
   Output:

   ```text
   2026-10-14 07:53:25 UTC
   2026-10-14T07:53:20.588414Z  INFO UDP TRACKER: Udp::run_udp_server::loop (connection id errors filter cleared)
   ```

2. Command: the same as V4 step 2. Output:

   ```text
   2026-10-14 07:53:30 UTC
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 0
   Prom  /stats   udp_banned_ips_total 0
   Prom  /metrics udp_tracker_server_ips_banned_total 0
   Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
   $ ./probe.sh connect 127.0.0.4
   response (hex): 0000000000001234ff4e55fbb6bb19fe
   response (connect): connection_id=ff4e55fbb6bb19fe
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 0
   Prom  /stats   udp_banned_ips_total 0
   Prom  /metrics udp_tracker_server_ips_banned_total 0
   Prom  /metrics udp_tracker_server_requests_banned_total{client_address_ip_family="inet",client_address_ip_type="plain",server_binding_address_ip_family="inet",server_binding_address_ip_type="plain",server_binding_ip="0.0.0.0",server_binding_port="16969",server_binding_protocol="udp"} 1
   ```

3. Command: the same as V4 step 3. Output:

   ```text
   2026-10-14 07:53:37 UTC
   $ ./probe.sh invalid-announce 127.0.0.5
   response (error message): tracker announce error: Connection cookie error: cooki
   $ ./probe.sh stats
   JSON  /stats   udp_banned_ips_total: 0
   Prom  /stats   udp_banned_ips_total 0
   Prom  /metrics udp_tracker_server_ips_banned_total 0
   ```

The tracker was then stopped (`SIGTERM` from the agent shell). Its full log is in
[evidence/final-recheck-tracker.log](evidence/final-recheck-tracker.log); the relevant lines:

```text
2026-10-14T06:53:20.576980Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=3600
2026-10-14T07:53:20.588414Z  INFO UDP TRACKER: Udp::run_udp_server::loop (connection id errors filter cleared)
2026-10-14T07:53:45.213086Z  INFO UDP TRACKER: Stopping UDP IP-ban cleanup job ...
```

### Final Recheck Conclusion

On the finished code the reported value is 0, 0, 1, and 0 after the reset (before the fix: 1, 3,
4, and a stale 4), the same as the first recheck. Neither defect is present in the delivered
artifact.

## Regression Tests

The maintained regression tests selected in the specification ("Regression Test Strategy"). Each
command below was run exactly as shown, with the stable Rust toolchain (`rustc 1.99.0
(b940084d7 2026-09-28)`, `cargo 1.99.0 (5f94df478 2026-08-27)`), and each output block is verbatim.

### Red - R1, R5, and R6 Against the Unfixed Code (T4)

Code state: `develop` at `b5f1801c6` plus `docs(issues): [#2505] start the implementation`,
`test(udp-core): [#2505] add the per-event ban service benchmark`, and
`docs(issues): [#2505] record the performance baseline`, with the new tests added to the working
tree and no production change.

R1 (`udp-core` unit tests in `packages/udp-core/src/services/banning.rs`). Command:

```bash
cargo test -p torrust-tracker-udp-core --lib services::banning 2>&1 | grep -E "^test |panicked|left:|right:|banned IPs total|test result" | head -40
```

Output:

```text
test services::banning::tests::it_should_allow_resetting_all_the_counters ... ok
test services::banning::tests::it_should_ban_ips_with_counters_exceeding_a_predefined_limit ... ok
test services::banning::tests::it_should_count_an_ip_as_banned_once_its_errors_exceed_the_limit ... ok
test services::banning::tests::it_should_increase_the_errors_counter_for_a_given_ip ... ok
test services::banning::tests::it_should_not_ban_ips_whose_counters_do_not_exceed_the_predefined_limit ... ok
test services::banning::tests::it_should_not_ban_ips_without_connection_id_errors ... ok
test services::banning::tests::it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids ... ok
test services::banning::tests::it_should_not_count_an_ip_as_banned_while_its_errors_do_not_exceed_the_limit ... FAILED
test services::banning::tests::it_should_not_count_any_ip_as_banned_when_several_ips_stay_within_the_limit ... FAILED
thread 'services::banning::tests::it_should_not_count_an_ip_as_banned_while_its_errors_do_not_exceed_the_limit' (2511572) panicked at packages/udp-core/src/services/banning.rs:166:9:
assertion `left == right` failed: banned IPs total for one IP with 2 errors and a limit of 2
  left: 1
 right: 0
thread 'services::banning::tests::it_should_not_count_any_ip_as_banned_when_several_ips_stay_within_the_limit' (2511573) panicked at packages/udp-core/src/services/banning.rs:230:9:
assertion `left == right` failed: banned IPs total for 3 IPs with 1 error each and a limit of 1
  left: 3
 right: 0
test result: FAILED. 7 passed; 2 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
```

The two below-limit cases are red. The two over-limit cases (`limit + 1` and `limit + 2` errors)
pass on the unfixed code because a single tracked IP is counted either way; they guard the new
counter and are proven by mutate-then-restore after the fix (T5).

R5 and R6 (`udp-server` package-level integration tests, module
`server::contract::publishing_the_banned_ips_total` in `packages/udp-server/tests/server/contract.rs`,
limit 2). The first attempt did not compile (`error: unused import: logging`); after removing the
import, the command was:

```bash
cargo test -p torrust-tracker-udp-server --test integration publishing_the_banned_ips_total 2>&1 | grep -E "^error|^warning|^test |panicked|left:|right:|published banned|test result" | head -40
```

Output:

```text
test server::contract::publishing_the_banned_ips_total::with_strict_connection_id_validation::should_not_publish_a_client_as_banned_before_it_exceeds_the_connection_id_error_limit ... FAILED
test server::contract::publishing_the_banned_ips_total::with_disabled_connection_id_validation::should_publish_a_client_that_exceeds_the_connection_id_error_limit_as_would_be_banned ... ok
test server::contract::publishing_the_banned_ips_total::with_disabled_connection_id_validation::should_not_publish_a_client_as_banned_before_it_exceeds_the_connection_id_error_limit ... FAILED
test server::contract::publishing_the_banned_ips_total::with_strict_connection_id_validation::should_publish_a_client_as_banned_once_it_exceeds_the_connection_id_error_limit ... ok
thread 'server::contract::publishing_the_banned_ips_total::with_strict_connection_id_validation::should_not_publish_a_client_as_banned_before_it_exceeds_the_connection_id_error_limit' (2514046) panicked at packages/udp-server/tests/server/contract.rs:794:13:
assertion `left == right` failed: published banned IPs total after 2 invalid connection IDs from one client with a limit of 2 (strict validation)
  left: 1
 right: 0
thread 'server::contract::publishing_the_banned_ips_total::with_disabled_connection_id_validation::should_not_publish_a_client_as_banned_before_it_exceeds_the_connection_id_error_limit' (2514044) panicked at packages/udp-server/tests/server/contract.rs:855:13:
assertion `left == right` failed: published banned IPs total after 2 invalid connection IDs from one client with a limit of 2 (disabled validation)
  left: 1
 right: 0
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
error: test failed, to rerun pass `-p torrust-tracker-udp-server --test integration`
```

Stability check, because these tests wait for asynchronous event processing. Command:

```bash
for i in 1 2 3 4 5 6 7 8; do cargo test -q -p torrust-tracker-udp-server --test integration publishing_the_banned_ips_total 2>&1 | grep -E "^test result"; done
```

Output:

```text
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
```

The below-limit tests read the gauge after the listener has recorded every error. The listener
handles events one at a time and publishes after each, so at that point the gauge holds at least
the value published for the previous error (1 on the unfixed code), which makes the red result
deterministic. The over-limit tests poll the gauge until it reaches 1 or a 5-second deadline
passes; they pass on the unfixed code and guard the published value after the fix.

### Green - R1, R1p, R2, R5, and R6 After the `udp-core` Fix (T5)

Code state: the red state above plus the T5 change: `BanService` maintains a banned-IP counter
(option P1), `BanningStats` and `last_connection_id_errors_reset` are removed, R1p and R2 are
added, and the `udp-server` handler and listener tests are updated to the banned meaning. The same
commands as the red run, recorded in one capture (each `$` line is an `echo` of the command that
follows it):

```text
$ cargo test -p torrust-tracker-udp-core --lib services::banning 2>&1 | grep -E "^test |panicked|left:|right:|banned IPs total|test result" | head -40
test services::banning::tests::it_should_allow_resetting_all_the_counters ... ok
test services::banning::tests::it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids ... ok
test services::banning::tests::it_should_ban_ips_with_counters_exceeding_a_predefined_limit ... ok
test services::banning::tests::it_should_count_an_ip_as_banned_once_its_errors_exceed_the_limit ... ok
test services::banning::tests::it_should_increase_the_errors_counter_for_a_given_ip ... ok
test services::banning::tests::it_should_count_an_ip_banned_again_after_a_reset_only_once ... ok
test services::banning::tests::it_should_not_ban_ips_whose_counters_do_not_exceed_the_predefined_limit ... ok
test services::banning::tests::it_should_not_ban_ips_without_connection_id_errors ... ok
test services::banning::tests::it_should_not_count_an_ip_as_banned_while_its_errors_do_not_exceed_the_limit ... ok
test services::banning::tests::it_should_not_count_any_ip_as_banned_after_the_bans_are_reset ... ok
test services::banning::tests::it_should_not_count_any_ip_as_banned_when_several_ips_stay_within_the_limit ... ok
test services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets ... ok
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
$ cargo test -p torrust-tracker-udp-server --test integration publishing_the_banned_ips_total 2>&1 | grep -E "^error|^warning|^test |panicked|left:|right:|published banned|test result" | head -40
test server::contract::publishing_the_banned_ips_total::with_strict_connection_id_validation::should_not_publish_a_client_as_banned_before_it_exceeds_the_connection_id_error_limit ... ok
test server::contract::publishing_the_banned_ips_total::with_disabled_connection_id_validation::should_publish_a_client_that_exceeds_the_connection_id_error_limit_as_would_be_banned ... ok
test server::contract::publishing_the_banned_ips_total::with_strict_connection_id_validation::should_publish_a_client_as_banned_once_it_exceeds_the_connection_id_error_limit ... ok
test server::contract::publishing_the_banned_ips_total::with_disabled_connection_id_validation::should_not_publish_a_client_as_banned_before_it_exceeds_the_connection_id_error_limit ... ok
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
$ for i in 1 2 3 4 5 6 7 8; do cargo test -q -p torrust-tracker-udp-server --test integration publishing_the_banned_ips_total 2>&1 | grep -E "^test result"; done
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 12 filtered out; finished in 0.04s
```

The focused suites and the root banning tests also pass. Commands and their filtered output:

```text
$ cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server 2>&1 | grep -E "^error|^warning|FAILED|test result|panicked|Running" | head -40
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_udp_core-ccc5ed7cbc967af2)
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
     Running unittests src/lib.rs (target/debug/deps/torrust_tracker_udp_server-ef29f8560219f22b)
test result: ok. 219 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.18s
     Running tests/integration.rs (target/debug/deps/integration-d4d1a2addb4f912f)
test result: ok. 16 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.09s
test result: ok. 3 passed; 0 failed; 2 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
$ cargo test --test banning-udp-metrics-disabled-port-zero --test banning-udp-shared-connection-id-error-limit --test banning-udp-shared-connection-id-error-limit-reverse-order --test metrics-fixed-ports 2>&1 | grep -E "^error|FAILED|test result|panicked|Running" | head -20
     Running tests/banning/udp_metrics_disabled_port_zero.rs (target/debug/deps/banning_udp_metrics_disabled_port_zero-14dd79079fd283da)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.05s
     Running tests/banning/udp_shared_connection_id_error_limit.rs (target/debug/deps/banning_udp_shared_connection_id_error_limit-f49990584324f262)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.03s
     Running tests/banning/udp_shared_connection_id_error_limit_reverse_order.rs (target/debug/deps/banning_udp_shared_connection_id_error_limit_reverse_order-825a58919689be0f)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.04s
     Running tests/metrics/fixed_ports.rs (target/debug/deps/metrics_fixed_ports-57d64ba4989d3ffc)
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.06s
```

### Mutate-Then-Restore - Guards That Pass on the Unfixed Code (T5)

R1's over-limit cases, R2, R1p, and the R5/R6 over-limit tests pass on the unfixed code, so each
was proven against the bug variants a maintained counter can have. The fixed file was copied to
`.tmp/banning.rs.fixed`; each mutation was applied to `packages/udp-core/src/services/banning.rs`
without staging, and the copy was restored after each run (`cmp` confirmed the restored file is
identical to the fixed file).

Each mutation was applied by a shell function `mutate OLD NEW`, which ran a short Python snippet
that asserts `OLD` occurs exactly once in `banning.rs` and replaces it with `NEW`. The mutations:

| ID | Bug variant | `OLD` | `NEW` |
| -- | ----------- | ----- | ----- |
| M1 (first attempt) | count every error above the limit | `if !was_banned && *count > self.max_connection_id_errors_per_ip {` | `if *count > self.max_connection_id_errors_per_ip {` |
| M1 | count every error above the limit | `let was_banned = *count > self.max_connection_id_errors_per_ip;` | `let was_banned = false;` |
| M2 | reset does not clear the counter | the line `self.banned_ips_total = 0;` | (removed) |
| M3 | off by one: count an IP at the limit | `if !was_banned && *count > self.max_connection_id_errors_per_ip {` | `if !was_banned && *count >= self.max_connection_id_errors_per_ip {` |
| M4 | never count a banned IP | the line `self.banned_ips_total += 1;` | (removed) |

After each mutation, the `udp-core` tests ran as
`cargo test -q -p torrust-tracker-udp-core --lib services::banning 2>&1 | grep -E "^test .*FAILED|^test result|^    services" | sed 's/^    /FAILED: /' | sort -u`,
and for M4 the `udp-server` tests ran as
`cargo test -q -p torrust-tracker-udp-server 2>&1 | grep -E "^    |test result" | sed 's/^    /FAILED: /' | sort -u`.
Output of that first run:

```text
== M1: count every error above the limit (no crossing check)
== M2: reset does not clear the banned counter
FAILED: services::banning::tests::it_should_count_an_ip_banned_again_after_a_reset_only_once
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
FAILED: services::banning::tests::it_should_not_count_any_ip_as_banned_after_the_bans_are_reset
test result: FAILED. 9 passed; 3 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
== M3: off by one, count an IP at the limit
FAILED: services::banning::tests::it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids
FAILED: services::banning::tests::it_should_count_an_ip_as_banned_once_its_errors_exceed_the_limit
FAILED: services::banning::tests::it_should_count_an_ip_banned_again_after_a_reset_only_once
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
FAILED: services::banning::tests::it_should_not_count_an_ip_as_banned_while_its_errors_do_not_exceed_the_limit
FAILED: services::banning::tests::it_should_not_count_any_ip_as_banned_when_several_ips_stay_within_the_limit
test result: FAILED. 6 passed; 6 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
== M4: never count a banned IP
FAILED: services::banning::tests::it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids
FAILED: services::banning::tests::it_should_count_an_ip_as_banned_once_its_errors_exceed_the_limit
FAILED: services::banning::tests::it_should_count_an_ip_banned_again_after_a_reset_only_once
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
test result: FAILED. 8 passed; 4 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
FAILED: banning::event::handler::tests::it_should_publish_the_banned_ip_total_after_a_connection_cookie_error
test result: FAILED. 218 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.21s
restored: identical to the fixed file
```

That run was incomplete in two places. M1 printed nothing: removing `was_banned` from the condition
left the variable unused, which the workspace's `-D warnings` turns into a compile error, so no test
ran. M4's `udp-server` run stopped at the first failing test binary, so the integration tests did
not run. Both were repeated: M1 with the compiling variant in the table, with the `udp-core`
command filter `grep -E "^error|^    services|^test result"`, and M4 with
`cargo test -q --no-fail-fast -p torrust-tracker-udp-server 2>&1 | grep -E "^error|^    [a-z]|^test result" | sed 's/^    /FAILED: /' | sort -u`.
Output:

```text
== M1: count every error above the limit (crossing check disabled)
error: test failed, to rerun pass `-p torrust-tracker-udp-core --lib`
FAILED: services::banning::tests::it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
test result: FAILED. 10 passed; 2 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
== M4: never count a banned IP (udp-server, all test binaries)
error: 2 targets failed:
error: test failed, to rerun pass `-p torrust-tracker-udp-server --lib`
error: test failed, to rerun pass `-p torrust-tracker-udp-server --test integration`
FAILED: banning::event::handler::tests::it_should_publish_the_banned_ip_total_after_a_connection_cookie_error
FAILED: server::contract::publishing_the_banned_ips_total::with_disabled_connection_id_validation::should_publish_a_client_that_exceeds_the_connection_id_error_limit_as_would_be_banned
FAILED: server::contract::publishing_the_banned_ips_total::with_strict_connection_id_validation::should_publish_a_client_as_banned_once_it_exceeds_the_connection_id_error_limit
FAILED: server::contract::receiving_an_announce_request::should_ban_the_client_ip_if_it_sends_more_than_10_requests_with_a_cookie_value_not_normal
test result: FAILED. 13 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.10s
test result: FAILED. 218 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.21s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
restored: identical to the fixed file
```

Result: every guard fails for at least one plausible variant. R1p fails for all four; R2 for M2,
M3, and M4; R1's over-limit cases for M1, M3, and M4; the R5/R6 over-limit tests and the handler
test for M4.

### Test Design Review (T4, T5)

Prose-first Arrange-Act-Assert comparison, per the `write-unit-test` skill:

- R1 and R2 (`banning.rs`): Arrange states the causal condition as an error count relative to a
  visible limit (`record_connection_id_errors(..., limit + 1)`, with a reset in between for R2);
  Act is the production read `get_banned_ips_total()`; Assert compares with an independently stated
  number, and the message names the error count and the limit. `record_connection_id_errors` names
  the repeated `increase_counter` call and hides nothing causal.
- R1p: the expected value comes from an independent model (per-IP counts kept in an array), not
  from `is_banned`, and the failure message names the step, the operation, the limit, and the model
  counts.
- R5 and R6 (`contract.rs`): the causal values (`CONNECTION_ID_ERROR_LIMIT`, the policy, and the
  number of invalid connection IDs) stay in the test body; the helpers hide only server startup,
  request transport, and waiting for asynchronous event processing. Each test has one observable
  contract.
- Handler test: renamed to `it_should_publish_the_banned_ip_total_after_a_connection_cookie_error`;
  the causal state (one client already over the limit, the event client at the limit) is visible in
  `with_client_errors(&[...])`, and the expected total of 2 is stated independently. This record is
  superseded by the follow-up review of the three-client version below ("Reviewer Follow-Up -
  Handler Test Distinguishes the Meanings (N2)").
- Listener tests: they asserted the total only to observe that an event was processed; they now
  observe that directly with `get_count(&EVENT_CLIENT_IP)`.

The temporary prose was removed after the comparison; the code expresses it.

### Red - R4 Against the Unfixed Reset Job (T6)

Code state: `fix(udp-core): [#2505] count only IPs over the limit as banned` plus R4 in
`src/bootstrap/jobs/udp_tracker_server.rs` and one preparatory change: `run_ban_cleanup_job` takes
the stats repository as an unused `_stats_repository` parameter, so the test can pass it; the job
body still only calls `BanService::reset_bans`. R4 publishes a gauge of 1 through the real event
handler (two connection-cookie errors from one IP, limit 1), starts the job under a paused Tokio
clock with a 60-second interval, and sleeps one interval plus 1 ms. Command:

```bash
cargo test -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server 2>&1 | grep -E "^error|^warning|^test |panicked|left:|right:|published banned|test result"
```

Output:

```text
test bootstrap::jobs::udp_tracker_server::tests::it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans ... FAILED
test bootstrap::jobs::udp_tracker_server::tests::it_should_stop_the_ban_cleanup_job_when_cancelled ... ok
thread 'bootstrap::jobs::udp_tracker_server::tests::it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans' (2559588) panicked at src/bootstrap/jobs/udp_tracker_server.rs:187:9:
assertion `left == right` failed: published banned IPs total 60 s after the cleanup job started, when 203.0.113.1 had been banned and published as 1 before the first reset
  left: 1
 right: 0
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
error: test failed, to rerun pass `-p torrust-tracker --lib`
```

### Green - R3 and R4 After Publishing on Reset (T7)

Code state: the red state above plus option G1: `torrust_tracker_udp_server::banning::reset_bans`
resets the ban service and publishes the new total through the event handler's
`update_metric_for_banned_ips_total`; the job calls it with `CurrentClock::now()` on every tick; R3
tests the new function in `packages/udp-server/src/banning/mod.rs`. Commands:

```bash
cargo test -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server 2>&1 | grep -E "^error|^warning|^test |panicked|left:|right:|published banned|test result" -A3 | head -20
cargo test -p torrust-tracker-udp-server --lib banning 2>&1 | grep -E "^error|^warning|^test |FAILED|test result" | head -20
```

Output:

```text
test bootstrap::jobs::udp_tracker_server::tests::it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans ... ok
test bootstrap::jobs::udp_tracker_server::tests::it_should_stop_the_ban_cleanup_job_when_cancelled ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s

test banning::event::listener::tests::it_should_ignore_non_connection_cookie_errors ... ok
test banning::event::handler::tests::it_should_publish_the_banned_ip_total_after_a_connection_cookie_error ... ok
test banning::event::listener::tests::it_should_stop_when_the_receiver_is_closed ... ok
test banning::event::listener::tests::it_should_prioritize_cancellation_over_a_ready_event ... ok
test banning::event::handler::tests::it_should_record_the_connection_cookie_error_for_its_client_ip ... ok
test banning::event::listener::tests::it_should_process_connection_cookie_errors_after_lagging ... ok
test banning::tests::it_should_publish_the_new_banned_ips_total_after_resetting_the_bans ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 213 filtered out; finished in 0.00s
```

R4 passing with the same timing also shows that the red result came from the missing publish,
not from the job not having run yet.

### Mutate-Then-Restore - R3 (T7)

M5 removes the publish from `reset_bans` (the call
`event::handler::update_metric_for_banned_ips_total(repository, ips_banned_total, now).await;` is
replaced by `let _ = (repository, ips_banned_total, now);`). The fixed file was copied to
`.tmp/banning-mod.rs.fixed` and restored afterwards. Commands:

```bash
f=packages/udp-server/src/banning/mod.rs; cp $f .tmp/banning-mod.rs.fixed
python3 -c "
p='$f'; s=open(p).read()
old='    event::handler::update_metric_for_banned_ips_total(repository, ips_banned_total, now).await;\n'
assert s.count(old)==1; open(p,'w').write(s.replace(old,'    let _ = (repository, ips_banned_total, now);\n'))"
echo "== M5: reset_bans does not publish the new total"
cargo test -q -p torrust-tracker-udp-server --lib banning 2>&1 | grep -E "^error|^    banning|^test result" | sed 's/^    /FAILED: /' | sort -u
cargo test -q -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server 2>&1 | grep -E "^error|^    bootstrap|^test result" | sed 's/^    /FAILED: /' | sort -u
cp .tmp/banning-mod.rs.fixed $f; cmp $f .tmp/banning-mod.rs.fixed && echo "restored: identical to the fixed file"
echo "== R4 stability"
for i in 1 2 3 4 5 6 7 8; do cargo test -q -p torrust-tracker --lib bootstrap::jobs::udp_tracker_server 2>&1 | grep -E "^test result"; done
```

Output:

```text
== M5: reset_bans does not publish the new total
error: test failed, to rerun pass `-p torrust-tracker-udp-server --lib`
FAILED: banning::tests::it_should_publish_the_new_banned_ips_total_after_resetting_the_bans
test result: FAILED. 6 passed; 1 failed; 0 ignored; 0 measured; 213 filtered out; finished in 0.00s
error: test failed, to rerun pass `-p torrust-tracker --lib`
FAILED: bootstrap::jobs::udp_tracker_server::tests::it_should_publish_the_new_banned_ips_total_when_it_resets_the_bans
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
restored: identical to the fixed file
== R4 stability
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 114 filtered out; finished in 0.00s
```

Test design review (prose-first AAA): R3 and R4 keep the causal state visible (the limit, the
banned client IP, and the events that publish 1), the production Act (`reset_bans` for R3; one
reset interval of the job for R4, with a comment saying the paused clock jumps to it), and an
independently stated expected total of 0 whose message names the interval and the banned IP.

### Mutate-Then-Restore - Variant B of the Counter (T9 Follow-Up)

After the T9 investigation (see `performance-evidence.md`, "P2 Investigation"), the maintainer
adopted variant B of the crossing check, which compares once after the increment:
`if Some(*count) == self.max_connection_id_errors_per_ip.checked_add(1)`. M1 and M3 no longer apply
in their original form, so all four variants were rerun against B. The fixed file was copied to
`.tmp/banning.rs.fixed-b` and restored after each run. Command:

```bash
f=packages/udp-core/src/services/banning.rs; cp $f .tmp/banning.rs.fixed-b
for m in \
 "M1b: count every error above the limit|if Some(*count) == self.max_connection_id_errors_per_ip.checked_add(1) {|if Some(*count) > Some(self.max_connection_id_errors_per_ip) {" \
 "M2: reset does not clear the banned counter|        self.banned_ips_total = 0;
|" \
 "M3b: off by one, count an IP at the limit|self.max_connection_id_errors_per_ip.checked_add(1) {|self.max_connection_id_errors_per_ip.checked_add(0) {" \
 "M4: never count a banned IP|            self.banned_ips_total += 1;
|"; do
  name="${m%%|*}"; rest="${m#*|}"; old="${rest%%|*}"; new="${rest#*|}"
  echo "== $name"
  OLD="$old" NEW="$new" python3 -c "import os; p='$f'; s=open(p).read(); o=os.environ['OLD']; assert s.count(o)==1, o; open(p,'w').write(s.replace(o, os.environ['NEW']))"
  cargo test -q -p torrust-tracker-udp-core --lib services::banning 2>&1 | grep -E "^error|^    services|^test result" | sed 's/^    /FAILED: /' | sort -u
  cp .tmp/banning.rs.fixed-b $f
done
cmp $f .tmp/banning.rs.fixed-b && echo "restored: identical to variant B"
```

Output:

```text
== M1b: count every error above the limit
error: test failed, to rerun pass `-p torrust-tracker-udp-core --lib`
FAILED: services::banning::tests::it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
test result: FAILED. 10 passed; 2 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
== M2: reset does not clear the banned counter
error: test failed, to rerun pass `-p torrust-tracker-udp-core --lib`
FAILED: services::banning::tests::it_should_count_an_ip_banned_again_after_a_reset_only_once
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
FAILED: services::banning::tests::it_should_not_count_any_ip_as_banned_after_the_bans_are_reset
test result: FAILED. 9 passed; 3 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
== M3b: off by one, count an IP at the limit
error: test failed, to rerun pass `-p torrust-tracker-udp-core --lib`
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
FAILED: services::banning::tests::it_should_not_count_an_ip_as_banned_while_its_errors_do_not_exceed_the_limit
FAILED: services::banning::tests::it_should_not_count_any_ip_as_banned_when_several_ips_stay_within_the_limit
test result: FAILED. 9 passed; 3 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
== M4: never count a banned IP
error: test failed, to rerun pass `-p torrust-tracker-udp-core --lib`
FAILED: services::banning::tests::it_should_count_a_banned_ip_only_once_when_it_keeps_sending_invalid_connection_ids
FAILED: services::banning::tests::it_should_count_an_ip_as_banned_once_its_errors_exceed_the_limit
FAILED: services::banning::tests::it_should_count_an_ip_banned_again_after_a_reset_only_once
FAILED: services::banning::tests::it_should_keep_the_banned_ips_total_equal_to_the_ips_above_the_limit_for_any_errors_and_resets
test result: FAILED. 8 passed; 4 failed; 0 ignored; 0 measured; 37 filtered out; finished in 0.00s
restored: identical to variant B
```

Every variant is caught by at least two tests, and R1p catches all four. With variant B the
`udp-core`, `udp-server`, root reset-job, and root banning test suites pass, and `cargo clippy -p
torrust-tracker-udp-core --all-targets -- -D clippy::pedantic` reports nothing.

### Reviewer Follow-Up - Handler Test Distinguishes the Meanings (N2)

The independent pre-PR review (see `agent-review-reports.md`) noted that
`it_should_publish_the_banned_ip_total_after_a_connection_cookie_error` passed with the old
distinct-IP total too, because both tracked IPs were over the limit. A third client at the limit
(`client_within_the_limit_ip`, 1 error with a limit of 1; renamed `client_at_the_limit_ip` after the
follow-up review, finding N9) now makes the old meaning publish 3 instead of 2. Proof with the
original map-length total (`self.accurate_error_counter.len()`) temporarily restored in `packages/udp-core/src/services/banning.rs`, without staging, and the file
restored from a copy afterwards. Command:

```bash
f=packages/udp-core/src/services/banning.rs; cp $f .tmp/banning.rs.head
OLD='        self.banned_ips_total
    }' NEW='        self.accurate_error_counter.len()
    }' python3 -c "import os; p='$f'; s=open(p).read(); o=os.environ['OLD']; assert s.count(o)==1; open(p,'w').write(s.replace(o, os.environ['NEW']).replace('pub const fn get_banned_ips_total','pub fn get_banned_ips_total'))"
echo "== M0: the original map-length total"; cargo test -q -p torrust-tracker-udp-server --lib banning::event::handler 2>&1 | grep -E "^error|^    banning|left:|right:|^test result" | sed 's/^    banning/FAILED: banning/'
cp .tmp/banning.rs.head $f; cmp $f .tmp/banning.rs.head && echo "restored"
```

Output:

```text
== M0: the original map-length total
  left: 3
 right: 2
FAILED: banning::event::handler::tests::it_should_publish_the_banned_ip_total_after_a_connection_cookie_error
test result: FAILED. 1 passed; 1 failed; 0 ignored; 0 measured; 218 filtered out; finished in 0.00s
error: test failed, to rerun pass `-p torrust-tracker-udp-server --lib`
restored
```

With the fix in place the handler tests pass
(`test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 218 filtered out`).

Test design review of the three-client version (prose-first Arrange-Act-Assert, per the
`write-unit-test` skill; finding B2 of the follow-up review):

- Arrange (prose): with a limit of 1, one client is already banned (2 errors), the event's client
  is at the limit (1 error), and a third client is at the limit too (1 error) and is not part of
  the event. Code: `with_client_errors(&[...])` lists each client with its error count written
  relative to `CONNECTION_ID_ERROR_LIMIT`, and `expected_banned_ip_total = 2` is stated before the
  Act, independent of the code under test.
- Act (prose): the handler processes one connection-cookie error from the event's client. Code:
  the single visible `handle_event(event, ...)` call, with the event built from
  `cookie_error_client_ip`.
- Assert (prose): the published total is 2, the already-banned client plus the event's client,
  which this error takes over the limit; the client at the limit is not counted. Code: one
  `assert_eq!` on `udp_banned_ips_total()` whose message names all three clients and the limit.

The code expresses each paragraph, so the temporary prose was removed. The variable for the third
client was renamed `client_at_the_limit_ip` so its name says the same as the prose (N9); the
renamed test passes (`cargo test -q -p torrust-tracker-udp-server --lib banning`:
`test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 213 filtered out`).

## Failures and Follow-up

- First attempt (tracker started at 15:07:57 UTC) used `socat` to send the probes. Each probe
  produced an extra "Couldn't parse action" error response besides the expected cookie error,
  because `socat` sent an extra datagram. That error is not a connection-cookie error, so it does
  not change the ban counter (the reported value was 1 after one probe, as in V1), but it made the
  evidence noisy. The probe was switched to OpenBSD netcat, the tracker was restarted with an empty
  database at 15:09:06 UTC, and all processes above come from that clean run.
- Final recheck, first start (2026-10-14 06:52:12 UTC): F-V1 ran correctly, but the V2 and V3
  commands were issued in the same batch of agent tool calls and ran concurrently (both started at
  06:52:27 UTC), so V3 did not start from the state V2 leaves. The results were consistent
  (V2 read 0, V3 read 1), but the run was not like-for-like. The tracker was stopped, its log kept
  locally as `.tmp/udp-banned-ips-metric/repro/aborted-interleaved-tracker.log`, and the recheck
  restarted with an empty database at 06:53:20 UTC, running each step only after the previous one
  finished.
