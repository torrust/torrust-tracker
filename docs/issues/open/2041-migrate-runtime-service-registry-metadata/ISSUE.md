---
doc-type: issue
issue-type: enhancement
status: open
priority: p1
github-issue: 2041
spec-path: docs/issues/open/2041-migrate-runtime-service-registry-metadata/ISSUE.md
branch: "2041-migrate-runtime-service-registry-metadata"
related-pr: null
last-updated-utc: 2026-07-30 00:00
semantic-links:
  skill-links:
    - create-issue
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md
    - docs/issues/open/2035-fix-duplicate-port-zero-tracker-instance-bootstrap/ISSUE.md
    - docs/issues/open/2036-add-runtime-service-registry-metadata/ISSUE.md
    - docs/issues/open/2039-normalize-per-instance-event-metrics-policy/ISSUE.md
    - docs/issues/open/1419-allow-multiple-integration-tests-at-main-app-level/ISSUE.md
    - src/container.rs
  related-issues:
    - 1419
    - 2035
    - 2036
    - 2039
---

<!-- skill-link: create-issue -->

# Issue #2041 - Migrate Runtime Service Registry Metadata

## Goal

Migrate `Registar` registrations to carry canonical tracker service role,
configuration-instance identity, and final listener binding metadata. Make this
metadata queryable without running a health check or depending on bind-IP
conventions.

## Background

Issue #2036 originally combined two independent deliveries:

1. defining tracker-owned canonical service role and configuration-instance
   identity types; and
2. changing the standalone `torrust-server-lib` registration API, releasing it,
   and migrating the tracker to that API.

The second delivery cannot be completed until #2035 propagates canonical
identity through actual bootstrap. Splitting it into this issue gives both
branches a complete, independently testable scope:

```text
#2036 canonical identity types
        ↓
#2035 bootstrap propagation
        ↓
this issue: registry metadata migration
```

The registry migration is required before #2039 can use canonical runtime
identity for event-metrics policy filtering. It also replaces #1419's temporary
bind-IP classification and fixed registration delay.

## Scope

### In Scope

- Extend `torrust-server-lib::ServiceRegistration` with immutable generic
  metadata and make health-check behavior optional.
- Publish a compatible `torrust-server-lib` release and upgrade the tracker
  dependency.
- Carry tracker-owned role and #2036 canonical configuration-instance identity
  into each HTTP, HTTPS, UDP, REST API, and health-check registration.
- Provide side-effect-free, deterministic registry query APIs without exposing
  `HashMap` iteration as a contract.
- Establish registration visibility as an application-readiness boundary.
- Build health-check reports from metadata plus health-check execution results,
  preserving the existing JSON contract.
- Replace #1419 test helpers' bind-IP classification and fixed startup delay
  with role/identity-based registry discovery.
- Add progressive automatic and manual verification evidence for each
  code-changing task.

### Out of Scope

- Defining the canonical tracker identity types; owned by #2036.
- Preserving bootstrap identity for duplicate port-zero listeners; owned by
  #2035.
- Event-metrics listener filtering; owned by #2039.
- Dynamic restart, deregistration, or configuration reload.
- Public URLs, proxy/DNS topology, or a public registry API.

## Prerequisites

- #2036: canonical tracker service role and configuration-instance identity
  types are merged.
- #2035 bootstrap phase: every configured HTTP/UDP listener preserves and
  propagates its canonical configuration-instance identity during startup.

Both prerequisites are merged. #2036 provides tracker-owned `ServiceRole` and
`ConfigurationInstanceId` types. #2035 retains HTTP and UDP startup
containers as ordered `(ConfigurationInstanceId, Container)` pairs. This
issue must propagate the retained identifier rather than reconstructing one
from a bootstrap index.

## Approved Design

### Server Library Release

This issue releases `torrust-server-lib` **0.2.0**. The current `0.1.0` API
publicly exposes `Arc<Mutex<HashMap<ServiceBinding, ServiceRegistration>>>`
and its unspecified iteration order. Replacing that raw storage API with
snapshots and queries is breaking, so a `0.1.x` release would not follow
pre-1.0 semantic versioning. All tracker dependency declarations and
`Cargo.lock` must explicitly upgrade to `0.2.0`; a `"0.1.0"` Cargo
requirement does not accept `0.2.0`.

