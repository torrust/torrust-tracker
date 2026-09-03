---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - packages/axum-http-server/src/server.rs
    - docs/testing/refactoring-patterns/scenario-fixture-independent-expected-outputs.md
    - docs/testing/refactoring-patterns/README.md
---

# Scenario Fixtures for Causal Initial State

## Problem

An Arrange section can consist of individually readable setup helpers while still concealing the
condition that makes the test behave differently. For example, address reservation, configuration
mutation, and registry seeding can leave readers to infer that the actual scenario is: "start a
server whose binding is already registered." The test is then correct but difficult to understand,
review, and extend.

## Pattern

First ask:

> What is the one difference in initial state that makes this Act behave differently?

Represent that answer with one focused test-only scenario fixture. Name the fixture after the
causal condition, not after construction operations. For example,
`ServerStartWithDuplicateRegistration` expresses the condition that makes a server start return a
duplicate-registration error.

The fixture owns the incidental mechanics required to establish that state: configuration selection
or mutation, concrete dependency construction, resource allocation, and registry/database seeding.
The test itself retains the production call under test and the observable assertions:

```rust
// Arrange
let scenario = ServerStartWithDuplicateRegistration::new().await;

// Act
let result = HttpServer::new(scenario.launcher())
    .start(scenario.container().await, scenario.registration_form(), scenario.metadata())
    .await;

// Assert
assert_duplicate_binding_error(result, scenario.bind_address());
```

A scenario fixture may expose the concrete values needed for the Act, but it must not perform the
Act, interpret its result, or assert it. Put comments about unavoidable setup constraints next to
the mechanism inside the fixture, where maintainers can find the reason without obscuring the test.

## Why This Works

- **Expressive:** the Arrange section names the state that causes the selected behavior.
- **Readable:** readers can understand the test without reconstructing a scenario from plumbing.
- **Specific:** failures identify a business-relevant scenario rather than an arbitrary setup step.
- **Maintainable:** detailed bootstrap logic has one discoverable home for that scenario.
- **Extensible:** several focused fixtures form a catalog of important configuration and state
  combinations, without forcing every test to duplicate their construction.
- **Behavioral:** the test keeps the production boundary call and observable contract visible.

## Use When

- Several setup operations collectively establish one meaningful precondition.
- The same state will be useful to understand, reuse, or vary in nearby tests.
- Concrete dependencies are structurally required but not individually relevant to the behavior
  being asserted.
- The scenario varies a configuration, authorization state, persisted state, registration state, or
  other condition that causally changes the Act's outcome.

## Do Not Use When

- An inline value states the condition more clearly than a new type.
- The setup has no meaningful causal condition beyond ordinary valid input.
- A readable builder chain already states the causal condition in the test body (see below).
- The fixture would accumulate unrelated options or optional components to serve many tests;
  split it into focused scenarios instead.
- The fixture would hide the production call, expected outcome, or assertions.
- The fixture would derive expected values by invoking production code under test.

## Scenario Fixtures and Test Builders

Scenario fixtures and test builders both make Arrange sections expressive. They solve different
problems and are often used together.

| Tool             | Reveals the causal state by…                                                     | Fits when…                                                                                                                    |
| ---------------- | -------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- |
| Test builder     | A readable call chain in the test body, e.g. `.private().with_expired_key(...)`. | The condition is one or two named choices on one object, and each test varies a different choice.                             |
| Scenario fixture | A single type named for the resulting state.                                     | The condition emerges from several coordinated steps across objects, resources, or registries, and no chain reads as clearly. |

A builder is the right choice when reading its chain tells you the scenario. A scenario fixture is
the right choice when the scenario is a coordinated combination that a chain would only narrate. A
fixture may use builders internally, and a scenario type may expose a small builder for the few
variations it legitimately supports. Choose whichever leaves the test body stating the causal
condition most directly; do not adopt one as a rule against the other.

## Repository Example

The duplicate-registration HTTP server-start test in
[`packages/axum-http-server/src/server.rs`](../../../packages/axum-http-server/src/server.rs) is
being refactored under package-testing EPIC issue #1347, subissue #1348. Its scenario fixture
captures a server binding that is available to bind but already registered, while the test visibly
starts the server and verifies both the typed error and listener cleanup.
