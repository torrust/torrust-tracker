---
semantic-links:
  pr: "https://github.com/torrust/torrust-tracker/pull/2050"
  i2p-bittorrent-spec: "https://i2p.net/en/docs/applications/bittorrent/"
  i2p-samv3: "https://i2p.net/en/docs/api/samv3"
  related-artifacts:
    - packages/http-core/src/services/announce.rs
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/primitives/src/i2p.rs
    - docs/pr-reviews-manual/pr-2050/i2p-addressing-primer.md
---

# I2P Destination Spoofing Analysis — PR #2050

This document records the destination-identity threat model for the I2P peer
support proposed in PR #2050, evaluates deployment options, and defines the
minimum requirements before the feature can be merged.

It complements the [I2P addressing primer](i2p-addressing-primer.md) and the
main [review report](review-pass-1.md).

---

## 1. Problem statement

The current implementation accepts an I2P Destination in the HTTP announce
request's `ip` query parameter and uses it as the peer identity:

```text
GET /announce?...&ip=<claimed Destination>.i2p
```

A normal HTTP request does not cryptographically prove that the requester owns
that Destination. Any client that can reach the listener can claim another
peer's valid Destination.

```text
Attacker ── HTTP announce ──► Torrust Tracker
           ip=<victim Destination>.i2p
```

This is **Destination spoofing**. It is distinct from forging an Internet
source IP, but it has the same essential property: an untrusted client controls
the identity recorded by the tracker.

---

## 2. Concrete attack scenario

Assume an I2P swarm contains Alice, Bob, and Carol.

1. Alice announces her valid Destination as a seeder.
2. Mallory sends an announce with Alice's Destination in the `ip` parameter.
3. The tracker accepts Mallory's supplied value as `PeerAddress::I2p(Alice)`.
4. The tracker can return I2P peer information to Mallory because Mallory now
   appears to belong to the I2P swarm.
5. Mallory can update Alice's tracker record, submit a `stopped` event, or
   report manipulated uploaded/downloaded/left counters.

Potential consequences:

- disclosure of I2P peer Destinations or compact Destination hashes to an
      unauthorized requester;
- peer-record takeover because the swarm is keyed by `PeerAddress`;
- manipulation of peer availability, announces, and swarm statistics;
- unexpected connection attempts toward an impersonated peer;
- bypass of intended I2P/clearnet separation through a forged identity.

Authentication can restrict _who_ may announce, but it does not prove that an
authenticated user owns the Destination they supply.

---

## 3. Why an I2P-only listener is insufficient

Restricting a tracker listener to traffic arriving through I2P improves privacy
but does not, by itself, bind the `ip` parameter to the sender.

```text
Some I2P client ── I2P connection ──► I2P-only tracker listener
                                      ip=<victim Destination>.i2p
```

The tracker knows that an I2P client made the request, but not that it owns the
Destination stated in the query. A spoofing-safe deployment needs a
**transport-derived identity**, not merely an I2P-only transport path.

| Deployment                                      | I2P traffic only | Sender Destination authenticated | Spoofing prevented            |
| ----------------------------------------------- | ---------------- | -------------------------------- | ----------------------------- |
| Public HTTP + query `ip`                        | No               | No                               | No                            |
| I2P-only HTTP + query `ip`                      | Yes              | No                               | No                            |
| I2P server tunnel + trusted Destination headers | Yes              | Yes                              | Yes, if configured correctly  |
| SAMv3/I2CP transport adapter                    | Yes              | Yes                              | Yes, if implemented correctly |

---

## 4. Recommended enforcement architecture

Torrust should own the authorization decision. An I2P component supplies a
trusted transport identity; the tracker validates the source and applies its
announce policy.

```text
I2P client
   │
   ▼
I2P network
   │
   ▼
Trusted I2P server tunnel / I2P-aware proxy
   │ injects or overwrites X-I2P-Dest* headers
   ▼
Loopback-only Torrust I2P listener
   │ validates trusted source and identity context
   ▼
Torrust announce application service
```

A tunnel implementation can provide headers such as:

```text
X-I2P-DestB64: <full Base64 Destination>
X-I2P-DestHash: <Destination hash>
X-I2P-DestB32: <Base32 hash>.b32.i2p
```

These headers are meaningful only when the request originates from a configured
trusted I2P tunnel/proxy. A public listener must never trust client-provided
versions of these headers.

### Identity rule

When enforcement is required:

```text
peer identity = validated Destination from trusted I2P transport context
```

The tracker must either ignore the announce `ip` Destination or require it to
exactly match the trusted Destination.

---

