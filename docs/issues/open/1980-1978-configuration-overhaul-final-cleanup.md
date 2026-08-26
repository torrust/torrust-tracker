---
doc-type: issue
issue-type: task
status: open
priority: p1
github-issue: 1980
spec-path: docs/issues/open/1980-1978-configuration-overhaul-final-cleanup.md
branch: "config-final-cleanup"
related-pr: null
last-updated-utc: 2026-08-26 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/configuration/src/lib.rs
    - packages/configuration/src/logging.rs
    - packages/configuration/src/v2_0_0/
    - packages/configuration/src/v3_0_0/
    - src/app.rs
    - src/bootstrap/
    - packages/tracker-core/src/
    - packages/http-core/src/
    - packages/udp-core/src/
    - packages/udp-server/src/
    - packages/axum-http-server/src/
    - packages/axum-rest-api-server/src/
    - packages/rest-api-runtime-adapter/src/
    - packages/test-helpers/src/
    - packages/tracker-client/
    - contrib/dev-tools/
    - docs/issues/open/2083-1978-move-max-connection-id-errors-per-ip-to-udp-tracker-server.md
---

# Issue #1980 - Final cleanup: remove global re-exports, migrate all consumers to explicit versioned imports

> **EPIC position**: Final-cleanup subissue in EPIC #1978 — **must precede #2023 and follow all implemented schema subissues, including the preceding secrecy effort.**

## Goal

After all v3 schema changes are implemented, perform the final cleanup:

1. Migrate all consumers from global re-exports (`pub type Core = v2_0_0::core::Core`) to explicit versioned imports (`use torrust_tracker_configuration::v3_0_0::core::Core`)
2. Remove the global re-exports from `packages/configuration/src/lib.rs`
3. Remove the crate-root `packages/configuration/src/logging.rs` (now duplicated inside `v2_0_0/` and `v3_0_0/`)
4. Make the #1453 v3 `udp_tracker_server.ip_bans_reset_interval_in_secs` setting effective in
   the single bootstrap-managed ban cleanup job, replacing its temporary default-constant value
5. Apply any other cleanup discovered during the EPIC implementation
6. Activate the corrected v3 global UDP connection-ID error limit in production

## Background

The `packages/configuration/src/lib.rs` currently re-exports all v2 types as global aliases:

```rust
pub type Configuration = v2_0_0::Configuration;
pub type Core = v2_0_0::core::Core;
pub type Logging = v2_0_0::logging::Logging;
pub type HttpApi = v2_0_0::tracker_api::HttpApi;
pub type HttpTracker = v2_0_0::http_tracker::HttpTracker;
pub type UdpTracker = v2_0_0::udp_tracker::UdpTracker;
pub type Database = v2_0_0::database::Database;
pub type Threshold = v2_0_0::logging::Threshold;
```

These re-exports silently couple consumers to a specific schema version. When the EPIC switches the default to v3, consumers that use `torrust_tracker_configuration::Core` would silently get a different type — potentially breaking at compile time in confusing ways.

The decision is to **remove all global re-exports** and force consumers to import from explicit versioned paths. This is a breaking change that is appropriate for the major version bump accompanying this EPIC.

Similarly, the crate-root `logging.rs` (which contains `TraceStyle`, `setup()`, and `tracing_init()`) was copied into both `v2_0_0/` and `v3_0_0/` during subissue #1. The original crate-root file should be removed.

## Scope

### In Scope

- Migrate all ~30 consumer files from global re-exports to explicit `v3_0_0` imports
- Remove global type aliases from `packages/configuration/src/lib.rs`
- Remove crate-root `packages/configuration/src/logging.rs`
- Update `pub mod logging;` in `lib.rs` (remove or redirect)
- Replace #1453's temporary default-constant cleanup interval with
  `Configuration::udp_tracker_server.ip_bans_reset_interval_in_secs`
- Read the corrected v3 `Configuration::udp_tracker_server.max_connection_id_errors_per_ip`
  once in `AppContainer` and pass it to the shared `UdpTrackerCoreServices`/
  `BanService` initialization path, replacing the current first-listener v2
  selection.
- Add production runtime coverage with two UDP listeners that proves the one
  declared v3 threshold is shared and listener declaration order has no effect.
