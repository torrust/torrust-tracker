---
doc-type: issue
issue-type: enhancement
status: open
priority: p2
github-issue: 1490
spec-path: docs/issues/open/1490-1978-decompose-database-configuration.md
branch: "1490-decompose-database-configuration"
related-pr: null
last-updated-utc: 2026-08-24 11:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/v3_0_0/
    - packages/configuration/src/v3_0_0/database.rs
    - packages/configuration/src/lib.rs
    - docs/issues/open/2079-adopt-secrecy-for-sensitive-configuration.md
---

# Issue #1490 - Decompose v3 database configuration

> **EPIC position**: Subissue #8 of 13. Depends on #1640 (subissue #3), because both change `Core`, and on the secrecy follow-up, which establishes secret-handling conventions and protects API tokens in both configuration versions. #1640 removes `core.net` first; the secrecy issue establishes `Secret<String>` use; then #1490 changes `database`. It can otherwise run in parallel with #1415, #1453, #889, and #1987.
>
> **Release sequencing**: The secrecy follow-up and this issue must both be completed before publishing a `torrust-tracker-configuration` release exposing these v3 types. The follow-up prevents a public API containing plain API tokens; this issue establishes `Secret<String>` for the isolated v3 database password. If a release exposing either plain-string API already exists, schedule the change for the next major package version.

## Goal

Replace the ambiguous v3 database `path` field with driver-specific configuration variants for SQLite, MySQL, and PostgreSQL. The resulting TOML makes each connection component explicit, validates driver-specific input, and uses the established `secrecy` convention to protect the new isolated database password.

## Background

The database configuration currently uses one `path` string for two different concepts:

```toml
# SQLite: path is a filesystem path
[core.database]
driver = "sqlite3"
path = "/var/lib/torrust/tracker/database/sqlite3.db"

# MySQL/PostgreSQL: path is a URL
[core.database]
driver = "mysql"
path = "mysql://db_user:db_user_password@mysql:3306/torrust_tracker"
```

This design has several problems:

1. **Misleading name**: `path` is a filesystem path for SQLite but a connection URL for MySQL and PostgreSQL.
2. **Incompatible validation**: SQLite paths and database connection URLs cannot share useful validation rules.
3. **Opaque configuration**: Connection host, port, user, password, and database name cannot be documented or configured independently.
4. **URL encoding burden**: Passwords with URL-reserved characters must be percent-encoded, which couples the configuration format to URL syntax.

The v3 configuration should instead model each database driver directly:

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

The [adopt secrecy for sensitive configuration](2079-adopt-secrecy-for-sensitive-configuration.md) issue is implemented first. It adds the dependency and protects API tokens in both configuration versions, but leaves legacy database URLs as plain strings because their embedded credentials cannot be isolated. This issue then uses the established `Secret<String>` convention for the new, isolated `ConnectionInfo.password`. The legacy v2 database URL retains its explicit `mask_secrets()` redaction.

### TOML representation

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
password = "db_user_password"
database = "torrust_tracker"

