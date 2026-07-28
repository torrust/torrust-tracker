---
doc-type: issue
issue-type: enhancement
status: open
priority: p1
github-issue: 2036
spec-path: docs/issues/open/2036-add-runtime-service-registry-metadata/ISSUE.md
branch: 2036-add-runtime-service-registry-metadata
related-pr: null
last-updated-utc: 2026-07-28 12:30
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - docs/issues/open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
  related-issues:
    - 1419
---

# Issue #2036 - Add Runtime Service Registry Metadata

## Goal

Evolve `Registar` into a side-effect-free internal source of final local listener bindings, tracker
service roles, and configuration-instance correlation metadata.

## Background

`ServiceRegistration` in the standalone `torrust-server-lib` crate stores a final
`ServiceBinding` and a health-check function. The health-check function constructs the service role
only when it executes network I/O. Consumers cannot discover a running HTTP tracker versus REST
API through the registry without relying on bind-IP conventions, `HashMap` iteration order, logs,
or an unnecessary health check.

The bootstrap prerequisite must land first. A final listener binding identifies a running listener,
but repeated `0.0.0.0:0` configuration blocks require bootstrap to preserve configuration-instance
identity before that identity can be reported to the registry.

## Scope

### In Scope

- Extend the standalone `torrust-server-lib` registration record and read-only query API.
- Define tracker-owned canonical service-role values without coupling the generic library to them.
- Carry bootstrap-provided configuration-instance identity with each registration.
- Make registration visibility a deterministic application-readiness boundary.
- Build health-check reports from registration metadata and health-check execution results.
- Release the standalone crate and upgrade the tracker dependency.

### Out of Scope

- Fixing duplicate port-zero bootstrap storage; owned by the prerequisite issue.
- Public URLs, proxies, domain names, and deployment topology.
- Dynamic service restart, deregistration, or configuration reload.

## Implementation Plan

| ID  | Status  | Task                                                          | Notes / Expected Output                                                     |
| --- | ------- | ------------------------------------------------------------- | --------------------------------------------------------------------------- |
| T1  | BLOCKED | Merge bootstrap identity prerequisite                         | Registration needs a stable configuration-instance identity to carry.       |
| T2  | TODO    | Define tracker-owned service role and instance identity types | Keep tracker semantics out of generic network and server crates.            |
| T3  | TODO    | Extend `ServiceRegistration` in `torrust-server-lib`          | Store immutable metadata and make health-check behavior optional.           |
| T4  | TODO    | Add side-effect-free registry query API                       | Do not expose `HashMap` ordering as a contract.                             |
| T5  | TODO    | Establish registration readiness                              | Acknowledge insertion or provide an equivalent readiness boundary.          |
| T6  | TODO    | Release the standalone crate                                  | Publish a compatible version before tracker migration.                      |
| T7  | TODO    | Migrate tracker registrations and health reporting            | Preserve the health API JSON contract.                                      |
| T8  | TODO    | Replace #1419 bind-IP helper                                  | Query runtime metadata by role and instance identity without a fixed delay. |

## Progress Tracking

### Workflow Checkpoints

- [x] Specification drafted and approved by user/maintainer
- [x] GitHub issue created: #2036
- [ ] Prerequisite #2035 completed
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests in both repositories)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-07-28 14:51 UTC - agent - User-approved specification promoted to GitHub feature #2036;
  implementation remains blocked on the configuration-instance identity fix in #2035.

## Acceptance Criteria

- [ ] AC1: Internal consumers discover final local bindings without running health checks.
- [ ] AC2: Registrations expose tracker role and configuration-instance correlation metadata.
- [ ] AC3: The health-check response preserves `service_binding`, `binding`, and `service_type`.
- [ ] AC4: The generic server library remains independent of tracker-specific role variants.
- [ ] AC5: #1419 removes its bind-IP endpoint classification and fixed registration delay.
- [ ] AC6: Focused tests pass in both repositories and `linter all` exits with code `0`.

## Verification Plan

### Automatic Checks

- `torrust-server-lib` unit tests for registration metadata and query behavior.
- Health-check API contract tests.
- `cargo test --test stats --test scaffold` in the tracker repository.
- `linter all` in both repositories.

### Manual Verification Scenarios

| ID  | Scenario                                                                 | Expected Result                                                                 | Status | Evidence |
| --- | ------------------------------------------------------------------------ | ------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Start HTTP, HTTPS, REST API, and UDP services with port zero.            | Registry distinguishes local protocol, role, and final binding.                 | TODO   |          |
| M2  | Start repeated HTTP tracker configuration instances after bootstrap fix. | Registry correlates each final listener to the intended configuration instance. | TODO   |          |

## References

- [ADR 20260728115400](../../../adrs/20260728115400_define_registar_as_runtime_service_registry.md)
- Prerequisite #2035: [fix duplicate port-zero tracker instance bootstrap](../2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md)
- Issue #1419: [main-application integration tests](../../open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md)
- [Runtime registry investigation](../../open/1419-allow-multiple-integration-tests-at-main-app-level/investigation-registar-and-health-check.md)
