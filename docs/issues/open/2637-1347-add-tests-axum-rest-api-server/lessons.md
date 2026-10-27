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

## Testing Lessons

- **Incidental coverage through test support moves the baseline.** #2449 added unit tests of `testing/environment.rs` that run the real router, which lifted the unit-only figures of files they never test directly (`routes.rs` 46% to 87%). Re-measure at every slice start, and judge a slice by the production lines and behavior contracts of its own files.