- Activate v3 configuration while retaining an explicit, named fixed-SQLite
  compatibility bridge for persistence composition. The bridge is temporary:
  it keeps the runtime persistence-enabled while the later activation follow-up
  makes an omitted v3 `[core.database]` effective at runtime.
- Complete the v2-to-v3 migration guide from the final schemas and defaults.
  Document every user-facing key move, rename, removal, semantic change, and a
  practical migration sequence. Do not claim persistence-free runtime support.
- Ensure all tests pass after migration
- Any additional cleanup items discovered during EPIC implementation

### Out of Scope

- Removing `v2_0_0/` module (it stays deprecated for backward compatibility)
- Changes to the v3 schema itself (already done in previous subissues)

## Consumer Migration Map

The following files import from global re-exports and need updating. Each import `torrust_tracker_configuration::X` becomes `torrust_tracker_configuration::v3_0_0::<module>::X`.

### Core consumers (~15 files)

| File                                                    | Current Import                                                                | New Import                                                                                                                          |
| ------------------------------------------------------- | ----------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `packages/tracker-core/src/announce_handler.rs`         | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/tracker-core/src/container.rs`                | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/tracker-core/src/authentication/service.rs`   | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/tracker-core/src/databases/setup.rs`          | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/tracker-core/src/torrent/manager.rs`          | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/tracker-core/src/whitelist/authorization.rs`  | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/http-core/src/services/announce.rs`           | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/http-core/src/services/scrape.rs`             | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/http-core/src/container.rs`                   | `use torrust_tracker_configuration::{Core, HttpTracker}`                      | `use torrust_tracker_configuration::v3_0_0::{core::Core, http_tracker::HttpTracker}`                                                |
| `packages/udp-core/src/container.rs`                    | `use torrust_tracker_configuration::{Core, UdpTracker}`                       | `use torrust_tracker_configuration::v3_0_0::{core::Core, udp_tracker::UdpTracker}`                                                  |
| `packages/udp-server/src/container.rs`                  | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/udp-server/src/handlers/announce.rs`          | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |
| `packages/rest-api-runtime-adapter/src/v1/container.rs` | `use torrust_tracker_configuration::{Core, HttpApi, HttpTracker, UdpTracker}` | `use torrust_tracker_configuration::v3_0_0::{core::Core, tracker_api::HttpApi, http_tracker::HttpTracker, udp_tracker::UdpTracker}` |
| `src/bootstrap/jobs/torrent_cleanup.rs`                 | `use torrust_tracker_configuration::Core`                                     | `use torrust_tracker_configuration::v3_0_0::core::Core`                                                                             |

### Configuration consumers (~10 files)

| File                                                       | Current Import                                                                                         | New Import                                                                                                                                                               |
| ---------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `src/app.rs`                                               | `use torrust_tracker_configuration::{Configuration, HttpTracker, UdpTracker}`                          | `use torrust_tracker_configuration::v3_0_0::{Configuration, http_tracker::HttpTracker, udp_tracker::UdpTracker}`                                                         |
| `src/container.rs`                                         | `use torrust_tracker_configuration::{Configuration, HttpApi}`                                          | `use torrust_tracker_configuration::v3_0_0::{Configuration, tracker_api::HttpApi}`                                                                                       |
| `src/bootstrap/app.rs`                                     | `use torrust_tracker_configuration::{Configuration, logging}`                                          | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                                                                                                    |
| `src/bootstrap/config.rs`                                  | `use torrust_tracker_configuration::{Configuration, Info}`                                             | `use torrust_tracker_configuration::v3_0_0::{Configuration, Info}`                                                                                                       |
| `src/bootstrap/jobs/http_tracker_core.rs`                  | `use torrust_tracker_configuration::Configuration`                                                     | `use torrust_tracker_configuration::v3_0_0::Configuration`                                                                                                               |
| `src/bootstrap/jobs/torrent_repository.rs`                 | `use torrust_tracker_configuration::Configuration`                                                     | `use torrust_tracker_configuration::v3_0_0::Configuration`                                                                                                               |
| `src/bootstrap/jobs/tracker_core.rs`                       | `use torrust_tracker_configuration::Configuration`                                                     | `use torrust_tracker_configuration::v3_0_0::Configuration`                                                                                                               |
| `src/bootstrap/jobs/activity_metrics_updater.rs`           | `use torrust_tracker_configuration::Configuration`                                                     | `use torrust_tracker_configuration::v3_0_0::Configuration`                                                                                                               |
| `src/console/ci/qbittorrent_e2e/tracker/config_builder.rs` | `use torrust_tracker_configuration::{Configuration, HealthCheckApi, HttpApi, HttpTracker, UdpTracker}` | `use torrust_tracker_configuration::v3_0_0::{Configuration, health_check_api::HealthCheckApi, tracker_api::HttpApi, http_tracker::HttpTracker, udp_tracker::UdpTracker}` |

### Test/example/bench consumers (~10 files)

| File                                                                   | Current Import                                                                                    | New Import                                                                                                                                                 |
| ---------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `packages/test-helpers/src/configuration.rs`                           | `use torrust_tracker_configuration::{Configuration, HttpApi, HttpTracker, Threshold, UdpTracker}` | `use torrust_tracker_configuration::v3_0_0::{Configuration, tracker_api::HttpApi, http_tracker::HttpTracker, logging::Threshold, udp_tracker::UdpTracker}` |
| `packages/test-helpers/src/logging.rs`                                 | `use torrust_tracker_configuration::logging::TraceStyle`                                          | `use torrust_tracker_configuration::v3_0_0::logging::TraceStyle`                                                                                           |
| `packages/axum-http-server/src/server.rs` (tests)                      | `use torrust_tracker_configuration::{Configuration, logging}`                                     | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                                                                                      |
| `packages/axum-http-server/src/testing/environment.rs`                 | `use torrust_tracker_configuration::{Core, HttpTracker}`                                          | `use torrust_tracker_configuration::v3_0_0::{core::Core, http_tracker::HttpTracker}`                                                                       |
| `packages/axum-rest-api-server/src/server.rs` (tests)                  | `use torrust_tracker_configuration::{Configuration, logging}`                                     | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                                                                                      |
| `packages/axum-rest-api-server/src/testing/environment.rs`             | `use torrust_tracker_configuration::{Configuration, logging}`                                     | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                                                                                      |
| `packages/axum-http-server/examples/http_only_public_tracker.rs`       | `use torrust_tracker_configuration::{Core, HttpTracker}`                                          | `use torrust_tracker_configuration::v3_0_0::{core::Core, http_tracker::HttpTracker}`                                                                       |
| `packages/udp-server/examples/udp_only_public_tracker.rs`              | `use torrust_tracker_configuration::{Core, UdpTracker}`                                           | `use torrust_tracker_configuration::v3_0_0::{core::Core, udp_tracker::UdpTracker}`                                                                         |
| `packages/http-core/benches/helpers/util.rs`                           | `use torrust_tracker_configuration::{Configuration, Core}`                                        | `use torrust_tracker_configuration::v3_0_0::{Configuration, core::Core}`                                                                                   |
| `contrib/dev-tools/analysis/workspace-coupling/tests/parse_imports.rs` | `use torrust_tracker_configuration::{Core, UdpTracker}`                                           | `use torrust_tracker_configuration::v3_0_0::{core::Core, udp_tracker::UdpTracker}`                                                                         |

### `logging` module consumers

Files that import `torrust_tracker_configuration::logging` (the module, not the type):

| File                                                       | Current Import                                                                           | New Import                                                                                       |
| ---------------------------------------------------------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ |
| `src/bootstrap/app.rs`                                     | `use torrust_tracker_configuration::{Configuration, logging}` then `logging::setup(...)` | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}` then `logging::setup(...)` |
| `packages/axum-http-server/src/server.rs` (tests)          | `use torrust_tracker_configuration::{Configuration, logging}`                            | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                            |
| `packages/axum-rest-api-server/src/server.rs` (tests)      | `use torrust_tracker_configuration::{Configuration, logging}`                            | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                            |
| `packages/axum-rest-api-server/src/testing/environment.rs` | `use torrust_tracker_configuration::{Configuration, logging}`                            | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                            |
| `packages/udp-server/src/server/mod.rs` (tests)            | `use torrust_tracker_configuration::{Configuration, logging}`                            | `use torrust_tracker_configuration::v3_0_0::{Configuration, logging}`                            |
| `packages/test-helpers/src/logging.rs`                     | `use torrust_tracker_configuration::logging::TraceStyle`                                 | `use torrust_tracker_configuration::v3_0_0::logging::TraceStyle`                                 |

