---
semantic-links:
  pr: "https://github.com/torrust/torrust-tracker/pull/2050"
  pr-title: "Feat/i2p peer support"
  review-pass: "review-pass-1.md"
---

# Manual Test Evidence — PR #2050 I2P Peer Support

Empirical verification of I2P peer behavior in the Torrust Tracker.
This file documents every command, output, and observation needed to
repeat the experiments.

---

## Prerequisites

- Branch `pr-2050` checked out locally
- Rust toolchain installed (MSRV 1.88)
- Tracker built with `cargo build`
- Storage directories created

---

## Test 1: Announce an I2P peer (non-compact)

**Goal**: Verify that an I2P Destination in the `ip` parameter is accepted
and stored.

**Command**:

```bash
# Generate a valid I2P Destination (516 chars Base64 + .i2p suffix)
# Using a padded Destination with 391 decoded bytes (387 + 4 cert payload)
# cspell:disable-next-line
I2P_DEST="$(python3 -c "print('A' * 512 + 'BQAEAAAAAA==.i2p')")"
INFO_HASH="%9C8B%22%13%E3%0B%FF%21%2B0%C3%60%D2o%9A%02%13d%22"
PEER_ID="-QT0001-000000000001"

curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=${PEER_ID}&port=1&ip=${I2P_DEST}&uploaded=0&downloaded=0&left=0&compact=0"
```

**Expected**: Tracker accepts the announce and returns a response with
`peers: []` (no other peers yet).

**Observed output**:

```text
Destination length: 528
# cspell:disable-next-line
d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peerslee
```

**Result**: **PASS** — an exactly generated Destination with raw `==` padding
is accepted. Responses are bencoded, so `python3 -m json.tool` is not an
appropriate response decoder.

**Important limitation**: sending the same padding as `%3D%3D` is rejected;
see Test 5.

---

## Test 2: Announce a second I2P peer — verify I2P-to-I2P matchmaking

**Goal**: Verify that two I2P peers on the same torrent see each other
in the response.

**Command**:

```bash
# cspell:disable-next-line
I2P_DEST_1="$(python3 -c "print('A' * 512 + 'BQAEAAAAAA==.i2p')")"
# cspell:disable-next-line
I2P_DEST_2="$(python3 -c "print('B' + 'A' * 511 + 'BQAEAAAAAA==.i2p')")"
INFO_HASH="9c38422213e30bff212b30c360d26f9a02136422"

# Announce first I2P peer
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000001&port=1&ip=${I2P_DEST_1}&uploaded=0&downloaded=0&left=0&compact=0" > /dev/null

# Announce second I2P peer — should see first peer in response
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000002&port=1&ip=${I2P_DEST_2}&uploaded=0&downloaded=0&left=0&compact=0" | python3 -m json.tool
```

**Expected**: Response contains one peer with:

- `ip` field = full I2P Destination string of peer 1
- `port` = 1 (placeholder)
- `peer_id` = peer 1's peer ID

**Observed output**:

```text
I2P-to-I2P: destination-present=yes, response-bytes=654
```

**Result**: **PASS** — an I2P requester receives another I2P peer in a
non-compact response.

---

## Test 3: Clearnet client does not receive I2P peers

**Goal**: Verify that a clearnet announce does not return I2P peers.

**Command**:

```bash
# cspell:disable-next-line
I2P_DEST="$(python3 -c "print('A' * 512 + 'BQAEAAAAAA==.i2p')")"
INFO_HASH="9c38422213e30bff212b30c360d26f9a02136422"

# Announce an I2P peer
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000001&port=6881&ip=${I2P_DEST}&uploaded=0&downloaded=0&left=0&compact=0" > /dev/null

# Announce a clearnet peer — check if I2P peer appears
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000002&port=6881&uploaded=0&downloaded=0&left=0&compact=0" | python3 -m json.tool
```

**Expected**: Response should NOT contain the I2P peer.

**Observed output**:

```text
Clearnet-to-I2P: destination-present=no, response-bytes=75
```

**Result**: **PASS** — peers are filtered by address kind in
`Coordinator::peers_excluding()`. The original review finding that the PR
mixed I2P and clearnet responses was incorrect.

