---
doc-type: issue
issue-type: bug
status: open
priority: p1
github-issue: 2035
spec-path: docs/issues/open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
branch: 2035-fix-duplicate-port-zero-tracker-instance-bootstrap
related-pr: null
last-updated-utc: 2026-07-29 18:14
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - src/container.rs
    - src/app.rs
    - archived-attempt.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/open/2036-add-runtime-service-registry-metadata/ISSUE.md
    - docs/issues/open/2039-normalize-per-instance-event-metrics-policy/ISSUE.md
    - docs/issues/open/2041-migrate-runtime-service-registry-metadata/ISSUE.md
    - docs/events-architecture.md
    - evidence.md
    - tests/servers/api/contract/stats/mod.rs
  related-issues:
    - 1419
    - 2036
    - 2039
    - 2041
---

# Issue #2035 - Fix Duplicate Port-Zero Tracker Instance Bootstrap

## Goal

Start every configured HTTP and UDP tracker instance with its own configuration, including when
multiple same-protocol blocks use the same configured port-zero bind address.

## Background

`AppContainer` stores HTTP and UDP instance containers in `HashMap<SocketAddr, _>`, keyed by each
configuration block's `bind_address`. `HashMap::insert` replaces the previous value for an equal
key. Consequently, two HTTP tracker blocks both configured as `0.0.0.0:0` leave only the later
container in the map.

Application startup then iterates both configuration blocks and looks up a container using the
same configured address. Both services start using the surviving later configuration, even though
the operating system gives each listener a distinct final port. The same defect exists for UDP
trackers. This can silently apply the wrong per-instance behavior, for example
`tracker_usage_statistics`, TLS, or network settings.

The local reproduction is recorded in [evidence.md](evidence.md).

## Scope

### In Scope

- Preserve each configured HTTP and UDP tracker instance even when configured bind addresses are equal.
- Replace address-keyed instance-container storage with an order-preserving representation aligned
  with configuration entries, or an equivalent stable configuration-instance identifier.
- Start each configured HTTP and UDP instance with its matching container.
- Include the configuration instance index in HTTP and UDP bootstrap lifecycle logs, including
  events that report configured and final bound addresses.
- Add regressions with repeated `0.0.0.0:0` blocks whose configuration differs.
- Add fixed-port HTTP statistics coverage that proves a disabled listener does
  not contribute after it receives its own configuration. Keep aggregate
  statistics coverage for repeated port-zero bindings deferred until #2039.

### Out of Scope

- Runtime registry metadata or health-check API changes.
- Public endpoint, proxy, or DNS configuration.
- User-supplied persistent service IDs in configuration.

## Archived Attempt / Revised Delivery Plan

The old implementation attempt lives on reference branch
`archive/2035-bootstrap-identity-attempt`. It must not merge. Its evidence and
the pause decision are recorded in [archived-attempt.md](archived-attempt.md).

The attempt showed that bootstrap identity alone cannot make per-listener UDP
metrics policy correct: the UDP server has one application-wide event bus and
repository, while producer-side metrics suppression can hide facts required by
the independent banning listener.

