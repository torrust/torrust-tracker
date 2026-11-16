---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2850-non-negative-number-of-bytes/ISSUE.md
last-updated-utc: "2026-11-16 15:50"
---

# Manual Verification Evidence

<!-- cspell:ignore EPOCHREALTIME peerslee -->

## Purpose

Real use of a running tracker for the scenarios M1 to M7 of the issue specification: UDP and HTTP
announces with negative, zero, positive and very large byte counts, and the REST API output of the
announced peer.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-15 21:01:41 to 21:01:42 (run 2; see "Failures and Follow-up" for
  run 1).
- Artifact under test: the branch of issue #2850 at
  `docs(axum-http-server): [#2850] describe the byte-count params as non-negative`, which follows
  every production change (`fix(udp-core)!: [#2850] refuse a negative byte count in the
  wire-to-domain conversion` and `refactor(primitives)!: [#2850] hold the number of bytes in a
  u64`). `git status --short` showed only the new `evidence/` folder before the build. Debug build:
  `cargo build --bin torrust-tracker`, stable Rust toolchain (`rustc 1.99.1`).
- Operating system: Linux 7.0.0-43-generic.
- Tracker configuration: [`evidence/tracker.toml`](evidence/tracker.toml): UDP on
  `127.0.0.1:6868`, HTTP on `127.0.0.1:7070`, REST API on `127.0.0.1:1212` with the development
  token, an SQLite database in the run folder, `info` logging. Before the run, no socket listened
  on these ports or on the health check API's default `127.0.0.1:1313`.
- UDP announces: the probe of issue #2838,
  [`udp_announce_probe.py`](../2838-1347-udp-reject-negative-announce-bytes/evidence/udp_announce_probe.py),
  run from this branch (arguments `<port> <left> [uploaded] [downloaded]`; info hash `0x11` × 20,
  peer port 6881, event `started`). HTTP announces: `curl` with info hash `0x22` × 20, peer port
  6882, event `started`. Each protocol uses its own info hash, so the UDP and HTTP peers never
  share a swarm, and each accepted announce replaces the peer of the previous one.
- REST output: `GET /api/v1/torrent/<info hash>` right after each accepted announce and before the
  next announce.

### Run Script

The steps run one after another from this disposable script (in the session scratchpad, not in the
repository), each stamped with `date -u` and bash's `$EPOCHREALTIME`:

```bash
#!/bin/bash
# Disposable: runs M1-M7 of issue #2850 one step at a time against a fresh tracker.
set -u
repo="$1"; run="$2"
probe="$repo/docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/evidence/udp_announce_probe.py"
cfg="$repo/docs/issues/open/2850-non-negative-number-of-bytes/evidence/tracker.toml"
rm -rf "$run"; mkdir -p "$run"; cd "$run"
TORRUST_TRACKER_CONFIG_TOML_PATH="$cfg" "$repo/target/debug/torrust-tracker" > "$run/tracker.log" 2>&1 &
tracker_pid=$!
sleep 4
ts() { echo "$(date -u +%T.%3N) (epoch $EPOCHREALTIME)"; }
step() { echo "$(ts) start: $*"; "$@"; echo; echo "$(ts) end"; }
udp_hash=1111111111111111111111111111111111111111
http_hash=2222222222222222222222222222222222222222
rest() { curl -s "http://127.0.0.1:1212/api/v1/torrent/$1?token=MyAccessToken"; }
http() { curl -s "http://127.0.0.1:7070/announce?info_hash=%22%22%22%22%22%22%22%22%22%22%22%22%22%22%22%22%22%22%22%22&peer_id=-XX0002-000000000000&port=6882&uploaded=$2&downloaded=$3&left=$1&event=started&compact=0"; }
echo "== M1 UDP left=-5"
step python3 -I "$probe" 6868 -5
step rest $udp_hash
echo "== M1 UDP uploaded=-1, downloaded=-1"
step python3 -I "$probe" 6868 0 -1 0
step python3 -I "$probe" 6868 0 0 -1
echo "== M2 UDP left=0"
step python3 -I "$probe" 6868 0 7 9
step rest $udp_hash
echo "== M3 UDP left=1000"
step python3 -I "$probe" 6868 1000 7 9
step rest $udp_hash
echo "== M3 UDP left=i64::MAX"
step python3 -I "$probe" 6868 9223372036854775807 7 9
step rest $udp_hash
echo "== M4 HTTP left=-5"
step http -5 7 9
echo "== M4 HTTP left=0"
step http 0 7 9
step rest $http_hash
echo "== M4 HTTP left=1000"
step http 1000 7 9
step rest $http_hash
echo "== M4 HTTP left=i64::MAX"
step http 9223372036854775807 7 9
step rest $http_hash
echo "== M5 HTTP left=i64::MAX+1"
step http 9223372036854775808 7 9
step rest $http_hash
echo "== M6 HTTP left=u64::MAX+1"
step http 18446744073709551616 7 9
step rest $http_hash
echo "== M6b HTTP uploaded=u64::MAX, downloaded=u64::MAX"
step http 1000 18446744073709551615 18446744073709551615
step rest $http_hash
kill -TERM $tracker_pid; wait $tracker_pid
echo "tracker exit=$?"
```

