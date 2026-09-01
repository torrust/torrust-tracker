---
doc-type: issue
issue-type: feature
status: done
priority: p2
epic: 1978
github-issue: 2107
spec-path: docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/ISSUE.md
branch: "2107-activate-persistence-free-v3-runtime-composition"
related-pr: 2112
depends-on:
  - 999
  - 1980
last-updated-utc: 2026-09-01 10:27
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/1978-configuration-overhaul-epic/EPIC.md
    - docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/closed/999-1978-optional-database-configuration/persistence-free-runtime-activation-draft.md
    - docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md
    - packages/configuration/docs/migrate-v2-to-v3.md
    - src/bootstrap/app.rs
    - src/bootstrap/persistence.rs
    - src/container.rs
    - packages/tracker-core/src/container.rs
    - share/container/entry_script_sh
    - contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh
    - docs/issues/open/2121-propagate-bootstrap-startup-errors/ISSUE.md
    - docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/manual-t2-rest-route-contract.md
    - docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/manual-t3-persistence-free-runtime.md
    - docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/manual-m2-persistence-requirements.md
    - docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/manual-m3-configured-driver-lifecycle.md
    - docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/manual-m6-container-no-persistence.md
---

# Issue #2107 - Activate persistence-free v3 runtime composition

> **EPIC position:** Configuration-overhaul subissue of EPIC #1978. This issue
> follows #999 and #1980, which respectively introduced v3
> `Option<Database>` and activated v3 at runtime with a temporary fixed-SQLite
> compatibility bridge.

## Goal

Honor an omitted v3 `[core.database]` at runtime when no enabled capability
requires persistence. A persistence-free public HTTP and/or UDP tracker must
start without a database driver, database file, database connection, migration,
persistence store, or database-backed service.

## Background

Before this issue, runtime composition substituted `Database::default()` when
the v3 `core.database: Option<Database>` field was absent. It consequently
started SQLite persistence and ran the shared migrations. The bootstrap
requirement check was also not active, and a persistence-free core container
could not form a usable public service graph.

T1-T3 delivered bootstrap validation, a true persistence-free public
HTTP/UDP/REST graph, and configuration-disabled `409` responses for direct
key/whitelist operations. The P1-P7 follow-up refactor then removed introduced
leaf-level persistence assumptions. The remaining work is T4-T7: configured
driver lifecycle, the container entrypoint, transition coverage, and complete
manual/documentation evidence.

## Scope

### In Scope

- Remove the fixed-SQLite compatibility bridge and compose from the actual v3
  `core.database: Option<Database>` value.
- Invoke the bootstrap-owned persistence requirement matrix after configuration
  validation but before global or application-container construction.
- Preserve the existing matrix entries for `core.listed`, `core.private`, and
  `core.tracker_policy.persistent_torrent_completed_stat`.
- Construct a usable persistence-free application graph for public HTTP and/or
  UDP tracker listeners. Resolve optionality at explicit composition seams;
  do not create a no-op database implementation or propagate an `Option`
  through unrelated consumers.
- In the persistence-free branch, construct no concrete database driver,
  `DatabaseStores`, migration runner, database-backed repository, key handler,
  whitelist manager, or database-backed torrent-metrics service.
- Adapt tracker-core services and jobs whose constructor signatures currently
  require database-backed metric repositories even when persistent completed
  metrics are disabled, including the default-enabled tracker usage statistics
  event listener.
- Keep the management REST API available in persistence-free operation. Its
  whitelist and key-management routes must remain registered but return a
  controlled HTTP `409 Conflict` response when `core.listed` or `core.private`
  is disabled, respectively. These requests must not construct or access
  persistence services.
- Keep torrent, statistics, and metrics routes available from their in-memory
  data. Document that completed-count values are process-local when persistence
  is absent; a later API version may add explicit historical-data provenance
  without changing this release's response shape.
- Preserve current server-error behavior for a configured database that fails
  operationally. Disabled-by-configuration responses must be distinct from
  database failures.