The standalone library change is deliberately small and application-agnostic:

1. Make `ServiceRegistration` generic over immutable metadata. It stores the
   final `ServiceBinding`, opaque application-owned metadata, and optional
   health-check behavior.
2. Make `Registar` and its registration form generic over the same metadata.
   Registration returns an acknowledgement only after insertion makes the
   registration visible to registry snapshots.
3. Keep registry storage private. Remove the public raw registry alias and
   `entries()` API rather than exposing a mutex or `HashMap` iteration as a
   contract.
4. Provide cloned, side-effect-free registration snapshots and metadata-based
   query support. Returned snapshots have a documented deterministic order by
   final `ServiceBinding`; neither hash-map nor task/insertion order is part
   of the API contract.
5. Expose optional health-check execution separately from metadata discovery.
   A registration without health behavior remains queryable and produces no
   health-check task.

Registrations are immutable records for the process lifetime in this delivery.
Dynamic restart, deregistration, replacement, liveness removal, and
re-registration are intentionally out of scope. The registry rejects duplicate
final bindings so a snapshot never represents two services at one listener.

The tracker owns a typed runtime metadata value containing the canonical
`ConfigurationInstanceId`; its `ServiceRole` is derived from that identity, so
the metadata cannot represent inconsistent role and identity values.
`torrust-server-lib` must not define tracker roles, configuration identifiers,
metrics policy, or tracker-specific metadata keys.

### Registration and Readiness

A local service is registry-ready only after it has successfully bound its
listener **and** received the registration-insertion acknowledgement. This is
a per-service boundary, not a new global application lifecycle coordinator.
`AppContainer` and `JobManager` retain their current composition and lifecycle
responsibilities.

Consumers needing application readiness must wait for the exact configured
canonical identities in registry snapshots, rather than a registry-size
threshold, a startup delay, a log line, or a health check. This accommodates
applications that omit optional services and repeated `0.0.0.0:0`
configuration blocks.

### Tracker Migration

- HTTP and HTTPS registrations use `ServiceRole::HttpTracker`; their final
  `ServiceBinding` distinguishes HTTP from HTTPS.
- UDP registrations use `ServiceRole::UdpTracker`.
- The REST API registers `ServiceRole::RestApi` with
  `ConfigurationInstanceId::new(ServiceRole::RestApi, 0)`.
- The health-check API registers `ServiceRole::HealthCheckApi` with
  `ConfigurationInstanceId::new(ServiceRole::HealthCheckApi, 0)` and has no
  health-check behavior, preventing recursive self-checking.

The health-check handler must read stable binding and role fields from the
metadata snapshot, then combine them with optional health-check execution
results. Its JSON contract remains compatible: `service_binding`, `binding`,
and `service_type` retain their established values. The existing HTTP/HTTPS
health-check URL behavior is outside this issue and must not change
incidentally.

### Related-Issue Compatibility

- **#2035:** use the configuration identifier retained with each container;
  never infer service identity from an address or re-create it from a loop
  index.
- **#2036:** use its canonical types directly; do not introduce strings or a
  second tracker identity model as the source of truth.
- **#2039:** registry metadata is immutable runtime discovery data only.
  Event producers must still carry canonical identity directly, and this issue
  does not implement event or metrics-policy behavior.
- **#1419:** replace raw-registry polling, bind-IP classification, and fixed
  registration delays with exact role/identity snapshot discovery.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                                          | Notes / Expected Output                                                                                          |
| --- | ----------- | --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Confirm prerequisites                         | #2036 and #2035 are merged; #2035's retained canonical identifiers must flow through registration.               |
| T2  | DONE        | Define generic registration metadata boundary | Approved: generic immutable metadata, optional health behavior, and tracker-owned typed role/identity metadata.  |
| T3  | DONE        | Extend registration and query API             | `torrust-server-lib` 0.2.0 provides metadata, optional checks, insertion acknowledgement, and ordered snapshots. |
| T4  | DONE        | Establish readiness semantics                 | Approved per-service insertion acknowledgement after bind; readiness consumers query exact expected identities.  |
| T5  | DONE        | Release and upgrade server library            | Published `torrust-server-lib` 0.2.0; all tracker declarations and lockfile resolve the release.                 |
| T6  | DONE        | Migrate tracker registrations                 | HTTP(S), UDP, REST API, and health-check API register canonical role and instance metadata.                      |
| T7  | DONE        | Migrate health reporting                      | Health reports combine metadata binding/role with optional check execution and preserve JSON fields.             |
| T8  | DONE        | Migrate #1419 discovery helpers               | Helpers await exact identities and query canonical roles; no bind-IP or map-order classification remains.        |
| T9  | DONE        | Add focused tests                             | Added health JSON compatibility and repeated port-zero identity-to-binding regressions.                          |
| T10 | IN_PROGRESS | Validate and record evidence                  | Focused compilation, tests, and linters passed; final full quality gate and manual scenarios remain.             |

