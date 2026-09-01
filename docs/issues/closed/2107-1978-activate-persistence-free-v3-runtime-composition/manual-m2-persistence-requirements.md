# M2 Missing-Persistence Requirement Verification

**Date:** 2026-08-29 10:43 UTC

## Scope

This verification exercised the active v3 bootstrap with no
`[core.database]`. It independently enabled each capability that requires
persistence and confirmed that setup rejected the configuration before
application composition or runtime-job startup.

## Configuration And Commands

Each run started from `.tmp/2107-no-persistence-verification.toml`, the complete
v3 configuration used for M1/M4. That configuration has no `[core.database]`
section, enables tracker usage statistics, and sets every persistence-required
capability to `false`. The commands changed exactly one setting for each run:

```text
TORRUST_TRACKER_CONFIG_TOML="$(sed 's/listed = false/listed = true/' .tmp/2107-no-persistence-verification.toml)" cargo run --bin torrust-tracker

TORRUST_TRACKER_CONFIG_TOML="$(sed 's/private = false/private = true/' .tmp/2107-no-persistence-verification.toml)" cargo run --bin torrust-tracker

TORRUST_TRACKER_CONFIG_TOML="$(sed 's/persistent_torrent_completed_stat = false/persistent_torrent_completed_stat = true/' .tmp/2107-no-persistence-verification.toml)" cargo run --bin torrust-tracker
```

## Observed Results

Each process loaded the complete v3 configuration and then stopped at
`src/bootstrap/app.rs:41`, where `setup` invokes the centralized
`validate_persistence_requirements` check before global services or
`AppContainer` construction.

```text
Configuration error: Configuration requires persistence for `core.listed`, but `[core.database]` is missing.

Configuration error: Configuration requires persistence for `core.private`, but `[core.database]` is missing.

Configuration error: Configuration requires persistence for `core.tracker_policy.persistent_torrent_completed_stat`, but `[core.database]` is missing.
```

No listener, tracker server, REST API, health API, persistence-driver, or
migration startup message appeared in any run. `git status --short` remained
limited to the pre-existing documentation formatting edits; the M2 processes
created no tracked workspace artifacts.

## Result

M2 passed. The active bootstrap emits a stable capability-specific diagnostic
for each missing-persistence configuration before application composition.
