---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 1978
github-issue: 999
spec-path: docs/issues/open/999-1978-optional-database-configuration/ISSUE.md
branch: "999-avoid-unneeded-database-initialization"
related-pr: null
depends-on: 1490
blocks: null
last-updated-utc: 2026-08-25 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1978-configuration-overhaul-epic/EPIC.md
    - docs/issues/open/1490-1978-decompose-database-configuration.md
    - docs/issues/open/1978-configuration-overhaul-epic/configuration-v2-to-v3-migration.md
    - packages/configuration/src/v3_0_0/core.rs
    - packages/configuration/src/v3_0_0/database.rs
    - packages/configuration/src/validator.rs
    - packages/tracker-core/
    - src/container.rs
    - share/container/entry_script_sh
    - docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md
    - docs/issues/open/999-1978-optional-database-configuration/analysis.md
    - docs/issues/open/999-1978-optional-database-configuration/solution.md
    - docs/issues/open/999-1978-optional-database-configuration/baseline-e2e-verification.md
---

# Issue #999 - Make v3 database configuration optional when persistence is unused

> **EPIC position**: Configuration-overhaul subissue of EPIC #1978. It follows
> #1490, which defines the driver-specific v3 `Database` representation. Phase
> 1 and Phase 2 must determine whether this issue blocks #1980 and activation of
> the v3 configuration schema.

## Goal

Allow a tracker using configuration schema v3.0.0 to omit `[core.database]`
when no enabled capability requires persistence. In that mode, startup must not
construct a database driver, create database files, connect to network
databases, or run migrations.

The tracker must reject an invalid configuration at startup when an enabled
persistence-backed capability requires a database but `[core.database]` is
omitted. It must not silently disable that capability or fail later through an
unexpected database access.

## Background

Issue #999 was opened when every tracker startup created a SQLite database and
its tables, even for benchmarking configurations that did not use persistence.
The persistence implementation has since changed substantially: the tracker
uses migrations, supports SQLite, MySQL, and PostgreSQL, and the configuration
overhaul has introduced a driver-specific v3 `Database` representation in
issue #1490. The active runtime still uses v2 configuration until #1980.

The original report remains relevant because startup may still initialize the
database and execute migrations even when no feature consumes persistence.
However, its proposed implementation—moving table creation out of a driver
constructor—is not a design decision for the current architecture. The Phase 1
inventory must establish the actual construction and migration lifecycle before
Phase 2 selects a solution.

Known persistence-backed domains include whitelist entries, torrent completion
metrics, and private-tracker keys. Management REST API paths may expose or
mutate the same domains. Their configuration switches, direct dependencies, and
indirect assumptions that a database is always available are not yet fully
inventoried.

## Scope

### In Scope

- Define the v3-only contract for an optional `[core.database]` section.
- Investigate and document the current configuration, startup, migration, and
  persistence-consumer behaviour for every supported database driver, including
  container entrypoint side effects.
- Inventory direct and indirect dependencies on `tracker-core` persistence,
  including whitelist, torrent metrics, private-tracker keys, and management
  REST API operations.
- Decide and document startup validation for every enabled capability that
  requires persistence.
- Preserve the all-or-nothing schema lifecycle: once any enabled capability
  requires persistence, initialize the selected database and apply the complete
  shared migration set. Do not create feature-specific database schemas or
  feature-specific migration streams.
- Decide and document the management REST API contract when persistence is
  unavailable.
- Define the implementation, regression coverage, migration documentation, and
  operational verification required by the approved solution.
- Determine whether the change must precede #1980 and v3 activation; update
  EPIC #1978's ordering and activation criteria if it does.

### Out of Scope

- Changing v2.0.0 configuration types, defaults, validation, or database
  lifecycle. V2 operators continue supplying a database configuration, even
  when an unused SQLite database is created.
- Replacing or redesigning the v3 driver-specific database representation
  introduced by #1490.
- Choosing a persistence abstraction outside `packages/tracker-core` without
  evidence that the current package boundary cannot support the approved
  contract.
- Changing persistence-domain behaviour, schema contents, or migration history
  except where required to avoid initialization when persistence is unused.
- Silently disabling a configured persistence-backed capability.

## Architectural Decisions

### Decision 1: Restrict any configuration change to v3

If the approved solution changes the configuration contract, v3 alone makes
`[core.database]` optional. V2 remains unchanged for compatibility; users can
continue configuring an otherwise unused SQLite database.

### Decision 2: Separate evidence, solution, and implementation delivery

This issue has three phases. The first follow-up PR completes Phase 1 and Phase
2 together without changing runtime behaviour. A second follow-up PR implements
the approved Phase 3 plan. The current PR contains only this planning scaffold.

