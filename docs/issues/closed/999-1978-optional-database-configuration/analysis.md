---
semantic-links:
  related-artifacts:
    - docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md
    - docs/issues/closed/999-1978-optional-database-configuration/baseline-e2e-verification.md
    - docs/issues/closed/1980-1978-configuration-overhaul-final-cleanup.md
    - packages/tracker-core/
    - packages/configuration/src/v2_0_0/
    - packages/configuration/src/v3_0_0/
    - src/container.rs
---

# Phase 1 - Persistence dependency analysis

## Scope and evidence status

This document records **verified current-state facts** as of the Phase 1
analysis branch. It does not select an optional-persistence design, change a
runtime contract, or decide whether #999 blocks #1980. The current runtime
uses schema v2 aliases; the v3 types are present but are not yet handed to the
application runtime. See `packages/configuration/src/lib.rs` and #1980's
consumer migration map.

The pre-implementation reproduction remains preserved in
[`baseline-e2e-verification.md`](baseline-e2e-verification.md): the v2 UDP
benchmark configuration starts with a 49,152-byte SQLite file and the complete
shared schema even with the known persistence features disabled.

## Configuration and startup lifecycle

### Active v2 configuration

#### Verified facts

- `packages/configuration/src/lib.rs` aliases `Configuration`, `Core`, and
  `Database` to `v2_0_0`. `src/bootstrap/config.rs` loads that alias, so this
  is the active runtime contract.
- `packages/configuration/src/v2_0_0/core.rs`, `Core::database`, is a
  non-optional Rust field with `#[serde(default = "Core::default_database")]`.
  `packages/configuration/src/v2_0_0/database.rs`, `Database::default`, uses
  SQLite and `./storage/tracker/lib/database/sqlite3.db`.
- Thus v2 does not require an explicitly written `[core.database]` TOML table:
  omission resolves to the SQLite default. It remains an unconditional runtime
  database requirement because `TrackerCoreContainer::initialize_from` always
  initializes it. The reconciled baseline missing-database control records the
  same distinction and does not claim an omitted v2 section is a parse error.
- `v2_0_0::Configuration::load` first selects full TOML from
  `TORRUST_TRACKER_CONFIG_TOML`, else the file named by
  `TORRUST_TRACKER_CONFIG_TOML_PATH`, else bootstrap's
  `share/default/config/tracker.development.sqlite3.toml`. It merges
  `TORRUST_TRACKER_CONFIG_OVERRIDE_` variables split on `__` before joining
  Rust defaults. Database overrides include
  `CORE__DATABASE__DRIVER` and `CORE__DATABASE__PATH`.
- V2 mandatory source values are `metadata.schema_version`,
  `logging.threshold`, `core.private`, and `core.listed`; database settings
  are supplied by defaults when absent. Sources:
  `packages/configuration/src/v2_0_0/mod.rs`, `Configuration::load` and
  `check_mandatory_options`.

### Dormant v3 configuration and #1980 handoff

#### Verified facts

- `packages/configuration/src/v3_0_0/core.rs`, `Core::database`, is presently
  non-optional and defaults to `Database::default()`.
- `packages/configuration/src/v3_0_0/database.rs` defines the driver-specific
  `Database::{Sqlite3 { path }, MySQL(ConnectionInfo), PostgreSQL(ConnectionInfo)}`.
  SQLite defaults its path; MySQL and PostgreSQL require `host`, `user`, a
  non-empty secret `password`, and `database`, with ports defaulting to 3306
  and 5432 respectively. Driver-incompatible and unknown fields are rejected.
- V3's loader uses the same TOML selection and override prefix. It removes
  `core.database` from Figment defaults before extraction, avoiding accidental
  merging of a default SQLite path with a supplied network-driver table.
  Sources: `v3_0_0/mod.rs`, `Configuration::load` and `defaults_for_loading`.
