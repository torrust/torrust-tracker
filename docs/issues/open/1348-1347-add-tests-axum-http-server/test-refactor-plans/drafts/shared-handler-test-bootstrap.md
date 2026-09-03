---
doc-type: test-bootstrap-refactor-plan
issue: 1348
package: torrust-tracker-axum-http-server
target-files:
  - packages/axum-http-server/src/v1/handlers/announce.rs
  - packages/axum-http-server/src/v1/handlers/scrape.rs
  - packages/axum-http-server/src/server.rs
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

### Why the tests do not reuse the production bootstrap

The test bootstraps deliberately duplicate composition instead of calling the production container
factories (for example `HttpTrackerCoreContainer::initialize_from_tracker_core`). This decision was
made when the tests were written and remains valid:

- **Fewer dependencies per test.** A test instantiates only the services it exercises, instead of
  the whole tracker with its persistence, statistics, and coordination dependencies.
- **Explicit coupling.** The test setup shows exactly which dependencies the unit under test needs,
  which documents (and pressures) its real coupling.
- **Faster execution.** Constructing fewer services keeps unit tests cheap.

Any shared test bootstrap must preserve these properties. Reusing production composition is a
non-goal; the goal is to remove duplicated _construction detail_ while each test still selects and
constructs only what it needs.

## Phase 1 — Identify Problems

### B1 — Similar infrastructure is initialized in both handler modules

Both test modules select an HTTP tracker configuration and instance ID, then create in-memory
whitelist, key, and torrent repositories plus authentication. These common concerns appear inside
two independently maintained `initialize_core_tracker_services` functions.

**Effect.** Changes to shared infrastructure can require parallel edits, while the large setup
functions make their common boundary hard to recognize.

### B2 — Service-specific dependencies legitimately differ

`announce.rs` additionally initializes database-backed download metrics, `AnnounceHandler`, and
whitelist authorization. `scrape.rs` creates `ScrapeHandler` and returns construction ingredients
used to instantiate `ScrapeService` in individual tests.

**Effect.** A generic bootstrap that returns every possible component would make dependencies
implicit, create optional fields, and weaken test expressiveness.

### B3 — Statistics infrastructure is no longer a shared bootstrap concern

The focused scrape setup now passes no event sender because its tests do not assert statistics.
The announce setup still creates statistics infrastructure because its service setup currently
retains it.

**Effect.** Statistics initialization is not a cohesive cross-file responsibility. Any future shared
context must not reintroduce listener work into scrape tests that do not assert it.

### B4 — `server.rs` is a third bootstrap consumer with a different shape

`server.rs::initialize_container` is a third near-duplicate bootstrap. Unlike the handler modules,
it must produce a complete `HttpTrackerCoreContainer` because `HttpServer::start` consumes the whole
container. It therefore composes the statistics event bus and optional listener, the swarm
coordination registry, the persistence-backed `TrackerCoreContainer`, and both services.

**Effect.** This is the "third consumer" trigger named in B2. The overlap with the handler
bootstraps is real (configuration selection, instance ID, `TrackerCoreContainer` ingredients,
`*Service::new_with_http_tracker_config` calls), but the required output differs: the server needs
everything, while the handler tests need one service each. A shared helper must be composable so
that handler tests can still stop early and skip what they do not use.

## Phase 2 — Proposed Refactorings

### B1 — Map exact common infrastructure and lifecycle needs

- **Status:** DONE
- **Priority:** High impact / low effort
- **Change:** List each setup dependency as common, announce-specific, scrape-specific, or
  configuration-dependent. Verify whether the event listener is needed for the current focused
  handler contracts.
- **Guardrail:** Do not modify production code or introduce a common abstraction during this
  assessment.
- **Decision:** The remaining common setup is configuration selection, in-memory repositories, and
  authentication. However, it does not form a useful standalone test fixture: announce needs
  whitelist authorization, persistence metrics, and `AnnounceHandler`; scrape needs
  `ScrapeHandler` and now deliberately has no statistics sender. Extracting the common objects
  would require a bundle that exposes construction ingredients rather than a behavior-focused
  capability.
