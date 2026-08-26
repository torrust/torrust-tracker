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
last-updated-utc: 2026-08-26 16:45
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
    - docs/issues/open/999-1978-optional-database-configuration/adr-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-awareness-epic-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-free-runtime-activation-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/persistence-unavailable-scenarios.md
---

# Issue #999 - Make v3 database configuration optional when persistence is unused

> **EPIC position**: Configuration-overhaul subissue of EPIC #1978. It follows
> #1490, which defines the driver-specific v3 `Database` representation. Phase
> 1 and Phase 2 must determine whether this issue blocks #1980 and activation of
> the v3 configuration schema.

## Goal

Allow configuration schema v3.0.0 to represent an omitted `[core.database]`
section with `Option<Database>`, while preserving the existing effective
database dependency through the temporary v3-activation compatibility bridge.
The post-activation follow-up drafted in this folder will make an omitted
database suppress driver construction, database files, network connections, and
migrations when no enabled capability requires persistence.

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
- Prepare the bootstrap validation design for every enabled capability that
  requires persistence; the post-activation follow-up implements it when
  bootstrap receives the actual v3 `Option<Database>`.
- Preserve the all-or-nothing schema lifecycle: once any enabled capability
  requires persistence, initialize the selected database and apply the complete
  shared migration set. Feature configuration controls code behavior, not
  schema fragments: do not create feature-specific database schemas,
  feature-specific migration streams, or feature-specific migration selection.
- Prepare optional persistence dependencies needed by the management REST API.
  The post-activation follow-up keeps it persistence-required; API #144 later
  makes it available without persistence and adds explicit
  configuration-disabled direct-route responses.
- Prepare a future persistence-awareness EPIC draft for remaining metric
  provenance and broader persistence-decoupling behavior.
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

The working Phase 2 direction is one reusable bootstrap-owned
application-composition validation step. Issue #999 implements and unit-tests
it, owning the feature-to-database requirement matrix exactly once; the same
rules must not be duplicated in `packages/configuration::Validator`. The
post-#1980 activation follow-up invokes it after v3 configuration loading and
before `AppContainer` construction, once bootstrap receives actual
`Option<Database>` rather than the temporary bridge.
The management REST API does not require persistence in the target architecture.
However, the approved HTTP 409 configuration-disabled response contract is
deferred to next-major REST API work in GitHub issue #144. Until that work is
implemented, `http_api` remains persistence-required in the activation
follow-up; it must not reinterpret intentionally absent persistence as an
operational database failure.

The initial persistence-required capabilities are `core.listed`, `core.private`,
and `core.tracker_policy.persistent_torrent_completed_stat`. If implementation
finds another persistence-required capability, it must be added to the one
centralized bootstrap matrix and its focused tests rather than checked ad hoc
by a repository, route, or feature.

The implementation must keep feature-to-database requirements explicit at the
application boundary. It must not distribute optional-database checks through
repositories or feature implementation code, where a missed call site could
become a delayed runtime failure.

### Decision 4: Start Phase 3 at the existing optional database initialization seam

Phase 3 selects, provisionally and reversibly, `Option<Database>` at the
existing tracker-core initialization seam. The container selects a
persistence-enabled or persistence-absent composition path before constructing
services that require initialized stores. Consequently, the enabled path can
continue passing ordinary required persistence dependencies to its consumers;
an `Option` must not cascade through every persistence consumer merely because
configuration can omit the database.

This is deliberately less invasive than injecting an
`Option<PersistenceServices>` bundle of already-initialized stores from
bootstrap. The alternative remains documented in `solution.md` and is the
fallback if the selected seam cannot keep the optional state at composition
without making container fields or unrelated consumers optional. Driver and
migration implementation ownership remains in `tracker-core` unless Phase 3
evidence establishes a reason to move it; this decision changes where
optionality is resolved, not schema ownership.

- Related ADRs:
  `docs/adrs/20260723184019_separate_configuration_value_invariants_from_consistency_validation.md`.
- ADRs to create: Decide during Phase 2. Create an ADR only if the selected
  optional-persistence lifecycle changes an enduring architecture boundary.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                           | Notes / Expected Output                                                                                                                                |
