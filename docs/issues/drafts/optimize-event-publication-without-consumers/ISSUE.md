---
doc-type: issue
issue-type: enhancement
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
branch: "{issue-number}-optimize-event-publication-without-consumers"
related-pr: null
last-updated-utc: 2026-08-18
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/events/src/bus.rs
    - packages/http-core/src/container.rs
    - packages/udp-core/src/container.rs
    - packages/udp-server/src/container.rs
    - src/container.rs
    - docs/architecture/events.md
    - docs/issues/closed/2039-normalize-per-instance-event-metrics-policy/ISSUE.md
    - docs/issues/closed/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Investigate Event Publication Performance and Consumer Demand

## Goal

Measure the practical cost of always publishing tracker events, inventory every
consumer required for each event family, and decide from evidence whether a
consumer-demand optimization is justified.

## Background

`EventBus` currently exposes an absent sender through `SenderStatus::Disabled`.
Producers that receive no sender skip event construction and publication. This
is preferable to a no-op sender because it makes the absent-consumer state
explicit and avoids a broadcast attempt.

Issue #2039 normalizes correctness: metrics-disabled listeners must still
produce policy-neutral facts for the shared stream. Those facts can be consumed
by metrics listeners and, for UDP-server cookie errors, by the banning listener.
Therefore #2039 must keep event publication enabled even when a listener's own
metrics policy is disabled.

Disabling per-instance metrics is not sufficient to disable publication. For an
event family to have no required consumer, all of its aggregate metrics
consumers, non-metrics consumers, and required counters must be inactive. For
UDP-server events this includes the banning listener and the cookie-error facts
and counters that it requires, even when ban enforcement is configured as
disabled. The exact service inventory and performance effect are not yet known.

This is an investigation draft, not an approved implementation issue. It is
independent of Issue #2039 correctness and is scheduled for reconsideration
after the final Issue #2035 verification. Do not create a GitHub issue or begin
implementation until benchmark evidence and the required consumer inventory
support the work.

## Scope

### In Scope

- Inventory the HTTP-core, UDP-core, and UDP-server event producers,
  aggregate-metrics consumers, banning consumers, and auxiliary counters.
- Benchmark representative tracker workloads with event publication enabled and
  with controlled instrumentation that quantifies publication work.
- Establish whether event construction and broadcast are a material bottleneck.
- Define candidate configurations that could safely have no required consumers,
  including the conditions for disabling aggregate metrics, banning, and
  banning-related counters.
- Evaluate an immutable bootstrap-time consumer-demand plan only if the
  benchmark demonstrates a material benefit.
- Preserve absent-sender injection rather than introducing a no-op sender if a
  later implementation is approved.

### Out of Scope

- Changing the semantic contents of HTTP, UDP-core, or UDP-server events.
- Per-listener metrics filtering and canonical identity propagation, owned by
  #2039.
- Changing connection-ID validation or shared ban-service semantics.
- Implementing consumer-demand optimization before benchmark evidence and
  explicit maintainer approval.
- Runtime subscription counting, dynamic listener registration, configuration
  reload, or an event bus per listener.
- Using compiler dead-code elimination to solve runtime configuration costs.

## Design Direction

First collect evidence. Benchmark a realistic HTTP and UDP workload while
recording event construction, clone, and broadcast behavior through controlled
instrumentation. Do not assume that publication overhead is material without
measurement.

If the evidence justifies a future implementation, determine publication demand
once during bootstrap from immutable configured services and consumers. Do not
infer it from Tokio receiver counts: listeners start asynchronously, and live
receiver counts would make event publication dependent on startup timing.

The plan must distinguish event family and consumer type:

| Event family | Publication demand                                                                                                                          |
| ------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| HTTP core    | All aggregate metrics and every other registered HTTP-core consumer must be inactive.                                                       |
| UDP core     | All aggregate metrics and every other registered UDP-core consumer must be inactive.                                                        |
| UDP server   | All aggregate metrics, banning, cookie-error counters required by banning, and every other registered UDP-server consumer must be inactive. |

