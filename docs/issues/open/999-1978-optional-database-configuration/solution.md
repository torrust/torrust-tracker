---
semantic-links:
  related-artifacts:
    - docs/issues/open/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/open/999-1978-optional-database-configuration/analysis.md
    - packages/configuration/docs/migrate-v2-to-v3.md
    - docs/issues/open/999-1978-optional-database-configuration/adr-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-awareness-epic-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-free-runtime-activation-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-unavailable-scenarios.md
---

# Phase 2 - Optional persistence solution

## Status

Phase 1 evidence and the Phase 2 design are approved. The approved design is
ready for the analysis-and-solution PR. Phase 3 implementation remains a
separate delivery and must follow this approved contract.

## Approved decision

The approved design allows v3 `[core.database]` to be omitted when persistence
is unused, while rejecting startup if an enabled persistence-backed capability
requires a database. It preserves v2 behaviour unchanged.

## Approved design

The expected configuration representation is `Option<Database>` on v3 `Core`.
An omitted `[core.database]` table deserializes as `None`; configured drivers
retain the existing v3 driver-specific representation.

This issue prepares optional persistence at the configuration and
application-container boundaries. Phase 3 provisionally resolves
`Option<Database>` at the existing tracker-core initialization seam: its
`Some` branch initializes the selected driver and complete migration set, then
passes ordinary required stores to persistence-backed consumers. Its future
`None` branch must select a persistence-absent composition path before those
consumers are built. This prevents configuration optionality from cascading as
`Option` through consumers that are only valid in the persistence-enabled
composition.

While the crate-root runtime aliases remain v2, bootstrap deliberately passes
`Some(Database)` to that optional container dependency. This preserves the
existing effective database dependency during the v3 activation transition. It
is a named, tested compatibility bridge—not the final persistence-free runtime
behavior.

### Test and activation sequencing

V3 is not yet the globally active runtime configuration: that migration remains
Issue #1980's responsibility. Issue #999 must not activate v3 merely to test
this contract.

Phase 3 must instead test the contract at two levels:

1. **Versioned configuration tests:** construct and deserialize
   `v3_0_0::Configuration` directly to prove that an omitted
   `[core.database]` becomes `None` and that configured SQLite, MySQL, and
   PostgreSQL variants retain their driver-specific behavior.
2. **Optional-container tests:** exercise the container constructors with an
   explicit persistence dependency and prove that the temporary bootstrap
   bridge passes `Some(Database)` while v2 remains active. These tests confirm
   the containers can receive `None`, but do not claim a persistence-free main
   runtime yet.

Issue #1980 activates v3 consumers using the temporary compatibility database
dependency. A small follow-up issue, drafted in
`persistence-free-runtime-activation-draft.md`, then replaces that explicit
`Some(Database)` with the actual v3 `core.database` value, runs the capability
requirement validation, and delivers the full persistence-free runtime
guarantee. The final M1–M6 end-to-end evidence belongs to that follow-up.

The intended activation-follow-up persistence-free deployment includes a public
UDP tracker and/or public HTTP tracker. Listing, private mode, persistent
completed statistics, and the management REST API remain disabled. The REST API
can join a later persistence-free deployment only after API #144 implements its
approved next-major contract. This deployment is the scope of the activation
follow-up, not the effective runtime result of #999.

This issue makes containers capable of representing absent persistence. The
activation follow-up owns the minimum configuration-aware REST API behavior
needed for persistence-free operation. The detailed drafts for the Phase 3 ADR,
the activation follow-up, and a future persistence-awareness EPIC are in this
issue folder.

## Required Solution Content

### Configuration contract

- Define v3 TOML semantics for an omitted `[core.database]` section.
- Specify whether empty or partial database sections are rejected and how their
  errors are reported.
- Define the v2-to-v3 migration guidance and confirm v2 remains unchanged.
- Define the temporary explicit database bridge used through v3 activation and
  the follow-up removal plan.

