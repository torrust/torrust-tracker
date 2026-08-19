---
doc-type: issue
issue-type: bug
status: open
priority: p1
github-issue: 2039
spec-path: docs/issues/open/2039-normalize-per-instance-event-metrics-policy/ISSUE.md
branch: "2039-normalize-per-instance-event-metrics-policy"
related-pr: null
last-updated-utc: 2026-08-18
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/events-architecture.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - docs/issues/open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
    - docs/issues/closed/2036-add-runtime-service-registry-metadata/ISSUE.md
    - docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md
    - evidence.md
    - tests/metrics/fixed_ports.rs
    - tests/metrics/port_zero.rs
    - tests/metrics/udp_error_enabled_port_zero.rs
    - tests/metrics/udp_error_disabled_port_zero.rs
    - tests/banning/udp_metrics_disabled_port_zero.rs
    - packages/events/src/bus.rs
    - packages/http-core/src/container.rs
    - packages/udp-core/src/container.rs
    - packages/udp-server/src/container.rs
    - packages/http-core/src/event.rs
    - packages/udp-core/src/event.rs
    - packages/http-core/src/statistics/event/listener.rs
    - packages/udp-core/src/statistics/event/listener.rs
    - packages/udp-server/src/statistics/event/listener.rs
    - src/bootstrap/jobs/http_tracker_core.rs
    - src/bootstrap/jobs/udp_tracker_core.rs
    - src/bootstrap/jobs/udp_tracker_server.rs
---

<!-- skill-link: create-issue -->

# Issue #2039 - Normalize Per-Instance Event Metrics Policy

## Goal

Make `tracker_usage_statistics` control metrics processing for an individual
public HTTP or UDP listener, without suppressing objective events or UDP ban
enforcement.

## Background

