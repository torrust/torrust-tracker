---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - packages/udp-server/tests/server/contract.rs
    - docs/testing/refactoring-patterns/README.md
---

# Named Helpers for Abstraction-Level Alignment

## Problem

Test code can mix domain-relevant behavior with low-level setup mechanics. A reader then has to
reconstruct one coherent operation from configuration extraction, dependency construction, or
transport bootstrapping. The opposite mistake is rejecting a useful helper solely because it has one
caller, leaving callers at an inconsistent and noisy abstraction level.

## Pattern

Extract a helper when it gives a coherent sequence of actions a specific meaningful name and keeps
the calling test focused on its behavioral scenario. The helper owns incidental mechanics; the test
retains causal state, the production Act, and independently specified assertions.

For example, an integration test that exercises a UDP datagram exchange can call:

```rust
let tracker = start_ephemeral_udp_tracker().await;
```

This is justified even with one caller because it names one complete ordinary setup action. The test
can then remain at a consistent level: start tracker, connect client, send datagram, receive/decode
response, assert behavior, stop tracker.

## Selection Criteria

Keep a helper when all of the following are true:

1. Its name describes an action, capability, or state rather than a vague implementation detail.
2. Its body performs one coherent responsibility.
3. It hides only incidental mechanics from the caller.
4. The caller retains the causal state, production Act, and expected result.
5. The helper makes the caller's abstraction level more consistent.

Caller count is not a selection criterion. Reuse may later confirm a helper's value, but it is not
a prerequisite.

## Do Not Use When

- The inline code already expresses the state or action more clearly.
- The helper has a vague name such as `setup`, `prepare`, or `make_test_data`.
- It becomes a parameter bag or accumulates unrelated optional behavior.
- It hides the production call, derives expected outputs, or conceals the causal state.

## Repository Example

[`packages/udp-server/tests/server/contract.rs`](../../../packages/udp-server/tests/server/contract.rs)
uses `start_ephemeral_udp_tracker()` for the empty-datagram real-loopback contract. The helper
contains configuration and server-start mechanics; the test visibly provides the empty datagram,
performs the UDP exchange, decodes the response, and asserts the expected protocol error. The helper
was introduced during package-testing EPIC issue #1347, subissue #2149.