The full output, with the repository path removed, is
[`evidence/manual-run-output.txt`](evidence/manual-run-output.txt). The tracker exited with code 0
after `SIGTERM`.

## Verification Processes

### V1 - M1: UDP, negative counters

- Status: `DONE`
- Steps: probe with `left = -5`, then the REST detail of the UDP info hash; probe with
  `uploaded = -1`; probe with `downloaded = -1`.
- Observed:

  ```text
  21:01:41.525 start: probe 6868 -5
  action=3 message='tracker announce error: Invalid announce request: `left` must not be negative, got -5'
  21:01:41.540 start: rest 1111111111111111111111111111111111111111
  "torrent not known"
  21:01:41.551 start: probe 6868 0 -1 0
  action=3 message='tracker announce error: Invalid announce request: `uploaded` must not be negative, got -1'
  21:01:41.567 start: probe 6868 0 0 -1
  action=3 message='tracker announce error: Invalid announce request: `downloaded` must not be negative, got -1'
  ```

  The probe exits with status 1 when a response does not carry its request's transaction id
  (4321), so each error response carried it. The tracker log has one `ERROR` line per rejection
  (`grep -c "must not be negative"` printed `3`); the three lines, without colour codes, are in
  [`evidence/tracker-log-rejections.txt`](evidence/tracker-log-rejections.txt). They show
  `error=AnnounceFailed { source: InvalidAnnounceRequest { source: NegativeNumberOfBytes { field: Left, value: -5 } } }`
  and `transaction_id=4321`.
- Conclusion: the negative counter gets a BEP 15 error response (action 3) with the transaction id;
  the swarm is not created, so no peer is stored and `complete` does not grow. The symptom of
  #2838 is gone. AC12.

### V2 - M2: UDP, `left = 0`

- Status: `DONE`
- Observed: `announce response: interval=120 leechers=0 seeders=1`; REST: `"seeders":1`, the peer
  with `"uploaded":7,"downloaded":9,"left":0`.
- Conclusion: a seeder, as before.

### V3 - M3: UDP, positive `left`

- Status: `DONE`
- Observed: `left = 1000`: `leechers=1 seeders=0`, REST `"left":1000`.
  `left = 9223372036854775807`: `leechers=1 seeders=0`, REST `"left":9223372036854775807`.
- Conclusion: leechers, including the value Transmission sends when the size is unknown.

### V4 - M4: HTTP, the same values

- Status: `DONE`
- Observed:

  ```text
  left=-5:   d14:failure reason153:Bad request. Cannot parse query params for announce request: invalid param value -5 for left in packages/http-protocol/src/v1/requests/announce.rs:593:27e
  left=0:    d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peerslee
  left=1000: d8:completei0e10:incompletei1e8:intervali120e12:min intervali120e5:peerslee
  left=9223372036854775807: d8:completei0e10:incompletei1e8:intervali120e12:min intervali120e5:peerslee
  ```

- Conclusion: refused, seeder, leecher, leecher: the same verdicts as V1 to V3. AC13.

### V5 - M5: HTTP, above `i64::MAX`

- Status: `DONE`
- Observed: `left = 9223372036854775808`:
  `d8:completei0e10:incompletei1e8:intervali120e12:min intervali120e5:peerslee`; REST
  `"left":9223372036854775808`.
- Conclusion: accepted as a leecher; before this issue it was refused with `NumberOfBytesOverflow`.
  AC10.

### V6 - M6: HTTP, above `u64::MAX`

- Status: `DONE`
- Observed: `left = 18446744073709551616`:
  `d14:failure reason171:Bad request. Cannot parse query params for announce request: invalid param value 18446744073709551616 for left in packages/http-protocol/src/v1/requests/announce.rs:593:27e`.
  The REST detail after it still shows the peer of V5 (`"left":9223372036854775808`), so the
  refused announce changed nothing.
- Additional step (M6b): `uploaded = downloaded = 18446744073709551615`, `left = 1000`: accepted;
  REST `"uploaded":18446744073709551615,"downloaded":18446744073709551615,"left":1000`.
- Conclusion: a bencoded `failure reason`, as expected; `u64::MAX` is accepted for every counter.
  AC8.

### V7 - M7: REST output

