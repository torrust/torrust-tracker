---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2314-preserve-udp-scrape-response-order/ISSUE.md
last-updated-utc: "2026-09-23 10:55"
---

# Manual Verification Evidence

## Purpose

Record the pre-fix reproduction of the UDP scrape response ordering defect against a real local
tracker build (V1), and later the like-for-like recheck after the fix (V2).

## Environment and Prerequisites

- Date and time (UTC): 2026-09-23, 07:52-08:26 UTC
- Artifact under test: local `develop` at commit `60a4a160`, unmodified, `cargo build` dev profile
  (`./target/debug/torrust-tracker`, `./target/debug/tracker_client`)
- Toolchain: stable Rust `rustc 1.98.1 (48a229cea 2026-09-01)`, `cargo 1.98.1`
- Operating system / environment: Linux 7.0.0-31-generic x86_64
- Prerequisites and setup performed: removed `storage/tracker/lib/database/sqlite3.db` for a fresh
  state; started the tracker with the default development configuration
  (`share/default/config/tracker.development.sqlite3.toml`, SQLite3 driver, UDP trackers on
  `0.0.0.0:6969` and `0.0.0.0:6868`, HTTP tracker on `0.0.0.0:7070`); tracker output captured to
  `.tmp/udp-scrape-order-tracker.log`.

## Verification Processes

### V1 - Reproduce pre-fix ordering mismatch

- Goal: show that the position of each entry in a UDP scrape response does not follow the order of
  the requested info hashes.
- Initial state: fresh tracker, no torrents. Two torrents were then seeded with distinguishable
  swarm statistics:
  - `A = aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa`: 1 seeder, 0 leechers
  - `B = bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb`: 2 seeders, 1 leecher
- Status: `DONE`

#### Steps Performed

1. Started the tracker in the background:

   ```text
   rm -f storage/tracker/lib/database/sqlite3.db
   ./target/debug/torrust-tracker > .tmp/udp-scrape-order-tracker.log 2>&1 &
   ss -ulnp | grep -E '6969|6868'
   ```

2. Seeded the two torrents via UDP announces (peer IDs are raw 20-byte strings for the UDP
   client; `--left 0` marks a seeder):

   ```text
   C=./target/debug/tracker_client
   $C udp announce 127.0.0.1:6969 $A --left 0    --port 6881 '--peer-id=-qB00000000000000001' --ip-address 10.0.0.1
   $C udp announce 127.0.0.1:6969 $B --left 0    --port 6882 '--peer-id=-qB00000000000000002' --ip-address 10.0.0.2
   $C udp announce 127.0.0.1:6969 $B --left 0    --port 6883 '--peer-id=-qB00000000000000003' --ip-address 10.0.0.3
   $C udp announce 127.0.0.1:6969 $B --left 1000 --port 6884 '--peer-id=-qB00000000000000004' --ip-address 10.0.0.4
   ```

3. Captured the ground truth with an HTTP scrape (keyed by info hash, so order-insensitive):

   ```text
   $C http scrape http://127.0.0.1:7070 $A $B
   ```

4. Sent the same positional UDP scrape request ten times in order `[A, B]`, then ten times in
   order `[B, A]`:

   ```text
   for i in $(seq 1 10); do $C udp scrape 127.0.0.1:6969 $A $B; echo; done
   for i in $(seq 1 10); do $C udp scrape 127.0.0.1:6969 $B $A; echo; done
   ```

5. Sent one request with a duplicated info hash to observe duplicate semantics:

   ```text
   $C udp scrape 127.0.0.1:6969 $A $A $B
   ```

6. Stopped the tracker.

#### Observed Result

Announce responses confirming the swarm state (last announce per torrent):

```text
A: {"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":0,"seeders":1,"peers":[]}}
B: {"AnnounceIpv4":{"transaction_id":-888840697,"announce_interval":120,"leechers":1,"seeders":2,"peers":["127.0.0.1:6882"]}}
```

HTTP scrape ground truth:

```text
{"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb":{"complete":2,"downloaded":0,"incomplete":1},
 "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa":{"complete":1,"downloaded":0,"incomplete":0}}
```

UDP scrape, request order `[A, B]` (expected every response: `[A(1s/0l), B(2s/1l)]`):

```text
 1: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   OK
 2: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   OK
 3: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   OK
 4: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   OK
 5: [{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]   SWAPPED
 6: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   OK
 7: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   OK
 8: [{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]   SWAPPED
 9: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   OK
10: [{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]   SWAPPED
```

UDP scrape, request order `[B, A]` (expected every response: `[B(2s/1l), A(1s/0l)]`):

```text
 1: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   SWAPPED
 2: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   SWAPPED
 3: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   SWAPPED
 4: [{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]   OK
 5: [{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]   OK
 6: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   SWAPPED
 7: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   SWAPPED
 8: [{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]   OK
 9: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   SWAPPED
10: [{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]   SWAPPED
```

Every response carried `"transaction_id":-888840697` (the client uses a fixed transaction ID).

UDP scrape with a duplicated info hash, request `[A, A, B]` (3 requested, expected 3 entries):

```text
[{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]   2 entries only
```

Tracker log: the UDP tracker does not log individual scrape requests at the default `INFO` level
(only startup and event-listener lines appear under `UDP TRACKER`), so the log adds no
per-request evidence. The HTTP scrape used for ground truth is logged:

```text
INFO request{method=GET uri=/scrape?info_hash=%AA...%AA&info_hash=%BB...%BB version=HTTP/1.1}: HTTP TRACKER: response ... status_code=200 OK
```

#### Conclusion

Reproduced. Out of 20 identical-state requests, 10 returned the statistics at the wrong index
(3 of 10 for `[A, B]`, 7 of 10 for `[B, A]`). The response order is independent of the request
order and varies between otherwise identical requests, which matches iteration over a `HashMap`
with a per-instance random hasher state. A client following BEP 15 would attribute torrent B's
statistics to torrent A (and vice versa) in those responses.

Additionally, duplicated info hashes are collapsed: a 3-hash request returned 2 entries, so the
positional contract (one entry per requested hash) is broken by the same `HashMap`
representation.

### V2 - Recheck fixed ordering

- Goal: repeat V1 unchanged against the fixed build.
- Initial state: same as V1.
- Status: `TODO`

## Failures and Follow-up

None. V1 completed as planned.