---

## Test 4: I2P compact response contains 32-byte hashes

**Goal**: Verify that an I2P compact response uses 32-byte Destination hashes.

**Command**:

```bash
# cspell:disable-next-line
I2P_DEST="$(python3 -c "print('A' * 512 + 'BQAEAAAAAA==.i2p')")"
INFO_HASH="9c38422213e30bff212b30c360d26f9a02136422"

# Announce an I2P peer
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000001&port=1&ip=${I2P_DEST}&uploaded=0&downloaded=0&left=0&compact=0" > /dev/null

# Announce a second I2P peer with compact=1 — inspect raw response
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000002&port=1&ip=${I2P_DEST_2}&uploaded=0&downloaded=0&left=0&compact=1" -o /tmp/compact_response.bin

# Analyze the peers byte string length
python3 -c "
import bencode
with open('/tmp/compact_response.bin', 'rb') as f:
    data = bencode.bdecode(f.read())
peers = data[b'peers']
print(f'peers length: {len(peers)} bytes')
print(f'6-byte chunks (IPv4): {len(peers) // 6}')
print(f'32-byte chunks (I2P): {len(peers) // 32}')
print(f'Mod 6: {len(peers) % 6}')
print(f'Mod 32: {len(peers) % 32}')
print(f'Hex: {peers.hex()}')
"
```

**Observed output**:

```text
peers_length=32, mod_32=0
sha256_hash_payload=19356c32b7979ecf6541a5233085564f2ec55578d603c520b49ccc459f758abc
```

**Result**: **PASS** — the I2P compact response contains exactly one 32-byte
Destination hash. A clearnet compact response after an I2P announce returned
an empty `peers` payload (`peers_length=0`), confirming separation.

---

## Test 5: Query parser — Base64 padding in `ip` parameter (finding 3.3)

**Goal**: Verify that `=` characters in the `ip` parameter are preserved
(not split).

**Command**:

```bash
# I2P Destination with == padding
# cspell:disable-next-line
I2P_DEST="$(python3 -c "print('A' * 512 + 'BQAEAAAAAA==.i2p')")"
INFO_HASH="9c38422213e30bff212b30c360d26f9a02136422"

echo "Destination: ${I2P_DEST}"
echo "Length: ${#I2P_DEST}"

# Announce with the Destination — should succeed
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000001&port=1&ip=${I2P_DEST}&uploaded=0&downloaded=0&left=0&compact=0" | python3 -m json.tool
```

**Expected**: Both raw `==` and URL-encoded `%3D%3D` padding should be accepted
after normal HTTP query decoding.

**Observed output**:

```text
# cspell:disable-next-line
Raw-padding response: d8:completei1e10:incompletei0e8:intervali120e12:min intervali120e5:peerslee
Percent-encoded-padding response: d14:failure reason... invalid param value ...%3D%3D.i2p for ipe
```

**Result**: **FAIL** — raw padding is accepted, but percent-encoded padding is
rejected because the custom query parser does not percent-decode values before
I2P Destination parsing. This is the confirmed URL-encoding interoperability
finding in `review-pass-1.md`.

**Required regression test**: Add a protocol-level test that parses a query
with `ip` set to the exact valid fixture

<!-- cspell:disable-next-line -->

`"A".repeat(512) + "BQAEAAAAAA%3D%3D.i2p"` and asserts that it produces
`Some(AnnounceAddress::I2p(_))`. The test must be committed with the
percent-decoding implementation fix; it is recorded here as an acceptance test
instead of being committed in its currently failing form.

---

## Test 6: Invalid I2P Destination is rejected

**Goal**: Verify that invalid I2P Destinations (wrong Base64, too short)
are rejected.

**Command**:

```bash
INFO_HASH="9c38422213e30bff212b30c360d26f9a02136422"

# Too short (384 decoded bytes = 512 Base64 chars, below 387 minimum)
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000001&port=1&ip=$(python3 -c "print('A' * 512 + '.i2p')")&uploaded=0&downloaded=0&left=0&compact=0" | python3 -m json.tool

# Invalid Base64 characters
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000001&port=1&ip=$(python3 -c "print('!' * 516 + '.i2p')")&uploaded=0&downloaded=0&left=0&compact=0" | python3 -m json.tool
```

