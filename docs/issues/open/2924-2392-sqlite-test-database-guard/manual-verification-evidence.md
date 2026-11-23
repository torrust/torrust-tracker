---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md
last-updated-utc: "2026-11-23 16:15"
---

# Manual Verification Evidence

## Purpose

Record real verification of the behavior of #2924. Nothing here is invented.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-23 (V1 run, before any code change)
- Artifact under test: `torrust/develop` at `98fa27df2`, no code change yet
- Operating system / environment: Linux 7.0.0-43-generic; `cargo +stable` 1.100.0
- Prerequisites and setup performed: fresh directory from `mktemp -d` used as `TMPDIR`

## Verification Processes

### V1 - Reproduce

- Goal: count the `data_*.db` files the `http-core` test binaries leave in a fresh `TMPDIR`.
- Initial state: empty `TMPDIR`.
- Status: `DONE`

#### Steps Performed

1. `TMPDIR=$(mktemp -d) cargo +stable test -p torrust-tracker-http-core --all-targets --all-features`
2. `ls "$TMPDIR" | grep -c '^data_.*\.db$'`

#### Observed Result

```text
6   (six files named data_<16 random characters>.db; names not recorded because they are random)
```

#### Conclusion

The run leaves 6 files. The only direct caller of `ephemeral_sqlite_database()` in `http-core`
(`packages/http-core/src/container.rs`, test
`it_should_panic_when_the_configured_database_cannot_be_initialized`) points at a path whose parent does not exist, so it
creates no file. The 6 files therefore come from the `ephemeral*()` configurations, which #2925 moves. V2 can show no drop for
the converted helper; its value is AC3 (no caller outside `test-helpers` is left). Per-test attribution of the 6 files is not
done.

## Failures and Follow-up

None.