[#1263][1263] and [#1401][1401] establish the intended operator model:
aggregate metrics remain available, while each public listener can opt in or
out through `tracker_usage_statistics`.

### Concrete UDP Failure Example

Consider two public UDP listeners:

```toml
[[udp_trackers]]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = false

[[udp_trackers]]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = true
```

Both listeners correctly serve connect and announce requests. However, after
one announce to each listener, the REST API's aggregate
`udp4_announces_handled` counter is currently `2`, not `1`.

The REST API reads this counter from the UDP **server** metrics repository. Its
metrics listener receives `UdpRequestAccepted` events from one application-wide
UDP server event bus, with no per-listener metrics policy. The configuration
option therefore does not suppress server-layer metrics for the disabled
listener.

The old implementation tried to disable metrics by suppressing event producers:
an `EventBus` returns no sender when statistics are disabled. This was
reasonable when events existed only to generate metrics. It is no longer valid:
UDP server events are generic objective facts and a separate banning listener
also consumes cookie-error events from that stream. Suppressing the stream to
avoid metrics would also prevent current or future non-metrics consumers from
observing those facts.

There is deliberately one aggregate metrics repository per layer, rather than
one repository per public listener. The repository does not currently filter
events by configuration policy; the listener increments counters from every
event it receives. Therefore, preserving aggregate repositories while allowing
per-listener metrics requires listener-side filtering before repository mutation.

This issue replaces producer-side metrics suppression with always-emitted facts
and listener-side metrics policy. The UDP server is the failure that exposes the
problem, but HTTP core and UDP core must follow the same normalized rule.

The prerequisites are [#2036][2036], which defines canonical runtime service
and configuration-instance identity, and the registry metadata migration that
exposes that identity for started services. A configured address cannot identify
a listener because repeated `0.0.0.0:0` blocks are valid. This issue must use
the canonical identity rather than create a competing identity.

## Scope

### In Scope

- Always emit objective HTTP core, UDP core, and UDP server events.
- Carry #2036 canonical runtime identity on metric-relevant events.
- Filter metrics in HTTP core, UDP core, and UDP server listeners before their
  shared aggregate repositories are updated.
- Keep UDP banning independent of metrics policy and subscribed to all relevant
  cookie-error events.
- Add focused and application-level regressions for enabled and disabled
  listeners, including duplicate port-zero configuration blocks.
- Add the deferred aggregate-statistics cases in
  `tests/metrics/fixed_ports.rs`: UDP enabled/disabled listeners on
  distinct fixed ports, then HTTP and UDP listeners with repeated port-zero
  bindings after bootstrap identity is available.
- Record manual baseline and post-change evidence at the risk-based
  verification checkpoints.

### Out of Scope

- Per-listener repositories or a public per-listener metrics API.
- A persistent user-supplied listener ID.
- Changing shared ban-service semantics.
- Replacing the runtime registry work owned by #2036.
- Migrating registry metadata; owned by the dedicated follow-up issue.

## Design Direction

The application retains one aggregate repository per event layer. Producers
always publish facts with canonical listener identity. A metrics listener uses
that identity to find the listener's immutable metrics policy and ignores a
disabled listener before repository mutation. The UDP banning listener receives
the same security events regardless of that policy.

To fix this issue, producers must always publish policy-neutral facts for every
relevant listener, independent of individual listener metrics policy. Metrics
policy is applied only by metrics listeners, while the UDP banning listener
continues to receive relevant security facts. This correctness delivery does not
attempt to disable publication when no consumer is active. The follow-up draft
specification at
[`docs/issues/drafts/optimize-event-publication-without-consumers/ISSUE.md`](../../drafts/optimize-event-publication-without-consumers/ISSUE.md)
will first measure the performance effect of publication and define whether a
safe consumer-demand optimization is worthwhile. It does not block this issue's
correctness delivery.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                             | Notes / Expected Output                                                                                               |
| --- | ------ | -------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Inventory event gates            | Mapped event buses, optional senders, metrics listeners, and the UDP banning consumer during implementation analysis. |
| T2  | DONE   | Consume #2036 canonical identity | Propagated stable runtime configuration-instance identity without using configured addresses.                         |
| T3  | DONE   | Always emit HTTP core facts      | HTTP core producer publication is independent of listener metrics policy.                                             |
| T4  | DONE   | Always emit UDP core facts       | UDP core producer publication is independent of listener metrics policy.                                              |
| T5  | DONE   | Always emit UDP server facts     | UDP server publication is independent of listener metrics policy.                                                     |
| T6  | DONE   | Filter metrics in listeners      | Shared HTTP, UDP-core, and UDP-server metrics listeners use immutable identity-to-policy filtering.                   |
| T7  | DONE   | Preserve banning independence    | Full-application regression proves cookie errors through a metrics-disabled UDP listener still trigger a shared ban.  |
| T8  | DONE   | Update REST metrics integration  | REST announce aggregates and deterministic UDP operational counters are verified.                                     |
| T9  | DONE   | Add focused tests                | Added producer, filtering, banning, and enabled-error identity coverage.                                              |
| T10 | DONE   | Add application tests            | Fixed-port routing and isolated port-zero policy binaries cover enabled/disabled traffic, errors, and banning.        |
| T11 | DONE   | Validate and document            | Captured an initial baseline, recorded final manual verification, and ran linting and focused tests.                  |

## Risk-Based Manual Verification Protocol

Manual evidence consists of one baseline before the correctness implementation
and one final verification after it. Intermediate checkpoints are optional
safety controls rather than mandatory evidence records: the final application
implementation and its regression suite provide the release decision. This
avoids duplicating expensive local probes while retaining an externally
observable before-and-after comparison.

| Checkpoint | Timing                  | Risk controlled                                                                                                        | Required manual probes         |
| ---------- | ----------------------- | ---------------------------------------------------------------------------------------------------------------------- | ------------------------------ |
| Baseline   | Before implementation   | Establishes the observable pre-change metrics and banning behavior.                                                    | M1, M2, M3, and M5 where valid |
| Final      | Final application build | Confirms the complete implementation, including identity filtering, banning, REST aggregates, and operational metrics. | M1, M2, M3, M4, and M5         |

For each required checkpoint:

1. Select the smallest externally observable probe for the task.
2. Run it against the pre-change implementation and record configuration,
   commands, endpoints, and output in [evidence.md](evidence.md).
3. Complete the implementation and run focused automated tests.
4. Repeat the unchanged probe against the final application build and record the post-change output in
   [evidence.md](evidence.md).
5. Compare the two records. Explain every intentional difference and add a
   regression before advancing; stop to diagnose every unexpected difference.

M1, M2, M4, and M5 must verify that a metrics-disabled listener does not update
aggregate metrics. M3 must verify that invalid UDP cookies through a
metrics-disabled listener still reach shared ban enforcement.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created: #2039
- [ ] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all` and relevant tests; pre-push checks remain pending)
- [x] Manual verification scenarios executed and recorded (baseline and final application evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-28 20:30 UTC - agent - Drafted from the #2035 manual verification finding and #1263/#1401 historical intent.
- 2026-07-29 00:00 UTC - agent - Converted to folder-style specification and added the progressive manual evidence protocol.
- 2026-07-29 07:10 UTC - agent - User approved the specification; created GitHub issue #2039 and moved this specification to `docs/issues/open/`.
- 2026-07-29 18:14 UTC - user - Separated the fixed-port UDP aggregate-metrics defect from #2035's
  duplicate-port-zero bootstrap collision. The former is a #2039 regression; the latter must be
  combined with #2035 before repeated-port-zero aggregate-statistics tests are enabled.
- 2026-08-18 - user - Confirmed that #2039 must be implemented before #2035 can complete its
  final verification. Replaced per-task manual evidence with risk-based checkpoints: after the
  combined identity/event/filtering change, after UDP banning independence, and after final
  REST/application integration.
- 2026-08-18 - agent - Implemented listener-identity propagation, policy-neutral event publication,
  listener-side metrics filtering, and full-application banning regression coverage. Fixed-port and
  repeated-port-zero probes are recorded in `evidence.md`; remaining evidence requirements are tracked
  as blockers rather than inferred from automated coverage.
- 2026-08-18 - agent - Captured the fixed-port pre-change baseline from isolated revision
  `e6b99635`; it counted both disabled and enabled listeners (`2`) compared with the post-change
  aggregate count (`1`). Added final operational-counter assertions and corrected UDP error-event
  identity propagation.
- 2026-08-19 - agent - Added and executed a tracked manual invalid-cookie probe against the
  metrics-disabled UDP listener. It observed eleven cookie-error responses, twelfth-request ban
  enforcement, and REST `udp_banned_ips_total: 1`.

## Acceptance Criteria

- [x] AC1: Metrics-disabled HTTP listeners emit facts but do not update aggregate HTTP metrics.
- [x] AC2: Metrics-disabled UDP listeners emit core and server facts but do not update aggregate UDP metrics.
- [x] AC3: Metrics-disabled UDP listeners still contribute relevant cookie-error facts to shared banning.
- [x] AC4: Metrics-enabled listeners update the existing shared aggregate repositories.
- [x] AC5: Metrics filtering uses #2036 canonical identity and works for repeated `0.0.0.0:0` blocks.
- [x] AC6: The REST API retains aggregate HTTP/UDP and UDP operational metrics.
- [x] AC7: The baseline and final application verification checkpoints have recorded evidence.
- [x] AC8: Relevant tests and `linter all` pass.

## Verification Plan

### Automatic Checks

- Focused tests for HTTP core, UDP core, UDP server metrics, and UDP banning listeners.
- Application-level enabled/disabled listener tests, including duplicate port-zero configuration.
- `cargo test --test stats -- --test-threads=1` until #1419 resolves test-process isolation.
- `linter all`.

### Manual Verification Scenarios

| ID  | Scenario                        | Expected Result                                                                  | Status | Evidence                   |
| --- | ------------------------------- | -------------------------------------------------------------------------------- | ------ | -------------------------- |
| M1  | HTTP policy filtering           | One enabled and one disabled listener produce aggregate announce count `1`.      | DONE   | [evidence.md](evidence.md) |
| M2  | UDP policy filtering            | One enabled and one disabled listener produce aggregate UDP announce count `1`.  | DONE   | [evidence.md](evidence.md) |
| M3  | UDP banning independence        | Invalid cookies through a disabled listener still update shared ban enforcement. | DONE   | [evidence.md](evidence.md) |
| M4  | Duplicate port-zero identity    | Policy follows runtime identity rather than configured address.                  | DONE   | [evidence.md](evidence.md) |
| M5  | Fixed-port UDP policy filtering | One enabled and one disabled listener produce aggregate UDP announce count `1`.  | DONE   | [evidence.md](evidence.md) |

## References

- [Events architecture](../../../events-architecture.md)
- [#1263][1263]
- [#1401][1401]
- [#2035][2035]
- [#2036][2036]

[1263]: https://github.com/torrust/torrust-tracker/issues/1263
[1401]: https://github.com/torrust/torrust-tracker/issues/1401
[2035]: https://github.com/torrust/torrust-tracker/issues/2035
[2036]: https://github.com/torrust/torrust-tracker/issues/2036