# PostgreSQL
[core.database]
driver = "postgresql"
host = "postgres"
port = 5432
user = "postgres"
password = "postgres_password"
database = "torrust_tracker"
```

For MySQL and PostgreSQL, `port` is optional and defaults to `3306` and `5432`, respectively, retaining the effective behavior of the database connection URL parsers. `password` is mandatory and non-empty. SQLite has only `path` and must reject network-database-only fields.

This is a **breaking v3 configuration-schema change** with no fallback for the legacy network database URL. It is appropriate for the v3.0.0 schema release.

## Scope

### In Scope

- Decompose `v3_0_0::database::Database` into `Sqlite3`, `MySQL(ConnectionInfo)`, and `PostgreSQL(ConnectionInfo)` variants.
- Deserialize the `driver` field as the enum discriminant and reject unknown or incompatible fields.
- Default omitted MySQL and PostgreSQL ports to `3306` and `5432`, respectively.
- Require non-empty MySQL and PostgreSQL password fields.
- Use `Secret<String>` for `ConnectionInfo.password`, following the secrecy follow-up's established convention.
- Remove the v3 database `mask_secrets()` implementation once the isolated password is protected by `Secret<String>`; leave v2 URL redaction unchanged.
- Update v3 consumers, tests, examples, benchmarks, E2E config builders, default TOML files, inline TOML, and operational documentation.
- Update the v2-to-v3 migration guide with before/after SQLite, MySQL, and PostgreSQL examples.

### Out of Scope

- Adding `secrecy` dependency infrastructure or changing API-token types; those belong to the preceding secrecy follow-up.
- Changing v2 database URLs or their manual redaction.
- Changing v2 configuration types or v2 TOML.
- Encrypting secrets at rest or changing runtime secret transmission.
- Changing the `Driver` enum in `packages/primitives`.

## Consumer Migration Map

| Category               | Files                                                                        | Change                                                              |
| ---------------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------------- |
| Config definition      | `v3_0_0/database.rs`, `v3_0_0/core.rs`, `v3_0_0/mod.rs`                      | Define and deserialize enum variants; test defaults and validation. |
| Database setup         | `tracker-core/src/databases/setup.rs`                                        | Match the v3 enum and build each driver's connection input.         |
| Test helpers           | `test-helpers/`, `tracker-core/src/test_helpers.rs`, `fixtures.rs`           | Build a `Sqlite3` variant instead of mutating `.path`.              |
| Driver tests           | `tracker-core/src/databases/driver/{mysql,postgres,sqlite}/mod.rs`           | Construct the appropriate variant.                                  |
| Examples               | `http_only_public_tracker.rs`, `udp_only_public_tracker.rs`                  | Construct a `Sqlite3` variant.                                      |
| Benchmarks             | `persistence-benchmark/`                                                     | Construct network variants from container connection data.          |
| E2E config builder     | `qbittorrent_e2e/tracker/config_builder.rs`                                  | Produce the appropriate v3 variant.                                 |
| Configuration fixtures | `share/default/config/*.toml`                                                | Use per-driver TOML fields.                                         |
| Docs and inline TOML   | `docs/containers.md`, migration guide, `mod.rs`, `lib.rs`, integration tests | Replace network database URLs with component fields.                |

## Implementation Plan

| ID  | Status   | Task                                      | Notes                                                                                                        |
| --- | -------- | ----------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| T1  | DONE     | Confirm secrecy prerequisite is merged    | Use the established dependency, `Secret<String>` convention, and API-token changes.                          |
| T2  | DONE     | Define `ConnectionInfo` and `Database`    | Replaced the v3 struct in `packages/configuration/src/v3_0_0/database.rs`.                                   |
| T3  | DONE     | Implement driver-specific deserialization | `driver` selects the variant; incompatible and unknown fields are rejected.                                  |
| T4  | DONE     | Validate network connection values        | Omitted ports default; omitted or blank passwords are rejected with safe errors.                             |
| T5  | DONE     | Protect the isolated v3 password          | Uses `SecretString`; v3 database masking is removed; v2 URL masking is unchanged.                            |
| T6  | DEFERRED | Update v3 database setup                  | Deferred to #1980, which migrates active runtime consumers to v3.                                            |
| T7  | DEFERRED | Update all v3 consumers                   | Active helpers, examples, benchmarks, E2E builders, and fixtures use v2 aliases; #1980 owns their migration. |
| T8  | DONE     | Update user-facing configuration docs     | Updated the v2-to-v3 migration guide; active v2 defaults and operational docs are deferred to #1980.         |
| T9  | DONE     | Verify compatibility and quality          | Stable-toolchain `linter all` and `cargo test --workspace` pass.                                             |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-07-13 21:00 UTC - josecelano - Initial specification drafted.
- 2026-07-14 00:00 UTC - josecelano - Reworked the proposal around a `Database` enum and `ConnectionInfo`; documented the consumer impact.
- 2026-08-21 00:00 UTC - josecelano - Initially replanned #1490 as v3 database-schema decomposition only, with secret typing, API tokens, and manual-redaction policy moved to a separate secrecy effort. Superseded by the later ordering decision for the isolated v3 database password.
- 2026-08-21 16:45 UTC - josecelano - Reordered the work: implement the smaller secrecy refactor first for API tokens in v2 and v3, retaining v2 database URLs and their masking. #1490 follows and uses `Secret<String>` for its new isolated v3 database password.
- 2026-08-24 00:00 UTC - josecelano - Confirmed that #2079 is merged into the implementation base. Confirmed that this is a v3-only breaking-schema migration: v2 remains unchanged because v3.0.0 migration is imminent and changing v2 would introduce an unnecessary breaking change.
- 2026-08-24 00:30 UTC - josecelano - Confirmed that active runtime consumers, defaults, examples, benchmarks, and E2E configuration remain on v2 aliases and are deferred to #1980, which performs the explicit v3 consumer migration. #1490 implements only the isolated v3 schema, validation, secret handling, tests, and migration guide.
- 2026-08-24 01:00 UTC - agent - Implemented the isolated v3 database enum and `ConnectionInfo`, driver-specific TOML validation and port defaults, `SecretString` redaction and authorized persistence serialization, and migration-guide examples. `cargo test -p torrust-tracker-configuration` (111 tests) and package Clippy passed. `linter all` was blocked by a Rust nightly Clippy internal compiler error in the unrelated `swarm-coordination-registry` crate.
- 2026-08-24 11:00 UTC - agent - Resolved an implementation-specific Clippy `needless_pass_by_value` diagnostic. The nightly-only Clippy ICE was reported upstream as rust-lang/rust-clippy#17622. Stable Rust 1.98.0 completed `linter all` successfully; final workspace tests remain.
- 2026-08-24 11:15 UTC - agent - Completed final verification using stable Rust 1.98.0: `cargo test --workspace` and `linter all` passed.

## Acceptance Criteria

- [x] AC1: v3 `Database` is an enum with `Sqlite3`, `MySQL(ConnectionInfo)`, and `PostgreSQL(ConnectionInfo)` variants.
- [x] AC2: v3 TOML accepts the documented fields for each driver and rejects fields that do not apply to its selected driver.
- [x] AC3: Omitted MySQL/PostgreSQL ports default to `3306`/`5432`; omitted or empty network database passwords are rejected.
- [x] AC4: `ConnectionInfo.password` uses `SecretString` and generic serialization emits `"***"`; the v3 database `mask_secrets()` implementation is removed.
- [x] AC5: Isolated v3 configuration consumers compile and pass tests with the new enum; active runtime consumer migration is deferred to #1980.
- [x] AC6: The v2-to-v3 migration guide uses the new per-driver format; active v2 default files, inline TOML, and operational documentation are deferred to #1980.
- [x] `linter all` exits with code `0` (stable Rust 1.98.0).
- [x] Relevant tests pass (`cargo test -p torrust-tracker-configuration` and `cargo test --workspace`).

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-configuration`
- `cargo test -p torrust-tracker-core`
- `cargo test --workspace`
- `linter all`

### Manual Verification Scenarios

| ID  | Scenario                                  | Command/Steps                                                    | Expected Result                                                    | Status | Evidence                                                |
| --- | ----------------------------------------- | ---------------------------------------------------------------- | ------------------------------------------------------------------ | ------ | ------------------------------------------------------- |
| M1  | Parse SQLite configuration                | Deserialize v3 SQLite TOML with `driver = "sqlite3"` and `path`. | The configuration loads and uses the supplied filesystem path.     | PASS   | Configuration tests cover SQLite default-path override. |
| M2  | Parse MySQL and PostgreSQL configurations | Deserialize v3 TOML for each network driver without `port`.      | Omitted ports become `3306`/`5432`.                                | PASS   | Dedicated configuration tests.                          |
| M3  | Reject invalid network credentials        | Deserialize v3 TOML with missing and blank `password` values.    | Loading fails with a safe validation error.                        | PASS   | Dedicated configuration test.                           |
| M4  | Verify database redaction                 | Serialize a v3 MySQL configuration containing a test password.   | The password is absent and generic serialization contains `"***"`. | PASS   | Dedicated configuration test.                           |

### Acceptance Verification

| AC ID | Status | Evidence                                                                 |
| ----- | ------ | ------------------------------------------------------------------------ |
| AC1   | PASS   | `Database` enum implementation and focused tests.                        |
| AC2   | PASS   | Driver-specific and unknown-field rejection tests.                       |
| AC3   | PASS   | MySQL/PostgreSQL port-default and password-validation tests.             |
| AC4   | PASS   | `SecretString` type and redacted generic-serialization test.             |
| AC5   | PASS   | `cargo test -p torrust-tracker-configuration` (111 tests).               |
| AC6   | PASS   | Updated v2-to-v3 migration guide; active v2 artifacts deferred to #1980. |

## Risks and Trade-offs

- **Breaking schema change**: Existing MySQL/PostgreSQL URLs are invalid in v3. Mitigation: document exact before/after examples in the migration guide and preserve v2 unchanged.
- **Consumer breadth**: Many helpers construct or mutate `Database`. Mitigation: use compiler errors and the migration map to update every v3 consumer systematically.
- **Dependency on secrecy conventions**: #1490 relies on the preceding secrecy issue's dependency, serialization, and exposure conventions. Mitigation: do not start #1490 until the secrecy issue is merged; preserve v2 URL masking as an intentionally separate legacy concern.
- **Validation change**: Empty passwords that were technically expressible in a URL will be rejected. Mitigation: this is intentional; report a clear configuration error.

## References

- Related issue: #1441 (secret leak through tracing).
- Prerequisite: [#2079 — Adopt `secrecy` for sensitive configuration](2079-adopt-secrecy-for-sensitive-configuration.md).
- Related: `packages/configuration/src/v2_0_0/database.rs`.
- Related: `packages/configuration/src/v3_0_0/database.rs`.
