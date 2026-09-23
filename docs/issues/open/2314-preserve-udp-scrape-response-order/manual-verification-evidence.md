---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2314-preserve-udp-scrape-response-order/ISSUE.md
last-updated-utc: "2026-09-23 12:45"
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
- Artifact under test: local branch `2314-preserve-udp-scrape-response-order` at a pre-rebase
  local commit (`aef07ca4`, since orphaned by the rebase onto `develop`); the published commit
  carrying the same fix is `d38c1d2c`. Rebuilt with
  `cargo build --bin torrust-tracker` and
  `cargo build -p torrust-tracker-client --bin tracker_client`.
- Status: `DONE`

#### Steps Performed

1. Removed `storage/tracker/lib/database/sqlite3.db` and started the rebuilt debug tracker with
   `share/default/config/tracker.development.sqlite3.toml`.
2. Repeated the four UDP announces from V1 to establish `A` with one seeder and `B` with two
   seeders and one leecher.
3. Repeated ten UDP scrapes for `[A, B]`, ten for `[B, A]`, and one for `[A, A, B]`, using the
   exact V1 `tracker_client` commands.
4. Stopped the tracker after the verification.

#### Observed Result

All ten `[A, B]` responses were:

```text
[{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]
```

All ten `[B, A]` responses were:

```text
[{"seeders":2,"completed":0,"leechers":1},{"seeders":1,"completed":0,"leechers":0}]
```

The duplicated request `[A, A, B]` returned three ordered entries:

```text
[{"seeders":1,"completed":0,"leechers":0},{"seeders":1,"completed":0,"leechers":0},{"seeders":2,"completed":0,"leechers":1}]
```

#### Conclusion

Passed. Every positional response entry aligned with its request hash and the duplicate hash
produced a duplicate response entry. An initial V2 attempt was excluded because it used a stale
debug tracker binary built before the fix; rebuilding both the tracker and unified client produced
the valid results recorded above.

## Regression-Test Design

### B3 - Test Boundary and Prose-First Review

- Date: 2026-09-23
- Test boundary: `packages/udp-server/src/handlers/scrape.rs`, in its existing public-tracker
   `scrape_request` collaboration tests. `handle_scrape` is the visible production Act; it reaches
   the UDP service, tracker-core handler, and positional response builder.
- Module decision under test: transform keyed `ScrapeData` into BEP 15's positional
   `ScrapeResponse.torrent_stats` sequence. The response must preserve every request position;
   authorization and swarm-state collection remain collaborator-owned.
- Existing helpers retained: `initialize_core_tracker_services_for_public_tracker` owns ordinary
   service wiring; cookie, socket, and event-sender construction are incidental. Helpers will be
   extended only to seed visible swarm state and construct a request from the visible ordered hash
   vector.

#### B4 - Duplicate Entry Count

- Arrange: seed torrent `A` with one visible seeder; construct the visible ordered request
   `[A, A]`; independently specify two identical typed statistics entries as the expected vector.
- Act: call production `handle_scrape` with that request.
- Assert: extract the typed scrape response and compare its complete `torrent_stats` vector to
   the expected two-entry vector. The one initial-state difference is the repeated requested hash.

#### B5 - Request Order

- Arrange: seed eight distinct hashes with distinguishable visible seeder counts, construct their
   deliberately non-sorted request order, and independently specify the complete typed statistics
   vector in that same order.
- Act: call production `handle_scrape` with that request.
- Assert: compare the complete typed response vector without sorting or accepting alternatives.
   With $N = 8$, the unfixed map iteration can accidentally pass only with probability $1/8! =
   1/40320$.

### B4-B6 - Red Regression Evidence and Design Review

- Date: 2026-09-23
- Toolchain: stable Rust `cargo 1.98.1`, `rustc 1.98.1 (48a229cea 2026-09-01)`

#### B4 - Duplicate Entry Count Is Red

Command:

```text
cargo test -p torrust-tracker-udp-server it_should_return_an_entry_for_each_duplicate_requested_info_hash
```

Result: `FAILED` as expected. The response contained one `TorrentScrapeStatistics` entry with
one seeder for request `[A, A]`; the independently specified expected response contained two
identical typed entries. This deterministically demonstrates that the keyed intermediate result
collapses the duplicate request position.

#### B5 - Eight-Hash Order Is Red

Command:

```text
cargo test -p torrust-tracker-udp-server it_should_preserve_the_order_of_eight_requested_info_hashes
```

Result: `FAILED` as expected. For requested distinct-seeder sequence `[8, 3, 6, 1, 7, 2, 5, 4]`,
the response returned `[3, 1, 8, 6, 7, 2, 4, 5]`. The test uses $N = 8$, so a map iteration would
match this deliberate request order by chance with probability about $1/8! = 1/40320$
(`HashMap` iteration order is unspecified, not a uniform permutation, so this is a heuristic
rather than a bound). The recorded failure above is the actual red evidence; the two sibling
regressions (B4 duplicate `[A, A]` and the direct `build_response` empty-map test) are
deterministic on the unfixed code.

#### B6 - Completed Prose-First Review

Both tests call production `handle_scrape`, rather than testing a helper or map directly. Their
causal initial state is visible: `[A, A]` for duplicate cardinality and the explicit non-sorted
eight-hash vector with distinct seeder counts for order. Each test independently specifies the
complete typed expected response vector and uses a single exact-response assertion. Neither
assertion sorts data, compares a set, or accepts an alternative order. The `add_seeders` helper
owns only incidental peer-ID/address construction; the request, expected statistics, Act, and
assertion remain visible in the order test.

### B7-B10 - Repair and Green Focused Verification

- Date: 2026-09-23
- Toolchain: stable Rust `cargo 1.98.1`, `rustc 1.98.1 (48a229cea 2026-09-01)`

#### B7 - Causal Boundary Re-confirmed

The red tests isolate the causal seam to `udp-server::handlers::scrape::build_response`: the
domain `ScrapeData.files` result remains keyed, while the UDP response is positional. No
`ScrapeData` consumer discovered during B3-B6 needs request order, so the selected Option 1
repair remains valid.

#### B8 - Selected Repair

`build_response` now iterates `request.info_hashes`, looks up each domain hash in
`ScrapeData.files`, and allocates the response vector with the request length. Each missing map
entry falls back to zeroed metadata; a direct response-builder test covers that defensive path.

#### B9 - Regression Tests Are Green

Command:

```text
cargo test -p torrust-tracker-udp-server scrape_request
```

Result: `10` scrape-request tests passed, including both maintained regressions:
`it_should_return_an_entry_for_each_duplicate_requested_info_hash` and
`it_should_preserve_the_order_of_eight_requested_info_hashes`.

#### B10 - Affected Crates Are Green

Commands:

```text
cargo test -p torrust-tracker-udp-server
cargo test -p torrust-tracker-primitives --doc
```

Results: UDP server `170` unit tests, `11` integration tests, and `1` doctest passed. Primitives
`2` documentation tests passed. The focused fallback command
`cargo test -p torrust-tracker-udp-server it_should_return_zeroed_statistics_when_scrape_data_does_not_contain_a_requested_hash`
also passed.

#### AC6 - HTTP Non-regression

Commands:

```text
cargo test -p torrust-tracker-http-core
cargo test -p torrust-tracker-axum-http-server
```

Results: HTTP core `31` tests passed. Axum HTTP server `36` unit tests and `61` integration tests
passed. The UDP-only positional adaptation did not change HTTP scrape behavior.

## Failures and Follow-up

None. V1 completed as planned.
