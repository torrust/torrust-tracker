---
doc-type: issue
status: draft
intended-destination: docs/issues/drafts/
github-issue: null
related-issues:
  - 999
  - 1980
  - 2107
last-updated-utc: 2026-08-25 00:00
semantic-links:
  related-artifacts:
    - docs/issues/open/999-1978-optional-database-configuration/solution.md
    - docs/issues/open/999-1978-optional-database-configuration/adr-draft.md
    - docs/issues/open/999-1978-optional-database-configuration/analysis.md
---

# Draft follow-up - Activate the v3 persistence-free runtime

> **Superseded planning draft:** This draft was refined, approved, and created
> as GitHub issue #2107. Its implementation owns the persistence-free runtime
> and disabled-capability REST behavior. Retain this file as the pre-issue
> planning record; use #2107's issue-local documents for current requirements
> and evidence.

## Goal

Replace the temporary bootstrap `Some(Database)` compatibility bridge with the
actual v3 `core.database: Option<Database>` value. A v3 tracker with no enabled
persistence-backed capability and no `[core.database]` must run without a
persistence driver, database file, network database connection, migration, or
persistence-backed service.

## Background

Issue #999 makes the v3 database representation and container dependencies
optional, but leaves an explicit temporary database dependency in bootstrap
while the application still transitions from v2 aliases to v3 consumers. Issue
1980 activates v3 consumers with that bridge in place. This follow-up changes
the runtime behavior without changing the public v3 configuration shape.

## Scope

### In Scope

- Use actual `v3_0_0::Core.database` at the bootstrap/container boundary.
- Invoke the bootstrap-owned capability-to-persistence requirement check
  implemented and unit-tested by Issue #999 before application-container
  construction.
- Reject enabled listing, private mode, or persistent completed metrics when no
  database is configured.
- Construct no persistence driver, stores, or migrations when persistence is
  absent and no capability requires it.
- Keep `http_api` available without persistence; direct disabled private-key
  and whitelist routes return the approved HTTP 409 configuration-disabled
  response. Historical metric semantics remain deferred to GitHub issue #144.
- Update the container entrypoint so no-persistence v3 deployments do not
  require a driver override, database directory, or packaged SQLite install.
- Defer persistence selection to actual v3 configuration; do not retain a
  separate entrypoint driver default or override that can contradict it.
- Preserve operator-managed database state across configuration transitions:
  never delete, overwrite, or migrate an unselected database target; do not
  automatically transfer state between database drivers or locations.
- Execute and record Issue #999 manual scenarios M1–M6.

### Out of Scope

- Changing v2 behavior.
- Redesigning the complete REST API beyond the minimum persistence-free
  contract.
- Creating feature-specific schemas or migration streams.
- The broader persistence-awareness work captured by
  `persistence-awareness-epic-draft.md`.

## Implementation Plan

| ID  | Status | Task                                      | Notes                                                                                                                       |
| --- | ------ | ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Remove temporary bootstrap bridge         | Pass actual v3 `Option<Database>` to optional composition.                                                                  |
| T2  | TODO   | Invoke bootstrap requirement validation   | Reuse the #999 implementation; do not duplicate its feature matrix.                                                         |
| T3  | TODO   | Gate persistence composition              | No driver/stores/migrations in persistence-free mode.                                                                       |
| T4  | TODO   | Preserve REST API persistence requirement | Do not attempt the #144 response-model redesign in this activation follow-up.                                               |
| T6  | TODO   | Adapt container entrypoint                | Defer persistence to v3 config; permit no-persistence deployment without SQLite setup or destructive mounted-state changes. |
| T7  | TODO   | Add regression coverage                   | Configuration, bootstrap, container, E2E, and restart-transition coverage.                                                  |
| T8  | TODO   | Run M1–M6 and update docs                 | Record evidence in Issue #999 artifacts.                                                                                    |

## Evidence ownership and sequence

| Stage                           | Owner                    | Required evidence                                                                                                             | Follow-up handoff                                                                   |
| ------------------------------- | ------------------------ | ----------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------- |
| V3 optional representation      | Issue #999 Phase 3       | V3 parsing tests prove omitted `[core.database]` is `None`; configured drivers retain their behavior.                         | Preserve the temporary bridge and document that runtime persistence remains active. |
| Optional dependency composition | Issue #999 Phase 3       | Container/constructor tests prove optional persistence dependencies are accepted; bootstrap passes explicit `Some(Database)`. | Provide the reusable validation matrix and final ADR.                               |
| V3 consumer activation          | Issue #1980              | Consumer migration activates v3 while retaining the temporary bridge.                                                         | Record that omitted database is not yet honored at runtime.                         |
| Persistence-free runtime        | GitHub issue #2107       | Actual `None` reaches composition; no driver/migration artifacts; M1–M6 and transition/container evidence pass.               | Update Issue #999 acceptance evidence and finalize operational guidance.            |
| Persistence-free REST API       | GitHub issue #2107       | Disabled direct routes use HTTP 409; completed-metric provenance remains deferred to API #144.                                | #144 defines the next-major completed-metric response model.                        |

The exact issue is intentionally not created until the preceding #999/#1980
implementation evidence is reviewed. Before it is opened, reconcile this draft
with the merged code, replace assumptions with verified behavior, and identify
any newly discovered persistence consumer in the centralized matrix.

## Acceptance Criteria

- [ ] Omitted v3 `[core.database]` is honored at runtime when no capability
      requires persistence.
- [ ] No persistence artifacts are created in the persistence-free scenario.
- [ ] Each enabled persistence-backed capability fails startup clearly when the
      database is absent.
- [ ] The activation follow-up documents that `http_api` remains
      persistence-required until GitHub issue #144 delivers the approved
      next-major REST API contract.
- [ ] The supported container path works without persistence configuration.
- [ ] Disabling persistence on restart leaves the previously selected database
      target unchanged; re-enabling the same target reuses its data and
      migrations; changing targets does not copy data automatically.
- [ ] Issue #999 M1–M6 evidence is completed.

## References

- Issue #999
- Issue #1980
- `docs/issues/open/999-1978-optional-database-configuration/solution.md`
- `docs/issues/open/999-1978-optional-database-configuration/adr-draft.md`