- Preserve the enabled-persistence lifecycle: a configured database selects one
  driver and runs the complete shared migration set before its required stores
  and services are built. Do not introduce feature-specific schemas, migration
  streams, or migration selection.
- Make the supported container entrypoint configuration-driven. A documented
  v3 no-persistence configuration source must start without a database-driver
  environment override, a packaged SQLite installation, or creation of the
  tracker database directory solely for persistence.
- Preserve operator-managed database state across restart/configuration
  transitions. The tracker must not delete, overwrite, migrate, copy, or
  otherwise alter an unselected database target.
- Execute and record the applicable #999 manual scenarios and update its
  acceptance evidence, migration guide, and operational documentation.

### Out of Scope

- Changing v2 configuration behavior, defaults, validation, or database
  lifecycle.
- Changing the REST API response shape or adding a completed-count provenance
  field; a later API version may make that distinction explicit.
- Feature-specific database schemas or partial migration streams.
- Automatically moving data between configured database targets.
- Persistence-awareness work not necessary for the initial public HTTP/UDP
  tracker composition.
- Refactoring bootstrap startup failures to return and propagate typed errors.
  That follow-up is explicitly deferred until this issue is complete; see
  `bootstrap-error-propagation-draft.md`.

## Architectural Decisions

- Related ADR: `docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md`.
- The bootstrap layer owns the requirement matrix exactly once. It must not be
  duplicated in configuration validation, route handlers, or repositories.
- `http_api` alone does not require persistence. The API remains available;
  individual routes represent disabled `listed` and `private` capabilities as
  controlled `409 Conflict` responses. This corrects the current behavior,
  which permits those routes to mutate persistent state even when their tracker
  feature is disabled.
- Disabled-capability responses must use the established `ActionStatus::Err`
  shape and a distinct `DisabledByConfiguration`-style domain error. They must
  not reuse an operational database error, which continues to map to the
  existing server-error response.
- The persistence-free path must be a real composition branch. A no-op database
  driver or repository is not acceptable because it can conceal unexpected
  persistence access.
- An important new architecture decision discovered during implementation must
  be recorded in a new ADR before it is finalized. No additional ADR is known
  to be required at drafting time.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                      | Notes / Expected Output                                                                                                                                                                 |
| --- | ------ | ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Activate persistence validation           | Active v3 bootstrap invokes the centralized check after configuration validation and before globals or containers are built.                                                            |
| T2  | DONE   | Compose capability-aware REST API         | Key and whitelist routes retain registration but short-circuit to `409`/`ActionStatus::Err` when their capability is disabled; their adapters are not constructed.                      |
| T3  | DONE   | Build persistence-free core graph         | Tracker-core now groups database stores and persistence-only services in optional `PersistenceServices`; public HTTP/UDP and REST composition has no database fallback.                 |
| T4  | DONE   | Preserve persistence-enabled composition  | SQLite, MySQL, and PostgreSQL configured-driver lifecycle suites passed, including complete-migration and idempotency coverage.                                                         |
| T5  | DONE   | Adapt supported container startup         | A packaged v3 public default omits persistence; no override, SQLite seed, or persistence-only directory is used unless SQLite is explicitly selected.                                   |
| T6  | DONE   | Add regression and transition tests       | Covered mounted-configuration precedence and non-destructive SQLite disable, target-change, and reuse transitions; existing configuration coverage preserves the v2 rejection boundary. |
| T7  | DONE   | Execute manual evidence and documentation | M1-M6, #999 evidence, migration guidance, final acceptance review, and quality gates are complete.                                                                                      |

## Progress Tracking

### Workflow Checkpoints

- [x] Draft copied from the #999 activation-follow-up planning artifact
- [x] Draft reconciled with merged #1980 runtime behavior
- [x] Draft reviewed and approved by user/maintainer
- [x] GitHub issue #2107 created, linked as a subissue of EPIC #1978, and number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (#2108)
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and applicable pre-push checks)
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and specification moved to `docs/issues/closed/`

