---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p2
epic: null
github-issue: 2850
spec-path: docs/issues/open/2850-non-negative-number-of-bytes/ISSUE.md
branch: "2850-non-negative-number-of-bytes-spec"
related-pr: null
last-updated-utc: "2026-11-12 20:29"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/primitives/src/number_of_bytes.rs
    - packages/primitives/src/peer.rs
    - packages/udp-protocol/src/request.rs
    - packages/udp-protocol/src/common.rs
    - packages/udp-core/src/peer_builder.rs
    - packages/udp-core/src/services/announce.rs
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-core/src/services/announce.rs
    - packages/rest-api-runtime-adapter/src/v1/conversion.rs
    - packages/rest-api-protocol/src/v1/context/torrent/resources/peer.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/handlers/announce.rs
    - packages/udp-server/src/handlers/error.rs
    - packages/http-core/benches/helpers/util.rs
    - packages/axum-http-server/src/lib.rs
    - docs/packages.md
    - deny.toml
    - docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
    - docs/adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md
    - docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md
    - docs/adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md
    - docs/adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - .github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md
    - docs/issues/open/2824-1347-add-tests-primitives/ISSUE.md
    - docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/ISSUE.md
    - docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/manual-verification-evidence.md
    - docs/issues/open/2842-inbound-tolerance-adr/ISSUE.md
    - docs/adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
    - docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md
    - docs/issues/closed/1835-1669-14-decouple-http-protocol-from-tracker-primitives/ISSUE.md
    - docs/issues/closed/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md
    - docs/issues/closed/2454-1669-mark-public-error-enums-non-exhaustive/ISSUE.md
    - docs/issues/closed/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md
    - docs/issues/drafts/generalize-error-events/EPIC.md
    - docs/issues/drafts/remove-source-locations-from-errors/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2850 - Make the number of bytes non-negative by construction

## Goal

A negative number of bytes cannot exist inside the tracker. The domain type
`torrust_tracker_primitives::NumberOfBytes` and the HTTP protocol type cannot hold one, each
delivery protocol refuses one as early as its public specification allows, and no standalone
validation function is needed to keep the rule.

