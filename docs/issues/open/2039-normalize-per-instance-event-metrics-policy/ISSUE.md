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
    - tests/aggregate_stats_fixed_ports.rs
    - packages/events/src/bus.rs
    - packages/http-core/src/container.rs
    - packages/udp-core/src/container.rs
    - packages/udp-server/src/container.rs
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
  `tests/aggregate_stats_fixed_ports.rs`: UDP enabled/disabled listeners on
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

| ID  | Status | Task                             | Notes / Expected Output                                                                                                                                                                                                      |
| --- | ------ | -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Inventory event gates            | Map every `SenderStatus`, optional sender, listener, and UDP ban consumer.                                                                                                                                                   |
| T2  | TODO   | Consume #2036 canonical identity | Propagate stable runtime configuration-instance identity; do not use configured addresses. #2036 and #2041 are complete prerequisites.                                                                                       |
| T3  | TODO   | Always emit HTTP core facts      | Remove metrics-driven producer suppression while preserving event semantics.                                                                                                                                                 |
| T4  | TODO   | Always emit UDP core facts       | Remove metrics-driven producer suppression while preserving event semantics.                                                                                                                                                 |
| T5  | TODO   | Always emit UDP server facts     | Decouple the shared UDP server producer from the global metrics gate.                                                                                                                                                        |
| T6  | TODO   | Filter metrics in listeners      | Apply immutable identity-to-policy filtering before aggregate repository updates.                                                                                                                                            |
| T7  | TODO   | Preserve banning independence    | Verify cookie-error events reach the shared ban service for every listener policy.                                                                                                                                           |
| T8  | TODO   | Update REST metrics integration  | Preserve aggregate API counters and UDP operational metrics from filtered repositories.                                                                                                                                      |
| T9  | TODO   | Add focused tests                | Cover producer independence, listener filtering, and ban-listener behavior.                                                                                                                                                  |
| T10 | TODO   | Add application tests            | In `tests/aggregate_stats_fixed_ports.rs` and `tests/aggregate_stats_port_zero.rs`, enable deferred UDP fixed-port coverage and add enabled/disabled HTTP/UDP repeated-port-zero coverage; each expects aggregate count `1`. |
| T11 | TODO   | Validate and document            | Run focused tests, `linter all`, and the manual protocol; update evidence and architecture docs.                                                                                                                             |

## Risk-Based Manual Verification Protocol

Manual evidence is required after the highest-risk behavior changes rather than
after every individual implementation task. This avoids duplicating expensive
local tracker probes for tightly coupled internal refactorings while retaining
an externally observable control after each change that could affect production
behavior. The high-risk checkpoints are:

| Checkpoint | Tasks  | Risk controlled                                                                                                                                           | Required manual probes                                         |
| ---------- | ------ | --------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------- |
| C1         | T2-T6  | Events may lose canonical listener identity, metrics filtering may misclassify a listener, or shared aggregate repositories may receive the wrong events. | M1, M2, M4, and M5                                             |
| C2         | T7     | Always-emitted UDP security facts may no longer reach the shared banning listener.                                                                        | M3                                                             |
| C3         | T8-T10 | REST-visible aggregate metrics or application bootstrap/discovery may regress despite focused tests.                                                      | Repeat M1, M2, M4, and M5 against the final application build. |

For each checkpoint:

1. Select the smallest externally observable probe for the task.
2. Run it against the pre-change implementation and record configuration,
   commands, endpoints, and output in [evidence.md](evidence.md).
3. Implement the smallest change and run focused automated tests.
4. Repeat the unchanged probe and record the post-change output in
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
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
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

## Acceptance Criteria

- [ ] AC1: Metrics-disabled HTTP listeners emit facts but do not update aggregate HTTP metrics.
- [ ] AC2: Metrics-disabled UDP listeners emit core and server facts but do not update aggregate UDP metrics.
- [ ] AC3: Metrics-disabled UDP listeners still contribute relevant cookie-error facts to shared banning.
- [ ] AC4: Metrics-enabled listeners update the existing shared aggregate repositories.
- [ ] AC5: Metrics filtering uses #2036 canonical identity and works for repeated `0.0.0.0:0` blocks.
- [ ] AC6: The REST API retains aggregate HTTP/UDP and UDP operational metrics.
- [ ] AC7: Every risk-based verification checkpoint has baseline and post-change evidence.
- [ ] AC8: Relevant tests and `linter all` pass.

## Verification Plan

### Automatic Checks

- Focused tests for HTTP core, UDP core, UDP server metrics, and UDP banning listeners.
- Application-level enabled/disabled listener tests, including duplicate port-zero configuration.
- `cargo test --test stats -- --test-threads=1` until #1419 resolves test-process isolation.
- `linter all`.

### Manual Verification Scenarios

| ID  | Scenario                        | Expected Result                                                                  | Status | Evidence                   |
| --- | ------------------------------- | -------------------------------------------------------------------------------- | ------ | -------------------------- |
| M1  | HTTP policy filtering           | One enabled and one disabled listener produce aggregate announce count `1`.      | TODO   |                            |
| M2  | UDP policy filtering            | One enabled and one disabled listener produce aggregate UDP announce count `1`.  | TODO   |                            |
| M3  | UDP banning independence        | Invalid cookies through a disabled listener still update shared ban enforcement. | TODO   |                            |
| M4  | Duplicate port-zero identity    | Policy follows runtime identity rather than configured address.                  | TODO   |                            |
| M5  | Fixed-port UDP policy filtering | One enabled and one disabled listener produce aggregate UDP announce count `1`.  | TODO   | [evidence.md](evidence.md) |

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
