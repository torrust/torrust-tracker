---
semantic-links:
  pr: "https://github.com/torrust/torrust-tracker/pull/2050"
  i2p-bittorrent-spec: "https://i2p.net/en/docs/applications/bittorrent/"
  i2p-common-structures-spec: "https://i2p.net/en/docs/specs/common-structures"
  related-artifacts:
    - packages/primitives/src/i2p.rs
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-protocol/src/v1/responses/announce/encoding.rs
---

# I2P Addressing Primer for PR #2050

This note explains the I2P address forms relevant to the Torrust Tracker I2P
peer-support review. It is a review aid, not a replacement for the official
[I2P BitTorrent specification](https://i2p.net/en/docs/applications/bittorrent/)
or the [I2P common-structures specification](https://i2p.net/en/docs/specs/common-structures).

---

## 1. Why I2P does not use an IP address and port

A clearnet BitTorrent peer is normally identified by a socket endpoint:

```text
203.0.113.42:6881
```

An I2P peer is identified by a **Destination**. A Destination is a public,
self-contained I2P endpoint identity. I2P routing delivers messages to that
Destination; a TCP-style peer port is not part of I2P peer addressing.

For BitTorrent tracker compatibility, I2P clients commonly send a fake
`port=6881` in an announce request. Trackers may ignore it, and I2P clients
must ignore the `port` field returned in a non-compact peer dictionary.

This is why the PR introduces a domain-level distinction:

```text
PeerAddress
├── Clearnet(SocketAddr)  -> IP address + port
└── I2p(I2pPeerAddress)   -> Destination only
```

---

## 2. Destination binary structure

An I2P Destination is a `KeysAndCert` structure:

```text
+------------------------------------+-------------------------+
| 384 bytes: key material / padding  | Certificate: 3+ bytes   |
+------------------------------------+-------------------------+
```

It is at least **387 bytes** long. The Certificate starts at byte 384 and is:

```text
+-----------+---------------------+---------------------------+
| type: 1 B | payload length: 2 B | payload: declared length  |
+-----------+---------------------+---------------------------+
```

The full Destination length must therefore be:

$$
384 + 1 + 2 + \text{certificate payload length}
$$

The PR verifies this total-length relation. A full implementation must also
validate the certificate type and its permitted payload structure/size; the
I2P specification warns implementers not to accept excess Certificate data.

---

## 3. Full Base64 Destination

Trackers receive the full Destination in the announce request's `ip`
parameter. It uses the **I2P Base64 alphabet**:

```text
ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-~
```

This differs from standard Base64 in the final two symbols (`-~` instead of
`+/`). A minimum 387-byte Destination becomes at least approximately 516
Base64 characters. I2P BitTorrent clients **must append `.i2p`** for older
tracker compatibility; trackers should also accept the suffixless form.

Example shape only (not a usable real-world Destination):

```text
<!-- cspell:disable-next-line -->
AAAA...AAAA BQAEAAAAAA==.i2p
└──── Base64 Destination ────┘
```

The `==` is ordinary Base64 padding. In an HTTP query value it may be
percent-encoded as `%3D%3D`. A tracker must percent-decode query values before
interpreting the I2P Base64 Destination.

---

## 4. Destination hash and `.b32.i2p` address

The compact tracker response does not return each full Destination. It returns
its 32-byte SHA-256 hash:

```text
destination hash = SHA-256(binary Destination)
```

For example, pseudocode equivalent to the PR's calculation is:

```rust
let binary_destination = decode_i2p_base64(full_destination)?;
let destination_hash: [u8; 32] = Sha256::digest(binary_destination).into();
```

A human-usable I2P Base32 address is derived from the same hash:

```text
lowercase-base32(SHA-256(binary Destination)).b32.i2p
```

The Base32 address is a lookup name, not the full Destination. A client that
only has a compact-response hash must convert it to the `.b32.i2p` form and
query I2P's naming service before connecting. A client that receives a full
Destination in a non-compact response should use it directly.

---

## 5. Tracker announce and response formats

### Announce request

```text
GET /announce?...&ip=<full I2P Base64 Destination>.i2p&port=6881&compact=1
```

The tracker should:

1. Percent-decode the HTTP query value exactly once.
2. Parse it as a clearnet IP first when applicable.
3. Parse a valid I2P Destination when it is an I2P candidate.
4. Reject malformed I2P candidates rather than silently registering them as
   clearnet peers.

### Non-compact response

I2P peers are returned as ordinary peer dictionaries, but `ip` contains the
full Base64 Destination and `port` is only a compatibility placeholder:

```text
{
  "peer id": <20 bytes>,
  "ip": "<full Destination>.i2p",
  "port": 1
}
```

### Compact response

The `peers` byte string contains concatenated 32-byte Destination hashes:

```text
+--------------------------------+--------------------------------+
| SHA-256(Destination peer 1)    | SHA-256(Destination peer 2)    |
| 32 bytes                       | 32 bytes                       |
+--------------------------------+--------------------------------+
```

I2P compact entries are not IPv4's 6-byte `IP + port` entries and must not be
parsed by an IPv4 compact-peer decoder.

---

## 6. Why Torrust Tracker should support this

Supporting I2P lets the tracker serve BitTorrent swarms whose participants use
the I2P anonymity network. This requires protocol-aware support rather than
just accepting a long string in the `ip` field:

- **Correct matching**: I2P peers need full Destinations or compact hashes of
  other I2P peers.
- **Network separation**: clearnet peers must not receive I2P peers, and I2P
  peers must not receive clearnet peers.
- **Accurate statistics**: announce peer lists and swarm counts must describe
  peers reachable by the requester.
- **Safety**: Destination input is public/untrusted HTTP input, so it must be
  bounded, structurally validated, and not reflected in full in error messages.
- **Future API clarity**: management APIs should not ambiguously serialize a
  Destination as if it were a socket address.

PR #2050 establishes the core domain model and HTTP tracker path. The manual
review documents the remaining protocol, input-safety, response-decoding, and
API-contract work needed before it is merge-ready.