## 5. Deployment modes and trade-offs

### Option A: Do not accept I2P announces until enforcement exists

```text
Public HTTP listener only
I2P Destination in query `ip` -> rejected
```

| Benefits                               | Costs                               |
| -------------------------------------- | ----------------------------------- |
| No spoofing path                       | Delays I2P peer support             |
| Simple and explicit security model     | Does not provide compatibility mode |
| No tunnel/proxy deployment requirement |                                     |

This is the safest default for an open/public tracker.

### Option B: Compatibility mode with unverified Destinations

```text
Public HTTP listener
I2P Destination in query `ip` -> accepted as unverified
```

| Benefits                                   | Costs                                                |
| ------------------------------------------ | ---------------------------------------------------- |
| Enables basic interoperability immediately | Destination spoofing remains possible                |
| Works with existing HTTP clients           | Can disclose or manipulate I2P swarm state           |
| No I2P tunnel configuration                | Must never be presented as authenticated I2P support |

This option is not suitable for an open/public tracker unless access is
strictly controlled and the risk is explicitly accepted.

### Option C: I2P server tunnel with header enforcement

```text
I2P server tunnel -> loopback-only Torrust listener
trusted X-I2P-Dest* header -> enforced identity
```

| Benefits                                      | Costs                                      |
| --------------------------------------------- | ------------------------------------------ |
| Strong practical identity binding             | Requires an I2P router/tunnel deployment   |
| Tracker can be reachable as a `.i2p` service  | Requires trusted-source configuration      |
| Does not require Torrust to implement routing | Direct listener exposure must be prevented |

This is the recommended first secure deployment mode.

### Option D: SAMv3 or I2CP transport adapter

```text
Torrust adapter <-> local I2P router via SAMv3/I2CP
```

| Benefits                                        | Costs                                               |
| ----------------------------------------------- | --------------------------------------------------- |
| Direct transport-derived identity context       | New transport adapter and operational complexity    |
| Strong integration with tracker request context | Requires I2P router dependency and lifecycle design |
| Can avoid proxy-header trust model              | Larger future implementation                        |

This is a strong long-term design, but it is separate from the peer-address
model proposed by PR #2050.

---

## 6. Minimum requirements before merge

The initial I2P peer-support feature must not expose I2P swarm membership based
solely on an untrusted claimed Destination. Before merging, choose and implement
one of the following safe policies:

### Required policy choice

- [ ] **Policy 1 — Enforced I2P transport identity**: Support I2P announces
      only on a dedicated trusted listener. Require trusted I2P transport identity
      headers or an equivalent SAMv3/I2CP identity context.
- [ ] **Policy 2 — Disable I2P announces**: Reject I2P Destinations until
      transport-derived identity enforcement is available.

Do not merge a public compatibility mode that treats a query parameter as an
authenticated I2P identity.

### Required implementation points for Policy 1

- [ ] Add per-listener configuration identifying I2P-enforced mode and trusted
      tunnel/proxy sources.
- [ ] Reject direct requests and untrusted `X-I2P-Dest*` headers on an
      I2P-enforced listener.
- [ ] Parse and validate the trusted full Destination header.
- [ ] Derive `PeerAddress::I2p` from the trusted transport context, not from an
      untrusted query parameter.
- [ ] Reject an `ip` Destination that differs from the trusted identity, or
      ignore the query value entirely.
- [ ] Keep public clearnet listeners on a distinct policy and reject I2P
      identity headers there.
- [ ] Add contract tests for trusted source, untrusted source, missing headers,
      malformed headers, matching/mismatching `ip` values, and direct-listener
      access.
- [ ] Document an operational deployment where the I2P forwarding listener is
      loopback-only and cannot be reached directly from the public Internet.

---

## 7. Future architecture decision

The destination-enforcement design affects tracker listener configuration,
reverse-proxy trust, authentication, request context, observability, and
network isolation. It requires an ADR before implementation.

The ADR should decide:

1. Whether the first secure integration is a trusted I2P server tunnel, a
   SAMv3 adapter, or both.
2. The per-listener mode and trusted-source configuration model.
3. Header handling, normalization, precedence, and mismatch policy.
4. How authentication and Destination ownership interact.
5. Logging/redaction requirements for Destinations.
6. Listener/network separation and migration behavior.
7. Test/deployment requirements for secure operation.

---

## 8. Recommendation for PR #2050

Keep PR #2050 as a draft until destination spoofing is addressed by one of the
safe policy choices in Section 6. This is not merely a future enhancement: it
is the trust boundary that determines whether the tracker records verified I2P
peer identities or attacker-controlled claims.