**Expected**: Both return error responses (invalid parameter).

**Observed output**:

```text
=== short Destination ===
d14:failure reason... invalid param value ... .i2p for ipe
=== invalid Destination ===
d14:failure reason... invalid param value !!!...!.i2p for ipe
```

**Result**: **PASS** — a too-short Destination and a Destination using an
invalid I2P Base64 character are rejected with bencoded failure responses.

---

## Test 7: UDP handler — I2P peers excluded

**Goal**: Verify that UDP responses do not include I2P peers.

**Command**:

```bash
# This requires the tracker_client binary (UDP announce)
# First announce an I2P peer via HTTP, then query via UDP
# cspell:disable-next-line
I2P_DEST="$(python3 -c "print('A' * 512 + 'BQAEAAAAAA==.i2p')")"
INFO_HASH="9c38422213e30bff212b30c360d26f9a02136422"

# Announce I2P peer via HTTP
curl -s "http://127.0.0.1:7070/announce?info_hash=${INFO_HASH}&peer_id=-QT0001-000000000001&port=1&ip=${I2P_DEST}&uploaded=0&downloaded=0&left=0&compact=0" > /dev/null

# Query via UDP — I2P peer should NOT appear
cargo run --bin tracker_client -- udp announce "udp://127.0.0.1:6969" "${INFO_HASH}" 2>/dev/null | python3 -m json.tool
```

**Expected**: UDP response has empty `peers` list (I2P peer filtered out).

**Observed output**:

```json
{
  "AnnounceIpv4": {
    "transaction_id": -888840697,
    "announce_interval": 120,
    "leechers": 0,
    "seeders": 2,
    "peers": []
  }
}
```

**Result**: **PASS** — the UDP response contains no I2P peer. The tracker
reported two seeders in swarm metadata from preceding HTTP I2P announces, but
the peer list itself is empty, as required by the UDP wire format.

---

## Summary

| Test | Description                  | Finding                  | Status |
| ---- | ---------------------------- | ------------------------ | ------ |
| 1    | I2P announce accepted        | Destination parsing      | PASS   |
| 2    | I2P-to-I2P matchmaking       | Swarm stores I2P peers   | PASS   |
| 3    | Clearnet excludes I2P peers  | Cross-network separation | PASS   |
| 4    | I2P 32-byte compact hash     | I2P compact wire format  | PASS   |
| 5    | Base64 padding URL encoding  | `%3D%3D` compatibility   | FAIL   |
| 6    | Invalid Destination rejected | Validation works         | PASS   |
| 7    | UDP excludes I2P peers       | UDP peer-list filtering  | PASS   |

---

## Focused Automated Tests

**Command**:

```bash
cargo test -p torrust-tracker-primitives \
  -p torrust-tracker-http-protocol \
  -p torrust-tracker-http-core \
  -p torrust-tracker-axum-http-server \
  --lib
```

**Result**: **PASS**.

| Package                         |     Tests |
| ------------------------------- | --------: |
| `torrust-tracker-http-core`     | 21 passed |
| `torrust-tracker-http-protocol` | 52 passed |
| `torrust-tracker-primitives`    | 32 passed |

The run included the I2P-specific unit tests for Destination validation,
announce parsing, I2P peer coordination, and compact/non-compact encoding.
The HTTP protocol tests do not cover percent-encoded `%3D%3D` padding, which
matches the manually reproduced interoperability failure.

---

## Environment

| Item           | Value                                                   |
| -------------- | ------------------------------------------------------- |
| Branch         | `pr-2050` (at `1bb9e9a3`, rebased on `develop`)         |
| Rust toolchain | _(fill in after `rustup show`)_                         |
| Tracker config | `share/default/config/tracker.development.sqlite3.toml` |
| HTTP port      | 7070                                                    |
| UDP port       | 6969                                                    |
| Test date      | 2026-08-17                                              |
| Tester         | Jose Celano + AI agent                                  |