| --- | ------ | ---------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | DONE   | Complete persistence analysis                  | `analysis.md` records current lifecycle, all discovered consumers, REST coupling, and driver-specific migration behaviour.                             |
| T2  | DONE   | Approve an optional-persistence design         | `solution.md` records the approved v3 contract, validation, API deferrals, compatibility bridge, and staged ordering.                                  |
| T3  | DONE   | Implement optional v3 database configuration   | `v3_0_0::Core.database` is `Option<Database>`; omitted TOML persists and loads as `None`. V2 remains unchanged.                                        |
| T4  | DONE   | Add regression coverage                        | Focused v3 parsing/serialization, validation-matrix, and optional constructor coverage added. Runtime-free scenarios stay deferred.                    |
| T5  | DONE   | Update migration and operational documentation | Published ADR `20260825193119_make_persistence_an_optional_application_composition_capability.md`; activation guidance remains in the follow-up draft. |
| T6  | DONE   | Verify and re-review                           | Focused tests, workspace compilation, `linter all`, and pre-commit pass. M1-M6 remain deferred to the activation follow-up.                            |

## Progress Tracking

### Workflow Checkpoints

- [x] GitHub issue #999 reviewed, including its original implementation comment
- [x] Spec-only branch created
- [x] Folder-based specification scaffold created
- [x] Spec reviewed and approved by user/maintainer
- [x] Spec-only PR merged into `develop` (#2094, merge commit `7aad6e79`)
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
- 2026-08-25 00:00 UTC - GitHub Copilot - Completed Phase 1 evidence in
  `analysis.md`: active v2/v3 configuration status, unconditional driver and
  migration lifecycle, container side effects, persistence consumers, REST API
  routes, validation layering, and Phase 2 questions. No runtime behavior or
  solution decision changed.
- 2026-08-25 00:00 UTC - GitHub Copilot/User - Recorded a working Phase 2
  direction in `solution.md`: initial v3 persistence-free operation is limited
  to deployments without listing, private keys, persistent completed metrics,
  or the management REST API; bootstrap owns one requirement check; a future
  persistence-awareness EPIC owns wider API and metric semantics. Explicit
  Phase 2 approval remains required.
- 2026-08-25 00:00 UTC - GitHub Copilot/User - Corrected the working direction:
  the management REST API remains available without persistence. Phase 3 must
  make its construction persistence-aware, return configuration-disabled
  responses for direct disabled capabilities, and make metric history explicit.
  Added draft ADR and future-EPIC artifacts for refinement during Phase 3.
- 2026-08-25 00:00 UTC - User - Approved v3 `Option<Database>` for the
  persistence-free contract. The implementation must test versioned v3
  configuration and v3-compatible composition before #1980 activates v3
  production consumers; it must not activate v3 early solely for testing.
- 2026-08-25 00:00 UTC - GitHub Copilot/User - Adopted staged activation:
  #999 adds the v3 optional representation and optional container dependencies;
  #1980 activates v3 with an explicit temporary database bridge; a small
  follow-up then honors `None` at runtime and completes persistence-free
  verification. Added the follow-up issue draft.
- 2026-08-25 00:00 UTC - User - Approved bootstrap as the single validation
  owner. Issue #999 implements and tests the reusable requirement matrix; the
  post-#1980 follow-up invokes it when replacing the temporary bridge with the
  actual v3 `Option<Database>` value.
- 2026-08-25 00:00 UTC - User - Approved the initial persistence-required
  capability matrix: listing, private mode, and persistent completed metrics.
  Any implementation discovery must extend the same centralized matrix and
  tests, not introduce a feature-local missing-database check.
- 2026-08-25 00:00 UTC - User - Approved `PersistenceRequirementError` with
  one diagnostic per persistence-required capability. Approved the desired REST
  configuration-disabled contract (HTTP 409, `ActionStatus::Err`, and a
  distinct disabled-by-configuration error), but deferred its implementation
  and historical-metric API changes to next-major REST API work in GitHub issue
  #144. Until then, `http_api` remains persistence-required at activation.
- 2026-08-25 00:00 UTC - User - Confirmed that session-versus-historical
  response-field semantics are deferred to the REST API v2 subissue draft under
  GitHub issue #144. The approved constraints remain no numeric sentinel and no
  session-only value documented as lifetime history.
- 2026-08-25 00:00 UTC - User - Approved the all-or-nothing persistence
  lifecycle. Once persistence is present, initialize one driver and the full
  shared schema; feature configuration controls code behavior only, not
  conditional schema or migration fragments.
- 2026-08-25 00:00 UTC - User - Approved the restart-only, non-destructive
  persistence transition contract: disabling persistence leaves prior database
  state untouched; re-enabling the same target reuses it; changing targets does
  not transfer data automatically; data produced while disabled is not
  recoverable.
- 2026-08-25 00:00 UTC - User - Approved the container entrypoint contract:
  defer persistence selection to actual v3 configuration, do not perform
  persistence-specific setup when absent, and never destructively alter mounted
  configuration or database state during transitions.
- 2026-08-25 00:00 UTC - User - Approved `adr-draft.md` as the Phase 3 ADR
  starting point. It must be copied to `docs/adrs/` with a timestamped filename
  and reconciled with final code, tests, API contract, and review outcome.
- 2026-08-25 00:00 UTC - User - Approved `persistence-awareness-epic-draft.md`
  as the post-merge starting point. Reconcile it with merged #999, #1980,
  persistence-free activation-follow-up, and API #144 work before creating the
  GitHub EPIC.
- 2026-08-25 00:00 UTC - User - Approved the staged #999 -> #1980 ->
  persistence-free activation-follow-up ordering. EPIC #1978 and the v2-to-v3
  migration guide record it. The activation-follow-up draft remains planning
  only until #999/#1980 implementation evidence permits it to be refined and
  opened.
- 2026-08-25 00:00 UTC - User - Approved the Phase 3 implementation and
  evidence sequence. The activation-follow-up draft records ownership across
  #999, #1980, the later runtime activation, and API #144; do not create that
  follow-up issue until preceding implementation evidence is reviewed.
- 2026-08-25 00:00 UTC - User - Approved the complete Phase 2 design for the
  analysis-and-solution PR. `solution.md` contains the approval record; Phase 3
  implementation remains a separate delivery.
- 2026-08-25 00:00 UTC - User/GitHub Copilot - For Phase 3, selected the
  existing tracker-core initialization seam as the provisional location for
  `Option<Database>`. The `Some` branch must retain required initialized-store
  dependencies, avoiding an `Option` cascade through consumers. The optional
  pre-initialized persistence-services injection alternative remains a
  documented fallback if this selection cannot keep optionality at composition.
- 2026-08-25 00:00 UTC - GitHub Copilot - Implemented the Phase 3 v3
  `Option<Database>` representation, persistence-safe serialization, optional
  tracker-core constructor seam, named active-v2 compatibility bridge, and the
  bootstrap-owned requirement matrix. Published ADR
  `20260825193119_make_persistence_an_optional_application_composition_capability.md`.
  The runtime still explicitly supplies persistence; activation and M1-M6 remain
  deferred to the post-#1980 follow-up.
- 2026-08-25 00:00 UTC - GitHub Copilot - Re-reviewed the Phase 3
  implementation after correcting the optional composition seam so that the
  supplied database, rather than `Core.database`, drives persistence setup.
  Focused configuration, tracker-core, and application tests passed; workspace
  targets compiled; `linter all` and the mandatory pre-commit gate passed.
- 2026-08-26 16:45 UTC - GitHub Copilot/User - #1980 activated v3 consumers while retaining the approved named fixed-SQLite compatibility bridge. Active runtime composition therefore remains persistence-enabled; omitted `[core.database]` is still not honored at runtime. The post-#1980 activation follow-up remains responsible for passing the actual optional value, invoking the bootstrap requirement matrix, and completing M1-M6.

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

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                 |
| ----- | ---------------------- | -------------------------------------------------------- |
| AC1   | DONE                   | `analysis.md` lifecycle inventory                        |
| AC2   | DONE                   | `analysis.md` consumer and API inventory                 |
| AC3   | DONE                   | `solution.md` approval record                            |
| AC4   | TODO                   | Implementation tests and M1 evidence                     |
| AC5   | TODO                   | Validation tests and M2 evidence                         |
| AC6   | TODO                   | REST API tests and M4 evidence                           |
| AC7   | TODO                   | Driver/migration tests and M3 evidence                   |
| AC8   | DONE                   | Approved staged ordering in EPIC and migration guide     |
| AC9   | DONE                   | V2 configuration tests and active explicit bridge review |
| AC10  | TODO                   | M5 evidence in `baseline-e2e-verification.md`            |
| AC11  | TODO                   | M6 container-startup evidence                            |
| AC12  | DONE                   | `linter all` passed on 2026-08-25                        |
| AC13  | TODO                   | Focused, relevant workspace, and M1–M6 evidence          |
| AC14  | DONE                   | Phase 3 review; activation-owned criteria remain pending |

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
- PostgreSQL migrations: `packages/tracker-core/migrations/postgresql/`