### Current Delivery Status

All implementation tasks T1-T7 are complete. Focused configuration tests, the
release-image transition regression, `linter all`, the pre-commit gate, and the
prior applicable pre-push suite pass. M1-M6 and #999 acceptance evidence are
recorded. The issue remains open only for pull-request review and formal issue
closure; see `persistence-capability-refactor.md` for the P1-P7 implementation
record and its deferred design boundaries.

### Progress Log

- 2026-08-28 00:00 UTC - GitHub Copilot - Copied the post-#1980 activation-follow-up draft from #999 and reconciled it with merged runtime code. Confirmed that bridge removal alone cannot produce a usable persistence-free tracker; expanded planned scope to real public tracker composition and bootstrap validation. Initial draft temporarily classified `http_api` as persistence-required pending maintainer review.
- 2026-08-28 00:00 UTC - User/GitHub Copilot - Approved keeping the management REST API available without persistence. This issue now corrects disabled whitelist/key endpoint behavior through controlled HTTP 409 responses while preserving API-wide availability; a later API version may add a response-field distinction between session and historical completed counts.
- 2026-08-28 00:00 UTC - GitHub Copilot - User approved the refined specification. Created GitHub issue #2107 and linked it as a native subissue of EPIC #1978.
- 2026-08-28 00:00 UTC - GitHub Copilot/User - Merged spec-only PR #2108 and started T1, activation of bootstrap persistence validation.
- 2026-08-28 00:00 UTC - GitHub Copilot - Activated the existing centralized persistence requirement check in bootstrap after configuration validation and before global or application-container construction. Focused root bootstrap tests passed.
- 2026-08-28 11:58 UTC - GitHub Copilot/User - Promoted this specification to
  an issue-local folder and recorded a deferred follow-up draft for typed
  bootstrap error propagation. The follow-up is not part of #2107.
- 2026-08-28 12:56 UTC - GitHub Copilot - Completed T2. REST route composition
  now reads `private` and `listed` from the existing tracker-core configuration,
  constructs the corresponding persistence-backed adapter only when enabled,
  and returns JSON `ActionStatus::Err` with HTTP `409` otherwise. Focused
  contracts force a database failure before each disabled request, proving the
  persistence service is not called. Existing enabled-route and operational
  database-failure contracts remain green.
- 2026-08-28 12:58 UTC - GitHub Copilot - Manually verified the T2 route
  contract against a locally running public-mode tracker with an isolated
  configured SQLite database. Authenticated key and whitelist requests returned
  their documented JSON `ActionStatus::Err`/HTTP `409` responses, while the
  health endpoint returned HTTP `200`. See `manual-t2-rest-route-contract.md`.
- 2026-08-28 14:38 UTC - GitHub Copilot - Completed T3. Removed the fixed
  SQLite bridge and composed public runtime services without persistence while
  grouping database-backed services explicitly. Local public HTTP/UDP/REST
  verification with `database: null` passed; see
  `manual-t3-persistence-free-runtime.md`.
- 2026-08-28 - GitHub Copilot - Completed P1-P4. Tracker-core now composes
  public or persistent-statistics announce state explicitly, splits in-memory
  and persistent completed-statistics listeners, and rejects persistent
  completed statistics unless tracker usage statistics is enabled. Focused
  tracker-core and bootstrap validation checks passed.
- 2026-08-28 - GitHub Copilot - Completed P5-P7. Startup loaders and REST
  private-key/whitelist adapter composition retain configuration as the feature
  gate and no longer assert optional persistence at leaf boundaries.
  `TorrentsManager` receives its required completed-downloads repository only
  for its persistence-only restoration operation. Focused application, REST,
  manager, and tracker-core integration checks passed.
- 2026-08-29 10:18 UTC - GitHub Copilot - Reconciled current normative
  documentation. #2107 owns the delivered persistence-free REST behavior;
  #144 remains the deferred next-major completed-metric provenance work.
  T4-T7 remain open; no issue-wide driver, container, transition, or complete
  manual-evidence claim is made.