- Status: `DONE`
- Observed: after every accepted announce of V2 to V6, the single-torrent detail returned
  `left`, `uploaded` and `downloaded` as JSON numbers equal to the values just sent, including
  `9223372036854775808` and `18446744073709551615` (above `i64::MAX`). The JSON field names and
  shapes are the ones of the previous `i64` fields. Full responses in
  [`evidence/manual-run-output.txt`](evidence/manual-run-output.txt).
- Conclusion: AC11. Values above 2^53 lose precision in JavaScript clients that parse them as
  numbers; that already held for large `i64` values (see the specification, "Risks and
  Trade-offs").

## Regression Tests: Red and Green Runs

The fix-bug process records the failing run of the maintained regression test before the fix and
the passing runs after it. Both runs were made on 2026-11-16 with `rustc 1.99.1 (58cf8f9c3
2026-10-13)` and `cargo 1.99.1`, debug test profile.

### Red: the handler test before the fix

- Commit: `test(udp-server): [#2850] reject a negative left in an announce (red)`, checked out in
  its own detached `git worktree` (no local edits), the commit right before
  `fix(udp-core)!: [#2850] refuse a negative byte count in the wire-to-domain conversion`.
- Command:

  ```sh
  cargo test -p torrust-tracker-udp-server --lib it_should_reject_an_announce_with_a_negative_number_of_bytes_left
  ```

- Output (08:16:06 to 08:16:41 UTC; excerpt in
  [`evidence/regression-red-run.txt`](evidence/regression-red-run.txt): cargo's output without its
  `Compiling` and `Blocking` lines, from `Finished` to the exit code and the end stamp; the command,
  toolchain and start stamp are the ones above): the test `FAILED` with
  exit code 101, and the panic shows the bug of #2838, a negative `left` counted as a seeder:

  ```text
  a negative `left` must be rejected, got: Ok(AnnounceIpv4(AnnounceResponse { fixed: AnnounceResponseFixedData { transaction_id: TransactionId(I32(0)), announce_interval: AnnounceInterval(I32(120)), leechers: NumberOfPeers(I32(0)), seeders: NumberOfPeers(I32(1)) }, peers: [] }))
  ```

- The fix commit then also tightened this test to match the exact error, the transaction id and
  the request kind; that final form is the one of the green run.

### Green: the regression tests after the fix

- Commit: the branch head with
  `docs(issues): [#2850] drop the removed overflow variant from the SI-3 draft` as its last
  commit, which includes every production change.
- Commands and output (08:16:11 to 08:16:38 UTC; excerpt in
  [`evidence/regression-green-run.txt`](evidence/regression-green-run.txt): the toolchain, the start
  and end stamps, each command, and only its `running`, `test` and `test result` lines), each `ok`:

  | Command | Result |
  | --- | --- |
  | `cargo test -p torrust-tracker-udp-server --lib it_should_reject_an_announce_with_a_negative_number_of_bytes_left` | 1 passed |
  | `cargo test -p torrust-tracker-udp-server --lib it_should_classify_a_negative_number_of_bytes_in_an_announce_as_a_bad_request` | 1 passed |
  | `cargo test -p torrust-tracker-udp-core --lib number_of_bytes` | 9 passed (the `peer_builder` cases and the two check-order tests) |
  | `cargo test -p torrust-tracker-udp-core --lib it_should_validate_the_connection_id_before_the_number_of_bytes` | 1 passed |
  | `cargo test -p torrust-tracker-udp-core --lib it_should_reject_a_negative_number_of_bytes_before_revealing_whether_the_torrent_is_whitelisted` | 1 passed |
  | `cargo test -p torrust-tracker-primitives --lib peer` | 43 passed (including the seeder and role-table tests) |
  | `cargo test -p torrust-tracker-http-protocol --lib when_a_number_of_bytes_param_is_parsed` | 15 passed |
  | `cargo test -p torrust-tracker-axum-http-server --test integration should_accept_a_number_of_bytes_left_above_the_largest_signed_64_bit_value` | 1 passed |

## Failures and Follow-up

- Run 1 (21:01:25 to 21:01:26 UTC), with the same script but only `date -u +%T.%3N` stamps, gave
  the same responses (the outputs of both runs are equal once the `updated*` fields and the stamps
  are removed), but its last stamps were not monotonic (`21:01:26.941 end`, then
  `21:01:26.111 start`), and the tracker's `updated_at_ms` of the last announce
  (`1794776486014`, 21:01:26.014) was earlier than the stamp of the step that sent it. The
  steps ran sequentially in one bash script, so the order is not in doubt, but the record could
  not show it. The cause is not known; the system clock is NTP-synchronized. Run 1 is discarded,
  and run 2 adds bash's `$EPOCHREALTIME` next to each stamp; its stamps are monotonic.