### Decision 3: Fail validation rather than degrade persistence silently

The final design must make an absent database configuration a startup
configuration error whenever an enabled capability needs persistence. The exact
capability inventory and validation location remain Phase 1 and Phase 2 work.

The expected mechanism is a configuration-consistency rule in
`packages/configuration/src/validator.rs`, invoked during bootstrap before
`AppContainer` construction. The existing precedent is
`UselessPrivateModeSection`: `[core.private_mode]` is rejected unless
`core.private = true`. This issue must use that layer only if the approved
database requirement is a relationship between configuration options; it must
not use it for field-local parsing or value invariants.

The implementation must keep feature-to-database requirements explicit at the
application boundary. It must not distribute optional-database checks through
repositories or feature implementation code, where a missed call site could
become a delayed runtime failure. Phase 2 will decide whether the bootstrap
rule is implemented through the configuration-consistency validator or a
bootstrap-owned validation step, based on the documented validation-layer
policy and the final configuration model.

- Related ADRs:
  `docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md`.
- ADRs to create: Decide during Phase 2. Create an ADR only if the selected
  optional-persistence lifecycle changes an enduring architecture boundary.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                           | Notes / Expected Output                                                                                                 |
| --- | ------ | ---------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Complete persistence analysis                  | Populate `analysis.md` with evidence for the current lifecycle, all consumers, and driver-specific migration behaviour. |
| T2  | TODO   | Approve an optional-persistence design         | Populate `solution.md` with the selected v3 contract, validation, API behaviour, compatibility, and ordering decision.  |
| T3  | TODO   | Implement optional v3 database configuration   | Apply only the Phase 2-approved design; do not change v2.                                                               |
| T4  | TODO   | Add regression coverage                        | Cover absent and present database configurations, required-feature validation, migrations, and REST API behaviour.      |
| T5  | TODO   | Update migration and operational documentation | Explain the v2-to-v3 difference and any changed deployment requirements.                                                |
| T6  | TODO   | Verify and re-review                           | Run required automatic and manual checks; update acceptance evidence.                                                   |

## Progress Tracking

### Workflow Checkpoints

- [x] GitHub issue #999 reviewed, including its original implementation comment
- [x] Spec-only branch created
- [x] Folder-based specification scaffold created
- [x] Spec reviewed and approved by user/maintainer
- [ ] Spec-only PR merged into `develop`
- [ ] Phase 1 and Phase 2 analysis-and-solution PR merged
- [ ] Phase 3 implementation PR merged
- [ ] Automatic verification completed
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Progress Log

- 2026-08-25 00:00 UTC - GitHub Copilot/User - Created the v3-only,
  folder-based planning scaffold. Confirmed that v2 remains unchanged and that
  the analysis-and-solution work precedes implementation.
- 2026-08-25 00:00 UTC - User - Approved the specification for the spec-only
  PR.

## Acceptance Criteria

- [ ] AC1: Phase 1 inventories the actual v2/v3 configuration, database-driver
      construction, and migration lifecycle for SQLite, MySQL, and PostgreSQL.
- [ ] AC2: Phase 1 inventories all direct and indirect persistence consumers,
      their enablement configuration, and their management REST API coupling.
- [ ] AC3: Phase 2 defines an approved v3-only configuration and startup
      validation contract for omitted `[core.database]`.
- [ ] AC4: The approved design prevents a database driver, database connection,
      database-file creation, and migration execution when persistence is not
      configured or required.
- [ ] AC5: The approved design rejects startup with a clear error when an
      enabled persistence-backed capability requires a missing database.
- [ ] AC6: The approved design defines deterministic REST API behaviour when
      persistence is unavailable.
- [ ] AC7: When persistence is required by at least one enabled capability, the
      implementation initializes the selected driver and applies the complete
      shared migration set; it does not create feature-specific schemas or run
      feature-specific migrations.
- [ ] AC8: Phase 2 determines and records whether this issue blocks #1980 and
      v3 activation; the EPIC ordering and migration guidance are updated if
      required.
- [ ] AC9: The implementation preserves v2 configuration and behaviour.
- [ ] AC10: The final v3 end-to-end scenario reproduces the original
      persistence-disabled benchmark use case without a database file,
      connection, or migration, with evidence recorded in
      `baseline-e2e-verification.md`.
- [ ] AC11: The supported container startup path permits a v3 deployment with
      no persistence, without requiring database-driver configuration or
      installing a default SQLite database solely for the tracker.
- [ ] AC12: `linter all` exits with code `0` after the implementation.
- [ ] AC13: Relevant automated tests and mandatory manual verification pass.
- [ ] AC14: Acceptance criteria are re-reviewed against implementation evidence.

