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

### Deferred Announce Response Decoration

`AnnounceHandler` currently needs persistent completed metrics before it can
populate `AnnounceData.stats` for a first announcement of a torrent. That
requirement makes a public handler and a persistent-statistics handler state a
proportionate #2107 solution: protocol consumers retain one
`Arc<AnnounceHandler>` API, while the container explicitly selects its state.
Keeping the selected persistent-statistics state inside that one handler avoids
duplicating the announce workflow merely to vary first-announce metric loading.

A later architectural refactor may split `AnnounceHandler` into separate public
and persistent-statistics types, or separate peer/swarm coordination from
response decoration. Under the latter model, tracker core would return a
peer-list result, and an upper layer would add metrics and policy fields to the
protocol response. This could remove persistent metrics from the announce
handler, but it changes a hot request path and the domain/protocol boundary.

The response-decoration alternative is postponed because it must first define
how peer updates and the enriched statistics share a consistent snapshot. It
also requires a compatibility review of the HTTP and UDP mappings of
`AnnounceData`, protocol-contract tests, and before/after announce-path
benchmarks to establish that any extra data access or handoff does not degrade
request latency. It requires a dedicated design issue before implementation
and is out of scope for #2107.

### Private-Key and Whitelist Composition

Private-key and whitelist behavior remains configuration-selected: `private`
and `listed` decide whether startup loads the corresponding data and whether
the REST routes receive their concrete adapters. The P5/P7 refactor must not
use persistence presence as the feature switch, nor make the REST API depend
on persistence when both features are disabled.

For the current bootstrap API, a configured feature with no persistence service
will omit only that feature's load or adapter instead of panicking. Bootstrap
validation already rejects that invalid configuration before composition. A
later typed startup-error refactor should report this impossible state directly
rather than relying on the validation order; that broader propagation work
remains deferred by `bootstrap-error-propagation-draft.md`.

### Torrent Restoration Operation

`TorrentsManager::load_torrents_from_database` is a persistence-only operation,
but no production startup path currently invokes it. P6 therefore must not add
a startup operation merely to relocate an optional dependency. The manager will
retain only the dependencies needed for its always-available cleanup behavior,
and the restoration operation will receive its required completed-downloads
repository directly from any future persistence-enabled caller.

## Inventory

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `NOT_APPLICABLE`.

| ID  | Status         | Location                                                           | Current pattern                                                                                                                             | Planned disposition                                                                                                                                                    |
| --- | -------------- | ------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| P1  | DONE           | `packages/tracker-core/src/announce_handler.rs`                    | Persistent completed-statistics configuration is paired with `Option<Arc<DatabaseDownloadsMetricRepository>>`; database load uses `expect`. | Composes public and persistent-statistics handler states through explicit constructors; the latter receives a required repository.                                     |
| P2  | DONE           | `packages/tracker-core/src/statistics/event/{listener,handler}.rs` | One listener handles both in-memory updates and optional database writes.                                                                   | Split into in-memory and persistence listeners with concrete dependencies.                                                                                             |
| P3  | DONE           | `src/bootstrap/jobs/tracker_core.rs`                               | One job starts when either configuration switch is enabled and passes a boolean plus optional repository.                                   | Explicitly start the mandatory in-memory listener and optional persistence listener from configuration.                                                                |
| P4  | DONE           | `src/bootstrap/persistence.rs`                                     | Persistent completed statistics requires a database but not enabled tracker usage statistics.                                               | Reject persistent completed statistics unless both prerequisites are enabled.                                                                                          |
| P5  | DONE           | `src/app.rs`                                                       | Private, listed, and persistent completed-statistics startup loading uses `expect` after configuration conditions.                          | Keep configuration as the feature gate; invoke loaders only with a concrete service. Bootstrap validation rejects invalid configurations before startup.               |
| P6  | DONE           | `packages/tracker-core/src/torrent/manager.rs`                     | Optional repository is unwrapped by `load_torrents_from_database`.                                                                          | Removed the unused optional manager dependency; the persistence-only restoration operation requires a concrete repository.                                             |
| P7  | DONE           | `packages/axum-rest-api-server/src/v1/routes.rs`                   | Private/listed route branches use `expect` after configuration guards before constructing adapters.                                         | Keep configuration as the feature gate; construct adapters only with a concrete service. Bootstrap validation rejects invalid configurations before route composition. |
| P8  | NOT_APPLICABLE | Test fixtures changed on this branch                               | Tests use `expect` to assert persistence is present before exercising private/listed behavior.                                              | Retain as explicit test preconditions unless a production refactor changes fixture construction.                                                                       |

## Implementation Steps

- [x] Create this issue-local design and progress tracker.
- [x] Identify the expectation-based persistence invariants introduced by the current branch and classify test-only assertions separately.
- [x] Maintainer reviewed the proposed scope and inventory.
- [x] Select configuration-driven branching with concrete feature dependencies; reject the capability-enum abstraction.
- [x] Add and test the persistent-completed-statistics prerequisite on tracker usage statistics (P4).
- [x] Refactor persistent completed-statistics announce-time loading (P1): retain the existing `Arc<AnnounceHandler>` consumer API while `TrackerCoreContainer` constructs explicit public or persistent-statistics handler state with concrete dependencies.
- [x] Split persistent completed statistics from in-memory statistics event handling (P2-P3) with focused tests.
- [x] Refactor private-key and listed-whitelist startup and route composition (P5 and P7) with focused tests.
- [x] Refactor the persistence-only torrent restoration operation (P6) with focused tests.
- [x] Run focused tests, formatting, and applicable quality checks.
- [x] Update this tracker with outcomes, evidence, and remaining follow-up work.

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
- 2026-08-28 - Refined P1 after tracing HTTP and UDP consumers. They depend on
  the stable `Arc<AnnounceHandler>` API, so the container will select explicit
  public and persistent-statistics handler states internally. This is a
  feature-owned composition choice, not the rejected generic capability type.
- 2026-08-28 - Completed P2-P4 in `e10d894b`: separated in-memory and
  persistent-completed-statistics listeners, composed their jobs explicitly,
  and rejected persistent statistics when tracker usage statistics is disabled.
- 2026-08-28 - Completed P1. `AnnounceHandler` no longer combines the feature
  configuration with an optional database repository or asserts its presence
  on an announce path. A focused container test proves that the
  persistent-statistics handler restores a stored completed count when the
  torrent is first announced. The handler module, tracker-core integration
  suite, formatting, and strict tracker-core Clippy checks passed.
- 2026-08-28 - Completed P5/P7. Startup loading and private-key/whitelist
  REST composition retain configuration as their feature gate and operate only
  when the concrete persistence services exist, removing production
  assertion panics. A focused application test covers persistence-free loader
  behavior, and focused REST contracts preserve the disabled-feature 409
  responses. Typed bootstrap-error propagation remains deferred by
  `bootstrap-error-propagation-draft.md`.
- 2026-08-28 - Completed P6. No production startup path invokes torrent
  restoration, so the refactor did not add one. `TorrentsManager` now owns only
  cleanup dependencies; its restoration operation receives the concrete
  completed-downloads repository from the persistence-enabled test caller.
  Focused manager and tracker-core integration tests passed.
