---
doc-type: epic
status: draft
github-issue: null
spec-path: docs/issues/drafts/generalize-error-events.md
epic-owner: null
last-updated-utc: 2026-08-19 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/events-architecture.md
    - docs/issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/error-event-observability-analysis.md
    - packages/events/src/bus.rs
    - packages/http-core/src/event.rs
    - packages/http-core/src/services/announce.rs
    - packages/http-core/src/services/scrape.rs
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-protocol/src/v1/requests/scrape.rs
    - packages/tracker-core/src/error.rs
    - packages/udp-core/src/event.rs
    - packages/udp-server/src/event.rs
    - packages/udp-server/src/error.rs
---

<!-- skill-link: create-issue -->

# EPIC #[To be assigned] - Define and Implement General Error Events

## Goal

Define a deliberate, stable, privacy-safe error-event contract and implement it
consistently for the tracker services and error paths that the approved design
includes.

## Why This Is Needed

The tracker event system decouples producers from metrics, banning, and future
consumers. Adding a one-off event merely to create a counter risks creating an
accidental public event API with incomplete coverage and unclear guarantees.

Issue #1987 exposed this problem when an event and metric were proposed for a
rejected HTTP announce `ip` parameter. The event and metric were deliberately
removed under Option B. The strict protocol behavior remains, but this EPIC
records the cross-service design work required before similar error events are
introduced.

## Scope

### In Scope

- Define the purpose, audience, compatibility guarantees, and coverage boundary
  of error events.
- Define objective, bounded, consumer-safe error reason types rather than
  exposing internal error enums or raw client-controlled values.
- Decide how parser/extractor failures, authentication and authorization
  denials, service errors, and response-generation failures are represented.
- Audit current HTTP, UDP, tracker-core, and REST error paths against the agreed
  boundary; implement events for every in-scope current case.
- Reconsider the rejected HTTP announce `ip` parameter once the general
  contract is implemented. Its counter is only added if it follows from that
  contract.
- Document source-level semantic links to the governing ADR, this EPIC, and
  relevant decision analyses wherever event/error APIs are defined.

### Out of Scope

- Reintroducing a rejected-`ip` counter or event before the general contract is
  designed and accepted.
- Direct metrics dependencies from request-handling services.
- Defining a new ADR or opening a GitHub issue before this draft is refined and
  approved.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue                                                            | Local Spec  | Status | Notes                                                                                                          |
| ----- | ---------------------------------------------------------------- | ----------- | ------ | -------------------------------------------------------------------------------------------------------------- |
| 1     | #[To be assigned] - Define the error-event contract              | Not created | TODO   | Establishes scope, reason stability, privacy, and compatibility rules; may require an ADR.                     |
| 2     | #[To be assigned] - Implement current in-scope error events      | Not created | TODO   | Audits and implements the contract across the agreed HTTP, UDP, tracker-core, and REST boundaries.             |
| 3     | #[To be assigned] - Observe rejected HTTP announce IP parameters | Not created | TODO   | Implement only if subissue 1 includes this outcome; expected to be delivered with subissue 2 where applicable. |

## Delivery Strategy

The EPIC is intentionally deferred. Before any implementation, refine the
service scope and create subissue 1. The implementation must follow the
approved contract; it must not add isolated event variants simply to support a
single metric.

For each implementation subissue:

1. Run `linter all`, relevant tests, and pre-push checks when applicable.
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria against observed behavior before completion.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic draft created in `docs/issues/drafts/`
- [ ] Epic draft reviewed and approved by user/maintainer
- [ ] GitHub epic issue created and issue number added to this spec
- [ ] Error-event contract subissue created and linked
- [ ] Current in-scope error-event implementation subissue created and linked
- [ ] Rejected-`ip` observability decision revisited under the approved contract
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-08-19 00:00 UTC - Maintainer decision - Created draft after selecting #1987 Option B; no implementation is planned yet.

## Acceptance Criteria

- [ ] The accepted contract states which services and rejection/error phases
      emit events, including explicit exclusions.
- [ ] Event payloads expose only stable bounded reason types and minimum safe
      context; raw client-controlled values and implementation error composition do
      not become public payloads.
- [ ] The design states compatibility/versioning expectations for consumers.
- [ ] All current error paths within the accepted boundary emit the specified
      objective events consistently.
- [ ] Metrics and other consumers remain decoupled from request handling.
- [ ] The rejected HTTP announce `ip` case is either implemented consistently
      with the contract or explicitly deferred with a documented rationale.
- [ ] Every modified event/error API has semantic links to the governing design
      documents.
- [ ] Automated and manual verification evidence is recorded for each
      implementation subissue.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                           |
| ----- | ---------------------- | ---------------------------------- |
| AC1   | TODO                   | Approved design/ADR and subissue 1 |
| AC2   | TODO                   | Event payload and privacy review   |
| AC3   | TODO                   | Contract compatibility section     |
| AC4   | TODO                   | Per-service implementation tests   |
| AC5   | TODO                   | Architecture and integration tests |
| AC6   | TODO                   | Subissue 2/3 decision record       |
| AC7   | TODO                   | Source semantic-link review        |
| AC8   | TODO                   | CI and manual verification records |

## Risks and Trade-offs

- "All errors" is too broad without a precise boundary. The contract must name
  the included services and phases before implementation begins.
- Error enums often contain wrapped errors, dynamically formatted messages, or
  raw client input. Reusing them directly would leak unstable or sensitive data.
- Existing UDP error events and consumers must remain compatible while the
  contract is introduced or migrated.

## References

- Events architecture: `docs/events-architecture.md`
- Governing ADR: `docs/adrs/20260727000000_events_are_objective_facts.md`
- #1987 analysis: `docs/issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/error-event-observability-analysis.md`