## Verification Plan

Define the final commands and test ownership in Phase 2. The implementation
must at minimum provide the following checks.

### Automatic Checks

- `linter all`
- Focused configuration tests for v3 optional database parsing and validation.
- Focused `tracker-core` tests for database construction and migration gating.
- Focused REST API tests for every persistence-backed route affected by the
  approved contract.
- Relevant workspace tests and pre-push checks when applicable.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                     | Command/Steps                                                                                                                              | Expected Result                                                                                                                                       | Status | Evidence                                                                                         |
| --- | -------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------ |
| M1  | Start v3 without persistence                 | Start with no `[core.database]` and all persistence-backed capabilities disabled.                                                          | Startup succeeds without a database file, connection, or migration.                                                                                   | TODO   | Phase 2 defines exact configuration and evidence.                                                |
| M2  | Reject missing required persistence          | Enable each persistence-backed capability without `[core.database]`.                                                                       | Startup fails with a precise configuration error naming the unmet requirement.                                                                        | TODO   | Phase 2 defines the complete capability matrix.                                                  |
| M3  | Initialize configured driver                 | Start with each supported configured database driver and a required feature enabled.                                                       | Startup initializes the selected driver and applies migrations according to the approved lifecycle.                                                   | TODO   | Phase 2 defines driver environments and evidence.                                                |
| M4  | Verify REST API contract                     | Exercise affected management endpoints with persistence disabled and enabled.                                                              | Each endpoint returns the approved, documented response rather than an unexpected runtime database error.                                             | TODO   | Phase 2 identifies routes and expected statuses.                                                 |
| M5  | Re-run the original benchmark scenario       | Follow `baseline-e2e-verification.md` with the completed v3 runtime and no `[core.database]`.                                              | Tracker remains available without creating a database file, connecting to a database, or running migrations.                                          | TODO   | Append final command, logs, artifact inspection, and revision to `baseline-e2e-verification.md`. |
| M6  | Verify container startup without persistence | Build or run the supported container startup path with v3 database configuration omitted and all persistence-backed capabilities disabled. | The entrypoint does not require a database-driver override, install a default SQLite database, or create a database directory solely for the tracker. | TODO   | Phase 2 defines the exact image/configuration and evidence.                                      |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                        |
| ----- | ---------------------- | ----------------------------------------------- |
| AC1   | TODO                   | `analysis.md` lifecycle inventory               |
| AC2   | TODO                   | `analysis.md` consumer and API inventory        |
| AC3   | TODO                   | `solution.md` approved contract                 |
| AC4   | TODO                   | Implementation tests and M1 evidence            |
| AC5   | TODO                   | Validation tests and M2 evidence                |
| AC6   | TODO                   | REST API tests and M4 evidence                  |
| AC7   | TODO                   | Driver/migration tests and M3 evidence          |
| AC8   | TODO                   | EPIC and migration-document updates             |
| AC9   | TODO                   | V2 compatibility tests/review                   |
| AC10  | TODO                   | M5 evidence in `baseline-e2e-verification.md`   |
| AC11  | TODO                   | M6 container-startup evidence                   |
| AC12  | TODO                   | `linter all` output                             |
| AC13  | TODO                   | Focused, relevant workspace, and M1–M6 evidence |
| AC14  | TODO                   | Post-implementation acceptance review           |

## Risks and Trade-offs

- **Hidden persistence coupling**: A path may access a repository without an
  obvious feature switch. Mitigation: Phase 1 traces construction and all
  `tracker-core` repository consumers before Phase 2 chooses an API.
- **Silent data loss or degraded private mode**: Treating persistence as
  optional could accidentally disable a required feature. Mitigation: reject
  invalid combinations during startup validation and test every enabled feature.
- **Incomplete migration gating**: Connecting to a configured driver may still
  run migrations in an unintended path. Mitigation: trace and test construction
  and migration invocation separately for all drivers.
- **REST API inconsistency**: Management routes may expose unavailable data or
  fail internally. Mitigation: inventory route-to-domain dependencies and define
  explicit endpoint behaviour before implementation.
- **V3 activation sequencing**: The v3 runtime migration in #1980 may otherwise
  activate a configuration contract that must change. Mitigation: Phase 2 makes
  and records an explicit blocker decision before #1980 is completed.

## References

- Original issue: #999
- Original design comment: https://github.com/torrust/torrust-tracker/issues/999#issuecomment-2273652872
- Parent EPIC: #1978
- V3 database-shape issue: #1490
- V3 runtime-consumer migration: #1980
- SQLite migrations: `packages/tracker-core/migrations/sqlite/`
- PostgreSQL migrations: `packages/tracker-core/migrations/postgres/`
