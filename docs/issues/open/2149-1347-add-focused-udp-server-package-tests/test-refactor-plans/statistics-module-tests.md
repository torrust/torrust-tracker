---
doc-type: test-refactor-plan
issue: 2149
package: torrust-tracker-udp-server
target-file: packages/udp-server/src/statistics/mod.rs
status: completed
semantic-links:
  related-artifacts:
    - packages/udp-server/src/statistics/mod.rs
    - packages/udp-server/src/statistics/metrics.rs
    - packages/udp-server/src/statistics/repository.rs
    - packages/udp-server/src/statistics/services.rs
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/coverage-evidence.md
    - docs/issues/open/2149-1347-add-focused-udp-server-package-tests/ISSUE.md
---

# UDP Statistics Module Test Assessment Plan

Follow the shared [purpose, quality goals, plan structure, and required two-phase
sequence](README.md). This assessment applies only to `packages/udp-server/src/statistics/mod.rs`.

## Phase 1 - Clean Current Tests

### Current state

The module declares UDP-server metric names and composes their counter/gauge descriptions in
`describe_metrics`. It contains no colocated tests. Clean baseline evidence already reports 52/52
lines, 60/60 regions, and 1/1 functions covered. Repository tests instantiate `Repository::new`,
which calls `describe_metrics`, and assert that the collection contains the expected metric types;
`metrics.rs` tests exercise aggregation/accessors; specialized event handlers exercise each metric
at its behavior-owning event boundary.

### Decision

Do not add direct module tests. A list-sized or all-metric registration test would duplicate the
repository's observable collection contract and turn a declarative registry into a brittle
implementation inventory. Individual metric naming, units, descriptions, and aggregation have
clear existing owners. No composition behavior remains that a direct test would make clearer.

## Phase 2 - Assess Missing Behavior Tests

### Strengths to preserve

1. This module owns only metric declaration/composition.
2. `statistics/repository.rs` owns repository initialization and observable metric collection
   registration.
3. `statistics/metrics.rs` owns aggregation and accessor behavior.
4. Specialized event handlers own event-to-metric updates.
5. `statistics/services.rs` owns service-level metric exposure.

### Problems and opportunities

#### P1 - No distinct module-level behavior is missing

**Decision.** No change. The observable outcome of composing the declarations is an initialized
repository collection, already tested at its owning initialization boundary. Testing the exact
sequence of eleven `describe_*` calls would assert implementation structure rather than a new
contract.

#### P2 - Metric details are already tested at narrower behavior-owning boundaries

**Decision.** Do not add per-metric tables or duplicate unit/description/type assertions here.
Repository tests own registration, `metrics.rs` owns values and aggregation, handlers own updates,
and service tests own exposure. The current full coverage is evidence of execution, not a reason
to manufacture another test layer.

## Proposed Refactorings

### R1 - Record fully-covered composition ownership

- **Status:** DONE
- **Priority:** Low impact / low effort
- **Change:** Document the no-test decision and ownership boundary.
- **Decision:** The module is fully covered indirectly and direct assertions would duplicate its
  repository/metrics/handler consumers. No test or production change is selected.
- **Done when:** The plan records why metric declaration composition has no additional direct test.

## Progress Tracking

### Plan Checklist

- [x] Statistics module, repository initialization, metrics, handlers, and services boundaries
      reviewed.
- [x] No-change decision recorded.
- [x] Plan completed and ready for final verification.

### Progress Log

- 2026-09-11 - GitHub Copilot - Completed this no-test assessment after confirming that metric
  declaration composition is fully covered through repository initialization and specialized
  metric behavior tests. No distinct module-level observable contract remains.

### Validation Evidence

| Increment | Status | Evidence |
| --- | --- | --- |
| R1 | DONE | Source review and existing clean baseline show 52/52 lines, 60/60 regions, and 1/1 functions covered. No code change is selected. |

## Non-Goals

- Do not change metric declarations, repository initialization, metric aggregation, event handlers,
  or services.
- Do not add duplicate declaration inventories, table tests, mocks, or percentage-only tests.

## Completion Criteria

- The fully covered metric-composition decision is documented.
- Metric registration, aggregation, update, and exposure behavior remains at its existing owner.
