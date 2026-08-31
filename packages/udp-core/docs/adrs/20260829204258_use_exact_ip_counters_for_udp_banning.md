---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - packages/udp-core/src/services/banning.rs
    - packages/udp-core/benches/ban_service_benchmark.rs
    - packages/udp-core/docs/benchmarking/banning.md
    - docs/issues/open/2114-consider-removing-bloom-filter/ISSUE.md
---

<!-- skill-link: create-adr -->

# Use Exact IP Counters for UDP Banning

## Scope

Package-local ADR. This decision affects only the UDP core package's banning
service and should remain with the package if it is extracted.

## Description

`BanService` previously maintained both a counting Bloom filter and an exact
`HashMap<IpAddr, u32>` for invalid UDP connection-ID requests. Every invalid
source was inserted into both structures. The exact map made the final ban
decision, so the Bloom filter did not bound the map's memory growth; it only
attempted to avoid an exact-map lookup below the ban threshold.

Issue #2114 added a focused Criterion comparison. The exact-map reference was
faster for every measured counter operation, including repeated and distinct
IPv4/IPv6 updates and lookups below, at, and above the threshold. The former
Bloom filter also added a direct runtime dependency requiring separate license
review.

## Agreement

Remove the `bloom` dependency and keep the exact `HashMap<IpAddr, u32>` counter
as the UDP ban service's sole state. An IP is banned only when its exact error
count is greater than the configured limit.

This preserves the prior externally observable ban decisions while removing
the probabilistic pre-check, its string conversion, and its dependency. It does
not make the exact map bounded, but removing the filter does not create that
condition: it already existed because every invalid source was recorded in the
exact map.

### Alternatives Considered

#### Retain The Former Two-Level Counter

The former implementation incremented both the counting Bloom filter and the
exact map for every invalid source. It consulted the Bloom estimate before the
map during ban checks, but the map remained authoritative.

This design was rejected because it neither limited map growth nor improved the
measured hot path. The pre-removal benchmark found the direct exact-map
reference faster for all tested update and lookup workloads. Retaining it would
also preserve the string conversions and the `bloom` dependency without a
corresponding correctness or capacity benefit.

#### Use Only A Counting Bloom Filter

A Bloom-only counter would give predictable, fixed memory use, but ban an IP
from an estimated count. Counter collisions can make an IP that did not send
enough invalid requests appear to exceed the ban limit. That would cause a
false ban.

This is rejected because UDP ban enforcement must not deny responses to an IP
solely because it collided with other sources. The former configuration,
`CountingBloomFilter::with_rate(4, 0.01, 100)`, requested a one-percent
membership false-positive rate at 100 expected distinct items; it did not
establish a fixed false-ban rate. The first parameter is four bits per counting
entry, not four hash functions. The probability that a collision produces an
estimated count above the ban threshold depends on the traffic distribution,
the number of distinct sources, repeated errors, and the reset interval.

#### Use A Bloom Filter To Gate Exact-Counter Allocation

The Bloom filter could record initial invalid requests and create an exact-map
entry only after the filter's estimate reaches a promotion threshold. This
would reduce normal-case map allocation when many sources send only a few
invalid requests.

It was rejected for this issue because it changes the enforcement contract. If
the exact counter starts at zero when an IP is promoted, the IP needs additional
invalid requests before it is banned. If the exact counter is seeded from the
Bloom estimate to preserve the old threshold, collisions can cause a false ban.
The no-false-ban variant therefore deliberately delays enforcement.

It also does not bound the exact map against a distributed attacker. An attacker
can send enough invalid requests from every source to cross the promotion
threshold, eventually creating one map entry per source. The design raises the
attack's traffic cost and may reduce ordinary map growth, but it only delays the
same attacker-controlled cardinality growth. It needs its own measurable
operational requirement and ADR before adoption.

#### Cap Exact-Counter State

Limiting the map to a maximum number of entries would create a hard memory
bound. Once the limit is reached, the service must reject new counters, evict
existing counters, or apply an explicit fallback policy.

This is deferred because each overflow behavior changes security guarantees.
Rejecting new entries permits new offenders to avoid tracking; eviction permits
an attacker to flush a target's counter; and a fixed capacity needs an
operator-visible sizing and observability policy. The appropriate limit and
overflow behavior require production traffic evidence and an explicit threat
model.

#### Use Time-Based Or LRU Eviction

Time-to-live or least-recently-used eviction can reduce retained exact state,
especially for low-volume sources. It remains vulnerable to deliberate churn:
an attacker can keep its own entries recent or force other entries out.

This is deferred because eviction makes ban enforcement depend on unrelated
traffic and requires a decision about whether an evicted offender starts again
at zero. It also needs bounded-memory tests across IPv4 and IPv6 traffic
patterns.

#### Use Prefix-Based State Or Rate Limiting

Tracking or limiting by network prefix, or rate limiting invalid requests before
they reach the counter, can bound state more directly. Both approaches can
affect clients that share infrastructure, such as carrier-grade NAT, enterprise
networks, or IPv6 allocation prefixes.

This is deferred because the correct IPv4 and IPv6 prefix policy, allowed
collateral impact, rate-limit response, and interaction with valid clients are
not established. They are separate abuse-control designs rather than a local
replacement for the removed lookup optimization.

### Consequences

- UDP ban decisions remain exact and do not produce collision-driven false
  bans.
- Counter operations are simpler and the pre-removal benchmark shows the
  retained exact-map path was faster.
- Invalid-source state remains unbounded until the configured reset. This is a
  known capacity-hardening concern, not a protection supplied by the removed
  filter.
- Future memory-bounding work must be designed and tracked separately; it must
  not silently change the exact ban-decision guarantee.

## Affected Code

- `packages/udp-core/src/services/banning.rs`
- `packages/udp-core/benches/ban_service_benchmark.rs`
- `packages/udp-core/docs/benchmarking/banning.md`

## Date

2026-08-29

## References

- Issue #2114 - Evaluate removing the UDP Bloom filter.
- PR #2115 - Issue #2114 specification.
- `packages/udp-core/docs/benchmarking/banning.md` - pre-removal Criterion
  results and reproducible benchmark procedure.
