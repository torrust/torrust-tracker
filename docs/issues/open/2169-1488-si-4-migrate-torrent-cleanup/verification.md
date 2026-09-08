# Verification Evidence — Torrent Cleanup Token Migration

> **Status**: Complete for T1-T5 and T7. T6 is recorded in a separate skill
> commit after this evidence.

## Environment

- Date: 2026-09-08
- OS: Linux
- Rust version (`rustc --version`): `rustc 1.100.0-nightly (f248f4038 2026-09-05)`
- Tracker base commit/branch: `e4db63d5`, `2169-migrate-torrent-cleanup`
- Isolated runtime configuration: `.tmp/2169-torrent-cleanup.toml`, UDP `127.0.0.1:16969`, REST API `127.0.0.1:11212`, health API `127.0.0.1:11313`
- Cleanup policy: interval `1`, peer timeout `1`, `remove_peerless_torrents = false`

## Test Results

### Deterministic cancellation

- [x] `cargo test -p torrust-tracker --lib bootstrap::jobs::torrent_cleanup::tests` passed both injected-token cancellation and weak-manager expiry cases.
- [x] The cancellation test awaits the unspawned runner and observes `Completion::Cancelled`.
- [x] No deterministic test delivers an OS signal; cancellation is injected through `CancellationToken`.

### Application wiring

- [x] `src/app.rs` derives the cleanup component token from `JobManager` and registers the unspawned runner through `component_runner`.
- [x] `rg 'ctrl_c' src/bootstrap/jobs/torrent_cleanup.rs` returned no matches.
- [x] `cargo test -p torrust-tracker --lib app::tests::it_should_register_torrent_cleanup_as_a_direct_cancelled_component` passed.

### Repository checks

- [x] `cargo test -p torrust-tracker --lib`: 91 passed, 0 failed.
- [x] `cargo fmt --check` and `git diff --check` passed.
- [x] `linter all` passed.

## Manual Evidence

### Direct-binary SIGTERM

Started `./target/debug/torrust-tracker` with `TORRUST_TRACKER_CONFIG_TOML_PATH` set to the isolated configuration. `pgrep` and `ss` confirmed direct executable PID `268438` owned UDP `16969` and TCP `11212` and `11313`; `kill -TERM 268438` delivered the signal to that process.

The runtime log recorded `Stopping torrent cleanup job ...`, followed by `Job completed after cooperative cancellation job=torrent_cleanup`. It contains no cleanup deadline-expiry or `torrent_cleanup` abort outcome. The process then logged `Torrust tracker successfully shutdown.` `pgrep` and `ss` found no remaining process or listener on the isolated ports.

`peers_inactivity_update` was disabled by this isolated configuration, so this run makes no claim about SI-5.

### Inactive-peer cleanup

Used info hash `2169216921692169216921692169216921692169` and announced it with the unified UDP `tracker_client`. The immediate authenticated REST response from `GET /api/v1/torrent/{info_hash}` contained one peer. After `linter all` ran for approximately 18 seconds, the same endpoint returned HTTP 200 for the same torrent with `"peers": []`.

The runtime log continuously recorded `Cleaning up torrents (executed every 1 secs) ...`; after removal it recorded `torrents=1 downloads=0 seeders=0 leechers=0` and `peerless_torrents=1 peers=0`. This confirms the peer was removed while the torrent remained, as required by `remove_peerless_torrents = false`.