### Capability validation matrix

Issue #999 implements and unit-tests one reusable bootstrap-owned
application-composition check. It is the **only** owner of the
feature-to-persistence matrix; do not duplicate the rule in
`packages/configuration::Validator`.

The active bootstrap path does not invoke this check while it deliberately
passes the temporary `Some(Database)` bridge. The activation follow-up invokes
the already-implemented check after v3 configuration loading and before
`AppContainer` or `TrackerCoreContainer` construction, using the actual v3
`Option<Database>` value.

The configuration crate continues to validate field-local values and its own
cross-field consistency. The persistence requirement is application policy: it
depends on the services bootstrap constructs and may shrink as the follow-up
refactoring decouples further capabilities.

The initial matrix below is authoritative for Phase 3. If implementation finds
another capability that requires a persistence store, add it to this centralized
matrix, the reusable validation implementation, its focused tests, and the
activation-follow-up draft before merging. Do not add an ad hoc repository or
route-level missing-database check.

The reusable check returns `PersistenceRequirementError` with one stable variant
per approved capability:

| Variant                                          | Diagnostic                                                                                                                              |
| ------------------------------------------------ | --------------------------------------------------------------------------------------------------------------------------------------- |
| `ListedRequiresDatabase`                         | `Configuration requires persistence for \`core.listed\`, but \`[core.database]\` is missing.`                                           |
| `PrivateRequiresDatabase`                        | `Configuration requires persistence for \`core.private\`, but \`[core.database]\` is missing.`                                          |
| `PersistentTorrentCompletedStatRequiresDatabase` | `Configuration requires persistence for \`core.tracker_policy.persistent_torrent_completed_stat\`, but \`[core.database]\` is missing.` |

The error type belongs beside the reusable bootstrap requirement-check function,
not in `packages/configuration::Validator`. Phase 3 tests each variant and its
diagnostic independently.

| Capability                   | Enabled when                                                   | Final activation-follow-up result                                                                                     | Initial test expectation                                                                                    |
| ---------------------------- | -------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| Whitelist                    | `core.listed = true`                                           | Startup fails before container construction.                                                                          | Error names `core.listed` and missing `[core.database]`.                                                    |
| Private keys                 | `core.private = true`                                          | Startup fails before container construction.                                                                          | Error names `core.private` and missing `[core.database]`.                                                   |
| Persistent completed metrics | `core.tracker_policy.persistent_torrent_completed_stat = true` | Startup fails before container construction.                                                                          | Error names the setting and missing `[core.database]`.                                                      |
| Management REST API          | `http_api` is configured                                       | Remains persistence-required until the next-major API #144 work implements the approved disabled-capability contract. | Activation follow-up documents the temporary requirement; API #144 tests the persistence-free API contract. |
| Persistence-free tracker     | None of the persistence-backed conditions apply                | Startup succeeds without driver construction, migrations, database file, or network connection.                       | Activation follow-up proves no persistence artifacts.                                                       |

This becomes deterministic startup validation when the activation follow-up
calls the already-implemented check; it is not a late runtime failure. Database
reachability, filesystem permissions, credentials, and DDL permission remain
runtime/environment failures after configuration has passed validation.

### Runtime lifecycle

The lifecycle is all or nothing:

1. **Persistence absent and permitted:** construct no driver, database stores,
   database file, network connection, or migration.
2. **Persistence configured or required:** construct the selected driver once
   and apply the complete shared migration set once before persistence-backed
   services are constructed.

Feature configuration controls code behavior, not schema fragments. Do not add
feature-specific database schemas, migration streams, or migration selection.
Although the current persistence features are relatively independent, managing
conditional migrations would increase upgrade, compatibility, and test
complexity as future features share data or evolve.

### Persistence configuration transitions

Persistence configuration is evaluated only when the tracker process starts.
Changing configuration requires a restart; the tracker does not dynamically add
or remove persistence while running.