- **Done when:** the remaining shared setup and explicit no-extraction rationale are recorded.

### B2 — Prototype a narrow test-only context only if B1 supports it

- **Status:** DEFERRED
- **Priority:** Medium impact / medium effort
- **Change:** If a cohesive boundary exists, introduce a private test-only context that owns only
  common infrastructure: core configuration, selected HTTP tracker configuration and instance ID,
  in-memory repositories, authentication, and necessary statistics sender/lifecycle.
- **Guardrail:** Each handler module must still visibly construct its own handler and service with
  service-specific dependencies. The context must not expose a generic `build_service` API or
  optional fields for unrelated behavior.
- **Decision:** Deferred. B1 found no cohesive shared responsibility that would make both modules
  clearer today. Revisit only if a third consumer needs the same test infrastructure or if a
  concrete shared capability emerges without optional fields.
- **Reassessment (B4 trigger):** `server.rs` is now the third consumer, so B2 must be reassessed.
  The candidate shape is **layered, composable helpers** rather than one context object, so each
  test stops at the layer it needs:
  1. `selected_http_tracker_config(&Configuration) -> (Arc<HttpTracker>, ConfigurationInstanceId)`
     — the configuration selection every consumer repeats.
  2. In-memory core ingredients (repositories, authentication, whitelist authorization, handlers)
     built without persistence — used by handler tests.
  3. Service constructors stay **at the call site** in each test module so service-specific
     dependencies remain visible.
  4. Only `server.rs` composes the full `HttpTrackerCoreContainer` from the layers above, because
     only it needs the whole container.
     Statistics sender/listener remain an explicit per-test choice (`None` where not asserted). This
     is still a proposal; implementation requires a separate approval after the server plan closes.
- **Done when:** the layered shape is approved or rejected and the decision is recorded.

### B3 — Retain local mode and behavior fixtures unless separately justified

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Decide whether mode-specific wrappers and common HTTP bindings remain local or have a
  broader, established test-support home.
- **Guardrail:** Do not move a fixture merely because it is duplicated twice. Shared ownership must
  be clearer than local ownership and must not broaden test coupling.
- **Decision:** Keep fixtures local. `sample_http_service_binding()` appears in both files but has
  only two consumers with different neighboring fixtures and test contexts. The mode wrappers also
  directly express each handler's supported service configuration. A shared fixture module would
  increase coupling without a clearer owner.
- **Done when:** the keep-local decision is recorded.

## Progress Tracking

- [x] Draft created from comparison of announce and scrape handler test bootstraps.
- [x] B1 assessment completed.
- [x] B2 deferred with a concrete reassessment trigger.
- [x] B3 assessment completed.
- [x] Maintainer reviewed the assessment decisions.
- [x] B4 recorded `server.rs` as the third consumer and documented why tests avoid the production
      bootstrap.
- [ ] Maintainer approved or rejected the layered-helper shape proposed under B2.

### Progress Log

- 2026-09-02 - GitHub Copilot - Created this draft after identifying similar test infrastructure in
  announce and scrape handler modules. No implementation has been approved or performed.
- 2026-09-02 - GitHub Copilot - Reassessed after completing the scrape-file plan. No cross-file
  extraction is currently justified: the remaining overlap is construction detail, while handler
  dependencies and statistics requirements differ. The draft remains deferred for a future concrete
  trigger.
- 2026-09-02 - User/maintainer - Reviewed and approved the deferred assessment. Revisit only when
  a concrete shared test capability or a third consumer justifies it.
- 2026-09-03 - GitHub Copilot - Recorded `server.rs::initialize_container` as the third bootstrap
  consumer (B4) and documented the maintainer's rationale for not reusing the production container
  factories. Proposed a layered-helper shape under B2 for later approval; nothing implemented.

## Non-Goals

- Do not create a universal handler or service factory.
- Do not move production bootstrap code merely to make tests shorter.
- Do not replace test bootstraps with the production container factories; tests must keep
  constructing only the services they need.
- Do not introduce shared mutable state, listeners without lifecycle ownership, retries, or sleeps.
- Do not merge this draft into a file-local plan without maintainer approval.