## Implementation Plan

| ID  | Status | Task                                                                | Notes                                                                                                                                                                                                                                                                                                                                                   |
| --- | ------ | ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Migrate all consumer imports to explicit `v3_0_0` paths             | ~30 files; see Consumer Migration Map above                                                                                                                                                                                                                                                                                                             |
| T2  | TODO   | Remove global type aliases from `lib.rs`                            | `pub type Configuration = ...` etc.                                                                                                                                                                                                                                                                                                                     |
| T3  | TODO   | Remove crate-root `logging.rs`                                      | Already copied into `v2_0_0/` and `v3_0_0/`                                                                                                                                                                                                                                                                                                             |
| T4  | TODO   | Remove `pub mod logging;` from `lib.rs`                             | Or redirect to versioned module if needed                                                                                                                                                                                                                                                                                                               |
| T5  | TODO   | Enable #1453's v3 ban-cleanup interval                              | Replace its temporary 24-hour default-constant bootstrap value after consumer migration                                                                                                                                                                                                                                                                 |
| T6  | TODO   | Remove hardcoded `ConnectionIdValidationPolicy` in test environment | `packages/udp-server/src/testing/environment.rs` hardcodes `Strict` because v2 config lacks the field; after v3 migration the field is available natively in `UdpTracker`                                                                                                                                                                               |
| T7  | TODO   | Apply any additional cleanup discovered during EPIC                 | Document in progress log                                                                                                                                                                                                                                                                                                                                |
| T8  | TODO   | Run #889 deferred manual verification scenarios (M1–M5)             | After consumer migration, run tracker with v3 config and verify all four trace styles + `trace_filter` filtering                                                                                                                                                                                                                                        |
| T9  | TODO   | Run `linter all` and full test suite                                |                                                                                                                                                                                                                                                                                                                                                         |
| T10 | TODO   | Complete v2-to-v3 migration guide                                   | Compare the final v2 and v3 schemas and defaults. Document all user-facing key/table moves, renames, removals, changed semantics, complete v3 examples, and a practical migration sequence. State clearly that #1980 retains a fixed-SQLite persistence bridge and the later activation follow-up makes omitted `[core.database]` effective at runtime. |
| T11 | TODO   | Run #1987 enabled-mode local manual verification                    | After consumer migration activates v3.0.0 at runtime, enable `use_ip_from_query_string` for a local HTTP tracker and execute #1987's enabled-mode local scenarios. Append reproducible commands and evidence to `docs/issues/open/1987-add-config-option-to-use-ip-from-announce-query-string/manual-verification.md`.                                  |
| T12 | TODO   | Activate corrected global UDP error limit                           | After its preceding v3 schema-fix subissue is merged, read `udp_tracker_server.max_connection_id_errors_per_ip` in `AppContainer` and pass it once to the shared UDP core services; remove the first-listener selection.                                                                                                                                |
| T13 | TODO   | Verify shared UDP error limit at runtime                            | Register and run a two-listener integration test using one bound UDP client socket. Verify the declared v3 limit, cross-listener shared ban enforcement, and listener-order independence.                                                                                                                                                               |

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