- 2026-08-29 10:43 UTC - GitHub Copilot - Completed M2 against the active v3
  bootstrap. With no `[core.database]`, independently enabling `listed`,
  `private`, or persistent completed metrics produced its stable
  capability-specific requirement diagnostic in `setup` before application
  composition. See `manual-m2-persistence-requirements.md`.
- 2026-08-29 10:57 UTC - GitHub Copilot - Completed M5. The active v3 tracker
  remained alive through a bounded isolated no-persistence baseline run. The
  new working directory contained only its log and no database artifact; see
  #999 `baseline-e2e-verification.md`. Supported-container verification remains
  M6.
- 2026-08-29 11:23 UTC - GitHub Copilot - Ran M3 configured-driver lifecycle
  checks. SQLite and PostgreSQL suites passed, including the PostgreSQL
  four-migration assertion. MySQL initially could not start because Docker Hub
  returned HTTP 401 when testcontainers requested `mysql:8.0`.
- 2026-08-29 21:45 UTC - GitHub Copilot - Retried M3 after `docker pull
mysql:8.0` succeeded. The canonical MySQL compatibility suite passed, so
  SQLite, MySQL, and PostgreSQL all have configured-driver lifecycle evidence.
  T4/M3/AC7 are complete; see `manual-m3-configured-driver-lifecycle.md`.
- 2026-08-30 09:50 UTC - GitHub Copilot - Completed T5/M6. The built release
  image started its packaged v3 no-persistence public configuration without a
  driver override. Health checks passed, and isolated mounted state contained
  no database directory or SQLite file; see `manual-m6-container-no-persistence.md`.
- 2026-08-30 12:25 UTC - GitHub Copilot - Began T6. A mounted v3
  no-persistence configuration paired with an explicit SQLite driver override
  preserved the mounted configuration and created no SQLite directory or file.
  The regression is recorded in
  `contrib/dev-tools/containers/tests/test-mounted-no-persistence-configuration.sh`.
- 2026-08-30 12:39 UTC - GitHub Copilot - Completed T6. The container
  transition regression starts the release image with `old.sqlite3`, no
  persistence, `new.sqlite3`, and `old.sqlite3` again. SHA-256 checks confirm
  both unselected SQLite targets remain unchanged. The active v3
  configuration's existing schema-version test also continues to reject v2,
  matching the established migration boundary.
- 2026-08-30 13:34 UTC - GitHub Copilot - Completed T7 and re-reviewed all
  acceptance criteria against the recorded evidence. The shipped-template and
  v2-boundary tests, release-image transition regression, and `linter all`
  passed. #999 evidence and v2-to-v3 migration guidance now describe the
  active persistence-free v3 runtime contract.

## Acceptance Criteria

- [x] AC1: Active v3 bootstrap evaluates the centralized persistence-requirement
      matrix before application composition.
- [x] AC2: With no `[core.database]`, each enabled required capability fails
      deterministically before containers are constructed: `core.listed`,
      `core.private`, and persistent completed metrics.
- [x] AC3: `http_api` alone is usable without `[core.database]`; no API-wide
      startup rejection or late composition panic occurs.
- [x] AC4: A v3 public HTTP and/or UDP tracker with no required capability and
      no `[core.database]` constructs and serves protocol traffic successfully.
- [x] AC5: The persistence-free composition constructs no concrete driver,
      database stores, migrations, database-backed repositories, database file,
      or network database connection.
- [x] AC6: Persistence-free operation works when
      `core.tracker_usage_statistics = true`, which is the current default.
- [x] AC7: With `[core.database]`, SQLite, MySQL, and PostgreSQL retain the
      all-or-nothing driver and complete shared migration lifecycle.
- [x] AC8: V2 configuration and runtime behavior remain unchanged. The active
      v3 runtime retains its established v2-schema rejection boundary, covered
      by `v3_configuration_should_reject_schema_version_2_0_0`.
