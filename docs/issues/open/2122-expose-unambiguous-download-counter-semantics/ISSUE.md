---
doc-type: issue
issue-type: bug
status: open
priority: p2
epic: null
github-issue: 2122
spec-path: docs/issues/open/2122-expose-unambiguous-download-counter-semantics/ISSUE.md
branch: "2122-expose-unambiguous-download-counter-semantics"
related-pr: 2123
depends-on:
  - 2107
last-updated-utc: 2026-09-01 10:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/closed/2107-1978-activate-persistence-free-v3-runtime-composition/ISSUE.md
    - packages/tracker-core/src/statistics/mod.rs
    - packages/tracker-core/src/statistics/repository.rs
    - packages/tracker-core/src/statistics/persisted/mod.rs
    - packages/tracker-core/src/statistics/event/handler.rs
    - packages/tracker-core/tests/integration.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs
    - packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs
    - packages/axum-rest-api-server/src/v1/routes.rs
    - packages/axum-rest-api-server/tests/server/v1/contract/context/stats.rs
    - tests/scaffold.rs
    - tests/common/statistics.rs
    - docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/open/2122-expose-unambiguous-download-counter-semantics/manual-verification.md
---

<!-- skill-link: create-issue -->

# Issue #2122 - Expose unambiguous download counter semantics

## Goal

Expose separate session and persisted completed-download totals without breaking
v1 consumers. Establish the `in_session` and `persisted` metric naming
convention that API v2 will use as its unambiguous completed-count contract.

## Background

`tracker_core_persistent_torrents_downloads_total` is an in-memory counter. It increments for every `PeerDownloadCompleted` event when tracker usage statistics are enabled, including a persistence-free runtime. In that mode it resets when the tracker restarts.

When persistent completed statistics are enabled, startup restores the global database aggregate into the same counter and the persistent listener updates the database aggregate. It then represents a historical total across restart.

The session-versus-persistent behavior is intentional and recorded in commit `b0e74439`. The defect is the public metric identifier and description, the repository documentation, and the REST `Stats.completed` documentation: they claim or imply an always-persisted lifetime. The REST response shape does not identify the counter's retention mode.

The v1 contract is additive-only: existing fields cannot be renamed or removed
before API v2. The v4 tracker release can nevertheless add the unambiguous
fields now, allowing consumers to migrate before v2 removes the legacy
ambiguous field.

## Scope

### In Scope

- Retain the legacy `completed` REST field and
  `tracker_core_persistent_torrents_downloads_total` metric identifier and
  conditional value semantics for compatibility. Correct their descriptions
  and document their deprecation in favor of the new explicit fields/metrics.
- Add v1 REST fields `completed_in_session: u64`,
  `completed_persisted: u64`, and `completed_persisted_enabled: bool`.
  When persistence is disabled, `completed_persisted` is zero and
  `completed_persisted_enabled` is false; clients must use the boolean to
  distinguish disabled persistence from an enabled zero count.
- Publish separate `in_session` and `persisted` tracker-core metrics. The
  in-session metric has the same availability as tracker usage statistics; the
  persisted metric is exposed only when persistent completed statistics are
  enabled. Do not expose zero as a Prometheus disabled-state sentinel.
- Record an ADR defining `in_session` for process-lifetime metrics and
  `persisted` for metrics restored and maintained in persistent storage.
- Add focused regressions that prove a persistence-free restart resets the exposed total and configured persistence restores it.
- Preserve #2107's independent in-memory and persistence listener topology.

### Out of Scope

- Removing, renaming, or changing the value semantics of the legacy v1
  `completed` field.
- Removing, renaming, or changing the conditional value semantics of the
  legacy public `tracker_core_persistent_torrents_downloads_total` metric.
- Implementing API v2 or removing deprecated legacy fields and metrics.
- Reworking listener topology, event delivery, database schema, migrations, or persistence configuration.

## Architectural Decisions

- Related ADR: `docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md`.
- Related completed work: #2107 established persistence-free runtime behavior and split in-memory completed-count updates from database persistence updates.
- The legacy v1 `completed` field and existing metric retain their current
  conditional value to preserve consumers. They are deprecated through their
  descriptions and migration documentation in favor of explicit views and are
  removed only in API v2.
