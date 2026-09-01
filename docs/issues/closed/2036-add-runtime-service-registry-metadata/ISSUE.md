---
doc-type: issue
issue-type: enhancement
status: done
priority: p1
github-issue: 2036
spec-path: docs/issues/closed/2036-add-runtime-service-registry-metadata/ISSUE.md
branch: 2036-add-runtime-service-registry-metadata
related-pr: null
last-updated-utc: 2026-08-17
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/closed/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
    - docs/issues/closed/2041-migrate-runtime-service-registry-metadata/ISSUE.md
    - packages/axum-http-server/src/server.rs
    - packages/axum-rest-api-server/src/server.rs
    - packages/primitives/src/configuration_instance_id.rs
    - packages/primitives/src/service_role.rs
    - packages/udp-server/src/server/launcher.rs
  related-issues:
    - 1419
---

# Issue #2036 - Define Canonical Runtime Service Identity

## Goal

Define tracker-owned canonical service-role and configuration-instance identity
types that can be used consistently by bootstrap, runtime registration, and
event-metrics consumers.

## Background

The earlier #2036 plan combined two deliveries: defining the canonical identity
model, then migrating `torrust-server-lib::Registar` and tracker registrations
to carry that model. The registry migration cannot be completed until #2035
preserves identity through real bootstrap. Keeping both deliveries in one issue
would leave its main work blocked after a small independently mergeable type
foundation.

This issue now owns the type foundation only. The registry migration is planned
in [#2041](../2041-migrate-runtime-service-registry-metadata/ISSUE.md), which depends on this issue and #2035 bootstrap propagation.

## Scope

### In Scope

- Define tracker-owned canonical service-role values without coupling the generic library to them.
- Define a canonical configuration-instance identity type with clear scope and
  equality semantics.
- Document ownership boundaries and ensure the types can be consumed by #2035,
  registry migration, and #2039 without a competing identity model.
- Add focused unit tests and public API documentation for the new types.

### Out of Scope

- Fixing duplicate port-zero bootstrap storage; owned by #2035.
- Extending `torrust-server-lib` registration records or query APIs; owned by
  #2041.
- Releasing or upgrading `torrust-server-lib`; owned by #2041.
- Public URLs, proxies, domain names, and deployment topology.
- Dynamic service restart, deregistration, or configuration reload.

## Approved Design Decisions

- The tracker-owned `primitives` package is the canonical home for both
  identity types. They must not be added to the generic
  `torrust-net-primitives` or `torrust-server-lib` packages.
- `ServiceRole` has `HttpTracker`, `UdpTracker`, `RestApi`, and
  `HealthCheckApi` variants. HTTPS remains the `HttpTracker` role; its final
  `ServiceBinding` differentiates HTTP from HTTPS.
- `ConfigurationInstanceId` combines a `ServiceRole` with a zero-based index
  in that role's configuration-entry list. Its equality is structural over
  those two values and never considers a configured or final `SocketAddr`.
- The index is derived during configuration/bootstrap enumeration and remains
  immutable for the lifetime of the process. It correlates one configured
  instance; it is not a user-supplied persistent service identifier.
- The public types provide the traits needed by their intended internal
  consumers, including comparison, hashing, and serialization, without
  introducing a parallel identity representation.

## Implementation Plan

| ID  | Status | Task                                                  | Notes / Expected Output                                                                                                   |
| --- | ------ | ----------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Define tracker-owned service role type                | Keep tracker semantics out of generic network and server crates.                                                          |
| T2  | DONE   | Define canonical configuration-instance identity type | Specify scope, equality, construction, documentation, and unit tests.                                                     |
| T3  | DONE   | Verify consumer boundaries                            | Confirm #2035 bootstrap, registry migration, and #2039 can consume the same types without creating competing identifiers. |
| T4  | DONE   | Run focused validation                                | `cargo test -p torrust-tracker-primitives` and `linter all` passed.                                                       |

## Progress Tracking

### Workflow Checkpoints

- [x] Specification drafted and approved by user/maintainer
- [x] GitHub issue created: #2036
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests)
- [x] Acceptance criteria reviewed after implementation
- [x] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-07-28 14:51 UTC - agent - User-approved specification promoted to GitHub feature #2036.
- 2026-07-29 14:45 UTC - agent - Split registry migration into a dedicated draft issue. #2036 now owns only canonical role and configuration-instance identity types, which can be implemented before #2035 bootstrap propagation.
- 2026-07-29 16:15 UTC - user and agent - Confirmed the canonical identity model: tracker-owned
  primitives define the four service roles and a role-qualified, zero-based configuration instance
  index. The identity is independent of socket addresses and is not a user-supplied persistent ID.
- 2026-07-29 16:28 UTC - agent - Added `ServiceRole` and `ConfigurationInstanceId` to the
  tracker-owned primitives package. `cargo test -p torrust-tracker-primitives`, `linter all`, and
  the full pre-commit check passed.
- 2026-07-29 16:28 UTC - agent - Replaced the HTTP, REST API, and UDP health-check
  `TYPE_STRING` values with their corresponding `ServiceRole` identifiers. The REST API canonical
  string is `tracker_rest_api` to preserve its existing health-check response value.
- 2026-08-17 UTC - GitHub Copilot - Archived the specification after GitHub issue #2036 was closed and implementation PR #2042 merged.

## Acceptance Criteria

- [x] AC1: Tracker-owned canonical service-role values are defined without coupling generic server/network libraries to tracker variants.
- [x] AC2: Canonical configuration-instance identity is typed, documented, and independent of configured socket addresses.
- [x] AC3: The types can be used by #2035, the registry migration follow-up, and #2039 without conversion to competing identity types.
- [x] AC4: Focused tests and `linter all` exit with code `0`.

## Verification Plan

### Automatic Checks

- Focused unit tests for the role and identity types.
- Compile checks at intended consumer boundaries.
- `linter all`.

### Manual Verification Scenarios

| ID  | Scenario                                                                    | Expected Result                                                                   | Status | Evidence |
| --- | --------------------------------------------------------------------------- | --------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Construct identity values for repeated same-protocol configuration entries. | Equal configured addresses remain distinguishable by canonical instance identity. | TODO   |          |

## References

- [ADR 20260728115400](../../../adrs/20260728115400_define_registar_as_runtime_service_registry.md)
- Future consumer #2035: [fix duplicate port-zero tracker instance bootstrap](../2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md)
- Follow-up #2041: [migrate runtime service registry metadata](../2041-migrate-runtime-service-registry-metadata/ISSUE.md)
