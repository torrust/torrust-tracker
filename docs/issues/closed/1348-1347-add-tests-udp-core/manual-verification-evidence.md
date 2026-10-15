---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/1348-1347-add-tests-udp-core/ISSUE.md
last-updated-utc: 2026-10-15 16:30
---

# Manual Verification Evidence

## Purpose

Real, human-oriented verification of the `udp-core` behaviors that Issue #1348's tests protect:
announce and scrape through a running tracker, connection-cookie rejection, and the per-IP ban
threshold. Every command and output below was actually run and copied from the terminal.

## Preserving Verification Artifacts

- The tracker configuration is embedded below.
- The M3/M4 client is committed in this folder:
  [manual-verification/invalid-connection-id/](manual-verification/invalid-connection-id/src/main.rs).
- The run wrote its log, database, and client build to the git-ignored `.tmp/1348-manual/`; the
  relevant output is recorded inline.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-14; tracker started at 16:13:59 UTC.
- Artifact under test: `torrust-tracker` built from branch `1348-1347-add-tests-udp-core` at the
  commit "docs(udp-core): record the final coverage and reconcile the #1348 records", rebased onto
  `develop` at `8045bd252` (`Merge torrust/torrust-tracker#2524`). No uncommitted Rust changes.
- Toolchain: stable Rust, `rustc 1.99.0 (b940084d7 2026-09-28)`, `cargo 1.99.0`; `dev` profile.
- Operating system: Ubuntu 26.04.1 LTS, Linux 7.0.0-38-generic.
- Network: loopback only. Linux accepts any `127.0.0.0/8` source address, so the client binds
  `127.0.0.2` and `127.0.0.3` to act as separate clients.
- Ports: UDP tracker `26969`, REST API `127.0.0.1:21212`, health check `127.0.0.1:21313`.

### Tracker configuration

Saved as `.tmp/1348-manual/tracker.toml`:

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
path = "./.tmp/1348-manual/storage/sqlite3.db"

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 3600
max_connection_id_errors_per_ip = 10
connection_id_validation = "strict"

[[udp_trackers]]
bind_address = "0.0.0.0:26969"
tracker_usage_statistics = true

[http_api]
bind_address = "127.0.0.1:21212"

[http_api.access_tokens]
admin = "MyAccessToken"

