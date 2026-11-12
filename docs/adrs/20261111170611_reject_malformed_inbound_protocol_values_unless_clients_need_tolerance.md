---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/issues/open/2842-inbound-tolerance-adr/ISSUE.md
    - docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md
    - docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/adrs/20260617093046_reject_wildcard_external_ip.md
    - docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md
    - docs/adrs/20260716000000_accept_only_ip_addresses_in_http_announce_ip_param.md
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/udp-protocol/src/request.rs
    - packages/primitives/src/peer.rs
    - docs/issues/open/2850-non-negative-number-of-bytes/ISSUE.md
    - docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/ISSUE.md
    - docs/packages.md
---

<!-- skill-link: create-adr -->

# Reject Malformed Inbound Protocol Values Unless Real Clients Need the Tolerance

## Scope

Repository-level. The decision applies to every delivery protocol that receives client requests
(`udp-protocol`, `udp-server`, `http-protocol`, `axum-http-server`) and to the domain types in
`primitives` that those protocols feed, so it belongs in `docs/adrs/`.

## Description

BitTorrent is decentralized and has no conformance authority. Implementations that accept the
widest range of client behavior interoperate with the most clients ("be liberal in what you
accept"). A tracker that is liberal everywhere, however, hides client bugs, lets different delivery
protocols disagree about the same value, and pushes values into the domain that its code never
considered.

The first case found is the announce `left` field. BEP 15 encodes it as a 64-bit integer,
which the implementation document it cites types as `int64_t`, and the UDP announce accepts a
negative value. The HTTP announce parses `left` as `u64` and rejects
it. `Peer::is_seeder` (`packages/primitives/src/peer.rs`) treats `left <= 0` as a seeder, so a
negative value from a UDP client counts the peer as a seeder (the `complete` count), while the same
value over HTTP is an error. Nobody decided this; it follows from the parsers being written independently.

The existing ADRs cover the other direction. [Bound protocol-agnostic values by the tightest
delivery protocol](20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md)
is about values the tracker sends. The [value-invariant
ADR](20260723184019_separate_configuration_value_invariants_from_consistency_validation.md) and the
[newtype ADR](20260721100000_use_newtypes_for_constrained_configuration_field_types.md) say how to
reject a bad configuration value. None of them says what to do with a value a client sends.

## Agreement

1. **Strict by default.** A value a client sends that its protocol specification does not allow is
   rejected with the protocol's normal error response (a BEP 15 error packet over UDP, a bencoded
   `failure reason` over HTTP). It is not clamped, defaulted, or silently ignored.

2. **Judge each field on its own.** A value that looks unusual may be legitimate: BEP 15
   `num_want = -1` means "use the default", and `left`, `downloaded` and `uploaded` are 64-bit
   integers in BEP 15, and the sources do not document why they are signed. The question for each field is what its
   specification allows, not whether the value is negative or large. "Reject all negatives" is
   not the rule.

