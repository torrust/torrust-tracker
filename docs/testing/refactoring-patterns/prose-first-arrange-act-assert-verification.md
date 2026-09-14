---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - packages/udp-server/src/handlers/mod.rs
    - docs/testing/refactoring-patterns/README.md
---

# Prose-First Arrange-Act-Assert Verification

## Problem

A test can pass while its behavioral intent remains implicit. Large Arrange blocks, parameter-bag
fixtures, opaque helpers, hidden production calls, and derived expected values make a test harder to
review and maintain. Conventional `Arrange`, `Act`, and `Assert` headings alone do not prove that
the code under each heading communicates what it is meant to establish.

## Pattern

Use temporary normal prose as the test specification, then make the code replace that prose:

1. Write one **Arrange** paragraph identifying the causal initial-state difference, one **Act**
   paragraph naming the production behavior, and one **Assert** paragraph stating the independently
   specified observable result.
2. Place the complete prose specification above the test and repeat each paragraph directly above
   its `// Arrange`, `// Act`, or `// Assert` section.
3. Compare each code section with its paragraph. Refactor names, setup, helper boundaries,
   builders, scenario fixtures, the production call, or assertions until the code communicates the
   same meaning.
4. Remove prose that the code now communicates. Retain a comment only when it supplies essential
   domain, portability, ownership, or safety context that code cannot express without a misleading
   or disproportionate abstraction.
5. Record the comparison in the test's task evidence before maintainer review and commit.

The prose constrains refactoring: simplify implementation mechanics, but do not weaken the stated
behavior merely to make the test shorter.

## Reveal Behavioral Data; Hide Collaborator Mechanics

Trace every value from Arrange to its use in the Act or Assert. Keep values visible when they select
the behavior, establish causal pre-existing state, or independently specify an expected result.
Hide only ordinary valid collaborator mechanics that do not change the selected behavior, such as
locks, reference-counted handles, default dependency construction, and required repository setup.

For example, a banning-handler gauge test makes the relationship visible as:

```text
unrelated_client_ip → state with one tracked client
cookie_error_client_ip → event context passed to the Act
expected_distinct_client_ip_total → asserted gauge result
```

Its test context may own `Arc<RwLock<BanService>>` and `Repository` construction, but must not hide
the IPs or expected total. Review with two questions:

1. Can a reader trace every value that makes the Act behave differently or sets the expected result
  from Arrange to Act/Assert?
2. Does this value merely make an ordinary collaborator valid? If yes, keep it in focused setup
  rather than the test narrative.

## Review Test-Code Smells Before Finishing

Before maintainer review, use the prose-first comparison to inspect these design smells. They prompt
a design decision rather than a mechanical rewrite rule: preserve the clearest behavioral contract
when a shorter alternative would hide intent or make failures less diagnostic.

| Smell | Question | Response |
| --- | --- | --- |
| Complex Arrange | Can the causal initial state be stated without reconstructing plumbing? | Prefer an inline value, readable builder, or scenario fixture named for the resulting state. Let it own coordinated incidental mechanics only. |
| Hidden behavioral data coupling | Can every behavior-selecting input, causal pre-existing state, and expected value be traced from Arrange into Act/Assert? | Keep those values visible and name their relationship; hide only ordinary collaborator-construction mechanics. |
| Multiple assertions | Are several assertions one complete observable result, or multiple behaviors? | Prefer one semantic assertion for a complete result; split unrelated behaviors into focused tests with one reason to fail each. |
| Hidden fixture coupling | Would an unrelated fixture change fail the test? | Derive incidental expected details from the same fixture used by the Act, while keeping causal expectations visible. |
| Hidden Act | Does the final test visibly invoke the production behavior? | Keep the Act in the test body. |
| Production-derived expected value | Does the expected output call code under test? | Construct it independently; a helper may compare it mechanically but must not calculate it through production behavior. |

For example, a receiver test can name the coordinated state `ReceiverWithQueuedLoopbackDatagram`
and use one semantic assertion for the resulting raw request. The test must still visibly provide
the causal datagram, await the receiver's next item, and compare an independently established
payload and sender address.

## Why This Works

- **Readable and expressive:** reviewers can first agree on behavior in plain language, then see
  that names and structure make the final code self-explanatory.
- **Maintainable:** a helper survives only when it has a specific, behavior-revealing responsibility.
- **Specific and behavioral:** the Act and independently specified outcome remain visible, preventing
  implementation-detail assertions or expectations derived from production code.
- **Deterministic:** the temporary prose makes hidden clock, I/O, retry, sleep, and shared-state
  dependencies easier to notice before they become flaky tests.
- **Structure-insensitive:** tests describe observable behavior, so internal refactoring need not
  require changing an opaque fixture or commentary.

## Use When

- Adding a new test or materially refactoring an existing test.
- An Arrange block needs multiple setup lines and the causal state is difficult to identify.
- A proposed helper or fixture might merely move complexity outside the test body.
- A passing test is difficult to explain in a concise review.

## Do Not Use When

- Never skip the process because a test looks small; the comparison may confirm that inline code is
  already the clearest design.
- Do not retain prose as permanent duplicate documentation once the code says the same thing.
- Do not force every domain explanation into code. Keep concise comments for irreducible facts, such
  as a protocol constraint or platform-specific limitation.
- Do not use prose to conceal an unclear test. Refactor until the code can express the intended
  behavior, or record why a direct test is not appropriate.

## Repository Example

The UDP handler-dispatch test in
[`packages/udp-server/src/handlers/mod.rs`](../../../packages/udp-server/src/handlers/mod.rs)
initially used a `SendableParseErrorPacketScenario` that combined the raw packet, environment, and
several ordinary `handle_packet` arguments. Its temporary prose distinguished the ordinary handler
environment from the causal raw scrape request containing no info hashes. The final code expresses
those responsibilities as `initialize_udp_handler_environment()` and
`scrape_request_without_info_hashes(transaction_id)`, while retaining the dispatcher Act and the
independent transaction-ID and request-kind assertions visibly in the test. This was reviewed under
package-testing EPIC issue #1347, subissue #2149.