- V3 is **not** runtime-compatible with current database setup:
  `packages/tracker-core/src/databases/setup.rs`, `initialize_database`, reads
  `config.database.driver` and `.path`, members only on v2's `Database`.
  No v3-to-runtime adapter exists. #1980 explicitly assigns migration of
  `src/bootstrap/`, `src/container.rs`, tracker-core, protocol packages, test
  helpers, examples, benchmarks, and the qBittorrent E2E builder to its
  consumer migration work. Configuration defaults require a separate
  compatibility review if the approved v3 optional-database contract changes
  them.

### Driver construction and migrations

#### Verified lifecycle

```text
src/app.rs::run
  -> bootstrap::app::setup
  -> AppContainer::initialize
  -> TrackerCoreContainer::initialize_from
  -> databases::setup::initialize_database
  -> selected driver construction + create_database_tables
  -> app::start loads enabled persisted state and starts jobs
```

- `src/bootstrap/app.rs::setup` loads configuration, calls
  `Configuration::validate()`, initializes logging, and then awaits
  `AppContainer::initialize`.
- `packages/tracker-core/src/container.rs::TrackerCoreContainer::initialize_from`
  unconditionally calls `initialize_database` before constructing its
  whitelist, keys, metrics, torrent, announce, and scrape services.
- The production `AppContainer::tracker_http_api_container` reuses that
  prebuilt tracker-core container. Separately,
  `packages/rest-api-runtime-adapter/src/v1/container.rs`,
  `TrackerHttpApiCoreContainer::initialize`, constructs a new
  `TrackerCoreContainer` and consequently performs the same database
  initialization and migration lifecycle. This latter path is used by REST
  server/test construction (`packages/axum-rest-api-server/src/server.rs` and
  `src/bootstrap/jobs/tracker_apis.rs` tests), not the main application startup.
- `initialize_database` creates one concrete driver, immediately calls
  `SchemaMigrator::create_database_tables()`, then exposes that one driver as
  narrow `SchemaMigrator`, `TorrentMetricsStore`, `WhitelistStore`, and
  `AuthKeyStore` trait objects in `DatabaseStores`. It uses `expect`; malformed
  connection input, unavailable network database, authentication/DDL failure,
  or migration failure is a startup panic.
- SQLite (`driver/sqlite/mod.rs`) uses lazy SQLx pooling with
  `SqliteConnectOptions::filename(...).create_if_missing(true)`. The immediate
  migration query causes a configured missing file to be created at startup.
- MySQL (`driver/mysql/mod.rs`) parses the v2 DSN with
  `MySqlConnectOptions::from_str`; PostgreSQL (`driver/postgres/mod.rs`) uses
  `PgConnectOptions::from_str`. Both pools are lazy, but the immediate migration
  requires a reachable database server at startup.
- All drivers embed and apply their full backend migration set through
  `migrations/{sqlite,mysql,postgresql}`. SQLx records applied migrations in
  `_sqlx_migrations`; repeated completed runs are idempotent. SQLite and MySQL
  schema migrators contain legacy pre-v4 bootstrap logic, including rejection
  of partially migrated legacy schemas. PostgreSQL runs embedded migrations
  directly because its schema migrator documents no pre-v4 PostgreSQL legacy
  database. Sources: the three driver `schema_migrator.rs` files and
  `packages/tracker-core/migrations/`.

### Container lifecycle

#### Verified facts

- `Containerfile` supplies
  `TORRUST_TRACKER_CONFIG_OVERRIDE_CORE__DATABASE__DRIVER=sqlite3` and uses
  `share/container/entry_script_sh` as the entrypoint. Its tester image creates
  a packaged empty SQLite file with `sqlite3 ... "VACUUM;"`.
- Before executing the tracker, `entry_script_sh` unconditionally creates
  `/var/lib/torrust/tracker/database/` and `/etc/torrust/tracker/`, applies
  ownership and mode changes, and exits if the driver override is absent.