| Previous process    | Next process configuration                   | Required behavior                                                                               |
| ------------------- | -------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| Persistence-free    | Persistence-free                             | Start without persistence artifacts.                                                            |
| Persistence-free    | A persistence-required capability is enabled | Require a configured database, then initialize its complete shared schema.                      |
| Persistence-enabled | Persistence-free                             | Do not open, migrate, write, delete, or otherwise alter the previous database.                  |
| Persistence-enabled | Persistence-enabled, same target             | Reuse the selected database and apply the complete migrations; completed migrations are no-ops. |
| Persistence-enabled | Different driver or database location        | Initialize and migrate the newly selected target; do not automatically copy historical data.    |

Existing database state is operator-managed durable state. Disabling
persistence prevents the next process from using that state but never drops
tables or deletes files/records. Re-enabling persistence against the same target
reuses its state; data produced while persistence was disabled is intentionally
not recoverable. Enabling a different target is not an automatic data migration
between drivers or locations.

### Container entrypoint contract

The activation follow-up changes the supported container entrypoint as follows:

1. **No persistence:** do not require a database-driver override, create the
   tracker database directory solely for persistence, or install a packaged
   SQLite database or database-specific default configuration.
2. **Persistence configured:** retain driver-specific setup only when the
   actual v3 configuration selects persistence; do not force a driver through a
   default environment override.
3. **Mounted state:** never overwrite or delete mounted `/etc` configuration or
   `/var/lib` database files merely because the next configuration disables
   persistence.
4. **Target change:** never copy or delete a prior database automatically when
   the configured driver or location changes.

The entrypoint must defer persistence selection to the v3 configuration rather
than independently inventing a database default. User identity, non-database
configuration installation, and runtime directory permissions remain separate
entrypoint responsibilities.

Phase 3 defines the owner/timing of driver construction and the optional
repository/service constructors. The activation follow-up proves the
persistence-free branch after it replaces the temporary bridge. It also defines
container-entrypoint behavior for no-persistence deployments, including driver
overrides, database directories, and packaged SQLite installation.

### REST API contract

The approved desired REST behavior is an explicit configuration-disabled
response for a direct route whose capability is disabled. It uses HTTP `409
Conflict` and the existing `ActionStatus::Err` response shape, for example:

```json
{
  "status": "err",
  "reason": "Whitelist capability is disabled by configuration (`core.listed = false`)."
}
```

Protocol/application layers must represent this as a distinct
`DisabledByConfiguration` error; it must not reuse the existing database error
path. Existing generic 500 database failures remain reserved for a configured
database that fails operationally after startup.

This response-model change is deferred to the next-major REST API subissue
draft `docs/issues/drafts/144-make-rest-api-persistence-aware.md`, under
GitHub EPIC issue #144. Therefore the post-#1980 persistence-free activation
follow-up does not include the management REST API: it delivers a public UDP
and/or HTTP tracker with no persistence. Until that subissue implements the
approved REST contract, `http_api` remains a persistence-required capability in
the activation follow-up.

The same #144 work must make persistence-dependent historical values explicit
rather than silently presenting session values as lifetime values. Do not use a
negative numeric sentinel for unavailable history. This response-field work is
explicitly deferred to REST API v2 rather than being implemented by #999 or its
activation follow-up.

### Follow-up persistence-awareness EPIC

Create a detailed future EPIC before closing Phase 2. It must not block #1980
or require subissues to be created immediately. Its initial work inventory is:

- distinguish session counters from historical persisted counters in API models;
- expose metric provenance or historical-data availability without sentinels;
- decide session versus lifetime semantics for per-torrent completed counts;
- add persistence-free test helpers, integration tests, examples, and benchmarks;
- remove remaining implicit database assumptions from application composition,
  container artifacts, and deployment documentation.

The API response-model work is coordinated with GitHub issue #144, which owns
the next-major REST API compatibility changes.

