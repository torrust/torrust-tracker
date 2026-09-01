---
semantic-links:
  related-artifacts:
    - docs/issues/closed/999-1978-optional-database-configuration/ISSUE.md
    - share/default/config/tracker.udp.benchmarking.toml
    - packages/tracker-core/migrations/sqlite/20240730183000_torrust_tracker_create_all_tables.sql
---

# Baseline end-to-end verification

## Purpose

Preserve a reproducible observation of the problem reported in #999 before any
solution is implemented. The Phase 3 implementation must repeat the final
scenario below and record that it no longer creates or initializes a database
when the equivalent v3 configuration omits `[core.database]` and no
persistence-backed capability is enabled.

## Baseline environment

- Date: 2026-08-25
- Revision: `f07553c0` (`develop` before this specification branch)
- Binary: `target/debug/torrust-tracker`, built by `cargo run --bin torrust-tracker`
- Configuration source: `share/default/config/tracker.udp.benchmarking.toml`
- Schema version: `2.0.0`
- Working directory: an isolated `.tmp/issue-999-baseline-*` directory
- Runtime limit: ten seconds; the tracker was stopped by `timeout` after
  remaining alive, so exit status `124` is expected.

The benchmarking configuration disables the known persistence settings:

```toml
[core]
listed = false
private = false
tracker_usage_statistics = false

[core.tracker_policy]
persistent_torrent_completed_stat = false
remove_peerless_torrents = false
```

## Baseline result

The active v2 runtime unconditionally initializes a database. This baseline
supplies an explicit SQLite section, and the tracker starts and creates a
49,152-byte SQLite database file despite the persistence settings above being
disabled:

```toml
[core.database]
driver = "sqlite3"
path = "./baseline.sqlite3.db"
```

Command:

```text
(cd "$work_dir" && \
  TORRUST_TRACKER_CONFIG_TOML_PATH="$work_dir/tracker.with-database.toml" \
  timeout --signal=INT --kill-after=3s 10s \
  "$repository_root/target/debug/torrust-tracker")
```

Observed output and artifacts:

```text
EXIT_STATUS=124
Loading extra configuration from file: `.../tracker.with-database.toml` ...

baseline.sqlite3.db 49152 bytes
```

The created database contains the current SQLite migration schema. A direct
SQLite inspection returned:

```text
_sqlx_migrations
keys
sqlite_sequence
torrent_aggregate_metrics
torrents
whitelist
```

This includes the `whitelist`, `torrents`, and `keys` tables defined by
`20240730183000_torrust_tracker_create_all_tables.sql` and shows that migrations
were applied.

## Missing-database control observation

Removing `[core.database]` from the equivalent v2 benchmarking configuration
does not provide a valid reproduction of the desired final state. The v2
`Core::database` field has a serde default, so the TOML section itself is not
mandatory: omission resolves to the default SQLite configuration. The active
runtime then still unconditionally constructs that default database and applies
migrations before it evaluates feature enablement.

In the recorded control run, the process remained alive after configuration
loading and provided no useful visible diagnostic at the `error` logging
threshold before the ten-second timeout. The control did not inspect the
default database location, so it does not independently prove whether that
location was created. This confirms why the implementation must preserve v2
behaviour and target v3 only; Phase 1 traces the precise v2 construction path
in `analysis.md`.

## Final implementation acceptance scenario

After Phase 3, run this scenario using the active v3 runtime path:

1. Create an isolated working directory and a v3 UDP benchmarking configuration
   with no `[core.database]` section.
2. Disable every persistence-backed capability identified and approved in the
   Phase 2 capability-validation matrix.
3. Start the tracker with a bounded timeout and capture logs.
4. Inspect the isolated working directory and any configured/default database
   locations.

Expected result:

- The tracker starts and remains alive until the bounded shutdown.
- No SQLite database file is created.
- No MySQL or PostgreSQL connection is attempted.
- No migration is executed.
- Logs contain no database initialization or migration activity.

Record the exact v3 configuration, command, timeout result, logs, artifact
inspection, and the commit or PR under test in this document. Mark the related
manual-verification scenario in `ISSUE.md` as `DONE` only after the evidence is
recorded.

## Final V3 No-Persistence Verification

- Date: 2026-08-29 10:57 UTC
- Revision: `05d88794` on
  `2107-activate-persistence-free-v3-runtime-composition`
- Binary: `target/debug/torrust-tracker`
- Working directory: new isolated `.tmp/2107-m5.bIaViE` directory

The verification derived its complete v3 configuration from
`share/default/config/tracker.udp.benchmarking.toml`. It removed only the
`[core.database]` table and replaced the UDP bind address with `127.0.0.1:0` to
avoid a fixed-port dependency. All persistence-backed capabilities remained
disabled.

```text
repository_root=$PWD
work_dir=$(mktemp -d .tmp/2107-m5.XXXXXX)
configuration=$(sed '/^\[core\.database\]$/,/^$/d; s|bind_address = "0.0.0.0:3000"|bind_address = "127.0.0.1:0"|' share/default/config/tracker.udp.benchmarking.toml)
(cd "$work_dir" && TORRUST_TRACKER_CONFIG_TOML="$configuration" timeout --signal=INT --kill-after=3s 10s "$repository_root/target/debug/torrust-tracker") >"$work_dir/tracker.log" 2>&1
```

Observed result:

```text
EXIT_STATUS=124
tracker.log 671 bytes
```

`124` is the expected status from the bounded run: the tracker remained alive
until `timeout` sent its interrupt. The captured configuration contained no
`[core.database]` table. The isolated directory contained only `tracker.log`;
no SQLite database file or other persistence artifact was created. At the
`error` logging threshold, the log emitted no database initialization,
connection, or migration message.

This verifies the final v3 baseline scenario for source-tree runtime behavior.
Supported-container verification remains M6.
