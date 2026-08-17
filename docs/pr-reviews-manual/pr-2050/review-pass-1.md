---
semantic-links:
  pr: "https://github.com/torrust/torrust-tracker/pull/2050"
  pr-title: "Feat/i2p peer support"
  pr-author: "Frigyes06"
  pr-branch: "feat/i2p-peer-support → develop"
  pr-stats: "+925 / -355, 45 files, 2 commits"
  i2p-spec: "https://i2p.net/en/docs/applications/bittorrent/"
  related-artifacts:
    - docs/packages.md
    - packages/primitives/src/i2p.rs
    - packages/http-protocol/src/v1/responses/announce/encoding.rs
---

# PR #2050 — I2P Peer Support

Manual review of [PR #2050](https://github.com/torrust/torrust-tracker/pull/2050)
"Feat/i2p peer support" by [Frigyes06](https://github.com/Frigyes06).

**Branch checked out locally**: `pr-2050` (at `1bb9e9a3`, rebased on `develop`).

---

## 1. PR Overview

Adds I2P anonymous network peer support to the HTTP tracker. I2P clients can
announce using their Base64 Destination in the `ip` query parameter, and the
tracker matchmakes I2P peers. The goal is I2P/Internet isolation — I2P peers
should not be mixed with clearnet peers in responses.

### Commits

| SHA        | Message                                         |
| ---------- | ----------------------------------------------- |
| `0f720738` | I2P support                                     |
| `1bb9e9a3` | feat(http-tracker): support I2P peers (rebased) |

### Scope

- HTTP tracker announce parsing, encoding, and response generation
- Swarm peer-address model (new `PeerAddress` enum)
- UDP handler filtering (I2P peers excluded from UDP responses)
- REST API adapter adjustments (references instead of by-value)
- Documentation and tests

---

## 2. Architecture Changes

### 2.1 New `I2pDestination` type (`packages/primitives/src/i2p.rs`)

Parses and validates I2P Base64 Destinations:

- Custom I2P Base64 alphabet (`A-Za-z0-9-~`)
- Minimum 387 decoded bytes, certificate length validation
- SHA-256 hash computed at parse time for compact responses
- Normalizes suffix (strips `.i2p`/`.I2P` for storage, re-adds on display)

**Assessment**: Solid implementation. Validation covers length, Base64 charset,
and certificate consistency.

### 2.2 `PeerAddress` enum (`packages/primitives/src/peer.rs`)

```rust
pub enum PeerAddress {
    Clearnet(SocketAddr),
    I2p(I2pPeerAddress),
}
```

Replaces `SocketAddr` in the `peer_addr` field of `Peer`. This is the
**highest-impact change** — it ripples through the entire codebase.

Helpers: `port()`, `ip() → Option<IpAddr>`, `socket_addr() → Option<SocketAddr>`,
`is_i2p()`. Implements `Ord`, `Hash`, `Eq` for use as `BTreeMap` key.

**Assessment**: Clean design. The `ip()` returning `Option` forces callers to
handle the I2P case explicitly — good.

### 2.3 HTTP announce request parsing

New `AnnounceAddress` enum in `packages/http-protocol/src/v1/requests/announce.rs`:

```rust
pub enum AnnounceAddress {
    Ip(IpAddr),
    I2p(I2pDestination),
}
```

The `extract_ip()` function now tries `IpAddr` first, then `I2pDestination`.
Invalid `.i2p` suffixes are rejected with a clear error.

**Query parser fix**: `split_once('=')` replaces `split('=').collect()` — values
containing `=` (like I2P Base64 padding `==`) are now preserved correctly. This
is also a bugfix for existing non-I2P behavior.

**Assessment**: Good. The fallback logic (try IP, then I2P, then ignore) is
correct. The query parser fix is a genuine improvement.

### 2.4 HTTP response encoding

**Non-compact**: `NormalPeer.ip` is now `String` (was `IpAddr`). I2P peers
serialize their full Destination string; clearnet peers serialize their IP as
a string. Port is `I2P_PLACEHOLDER_PORT = 1` for I2P.

**Compact**: New `CompactPeer::I2p([u8; 32])` variant. For an I2P requester,
the response encodes each returned I2P peer as its 32-byte SHA-256 hash in the
`peers` byte string.

**Assessment**: Encoding is correct per the I2P BitTorrent spec. Manual
verification confirmed an I2P-only compact response with a 32-byte `peers`
payload. See Section 3 for the URL-encoding interoperability finding.

### 2.5 Swarm registry

`BTreeMap<PeerAddress, Arc<Peer>>` — the key change from `SocketAddr` to
`PeerAddress` prevents key collisions between I2P and clearnet peers. Peer
inactivity cleanup operates on `PeerAddress` correctly (Ord/Hash implemented).

### 2.6 UDP handler filtering

UDP announce response builder uses `peer.peer_addr.ip()` with `filter_map` —
I2P peers return `None` for `ip()` and are excluded. Correct behavior per
spec (UDP-over-I2P was only standardized 2025-06).

---

## 3. Key Findings

### 3.1 ✅ I2P and clearnet peers are isolated in swarm responses

The swarm coordinator filters peers by address kind before returning them:

```rust
.filter(|peer| peer.peer_addr.is_i2p() == peer_addr.is_i2p())
```

Manual verification confirms that an I2P requester received another I2P peer,
while a subsequent clearnet request contained no I2P Destination and an empty
compact `peers` payload. This satisfies the I2P cross-network prevention
requirement. The initial review conclusion that peers were mixed was incorrect.

### 3.2 ⚠️ Percent-encoded Base64 padding is rejected

I2P Destinations can contain Base64 padding (`==`). A valid Destination with
raw padding is accepted, but the same value with URL-safe `%3D%3D` padding is
rejected. The custom query parser preserves `%3D` literally instead of
percent-decoding it before I2P Destination parsing.

This is an interoperability issue: a compliant HTTP client may encode reserved
`=` characters in query parameter values. The parser should URL-decode query
parameter values before parsing the I2P Destination.

### 3.3 ⚠️ Client-side compact deserialization is not I2P-aware

`DeserializedCompactParsed` in `deserialization.rs` assumes fixed 6-byte chunks:

```rust
for peer_bytes in compact_announce.peers.chunks_exact(6) {
    peers.push(CompactPeer::new_from_bytes(peer_bytes));
}
```

A 32-byte I2P hash is not parsed correctly by this 6-byte IPv4-oriented path.
The test `it_should_return_i2p_destination_hashes_in_a_compact_response` works
around this by comparing raw bytes (`announce.peers == *hash`) rather than
going through the deserialization path.

### 3.4 ✅ Query parser bugfix (unrelated to I2P)

`split_once('=')` replacing `split('=').collect()` is a genuine improvement.
Previously, `name=value=value` was rejected; now `name=value==` is correctly
parsed as value `value==`. This is relevant for Base64 padding but also
benefits any parameter value containing `=`.

### 3.5 ✅ I2P Destination validation is thorough

- Base64 alphabet check
- Minimum length (387 decoded bytes)
- Certificate payload length consistency
- SHA-256 hash computed at parse time (not on every response)

### 3.6 ✅ UDP filtering is correct

The UDP handler correctly excludes I2P peers. No changes needed there.

### 3.7 ⚠️ Tracker client does not support I2P announces

The `tracker_client` binary (`console/tracker-client`) accepts `--ip` as an
`IpAddr` parameter. It cannot send I2P Destinations in the `ip` query
parameter. This means:

- Manual testing of I2P announces requires raw `curl` commands
- The client should be updated in a follow-up PR to support `--i2p-destination`
  or accept a string `--ip` parameter

This is not a blocker for the PR, but should be documented as a known
limitation and tracked as a follow-up issue.

### 3.8 ⚠️ Interaction with issue #1987 must be specified

Issue [#1987](../../issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/ISSUE.md)
plans an opt-in setting for trusting a clearnet IP supplied through the `ip`
query parameter. PR #2050 deliberately creates a protocol-specific exception:
a valid I2P Destination must be used even while that setting is disabled.

The eventual precedence must be explicit:

1. A valid I2P Destination is always used as an I2P peer address.
2. A valid clearnet IP is used only when the future opt-in setting is enabled.
3. Otherwise, use the resolved connection IP.

The #1987 specification now records this rule and requires an ADR when that
issue is implemented. This PR should link to #1987 and document the exception.

---

## 4. Actions for the Contributor

### Required before merge

- [ ] **A1**: **URL-decode I2P Destination query values** — A valid Destination
      with percent-encoded Base64 padding (`%3D%3D`) is rejected, while the
      raw-padding form succeeds. Decode query parameter values before I2P
      Destination parsing and add contract tests for raw and percent-encoded
      padding.

- [ ] **A2**: **Deserialization fix** — `DeserializedCompactParsed` uses
      `chunks_exact(6)` and cannot parse a valid 32-byte I2P compact peer hash.
      Extend the client-side response types or explicitly separate them from
      the clearnet-only parsed compact representation.

### Follow-up work or documentation

- [ ] **F1**: **Query parser regression test** — The `split_once('=')` fix
      changes behavior for `name=value=value` (now accepted instead of rejected).
      Retain a test documenting this intentional change.

- [ ] **F2**: **Placeholder port documentation** — The I2P spec says:
      _"Clients generally include a fake port=6881 parameter... Trackers may
      ignore the port parameter, and should not require it."_ Document why
      `I2P_PLACEHOLDER_PORT` is `1` (not `6881`) and that this value is
      conventional.

- [ ] **F3**: **Destination enforcement as future work** — Document that
      `X-I2P-DestHash` / `X-I2P-DestB64` / `X-I2P-DestB32` header validation
      is not implemented and is planned as future work. The I2P spec says:
      _"we expect that all trackers will eventually enforce destinations."_

- [ ] **F4**: **Tracker client I2P support** — The `tracker_client` binary
      (`console/tracker-client`) accepts `--ip` as `IpAddr` only. It cannot
      send I2P Destinations. Add a comment in the PR noting this limitation
      and track a follow-up issue to add `--i2p-destination` support to the
      client.

---

## 5. Spec-Informed Answers to Our Questions

Cross-referenced with the [I2P BitTorrent specification](https://i2p.net/en/docs/applications/bittorrent/).

### Q1: Swarm isolation model — separate swarms or filtering at response layer?

**Spec answer**: Filtering at the response layer is sufficient. The spec says:

> "Trackers should reject standard network announces with IPv4 or IPv6 IPs,
> and not deliver them in responses."

The spec does not require separate swarms — it requires that **I2P peers are
not delivered in clearnet responses** and vice versa. The tracker stores all
peers in one swarm and filters at encode time.

**Implication for PR #2050**: The PR stores I2P and clearnet peers in the same
`BTreeMap<PeerAddress, ...>` and filters returned peers by `is_i2p()` in the
swarm coordinator. Manual verification confirmed that clearnet and I2P
responses are isolated, satisfying this requirement.

### Q2: Destination enforcement via X-I2P-Dest\* headers

**Spec answer**: Optional but recommended, and expected to become universal:

> "Trackers may choose to prevent spoofing by requiring this, and verifying the
> client's Destination using HTTP headers added by the I2PTunnel HTTP Server
> tunnel... Unfortunately, as the network grows, so will the amount of
> maliciousness, so we expect that all trackers will eventually enforce
> destinations."

The headers (`X-I2P-DestHash`, `X-I2P-DestB64`, `X-I2P-DestB32`) cannot be
spoofed by the client. A tracker enforcing destinations "need not require the
`ip` announce parameter at all."

**Implication for PR #2050**: Not blocking for merge, but should be documented
as a known limitation and future work item. Without enforcement, any client
can spoof I2P announces by passing a valid Destination in `ip`.

### Q3: Compact response format — separate field or mixed?

**Spec answer**: The spec is clear — the compact response `peers` key should
be a **single byte string of concatenated 32-byte SHA-256 hashes**:

> "In the compact response, the value of the 'peers' dictionary key is a
> single byte string, whose length is a multiple of 32 bytes. This string
> contains the concatenated 32-byte SHA-256 Hashes of the binary Destinations
> of the peers."

**Critical finding**: The spec describes an **I2P-only** compact response.
The `peers` field should contain ONLY 32-byte I2P hashes — not mixed with
IPv4 (6-byte) or IPv6 (18-byte) entries. This is a different wire format than
standard BEP 23 compact responses.

**Implication for PR #2050**: The coordinator filters by address kind before
encoding. Manual verification confirmed that an I2P compact response contained
only a 32-byte hash and that a clearnet compact response contained no I2P
hashes. This means:

- A clearnet client should never see I2P hashes in `peers`
- An I2P client should only see 32-byte hashes in `peers`
- The tracker needs to know the requester's network type to choose the right
  encoding

### Q4: UDP over I2P

**Spec answer**: Spec finalized 2025-06, support rolling out later in 2025.
The spec references a separate [UDP announce specification](https://i2p.net/en/docs/specs/udp-announces)
with differences from BEP 15.

**Implication for PR #2050**: HTTP-only scope is correct for now. UDP I2P
support is a separate, future effort.

### Q5: Testing — raw bytes vs. deserialization

**Spec answer**: Not directly addressed, but the spec says compact responses
are "a single byte string, whose length is a multiple of 32 bytes." The
current `DeserializedCompactParsed` (which uses `chunks_exact(6)`) is
incompatible with I2P compact responses.

**Implication for PR #2050**: The test comparing raw bytes is correct for
validating the wire format, but the deserialization path needs to be updated
to handle I2P compact responses (32-byte chunks, not 6-byte).

### Q6: `max_peers_per_announce` — separate or shared?

**Spec answer**: The spec does not address this. The `numwant` parameter is
the same as standard bittorrent.

**Implication for PR #2050**: Currently shared. This is acceptable per spec,
but may need clarification in documentation.

### Q7: PEX / DHT over I2P

**Spec answer**: Both are specified:

- **PEX**: Extension message `i2p_pex`, uses 32-byte SHA-256 hashes (same
  format as compact response)
- **DHT**: Extension message `i2p_dht`, compact node info is 54 bytes
  (20-byte Node ID + 32-byte hash + 2-byte port). Requires SAM v3.3
  PRIMARY and SUBSESSIONS.

**Implication for PR #2050**: Not required for this PR, but the spec explicitly
defines these. Should be documented as future work.

---

## 6. Review Progress

| Date       | Reviewer               | File                           | Status                                              |
| ---------- | ---------------------- | ------------------------------ | --------------------------------------------------- |
| 2026-08-17 | Jose Celano + AI agent | `review-pass-1.md` (this file) | Draft — awaiting contributor responses              |
| 2026-08-17 | Jose Celano + AI agent | `review-pass-1.md` §5          | Spec cross-reference complete (I2P BitTorrent spec) |

---

## 7. Appendix: I2P BitTorrent Spec Summary

Cross-referenced with the [official I2P BitTorrent specification](https://i2p.net/en/docs/applications/bittorrent/)
on 2026-08-17.

| Aspect                   | Spec Requirement                                                     | PR #2050 Status                                     |
| ------------------------ | -------------------------------------------------------------------- | --------------------------------------------------- |
| Addressing               | Destination (387+ bytes, Base64 ~516+ chars), optional `.i2p` suffix | ✅ Implemented                                      |
| Announce `ip` param      | Full Base64 Destination (port is placeholder, often `6881`)          | ✅ Implemented (port=1, not 6881)                   |
| Non-compact response     | Full Destination string in `ip` field of peer dictionary             | ✅ Implemented and isolated from clearnet responses |
| Compact response         | 32-byte SHA-256 hash of binary Destination (I2P-only, no mixing)     | ✅ Implemented and isolated from clearnet responses |
| Enforcement headers      | `X-I2P-DestHash`, `X-I2P-DestB64`, `X-I2P-DestB32` (I2PTunnel-added) | ❌ Not implemented (future work)                    |
| Cross-network prevention | "Reject standard network announces... not deliver them in responses" | ✅ Implemented through swarm address-kind filtering |
| UDP announce             | Spec finalized 2025-06, rolling out later in 2025                    | ✅ HTTP-only scope is correct                       |
| PEX                      | Extension message `i2p_pex` (32-byte hashes)                         | ❌ Not in scope                                     |
| DHT                      | Extension message `i2p_dht`, 54-byte compact node info               | ❌ Not in scope                                     |
| SAMv3                    | Recommended for non-Java clients; `SIGNATURE_TYPE=7` (Ed25519)       | N/A (tracker-side only)                             |