- 2026-07-14 00:00 UTC - josecelano - Initial spec drafted
- 2026-07-15 00:00 UTC - josecelano - GitHub issue #1980 created; spec moved to `docs/issues/open/1980-1978-configuration-overhaul-final-cleanup.md`
- 2026-07-23 17:02 UTC - josecelano - Added the deferred #1453 runtime-consumption task: after
  migrating consumers to v3, replace the temporary 24-hour default-constant global ban cleanup interval
  with `udp_tracker_server.ip_bans_reset_interval_in_secs`.
- 2026-07-27 12:36 UTC - agent - Added T6: `environment.rs` hardcoded `ConnectionIdValidationPolicy::Strict`
  must be replaced with the v3 config's native field after consumer migration (#1136).
- 2026-07-28 00:00 UTC - agent - Added T8: run #889 deferred manual verification scenarios (M1–M5)
  after consumer migration. These scenarios require the tracker to use v3 config, which is not
  possible until this cleanup migrates global callers.
- 2026-08-18 00:00 UTC - Copilot/User - Added T11: run #1987 enabled-mode local manual verification after this issue activates v3.0.0 configuration at runtime.
- 2026-08-24 00:00 UTC - GitHub Copilot/User - Added T12–T13 as the production-activation handoff for the preceding v3 schema correction that moves `max_connection_id_errors_per_ip` to `udp_tracker_server`; this issue must replace the current first-listener runtime selection and prove shared, order-independent enforcement.
- 2026-08-26 00:00 UTC - GitHub Copilot/User - Confirmed that #1980 retains an explicit named fixed-SQLite compatibility bridge while activating v3 consumers. The later activation follow-up will replace it with the real optional `core.database` value after #1980 is merged and its evidence is reviewed. Expanded T10: the migration guide must be completed from a final v2-versus-v3 schema/default comparison, not treated as a brief cleanup note.