This issue is the fix for the bug of issue #2838, at its root. The general rules behind it are
recorded in the [ADR of issue #2842](../../../adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md) (see D5), so the next field with the same problem is
solved the same way.

## Background

Issue #2838 found that a UDP announce with a negative `left` is accepted and counted as a seeder,
while the HTTP tracker refuses the same value
([specification of #2838](../2838-1347-udp-reject-negative-announce-bytes/ISSUE.md)). PR #2839 proposed a tactical patch: a
`validate_number_of_bytes` function in `packages/udp-core/src/services/announce.rs` that returns
`UdpAnnounceError::NegativeNumberOfBytes`. The maintainer decided to fix the root cause instead, so
that patch was not merged and the bug is still on `develop`. The root cause:

- **The domain type allows the invalid value.** `NumberOfBytes` is `pub struct NumberOfBytes(pub i64)`
  with an infallible `new` (`packages/primitives/src/number_of_bytes.rs`). Any code can build a
  negative count.
- **Domain logic compensates for it.** `Peer::is_seeder` is `left.0 <= 0 && event != Stopped`, in
  three places in `packages/primitives/src/peer.rs` (lines 217, 243 and 270). The `<= 0` is how a
  negative `left` became a seeder.
- **A validation function does not remove the cause.** With the patch, nothing in the types would
  force a new caller of `peer_builder::from_request` to call `validate_number_of_bytes` first, and
  the value would be refused beside the wire-to-domain conversion, not by it.
- **HTTP has the same weakness in its type.** `packages/http-protocol` parses a `u64`, but stores it
  in its own `NumberOfBytes(pub i64)`. The parser protects the rule; the type does not.

The maintainer's rule: fail as soon as possible, in layers. The delivery layer validates syntax,
the service validates application invariants, and the domain validates domain invariants.
Invariants are protected by constructors or newtypes, not by standalone validation functions.

One rule has priority over failing fast (maintainer, 2026-11-12): **a public protocol specification
is implemented strictly.** A type that mirrors a BEP keeps the shape the BEP gives it, also when it
can then carry a value the domain considers invalid. The specification is not the project's
decision, and the reasons behind it may be unknown. In that case the value is refused at the first
conversion into a type the project owns, which is the earliest point the project controls. This
extends a principle that [docs/packages.md](../../../packages.md) already states: "`*-protocol`
packages strictly implement BEP specifications."

### Current state (audit on `develop`, 2026-11-12)

`grep -rn NumberOfBytes packages` finds 34 Rust files. They fall into these groups:

| Group | Files | What they do |
| --- | --- | --- |
| Domain type | `primitives/src/number_of_bytes.rs`, `primitives/src/lib.rs` | Definition and re-export |
| Domain logic | `primitives/src/peer.rs` | Three `is_seeder` copies read `left.0 <= 0`; `ser_number_of_bytes` serializes with `serialize_i64`; a test builder takes `left: i64` |
| UDP wire type | `udp-protocol/src/common.rs`, `announce.rs`, `request.rs` | `NumberOfBytes(pub I64)`, a zero-copy map of the BEP 15 packet. Must stay `I64` |
| UDP wire to domain | `udp-core/src/peer_builder.rs` | `from_request` copies the three counters with `.0.get()`; infallible |
| UDP service | `udp-core/src/services/announce.rs` | No check on `develop`. PR #2839 (not merged) proposed: `fn validate_number_of_bytes`, called before `from_request`; `UdpAnnounceError::NegativeNumberOfBytes { field: ByteCountField, value: i64 }` with the message ``Invalid announce request: `{field}` must not be negative, got {value}``; `pub enum ByteCountField { Left, Uploaded, Downloaded }` |
| HTTP protocol type | `http-protocol/src/v1/requests/announce.rs` | Its own `NumberOfBytes(pub i64)` (lines 34 to 47); `extract_number_of_bytes_from_param` parses `u64`, then refuses values above `i64::MAX` with `NumberOfBytesOverflow` (lines 598 to 620); the builder methods `with_downloaded`, `with_uploaded` and `with_left` take a `u64` and panic above `i64::MAX` (lines 478 to 507); the query building reads `.0` (lines 366 to 374) |
| HTTP protocol to domain | `http-core/src/services/announce.rs` | `peer_from_request` copies with `.0` (lines 252 to 254) |
| REST API | `rest-api-runtime-adapter/src/v1/conversion.rs` | Reads `.0` into the resource fields `uploaded`, `downloaded`, `left`, which are `i64` (`rest-api-protocol/.../resources/peer.rs`) |
| Benchmarks | `http-core/benches/helpers/util.rs` | Reads `.0` to build a protocol request from a peer |
| Tests, fixtures, docs | The other files, in `tracker-core`, `swarm-coordination-registry`, `torrent-repository-benchmarking`, `udp-core`, `udp-server`, `axum-http-server`, `http-core` | About 130 calls of `NumberOfBytes::new(<literal>)` with `0`, `1`, `10`, `1000` and similar |

Public `.0` access to the domain type, outside tests: `primitives/src/peer.rs` (4 places),
`rest-api-runtime-adapter/src/v1/conversion.rs` (3) and `http-core/benches/helpers/util.rs` (3).

Values accepted from clients today: UDP accepts any `i64`, and a negative `left` becomes a seeder
(the bug). HTTP accepts `0` to `i64::MAX` and refuses a larger value with `NumberOfBytesOverflow`.

Tests that exist today for the HTTP params (`http-protocol/src/v1/requests/announce.rs`, lines 889
to 925) cover only a non-numeric value. No test covers a negative value, zero, the largest value or
the overflow. The crate documentation of `axum-http-server` (`src/lib.rs`, lines 48 to 52) calls
the three params "positive integer", although zero is valid.

## Sources for the Decisions

Several questions in the review of this specification already had an answer in an ADR or another
document that nobody remembered. This table maps each question to its source, so the implementer and
later readers start from the recorded decisions.

| Question | Answer | Source |
| --- | --- | --- |
| Why are there three `NumberOfBytes` types? Is that duplication? | One type per layer, mapped at the boundaries; a single shared type was rejected | [Keep protocol and domain types decoupled](../../../adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md) (2026-05-27, commit `f7f29500d`); [issue #1835 specification](../../closed/1835-1669-14-decouple-http-protocol-from-tracker-primitives/ISSUE.md) |
| May a protocol type differ from what its BEP defines? | No. "`*-protocol` packages strictly implement BEP specifications" | [docs/packages.md](../../../packages.md), Package Conventions, principle 2 |
| Which way do dependencies point? | Servers, then core, then protocol, then domain; forbidden edges are checked by `cargo deny` | [docs/packages.md](../../../packages.md), Layer Boundary Enforcement; [deny.toml](../../../../deny.toml) |
| Must UDP and HTTP accept the same range for a value a client sends? | No. Each protocol has its own limit for its own reason, and the domain has none | [Cap scrape info hashes per protocol](../../../adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md) (2026-10-05); [issue #2417 specification](../../closed/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md) |
| When is a value bounded by the tightest protocol? | Only a value the operator configures and every protocol sends, such as the announce interval | [Bound protocol-agnostic values by the tightest delivery protocol](../../../adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md) (2026-10-07) |
| How is a single-value invariant protected? | By a newtype that rejects the value at construction (decided for configuration values) | [Use newtypes for constrained configuration field types](../../../adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md); [Separate configuration value invariants from consistency validation](../../../adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md) |
| Are wrapped inner errors public? When is an error enum `#[non_exhaustive]`? | Public and typed per package. `#[non_exhaustive]` when consumers only report or convert; exhaustive when variants select a recovery action. A generic `Box<dyn Error>` is rejected | [Return `Result` only for concretely fallible public APIs](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md) with its amendment of 2026-10-19; [issue #2454 specification](../../closed/2454-1669-mark-public-error-enums-non-exhaustive/ISSUE.md); the [`handle-errors-in-code` skill](../../../../.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md) |
| Why are packages treated as separate public APIs? | Each package is versioned and published on its own | [Adopt independent package versioning](../../../adrs/20260629000000_adopt_independent_package_versioning.md); [issue #2583 specification](../../open/2583-1669-first-publish-of-workspace-crates/ISSUE.md) |
| May an event carry an internal error type? | No. Events are objective facts with a stable payload | [Events are objective facts](../../../adrs/20260727000000_events_are_objective_facts.md); [general error-events EPIC draft](../../drafts/generalize-error-events/EPIC.md) |
| Is a questionable inbound value refused or tolerated? | Strict by default; a tolerance needs recorded evidence | [Reject malformed inbound protocol values unless real clients need the tolerance](../../../adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md), delivered by the spec-only pull request of this issue; [issue #2842 specification](../2842-inbound-tolerance-adr/ISSUE.md) |
| Which layer refuses a value the wire allows and the domain does not? What has priority over failing fast? | The first conversion into a type the project owns; a strict protocol specification has priority | The same ADR, points 6, 8 and 9 (decided in this review, D2) |
| Does any real client send a negative byte count? | None of the clients checked does: libtorrent sends `16384` and Transmission `i64::MAX` when the size is unknown | [Specification of #2838](../2838-1347-udp-reject-negative-announce-bytes/ISSUE.md), "Evidence on real clients and other trackers" |
| What do the specifications say about the sign of the byte counts? | BEP 15 and BEP 3 do not say; the libtorrent document uses `int64_t` | [BEP 15](https://www.bittorrent.org/beps/bep_0015.html), [BEP 3](https://www.bittorrent.org/beps/bep_0003.html), [XBT Tracker](https://xbtt.sourceforge.net/udp_tracker_protocol.html), [libtorrent](https://www.libtorrent.org/udp_tracker_protocol.html); see "Research Findings" |
| Where were numeric wire conversions reviewed before? | The audit of every numeric conversion between wire and domain types | [issue #2245 specification](../../closed/2245-2243-review-numeric-protocol-wire-conversions/ISSUE.md) |

Questions with no recorded answer, found in this review:

| Question | State |
| --- | --- |
| When does an error stay typed and when is it replaced by a generic one for the user? How far do errors bubble up? | The maintainer's rules are not written in any ADR. Candidate for an ADR outside this issue |
| Why did the UDP protocol designers use a signed type for byte counts? | Unknown; not documented in the four sources |
| Does a client that was not checked send a negative byte count on purpose? | Unknown. The specification of #2838 lists the unchecked clients |

## Research Findings

### One type per layer is a recorded decision

There are three types called `NumberOfBytes`, one per layer. This is intended and documented:

| Layer | Type | Who defines its shape |
| --- | --- | --- |
| UDP delivery | `torrust_tracker_udp_protocol::NumberOfBytes(pub I64)` | The BEP 15 wire layout |
| HTTP delivery | `torrust_tracker_http_protocol` `NumberOfBytes(pub i64)` | The project; BEP 3 gives no width and no sign |
| Domain | `torrust_tracker_primitives::NumberOfBytes(pub i64)` | The project |

The decision is
[Keep protocol and domain types decoupled](../../../adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md),
dated 2026-05-27. It was added in commit `f7f29500d` ("refactor(http-protocol): decouple protocol types
from domain primitives") for issue #1835, SI-14 of the packages overhaul EPIC #1669. It uses
`NumberOfBytes` as its example, says that the three types can look like accidental duplication but
have different reasons to change, and rejects a single shared type. The code cites the ADR in a
comment above the UDP and the HTTP type.

That ADR says that adapters map between the types at the boundaries. It does not say which layer
refuses a value that the wire allows but the domain does not. The [ADR of issue #2842](../../../adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md) now does (D5).

Maintainer constraint (2026-11-12): a type that represents a type defined by a BEP is not changed.
The delivery layer may therefore have to accept a value that makes no sense in the domain, and the
refusal happens when the value is converted into the next type the project owns.

### What the specifications say about the sign

Read on 2026-11-12: [BEP 15](https://www.bittorrent.org/beps/bep_0015.html), the original
[XBT Tracker document](https://xbtt.sourceforge.net/udp_tracker_protocol.html), the
[libtorrent document](https://www.libtorrent.org/udp_tracker_protocol.html) and
[BEP 3](https://www.bittorrent.org/beps/bep_0003.html).

| Announce field | BEP 15 | XBT Tracker | libtorrent |
| --- | --- | --- | --- |
| `downloaded`, `left`, `uploaded` | 64-bit integer | 64-bit integer | `int64_t` |
| `num_want` | 32-bit integer | 32-bit integer | `int32_t` |
| `event`, `action`, `transaction_id` | 32-bit integer | 32-bit integer | `int32_t` |
| `ip`, `key` | 32-bit integer | 32-bit integer | `uint32_t` |
| `port` | 16-bit integer | 16-bit integer | `uint16_t` |

- BEP 15 and the XBT Tracker document never say "signed" or "unsigned" for any field.
- The libtorrent document uses C types, so it is the source that makes the byte counts signed.
  BEP 15 cites it in a footnote as an existing implementation. Its credits say the protocol was
  designed by Olaf van der Spek (the author of BEP 15) and extended by Arvid Norberg.
- BEP 15 uses `-1` as the default of `num_want`, so "integer" in BEP 15 includes signed values at
  least for that field.
- BEP 3 describes `uploaded`, `downloaded` and `left` only as "encoded in base ten ascii", with no
  width and no sign.
- No document explains why a byte count is signed, and none says what a tracker must do with a
  negative or out-of-range value.

Conclusion: the UDP wire type `I64` follows the libtorrent reading, which is the de facto one, and
stays. Refusing a negative count is a decision of this project (#2838), not a rule of a BEP. Why the
protocol designers chose a signed type is not documented in these sources.

### Why HTTP has an upper limit

`NumberOfBytesOverflow` was added in commit `74ed59221` (2023-02-15, "feat(http): [#184] added
optional params to announce req in Axum implementation"). At that time the peer stored its counters
in `aquatic_udp_protocol::NumberOfBytes(pub i64)`, the UDP wire type, which the whole tracker used.
The new HTTP parser read a `u64` to refuse negative values and then had to fit the result into that
`i64`, so it needed an error for values that do not fit. The commit message and the code give no
other reason.

The signed type itself came from the UDP side: commit `9d5833390` (2021-08-23) defined
`NumberOfBytes(pub i64)` and commit `fe5e8ea40` (2022-01-28) ported UDP to `aquatic_udp_protocol`.

Conclusion: the limit is not a protocol rule or a policy. It is a consequence of the HTTP tracker
storing its values in a type borrowed from the UDP protocol. The types were decoupled in 2026
(`f7f29500d`), but the `i64` and the limit were copied along.

### Is a negative value ever meaningful?

Question from the maintainer (2026-11-12): is it certain that no `NumberOfBytes` type ever needs a
negative value, for example as a sentinel?

Inside the workspace (searched on `develop`, production code, tests, benches, examples and the
`tracker-client` console):

- No code builds a domain or HTTP `NumberOfBytes` from a negative literal. Every literal is `0` or a
  small positive number.
- No code uses a negative value as a sentinel ("unknown", "not set"). The HTTP protocol type
  expresses "not sent" with `Option`.
- No arithmetic is done on a byte count. The domain type has no `Add`, `Sub` or ordering; it
  derives only equality, hashing, `Clone`, `Copy` and `Debug`. So no subtraction can go below zero.
- The only code that depends on the sign is `Peer::is_seeder` (`left.0 <= 0`).
- The only sources of a negative value are the UDP wire (any `i64`) and two test helpers that take
  an `i64`: the quickcheck generator of the UDP wire type (`udp-protocol/src/request.rs`), which
  stays, and the `with_bytes_left_to_download(left: i64)` builder in `primitives/src/peer.rs`.
- The UDP checker of `tracker-client` takes `left: Option<i64>`
  (`console/tracker-client/src/console/clients/udp/checker.rs`); the other client commands take a
  `u64`.

The type change itself is the test: with a `u64` inside, a negative literal or an `i64` passed to
the domain type does not compile, so any use that this search missed is found by the compiler
during the implementation, before any test runs.

Outside the workspace, for clients in the wild, the evidence is in the
[specification of #2838](../2838-1347-udp-reject-negative-announce-bytes/ISSUE.md), section "Evidence on real clients and other trackers". It was
collected from the client and tracker sources:

- The case where a client does not know the size is a magnet link before the metadata arrives.
  libtorrent then sends `left = 16384` and Transmission sends `left = i64::MAX`. Neither sends a
  negative value. The tests written for PR #2839 cover each value; this issue takes them over.
- No client that was checked sends a negative `left`. That specification lists the clients that
  were not checked.
- No tracker that was checked refuses a negative value: opentracker and aquatic treat it as a
  leecher. Refusing is stricter than the ecosystem; the maintainer decided it in #2838.
- The specifications define no meaning for a negative count (table above).
- The HTTP tracker has refused negative values since it parses a `u64`. No report of a client
  broken by that is known.

The residual risk, an unchecked client that sends a negative count on purpose, is for UDP only. It
is the subject of the ADR of issue #2842 (reject malformed inbound values unless real clients need
the tolerance).

## Design Decisions

Each decision lists its options. D1 to D6 were decided by the maintainer on 2026-11-12, in the
review of this specification, which started as an EPIC with sub-issues (D6). The [ADR of issue #2842](../../../adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md) records the general rules (D5). The documents behind each decision are in
"Sources for the Decisions".

### D1: the domain type

| Option | Shape | For | Against |
| --- | --- | --- | --- |
| **A (decided)** | Private `u64` inside; `const fn new(u64)` stays infallible; `get() -> u64` | The type system gives the invariant; no error type in `primitives`. The about 130 `NumberOfBytes::new(0)` call sites compile unchanged. The only fallible conversion left in the workspace is the UDP wire edge (`i64` to `u64`), which is where the syntax check belongs. HTTP to domain is infallible | The domain can hold values above `i64::MAX`. The REST resource fields must become `u64` (D4) |
| B | Private field, range `0..=i64::MAX`; `TryFrom<i64>` and `TryFrom<u64>`; a `ZERO` constant | Same accepted range as today on every protocol. The REST resource keeps `i64` | A new error type in `primitives`. Every literal call site changes. The HTTP to domain conversion becomes fallible in the types although it can never fail, so `http-core` needs an error path that is dead code. The upper bound is a wire-width limit of BEP 15 leaking into the domain |
| C | Keep `i64` and the public field; only document the rule | No change | Does not solve the problem |

**Decided by the maintainer (2026-11-12): A, with no upper bound.** Non-negative is the only
invariant the domain has for a byte count, and `u64` states exactly that. The field becomes private
with a `get()` accessor, so a later invariant can be added without touching every reader again.

The maintainer's priority is the cleanest long-term design, not the cost of the refactor. Under
that priority A still wins: B's upper bound is the width of a UDP wire field, not a domain rule,
and B forces `http-core` to handle a conversion error that can never happen. No new type is
introduced: the existing newtype of each layer stays and only the domain one changes what it holds.
The newtype pattern is what makes the new constraint a local change.

With the new type, `Peer::is_seeder` becomes `left == 0 && event != Stopped` in its three copies.
Removing the duplication of the three copies is not part of this issue.

### D2: where UDP fails

| Option | Where | For | Against |
| --- | --- | --- | --- |
| U1 | Only in `Request::parse_bytes`, beside the port `0` and event checks | Earliest point; the error is sendable and carries the transaction id | Puts a project decision into the layer that mirrors BEP 15, which is silent on negative values. `AnnounceRequest` has public fields, so the conversion to the domain is fallible anyway |
| **U2 (decided)** | Only in `peer_builder::from_request`, as a fallible wire-to-domain conversion | The parser stays a pure decoder of the BEP 15 packet. The rule is in one place and the types enforce it | Fails after the connection cookie check, inside the service, not at the edge |
| U3 | Both | Fails at the earliest point and the types still protect the domain | The rule is written twice, and the parser no longer only mirrors the specification |

**Decided by the maintainer (2026-11-12): U2.** Failing fast is the preference, but implementing the
public specification strictly has priority (see "Background"). BEP 15 and the documents it
cites allow a signed 64-bit value here (see "Research Findings"), so the delivery layer accepts it.
The value is refused when it is converted into the domain type, the first type the project owns.

The wire type stays `NumberOfBytes(I64)`: it is a zero-copy map of the packet
(`packages/udp-protocol/src/common.rs`) and
`docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md` keeps it protocol-local.
`Request::parse_bytes` is not changed.

Observation, not part of this issue: `Request::parse_bytes` already refuses a port `0` and an event
outside `0..=3`. Under the rule above those checks deserve a review: BEP 15 lists the four event
values, so that check mirrors the specification, but it does not forbid port `0`. The ADR of issue #2842
states the rule (D5); a follow-up issue can apply it to the port check.

Rejected: a second, validated request type returned by `parse_bytes` ("parse, don't validate" in
full). It would duplicate the announce request struct and is out of proportion for three fields.

### D3: the error of the conversion, and the patch of PR #2839

After D1 and D2, the conversion in `peer_builder::from_request` refuses a negative counter.
`validate_number_of_bytes`, the function of PR #2839, is never added: that pull request is closed
without merging. The open point was the error the conversion returns.

| Option | Shape | For | Against |
| --- | --- | --- | --- |
| 1 | `from_request` returns `UdpAnnounceError::NegativeNumberOfBytes` directly | Uses the variant proposed by PR #2839 | The conversion module depends on the error enum of the announce service that calls it: an inner module depends on an outer one |
| **2 (decided)** | `peer_builder` defines its own error for the wire-to-domain conversion (field name and value). `UdpAnnounceError` wraps it as the `source` of one new variant | The dependency points inwards. It is what `UdpAnnounceError` already does for `ConnectionCookieError`, `AnnounceError` and `WhitelistError`. A new failure cause in the conversion does not change the service enum | One more error type in `udp-core` |

**Decided by the maintainer (2026-11-12): option 2.** Dependencies follow the documented
architecture: delivery layer, then application services, then domain
([docs/packages.md](../../../packages.md), Layer Boundary Enforcement). An inner module never
depends on an outer one, and an outer error wraps the inner error. The reason is to protect the
domain from the outer layers, which change more often.

Both modules are in `udp-core` (application layer), so no domain code is involved: with D1 the
domain type needs no error at all, and the conversion uses `u64::try_from` on the wire value.

Whether the wrapped error is public is already answered by the repository's rules, so this issue follows
the existing shape, a public error type in a public `source` field:

- [Return `Result` only for concretely fallible public APIs](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md)
  and its amendment of 2026-10-19: every package is published and consumed on its own, so its error enums
  are public and typed. An enum that consumers only report or convert is `#[non_exhaustive]`; an
  enum whose variants select a recovery action stays exhaustive. The same ADR rejects a generic
  error such as `Box<dyn Error>` because it hides which failures are possible. The new conversion
  error is only reported (it becomes a BEP 15 error response), so it is `#[non_exhaustive]`.
- The place where an inner error stops being exposed is the user-facing boundary, not the package
  boundary: the delivery layer turns the error into a protocol error response, and events must not
  carry internal error types (doc comments on `HttpAnnounceError` in
  `packages/http-core/src/services/announce.rs` and in `packages/udp-server/src/event.rs`;
  [Events are objective facts](../../../adrs/20260727000000_events_are_objective_facts.md); the
  [general error-events EPIC draft](../../drafts/generalize-error-events/EPIC.md)).

No ADR was found that records the maintainer's two general rules on errors: expose variants a
consumer can act on and return something generic when it cannot, and bubble errors up to the user
unless the number of abstraction levels makes that too costly. They are a candidate for an ADR of
their own, outside this issue.

The message sent to the client is the one PR #2839 proposed (``Invalid announce request: `left`
must not be negative, got -5``), or a reviewed variation of it. The tests written for PR #2839 are
taken over and adapted (see "Regression Test Strategy").

### D4: HTTP and the REST API

HTTP already refuses a negative value, because it parses a `u64`.
**Decided by the maintainer (2026-11-12), all three points:**

- **The HTTP protocol type holds a private `u64`.** BEP 3 gives no width and no sign, so the
  `i64` is the project's choice, not a BEP type. The conversion to the domain in `http-core` is
  then infallible. Rejected: leave `NumberOfBytes(pub i64)`, which forces `http-core` to handle a
  conversion error that can never happen.
- **The upper limit is removed.** The check that refuses values above `i64::MAX` and its
  `NumberOfBytesOverflow` variant go away, and HTTP accepts up to `u64::MAX`. The limit has no
  reason of its own (see "Why HTTP has an upper limit"). This is the only client-visible change of
  this issue: an announce with a counter above `i64::MAX`, refused today, is accepted.
- **The REST API resource fields `uploaded`, `downloaded` and `left` become `u64`.** The JSON
  output does not change for any value that can exist today; the Rust type and the generated API
  description do. Rejected: keep `i64` and saturate in the adapter, which hides a value.

After this change UDP and HTTP accept different ranges (`0..=i64::MAX` and `0..=u64::MAX`). That
is each protocol following its own specification, not an inconsistency to remove.

### D5: the ADR

The repository had ADRs for configuration values (newtypes for constrained fields, value
invariants separate from consistency validation) but none for values that arrive from clients.
Issue #2842 drafted one in PR #2843: "Reject malformed inbound protocol values unless real clients
need the tolerance". It decides whether a questionable value is refused.

**Decided by the maintainer (2026-11-12): no separate ADR for the rules of this issue. They are
part of [the ADR of issue #2842](../../../adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md), and that ADR is delivered by the spec-only pull request of
this issue**, so the rule and its first application are reviewed together. PR #2843 is closed
without merging; its four commits are carried over with their authorship. The spec-only pull
request closes issue #2842.

The ADR now answers both questions about an inbound value, whether it is refused and where:

| Rule of this issue | Point of the ADR |
| --- | --- |
| A public protocol specification is implemented strictly; that has priority over failing fast. A type that mirrors a BEP is not narrowed. The value is refused at the first conversion into a type the project owns | 6 |
| Wire ranges may differ per protocol; the domain is the same for all | 7 |
| Each kind of rule is validated in the layer that owns it, as early as possible: syntax in the delivery layer, application invariants in the service, domain invariants in the domain | 8 |
| Invariants are protected by constructors or newtypes, not by standalone validation functions | 9 |
| Byte counts as the first application, with the research on the sign of the UDP fields and the client evidence | "First application: byte counts" |

History of the alignment: the first text of PR #2843 differed from the decisions of this issue in
three places. Point 2 said the byte counts are signed "only because the format has no unsigned
type", which the sources do not support. Point 6 allowed the refusal in the protocol parser. Point
7 said "the stricter reading wins" between UDP and HTTP, which would have put the UDP wire limit
back on HTTP. The session of #2842 corrected the three points and added points 8 and 9 on
2026-11-12, after this review.

The question behind point 7, whether UDP and HTTP must accept the same ranges, already had two
recorded decisions, and both support D4:

- **Values clients send: each protocol has its own limit, for its own reason, and the domain has
  none.** [Cap scrape info hashes per protocol](../../../adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md)
  (2026-10-05, issue #2417): UDP keeps 74 scrape info hashes because of the packet size, HTTP keeps
  100 as a policy value, and "limits belong to the protocol that has the reason for them";
  `tracker-core` has no cap.
- **Values the operator configures and every protocol sends: one value, bounded by the tightest
  protocol.** [Bound protocol-agnostic values by the tightest delivery protocol](../../../adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md)
  (2026-10-07). It says that it does not cover limits on what clients send and points to the
  scrape ADR.

A byte count is a value a client sends, so the first decision applies: UDP accepts `0..=i64::MAX`
because of its wire field, HTTP accepts `0..=u64::MAX`, and the domain has no upper bound. The
maintainer confirmed it (2026-11-12): the protocols may differ at the wire level and are equal in
the domain.

The ADR also sets a general policy that this issue does not implement: a register of tolerances
and a metric that counts rejected requests per protocol and field (points 4 and 5).

### D6: one standalone issue for three threads

The bug was found by the `primitives` tests work of EPIC #1347 (Overhaul: Packages Testing).

**Decided by the maintainer (2026-11-12): one standalone issue that references #1347, not an EPIC
and not a child of #1347. It replaces the patch of PR #2839 and carries the ADR of PR #2843.**

- Standalone: this is a design change in several packages, not test work, so it does not belong
  under a testing EPIC. The EPIC to remove source locations from errors, which has the same origin,
  is also standalone.
- One issue, not an EPIC: the first draft was an EPIC with four sub-issues (an ADR, a check in the
  UDP parser, the HTTP protocol type and the domain type). D2 removed the parser change. The two
  code changes are not independent: between them the HTTP type would hold a `u64` and the domain
  type an `i64`, which needs a temporary error for values that do not fit. In one issue that
  interim state does not exist. The real code changes are in about ten files; the other call sites
  compile unchanged.
- No patch first: the patch of PR #2839 adds a function, an error variant and an enum that this
  issue would delete or replace right after. The bug only inflates a seeder count and has been
  there since UDP was ported, so it can wait for the proper fix. The implementation of this issue
  closes #2838.
- The ADR here: the patch, the ADR and this issue are three views of one decision, and were being
  worked on in three sessions that had to be reconciled. The decision documents are now reviewed in
  one docs-only pull request, and the code in a second one.

What is taken over, so nothing is lost:

| From | What | Where it is now |
| --- | --- | --- |
| PR #2843, branch `2842-inbound-tolerance-adr` | The ADR, its index entry and the specification of #2842 | Carried over as commits of the spec-only pull request |
| PR #2839, branch `2838-udp-reject-negative-announce-bytes` | The specification of #2838 with the field audit and the client and tracker evidence; the reproduction evidence; the UDP probe script | [`docs/issues/open/2838-1347-udp-reject-negative-announce-bytes/`](../2838-1347-udp-reject-negative-announce-bytes/ISSUE.md), carried over by the spec-only pull request |
| PR #2839, the same branch | The tests, in the commits "test(udp-server): reject a negative left in an announce (red)", "test(udp-core): [#2838] cover rejected and accepted byte counts in announce" and "fix(udp-server): [#2838] classify a negative byte count as a bad request" | Not carried over yet. The implementation takes them from that branch and adapts them to the new error |

The frontmatter keeps `epic: null`.

## Scope

### In Scope

- `primitives`: `NumberOfBytes` holds a private `u64`, with `const fn new(u64)` and
  `const fn get(&self) -> u64`; the three `is_seeder` copies compare `left` with zero; the
  serializer writes a `u64`; the test builder takes a `u64`; doc examples adapted.
- `udp-core`: `peer_builder::from_request` returns a `Result` with its own error type and converts
  each counter with `u64::try_from`; the announce service propagates it with `?`;
  `UdpAnnounceError` gets one variant that wraps the conversion error. The field is named by a
  typed value, as `ByteCountField` of PR #2839 does, or by the error's own variants; the
  implementation decides and records why. No `validate_number_of_bytes` function is added.
- `udp-server`: the new error is classified as `ErrorKind::BadRequest` in `src/event.rs`, as
  the commit "fix(udp-server): [#2838] classify a negative byte count as a bad request" of PR #2839
  does for its variant; the handler test; the message sent to the
  client is listed in the pull request for approval.
- `http-protocol`: `NumberOfBytes` holds a private `u64` with `new(u64)` and `get()`; the parser
  stores the `u64` it parses; `ParseAnnounceQueryError::NumberOfBytesOverflow` is removed; the
  builder methods no longer convert or panic and lose their `# Panics` sections; the query building
  uses `get()`.
- `http-core`: the protocol to domain conversion is `NumberOfBytes::new(uploaded.get())`,
  infallible; tests and the benches helper adapted.
- `axum-http-server`: tests adapted; a contract test for an announce with `left` above `i64::MAX`;
  the crate documentation of the three params says "non-negative integer".
- `rest-api-protocol` and `rest-api-runtime-adapter`: the resource fields `uploaded`, `downloaded`
  and `left` are `u64`; the adapter uses `get()`. The generated API description is checked for the
  changed fields.
- Every other caller in the workspace that does not compile.
- Tests (see "Regression Test Strategy").
- The characterization test of the primitives tests work (#2824) that pins "a negative `left`
  counts as a seeder": it cannot be written any more and is replaced.
- The documents of the spec-only pull request: this specification, the ADR of issue #2842 and the
  specification and evidence of #2838.

### Out of Scope

- The UDP wire type `torrust_tracker_udp_protocol::NumberOfBytes(I64)` and the UDP request parser
  `Request::parse_bytes` (D2).
- A review of the port `0` check in the UDP parser against the priority rule (D2, follow-up).
- Other announce fields. `num_want` is legitimately negative (`-1` means default); the field audit
  is in the specification of #2838.
- An upper bound for byte counts (D1, D4).
- Removing the three copies of `is_seeder`.
- A general review of the other newtypes in `primitives`.
- Implementing the register of tolerances and the rejection metric of the ADR (its points 4 and
  5).
- Hiding the wrapped errors of `UdpAnnounceError` behind an opaque source (D3).
- An ADR for the maintainer's general rules on error exposure and propagation (D3, follow-up).
- The `location` field of the error variants touched here; that is the EPIC drafted in
  `docs/issues/drafts/remove-source-locations-from-errors/EPIC.md`.
- The console `tracker-client`. Its UDP commands build wire requests, whose type does not change,
  and its HTTP commands already pass a `u64` to the builder.

## Architectural Decisions

The decisions are D1 to D6 above, with their sources in "Sources for the Decisions".

- Related ADRs:
  [Keep protocol and domain types decoupled](../../../adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md),
  [Cap scrape info hashes per protocol](../../../adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md),
  [Bound protocol-agnostic values by the tightest delivery protocol](../../../adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md),
  [Return `Result` only for concretely fallible public APIs](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md),
  and [Reject malformed inbound protocol values unless real clients need the tolerance](../../../adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md).
- ADRs to create: None. The ADR of issue #2842 is delivered by the spec-only pull request of this
  issue (D5) and must be merged before the implementation starts.

Design constraints for the implementation:

- The domain type has no error type and no fallible constructor. The check is `u64::try_from` on
  the UDP wire value, in `udp-core`.
- The conversion error names the field and carries the refused value, as the variant proposed by PR #2839
  does. It is a public `#[non_exhaustive]` type in a public `source` field (D3).
- `peer_builder` does not import the error of the announce service.
- No `expect` or `unwrap` on a conversion in production code, and no `Result` for a conversion that
  cannot fail.

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

Not applicable. The change is two value types, a pure parser, a pure conversion and their callers;
there is no child process, asynchronous I/O, network readiness, resource cleanup or reusable test
fixture.

## Bug-Fix Process

This is the fix for the bug of #2838, following `.github/skills/dev/debugging/fix-bug/SKILL.md`.
The first steps were done for PR #2839 and are recorded in the folder of #2838:

- Analysis and hypothesis: done. [Specification of #2838](../2838-1347-udp-reject-negative-announce-bytes/ISSUE.md), "Background" and "Field audit".
- Real-artifact reproduction: done, with the UDP probe against a running tracker.
  [Evidence of #2838](../2838-1347-udp-reject-negative-announce-bytes/manual-verification-evidence.md),
  `evidence/probe-before-fix.txt`.
- Regression-test boundary: the wire-to-domain conversion `peer_builder::from_request` (unit), the
  announce service (order of checks) and the UDP handler (response and no stored peer).
- Red evidence: recorded for the handler test (`evidence/red-test-output.txt`, commit "test(udp-server): reject a negative left in an announce
  (red)" of PR #2839).
  The implementation repeats it on its own branch before the production change.
- Production fix: this issue (T2 and T3 of the plan).
- Green evidence and like-for-like recheck: the same probe and values, recorded in this issue's
  `manual-verification-evidence.md` (M1 to M3).

## Regression Test Strategy

Each rule is protected at the boundary that owns it. The UDP tests exist on the branch of PR #2839
(`2838-udp-reject-negative-announce-bytes`) and are taken over, adapted to the new error:

- `udp-core`, unit tests of `peer_builder::from_request` (the causal seam for UDP): each of the
  three counters with `-1`; `left` with `i64::MIN`; `left` with `0`, `16384` (what libtorrent
  sends before it has the metadata) and `i64::MAX` (what Transmission sends). The tests of PR #2839
  for these values (commit "test(udp-core): [#2838] cover rejected and accepted byte counts in
  announce") move here from the service tests.
- `udp-core`, service tests of PR #2839 for the order of checks: the connection cookie is validated
  before the conversion, and a negative counter is refused before the whitelist is consulted. They
  stay.
- `udp-server`, the handler test of PR #2839
  (`it_should_reject_an_announce_with_a_negative_number_of_bytes_left`), adapted to the new
  variant.
- `http-protocol`, unit tests for each of `uploaded`, `downloaded` and `left`: a negative value is
  refused with `InvalidParam`; `0` is accepted; `u64::MAX` is accepted; a value above `u64::MAX` is
  refused with `InvalidParam`. The tests for negative and zero are written first and pass before
  any production change.
- `axum-http-server`, a contract test: an announce with `left` above `i64::MAX` is accepted.
- `primitives`: tests for `left == 0` (seeder) and `left > 0` (leecher), each with `Stopped` and
  with another event, replace the characterization test for a negative `left`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Pin today's HTTP range with tests | Unit tests in `http-protocol` for each param: negative refused, `0` accepted, non-numeric refused. They pass before any production change |
| T2 | TODO | Make the UDP conversion fallible | Red first: the handler test of PR #2839 fails. Then `peer_builder::from_request` returns `Result` with its own error; the service uses `?`; `UdpAnnounceError` wraps it; `udp-server` classifies it as a bad request; tests taken from the branch of PR #2839. The domain type is still `i64`. This step alone fixes the bug of #2838 |
| T3 | TODO | Change the two types and their callers | `primitives` and `http-protocol` types hold a private `u64`; `is_seeder`, serializer, parser, builder, `http-core`, REST resource and adapter, benches and any test code that does not compile. `NumberOfBytesOverflow` removed. The workspace builds and all tests pass |
| T4 | TODO | Add the tests for the new range | `u64::MAX` and above in `http-protocol`; the contract test in `axum-http-server`; seeder and leecher tests in `primitives` |
| T5 | TODO | Check client-visible output and documentation | UDP error message for a negative counter; REST peer JSON; API description; "non-negative integer" in the `axum-http-server` documentation. Old and new listed in the pull request |
| T6 | TODO | Verification and evidence | Automatic checks, manual scenarios, acceptance review, completion review |

T2 goes before T3 on purpose: it fixes the bug in the conversion while the domain type is
unchanged, so it can be reviewed and tested alone. T3 is one commit, because the workspace does not
compile between the type change and its callers.

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | `test(http-protocol)`: tests that pin the current accepted range | Commit after focused validation and required review. |
| T2 | `test(udp-server)`: the red handler test; then `fix(udp-core)!`: the wire-to-domain conversion refuses a negative counter | Commit after focused validation and required review. |
| T3 | `refactor(primitives)!`: the number of bytes holds a `u64` in the domain and in the HTTP protocol, with every caller | One commit, because the workspace must compile. Commit after focused validation and required review. |
| T4 | `test(...)`: the new range and the seeder rule | Commit after focused validation and required review. |
| T5 | `docs(axum-http-server)`: param documentation; other commits only if a text changes | Commit after focused validation and required review. |
| T6 | `docs(issues)`: evidence and progress | Commit after the evidence is recorded. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count: a single-use helper is valid when it
keeps the test readable and hides only incidental mechanics. Commit each reviewed test-design
increment before starting the next planned file or behavior area. Keep final verification and
completion evidence separate when it improves reviewability. Use a Conventional Commit message
with the narrow affected scope, and sign every commit with GPG.

## Coordination

- **PR #2839 (issue #2838).** Closed without merging, by its own session. Its branch
  `2838-udp-reject-negative-announce-bytes` is kept until the implementation has taken the tests
  from it. Issue #2838 stays open; the implementation pull request of this issue closes it.
- **Primitives tests (issue #2824, EPIC #1347).** That work is merged (PR #2841). Its tests in
  `packages/primitives/src/peer.rs` pin today's behaviour for a negative `left`:
  `it_should_treat_a_negative_number_of_bytes_left_as_a_seeder`, whose comment already points to
  this issue, and the `-1` cases of the trait-qualified role table. After this issue those cases do
  not compile. This issue deletes them and keeps the `left == 0` and `left > 0` cases.
- **EPIC to remove source locations from errors (draft).** Its SI-3 edits
  `ParseAnnounceQueryError` in `packages/http-protocol/src/v1/requests/announce.rs`, a file this
  issue edits. The two must not run at the same time; whichever starts second rebases.
- **PR #2843 (issue #2842).** Closed without merging, by its own session. The ADR is part of the
  spec-only pull request of this issue, which closes issue #2842.
- **EPIC #1347.** Its sub-issue table lists #2838; the row now points to this issue for the fix.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/non-negative-number-of-bytes/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-12 13:15 UTC - Claude Code (Fable 5.2) - Drafted the EPIC from the audit of `NumberOfBytes` on `develop` and the maintainer's review of PR #2839. Decisions D1 to D6 are proposals. Not approved; no GitHub issue created.
- 2026-11-12 16:40 UTC - Claude Code (Fable 5.2) - Added the research findings (decoupling ADR history; sign of the UDP fields in BEP 15, XBT Tracker and libtorrent). Recorded the maintainer's decisions D1 (private `u64`, no upper bound) and D2 (refuse at the wire-to-domain conversion only) and the priority rule: a strict protocol specification before failing fast. Removed the parser sub-issue. D4 to D6 are open.
- 2026-11-12 16:52 UTC - Claude Code (Fable 5.2) - Recorded the audit of negative values (none used in the workspace; no proof for clients in the wild) and the maintainer's decision D3: the conversion owns its error and `UdpAnnounceError` wraps it, so dependencies point inwards.
- 2026-11-12 17:07 UTC - Claude Code (Fable 5.2) - Recorded the history of the HTTP upper limit (commit `74ed59221`, 2023), the documented rule for exposing wrapped errors, and the maintainer's decision D4: HTTP type as `u64`, limit removed, REST fields as `u64`. D5 and D6 are open.
- 2026-11-12 17:17 UTC - Claude Code (Fable 5.2) - Recorded the two existing ADRs on value ranges and the three corrections for the ADR of PR #2843 (sent to its session). Added "Sources for the Decisions", which maps each question of the review to its ADR or document, lists the questions with no recorded answer, and linked the sources in the frontmatter and the text.
- 2026-11-12 17:19 UTC - Claude Code (Fable 5.2) - Recorded the maintainer's decision D5: no separate ADR; the rules are folded into the ADR of issue #2842 (PR #2843). Removed the ADR sub-issue; the EPIC has two code sub-issues that wait for that PR. D6 is open.
- 2026-11-12 17:22 UTC - Claude Code (Fable 5.2) - Recorded the maintainer's decision D6: standalone EPIC. All six decisions are made. Waiting for the approval of the specification as a whole; no GitHub issue created.
- 2026-11-12 17:27 UTC - Claude Code (Fable 5.2) - Drafted the specifications of SI-1 and SI-2 and linked them. Replaced the "client sources were not read" statement with the client evidence already recorded in the specification of #2838. Recorded the interim state between SI-1 and SI-2. Not approved; no GitHub issue created.
- 2026-11-12 17:32 UTC - Claude Code (Fable 5.2) - The maintainer decided that one issue is enough (D6). Merged the EPIC draft and the two sub-issue drafts into this single issue specification; nothing was dropped from the research, the decisions or the source map. Not approved; no GitHub issue created.
- 2026-11-12 17:40 UTC - Claude Code (Fable 5.2) - The maintainer approved the plan: this issue is the fix for #2838 and replaces the patch of PR #2839, and the ADR of #2842 is delivered by this issue's spec-only pull request instead of PR #2843. Created issue #2850, moved the specification to `docs/issues/open/`, carried over the four ADR commits and the specification and evidence of #2838, and rewrote D3, D5, D6, the bug-fix process and the coordination for that plan. The two other sessions were told and close their pull requests.
- 2026-11-12 20:29 UTC - Claude Code (Fable 5.2) - Rebased the spec-only branch onto `develop` after PR #2841 (the primitives tests of #2824) merged and conflicted in the EPIC #1347 table. Updated the coordination note: those tests are on `develop` and name this issue.

## Acceptance Criteria

- [ ] AC1: `torrust_tracker_primitives::NumberOfBytes` and the HTTP protocol `NumberOfBytes` have no public field and no constructor that takes a signed integer.
- [ ] AC2: the three `is_seeder` implementations compare `left` with zero, and tests cover `left == 0` and `left > 0`, each with `Stopped` and with another event.
- [ ] AC3: `peer_builder::from_request` returns an error that names the field and the value when `bytes_left`, `bytes_uploaded` or `bytes_downloaded` is negative; unit tests cover the three fields and `i64::MIN`.
- [ ] AC4: `peer_builder::from_request` accepts `left` of `0`, `16384` and `i64::MAX`.
- [ ] AC5: `validate_number_of_bytes` does not exist, and no other standalone function validates a byte count.
- [ ] AC6: `peer_builder` does not import the error of the announce service; `UdpAnnounceError` wraps the conversion error as a `source`.
- [ ] AC7: `torrust_tracker_udp_protocol::NumberOfBytes` and `Request::parse_bytes` are unchanged (no diff in `packages/udp-protocol`).
- [ ] AC8: `ParseAnnounceQueryError::NumberOfBytesOverflow` does not exist; for each of `uploaded`, `downloaded` and `left` the HTTP parser refuses a negative value and a value above `u64::MAX` with `InvalidParam`, and accepts `0` and `u64::MAX`.
- [ ] AC9: `AnnounceBuilder::with_downloaded`, `with_uploaded` and `with_left` cannot panic and have no `# Panics` section.
- [ ] AC10: the conversion from the HTTP protocol type to the domain type is infallible, and an HTTP announce with a counter above `i64::MAX` is accepted.
- [ ] AC11: the REST resource fields `uploaded`, `downloaded` and `left` are `u64`, and the JSON of a peer is the same as before for the same peer.
- [ ] AC12: a UDP announce with a negative counter gets a BEP 15 error response with the request's transaction id, the peer is not stored, and the swarm statistics do not change.
- [ ] AC13: an announce with a negative, zero or positive `left` gets the same verdict over UDP and HTTP on a running tracker.
- [ ] AC14: the ADR of issue #2842 is merged, holds the rules listed in D5, and is listed in `docs/adrs/index.md`.
- [ ] AC15: the new UDP error is classified as `ErrorKind::BadRequest`, and the implementation pull request closes issue #2838.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- `cargo test --workspace`, because a `primitives` type changes
- `cargo test --doc --workspace`, because doc examples build the type
- `cargo bench --workspace --no-run`, so the benches compile
- `cargo deny check bans`, the layer boundary check
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Run a tracker with a UDP tracker, an HTTP tracker and the REST API on local ports. UDP announces
are sent with the probe of #2838; HTTP announces with `curl`.

The probe uses one info hash and one peer endpoint, so each accepted announce replaces the peer of
the previous one. Read the REST output of M7 right after each accepted announce, before the next
announce is sent, and use the single-torrent detail: the torrent list does not include peers.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | UDP, negative `left` | Probe of #2838 with `left = -5` | Error response with the transaction id; `complete` does not grow | TODO | `manual-verification-evidence.md` section V1 |
| M2 | UDP, `left = 0` | Probe with `left = 0` | Normal response; the peer is a seeder | TODO | `manual-verification-evidence.md` section V2 |
| M3 | UDP, positive `left` | Probe with `left = 1000` and with `left = 9223372036854775807` | Normal responses; the peers are leechers | TODO | `manual-verification-evidence.md` section V3 |
| M4 | HTTP, the same values | `curl` announces with `left=-5`, `left=0`, `left=1000` and `left=9223372036854775807` | Refused with a `failure reason`, seeder, leecher, leecher: the same verdicts as M1 to M3 | TODO | `manual-verification-evidence.md` section V4 |
| M5 | HTTP, above `i64::MAX` | `curl` announce with `left=9223372036854775808` | Normal response; the peer is a leecher | TODO | `manual-verification-evidence.md` section V5 |
| M6 | HTTP, above `u64::MAX` | `curl` announce with `left=18446744073709551616` | Bencoded `failure reason` | TODO | `manual-verification-evidence.md` section V6 |
| M7 | REST output | Right after each accepted announce of M2, M3, M4 and M5, and before the next announce, `GET` the single-torrent detail from the REST API | For each announce, `left`, `uploaded` and `downloaded` of the peer are JSON numbers with the values just sent, including the one above `i64::MAX` | TODO | `manual-verification-evidence.md` section V7 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a
  real human-oriented use of the feature or reproduction of the bug fix, not a
  simulated result and not merely running automated tests.
- Every recorded validation command result must identify the toolchain or
  runtime that produced it when one can affect behavior. For example, record
  `cargo +nightly fmt --all -- --check` as `nightly Rust toolchain`, rather than
  the ambiguous `cargo fmt --all -- --check`.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these
  scenarios. Record actual prerequisites, actions, commands, program output,
  relevant tracker logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

This issue reuses the probe in the specification folder of #2838,
[`evidence/udp_announce_probe.py`](../2838-1347-udp-reject-negative-announce-bytes/evidence/udp_announce_probe.py).
The reasons for a script and for Python are recorded there: a raw UDP packet with a negative
counter cannot be sent with the project's client, whose command line takes an unsigned value. No
new script is planned. The probe is not copied; the evidence file records the commit it was run
from.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |
| AC6 | TODO | {test/log/PR link} |
| AC7 | TODO | {test/log/PR link} |
| AC8 | TODO | {test/log/PR link} |
| AC9 | TODO | {test/log/PR link} |
| AC10 | TODO | {test/log/PR link} |
| AC11 | TODO | {test/log/PR link} |
| AC12 | TODO | {test/log/PR link} |
| AC13 | TODO | {test/log/PR link} |
| AC14 | TODO | {test/log/PR link} |
| AC15 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **A wide change in one commit.** The type change must compile in every package. Mitigation: T2
  is split off; most call sites compile unchanged; the compiler lists the rest.
- **One pull request across nine packages.** Mitigation: the commits of the plan can be reviewed
  one by one, and most touched files are test code that only needs to compile.
- **UDP does not fail at the earliest point.** A negative counter is decoded and passes the cookie
  check before it is refused. Mitigation: this is the accepted cost of mirroring BEP 15 strictly;
  the ADR records the priority rule.
- **HTTP accepts values it refused before.** Counters above `i64::MAX` are not realistic sizes, and
  the tracker does no arithmetic on them. Mitigation: the tests and M5 cover the largest value end
  to end, including the REST output. No test covered the old overflow path.
- **The UDP error message is new.** The text is built by `error.to_string()` through the wrapping
  variant. Mitigation: T5 lists the exact text in the pull request for approval.
- **A value above `i64::MAX` in the REST API.** JSON numbers above 2^53 lose precision in some
  clients. That already holds for `i64` values today. Mitigation: M7 records the output.
- **Breaking public types** in `primitives`, `http-protocol`, `udp-core` and `rest-api-protocol`.
  Mitigation: `primitives` owes a major release (4.0.0) and the others are before their first
  publish (`docs/issues/open/2583-1669-first-publish-of-workspace-crates/ISSUE.md`).
- **Conflict with parallel work** on the same files (#2824, the source-locations EPIC).
  Mitigation: the order in the Coordination section.
- **A real client sends a negative count on purpose.** libtorrent and Transmission do not; other
  clients are unchecked (see "Is a negative value ever meaningful?"). Mitigation: the refusal is an
  error response the client can show, the existing UDP error event counts it, and the ADR of issue
  #2842 defines when a tolerance is added.
- **The fix lands later than the patch would.** The bug stays on `develop` until the
  implementation merges. Mitigation: it only inflates a seeder count; T2 fixes it as the first code
  commit.
- **Tests live on a branch of a closed pull request.** Mitigation: the branch is kept until the
  implementation has taken them; the commits are named by their subject in D6.
- **Scope growth into a review of every primitive.** Mitigation: the Out of Scope list; other types
  get their own issues, using the ADR.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

The documents behind each decision, with links, are in "Sources for the Decisions".

- Related issues: #2838 (the bug, closed by the implementation of this issue;
  [specification](../2838-1347-udp-reject-negative-announce-bytes/ISSUE.md)), #2824 (primitives tests, where it was found as finding F6),
  #1347 (Overhaul: Packages Testing), #2842 (ADR on inbound tolerance), #1835 (decoupling of the
  protocol and domain types), #2417 (per-protocol scrape limits), #2454 (`#[non_exhaustive]` error
  enums), #2245 (numeric wire conversions), #184 (origin of the HTTP upper limit), #2583 (first
  publish of the workspace crates), #1669 (Overhaul: Packages)
- Related PRs: #2839 (the tactical patch, closed without merging), #2843 (the ADR, closed without
  merging and carried over here)
- Related commits: `f7f29500d` (types decoupled, 2026), `74ed59221` (HTTP upper limit, 2023),
  `fe5e8ea40` (UDP ported to `aquatic_udp_protocol`, 2022), `9d5833390` (first
  `NumberOfBytes(pub i64)`, 2021)
- Related ADRs:
  [Keep protocol and domain types decoupled](../../../adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md),
  [Cap scrape info hashes per protocol](../../../adrs/20261005124222_cap_scrape_info_hashes_per_protocol.md),
  [Bound protocol-agnostic values by the tightest delivery protocol](../../../adrs/20261007082938_bound_protocol_agnostic_values_by_the_tightest_delivery_protocol.md),
  [Return `Result` only for concretely fallible public APIs](../../../adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md),
  [Use newtypes for constrained configuration field types](../../../adrs/20260721100000_use_newtypes_for_constrained_configuration_field_types.md),
  [Separate configuration value invariants from consistency validation](../../../adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md),
  [Adopt independent package versioning](../../../adrs/20260629000000_adopt_independent_package_versioning.md),
  [Events are objective facts](../../../adrs/20260727000000_events_are_objective_facts.md),
  and [Reject malformed inbound protocol values unless real clients need the tolerance](../../../adrs/20261111170611_reject_malformed_inbound_protocol_values_unless_clients_need_tolerance.md),
  which carries the rules of this issue
- Architecture: [docs/packages.md](../../../packages.md) (package conventions and layer boundary
  enforcement)
- Specifications: [BEP 3](https://www.bittorrent.org/beps/bep_0003.html),
  [BEP 15](https://www.bittorrent.org/beps/bep_0015.html),
  [XBT Tracker UDP tracker protocol](https://xbtt.sourceforge.net/udp_tracker_protocol.html),
  [libtorrent UDP tracker protocol](https://www.libtorrent.org/udp_tracker_protocol.html)
