---
doc-type: spec
issue-type: task
status: done
priority: p2
epic: 1938
github-issue: 1944
spec-path: docs/issues/closed/1944-1938-si-6-align-rest-api-client.md
last-updated-utc: 2026-07-15
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/closed/1938-rest-api-contract-first-migration/EPIC.md
    - packages/rest-api-client/
    - packages/rest-api-protocol/
    - packages/rest-api-client/src/v1/client.rs
    - packages/rest-api-client/Cargo.toml
---


# SI-6: Introduce `ApiClient` — a high-level REST API client over protocol DTOs

## Subissue of REST API Contract-First Migration EPIC

## Clarifying Decisions (from AI agent Q&A with user)

- **`AddKeyForm`**: Use the protocol package's `AddKeyForm` (with field `opt_seconds_valid`) and remove the local `AddKeyForm` from client.
- **`ClientError` enum variants**:
  - `TransportError(reqwest::Error)` — network/connection failures
  - `ApiError { status: StatusCode, body: String }` — non-2xx responses with the error body
  - `DeserializationError(reqwest::Error)` — JSON parsing failures
- **Public `get()` function**: Keep as a public free function (used by health_check tests directly).
- **Re-export strategy**: Re-export both `ApiClient` and `ApiHttpClient` from the crate root for ergonomics.

## Problem

The REST API client package (`torrust-tracker-rest-api-client`) currently exposes only a **low-level** `Client` struct where all 10 methods return raw `reqwest::Response` values. Callers must manually deserialize responses and handle errors. Some internal methods use `.unwrap()`, panicking on transport errors.

Per the contract-first architecture defined in SI-33, consumers should be able to work with typed DTOs from `rest-api-protocol` directly, without manual response parsing. The package needs a separate **high-level client** that wraps the low-level HTTP transport and provides a type-safe, ergonomic API.

## Current State

The current `Client` struct in `src/v1/client.rs` is used as an HTTP transport for the REST API. It connects to a tracker instance and provides methods for all endpoints, but returns raw `reqwest::Response`.

### Current API

**Low-level client methods** (current `Client`, to be renamed to `ApiHttpClient`):

| Method                                     | Currently returns | Notes                                |
| ------------------------------------------ | ----------------- | ------------------------------------ |
| `get_torrent(info_hash)`                   | `Response`        | raw reqwest response                 |
| `get_torrents(params)`                     | `Response`        | raw reqwest response                 |
| `get_tracker_statistics()`                 | `Response`        | raw reqwest response                 |
| `generate_auth_key(seconds_valid)`         | `Response`        | raw reqwest response                 |
| `add_auth_key(add_key_form)`               | `Response`        | raw reqwest response                 |
| `delete_auth_key(key)`                     | `Response`        | panics on send failure (`.unwrap()`) |
| `reload_keys()`                            | `Response`        | raw reqwest response                 |
| `whitelist_a_torrent(info_hash)`           | `Response`        | raw reqwest response                 |
| `remove_torrent_from_whitelist(info_hash)` | `Response`        | panics on send failure (`.unwrap()`) |
| `reload_whitelist()`                       | `Response`        | raw reqwest response                 |

**Current limitations of the low-level API**:

- Returns raw `reqwest::Response` — callers parse the body manually.
- Some methods (`post_empty`, `post_form`, `delete`) `.unwrap()` internally, panicking on transport errors.
- No `ClientError` enum for unified error handling.
- No dependency on `rest-api-protocol`.

### Existing Consumers Already Building Their Own Wrappers

The need for a high-level typed client is validated by two existing adoptions:

**1. E2E test runner** — `src/console/ci/qbittorrent_e2e/tracker/client.rs`

The `TrackerApiClient` struct wraps the low-level `Client` (eventually `ApiHttpClient`) and provides a typed `get_torrent()` returning `anyhow::Result<Torrent>`. Only the one method needed for E2E scenarios is wrapped.

```rust
pub(crate) struct TrackerApiClient {
    inner: Client,  // the low-level HTTP client
}

impl TrackerApiClient {
    pub(crate) async fn get_torrent(&self, hash: &InfoHash) -> anyhow::Result<Torrent> {
        let response = self.inner.get_torrent(hash.as_str(), None).await;
        if !response.status().is_success() {
            return Err(anyhow::anyhow!(...));
        }
        response.json::<Torrent>().await.with_context(...)
    }
}
```

