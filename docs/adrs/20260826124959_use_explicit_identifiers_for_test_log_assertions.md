---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - packages/test-helpers/src/logging.rs
    - docs/issues/open/1430-fix-tracing-span-log-assertions.md
    - https://github.com/dbrgn/tracing-test/issues/23
---

<!-- skill-link: create-adr -->

# Use explicit identifiers for test log assertions

## Description

Integration tests may need to assert that a specific operation emitted a log record. The tracker
uses a process-wide `tracing` subscriber, initialized once, and
`packages/test-helpers/src/logging.rs` captures its formatted output in a bounded shared buffer.

An earlier attempt considered identifying a test's records with the name of a `tracing` span
entered by the test. That association is not automatic across Tokio tasks, `spawn_blocking`, OS
threads, or nested child tasks. Correct propagation requires deliberate instrumentation or manual
span entry at every relevant execution boundary.

The tracker has many concurrent and nested execution paths. Establishing and maintaining complete
test-span propagation would add fragile, cross-cutting behavior while the current assertions have
no unmet capability requirement. The repository-owned capture helper already supports log
assertions and is easier to customize and diagnose than an external test harness.

## Agreement

Use explicit identifiers selected by the test author to associate an expected operation with a
captured log record. Suitable identifiers include a request ID, info hash, peer ID, or another
value that the operation deliberately records.

Keep `packages/test-helpers/src/logging.rs` as the repository-owned test logging mechanism. It
installs the global subscriber once, writes each captured record to the test output, and retains
recent formatted records in a bounded buffer for assertions through
`logging::logs_contains_a_line_with`.

Do not introduce automatic propagation of test-owned `tracing` spans through tracker execution
paths solely to identify log lines in tests. Do not adopt the `tracing-test` crate as a replacement
for the current helper.

### Alternatives Considered

**Automatically propagate a test-owned tracing span.** Rejected for current needs. Async tasks
can be instrumented with the current span, and blocking or OS threads can receive a cloned span
that is explicitly entered. However, the tracker would need to apply and maintain this behavior
at every relevant concurrent boundary. Missed nested paths would make assertions unreliable, and
the resulting test-correlation mechanism would be implicit rather than chosen by the developer.

**Use the `tracing-test` crate.** Rejected for current needs. It has the same fundamental
cross-thread and blocking-task association limitation documented in upstream issue #23. The
repository-owned helper supplies the needed capture behavior, keeps its bounded-buffer policy
under project control, and is easier to inspect and adapt when tests fail.

**Add a generic test logging guide.** Deferred. This ADR is the source of truth for the strategy.
Procedural documentation is warranted only when a future contributor workflow requires guidance
beyond the small helper API and ordinary test patterns.

### Consequences

#### Positive

- Test authors choose the correlation value they assert, making the relationship between test
  input and expected log record explicit.
- The tracker avoids pervasive tracing-context propagation across a complex concurrent runtime.
- The logging-test mechanism remains customizable and debuggable within the repository.

#### Negative

- Tests that assert logs must ensure the selected identifier is emitted by the exercised path.
- The shared bounded buffer remains a process-wide resource. Tests should use values unique to the
  operation under test so unrelated concurrent output cannot satisfy an assertion.
- Tests cannot assume an outer test span will identify records emitted by spawned work.

### Reopening Criteria

Reconsider this decision only when a concrete logging-test requirement cannot be met with an
explicit identifier. Before adding propagation infrastructure, evaluate the current
`tracing-test` ecosystem and reproduce the requirement against the relevant tracker execution
path. Any proposed solution must demonstrate reliable behavior across the required nested async,
blocking, or OS-thread boundaries.

## Affected Code

- `packages/test-helpers/src/logging.rs` - the global subscriber, bounded captured-log buffer,
  and assertion helper.
- Existing server contract tests that call `logging::setup()` and
  `logging::logs_contains_a_line_with` - consumers should continue selecting explicit operation
  identifiers for assertions.

## Date

2026-08-26

## References

- Issue #1430: <https://github.com/torrust/torrust-tracker/issues/1430>
- PR #1147: <https://github.com/torrust/torrust-tracker/pull/1147>
- PR #1148: <https://github.com/torrust/torrust-tracker/pull/1148>
- PR #1149: <https://github.com/torrust/torrust-tracker/pull/1149>
- PR #1429: <https://github.com/torrust/torrust-tracker/pull/1429>
- PR #1735: <https://github.com/torrust/torrust-tracker/pull/1735>
- Upstream `tracing-test` limitation: <https://github.com/dbrgn/tracing-test/issues/23>