- [x] AC9: Whitelist and key-management routes remain registered but return
      HTTP `409 Conflict` with `ActionStatus::Err` when their respective
      feature is disabled, without database access. Configured operational
      database failures remain distinguishable server errors.
- [x] AC10: Torrent, statistics, and metrics routes remain available in
      persistence-free operation. Documentation does not claim an across-restart
      lifetime interpretation for completed counts without persistence.
- [x] AC11: The supported container startup path runs a documented v3
      no-persistence configuration without a database-driver override, packaged
      SQLite setup, or a tracker database directory created solely for
      persistence.
- [x] AC12: Persistence configuration restart transitions leave unselected
      database targets unchanged and never copy data automatically. The
      release-image transition regression checks checksums across persistence
      disable, target change, and original-target reuse.
- [x] AC13: #999 manual evidence and acceptance verification are updated
      truthfully, including API-disabled-capability evidence.
- [x] AC14: `linter all` exits with code `0`, relevant automated tests pass,
      and acceptance criteria are re-reviewed against observed evidence.

## Verification Plan

### Automatic Checks

- Focused bootstrap tests for the persistence-requirement matrix.
- REST API contract tests for disabled whitelist/key capabilities, API-wide
  persistence-free startup, and retained operational-database-failure behavior.
- Focused tracker-core, HTTP-core, UDP-core, and startup-job tests for an
  operational persistence-free service graph.
- Protocol integration tests proving public HTTP announce/scrape and UDP
  connect/announce/scrape work without `[core.database]`.
- Driver and migration tests for SQLite, MySQL, and PostgreSQL with persistence
  configured.
- Container entrypoint/image tests for both no-persistence and configured
  persistence startup paths.
- Restart-transition tests that inspect selected and unselected storage
  targets.