- `completed_in_session` starts at zero for each tracker process and increments
  for every completed-download event processed by the in-memory listener.
  `completed_persisted` starts at zero when disabled; when enabled, it is
  seeded from the database aggregate and advances after successful persistent
  updates. `completed_persisted_enabled` is the authoritative availability
  indicator. Consumers must never infer availability from a numeric zero.
- The REST composition root derives `completed_persisted_enabled` from the
  validated `persistent_torrent_completed_stat` configuration, rather than
  inferring it from a metric value or a composed database service.
- Manual verification uses SQLite because the required behavior is independent
  of a database driver. Use the documented v3 configurations for the
  persistence-free and persistence-enabled scenarios.
- Prometheus uses distinct `in_session` and `persisted` metric identifiers. The
  persisted metric is omitted from the exported metric collection when disabled,
  so zero remains an unambiguous observed historical count. The legacy metric
  stays exported with its current conditional value.
- Create a repository-wide ADR in `docs/adrs/` that defines retention names,
  legacy deprecation communication, update ordering, and this additive v1
  bridge. It explicitly refines #999's deferral: a zero persisted field is
  permitted only with the separate authoritative availability boolean.

## Known Refactoring Targets

These targets are confirmed current behavior and are subject to T1 reconciliation; they are not an exhaustive implementation inventory.

- `packages/tracker-core/src/statistics/mod.rs`: declare legacy, in-session,
  and persisted counter views, with accurate descriptions.
- `packages/tracker-core/src/statistics/repository.rs`: expose named queries
  and a capability-aware metric collection for all three views.
- `packages/tracker-core/src/statistics/event/handler.rs` and
  `packages/tracker-core/src/statistics/persisted/mod.rs`: update the
  independent in-session and persisted views in the defined order.
- `packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs`: add
  the three additive v1 fields, document legacy deprecation, and preserve
  deserialization compatibility for callers that consume older payloads.
- `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs` and
  `packages/axum-rest-api-server/src/v1/routes.rs`: map the separate values
  and inject validated persistence capability at adapter composition.
- `packages/tracker-core/tests/common/test_env.rs` and
  `packages/tracker-core/tests/integration.rs`: support persistence-free test
  environments and prove both restart contracts.
- `packages/axum-rest-api-server/tests/server/v1/contract/context/stats.rs`:
  review and extend the existing authenticated `GET /api/v1/stats` contract
  coverage for every new field.
- `tests/` and `tests/common/statistics.rs`: review the application-level REST
  test harness. Add a focused integration test when endpoint values and metric
  presence/absence across both persistence modes cannot be proven by
  package-local contract tests.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                         | Notes / Expected Output                                                                                                                                                                                                                                  |
| --- | ------ | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Record retention ADR         | Add the repository-wide ADR and reconcile it with #999's API-v2 deferral, legacy compatibility, availability, update ordering, and API-v2 removal.                                                                                                       |
| T2  | TODO   | Separate counter views       | Implement legacy, in-session, and persisted tracker-core views; retain listener topology and make persisted metric export capability-aware.                                                                                                              |
| T3  | TODO   | Extend the v1 stats contract | Add the three fields with backward-compatible deserialization; inject validated persistence capability and map all values through the REST adapter.                                                                                                      |
| T4  | TODO   | Prove retention regressions  | Make the test harness support no persistence; prove the reset, restoration, disabled metric omission, and enabled zero-value cases.                                                                                                                      |
| T5  | TODO   | Review and extend API tests  | Review the existing `GET /api/v1/stats` contract test and add direct `GET /api/v1/metrics` endpoint coverage. Add focused `tests/` integration coverage if package-local tests cannot prove configuration, restart, and exported REST behavior together. |
| T6  | TODO   | Verify public contract       | Run focused tracker-core, REST contract, and any new application integration tests; inspect legacy/new REST fields and legacy/new metrics for enabled and disabled persistence.                                                                          |
| T7  | TODO   | Record local manual evidence | Run the tracker locally for M1-M3 and record exact commands, HTTP requests, redacted responses, configuration, and outcome in `manual-verification.md`.                                                                                                  |

## Progress Tracking

### Workflow Checkpoints

