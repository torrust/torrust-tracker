---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - packages/axum-http-server/src/v1/handlers/announce.rs
    - docs/testing/refactoring-patterns/README.md
---

# Scenario Fixture with Independent Expected Outputs

## Problem

The announce-response tests repeated many field assertions for the same decoded response. Moving
those assertions into separate `expected_*` helper functions reduced repetition but separated the
domain input from the expected protocol contracts. Readers had to find and mentally synchronize
multiple fixtures, which made the scenario less expressive and created hidden coupling.

## Pattern

Represent one meaningful domain situation with a small test-only scenario type. It owns the domain
input and each independently specified expected representation.

```rust
struct AnnounceResponseScenario {
    announce_data: AnnounceData,
    expected_normal: DeserializedNormal,
    expected_compact: DeserializedCompact,
}
```

Give the scenario an associated factory with a behavior-oriented name, such as
`AnnounceResponseScenario::one_ipv4_seeder()`. Construct every expected value explicitly, and
visibly set every input value that the expected output asserts. Do not derive expected values by
calling the production mapping or response-building functions being tested.

Each test keeps its distinguishing request condition local, then decodes and compares the whole
observable response directly:

```rust
assert_eq!(decoded, scenario.expected_compact);
```

## Why This Works

- **Expressive:** the factory identifies the real scenario rather than an implementation detail.
- **Readable:** all related input and expected normal/compact contracts are adjacent.
- **Maintainable:** changing the scenario updates one intentional test fixture instead of several
  disconnected helpers and field assertions.
- **Deterministic and fast:** the fixture has no clock, I/O, randomness, network listener, or
  shared mutable state.
- **One behavior-focused contract:** each test varies only the request's compact mode, so a failure
  identifies the selected representation or its domain-to-protocol contract.

## Use When

- A single domain input has two or more observable protocol representations.
- Response types implement `Debug` and `PartialEq`, making direct whole-value comparison useful.
- Multiple tests differ only in a small selector such as a request flag or negotiated format.

## Do Not Use When

- The expected value is only used once and an inline literal is clearer.
- A scenario would accumulate unrelated optional variants; split it into focused named scenarios.
- Constructing an expected value would call the production code under test. Specify the contract
  independently instead.

## Repository Example

[`packages/axum-http-server/src/v1/handlers/announce.rs`](../../../packages/axum-http-server/src/v1/handlers/announce.rs)
uses this pattern to verify normal and compact bencoded announce responses. It was introduced
during package-testing EPIC issue #1347, subissue #1348.