**2. Torrust Index** — [`src/tracker/api.rs`](https://raw.githubusercontent.com/torrust/torrust-index/refs/heads/develop/src/tracker/api.rs)

The Index project built a separate tracker API client from scratch (effectively a copy of the low-level patterns) containing only the methods it needs. This duplication exists because the official `rest-api-client` didn't provide a typed high-level client.

**Implication**: SI-6 eliminates this duplication. Once `ApiClient` is published, the Index can import it instead of maintaining its own copy, and the E2E test runner can switch to the official high-level client.

## Decision

Introduce a two-tier client architecture. Both structs live in the same file `packages/rest-api-client/src/v1/client.rs`:

### Naming

- **`ApiHttpClient`** (renamed from `Client`) — the low-level HTTP transport. Handles connection info, URL building, auth headers, and raw HTTP requests. Returns `reqwest::Response`.
- **`ApiClient`** (new) — the high-level typed client. Wraps `ApiHttpClient`. Returns `Result<DtoType, ClientError>`. Never panics.

The `ApiClient` is placed **before** `ApiHttpClient` in the file so new readers encounter the primary API first.

### Responsibilities

| Concern              | `ApiHttpClient`                        | `ApiClient`                              |
| -------------------- | -------------------------------------- | ---------------------------------------- |
| HTTP transport       | ✅ Owns `reqwest::Client`              | ❌ Delegates to inner                    |
| URL building         | ✅ Constructs endpoint URLs            | ❌                                       |
| Auth headers         | ✅ Manages API token                   | ❌                                       |
| Raw HTTP methods     | ✅ GET, POST, DELETE                   | ❌                                       |
| Type deserialization | ❌                                     | ✅ Parses `Response` into DTOs           |
| Status code checking | ❌                                     | ✅ Maps non-2xx to `ClientError`         |
| Error types          | ❌ Uses `Result` only for construction | ✅ `ClientError` enum                    |
| Panics               | ✅ Can panic on transport errors       | ❌ Never panics — all errors in `Result` |

### Architecture

```text
ApiClient (high-level, typed)
    │
    │ uses
    ▼
ApiHttpClient (low-level, HTTP transport)  ───► reqwest
    │
    ▼
rest-api-protocol (DTOs used by ApiClient)
```

### Example pattern

```rust
// client.rs — both structs in the same file

/// Low-level HTTP transport for the Torrust Tracker REST API.
pub struct ApiHttpClient { ... }

impl ApiHttpClient {
    pub async fn get_torrent(&self, info_hash: &str) -> Response { ... }
}

/// High-level typed client wrapping [`ApiHttpClient`].
///
/// Returns protocol DTOs from `rest-api-protocol` and never panics.
pub struct ApiClient { ... }

impl ApiClient {
    pub async fn get_torrent(&self, info_hash: &InfoHash) -> Result<Torrent, ClientError> {
        let response = self.inner.get_torrent(info_hash).await;
        if !response.status().is_success() {
            return Err(ClientError::ApiError(response.status(), ...));
        }
        response.json::<Torrent>().await.map_err(ClientError::from)
    }
}
```

## Scope

### In Scope

- Rename existing `Client` → `ApiHttpClient` (mechanical rename, covered by compiler).
- Introduce `ApiClient` struct that wraps `ApiHttpClient`.
- Add `rest-api-protocol` as a dependency of `rest-api-client`.
- Define `ClientError` enum covering: transport errors, deserialization errors, API error responses (non-2xx status codes).
- Implement typed methods on `ApiClient` for all endpoints, returning protocol DTOs.
- Add `ApiClient` before `ApiHttpClient` in `client.rs`.

### Out of Scope

- Migrating existing consumers (`tracker_client`, E2E runner, etc.) from `ApiHttpClient` to `ApiClient` — progressive, not required.
- Changing `ApiHttpClient`'s HTTP transport or connection model.
- Adding retry/timeout policy (tracked separately).
- Removing the low-level `ApiHttpClient` methods.

## Implementation Plan

| ID  | Status | Task                                                         | Notes                                            |
| --- | ------ | ------------------------------------------------------------ | ------------------------------------------------ |
| T1  | DONE   | Rename `Client` → `ApiHttpClient` in `client.rs`             | Compiler catches all references                  |
| T2  | DONE   | Add `rest-api-protocol` to `rest-api-client/Cargo.toml`      |                                                  |
| T3  | DONE   | Define `ClientError` enum                                    | Wraps reqwest error, deserialization, API errors |
| T4  | DONE   | Add `ApiClient` struct before `ApiHttpClient` in `client.rs` | New high-level typed client                      |
| T5  | DONE   | Implement typed methods on `ApiClient` for all endpoints     | Returns `Result<DtoType, ClientError>`           |
| T6  | DONE   | Verify pre-commit and pre-push checks pass                   |                                                  |

## Verification / Progress

- [x] `Client` renamed to `ApiHttpClient` across the codebase
- [x] `rest-api-protocol` added as dependency
- [x] `ClientError` enum defined
- [x] `ApiClient` struct with typed methods for all endpoints added
- [x] `ApiClient` appears before `ApiHttpClient` in `client.rs`
- [x] All existing tests pass unchanged
- [x] Pre-commit checks pass
- [x] Pre-push checks pass

### Progress Log

| Date       | Event                                      |
| ---------- | ------------------------------------------ |
| 2026-06-24 | Draft spec created                         |
| 2026-06-30 | PR #1968 merged - Implementation completed |
| 2026-07-15 | Spec archived to `docs/issues/closed/`     |
