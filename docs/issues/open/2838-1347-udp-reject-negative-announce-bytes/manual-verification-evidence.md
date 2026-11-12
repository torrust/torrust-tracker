---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/ISSUE.md
last-updated-utc: 2026-11-11 17:30
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-11 17:28-17:30
- Artifact under test: `develop` plus the `test(udp-server): reject negative left` red-test patch
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

- Status: `DONE`
- Build: `develop` plus the `fix(udp-core): reject a negative number of bytes in an announce` commit
  and the tests after it (no production change after the fix commit); `cargo build --bin
  torrust-tracker`, same config and probe as V1, run 2026-11-12 09:12 UTC.
- Output: [`evidence/probe-after-fix.txt`](evidence/probe-after-fix.txt)

```text
left=1000          -> announce response: interval=120 leechers=1 seeders=0
left=-5            -> action=3 message='tracker announce error: Invalid announce request: `left` must not be negative, got -5'
left=0             -> announce response: interval=120 leechers=0 seeders=1
uploaded=-1        -> action=3 message='... `uploaded` must not be negative, got -1'
downloaded=-1      -> action=3 message='... `downloaded` must not be negative, got -1'
```

The tracker log shows one `ERROR ... handle_error` line per rejection (the existing UDP error
path, which also emits the UDP error event).

Green regression tests: `cargo test -p torrust-tracker-udp-server --lib negative_number_of_bytes_left`
(1 passed) and `cargo test -p torrust-tracker-udp-core --lib announce` (23 passed).

Conclusion: the symptom observed in V1 is gone; valid values (`left` 0 and 1000) behave as before.

## Failures and Follow-up

None.
