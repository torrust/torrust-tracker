---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 1490
spec-path: docs/issues/open/1490-1978-decompose-database-config-and-overhaul-secrets.md
branch: "1490-secrets-overhaul"
related-pr: null
last-updated-utc: 2026-07-13 21:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
    - packages/configuration/src/v3_0_0/database.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
    - packages/configuration/src/lib.rs
---

<!-- skill-link: create-issue -->

# Issue #1490 - Decompose database config and overhaul secrets with `secrecy` crate

> **EPIC position**: Subissue #7 of 9. Depends on #1640 (subissue #3) — both touch `Core`, so #1640 goes first (removes `core.net`, then #1490 changes `database` type). Can run in parallel with #1415, #1453, #889.

## Goal

Decompose the database configuration into driver-specific variants (SQLite, MySQL, PostgreSQL) and replace the manual secret-masking approach with the [`secrecy`](https://docs.rs/secrecy/) crate. This provides systematic protection for API tokens and database passwords, ensuring they are never accidentally exposed via `Debug`, `Display`, tracing instrumentation, or log output.

## Background

The Torrust Tracker handles these secrets:

- **API tokens** — in `[http_api.access_tokens]` (e.g. `admin = "MyAccessToken"`)
- **Database passwords** — embedded in the database connection URL (e.g. `mysql://db_user:db_user_secret_password@mysql:3306/torrust_tracker`)

Currently, secrets are masked manually via a `mask_secrets()` method that clones the configuration and replaces secret values with `"***"` before logging. This approach has several weaknesses:

1. **Forgetfulness**: Any new secret added to the config must be manually added to `mask_secrets()`. If forgotten, it leaks.
2. **Tracing instrumentation**: As discovered in issue #1441, secrets can leak via tracing instrumentation even when `mask_secrets()` is called.
3. **No compile-time protection**: There is no type-level distinction between a secret and a regular string.

### Proposed solution

Use the [`secrecy`](https://docs.rs/secrecy/) crate, which provides:

- A `Secret<T>` wrapper type that implements `Debug` and `Display` without exposing the inner value
- Automatic zeroing of memory when the secret is dropped (via `zeroize`)
- Clear type-level distinction between secrets and plain strings

### Database connection string

The database configuration currently uses a single `path` string that serves double duty:

```toml
# SQLite: path is a filesystem path
[core.database]
driver = "sqlite3"
path = "/var/lib/torrust/tracker/database/sqlite3.db"

# MySQL/PostgreSQL: path is a URL with embedded password
[core.database]
driver = "mysql"
path = "mysql://db_user:db_user_secret_password@mysql:3306/torrust_tracker"
```

This design has several problems:

1. **Field name lies**: `path` means "filesystem path" for SQLite but "connection URL" for MySQL/PostgreSQL
2. **Password hidden in URL**: The password is embedded in a URL string, making it hard to isolate as a secret
3. **Validation is impossible**: You can't validate a SQLite path and a MySQL URL with the same rules
4. **`mask_secrets()` is fragile**: It parses the URL just to mask the password (see `database.rs` lines 49-62)

This issue decomposes the database config into an enum with driver-specific variants:

```rust
pub struct ConnectionInfo {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: Secret<String>,
    pub database: String,
}

pub enum Database {
    Sqlite3 { path: String },
    MySQL(ConnectionInfo),
    PostgreSQL(ConnectionInfo),
}
```

TOML representation:

```toml
# SQLite
[core.database]
driver = "sqlite3"
path = "/var/lib/torrust/tracker/database/sqlite3.db"

# MySQL
[core.database]
driver = "mysql"
host = "mysql"
port = 3306
user = "db_user"
password = "db_user_secret_password"
database = "torrust_tracker"

# PostgreSQL
[core.database]
driver = "postgresql"
host = "postgres"
port = 5432
user = "postgres"
password = "postgres_secret_password"
database = "torrust_tracker"
```

This is a **breaking change** with no backward-compatibility fallback. Since we are releasing config schema v3.0.0 and tracker v4.0.0, breaking changes are expected and documented.

### Ripple effect

~25 files will need changes (see full analysis in spec review). Key consumers:

| Category           | Files                                                              | Change                  |
| ------------------ | ------------------------------------------------------------------ | ----------------------- |
| Config definition  | `database.rs`, `core.rs`, `mod.rs`                                 | Enum + `Secret<String>` |
| DB setup dispatch  | `tracker-core/src/databases/setup.rs`                              | Match on enum variant   |
| Test helpers       | `test-helpers/`, `tracker-core/src/test_helpers.rs`, `fixtures.rs` | Construct enum variant  |
| Examples           | `http_only_public_tracker.rs`, `udp_only_public_tracker.rs`        | Construct enum variant  |
| Benchmarks         | `persistence-benchmark/` (4 files)                                 | Construct enum variant  |
| E2E config builder | `qbittorrent_e2e/tracker/config_builder.rs`                        | Construct enum variant  |
| Default TOML files | `share/default/config/*.toml` (6 files)                            | New format              |
| Inline TOML/docs   | `mod.rs` tests, `lib.rs` doc comments, integration tests           | New format              |

**AccessTokens `Secret<String>` wrapping (~10 additional files):**

| Category        | Files                                                        | Change                                               |
| --------------- | ------------------------------------------------------------ | ---------------------------------------------------- |
| Type alias      | `tracker_api.rs`                                             | `HashMap<String, Secret<String>>`                    |
| Auth middleware | `axum-rest-api-server/src/v1/middlewares/auth.rs`            | `t.expose_secret() == token`                         |
| Test env        | `axum-rest-api-server/src/testing/environment.rs`            | `.get("admin").map(\|s\| s.expose_secret().clone())` |
| Bootstrap       | `src/bootstrap/jobs/tracker_apis.rs`, `src/bootstrap/app.rs` | Remove `mask_secrets()` call                         |
| Config tests    | `tracker_api.rs`, `mod.rs`                                   | Add `.expose_secret()` in assertions                 |
| `mask_secrets`  | `tracker_api.rs`, `mod.rs`                                   | Remove or adapt (Secret handles display)             |

The `rest-api-client` crate and `tracker-core` authentication are **not affected** — they consume plain `String` tokens extracted from the map.

## Scope

### In Scope

- Add `secrecy` crate as a dependency to `packages/configuration`
- Decompose `Database` struct into an enum: `Sqlite3 { path }`, `MySQL(ConnectionInfo)`, `PostgreSQL(ConnectionInfo)`
- Wrap database password in `Secret<String>` (inside `ConnectionInfo`)
- Wrap API tokens in `Secret<String>` (in `HttpApi` config struct)
- Remove manual `mask_secrets()` methods (replaced by type-level protection)
- Update all ~25 consumers to use the new enum variants and `.expose()` for secret access
- Update default config TOML files (6 files) to the new format
- Update inline TOML in tests and doc comments

### Out of Scope

- Applying `secrecy` to secrets outside the configuration package
- Changing how secrets are stored or transmitted at runtime
- Encrypting secrets at rest in the config file
- Changing the `Driver` enum in `packages/primitives` (may be deprecated after this change)

## Implementation Plan

| ID  | Status | Task                                                            | Notes                                                                     |
| --- | ------ | --------------------------------------------------------------- | ------------------------------------------------------------------------- |
| T1  | TODO   | Add `secrecy` dependency to `packages/configuration/Cargo.toml` | Latest stable version                                                     |
| T2  | TODO   | Define `ConnectionInfo` struct and `Database` enum              | In `packages/configuration/src/v3_0_0/database.rs`                        |
| T3  | TODO   | Implement serde for `Database` enum (internally tagged)         | `driver` field as discriminant; `Sqlite3`, `MySQL`, `PostgreSQL` variants |
| T4  | TODO   | Wrap database password in `Secret<String>`                      | In `ConnectionInfo`; `Sqlite3` variant has no secrets                     |
| T5  | TODO   | Wrap API tokens in `Secret<String>`                             | In `HttpApi` config struct                                                |
| T6  | TODO   | Remove manual `mask_secrets()` methods                          | No longer needed with type-level protection                               |
| T7  | TODO   | Update `tracker-core/src/databases/setup.rs` dispatch           | Match on `Database` enum variant instead of `Driver`                      |
| T8  | TODO   | Update all ~25 consumers (tests, examples, benchmarks, E2E)     | Construct enum variants; use `.expose()` for secrets                      |
| T9  | TODO   | Update default config TOML files (6 files)                      | New per-driver format                                                     |
| T10 | TODO   | Update inline TOML in tests and doc comments                    | `mod.rs` tests, `lib.rs`, integration tests                               |
| T11 | TODO   | Run `linter all` and full test suite                            |                                                                           |

## Progress Tracking

### Workflow Checkpoints

- [ ] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/open/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial spec drafted
- 2026-07-14 00:00 UTC - josecelano - Rewrote spec: decomposed `Database` into enum with `ConnectionInfo`; removed backward-compat fallback; added ripple-effect analysis (~25 files); renamed issue title

## Acceptance Criteria

- [ ] AC1: `Database` is an enum with `Sqlite3`, `MySQL(ConnectionInfo)`, `PostgreSQL(ConnectionInfo)` variants
- [ ] AC2: `secrecy` crate is used for all secret values (`password` in `ConnectionInfo`, API tokens in `HttpApi`)
- [ ] AC3: `Debug` and `Display` on config structs do not expose secret values
- [ ] AC4: Manual `mask_secrets()` methods are removed
- [ ] AC5: All ~25 consumers compile and pass tests with the new enum + `Secret<String>`
- [ ] AC6: Default config TOML files use the new per-driver format
- [ ] AC7: No secrets leak in logs or tracing output
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`

### Manual Verification Scenarios

| ID  | Scenario                             | Command/Steps                                    | Expected Result          | Status | Evidence |
| --- | ------------------------------------ | ------------------------------------------------ | ------------------------ | ------ | -------- |
| M1  | Verify secrets masked in logs        | Run tracker, check startup log for config output | Secrets show as `***`    | TODO   |          |
| M2  | Verify Debug output masks secrets    | `println!("{:?}", config)` in test or debug      | Secrets show as `***`    | TODO   |          |
| M3  | Verify secrets accessible via expose | Write test that reads a secret via `.expose()`   | Returns the actual value | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |
| AC6   | TODO   |          |

## Risks and Trade-offs

- **Breaking change for database config**: The `Database` struct becomes an enum; the old `path` field is removed with no backward-compatibility fallback. Mitigation: this is part of the v3.0.0 config schema bump where breaking changes are expected and documented.
- **Consumer updates (~25 files)**: Every place that constructs or reads a `Database` value needs updating. Mitigation: the compiler will catch all mismatches; changes are mechanical (construct enum variant, use `.expose()` for secrets).
- **Performance**: `secrecy` adds zeroize-on-drop overhead. Mitigation: negligible for config values read once at startup.

## References

- Related issues: #1441 (secret leak via tracing)
- Related: `packages/configuration/src/v2_0_0/database.rs`
- Related: `packages/configuration/src/v2_0_0/tracker_api.rs`
- Related: [secrecy crate docs](https://docs.rs/secrecy/)
