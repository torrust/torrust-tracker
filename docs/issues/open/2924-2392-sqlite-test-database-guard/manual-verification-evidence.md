---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2924-2392-sqlite-test-database-guard/ISSUE.md
last-updated-utc: "2026-11-23 17:03"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands
actually performed; nothing here is invented.

## Preserving Verification Artifacts

The run wrote only files under a temporary directory. Everything a reviewer needs is embedded
below; the random file names are not recorded (they are random and fail the spell check).

## Environment and Prerequisites

- Date and time (UTC): 2026-11-23, 16:50 (V1 rerun with stamps)
- Artifact under test: `torrust/develop` at `98fa27df2`, no code change
- Operating system / environment: Linux 7.0.0-43-generic; `cargo +stable` 1.100.0
- Prerequisites and setup performed: a fresh directory from `mktemp -d`, exported as `TMPDIR`

## Verification Processes

### V1 - Reproduce

- Symptom: `data_<16 random characters>.db` files stay in `TMPDIR` after a test run.
- Hypothesis: the `http-core` direct caller of `ephemeral_sqlite_database()` leaves none, and the
  files come from the `ephemeral*()` configurations.
- Goal: count the `data_*.db` files the `http-core` test binaries leave in a fresh `TMPDIR`.
- Initial state: empty directory from `mktemp -d`.
- Outcome class: Reproduced for the `ephemeral*()` configurations (6 files); for the direct
  caller the observation is that it creates no file (hypothesis held; not isolated per test).
- Status: `DONE`

#### Steps Performed

The dependent steps ran one after another in one shell, from this throwaway script (the shebang line
was added after the run, the rest is what ran). It wrote the stamps below to `.tmp/v1r.times`:

```bash
#!/usr/bin/env bash
set -u
date -u +%T.%3N > .tmp/v1r.times
export TMPDIR=$(mktemp -d)
echo "TMPDIR=$TMPDIR" >> .tmp/v1r.times
echo "start $(date -u +%T.%3N)" >> .tmp/v1r.times
cargo +stable test -p torrust-tracker-http-core --all-targets --all-features > .tmp/v1r.log 2>&1
echo "cargo exit=$? end $(date -u +%T.%3N)" >> .tmp/v1r.times
echo "count=$(ls "$TMPDIR" | grep -c '^data_.*\.db$') at $(date -u +%T.%3N)" >> .tmp/v1r.times
```

1. `export TMPDIR=$(mktemp -d)` (16:50:39.277; directory `/tmp/tmp.XduT1kqYcK`)
2. `cargo +stable test -p torrust-tracker-http-core --all-targets --all-features`
   (start 16:50:39.281, end 16:50:40.653, exit 0, 191 tests passed in the first test binary reported;
   build was already cached)
3. `ls "$TMPDIR" | grep -c '^data_.*\.db$'` (16:50:40.695); the script appends it to `.tmp/v1r.times` as `count=<n>`.

An earlier run of the same command with a fresh directory, using `TMPDIR=$TD cargo ...` and
`ls $TD`, also counted 6; this run replaces its record because that earlier record did not
show the shell state it relied on.

#### Observed Result

```text
count=6 at 16:50:40.695
```

#### Conclusion

The run leaves 6 files. The only direct caller of `ephemeral_sqlite_database()` in `http-core` is
the test `it_should_panic_when_the_configured_database_cannot_be_initialized`
(`packages/http-core/src/container.rs`); it points at a path whose parent does not exist and
creates no file. The 6 files therefore come from the `ephemeral*()` configurations, which #2925
moves. V2 can show no drop for the converted test; its value is AC3. Per-test attribution of the 6
files is not done.

## Failures and Follow-up

The first record of V1 set `TMPDIR` for the `cargo` command only and was questioned in review
(PR #3011, F1); it was rerun as above.
