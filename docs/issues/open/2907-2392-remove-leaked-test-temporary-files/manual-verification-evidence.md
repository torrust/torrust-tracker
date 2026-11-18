---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
last-updated-utc: "2026-11-18 09:50"
---

# Manual Verification Evidence

## Purpose

Record the controlled reproduction made while drafting this issue (V1): which files one full test
run leaves in a fresh temporary directory. The recheck (V2) is added when the issue is implemented.
This is evidence from commands actually performed; nothing here is simulated.

## Preserving Verification Artifacts

The suite output went to a scratch log outside the repository, and the temporary directory was
removed after counting. The counts and the summary line a reviewer needs are recorded inline below.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-16 22:04:10 to 22:05:53
- Artifact under test: `develop` at `0373f05f8` merged into the #2394 audit branch; the branch adds
  documentation only, so the test code is `develop`'s
- Operating system / environment: Linux 7.0.0-43-generic, stable Rust toolchain
  `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1`; the test binaries were already built
- Prerequisites and setup performed: an empty directory from `mktemp -d` used as `TMPDIR`; no
  database driver test gates (`TORRUST_TRACKER_CORE_RUN_*_DRIVER_TEST`) set

## Verification Processes

### V1 - Files one full test run leaves behind

- Goal: count the temporary files one full test run leaves behind.
- Initial state: a fresh, empty `TMPDIR` (`ls -A` printed 0 entries).
- Status: `DONE`

#### Steps Performed

1. Create the directory and check that it is empty:

   ```bash
   T=$(mktemp -d)
   ls -A "$T" | wc -l
   ```

2. Run the pre-push hook's test step with `TMPDIR` pointed at it (stable toolchain):

   ```bash
   TMPDIR=$T cargo +stable test --tests --benches --examples --workspace --all-targets --all-features
   ```

3. Count what is left, then remove the directory:

   ```bash
   find "$T" -maxdepth 1 -name 'data_*.db' | wc -l
   find "$T" -maxdepth 1 -name 'test_config_*.toml' | wc -l
   find "$T" -maxdepth 1 -name '*.pem' | wc -l
   find "$T" -mindepth 1 -maxdepth 1 ! -name 'data_*.db' ! -name 'test_config_*.toml' ! -name '*.pem' | wc -l
   du -sh "$T"
   rm -rf "$T"
   ```

#### Observed Result

| Measure | Value |
| --- | --- |
| Suite exit status | `0` (4093 tests passed, 0 failed, summed over the `test result:` lines) |
| `data_*.db` left | 301 |
| `test_config_*.toml` left | 1 |
| `*.pem` left | 0 (every TLS test passed, so `make_temp_file`'s cleanup ran) |
| Other entries left | 0 |
| Size of the directory | 15 MiB |

#### Conclusion

Reproduced, as the issue's V1 expects: each full run leaves about 300 SQLite files and one TOML
file. The PEM leak needs a failing TLS test and was not triggered here.

### V1 repeated by T1 - On the `develop` the work starts from

- Goal: repeat V1 on `develop` before changing anything (Bug-Fix Process step 1).
- Initial state: a fresh, empty `TMPDIR` made with `mktemp -d` in the session's scratch directory
  (`ls -A` printed 0 entries).
- Artifact under test: `develop` at `61e3fedcf`. The work was later rebased on `90a7c7d3f`; neither
  `packages/configuration` nor `packages/axum-server` changed between the two
  (`git diff --stat 61e3fedcf 90a7c7d3f -- packages/configuration packages/axum-server` is empty).
- Date and time (UTC): 2026-11-17 17:18:12 to 17:19:54
- Toolchain: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, Linux
  7.0.0-43-generic; the same command built the test binaries.
- Status: `DONE`

#### Steps Performed

The same commands as V1: create the directory, run
`TMPDIR=$T cargo +stable test --tests --benches --examples --workspace --all-targets --all-features`,
count each kind of file with `find "$T" -maxdepth 1`, measure with `du -sh`, then remove it.

#### Observed Result

| Measure | Value |
| --- | --- |
| Suite exit status | `0` (4128 tests passed, 0 failed, summed over the `test result:` lines) |
| `test_config_*.toml` left | 2 |
| `*.pem` left | 0 (every TLS test passed) |
| `data_*.db` left (out of scope, #2924 and #2925) | 301 |
| Other entries left | 0 |
| Size of the directory | 15 MiB |

#### Conclusion

Reproduced. The run leaves 2 TOML files, not the 1 that V1 counted: both
`configuration_should_be_saved_in_a_toml_config_file` tests, in `v2_0_0` and in `v3_0_0`, leave one.
The PEM leak still needs a failing TLS test and was not triggered.

## Failures and Follow-up

None. V2 (the recheck after the fix) is recorded when the issue is implemented.

## Background Measurement

Not a controlled run, recorded for scale only: on the same machine on 2026-11-16 the shared `/tmp`
held 232,229 `data_*.db` files (about 10.6 GiB, modification dates from 2026-11-13 to
2026-11-16) and 1,590 `test_config_*.toml` files, counted with
`find /tmp -maxdepth 1 -name 'data_*.db' | wc -l` and the same command for the TOML files.