Issue [#2035](https://github.com/torrust/torrust-tracker/issues/2035) is
delivered in two phases. After #2036 defines canonical runtime
service/configuration-instance identity, this issue can reimplement bootstrap
identity preservation and prove that each duplicate port-zero configuration
starts with its matching container. This phase must not introduce registry
metadata or metrics-policy behavior.

After bootstrap identity propagation is merged, [#2041](../2041-migrate-runtime-service-registry-metadata/ISSUE.md)
will carry the same identity through started-service registration metadata, and
Issue #2039 will make event publication independent of metrics policy and filter
metrics in listeners by canonical identity. Those follow-ups are prerequisites
only for this issue's metrics-related final verification and closure.

## Implementation Plan

| ID  | Status  | Task                                                                                                   | Notes / Expected Output                                                                                                                                          |
| --- | ------- | ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE    | Land [#2036](../2036-add-runtime-service-registry-metadata/ISSUE.md) canonical identity                | Bootstrap identity aligns with the canonical runtime identity contract.                                                                                          |
| T2  | TODO    | Replace address-keyed container lookup                                                                 | Use an order-preserving representation or canonical identity, not configured `SocketAddr`.                                                                       |
| T3  | TODO    | Start matching containers                                                                              | Pass each configuration entry's matching container into HTTP and UDP startup.                                                                                    |
| T4  | TODO    | Correlate lifecycle logs                                                                               | Include canonical identity with configured and final binding logs.                                                                                               |
| T5  | TODO    | Add HTTP statistics integration coverage                                                               | In `tests/servers/api/contract/stats/mod.rs`, add a fixed-port enabled/disabled HTTP test that expects aggregate count `1`.                                      |
| T6  | TODO    | Add bootstrap regressions                                                                              | Cover duplicate port-zero HTTP/UDP configuration-to-container correspondence without asserting aggregate metrics policy.                                         |
| T7  | TODO    | Run and record local tracker probes                                                                    | Run fixed-port HTTP and duplicate-port-zero bootstrap scenarios locally; append configuration, commands, outputs, and comparisons to [evidence.md](evidence.md). |
| T8  | BLOCKED | Land registry metadata migration                                                                       | #2041 must expose started-service canonical identity before final verification.                                                                                  |
| T9  | BLOCKED | Land [#2039](../2039-normalize-per-instance-event-metrics-policy/ISSUE.md) event-metrics normalization | Required only for deferred UDP and repeated-port-zero aggregate-statistics tests, final verification, and issue closure.                                         |

## Progress Tracking

### Workflow Checkpoints

- [x] Specification drafted and approved by user/maintainer
- [x] GitHub issue created: #2035
- [x] Prerequisite #2036 completed
- [ ] Bootstrap identity preservation completed
- [ ] Registry metadata migration completed
- [ ] Event-metrics normalization completed
- [ ] Final automatic and manual verification completed
- [ ] Acceptance criteria reviewed after implementation

### Progress Log

- 2026-07-28 14:51 UTC - agent - User-approved specification promoted to GitHub issue #2035;
  the ignored HTTP stats-contract regression and its current `2 != 1` failure are recorded in
  [evidence.md](evidence.md).
- 2026-07-29 00:00 UTC - agent - Archived the prior implementation attempt and deferred
  implementation until #2036 and event-metrics normalization are complete.
- 2026-07-29 16:51 UTC - agent - Clarified the two-phase delivery order from the #2036 handoff:
  bootstrap identity propagation begins after #2036; #2041 and #2039 are required only for
  metrics-related final verification and closure.
- 2026-07-29 18:14 UTC - user - Distinguished bootstrap configuration collision from UDP server aggregate
  metrics filtering. Fixed-port HTTP statistics coverage belongs to this phase; UDP aggregate
  statistics and repeated-port-zero aggregate statistics remain deferred to #2039.

## Acceptance Criteria

- [ ] AC1: Two HTTP tracker blocks with the same `0.0.0.0:0` binding each start with their own configuration.
- [ ] AC2: Two UDP tracker blocks with the same `0.0.0.0:0` binding each start with their own configuration.
- [ ] AC3: Bootstrap does not use configured `SocketAddr` as a unique instance identity.
- [ ] AC4: HTTP and UDP startup logs include the configuration `instance_index`, allowing logs
      with duplicate configured addresses to be correlated with their source configuration block.
- [ ] AC5: Focused HTTP, UDP, and application bootstrap tests pass.
- [ ] AC6: `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- Focused regression tests for `AppContainer` and startup jobs after prerequisites land.
- `tests/servers/api/contract/stats/mod.rs`: enabled/disabled HTTP listeners on distinct fixed
  ports must produce aggregate `tcp4_announces_handled == 1`.
- Defer the equivalent UDP fixed-port and repeated-port-zero HTTP/UDP aggregate-statistics tests
  to #2039; they assert listener-side metrics filtering rather than bootstrap configuration selection.
- `cargo test --test stats --test scaffold`.
- `linter all`.

### Manual Evidence Protocol

Before and after the bootstrap implementation, run the relevant scenarios
against a locally launched tracker and append exact configuration, commands,
final listener addresses, REST statistics, and observed result to
[evidence.md](evidence.md). Do not replace existing baseline evidence.

### Manual Verification Scenarios

| ID  | Scenario                                                                            | Expected Result                                                          | Status | Evidence                   |
| --- | ----------------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ------ | -------------------------- |
| M1  | Start two HTTP trackers with identical `0.0.0.0:0` bindings and different settings. | Each listener retains the settings from its own configuration block.     | TODO   | [evidence.md](evidence.md) |
| M2  | Repeat M1 for UDP trackers.                                                         | Each UDP listener retains the settings from its own configuration block. | TODO   |                            |
| M3  | Run fixed-port enabled/disabled HTTP listeners locally.                             | The aggregate HTTP announce count is `1`.                                | TODO   | [evidence.md](evidence.md) |

## References

- Issue #1419: [main-application integration tests](../../open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md)
- [Runtime registry investigation](../../open/1419-allow-multiple-integration-tests-at-main-app-level/investigation-registar-and-health-check.md)
- Feature #2036: [add runtime service registry metadata](../2036-add-runtime-service-registry-metadata/ISSUE.md)
- Bug #2039: [normalize per-instance event metrics policy](../2039-normalize-per-instance-event-metrics-policy/ISSUE.md)
- Issue #2041: [migrate runtime service registry metadata](../2041-migrate-runtime-service-registry-metadata/ISSUE.md)
- [Archived implementation attempt](archived-attempt.md)
- [Events architecture](../../../events-architecture.md)