`persistence-unavailable-scenarios.md` is the cross-layer case log for these
states. It distinguishes intentional absence, disabled capability, and
operational database failure, and assigns each case to its delivery issue.

### EPIC ordering and activation decision

Issue #999 is a prerequisite for Issue #1980 because it introduces the v3
optional representation and optional container dependencies. It does **not**
by itself deliver the persistence-free runtime guarantee. Issue #1980 activates
v3 with the named temporary database bridge, and the small activation follow-up
removes that bridge. The future persistence-awareness EPIC does not block either
issue.

EPIC #1978 and the v2-to-v3 migration guidance record the approved three-stage
ordering.

### Alternatives and trade-offs

Evaluate at least these alternatives against Phase 1 evidence:

- Keep the database mandatory in v3.
- Make database configuration optional but allow runtime failures for users of
  persistence-backed capabilities.
- Make database configuration optional and validate capability requirements at
  startup.

The working direction rejects the first two alternatives: the first abandons
the explicit in-memory deployment capability, and the second permits delayed
failures and hidden feature-to-database coupling.

#### Composition alternative A: resolve `Option<Database>` in tracker-core (selected)

`TrackerCoreContainer::initialize_from` receives `Option<Database>` and
matches it before constructing persistence-backed services. With `Some`, it
uses tracker-core's existing driver, migration, and store setup to construct a
persistence-enabled composition. With `None`, the future activation path can
construct a separate persistence-absent composition without creating a driver,
database file, network connection, or migration.

This is selected for Phase 3 because it is the least aggressive evolution of
the existing lifecycle. It localizes optionality at the current database
initialization seam: persistence-enabled consumers receive required store
dependencies, rather than each receiving and repeatedly handling an `Option`.
An `Arc` can share an initialized driver or store, but it does not remove the
need to choose a composition branch before constructing services whose
dependencies must exist. The current active v2 runtime keeps choosing `Some`
through the named compatibility bridge.

#### Composition alternative B: inject optional initialized persistence services

Bootstrap or application composition would initialize the driver, migrations,
and stores first, then pass `Option<PersistenceServices>` into tracker-core.
This can enforce that tracker-core never initiates infrastructure when no
dependency is supplied. It may also be appropriate if multiple top-level
containers need to share exactly one prebuilt persistence bundle.

It is not selected initially because it is more invasive and could make the
top-level composition own lifecycle details that currently belong to
tracker-core. The database setup implementation, including schema and
migration ownership, may remain in tracker-core even if a later refactor moves
the invocation boundary. Reconsider alternative B if alternative A requires
optional container fields, optionality in unrelated consumers, duplicate
initialization paths, or cannot represent the future persistence-absent branch
without weakening dependency invariants.

Phase 3 must preserve this reversibility: keep the optional boundary explicit,
avoid exposing the temporary v2 bridge as a generic default, and avoid coupling
the persistence-absent branch to the active runtime before the activation
follow-up.

## Approval Record

| Field         | Record                                                                                                                                                                                                                                                               |
| ------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Status        | Approved                                                                                                                                                                                                                                                             |
| Approver      | User/maintainer                                                                                                                                                                                                                                                      |
| Approved at   | 2026-08-25 UTC                                                                                                                                                                                                                                                       |
| Decision      | Implement v3 `Option<Database>`, optional container dependencies, the reusable bootstrap requirement matrix, and a temporary bridge in #999; activate v3 with the bridge in #1980; activate the actual persistence-free runtime in the refined post-#1980 follow-up. |
| Rationale     | This stages a non-breaking configuration representation and composition refactor before runtime activation, preserves the in-memory design goal, and avoids activating an untested `None` path prematurely.                                                          |
| Deferred work | Persistence-free REST API behavior and historical metric response semantics are next-major API work under EPIC #144.                                                                                                                                                 |
| ADR           | `adr-draft.md` is approved for Phase 3 reconciliation and timestamped publication in `docs/adrs/`.                                                                                                                                                                   |
