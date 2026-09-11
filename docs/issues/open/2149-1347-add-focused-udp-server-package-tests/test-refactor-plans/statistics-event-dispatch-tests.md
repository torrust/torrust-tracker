---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/event/handler/mod.rs
status: completed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/event/handler/mod.rs
    - packages/udp-server/src/statistics/event/handler/error.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/udp-server/src/statistics/event/listener.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Statistics Event-Dispatch Test Refactor Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This plan applies only to
`packages/udp-server/src/statistics/event/handler/mod.rs`.

## Phase 1 - Clean Current Tests

### Current state

The dispatcher has no colocated tests. Its `handle_event` match routes every UDP server event to a
specialized statistics handler. Unit-only coverage at the container-plan checkpoint is 19/21 lines
(90.48%), but aggregate/global coverage cannot establish that each dispatch decision is protected
at the unit boundary.

Six dispatch arms already have parent-dispatcher tests colocated with their specialized handlers:
request aborted, discarded, banned, received, accepted, and response sent. The three error-metric
handler tests intentionally call their local `error::handle_event` directly. The remaining
`Event::UdpError` arm has no independent observable seam: a metric assertion would test the error
handler and repository in addition to delegation.

### Decision

No cleanup increment is proposed because the dispatcher has no direct test code. Preserve the
specialized handler tests and their existing parent-router coverage. Do not create a seven-event
matrix or a production injection seam: `UdpError` delegation has no strict direct unit boundary in
the current design.

## Phase 2 - Add Missing Behavior Tests

### Strengths to preserve

1. The parent `handle_event` owns event-enum-to-specialized-handler dispatch.
2. Specialized handlers own metric routing and must remain directly testable without the parent
   dispatcher.
3. `event.rs` owns event classification and schema.
4. The repository and metrics modules own aggregation and query mechanics.
5. The statistics listener owns receiver and lifecycle behavior.

### Problems and opportunities

#### P1 - Error-event dispatch has no strict direct unit boundary

**Problem.** `Event::UdpError` is the only parent dispatch arm without a specialized-handler test
that invokes the parent `handle_event` function. The local error-handler tests do not exercise the
dispatcher arm directly.

**Why it matters.** A refactor can route the arm incorrectly or stop forwarding an error payload.
An omitted arm is compiler-enforced by the exhaustive `match`.

**Decision.** Do not add a direct test. The attempted metric-based test observed the specialized
error handler and repository rather than strict delegation, so it could fail for collaborator
behavior. The dispatcher returns no result and has no injectable collaborator seam. Adding a
production abstraction solely to observe a trivial delegation would add indirection without a
production benefit.

#### P2 - Other event variants are already represented at this boundary

**Decision.** Do not add dispatch tests for request aborted, discarded, banned, received, accepted,
or response sent. Existing tests in their specialized handler modules already call the parent
dispatcher and assert the relevant observable metric. A top-level matrix would duplicate those
contracts rather than improve unit coverage meaningfully.

## Proposed Refactorings

Apply items in order. Complete one approved increment—including prose-first comparison, focused
validation, review, and its mapped commit point—before beginning the next item.

### R1 - Record the Phase 1 no-change decision

- **Status:** DONE
- **Priority:** High impact / trivial effort
- **Addresses:** Phase 1
- **Change:** Confirm that the dispatcher has no direct test code to clean and that six existing
  specialized-handler tests already cover their parent dispatch arms.
- **Guardrails:** Do not move existing tests or create a dispatch matrix.
- **Decision:** The dispatcher has no colocated test code or concrete cleanup opportunity. Its six
  already-covered parent dispatch arms remain in focused specialized-handler tests. R2 assesses only
  the unprotected `UdpError` parent-routing gap rather than moving tests or creating a matrix.
- **Done when:** The no-cleanup decision is recorded before adding a test.

### R2 - Assess error-event dispatch

- **Status:** DONE
- **Priority:** High impact / low effort
- **Addresses:** P1
- **Change:** Originally proposed one direct asynchronous test that gives the parent dispatcher
  an IPv4 `Event::UdpError` and asserts the aggregate IPv4 error metric.
- **Guardrails:** Keep the event classification, parent dispatcher Act, and one metric assertion
  visible. Do not assert labels, client metrics, event conversion, logs, listener behavior, or
  repository arithmetic.
- **Decision:** No dispatcher unit test is added. The module's responsibility is exhaustive
  variant delegation with unchanged payload, repository, and timestamp. An omitted variant is a
  compile error, and the module exposes no seam that observes delegation without asserting a
  collaborator's metric side effect. A metric-based test would require knowing collaborator
  behavior and would fail for handler or repository reasons, not only routing reasons. Adding a
  production abstraction solely to unit test trivial delegation is not justified. The module
  documents this ownership, and routing remains verified indirectly by the parent-dispatcher
  tests inside specialized handler modules.
- **Done when:** The no-test decision and indirect verification path are recorded in the module
  and this plan.

### R3 - Review the test design after the vertical slice

- **Status:** DONE
- **Priority:** High impact / low effort
- **Change:** Complete and record the mandatory prose-first Arrange-Act-Assert and test-code-smell
  review for the candidate test.
- **Guardrails:** Keep one behavior and one reason to fail. Use an ordinary IPv4 context helper
  only if it names incidental construction without hiding the causal error event.
- **Review outcome:** The candidate test had one Act and one assertion, but its only observable
  result was a collaborator metric. The review identified a hidden-collaborator-knowledge smell:
  the test could fail for error-handler or repository reasons rather than dispatcher routing. The
  candidate was removed before commit; the duplicated IPv4 context helper was also removed.
