---
semantic-links:
  pr: "https://github.com/torrust/torrust-tracker/pull/2050"
  superseding-pr: "https://github.com/torrust/torrust-tracker/pull/2059"
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

> **Review status (2026-08-18):** PR #2050 and commit `1bb9e9a3` are the
> historical source reviewed in this report. The signed review baseline is
> `2050-i2p-peer-support-reviewed`; the active proposed implementation is
> draft [PR #2059](https://github.com/torrust/torrust-tracker/pull/2059).
> Findings and merge gates below apply to PR #2059 unless a section explicitly
> identifies the historical PR #2050 source.

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
I2P peers return `None` for `ip()` and are excluded. This is appropriate for
the HTTP-only scope of this PR. UDP-over-I2P was standardized in 2025-06 and
is covered by a separate I2P specification.

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

### 3.9 ⚠️ I2P parse errors lose useful diagnostic context

`I2pDestination::from_str` exposes structured errors that distinguish invalid
I2P Base64, an undersized Destination, and an inconsistent certificate length.
However, `extract_ip()` discards those errors and, when the value ends in
`.i2p`, returns the generic `ParseAnnounceQueryError::InvalidParam` error.

This does not meet the error-handling convention's clarity and actionability
goals: an I2P client cannot tell whether it must correct Base64 encoding,
provide a full Destination, or correct certificate data. Preserve the
structured I2P parse error as the source of an I2P-specific announce error, or
include its reason in the user-facing failure response.

### 3.10 ⚠️ Destination structure validation is incomplete and input is unbounded

`I2pDestination::from_str` decodes and hashes the complete untrusted query
value without a maximum encoded or decoded length. On failure,
`ParseAnnounceQueryError::InvalidParam` includes that complete value in the
bencoded HTTP failure response. This can amplify a large request into memory,
CPU, and response/log output work.

The I2P common-structures specification defines a Destination as `KeysAndCert`:
384 key/padding bytes followed by a Certificate whose type and payload length
must be valid. The implementation checks only the declared certificate length;
it accepts arbitrary certificate types and payloads when their lengths match.
The specification explicitly cautions implementers to prohibit excess data and
enforce the appropriate length for each certificate type.

Add a compatibility-aware maximum before Base64 decoding, validate supported
certificate types and their payload structure, reject unsupported types
explicitly, and avoid echoing full untrusted input. Do **not** impose the I2P
BitTorrent page's current $475$-byte "reasonable maximum" as a universal hard
limit: modern valid Key Certificates may contain excess key data and exceed it.

### 3.11 ⚠️ REST peer-address contract is undocumented and untested for I2P

The REST adapter safely serializes `PeerAddress` using `to_string()`, so an I2P
peer is not dropped or converted into a misleading clearnet socket address. It
is returned as its full normalized Destination. However, the public REST DTO
documents `peer_addr` as "The peer's socket address" and gives only an
`IP:port` example. No REST adapter or endpoint contract test covers an I2P
peer.

Do not silently redefine this existing field as a polymorphic endpoint string.
That would produce partial I2P support with an ambiguous REST contract. The
initial I2P work must instead document the intended final REST behavior: I2P
peers should be returned separately from clearnet peers in an additive,
explicitly typed JSON collection. The API redesign and migration belong in the
[REST API overhaul epic #144](https://github.com/torrust/torrust-tracker/issues/144),
with an ADR agreed before implementation.

### 3.12 ⚠️ Announce statistics are not filtered by peer network

`Coordinator::peers_excluding()` correctly filters the returned peer list by
`PeerAddress::is_i2p()`, but `Coordinator::metadata()` remains aggregate across
both clearnet and I2P peers. The announce response obtains `complete` and
`incomplete` from that aggregate metadata. Consequently, an I2P requester can
receive no usable peers while being told that the swarm has clearnet seeders or
leechers (and vice versa).

This produces misleading announce statistics after network isolation. Return
network-scoped metadata using the same address-kind criterion as peer selection,
while retaining aggregate metadata for administrative/statistics APIs where it
is explicitly intended.

### 3.13 ⚠️ Malformed suffixless Destinations silently fall back to clearnet

The I2P protocol permits a Destination without the `.i2p` suffix. In
`extract_ip()`, a non-IP value is rejected only when it has that suffix. A
malformed or truncated suffixless Destination therefore returns `Ok(None)`, and
the announce service registers the requester as a clearnet peer using its
transport address. That silently crosses the network boundary the PR is meant
to enforce.

Define an unambiguous suffixless-I2P candidate rule and reject candidates that
fail I2P validation. At minimum, values that use the I2P Base64 alphabet and
are destination-sized must not silently fall back to clearnet behavior.

### 3.14 ⚠️ Destination identity is spoofable without trusted transport context

A normal HTTP request does not prove that the requester owns the I2P
Destination supplied in the `ip` parameter. An attacker can claim a victim's
Destination, receive I2P swarm information, and update or remove the victim's
peer record because the swarm is keyed by `PeerAddress`.

Serving the tracker only through I2P is not sufficient: it establishes that
_some_ I2P client made the request but not that it owns the Destination claimed
in the query. The tracker must derive identity from trusted I2P transport
context (for example, a validated server-tunnel `X-I2P-DestB64` header or a
SAMv3/I2CP adapter) and reject or ignore mismatching query values. See the
[destination spoofing analysis](destination-spoofing-analysis.md) for the
attack scenario, deployment trade-offs, and minimum secure policies.

---

## 4. Actions for the Contributor

### Required before merge

- [ ] **A1**: **URL-decode I2P Destination query values** — A valid Destination
      with percent-encoded Base64 padding (`%3D%3D`) is rejected, while the
      raw-padding form succeeds. Decode query parameter values before I2P
      Destination parsing.
  - **Implementation area**: `packages/http-protocol/src/v1/query.rs` and `packages/http-protocol/src/v1/requests/announce.rs`.
  - **Required behavior**: Decode percent-encoded query parameter names and values exactly once before protocol parsing. Do not decode a second time or change raw binary `info_hash` and `peer_id` handling.
  - **Regression test**: Restore the existing disabled
    `it_should_parse_a_percent_encoded_padded_i2p_destination_from_the_ip_param`
    test and make it pass. It must parse
    <!-- cspell:disable-next-line -->
    `"A".repeat(512) + "BQAEAAAAAA%3D%3D.i2p"` as `Some(AnnounceAddress::I2p(_))`.
  - **Keep existing coverage**: Preserve the raw-padding `==` test and add a query-level test showing `%3D` decodes to `=`.
  - **Completion criteria**: Raw and percent-encoded padding produce the same normalized `I2pDestination`; malformed percent escapes return a structured error; focused protocol tests and `linter all` pass.

- [ ] **A2**: **Deserialization fix** — `DeserializedCompactParsed` uses
      `chunks_exact(6)` and cannot parse a valid 32-byte I2P compact peer hash.
      Extend the client-side response types or explicitly separate them from
      the clearnet-only parsed compact representation.
  - **Implementation area**: `packages/http-protocol/src/v1/responses/announce/deserialization.rs` and `packages/http-protocol/src/v1/responses/announce/encoding.rs`.
  - **Required behavior**: Do not parse a 32-byte I2P hash as IPv4 peers. Represent I2P compact responses explicitly, or make the clearnet parser reject I2P-format payloads with a clear error.
  - **Regression tests**: Parse a valid 32-byte I2P compact `peers` payload into the correct I2P representation and prove it cannot be interpreted as IPv4 entries.
  - **Completion criteria**: The client-side representation has an unambiguous I2P path, invalid compact lengths do not panic or silently truncate, and IPv4/IPv6 compact tests continue to pass.

- [ ] **A3**: **Preserve I2P parse-error context** — Do not collapse
      `ParseI2pDestinationError` into generic `InvalidParam` for `.i2p`
      values. Return an I2P-specific error with the underlying reason so users
      can correct invalid Base64, insufficient length, or certificate data.
  - **Implementation area**: `packages/http-protocol/src/v1/requests/announce.rs` and `packages/primitives/src/i2p.rs`.
  - **Required behavior**: Add an I2P-specific `ParseAnnounceQueryError` variant retaining `ParseI2pDestinationError` as source and identifying `ip`. Map it to a concise failure reason without reflecting the full Destination.
  - **Regression tests**: Cover invalid I2P Base64, too-short Destination, and invalid certificate length. Assert the structured source and a useful bounded failure message.
  - **Completion criteria**: The three modes remain distinguishable, errors answer what and why, and no production `unwrap()` is added to the parsing path.

- [ ] **A4**: **Bound and redact invalid Destination input** — Set a documented
      compatibility-aware maximum I2P Destination size before Base64 decoding.
      Do not reflect the full untrusted `ip` value in the error response; return
      an I2P-specific, actionable reason instead. Add boundary tests.
  - **Implementation area**: `packages/primitives/src/i2p.rs` and announce error mapping in `packages/http-protocol/src/v1/requests/announce.rs`.
  - **Required behavior**: Check encoded input length before allocating the decoded Base64 buffer. Validate Certificate type at byte $384$, declared payload length at bytes $385$–$386$, and payload structure for supported types. Reject unsupported types and excess certificate data. The limit must allow all supported modern key types; $475$ decoded bytes is not a safe universal bound.
  - **Error behavior**: Errors may identify parameter and bounded actual/maximum lengths, but must not include the full Destination.
  - **Regression tests**: Cover NULL and every supported Key Certificate layout, unsupported type, declared-length mismatch, selected maximum, one-character overflow, and oversized-input redaction.
  - **Completion criteria**: No unbounded decode/hash occurs, only structurally valid supported Destinations are accepted, errors are bounded/actionable, and supported types/limits are documented with their specification rationale.

- [ ] **A5**: **Return network-scoped announce statistics** — `complete` and
      `incomplete` must describe peers reachable by the requesting network.
  - **Implementation area**: `packages/swarm-coordination-registry/src/swarm/coordinator.rs`, registry/repository query methods, and the tracker-core announce response assembly.
  - **Required behavior**: Derive response metadata using the same `is_i2p()` filter as `peers_excluding()`. Keep aggregate metadata only for administrative and aggregate-statistics consumers that intentionally span both networks.
  - **Regression tests**: Build a swarm with an I2P leecher and a clearnet seeder sharing one info hash. Assert that an I2P announce receives no clearnet peer and reports zero reachable seeders; assert the reciprocal clearnet case; assert same-network counts remain correct.
  - **Completion criteria**: Peer list and `complete`/`incomplete` response fields are internally consistent for both networks, without regressing aggregate scrape/management statistics.

- [ ] **A6**: **Reject malformed suffixless I2P candidates** — Do not treat a
      malformed suffixless I2P Destination as an absent `ip` parameter.
  - **Implementation area**: `packages/http-protocol/src/v1/requests/announce.rs` and `packages/http-protocol/src/v1/query.rs`.
  - **Required behavior**: Define a documented candidate rule for suffixless I2P Destinations. If a non-IP value meets that rule but fails `I2pDestination` validation, return an I2P-specific announce parse error; do not fall back to the TCP/X-Forwarded-For address.
  - **Regression tests**: Cover a valid suffixless Destination, a truncated suffixless candidate, an invalid-alphabet suffixless candidate, an ordinary non-I2P value, and a valid clearnet IP. Verify only the ordinary non-I2P value retains existing ignore behavior, if that compatibility behavior is retained.
  - **Completion criteria**: A malformed suffixless Destination cannot cause a clearnet registration or expose clearnet peers to an intended I2P announce.

- [ ] **A7**: **Eliminate Destination spoofing before enabling I2P announces** —
      Do not use an untrusted HTTP `ip` query value as an authenticated I2P peer
      identity. Select and implement one of the safe policies defined in the
      [destination spoofing analysis](destination-spoofing-analysis.md):
      trusted I2P transport identity enforcement on a dedicated listener, or
      rejection of I2P announces until enforcement exists.
  - **Current status**: Neither policy is implemented in PR #2059. The draft
    must not merge while public I2P announces remain enabled with an untrusted
    query Destination.
  - **Implementation area**: HTTP tracker listener configuration, trusted
    reverse-proxy/tunnel context extraction, and
    `packages/http-core/src/services/announce.rs`.
  - **Required behavior**: In I2P-enforced mode, derive `PeerAddress::I2p`
    from a validated trusted I2P transport identity. Reject direct/untrusted
    requests, missing or malformed identity headers, and mismatches between the
    query `ip` Destination and the trusted identity. Keep clearnet and
    I2P-enforced listeners on separate explicit policies.
  - **Regression tests**: Cover trusted and untrusted proxy sources; absent and
    malformed identity headers; matching and mismatching query Destinations;
    direct listener access; and attempted peer-record takeover.
  - **Completion criteria**: An attacker cannot obtain I2P peer information or
    update a victim peer record by claiming the victim's Destination.

### Follow-up work or documentation

- [ ] **F1**: **Query parser regression test** — The `split_once('=')` fix
      changes behavior for `name=value=value` (now accepted instead of rejected).
      Retain a test documenting this intentional change. Add a query-level
      assertion that the first `=` separates the name and value while later
      `=` characters remain part of the decoded value.

- [ ] **F2**: **Placeholder port documentation** — The I2P spec says:
      _"Clients generally include a fake port=6881 parameter... Trackers may
      ignore the port parameter, and should not require it."_ Document why
      `I2P_PLACEHOLDER_PORT` is `1` (not `6881`) and that this value is
      conventional. The documentation must state that I2P routes by Destination,
      clients must ignore the response port, and `1` exists only for legacy
      non-compact peer dictionary compatibility.

  **Follow-up numbering note**: F3 was promoted to required action A5 when the
  manual UDP check confirmed that aggregate announce statistics violate network
  isolation. The remaining follow-up identifiers preserve the review history.

- [ ] **F4**: **Tracker client I2P support** — The `tracker_client` binary
      (`console/tracker-client`) accepts `--ip` as `IpAddr` only. It cannot
      send I2P Destinations. Add a comment in the PR noting this limitation
      and track a follow-up issue to add `--i2p-destination` support to the
      client. The follow-up should accept raw or percent-encoded Destinations,
      validate through `I2pDestination`, and parse both I2P non-compact and
      compact responses without treating hashes as clearnet addresses.

- [ ] **F5**: **Define REST API I2P representation in epic #144** — Do not
      change the current `peer_addr` socket-address field to carry I2P
      Destinations. Add an ADR under the
      [REST API overhaul epic #144](https://github.com/torrust/torrust-tracker/issues/144)
      that defines the final, versioned JSON contract before implementation.
  - **Expected final behavior**: Return clearnet and I2P peers in separate,
    explicitly typed collections. An I2P entry must identify its Destination
    (for non-compact management responses) without inventing an IP address or
    a meaningful port.
  - **ADR decision points**: Resource names; versioning/migration strategy;
    whether the I2P collection is optional, paginated, or separately queried;
    authorization/privacy implications of exposing full Destinations; and
    backwards compatibility for existing REST clients.
  - **Implementation gate**: No REST I2P representation should be added until
    the ADR is accepted and the epic's API migration plan includes contract,
    adapter, endpoint, and client tests.

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

**Implication for PR #2050**: This blocks merge. Without enforcement, any
client can spoof I2P announces by passing a valid Destination in `ip`.
Implement the minimum secure policy described in the
[destination spoofing analysis](destination-spoofing-analysis.md) before
enabling I2P announces.

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

**Spec answer**: The UDP-over-I2P specification was finalized in 2025-06. It
is separate from BEP 15 and defines its own
[UDP announce protocol](https://i2p.net/en/docs/specs/udp-announces).

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
| 2026-08-17 | Jose Celano + AI agent | `review-pass-1.md` (this file) | Findings carried to draft PR #2059                  |
| 2026-08-17 | Jose Celano + AI agent | `review-pass-1.md` §5          | Spec cross-reference complete (I2P BitTorrent spec) |

### Review Work Tracker

| ID  | Review activity                                 | Status           | Evidence / next step                                                                                                                           |
| --- | ----------------------------------------------- | ---------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- |
| R1  | HTTP I2P announce and response behavior         | Complete         | `manual-test-evidence.md` Tests 1–7                                                                                                            |
| R2  | Cross-network isolation and compact wire format | Complete         | I2P and clearnet peers are filtered by address kind; compact I2P output is 32-byte hashes                                                      |
| R3  | Percent-encoded Base64 padding                  | Finding recorded | A disabled regression test is in `packages/http-protocol/src/v1/requests/announce.rs`; restore it while implementing URL decoding              |
| R4  | I2P parse-error handling                        | Complete         | Findings 3.9–3.10 / actions A3–A4 record lost parse context, unbounded input, and full-value echoing                                           |
| R5  | I2P Destination validation and resource limits  | Complete         | Common-structures audit found missing certificate-type/payload validation and no pre-decode bound; A4 defines compatibility-aware requirements |
| R6  | REST API representation of I2P peers            | Complete         | Finding 3.11 / F5 defers final typed, separate I2P REST collections to REST API overhaul epic #144 and its ADR                                 |
| R7  | Copilot review suggestions from original PR     | Complete         | Two additional valid blockers recorded as findings 3.12–3.13 / actions A5–A6; the compact parser suggestion duplicates A2                      |
| R8  | I2P Destination spoofing threat model           | Complete         | Finding 3.14 / action A7 and `destination-spoofing-analysis.md` define the minimum secure policies before merge                                |
| R9  | Contributor response and fix verification       | Pending          | Re-run focused tests, manual evidence, and `linter all` after updates                                                                          |

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
| Enforcement headers      | `X-I2P-DestHash`, `X-I2P-DestB64`, `X-I2P-DestB32` (I2PTunnel-added) | ⚠️ Required before merge (A7)                       |
| Cross-network prevention | "Reject standard network announces... not deliver them in responses" | ✅ Implemented through swarm address-kind filtering |
| UDP announce             | Separate UDP-over-I2P specification finalized 2025-06                | ✅ HTTP-only scope is correct                       |
| PEX                      | Extension message `i2p_pex` (32-byte hashes)                         | ❌ Not in scope                                     |
| DHT                      | Extension message `i2p_dht`, 54-byte compact node info               | ❌ Not in scope                                     |
| SAMv3                    | Recommended for non-Java clients; `SIGNATURE_TYPE=7` (Ed25519)       | N/A (tracker-side only)                             |