3. **Tolerate only with recorded evidence.** The tracker tolerates a malformed value only when
   there is evidence that a meaningful share of real clients relies on it. Evidence is one of: a
   named client and version that sends the value (an issue report, a packet capture, or the
   client's source), or the rejection metric described in point 5. A tolerance without evidence is
   not added, and a "might break some client" argument alone is not evidence.

4. **Record every tolerance in code and in this ADR.** A tolerance is one row in the register
   below, and its code carries a comment that links to this ADR. The row says the field, the
   protocols, the tolerated values, what they are normalized to, the evidence, the metric, and
   when to reconsider it. Adding or removing a row is an amendment to this ADR and goes through a
   PR like any other change.

5. **Make the decision measurable.** The tracker counts every request rejected for a malformed
   inbound value, labeled by delivery protocol, field and a bounded client family, so the number
   of affected real clients can be read before a tolerance is added. Every tolerance has its own
   counter, emitted as an objective event per [Events Are Objective Facts](20260727000000_events_are_objective_facts.md) (or, where a metric is not feasible, a rate-limited log) so the register can be
   reviewed with data and a tolerance that no client uses is removed.

6. **Implement the public specification strictly; refuse at the first project-owned type.**
   Following a public protocol specification faithfully has priority over failing fast. A type
   that mirrors a BEP keeps the shape the BEP gives it and is not narrowed, because the
   specification is not our decision and its reasons may be unknown. The wire parser therefore
   accepts what the BEP type allows. A value the specification allows but the domain does not is
   refused at the first conversion into a type the project owns (application or domain layer),
   as issue #2850 plans for a negative `left` with a fallible conversion in `udp-core`. The malformed value
   never reaches `primitives` or `tracker-core`. A tolerance normalizes to a value the domain
   already accepts. Domain types protect their invariants with newtypes and constructors (for
   example a non-negative number of bytes) and carry no special cases for malformed input. See
   [Keep protocol and domain types decoupled](20260527175600_keep_protocol_and_domain_types_decoupled.md).

7. **The protocols agree on the verdict for a value both wire formats can express.** UDP and
   HTTP must not disagree about whether such a value is accepted. A range that only one wire
   format can express follows that protocol's own specification: the protocols may differ at the
   wire level and are equal in the domain, as in [Cap scrape info hashes per
   protocol](20261005124222_cap_scrape_info_hashes_per_protocol.md). Rejecting a malformed value
   is not by itself a reason to ban the client; banning stays with the rules of the UDP ban
   service. Each field has a parity test that sends the same values through both protocols up to the
   point where the request is accepted or refused, which includes the wire-to-domain conversion
   and not only the parser.

8. **Validate each kind of rule in the layer that owns it, as early as possible.** Syntax is
   validated in the delivery layer (the wire parser), application invariants in the service, and
   domain invariants in the domain. Public protocol packages follow
   [Package Conventions](../packages.md#package-conventions), principle 2: `*-protocol`
   packages strictly implement BEP specifications.

9. **Protect invariants with constructors or newtypes, not standalone validation functions.** A
   value that must hold an invariant has a type whose constructor enforces it, so a valid value
   cannot be bypassed and the check is not repeated by every caller.

### First application: byte counts

Announce `left`, `downloaded` and `uploaded` apply the rules above. This records the intended
design; the code change is tracked in issue #2850
([specification](../issues/open/2850-non-negative-number-of-bytes/ISSUE.md)).

- Domain `NumberOfBytes` (in `primitives`) is a private `u64` with no upper bound.
- UDP wire type `NumberOfBytes(I64)` and `Request::parse_bytes` stay unchanged. A negative value
  is refused in a fallible `peer_builder::from_request` in `udp-core`; its error is wrapped by
  `UdpAnnounceError`. No standalone validation function is added: PR #2839, which proposed one,
  was closed without merging in favour of this design.
- The HTTP protocol type becomes a private `u64` and its `i64::MAX` limit
  (`NumberOfBytesOverflow`) is removed, because the limit only existed to fit the old shared
  `i64` type. HTTP then accepts `0..=u64::MAX` and UDP `0..=i64::MAX`; the domain is the same for
  both.
- REST resource fields for byte counts are `u64`.

Research behind the signed UDP fields: BEP 15 and the XBT Tracker document say only "64-bit
integer" for these fields, never signed or unsigned. The libtorrent document (cited by BEP 15 as
an existing implementation) uses `int64_t` for the byte counts, but `uint32_t` for `ip` and `key`
and `uint16_t` for `port`, so unsigned types exist in the format. BEP 3 says only "encoded in
base ten ascii". No document says why the counts are signed or what a tracker must do with a
negative value. The "Evidence on real clients and other trackers" section of the
[#2838 specification](../issues/open/2838-1347-udp-reject-negative-announce-bytes/ISSUE.md) shows that, before the metadata is known, libtorrent sends `left = 16384`
and Transmission sends `left = i64::MAX`; neither sends a negative value.

### Register of tolerances

| Field | Protocols | Tolerated values | Normalized to | Evidence | Metric | Reconsider when |
| ----- | --------- | ---------------- | ------------- | -------- | ------ | --------------- |
| _None yet_ | | | | | | |

### Existing leniencies (inventory, not yet applied to this rule)

These were found while writing this ADR. They predate it, so none has recorded evidence or a
metric. Each is to be kept (and registered with its evidence), changed to reject, or aligned across
protocols in its own issue; this ADR does not change any of them.

| Field | UDP | HTTP | Note |
| ----- | --- | ---- | ---- |
| Announce `left`, `downloaded`, `uploaded` | accepts negative (signed 64-bit) | rejects negative and above `i64::MAX` (the limit exists only to fit an `i64` type) | First application: bug #2838; UDP still accepts it; the planned change is issue #2850 |
| Announce `port` = 0 | rejected by the parser | no zero check found in the parser (to verify) | Possible disagreement. BEP 15 does not forbid port 0, so under rule 6 this parser check deserves a review. The event check (`0..=3`) mirrors BEP 15 |
| Announce `num_want` / `numwant` | any `i32`; zero or negative means as many as possible, a positive value is capped at the tracker limit (`PeersWanted::from_client_request`) | `u32` only, so `-1` is rejected; `0` asks for no peers (`PeersWanted::only`) | `-1` is legitimate for UDP (BEP 15 default). UDP is lenient for values below `-1`. Zero means different things on the two protocols. HTTP `-1` is unspecified |
| Announce event outside the defined set | rejected | rejected | Consistent |
| Announce `ip` that is not an IP address | field ignored always | ignored; connection IP used | Recorded in [the `ip` ADR](20260716000000_accept_only_ip_addresses_in_http_announce_ip_param.md); no metric |
| Scrape above the info-hash cap | extra hashes dropped | extra hashes dropped | Recorded in [the scrape cap ADR](20261005124222_cap_scrape_info_hashes_per_protocol.md); no metric |

## Alternatives Considered

### Tolerant by default

Accept and normalize anything that can be interpreted. Rejected: it maximizes compatibility but
hides client bugs, discourages good implementations, and has already produced a protocol
disagreement and a domain type that must handle values it should never receive.

### Strict by default with no exceptions

Rejected. A widely deployed client that cannot be fixed would be locked out of a tracker, which
defeats a tracker's purpose. The evidence rule keeps the choice open without making tolerance the
default.

### Per-protocol policy

Let each delivery protocol decide its own tolerance. Rejected: it is how the `left` disagreement
arose, and one swarm served over several protocols should not depend on which one a client used.

### Make tolerance a configuration option

Rejected for now. An operator switch multiplies the combinations to test and does not produce the
evidence this ADR asks for. It can be reconsidered for one specific tolerance if the evidence
shows operators disagree.

### Normalize in the domain instead of the parser

Rejected. It spreads protocol knowledge into `primitives`, against [Keep protocol and domain types
decoupled](20260527175600_keep_protocol_and_domain_types_decoupled.md), and the same value would
need handling in every consumer.

## Consequences

- **Positive**: one explicit, reviewable rule for every inbound field; UDP and HTTP stop
  disagreeing by accident.
- **Positive**: the domain can rely on valid values, so `is_seeder` and similar code stay simple.
- **Positive**: tolerance decisions are driven by data (the rejection counters) and can be undone.
- **Negative**: a client with a malformed value is rejected until someone shows evidence and a
  tolerance is registered, so the maintainers will occasionally get a compatibility bug report.
- **Negative**: the rejection counter and per-tolerance metrics are new work, and the client-family
  label needs a bounded cardinality.
- **Negative**: the existing leniencies in the inventory are inconsistent with the rule until each
  is handled.

## Affected Code

- [`packages/http-protocol/src/v1/requests/announce.rs`](../../packages/http-protocol/src/v1/requests/announce.rs):
  HTTP announce parsing.
- [`packages/udp-protocol/src/request.rs`](../../packages/udp-protocol/src/request.rs): UDP request
  parsing.
- [`packages/primitives/src/peer.rs`](../../packages/primitives/src/peer.rs): `Peer::is_seeder`.

This ADR changes no code. Follow-up issues apply it field by field.

## Date

2026-11-11

## References

- [Issue #2842](https://github.com/torrust/torrust-tracker/issues/2842) — this ADR
- [EPIC #1347](https://github.com/torrust/torrust-tracker/issues/1347) — package testing overhaul, where the `left` finding came up
- [BEP 3: The BitTorrent Protocol Specification](https://www.bittorrent.org/beps/bep_0003.html)
- [BEP 15: UDP Tracker Protocol](https://www.bittorrent.org/beps/bep_0015.html)
- [Issue #2850](https://github.com/torrust/torrust-tracker/issues/2850) — first application, the design recorded above; it fixes [issue #2838](https://github.com/torrust/torrust-tracker/issues/2838). [PR #2839](https://github.com/torrust/torrust-tracker/pull/2839) was the first, tactical attempt: reject a negative `left`, `uploaded` or `downloaded` in a UDP announce
- [Events Are Objective Facts](20260727000000_events_are_objective_facts.md) — how rejections and tolerances are reported
- [Reject wildcard IPs as invalid `external_ip` values](20260617093046_reject_wildcard_external_ip.md) — the same fail-fast stance for configuration
- [Use Exact IP Counters for UDP Banning](../../packages/udp-core/docs/adrs/20260829204258_use_exact_ip_counters_for_udp_banning.md) — ban rules, independent of this decision
- [Bound protocol-agnostic values by the tightest delivery protocol](20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md)
- [Separate Configuration Value Invariants from Consistency Validation](20260723184019_separate_configuration_value_invariants_from_consistency_validation.md)
- [Use Newtypes for Domain-Constrained Configuration Field Types](20260721100000_use_newtypes_for_constrained_configuration_field_types.md)