## Acceptance Criteria

- [ ] AC1: All consumer imports use explicit `v3_0_0` paths (no global re-export usage remains)
- [ ] AC2: Global type aliases removed from `packages/configuration/src/lib.rs`
- [ ] AC3: Crate-root `packages/configuration/src/logging.rs` removed
- [ ] AC4: `pub mod logging;` removed or redirected in `lib.rs`
- [ ] AC5: All tests pass with the new import paths
- [ ] AC6: `v2_0_0` module remains available (deprecated but not removed)
- [ ] AC7: The global ban cleanup job uses the v3 `udp_tracker_server.ip_bans_reset_interval_in_secs` value
- [ ] AC8: `AppContainer` reads the one v3 `udp_tracker_server.max_connection_id_errors_per_ip` value and initializes the shared `BanService` with it; it does not select a listener value.
- [ ] AC9: With two UDP listeners, the configured v3 threshold is enforced by the one shared `BanService`, and reversing listener declarations does not change enforcement.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace`
- `cargo build --workspace` (verify no broken imports)
- `cargo test --test banning-udp-shared-connection-id-error-limit`

### Manual Verification Scenarios

| ID  | Scenario                          | Command/Steps                                                                                                                                                                                                                                  | Expected Result                                                                                                                          | Status | Evidence |
| --- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Verify no global re-export usage  | `rg 'torrust_tracker_configuration::(Core\|Configuration\|Logging\|HttpApi\|HttpTracker\|UdpTracker\|Database\|Threshold)[^:]'`                                                                                                                | No matches (all use v3_0_0 paths)                                                                                                        | TODO   |          |
| M2  | Verify v2 module still accessible | `cargo doc --document-private-items`                                                                                                                                                                                                           | v2_0_0 types documented                                                                                                                  | TODO   |          |
| M3  | Verify v3 module is the default   | Check `lib.rs` for `LATEST_VERSION`                                                                                                                                                                                                            | `LATEST_VERSION = "3.0.0"`                                                                                                               | TODO   |          |
| M4  | Verify global UDP error limit     | Start two UDP listeners using v3 configuration with `udp_tracker_server.max_connection_id_errors_per_ip = 2`. From one bound UDP socket, send invalid connection-ID requests to both listeners and repeat with listener declarations reversed. | The shared ban budget is consumed across listeners, and both declaration orders produce the same response sequence and ban metric delta. | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |
| AC6   | TODO   |          |
| AC7   | TODO   |          |
| AC8   | TODO   |          |
| AC9   | TODO   |          |

## Risks and Trade-offs

- **Large diff**: ~30 files changed in one subissue. Mitigation: the changes are mechanical (search-and-replace import paths); each file change is trivial.
- **Merge conflicts**: Other subissues may touch the same consumer files. Mitigation: this subissue runs last (Phase 4), after all v3 schema changes are merged.
- **Breaking change for external consumers**: Any external crate depending on `torrust-tracker-configuration` must update imports. Mitigation: this is expected for a major version bump; documented in changelog.

## References

- EPIC: Configuration Overhaul (schema v3.0.0)
- Related: `packages/configuration/src/lib.rs`
- Related: `packages/configuration/src/logging.rs`
