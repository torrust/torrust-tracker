---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/ISSUE.md
last-updated-utc: 2026-11-12 16:40
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-11 17:28-17:30
- Artifact under test: `develop` plus the `test(udp-server): reject a negative left in an announce (red)` patch
  (no production change); debug build of `torrust-tracker` (`cargo build --bin torrust-tracker`).
- Operating system: Linux 7.0.0-43-generic; stable Rust toolchain, rustc 1.99.1.
- Setup: tracker config [`evidence/tracker.toml`](evidence/tracker.toml) (the shipped
  `tracker.development.sqlite3.toml` with a temp database, HTTP/API removed, UDP on
  `127.0.0.1:6868`); probe [`evidence/udp_announce_probe.py`](evidence/udp_announce_probe.py).

## Verification Processes

### V1 - Negative `left` over UDP (before the fix)

- Goal: observe whether the UDP tracker accepts `left = -5` and counts the peer as a seeder.
- Initial state: fresh tracker, empty database.
- Status: `DONE`
- Outcome: **Reproduced** (the wrong outcome itself was observed).

#### Steps Performed

1. `TORRUST_TRACKER_CONFIG_TOML_PATH=<tracker.toml> ./target/debug/torrust-tracker`
2. For `left` in `1000`, `-5`, `0`:
   `python3 -I evidence/udp_announce_probe.py 6868 <left>`
   (same info hash, same peer id, `event = started`, so each announce replaces the peer).

#### Observed Result

Full output: [`evidence/probe-before-fix.txt`](evidence/probe-before-fix.txt).

```text
left=1000
announce response: interval=120 leechers=1 seeders=0
left=-5
announce response: interval=120 leechers=0 seeders=1
left=0
announce response: interval=120 leechers=0 seeders=1
```

#### Conclusion

`left = -5` is accepted and the peer is counted as a seeder, identically to `left = 0`. Expected
after the fix: an error response (action 3) and no peer registered.

### V1b - Red regression test at the UDP handler boundary

- Boundary: `udp-server` handler test; it is the smallest deterministic test at the UDP boundary
  that runs the real announce service and in-memory repository.
- Command: `cargo test -p torrust-tracker-udp-server --lib negative_number_of_bytes_left`
  (stable toolchain). Output: [`evidence/red-test-output.txt`](evidence/red-test-output.txt).

```text
a negative `left` must be rejected, got: Ok(AnnounceIpv4(AnnounceResponse { ... leechers: 0, seeders: 1 ... }))
test result: FAILED. 0 passed; 1 failed
```

### V2 - Final like-for-like recheck (after the fix)

> Superseded (PR #2839, not merged): this run used a tracker built from the branch of PR #2839.
> That fix is not on `develop`, where the behaviour is still the one of V1. The recheck is repeated
> by issue #2850 against its own implementation.

- Status: `DONE`
- Date and time (UTC): 2026-11-12 16:32
- Build: a clean tree (`git status --short` printed nothing before the build) at the commit
  `fix(udp-server): [#2838] classify a negative byte count as a bad request`, which is the last
  production change and follows `fix(udp-core): [#2838] reject a negative number of bytes in an
  announce` and `refactor(udp-core): [#2838] type the rejected byte-count field`;
  `cargo build --bin torrust-tracker`, stable Rust toolchain, same config and probe as V1.
- An earlier run of this recheck (2026-11-12 09:12) was made on an uncommitted tree before the
  review fixes; it is superseded by this one.

#### Steps Performed

1. `TORRUST_TRACKER_CONFIG_TOML_PATH=evidence/tracker.toml ./target/debug/torrust-tracker`
2. The five probe commands below, one after another (each announce replaces the same peer). The
   probe arguments are `<port> <left> [uploaded] [downloaded]`.

#### Observed Result

Probe output with step times, verbatim from
[`evidence/probe-after-fix.txt`](evidence/probe-after-fix.txt):

```text
16:32:52.528 start: python3 -I evidence/udp_announce_probe.py 6868 1000
announce response: interval=120 leechers=1 seeders=0
16:32:52.540 end
16:32:52.543 start: python3 -I evidence/udp_announce_probe.py 6868 -5
action=3 message='tracker announce error: Invalid announce request: `left` must not be negative, got -5'
16:32:52.555 end
16:32:52.557 start: python3 -I evidence/udp_announce_probe.py 6868 0
announce response: interval=120 leechers=0 seeders=1
16:32:52.570 end
16:32:52.572 start: python3 -I evidence/udp_announce_probe.py 6868 0 -1 0
action=3 message='tracker announce error: Invalid announce request: `uploaded` must not be negative, got -1'
16:32:52.585 end
16:32:52.588 start: python3 -I evidence/udp_announce_probe.py 6868 0 0 -1
action=3 message='tracker announce error: Invalid announce request: `downloaded` must not be negative, got -1'
16:32:52.600 end
```

Tracker log: `grep -c "must not be negative"` on the tracker log printed `3`, one line per
rejection. The three lines are in
[`evidence/tracker-log-after-fix.txt`](evidence/tracker-log-after-fix.txt); the first, shortened
(the request dump is elided):

```text
2026-11-12T16:32:52.551837Z ERROR process_request:handle_packet{...}:handle_error{req_kind=Some(Announce { ... bytes_left: NumberOfBytes(I64(-5)) ... }) ... error=AnnounceFailed { source: NegativeNumberOfBytes { field: Left, value: -5 } } opt_transaction_id=Some(TransactionId(I32(4321)))}: UDP TRACKER: response error error=tracker announce error: Invalid announce request: `left` must not be negative, got -5 client_socket_addr=127.0.0.1:37317 server_socket_addr=127.0.0.1:6868 service_binding=udp://127.0.0.1:6868 request_id=8c12f0e4-869e-41a1-9167-cd68cd05c118 transaction_id=4321
```

Green tests on the same tree: `cargo test -p torrust-tracker-udp-core -p torrust-tracker-udp-server --lib`
(stable toolchain): 108 passed and 225 passed, 0 failed.

#### Conclusion

The symptom observed in V1 is gone; valid values (`left` 0 and 1000) behave as before. The UDP
error event classification (`ErrorKind::BadRequest`) is covered by a unit test, not by this run.

## Failures and Follow-up

None.