- It selects a SQLite, MySQL, or PostgreSQL default config from the driver
  override. For SQLite it also selects the packaged empty database. `inst`
  installs only when the target does not exist, so mounted prior config/database
  files persist across later starts; changing only the driver variable does not
  replace them.
- The v2 container configs are
  `tracker.container.{sqlite3,mysql,postgresql}.toml`; each includes a v2
  database contract. SQLite uses the container database path; MySQL and
  PostgreSQL use DSNs. Therefore the entrypoint has independent persistence
  side effects before application configuration validation.

## Persistence-consumer inventory

The following requirements are **facts about current behavior**, not Phase 2
decisions. All persistence objects are currently constructed before any feature
condition is inspected.

### Whitelist

- **Enabled by:** `core.listed`, which controls announce and scrape enforcement.
- **Dependency path:** `DatabaseStores.whitelist_store` ->
  `DatabaseWhitelist` -> `WhitelistManager`.
- **Current behavior:** writes persist before in-memory mutation; a store
  failure returns a database error.
- **Startup and tests:** `src/app.rs::load_whitelisted_torrents` reads the
  database only when listed, although the service is always constructed. See
  `whitelist/repository/persisted.rs` and `whitelist/manager.rs` tests.
- **REST API coupling:** direct add, remove, and reload routes, not gated by
  `core.listed`; see the route inventory below.

### Private-tracker keys

- **Enabled by:** `core.private`, which controls authentication.
- **Dependency path:** `auth_key_store` -> `DatabaseKeyRepository` ->
  `KeysHandler`.
- **Current behavior:** add, generate, and remove persist before in-memory
  mutation. Store errors are returned; no in-memory-only fallback exists.
- **Startup and tests:** `load_peer_keys` reads only when private, although the
  service is always constructed. See `authentication/handler.rs` and
  `authentication/key/repository/persisted.rs` tests.
- **REST API coupling:** direct key add, generate, delete, and reload routes,
  not gated by `core.private`; see the route inventory below.

### Persistent completed metrics

- **Enabled by:** `core.tracker_policy.persistent_torrent_completed_stat`.
- **Dependency path:** `torrent_metrics_store` ->
  `DatabaseDownloadsMetricRepository`.
- **Current behavior:** the announce path conditionally loads a torrent's
  completed count. Completion handling reads, then inserts or updates, both a
  per-torrent count and the aggregate count.
- **Startup and tests:** `load_torrent_metrics` restores only the global
  aggregate metric when enabled. `TorrentsManager::load_torrents_from_database`
  has no production startup call; `AnnounceHandler` lazily loads a per-torrent
  count on its first announce. Pre-load errors propagate, while event write
  failures are logged and processing continues. See persistence/restart cases
  in `tracker-core/tests/integration.rs` and
  `statistics/persisted/downloads.rs`.
- **REST API coupling:** indirect only. Torrent, stats, and metrics routes read
  in-memory values that may have been seeded from persistence.

### In-memory torrent, swarm, and usage metrics

- **Enabled by:** `tracker_usage_statistics` controls some jobs, but does not
  alone require persistence.
- **Dependency path:** `InMemoryTorrentRepository`, the swarm registry, and
  metric repositories have no database constructor dependency.
- **Current behavior:** a torrent can receive a persisted completed count only
  when persistent completion metrics are enabled. `torrent_cleanup` and
  activity jobs are in-memory. `tracker_core_event_listener` starts when usage
  statistics **or** persistent completion metrics are enabled; only the latter
  causes persistence writes.
- **REST API coupling:** torrent, stats, and metrics routes do not directly
  query the database.

### REST management service

- **Enabled by:** `http_api.is_some()`.
- **Dependency path:** API construction receives the already-created
  `TrackerCoreContainer`; it has no unavailable-store representation.
- **Startup:** `src/app.rs::start_the_http_api` runs after unconditional
  database creation.