- [x] Deferred investigation recorded while implementing #2107.
- [x] #2107 completed and the resulting listener topology reviewed.
- [x] Spec drafted in `docs/issues/drafts/`.
- [x] Spec reviewed and approved by user/maintainer.
- [x] GitHub issue #2122 created and issue number added to this spec.
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation.
- [ ] Implementation completed.
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks).
- [ ] Manual verification scenarios executed and recorded (status + evidence).
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-08-28 00:00 UTC - GitHub Copilot - Recorded the counter-semantics defect independently from #2107.
- 2026-08-28 00:00 UTC - GitHub Copilot - Confirmed through public API/export tracing and commit `b0e74439` that the counter is session-scoped without persistence and historical with persistence. The defect is inaccurate public naming/documentation, not retention behavior.
- 2026-08-28 00:00 UTC - GitHub Copilot - Confirmed that tracker usage statistics needs the in-memory counter and selected independent in-memory and persistence listeners. #2107 delivered that topology.
- 2026-08-31 16:13 UTC - GitHub Copilot - Reconciled the investigation with merged #2107 and converted it into a formal bug draft. The explicit metric-identifier compatibility decision precedes implementation.
- 2026-08-31 16:20 UTC - GitHub Copilot/User - Added mandatory local manual verification. The folder-style specification owns `manual-verification.md`, which records commands, requests, redacted responses, configuration, and outcome for M1-M3.
- 2026-08-31 16:45 UTC - GitHub Copilot/User - Reconciled the approved additive v1 bridge with the current counter, REST-adapter, metrics-export, and test-harness boundaries. The final draft defines distinct views and capability behavior, and records the required ADR refinement of #999.
- 2026-08-31 16:48 UTC - GitHub Copilot/User - Created GitHub issue #2122 with the `bug` label and promoted this folder-style specification into `docs/issues/open/`.
- 2026-08-31 16:49 UTC - GitHub Copilot/User - Located the REST API test boundary. The implementation plan requires review of the existing stats endpoint contract coverage and direct metrics endpoint coverage, with a focused `tests/` integration test when package-local coverage cannot prove the configuration-to-endpoint contract.
- 2026-08-31 17:13 UTC - GitHub Copilot/User - Opened spec-only PR #2123 for this specification and #2121. It is related to, not an implementation that closes, either issue.
- 2026-09-01 10:30 UTC - GitHub Copilot/User - Confirmed that PR #2123 merged
  into `develop`. Manual verification will use SQLite because the required
  behavior is database-driver independent. Repository ADR guidance is available
  in the `create-adr` skill and `docs/adrs/`; ADR filenames use a UTC timestamp
  and descriptive snake-case title. GitHub entity status is resolved directly
  through repository tooling rather than by asking the user.

## Acceptance Criteria