[health_check_api]
bind_address = "127.0.0.1:21313"
```

### Setup commands

```bash
cargo build --bin torrust-tracker
cargo build -p torrust-tracker-client --bin tracker_client
mkdir -p .tmp/1348-manual/storage
./target/debug/torrust-tracker --config-toml-path ./.tmp/1348-manual/tracker.toml > .tmp/1348-manual/tracker.log 2>&1 &
```

Relevant startup log lines:

```text
2026-10-14T16:13:59.905496Z  INFO UDP TRACKER: Starting UDP IP-ban cleanup job reset_interval_in_secs=3600
... UDP TRACKER: Started on: udp://0.0.0.0:26969
```

The M3/M4 client was built outside the workspace:

```bash
cd docs/issues/closed/1348-1347-add-tests-udp-core/manual-verification/invalid-connection-id
CARGO_TARGET_DIR=<repo>/.tmp/1348-manual/client-target cargo build
```

## Verification Processes

### V1 - UDP Announce (M1)

- Goal: a valid announce through the running tracker succeeds.
- Initial state: fresh tracker, empty database.
- Status: `DONE`

#### Steps Performed

```bash
./target/debug/tracker_client udp announce udp://127.0.0.1:26969 9c38422213e30bff212b30c360d26f9a02136422
```

#### Observed Result

```text
{"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
```

#### Conclusion

Met: a valid announce response; the client is recorded as the torrent's only seeder.

### V2 - UDP Scrape (M2)

- Goal: a scrape for the announced hash returns that torrent.
- Initial state: after V1.
- Status: `DONE`

#### Steps Performed

```bash
./target/debug/tracker_client udp scrape udp://127.0.0.1:26969 9c38422213e30bff212b30c360d26f9a02136422
```

#### Observed Result

```text
{"Scrape":{"transaction_id":-888840697,"torrent_stats":[{"seeders":1,"completed":0,"leechers":0}]}}
```

#### Conclusion

Met: the scrape returns the announced torrent with one seeder.

### V3 - Invalid Connection ID (M3)

- Goal: with `connection_id_validation = "strict"`, an announce and a scrape carrying an invalid
  connection ID (zero) each get a UDP error response naming the cookie failure.
- Initial state: after V2; `127.0.0.1` has no connection ID errors.
- Status: `DONE`

#### Steps Performed

```bash
C=./.tmp/1348-manual/client-target/debug/invalid-connection-id-client
$C invalid-announce 127.0.0.1:26969 127.0.0.1
$C invalid-scrape 127.0.0.1:26969 127.0.0.1
```

#### Observed Result

```text
request 1 from 127.0.0.1: Error(ErrorResponse { transaction_id: TransactionId(I32(1)), message: "tracker announce error: Connection cookie error: cookie value is expired: -50173409471191.87, expected > 1791994930.8485813" })
request 1 from 127.0.0.1: Error(ErrorResponse { transaction_id: TransactionId(I32(1)), message: "tracker scrape error: Connection cookie error: cookie value is expired: 0.000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002564322468380983, expected > 1791994930.853297" })
```

#### Conclusion

Met: both requests are rejected with an error response that names the connection cookie failure
and keeps the request's transaction ID. The `expected >` wording is the off-by-one message already
recorded in [lessons.md](lessons.md) (`check` accepts the boundary).

### V4 - Ban After Repeated Invalid IDs (M4)

- Goal: with `max_connection_id_errors_per_ip = 10`, an IP gets error responses until it passes the
  limit, then the tracker stops answering it; other IPs are unaffected.
- Initial state: after V3; `127.0.0.2` and `127.0.0.3` have no errors.
- Status: `DONE`

#### Steps Performed

```bash
$C invalid-announce 127.0.0.1:26969 127.0.0.2 12 | cut -c1-110
$C connect 127.0.0.1:26969 127.0.0.2
$C connect 127.0.0.1:26969 127.0.0.3
curl -s "http://127.0.0.1:21212/api/v1/stats?token=MyAccessToken" | jq '{udp_banned_ips_total, udp_requests_banned}'
```

#### Observed Result

Run at 2026-10-14 16:24:48 UTC. The first command's lines are cut at 110 characters by `cut`; the
full message matches V3's announce error.

<!-- cspell:ignore annou -->

```text
request 1 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(1)), message: "tracker annou
request 2 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(2)), message: "tracker annou
request 3 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(3)), message: "tracker annou
request 4 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(4)), message: "tracker annou
request 5 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(5)), message: "tracker annou
request 6 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(6)), message: "tracker annou
request 7 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(7)), message: "tracker annou
request 8 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(8)), message: "tracker annou
request 9 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(9)), message: "tracker annou
request 10 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(10)), message: "tracker ann
request 11 from 127.0.0.2: Error(ErrorResponse { transaction_id: TransactionId(I32(11)), message: "tracker ann
request 12 from 127.0.0.2: no response (Timeout while trying to receive data.)
request 1 from 127.0.0.2: no response (Timeout while waiting for the socket to become readable.)
request 1 from 127.0.0.3: Connect(ConnectResponse { transaction_id: TransactionId(I32(1)), connection_id: ConnectionId(I64(6622740845759958365)) })
{
  "udp_banned_ips_total": 1,
  "udp_requests_banned": 2
}
```

#### Conclusion

Met. Requests 1-11 get error responses (the 11th error is the one that exceeds the limit of 10).
From then on `127.0.0.2` is banned: its 12th invalid announce and a valid connect get no response,
counted as `udp_requests_banned: 2`. The control IP `127.0.0.3` still gets a connect response.
`udp_banned_ips_total` is 1: `127.0.0.1`, with V3's two errors, is not counted, which matches the
fix in #2505.

The tracker was then stopped (process killed); port `26969` was confirmed free.

## Failures and Follow-up

None. All four scenarios passed on the first run.
