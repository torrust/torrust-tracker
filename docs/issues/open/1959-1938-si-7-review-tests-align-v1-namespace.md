---
doc-type: spec
issue-type: task
status: planned
priority: p3
epic: 1938
github-issue: 1959
spec-path: docs/issues/open/1959-1938-si-7-review-tests-align-v1-namespace.md
last-updated-utc: 2026-06-29
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1938-rest-api-contract-first-migration/EPIC.md
    - packages/axum-rest-api-server/src/v1/context/torrent/resources/torrent.rs
    - packages/axum-rest-api-server/src/v1/context/torrent/resources/peer.rs
    - packages/rest-api-runtime-adapter/src/conversion.rs
    - packages/rest-api-application/src/
    - packages/rest-api-runtime-adapter/src/
    - packages/rest-api-protocol/src/
    - packages/axum-rest-api-server/src/
    - packages/rest-api-client/src/
---

<!-- skill-link: create-issue -->

# SI-7: Review tests and align v1 namespace across REST API packages

## Subissue of REST API Contract-First Migration EPIC

## Problem

During the contract-first migration (SI-1 through SI-5), production code was moved from `axum-rest-api-server` to the new layered packages (`rest-api-protocol`, `rest-api-application`, `rest-api-runtime-adapter`). However, some unit tests were left behind in the wrong package, and the `v1` namespace is not consistently applied across all packages.

### Issue 1: Tests in wrong packages

The file `packages/axum-rest-api-server/src/v1/context/torrent/resources/torrent.rs` contains two unit tests that test functions defined in `rest-api-runtime-adapter::conversion`:

- `torrent_resource_should_be_converted_from_torrent_info()` — tests `conversion::from_domain_info()`
- `torrent_resource_list_item_should_be_converted_from_the_basic_torrent_info()` — tests `conversion::list_item_from_domain()`

These tests should live alongside the production code they test, in `rest-api-runtime-adapter`.

Additionally, `packages/axum-rest-api-server/src/v1/context/torrent/resources/peer.rs` is a stub file containing only a doc comment saying _"Protocol DTOs are defined in `rest-api-protocol`."_ — it has no production code and should be removed.

A review of the whole `axum-rest-api-server` package is needed to identify all such cases.

### Issue 2: Inconsistent v1 namespace

The API packages use the `v1` module inconsistently:

| Package                    | Has `v1` module?   | Notes                                        |
| -------------------------- | ------------------ | -------------------------------------------- |
| `rest-api-protocol`        | ✅ `src/v1/mod.rs` | Canonical home for v1 DTOs                   |
| `axum-rest-api-server`     | ✅ `src/v1/`       | Axum handlers, routes, responses             |
| `rest-api-client`          | ✅ `src/v1/`       | Client for v1 endpoints                      |
| `rest-api-application`     | ❌ No `v1`         | Ports and use-cases at top level             |
| `rest-api-runtime-adapter` | ❌ No `v1`         | Adapters, container, conversion at top level |

For `rest-api-application` and `rest-api-runtime-adapter`, the content is specific to the v1 API contract. Adding a `v1` module would align them with the other packages and make the version boundary explicit.

## Scope

### In Scope

#### Part A: Move misplaced tests

- Move the two conversion tests from `axum-rest-api-server/src/v1/context/torrent/resources/torrent.rs` to `rest-api-runtime-adapter/src/conversion.rs` (or a new `tests/` module in that package).
- Remove the empty stub file `axum-rest-api-server/src/v1/context/torrent/resources/peer.rs` and its module declaration.
- Review the entire `axum-rest-api-server` package for any other tests that test code from other packages.

#### Part B: Align v1 namespace

- Add `src/v1/` module to `rest-api-application` and move `ports/` and `use_cases/` under it.
- Add `src/v1/` module to `rest-api-runtime-adapter` and move `adapters/`, `container.rs`, `conversion.rs` under it.
- Update all internal imports across the workspace to use the new paths.
- Update `lib.rs` in both packages to re-export from `v1`.

### Out of Scope

- Changing test logic or adding new tests — only moving existing tests.
- Changing the Axum server test infrastructure or integration tests.
- Creating the SI-6 `ApiClient` implementation.

## Implementation Plan

### Part A: Move misplaced tests

| ID  | Status | Task                                                                                        | Notes                                                        |
| --- | ------ | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------ |
| A1  | TODO   | Move conversion tests from `axum-rest-api-server` to `rest-api-runtime-adapter::conversion` | Tests for `from_domain_info()` and `list_item_from_domain()` |
| A2  | TODO   | Remove empty `axum-rest-api-server/src/v1/context/torrent/resources/peer.rs` stub           | Only doc comment, no code                                    |
| A3  | TODO   | Clean up module declarations after removing peer.rs                                         | Remove `pub mod peer;` from `resources/mod.rs`               |
| A4  | TODO   | Review the whole `axum-rest-api-server/` package for similar misplaced tests                | Check all context handlers, responses, routes                |

### Part B: Align v1 namespace

| ID  | Status | Task                                                                                                       | Notes                                                               |
| --- | ------ | ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| B1  | TODO   | Add `v1/` module to `rest-api-application`, move `ports/` and `use_cases/` under it                        | Update `lib.rs`                                                     |
| B2  | TODO   | Add `v1/` module to `rest-api-runtime-adapter`, move `adapters/`, `container.rs`, `conversion.rs` under it | Update `lib.rs`                                                     |
| B3  | TODO   | Update internal imports across workspace                                                                   | For `rest-api-application` and `rest-api-runtime-adapter` consumers |
| B4  | TODO   | Verify workspace builds cleanly                                                                            | `cargo build`                                                       |
| B5  | TODO   | Pre-commit and pre-push checks pass                                                                        |                                                                     |

## Verification / Progress

- [ ] A1: Conversion tests moved to `rest-api-runtime-adapter`
- [ ] A2: Empty `peer.rs` stub removed
- [ ] A3: Module declarations cleaned up
- [ ] A4: No other misplaced tests found in `axum-rest-api-server`
- [ ] B1: `rest-api-application` has `v1/` module with ports + use-cases
- [ ] B2: `rest-api-runtime-adapter` has `v1/` module with adapters + container + conversion
- [ ] B3: All internal imports updated
- [ ] B4: Workspace builds cleanly
- [ ] B5: Pre-commit and pre-push checks pass