- **Persistence coupling:** direct persistence routes are always assembled
  while the API is enabled.

Other direct `initialize_database` callers identified by exact search are
test helpers, repository/manager tests, protocol tests and benchmarks, and the
explicit `packages/persistence-benchmark` tool. They are not production
application construction paths. `TrackerHttpApiCoreContainer::initialize` is
the additional REST server/test construction path described above. Main
production construction is the container lifecycle above.

`packages/test-helpers/src/configuration.rs::ephemeral_configuration` always
provisions an ephemeral SQLite database and assigns its path to the v2 core
configuration. Its public, private, and listed helpers derive from that base.
Consequently, these test environments—including REST API environments—exercise
configured SQLite even when their feature flag is disabled; they do not provide
coverage for absent persistence.

## Management REST API inventory

### Shared API facts

`http_api` is optional, but when present `src/app.rs::start_the_http_api`
constructs `TrackerHttpApiCoreContainer` from the full tracker-core container.
`packages/axum-rest-api-server/src/routes.rs` applies the shared token
middleware to v1 routes. `v1/middlewares/auth.rs` accepts Bearer or query-token
authentication (header wins); configured tokens have equal privilege. This is
the current authorization policy, not a persistence feature gate.

In the active v2 runtime, a database driver and its shared schema are always
initialized before the REST API starts. Therefore, the existing database-error
handling on direct whitelist and key routes is a defense against a configured
database becoming unavailable or failing after startup; it is not behavior for
an omitted database configuration. That state is not representable in the
current application.

| Route / operation                                         | Domain                             | Current dependency path                                                                          | Current unavailable behavior and evidence                                                                                                                                                                                                               |
| --------------------------------------------------------- | ---------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `POST /api/v1/whitelist/{info_hash}`                      | Whitelist write                    | `WhitelistApiService` -> `TrackerWhitelistAdapter` -> `WhitelistManager` -> `DatabaseWhitelist`. | `WhitelistError::Database` becomes the existing generic failure response documented/tested as 500. No absent-persistence branch exists. Sources: `v1/context/whitelist/{routes,handlers}.rs`, runtime adapter, application use case, and contract test. |
| `DELETE /api/v1/whitelist/{info_hash}`                    | Whitelist write                    | Same manager/repository chain.                                                                   | Same generic database failure mapping; not gated by `core.listed`.                                                                                                                                                                                      |
| `GET /api/v1/whitelist/reload`                            | Whitelist read                     | `WhitelistManager::load_whitelist_from_database`.                                                | Same database failure mapping; directly accesses persistence even when `core.listed` is false.                                                                                                                                                          |
| `POST /api/v1/keys`                                       | Authentication-key write           | `AuthKeyApiService` -> `TrackerAuthKeyAdapter` -> `KeysHandler` -> `DatabaseKeyRepository`.      | `AuthKeyError::Database` follows the existing failure response path; no unavailable-store branch. Sources: `v1/context/auth_key/{routes,handlers}.rs`, adapter, use case, contract test.                                                                |
| `POST /api/v1/key/{seconds_valid_or_key}`                 | Deprecated expiring-key generation | `KeysHandler::generate_expiring_peer_key` -> key repository.                                     | Existing generation failure response on database error; not gated by `core.private`.                                                                                                                                                                    |
| `DELETE /api/v1/key/{seconds_valid_or_key}`               | Authentication-key deletion        | `KeysHandler::remove_peer_key` -> key repository.                                                | Existing failure response on database error; not gated by `core.private`.                                                                                                                                                                               |
| `GET /api/v1/keys/reload`                                 | Authentication-key read            | `KeysHandler::load_peer_keys_from_database`.                                                     | Existing failure response on database error; directly accesses persistence even when `core.private` is false.                                                                                                                                           |
| `GET /api/v1/torrent/{info_hash}`, `GET /api/v1/torrents` | Torrent reads                      | In-memory torrent repository.                                                                    | No handler database access. A completed count can have originated from persistence when that feature is enabled. Sources: `v1/context/torrent/*` and adapter/tests.                                                                                     |
| `GET /api/v1/stats`, `GET /api/v1/metrics`                | Statistics reads                   | In-memory metric repositories.                                                                   | No handler database access. The completed metric may be seeded or updated by persistence-backed completion metrics. Sources: `v1/context/stats/*` and adapter/tests.                                                                                    |

