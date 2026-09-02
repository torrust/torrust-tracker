---
doc-type: test-bootstrap-refactor-plan
issue: 1348
package: torrust-tracker-axum-http-server
target-files:
  - packages/axum-http-server/src/v1/handlers/announce.rs
  - packages/axum-http-server/src/v1/handlers/scrape.rs
status: draft
---

# Draft Plan — Shared Handler Test Bootstrap

Follow the shared [test-refactor-plan guidance](../README.md). This is a cross-file draft that must
be reviewed and explicitly approved before implementation. It is intentionally separate from the
file-local announce and scrape test plans.

## Purpose

Assess whether the duplicated **test infrastructure** in the announce and scrape handler modules
can be reduced without hiding the distinct dependencies of `AnnounceService` and `ScrapeService`.
This is not a plan to create a generic service factory.

## Phase 1 — Identify Problems

### B1 — Similar infrastructure is initialized in both handler modules

Both test modules select an HTTP tracker configuration and instance ID, then create in-memory
whitelist, key, and torrent repositories; authentication; HTTP statistics event infrastructure; and
an optional listener. These common concerns appear inside two independently maintained
`initialize_core_tracker_services` functions.

**Effect.** Changes to shared infrastructure can require parallel edits, while the large setup
functions make their common boundary hard to recognize.

### B2 — Service-specific dependencies legitimately differ

`announce.rs` additionally initializes database-backed download metrics, `AnnounceHandler`, and
whitelist authorization. `scrape.rs` creates `ScrapeHandler` and returns construction ingredients
used to instantiate `ScrapeService` in individual tests.

**Effect.** A generic bootstrap that returns every possible component would make dependencies
implicit, create optional fields, and weaken test expressiveness.

### B3 — Background statistics lifecycle needs explicit ownership

Both setups can start an event listener but do not retain its task handle. The listener is currently
not part of the focused handler assertions.

**Effect.** This is not a demonstrated flaky-test defect, but any shared context must make a
listener's necessity and lifecycle explicit rather than spreading detached background work.

## Phase 2 — Proposed Refactorings

### B1 — Map exact common infrastructure and lifecycle needs

- **Status:** TODO
- **Priority:** High impact / low effort
- **Change:** List each setup dependency as common, announce-specific, scrape-specific, or
  configuration-dependent. Verify whether the event listener is needed for the current focused
  handler contracts.
- **Guardrail:** Do not modify production code or introduce a common abstraction during this
  assessment.
- **Done when:** the plan records a minimal candidate responsibility and an explicit lifecycle
  decision.

### B2 — Prototype a narrow test-only context only if B1 supports it

- **Status:** TODO
- **Priority:** Medium impact / medium effort
- **Change:** If a cohesive boundary exists, introduce a private test-only context that owns only
  common infrastructure: core configuration, selected HTTP tracker configuration and instance ID,
  in-memory repositories, authentication, and necessary statistics sender/lifecycle.
- **Guardrail:** Each handler module must still visibly construct its own handler and service with
  service-specific dependencies. The context must not expose a generic `build_service` API or
  optional fields for unrelated behavior.
- **Done when:** both modules become simpler, dependencies remain readable at service construction,
  and focused tests retain fresh isolated state.

### B3 — Retain local mode and behavior fixtures unless separately justified

- **Status:** TODO
- **Priority:** Low impact / low effort
- **Change:** Decide whether mode-specific wrappers and common HTTP bindings remain local or have a
  broader, established test-support home.
- **Guardrail:** Do not move a fixture merely because it is duplicated twice. Shared ownership must
  be clearer than local ownership and must not broaden test coupling.
- **Done when:** the plan records a keep-local or shared-home decision with rationale.

## Progress Tracking

- [x] Draft created from comparison of announce and scrape handler test bootstraps.
- [ ] B1 assessment approved and completed.
- [ ] B2 approved, implemented, reviewed, and validated, or no-change decision recorded.
- [ ] B3 assessment completed and decision recorded.
- [ ] Maintainer reviewed all completed work.

### Progress Log

- 2026-09-02 - GitHub Copilot - Created this draft after identifying similar test infrastructure in
  announce and scrape handler modules. No implementation has been approved or performed.

## Non-Goals

- Do not create a universal handler or service factory.
- Do not move production bootstrap code merely to make tests shorter.
- Do not introduce shared mutable state, listeners without lifecycle ownership, retries, or sleeps.
- Do not merge this draft into a file-local plan without maintainer approval.