- `cargo machete`, `linter all`, documentation tests, the mandatory pre-commit
  gate, and relevant workspace/pre-push checks.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`, `DEFERRED`.

| ID  | Scenario                                      | Command/Steps                                                                                                                                | Expected Result                                                                                                                                   | Status | Evidence                                                                                      |
| --- | --------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------------------------------------------- |
| M1  | Start public v3 tracker without persistence   | Start an ephemeral public HTTP and/or UDP tracker with no `[core.database]`, no required capabilities, and tracker usage statistics enabled. | Startup and protocol traffic succeed with no persistence artifacts.                                                                               | DONE   | Local source-tree evidence: `manual-t3-persistence-free-runtime.md`.                          |
| M2  | Reject all missing-persistence combinations   | Independently enable listing, private mode, and persistent completed metrics without `[core.database]`.                                      | Each configuration fails before composition with its stable requirement diagnostic.                                                               | DONE   | `manual-m2-persistence-requirements.md`; #999 M2/AC5 updated.                                 |
| M3  | Initialize configured drivers                 | Start each supported configured driver with a persistence-required capability enabled.                                                       | The selected driver and complete shared migrations initialize normally.                                                                           | DONE   | SQLite, MySQL, and PostgreSQL lifecycle evidence: `manual-m3-configured-driver-lifecycle.md`. |
| M4  | REST API persistence-free route contract      | Start `http_api` with no persistence, exercise torrent/stats/metrics routes and disabled whitelist/key routes.                               | API starts; in-memory routes remain available; disabled direct capability routes return controlled HTTP 409 responses without persistence access. | DONE   | Local source-tree evidence: `manual-t3-persistence-free-runtime.md`.                          |
| M5  | Repeat baseline no-persistence run            | Follow `baseline-e2e-verification.md` with the active v3 runtime and no `[core.database]`.                                                   | Tracker remains available without a database file, connection, or migration.                                                                      | DONE   | #999 `baseline-e2e-verification.md`.                                                          |
| M6  | Start supported container without persistence | Build/run the normal image using the documented no-persistence v3 configuration and no driver override.                                      | Entrypoint does not select/install SQLite or create its database directory solely for tracker persistence.                                        | DONE   | Release-image evidence: `manual-m6-container-no-persistence.md`.                              |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                                                                                      |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | Bootstrap wiring and focused root bootstrap tests                                                                                                                                                             |
| AC2   | DONE                   | Focused matrix tests plus M2 local runtime evidence in `manual-m2-persistence-requirements.md`.                                                                                                               |
| AC3   | DONE                   | REST API started and served health plus torrent routes with `database: null`; `manual-t3-persistence-free-runtime.md`.                                                                                        |
| AC4   | DONE                   | Local HTTP and UDP announces passed with `database: null`; `manual-t3-persistence-free-runtime.md`.                                                                                                           |
| AC5   | DONE                   | Constructor tests plus M5 isolated artifact inspection in #999 `baseline-e2e-verification.md`.                                                                                                                |
| AC6   | DONE                   | Focused listener lifecycle test and local run passed with tracker usage statistics enabled and `database: null`.                                                                                              |
| AC7   | DONE                   | SQLite, MySQL, and PostgreSQL lifecycle suites passed; see `manual-m3-configured-driver-lifecycle.md`.                                                                                                        |
| AC8   | DONE                   | Existing configuration compatibility test `v3_configuration_should_reject_schema_version_2_0_0` confirms the intentional active-runtime v2 rejection boundary remains unchanged.                              |
| AC9   | DONE                   | Two forced-database-failure REST contracts return `409`/`ActionStatus::Err`; all 55 REST integration tests retain enabled and operational-error behavior; local evidence: `manual-t2-rest-route-contract.md`. |
| AC10  | DONE                   | Local REST torrent query returned the in-memory swarm; disabled capability routes returned `409`; `manual-t3-persistence-free-runtime.md`.                                                                    |
| AC11  | DONE                   | Release-image M6 evidence: `manual-m6-container-no-persistence.md`.                                                                                                                                           |
| AC12  | DONE                   | `test-mounted-no-persistence-configuration.sh` checks old/new SQLite checksums across persistence disable, target change, and original-target reuse.                                                          |
| AC13  | DONE                   | #999 M1-M6 scenario and acceptance records now link the #2107 runtime, REST, container, and transition evidence.                                                                                              |
| AC14  | DONE                   | Focused shipped-template/v2-boundary tests, release-image transition regression, `linter all`, and the prior applicable pre-push suite passed; acceptance criteria re-reviewed.                               |

## Risks and Trade-offs

- **Composition breadth:** Existing container fields and constructors make
  persistence mandatory. Mitigation: introduce explicit persistence-enabled and
  persistence-free composition branches, retaining non-optional dependencies in
  the enabled branch.
- **API compatibility:** Disabled endpoints previously operate against the
  database even when their feature is disabled. Mitigation: retain their route
  paths and response envelope, but make the behavior explicit as HTTP 409;
  document this breaking correction in the v3 migration guidance.
- **Hidden side effects:** A no-op implementation could prevent visible driver
  setup while retaining unexpected database-shaped services. Mitigation: assert
  absence at construction seams and inspect isolated runtime artifacts.
- **Entrypoint ambiguity:** Removing the driver override without defining a
  configuration source can leave container startup unspecified. Mitigation:
  explicitly document and test one supported v3 no-persistence source.
- **Data loss:** Restart changes can accidentally alter old targets. Mitigation:
  test checksums/state before and after disable, re-enable, and target-change
  transitions.

## References

- Parent EPIC: #1978
- Prerequisite issue: #999
- V3 runtime activation: #1980
- Future REST API evolution: #144
- `docs/issues/closed/999-1978-optional-database-configuration/analysis.md`
- `docs/issues/closed/999-1978-optional-database-configuration/solution.md`
- `docs/issues/closed/999-1978-optional-database-configuration/persistence-unavailable-scenarios.md`
- `docs/issues/closed/999-1978-optional-database-configuration/baseline-e2e-verification.md`
- `docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md`