The whitelist and auth-key contract tests force database failures by dropping
schema tables through `packages/axum-rest-api-server/tests/server/mod.rs`,
`force_database_error`. They test a configured-but-failing database, not an
absent database configuration.

**Phase 2 constraints evidenced by this inventory:** direct persistence routes
are currently built independently of `listed` and `private`, and the code only
represents a database that succeeds or fails. An absent database must therefore
be represented or excluded deliberately before runtime activation; final route
availability and status semantics are not selected here.

## Validation-layer and activation compatibility inventory

### Current validation path

- `packages/configuration/src/validator.rs` defines the cross-field
  `Validator` trait and presently only
  `SemanticValidationError::UselessPrivateModeSection`.
- Both `v2_0_0::Core::validate` and `v3_0_0::Core::validate` reject a supplied
  `private_mode` section when `private` is false. Each version's
  `Configuration::validate` delegates to `Core::validate`.
- `src/bootstrap/app.rs::setup` invokes `configuration.validate()` before
  `AppContainer::initialize` and therefore before driver construction.
- The validation-layer ADR classifies a database requirement induced by
  `core.private`, `core.listed`, or
  `core.tracker_policy.persistent_torrent_completed_stat` as a **cross-field
  configuration relationship** if the final model needs only those settings.
  Database reachability, DDL permission, filesystem access, and credentials
  remain **runtime/environment facts**. No new rule is selected in Phase 1.

### #1980 and v3 activation surfaces

The #1980 consumer migration map identifies all runtime users of configuration
types, including `src/app.rs`, `src/container.rs`, bootstrap, tracker-core
database setup and protocol consumers, REST adapter container, test helpers,
examples, benchmarks, and the qBittorrent E2E builder. Its T1/T9/T10 tasks and
the v2-to-v3 migration guide are affected by any approved v3 optional-database
contract. `share/default/config/`, `docs/containers.md`, container defaults,
and the entrypoint also need a later compatibility review because they currently
encode or install the v2 database lifecycle.

**Unresolved Phase 2 question:** #1980 is the planned runtime activation point,
but Phase 1 does not decide whether v3 optional database configuration must be
implemented before that migration. The decision requires maintainer approval of
the v3 contract and the direct REST API behavior above.

## Reconciliation and unresolved questions

1. The baseline result is consistent with source: an unconditional call to
   `initialize_database` runs before any persistence feature condition, and
   SQLite migration activates `create_if_missing`.
2. The one shared migration lifecycle is already enforced by one driver object
   exposed as all narrow stores; Phase 1 found no feature-specific schema or
   migration stream.
3. Confirm the staged direction in `solution.md`: #999 adds `Option<Database>`
   and optional container dependencies, while the active bootstrap deliberately
   supplies a temporary `Some(Database)` bridge.
4. Confirm the initial capability matrix and exact diagnostics for the small
   post-activation follow-up that replaces the bridge with actual v3
   configuration.
5. Define the container-entrypoint changes required by that follow-up for a
   v3 persistence-free startup without its current driver variable, database
   directory, or packaged SQLite install.
6. Approve the ADR draft and refine it during Phase 3; retain the activation
   follow-up and future persistence-awareness EPIC drafts for their respective
   post-#1980 and post-#999 planning work.
7. Confirm the staged #999 -> #1980 -> activation-follow-up ordering and update
   EPIC #1978 and the v2-to-v3 migration guidance during Phase 2.