- **Done when:** The review outcome is recorded and no misleading dispatcher test remains.

### R4 - Record residual dispatch ownership decisions

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Measure unit-only coverage and retain aggregate/global and integration-only evidence
  separately if it informs the assessment. Record why the other six event arms retain their
  existing test coverage.
- **Guardrails:** Do not add percentage-only tests or use aggregate/integration coverage as proof
  that this dispatcher's unit coverage is sufficient.
- **Decision:** Fresh clean reports show unit-only coverage of 19/21 lines (90.48%), 41/52 regions
  (78.85%), and 2/2 functions (100%). Integration-only coverage separately reports 17/21 lines
  (80.95%), 40/52 regions (76.92%), and 2/2 functions (100%). The reports are not combined. The
  remaining `UdpError` delegation arm is intentionally not covered by a strict direct unit test:
  omission is compiler-enforced and the current design has no non-collaborator observation seam.
  Existing specialized-handler parent-routing tests retain their observable metric contracts; the
  local error-handler tests retain error-routing behavior. Do not add a percentage-only test or a
  production injection abstraction.
- **Done when:** The unit-only dispatch coverage and every residual ownership decision are recorded.

## Progress Tracking

### Plan Checklist

- [x] Dispatcher arms, specialized-handler tests, current unit-only evidence, and listener/repository
      ownership reviewed.
- [x] Maintainer approved R1.
- [x] R1 implemented, reviewed, validated, and committed.
- [x] Maintainer approved R2.
- [x] R2 candidate implemented, reviewed, and replaced by a documented no-test decision.
- [x] Maintainer approved R3 design review.
- [x] R3 recorded, validated, and committed.
- [x] R4 coverage/ownership review completed and decision recorded.
- [x] Maintainer reviewed all approved changes.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Created this proposed plan after mapping each dispatcher arm to
  specialized-handler tests and confirming only `Event::UdpError` lacks a direct parent-dispatcher
  contract. No test or production change has been made.
- 2026-09-11 - User/maintainer - Approved R1. Record that the dispatcher has no direct tests to
  clean and retain its existing six specialized-handler parent-routing contracts; do not create a
  dispatch matrix before assessing the single R2 `UdpError` gap.
- 2026-09-11 - User/maintainer - Approved R2 as an assessment. After reviewing the candidate
  test, questioned whether a metric-based assertion strictly tests the dispatcher and whether a
  unit test is appropriate for a trivial delegation module.
- 2026-09-11 - GitHub Copilot - Analysed the module's responsibility and failure modes: variant
  omission is compiler-enforced; routing or payload loss has no observable seam without
  collaborator side effects or a production abstraction. Recommended a no-test decision.
- 2026-09-11 - User/maintainer - Agreed with the no-test decision. Requested recording the
  decision in this plan and a module comment explaining why there are no unit tests and how the
  routing is verified by other means.
- 2026-09-11 - User/maintainer - Approved R4. Measure and record separate unit-only and
  integration-only coverage, then retain the no-test decision without adding a percentage-only
  test or a production injection abstraction.
- 2026-09-11 - User/maintainer - Reviewed and approved the completed statistics event-dispatch
  plan. The module documents its routing-only responsibility and indirect verification boundary;
  no production abstraction or collaborator-side-effect test is justified.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| Plan documentation | DONE | Markdown and spelling checks passed after all maintainer review changes. |
| R1 | DONE | The dispatcher has no direct test code or concrete cleanup opportunity. Six existing specialized-handler tests retain their parent-dispatcher contracts; only the `UdpError` arm remains for R2 assessment. |
| R2/R3 | DONE | The candidate `UdpError` metric-based dispatcher test passed focused validation but was removed after design review because its only observable result belonged to collaborators. The module now documents its routing-only responsibility and indirect verification via specialized-handler parent-dispatcher tests. |
| R4 | DONE | Fresh clean reports: unit-only is 19/21 lines (90.48%), 41/52 regions (78.85%), and 2/2 functions (100%); integration-only is 17/21 lines (80.95%), 40/52 regions (76.92%), and 2/2 functions (100%). The remaining `UdpError` delegation arm lacks a strict non-collaborator observation seam; omission is compiler-enforced, and no production injection abstraction is justified. |
| Plan completion | DONE | Maintainer reviewed all approved increments and evidence before the next file plan begins. |

## Non-Goals

- Do not change production dispatch, event classification, specialized metric handlers, repository
  aggregation, listener behavior, event buses, sockets, clocks, or lifecycle behavior.
- Do not test all event variants or duplicate their existing parent-dispatcher contracts.
- Do not assert request-kind labels, client-software labels, peer-ID parsing, logging, or error
  conversion in the `UdpError` dispatcher test.

## Validation Per Approved Increment

- Apply the mandatory prose-first Arrange-Act-Assert and test-code-smell review before maintainer
  review.
- Run the focused `statistics::event::handler::tests` target and then the package `--lib` target
  when the increment is approved for broader validation.
- Run `cargo fmt --all -- --check` and `git diff --check`.
- Run `linter markdown` and `linter cspell` when this plan changes.
- Measure aggregate/global, unit-only, and integration-only coverage separately whenever coverage
  informs a decision.

## Completion Criteria

- The dispatcher routing responsibility and the reason it has no strict colocated unit test are
  documented in the module and this plan.
- The candidate metric-based test is rejected because it tests collaborator side effects rather
  than only dispatcher delegation.
- Existing specialized-handler tests retain ownership of their metric-routing contracts.
- Aggregate/global, unit-only, and integration-only coverage are kept separate in evidence.
- The maintainer reviews every approved increment before the next increment and before final
  verification.
