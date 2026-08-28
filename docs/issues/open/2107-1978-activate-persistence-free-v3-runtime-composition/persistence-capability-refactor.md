---
doc-type: implementation-tracker
issue: 2107
status: in-progress
last-updated-utc: 2026-08-28
---

# Persistence Capability Refactor

## Purpose

Replace runtime APIs that combine a configuration-gated action with an
optional service. The configuration must explicitly select the action at the
composition boundary, and the selected branch must pass concrete dependencies
to its consumers. Downstream services must not panic because an optional
service is absent.

## Proposed Design

Branch explicitly on configuration where the application composes or starts a
feature. In an enabled branch, obtain the feature's concrete service from
`PersistenceServices` and pass it to operations that require it. In the
disabled branch, do not construct or invoke that feature's persistence work.

`Option<PersistenceServices>` remains the root representation of an optional
application capability. It must be resolved at a composition boundary; it must
not propagate as `Option<Arc<...>>` into a service that cannot work without the
dependency.

An unexpected absent service in an enabled branch is a composition failure.
The desired outcome is a typed error returned from that boundary and bubbled to
bootstrap. Full startup error propagation is deferred by
`bootstrap-error-propagation-draft.md`; until it is implemented, the current
bootstrap validation remains the normal operator-facing diagnostic. The
refactor must still remove assertion panics from leaf services.

For tracker-core completed statistics, `core.tracker_usage_statistics` is the
master switch. When it is disabled, no tracker-core statistics listener starts.
When it is enabled, an in-memory statistics listener starts. When
`core.tracker_policy.persistent_torrent_completed_stat` is also enabled, a
second listener starts with a concrete
`Arc<DatabaseDownloadsMetricRepository>` to persist completed statistics.
Persistent completed statistics therefore requires both a database and enabled
tracker usage statistics.

This is not a repository-wide replacement for every `Option<Arc<...>>` or
every `expect`:

- `Option<PersistenceServices>` continues to describe whether the application
  has any persistence services.
- Persistence-only operations should instead receive a required repository or
  live behind the persistence-services composition boundary.
- Test-only `expect` calls may state fixture preconditions and are excluded
  unless they hide a production composition defect.

### Rejected Alternatives

| Alternative                                               | Reason discarded                                                                                                                                                                         |
| --------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Generic `PersistenceCapability<T>` enum plus type aliases | It does not remove the configuration boolean by itself and introduces a shared abstraction with no behavior. The actual problem is the missing explicit composition branch.              |
| One feature-specific enum per service                     | It has the same limitation as the generic enum while adding three types that duplicate `Option` state.                                                                                   |
| Treat service presence as the action switch               | Presence is an implementation detail. Configuration must explicitly determine whether a feature runs.                                                                                    |
| Continue passing optional services into leaf handlers     | It makes invalid states operational and requires each consumer to handle or assert the same composition invariant.                                                                       |
| Retain `expect` in handlers and repositories              | It converts a startup/composition fault into a late runtime panic on an event or request path.                                                                                           |
| Implement full bootstrap error propagation now            | #2107 explicitly defers that cross-cutting error-flow refactor. This work introduces typed lower-layer errors where practical and leaves bootstrap propagation to the tracked follow-up. |

## Inventory

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `NOT_APPLICABLE`.

| ID  | Status         | Location                                                           | Current pattern                                                                                                                             | Planned disposition                                                                                                  |
| --- | -------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| P1  | TODO           | `packages/tracker-core/src/announce_handler.rs`                    | Persistent completed-statistics configuration is paired with `Option<Arc<DatabaseDownloadsMetricRepository>>`; database load uses `expect`. | Construct this handler only in the persistence-enabled statistics branch, with a concrete repository.                |
| P2  | TODO           | `packages/tracker-core/src/statistics/event/{listener,handler}.rs` | One listener handles both in-memory updates and optional database writes.                                                                   | Split into in-memory and persistence listeners with concrete dependencies.                                           |
| P3  | TODO           | `src/bootstrap/jobs/tracker_core.rs`                               | One job starts when either configuration switch is enabled and passes a boolean plus optional repository.                                   | Explicitly start the mandatory in-memory listener and optional persistence listener from configuration.              |
| P4  | TODO           | `src/bootstrap/persistence.rs`                                     | Persistent completed statistics requires a database but not enabled tracker usage statistics.                                               | Reject persistent completed statistics unless both prerequisites are enabled.                                        |
| P5  | TODO           | `src/app.rs`                                                       | Private, listed, and persistent completed-statistics startup loading uses `expect` after configuration conditions.                          | Resolve concrete services in explicit configuration branches and return typed composition errors.                    |
| P6  | TODO           | `packages/tracker-core/src/torrent/manager.rs`                     | Optional repository is unwrapped by `load_torrents_from_database`.                                                                          | Receive a required repository for this persistence-only operation or move the operation behind persistence services. |
| P7  | TODO           | `packages/axum-rest-api-server/src/v1/routes.rs`                   | Private/listed route branches use `expect` after configuration guards before constructing adapters.                                         | Resolve concrete services in explicit configuration branches and return typed composition errors.                    |
| P8  | NOT_APPLICABLE | Test fixtures changed on this branch                               | Tests use `expect` to assert persistence is present before exercising private/listed behavior.                                              | Retain as explicit test preconditions unless a production refactor changes fixture construction.                     |

## Implementation Steps

- [x] Create this issue-local design and progress tracker.
- [x] Identify the expectation-based persistence invariants introduced by the current branch and classify test-only assertions separately.
- [x] Maintainer reviewed the proposed scope and inventory.
- [x] Select configuration-driven branching with concrete feature dependencies; reject the capability-enum abstraction.
- [ ] Add and test the persistent-completed-statistics prerequisite on tracker usage statistics (P4).
- [ ] Refactor persistent completed-statistics announce-time loading (P1) with focused tests.
- [ ] Split persistent completed statistics from in-memory statistics event handling (P2-P3) with focused tests.
- [ ] Refactor private-key and listed-whitelist startup and route composition (P5 and P7) with focused tests.
- [ ] Refactor the persistence-only torrent startup operation (P6) with focused tests.
- [ ] Run focused tests, formatting, and applicable quality checks.
- [ ] Update this tracker with outcomes, evidence, and remaining follow-up work.

## Progress Log

- 2026-08-28 - Created after T3 to track removal of internal runtime
  `expect` invariants introduced by optional persistence composition. The first
  proposed slice is persistent completed statistics (P1-P2); persistence-only
  torrent startup and REST route adapters remain separate decisions.
- 2026-08-28 - Expanded the refactor to include the equivalent private-key and
  listed-whitelist invariants after auditing all `expect` calls introduced on
  this branch. Test fixture precondition assertions remain out of scope.
- 2026-08-28 - Rejected generic and feature-specific capability enums. The
  approved approach uses configuration-selected composition branches with
  concrete dependencies, typed composition errors, and no leaf-level
  assertion panics. For statistics, usage statistics is the master switch and
  persistent completed statistics is an optional second listener that requires
  both enabled usage statistics and persistence.
- 2026-08-28 - The maintainer required this specification to be committed
  before implementation begins. P1-P7 remain planned until source changes are
  reviewed, validated, and committed separately.