- [ ] AC1: The legacy `completed` field and legacy metric identifier retain their conditional value semantics, have accurate descriptions, and are documented as deprecated migration paths to explicit views.
- [ ] AC2: `completed_in_session` resets to zero for every tracker process and increments with every in-memory completed-download event.
- [ ] AC3: With persistent completed statistics enabled, `completed_persisted` is seeded from the database aggregate and advances only after successful database persistence; `completed_persisted_enabled` is true.
- [ ] AC4: With persistent completed statistics disabled, `completed_persisted` is zero, `completed_persisted_enabled` is false, and clients can distinguish this from an enabled zero count only through the boolean.
- [ ] AC5: The in-session metric has the tracker-usage-statistics availability contract; the persisted metric is exported only when persistent completed statistics are enabled; the legacy metric remains exported with legacy semantics.
- [ ] AC6: The REST composition root supplies persistence capability from validated configuration, and the v1 protocol remains backward-compatible for clients deserializing older payloads.
- [ ] AC7: REST server contract tests cover the additive `GET /api/v1/stats` fields and direct `GET /api/v1/metrics` behavior for both persistence modes.
- [ ] AC8: Focused tests prove a persistence-free restart reset, persistence-enabled restoration, enabled zero-value behavior, and persisted-metric omission when disabled without changing #2107's listener topology. Add a `tests/` application integration test when package-local tests cannot prove the configuration-to-endpoint contract.
- [ ] AC9: A repository-wide ADR records the names, lifecycle, compatibility/deprecation policy, and API-v2 migration; it explicitly refines #999's session-versus-historical deferral.
- [ ] AC10: `linter all` exits with code `0`, relevant tests pass, manual verification is documented, and acceptance criteria are re-reviewed against actual behavior.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- Focused tracker-core tests for independent legacy, in-session, and persisted values and metric descriptions.
- Focused tracker-core integration tests for no-persistence reset, persistence-enabled restoration, and an enabled persisted zero count across a simulated restart.
- Focused export tests proving the persisted metric is absent when disabled and present when enabled.
- Focused REST protocol/runtime-adapter tests for the additive v1 fields, configuration-derived availability, and backward-compatible deserialization.
- Review and extend `packages/axum-rest-api-server/tests/server/v1/contract/context/stats.rs` for the authenticated `GET /api/v1/stats` endpoint contract, and add direct authenticated `GET /api/v1/metrics` coverage for JSON and Prometheus output as applicable.
- Add a focused integration test under `tests/` using `TrackerApplicationFixture` when the package-local server environment cannot prove the configuration-to-endpoint behavior across persistence modes and restart.
- `cargo fmt`, `linter all`, relevant package tests, and pre-push checks when applicable.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Command/Steps                                                                                                                                         | Expected Result                                                                                                                          | Status | Evidence                    |
| --- | ---------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------- |
| M1  | Inspect disabled persistence | Start documented v3 no-persistence configuration, complete a download, inspect stats/metrics, then restart.                                           | `completed_in_session` resets on restart; `completed_persisted` is zero with its boolean false; no persisted metric is exported.         | TODO   | `manual-verification.md` M1 |
| M2  | Inspect enabled persistence  | Start configured SQLite v3 tracker with persistent completed statistics, complete a download, restart using the same database, and inspect endpoints. | The persisted value survives restart with its boolean true; the persisted metric is exported, including when its observed value is zero. | TODO   | `manual-verification.md` M2 |
| M3  | Verify legacy migration      | Inspect `GET /api/v1/stats` and `GET /api/v1/metrics` in both modes.                                                                                  | Legacy and new names, descriptions, values, and availability match the ADR; legacy values remain compatible.                             | TODO   | `manual-verification.md` M3 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record exact local commands, HTTP requests, redacted response bodies, HTTP
  statuses, configuration, and outcome for M1-M3 in
  `manual-verification.md`; do not record tokens, credentials, or other
  secrets.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |
| AC7   | TODO                   | {test/log/PR link} |
| AC8   | TODO                   | {test/log/PR link} |
| AC9   | TODO                   | {test/log/PR link} |
| AC10  | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Metric compatibility:** Existing dashboards may treat the legacy metric as historical. Mitigation: retain its identifier/value semantics, correct its description, and publish a documented migration window before API v2 removal.
- **Cross-view consistency:** Events and database writes are asynchronous. Mitigation: define the persisted-view update after successful database persistence and test eventual values with bounded waits.
- **Disabled-state ambiguity:** A numeric zero can mean no completed downloads or unavailable persistence. Mitigation: REST uses the explicit boolean and Prometheus omits the persisted metric when disabled.
- **REST compatibility:** New required DTO fields can break deserializers of stored or fixture JSON. Mitigation: preserve v1 deserialization compatibility with defaults and test both payload shapes.
- **Test isolation:** Existing tracker-core integration fixtures assume persistence. Mitigation: make their persistence setup conditional before adding no-persistence restart coverage.
- **Endpoint regression:** Repository tests cover the stats endpoint but not the metrics endpoint directly. Mitigation: review that contract suite and require direct metrics coverage plus a top-level integration test when composition-level behavior is not otherwise observable.

## References

- Completed prerequisite: #2107
- Parent EPIC of completed prerequisite: #1978
- Historical behavior: commit `b0e74439` (`fix: [#1543] return always in API the downloads number from tracker-core`)
- Persistence capability ADR: `docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md`
- Earlier API-v2 deferral: `docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md`
- Tracker metric: `packages/tracker-core/src/statistics/mod.rs`
- REST stats adapter: `packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`
- REST stats protocol: `packages/rest-api-protocol/src/v1/context/stats/resources/stats.rs`
