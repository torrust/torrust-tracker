---
doc-type: spec
issue-type: task
status: done
priority: p2
epic: 1938
github-issue: 1969
spec-path: docs/issues/closed/1969-1938-si-8-eliminate-unwraps-from-rest-api-client.md
last-updated-utc: 2026-07-15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/1938-rest-api-contract-first-migration/EPIC.md
    - packages/rest-api-client/
    - packages/rest-api-client/src/v1/client.rs
---

<!-- skill-link: create-issue -->

# SI-8: Eliminate all unwraps from the REST API client package

## Subissue of REST API Contract-First Migration EPIC

## Goal

Eliminate all `.unwrap()` calls from the `torrust-tracker-rest-api-client` package. Every operation that can fail must return a `Result`. For operations that are provably infallible, replace bare `.unwrap()` with an explicit `.expect("infallible: ...")` that documents why the operation cannot fail.

## Background

The `ApiClient` was made fully panic-free in SI-6 (PR #1968). However, the low-level `ApiHttpClient` and several free functions/helpers in `client.rs` still contain `.unwrap()` and `.expect()` calls that can panic at runtime.

The calls fall into two categories:

### Transport unwraps (must return `Result`)

These are real failure points — network errors, URL parsing failures, etc. They must return `Result`:

1. **11 public `ApiHttpClient` methods** — thin wrappers that delegate to fallible `*_result()` counterparts but `.unwrap()` the result.
2. **`post_empty()`, `post_form()`** (private) — same wrapper-with-unwrap pattern.
3. **`get()` (pub method on `ApiHttpClient`)** — same pattern.
4. **`get()` (pub free function)** — thin wrapper around `get_result()`.
5. **`get_request()` (pub on `ApiHttpClient`)** — calls `base_url()` which already returns `Result`.

### Infallible conversions (replace `unwrap` with `expect`)

These are provably infallible operations where a descriptive `expect` message is the right pattern:

1. **`headers_with_request_id()`** — `Uuid::to_string()` always produces a valid ASCII string, and `HeaderValue::from_str()` for ASCII strings never fails.
2. **`headers_with_auth_token()`** — same pattern, pre-formatted token string.
3. **`get_request_with_query_result()` auth token inserts** — 2 token-to-HeaderValue conversions, same provably-infallible pattern.

## Scope

### In Scope

- Change all panicking public `ApiHttpClient` methods to return `Result<Response, ClientError>` instead of `Response`.
- Update all caller sites across the repository (contract tests, E2E tests, integration tests) to handle the new `Result` return types.
- Change helper functions (`post_empty`, `post_form`, `get`, `get_request`, `get()`) to return `Result`.
- Replace bare `.unwrap()` with `.expect("infallible: ...")` in `headers_with_request_id()`, `headers_with_auth_token()`, and `get_request_with_query_result()` auth token inserts.
- Update issue spec and documentation.

### Out of Scope

- Changing the `ApiHttpClient`'s HTTP transport or connection model.
- Adding retry/timeout policy (tracked separately).
- Removing the `ApiClient`/`ApiHttpClient` two-tier architecture.

## Implementation Plan

| ID  | Status | Task                                                                                           | Notes                                                                                                                                                              |
| --- | ------ | ---------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Move `ApiHttpClient` public methods to return `Result`                                         | 10 methods + `get()` method + `get_request()` + `get_request_with_query()` — return `ClientError`                                                                  |
| T2  | TODO   | Update all callers in contract tests (`packages/axum-rest-api-server/tests/`)                  | ~65 `ApiHttpClient::new(...)` call sites. First iteration: callers `.unwrap()` the `Result`. Prefer `.expect("...")` over bare `.unwrap()` in tests for precision. |
| T3  | TODO   | Update callers in `src/console/ci/qbittorrent_e2e/tracker/client.rs`                           | E2E test runner wrapper. Production code — propagate errors properly with `?` / `Context`.                                                                         |
| T4  | TODO   | Update callers in `tests/servers/api/contract/stats/mod.rs`                                    | Integration test. Use `.unwrap()` or `.expect()` since it's test code.                                                                                             |
| T5  | TODO   | Replace bare `.unwrap()` with `.expect("infallible: ...")` for provably infallible conversions | `headers_with_request_id`, `headers_with_auth_token`, auth token inserts                                                                                           |
| T6  | TODO   | Verify pre-commit and pre-push checks pass                                                     |                                                                                                                                                                    |

## Design Decisions

### Caller handling strategy (two-phase)

Per discussion with the issue author (2026-07-13):

- **Phase 1 (this PR)**: Change all `ApiHttpClient` public methods to return `Result<Response, ClientError>`. Update all callers to compile — test callers use `.unwrap()` / `.expect()`, production callers propagate errors properly.
- **Phase 2 (follow-up)**: Evaluate each caller site and decide whether to keep `.unwrap()` (acceptable in tests), switch to `.expect("...")` (preferred in tests), or propagate with `?` (required in production code).

### All public functions must return `Result`

Per discussion with the issue author (2026-07-13):

- `get_request(&self, path: &str)` — changed to return `Result<Response, ClientError>` (was panicking via `base_url().unwrap()`)
- `get_request_with_query(&self, path, params, headers)` — changed to return `Result<Response, ClientError>` (was panicking via `.unwrap()` on the `_result` counterpart)
- Free function `get(path, query, headers)` — changed to return `Result<Response, ClientError>` (was panicking via `.unwrap()` on `get_result`)
- All other public `ApiHttpClient` methods — changed to return `Result<Response, ClientError>`

## Verification / Progress

- [x] All `ApiHttpClient` public methods return `Result<Response, ClientError>`
- [x] No bare `.unwrap()` calls remain (only `.expect("infallible: ...")` for provably infallible operations)
- [x] All contract tests pass unchanged (except for updated `.unwrap()` calls on test side)
- [x] E2E tests compile
- [x] Pre-commit checks pass
- [x] Pre-push checks pass

### Progress Log

| Date       | Event                                      |
| ---------- | ------------------------------------------ |
| 2026-07-13 | Draft spec created                         |
| 2026-07-13 | PR #1973 merged - Implementation completed |
| 2026-07-15 | Spec archived to `docs/issues/closed/`     |

## Acceptance Criteria

- `ApiHttpClient` never panics on transport/URL failures; all errors are returned as `ClientError`
- Provably infallible conversions use `.expect("infallible: ...")` with a clear rationale
- No regressions in existing tests
- `linter all` passes

### Progress Log

| Date       | Event              |
| ---------- | ------------------ |
| 2026-06-30 | Spec drafted       |
| 2026-06-30 | Spec moved to open |