## Progressive Verification Protocol

For every code-changing task (T2-T9):

1. Choose the smallest observable behavior affected by the task.
2. Record a baseline configuration, command, endpoint/query, and output in an
   issue-local `evidence.md` before changing code.
3. Implement the smallest change and run focused tests.
4. Repeat the same manual probe and record the post-change result.
5. Stop on unexpected changes; document expected deltas and add regressions.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created: #2041
- [ ] Spec-only PR merged into `develop` before implementation
- [x] Prerequisites merged
- [x] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests in both repositories)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-07-29 14:45 UTC - agent - Drafted by splitting the registry migration from #2036, which now owns canonical identity types only. Awaiting user review.
- 2026-07-29 15:10 UTC - agent - User approved the specification; created GitHub issue #2041 and moved this specification to `docs/issues/open/`.
- 2026-07-30 00:00 UTC - user and agent - Confirmed that the standalone server-library release, publication, and tracker upgrade are in scope. Approved generic immutable metadata, per-service insertion-acknowledgement readiness, and a concrete `0.2.0` server-library API plan. Reviewed compatibility with #2035, #2036, #2039, and #1419.
- 2026-07-31 UTC - agent - Published `torrust-server-lib` 0.2.0 after a successful `cargo publish --dry-run`; pushed signed release commit `d17fdb1`.
- 2026-07-31 UTC - agent - Migrated tracker registrations and health reporting to typed runtime metadata. Replaced #1419 bind-IP/count-based helper behavior with exact canonical identity readiness and role queries. Focused tests, workspace compilation, and `linter all` passed; final validation and manual evidence remain pending.

## Acceptance Criteria

- [ ] AC1: Registrations expose final binding and opaque metadata without
      running network health checks.
- [ ] AC2: Tracker registrations carry canonical role and configuration-instance
      identity for each started local service.
- [ ] AC3: Registry queries are deterministic and do not expose map ordering.
- [ ] AC4: Registration visibility provides a testable application-readiness
      boundary.
- [ ] AC5: Health-check JSON preserves `service_binding`, `binding`, and
      `service_type` compatibility.
- [ ] AC6: #1419 helpers discover endpoints by role/identity without a fixed
      startup delay or bind-IP convention.
- [ ] AC7: Port-zero and repeated configuration blocks retain correct identity.
- [ ] AC8: Both repository validation suites pass.
- [ ] AC9: Manual verification evidence is recorded for every code-changing
      task.

## Verification Plan

### Automatic Checks

- `torrust-server-lib` unit tests for metadata, query, and readiness behavior.
- Tracker registry/health-check tests.
- `cargo test --test stats --test scaffold` after #1419 helper migration.
- `linter all` in both repositories.

### Manual Verification Scenarios

| ID  | Scenario                                                                  | Expected Result                                                                    | Status | Evidence |
| --- | ------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Start HTTP, HTTPS, REST API, health API, and UDP services with port zero. | Registry queries distinguish canonical role, instance identity, and final binding. | TODO   |          |
| M2  | Start repeated HTTP and UDP `0.0.0.0:0` configuration blocks.             | Each final listener is correlated with the intended configuration instance.        | TODO   |          |
| M3  | Run health checks after registry migration.                               | Health response preserves existing JSON fields and values.                         | TODO   |          |

## References

- #2036: canonical identity type foundation
- #2035: bootstrap identity propagation prerequisite
- #2039: event-metrics normalization consumer
- #1419: main application test helper migration
- `docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md`
- `docs/issues/open/2036-add-runtime-service-registry-metadata/ISSUE.md`
