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

Represent one complete behavioral example with a small test-only scenario type. It owns the
request, domain input, and independently specified expected response.

```rust
struct AnnounceResponseScenario<TExpectedResponse> {
    announce_request: Announce,
    announce_data: AnnounceData,
    expected_response: TExpectedResponse,
}
```

Give the scenario an associated factory with a behavior-oriented name, such as
`AnnounceResponseScenario::compact_response_for_one_ipv4_seeder_when_accepted()`. A builder may
hide fields that are irrelevant to the behavior, but the scenario owns all artifacts that form the
example. Construct every expected value explicitly, and visibly set every input value that the
expected output asserts. Do not derive expected values by calling the production mapping or
response-building functions being tested.

Each test executes the production boundary with the complete scenario, then decodes and compares
the whole observable response directly:

```rust
let response = build_response(&scenario.announce_request, scenario.announce_data);
let actual_response: DeserializedCompact = decode_successful_bencoded_response(response).await;

assert_eq!(actual_response, scenario.expected_response);
```

`decode_successful_bencoded_response` is a narrow test helper for repeated transport mechanics: it
asserts the successful HTTP status, reads the body, and deserializes bencode. Keep the production
boundary call, the expected response type, and the final behavioral assertion visible in each test.
Do not use the helper to derive expected values or select a response representation.

## Why This Works

- **Expressive:** the factory identifies the real scenario rather than an implementation detail.
- **Readable:** all related request, domain input, and expected response contracts belong to one
  scenario.
- **Maintainable:** changing the scenario updates one intentional test fixture instead of several
  disconnected helpers and field assertions.
- **Deterministic and fast:** the fixture has no clock, I/O, randomness, network listener, or
  shared mutable state.
- **One behavior-focused contract:** each test varies only the request's compact mode, so a failure
  identifies the selected representation or its domain-to-protocol contract.

## Use When

- A single domain input has two or more observable protocol representations.
- Response types implement `Debug` and `PartialEq`, making direct whole-value comparison useful.
- Related test artifacts form one concrete example, including the request that selects the result.

## Do Not Use When

- The expected value is only used once and an inline literal is clearer.
- A scenario would accumulate unrelated optional variants; split it into focused named scenarios.
- Constructing an expected value would call the production code under test. Specify the contract
  independently instead.

## Repository Example

[`packages/axum-http-server/src/v1/handlers/announce.rs`](../../../packages/axum-http-server/src/v1/handlers/announce.rs)
uses this pattern to verify normal and compact bencoded announce responses. It was introduced
during package-testing EPIC issue #1347, subissue #1348.
