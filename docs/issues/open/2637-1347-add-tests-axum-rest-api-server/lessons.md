---
doc-type: issue-lessons
issue: 2637
package: torrust-tracker-axum-rest-api-server
---

# Axum REST API Server Testing Lessons

Candidate lessons and follow-ups collected while working on Issue #2637. Promoting any of them into
`docs/testing/`, `.github/skills/`, or a production change is a separate decision after this issue, per the
[subissue guidelines](../1347-overhaul-packages-testing/subissue-spec-guidelines.md).

## Follow-Up Candidates

| Source | Candidate | Evidence |
| --- | --- | --- |
| [Auth middleware plan](test-refactor-plans/v1-middlewares-auth-tests.md) | Decide whether a non-UTF-8 `Authorization` header should be rejected like another scheme or documented as ignored. Fixed as #2653 (merged through #2656; spec in `docs/issues/closed/2653-rest-api-auth-ignores-unreadable-authorization-header/`): the header is rejected as `unknown token provided`. | A throwaway test (reverted) showed `Bearer \xff` with a valid `token` query answers `200`, while `Basic <token>` answers `500 unknown token provided`. The #2653 test pins the fixed behavior; this plan keeps it in its table. |
| `lib.rs` crate documentation | Correct `[http_api.tsl_config]` to `[http_api.tls_config]` in the three places of the crate documentation (found in the review of #2638, finding F1). | The v3 `HttpApi` key is `tls_config` and the type denies unknown fields. Planned for the `lib.rs` plan (S6). |
| [Routes plan](test-refactor-plans/v1-routes-tests.md) | Design question: make the state "private or listed set but no persistence" unrepresentable. `TrackerCoreContainer` holds `core_config.private` and `persistence: Option<PersistenceServices>` as independent fields, and `v1/routes.rs` re-derives service availability from them, which ADR `20260825193119` (agreement 2) says route handlers should not duplicate. Composing the capabilities once (for example `Option` handles built where the database check happens) would remove the combination. It touches `tracker-core` and `rest-api-runtime-adapter`, so it is outside #2637. Maintainer decision 2026-10-28: record here only for now. | The first line of defense is `validate_persistence_requirements` in `src/bootstrap/persistence.rs`, called by `setup` in `src/bootstrap/app.rs`; the second is `v1/routes.rs`, protected by the plan's no-persistence cases. |
| `src/bootstrap/persistence.rs` module comment | Fix the stale comment: it says the check is "intentionally not called" while the runtime uses v2 configuration, but `setup` in `src/bootstrap/app.rs` calls it. Not a behavior bug and outside the package under test. | `grep -n validate_persistence_requirements src/bootstrap/app.rs` finds the call. |

## Testing Lessons

- **Incidental coverage through test support moves the baseline.** #2449 added unit tests of `testing/environment.rs` that run the real router, which lifted the unit-only figures of files they never test directly (`routes.rs` 46% to 87%). Re-measure at every slice start, and judge a slice by the production lines and behavior contracts of its own files.