The draft must explicitly resolve whether disabled connection-ID validation
still requires cookie-error facts and counters. Current documentation says it
does, so it is not itself a sufficient condition to disable UDP-server
publication.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                             | Notes / Expected Output                                                                                                        |
| --- | ------ | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Inventory consumers and counters | Map every producer, metrics consumer, banning consumer, counter, and bootstrap location for the three shared event families.   |
| T2  | TODO   | Define measurable workloads      | Select realistic HTTP and UDP workloads plus controlled instrumentation for event publication work.                            |
| T3  | TODO   | Capture baseline performance     | Record throughput, latency, CPU, and event-publication measurements with #2039 behavior enabled.                               |
| T4  | TODO   | Analyze optimization feasibility | Identify configurations that could safely have no required consumer and the configuration/service changes they require.        |
| T5  | TODO   | Decide whether to implement      | Obtain maintainer approval from the evidence; either promote this draft to an implementation issue or close it as not planned. |

## Progress Tracking

### Workflow Checkpoints

- [x] Investigation draft created in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] Benchmark evidence reviewed by user/maintainer
- [ ] Decision recorded to create an implementation issue or close this draft as not planned
- [ ] #2035 final verification completed; implementation scheduling prerequisite

### Progress Log

- 2026-08-18 UTC - agent and user - Converted from an implementation proposal to an investigation draft. #2039 must always publish policy-neutral facts for correctness; measure the cost and map every consumer/counter before deciding whether a total-publication-disable optimization is worthwhile.

## Acceptance Criteria

- [ ] AC1: A documented inventory identifies every producer, consumer, and required counter for each event family.
- [ ] AC2: Benchmark evidence quantifies the cost of event publication under representative HTTP and UDP workloads.
- [ ] AC3: The investigation identifies the exact configuration and service conditions required to have no consumer for each event family.
- [ ] AC4: The analysis shows whether disabled connection-ID validation still needs UDP cookie-error facts and counters.
- [ ] AC5: A maintainer-approved decision records whether an optimization implementation issue is justified.
- [ ] AC6: Documentation records the evidence and decision.

## Verification Plan

### Automatic Checks

- Focused inspection and tests that validate the consumer/counter inventory.
- Benchmark or controlled instrumentation covering event construction and
  publication behavior.
- `linter all` for the investigation documentation.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Command/Steps                                                                                                                         | Expected Result                                                                                 | Status | Evidence                     |
| --- | ------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | ------ | ---------------------------- |
| M1  | HTTP publication baseline             | Run a representative local HTTP announce workload with #2039 behavior enabled and controlled event instrumentation.                   | Evidence records throughput, latency, CPU, and event-publication work.                          | TODO   | Investigation evidence file. |
| M2  | UDP publication baseline              | Run a representative local UDP announce and invalid-cookie workload with #2039 behavior enabled and controlled event instrumentation. | Evidence records throughput, latency, CPU, cookie-error processing, and event-publication work. | TODO   | Investigation evidence file. |
| M3  | Consumer and counter inventory review | Trace each event family from producer through metrics, banning, and required counters.                                                | Evidence identifies the complete conditions for a potential no-consumer configuration.          | TODO   | Investigation evidence file. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |

## Risks and Trade-offs

- Assuming that disabled per-instance metrics disable every consumer would recreate #2039's banning defect. Mitigation: inventory metrics, banning, and counters before proposing a demand plan.
- An optimization may add configuration complexity without a measurable benefit. Mitigation: benchmark before proposing implementation.
- Runtime receiver counts can transiently report zero during asynchronous startup. Mitigation: exclude live subscription counting from any future design unless independently justified.
- Optimization work could delay correctness. Mitigation: keep this as a draft and do not make it a #2039 prerequisite.

## References

- Related issues: Issue #2035 and Issue #2039
- Related PRs: PR #2044 and PR #2048
- Events architecture: `docs/architecture/events.md`
- Event bus: `packages/events/src/bus.rs`
