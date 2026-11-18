---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2907-2392-remove-leaked-test-temporary-files/ISSUE.md
last-updated-utc: "2026-11-18 11:55"
---

# Manual Verification Evidence

## Purpose

Record the controlled reproduction made while drafting this issue (V1): which files one full test
run leaves in a fresh temporary directory, its repetition on the `develop` the work started from
(T1), the focused red and green runs of T3 and T4, and the recheck after the change (V2).
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

Reproduced. The run left 2 TOML files, not the 1 that V1 counted: both
`configuration_should_be_saved_in_a_toml_config_file` tests, in `v2_0_0` and in `v3_0_0`, leave one.
T3 found why the count varies between runs (see "T3 - Configuration-file tests"): the count is a
lower bound. The PEM leak still needs a failing TLS test and was not triggered.

### T3 - Configuration-file tests

- Goal: the two `configuration_should_be_saved_in_a_toml_config_file` tests remove their file when
  they end, also when they fail.
- Change: each test writes `tracker.toml` inside a `tempfile::tempdir()` it owns, instead of
  `test_config_<uuid>.toml` in `std::env::temp_dir()`. `tempfile` replaces `uuid` as the
  configuration package's only dev-dependency; no other test used `uuid`.
- Date and time (UTC): 2026-11-18 10:05:46 to 10:05:58
- Toolchain: stable, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`
- Status: `DONE`

#### Focused Red and Green

The same command with a fresh `TMPDIR`, before the change (the working tree restored to `HEAD`)
and after it:

```bash
T=$(mktemp -d)
TMPDIR=$T cargo +stable test -p torrust-tracker-configuration --all-features
ls -A "$T"
```

| Run | Tests | Entries left in `TMPDIR` |
| --- | --- | --- |
| Before | 159 passed, 0 failed | 1 (`test_config_950c826e-ffe7-4f41-b137-a9e7a6170d0e.toml`) |
| After | 159 passed, 0 failed | 0 |

Run alone (`cargo +stable test -p torrust-tracker-configuration --all-features <module>::tests::configuration_should_be_saved_in_a_toml_config_file`),
each of the old `v2_0_0` and `v3_0_0` tests left 1 file.

#### Why a Full Run Counts Fewer Files Than the Tests That Leak

Eighteen tests in `packages/configuration/src/lib.rs` call `figment::Jail::clear_env()`, one call
each, at lines 438, 462, 483, 506, 528, 555, 579, 604, 649, 671, 697, 734, 759, 786, 814, 838, 866
and 892 (`grep -n 'clear_env()' packages/configuration/src/lib.rs`; no other file of the package
calls it). The first version of this record said five tests and named only the first five lines;
review finding F4 of PR 2938 corrected it. It removes every environment variable of the process, `TMPDIR`
included, until the jail ends (`figment-0.10.19/src/jail.rs:320`). `Jail` serializes only the tests
that use `Jail`. A test that runs in parallel at that moment and calls `std::env::temp_dir()` gets
`/tmp` instead of `TMPDIR`. So the old tests sometimes leaked into the shared `/tmp`, outside the
directory the evidence counts: V1 counted 1 file, T1 counted 2, and the focused run above 1.

The new tests do not depend on this: `TempDir` keeps the absolute path it created and removes that
directory on drop, wherever `std::env::temp_dir()` pointed. The environment race itself is outside
this issue's scope; see "Failures and Follow-up".

#### Test-Design Review (Prose-First Arrange-Act-Assert)

Temporary prose written before refactoring, compared with the code, then removed:

- Arrange: the test owns an empty temporary directory, removed when the test ends, and picks a
  configuration file path inside it that does not exist yet.
- Act: save the default configuration to that path.
- Assert: the file holds the default configuration as persisted TOML, specified independently by
  `default_config_toml()` (`v2_0_0`) or `default_persisted_config_toml()` (`v3_0_0`).

Result: the code states each paragraph directly, so the prose was removed. One comment stays in
Arrange, because the code alone does not say why the test uses a temporary directory: it is removed
with the file when the test ends, also when it fails. The one causal initial state is "the path does
not exist yet"; there is no fixture; the production Act (`save_to_file`) and the expected TOML are
visible. The old test's leftover comments and the `default_configuration` variable that only named
`Configuration::default()` were removed.

### T4 - TLS test files

- Goal: the PEM files of the TLS tests are removed when the test ends, also when it fails (AC2,
  AC3).
- Change: `make_temp_file` in `packages/axum-server/src/tls.rs` returns a `tempfile::NamedTempFile`
  (built with `tempfile::Builder::new().prefix(prefix).suffix(".pem")`) instead of a path. The test
  keeps the guard, and the two `fs::remove_file` calls that only ran when `make_rust_tls` returned
  the expected error are gone. `tempfile` is a new dev-dependency of `torrust-tracker-axum-server`.
- New tests: `it_should_remove_the_temporary_file_when_its_guard_is_dropped` (AC2) and
  `it_should_remove_the_temporary_file_when_the_test_using_it_fails` (AC3).
- Toolchain: stable, `rustc 1.99.1 (58cf8f9c3 2026-10-13)`; `cargo +nightly fmt` for formatting.
- Status: `DONE`

#### Red Before the Change

Two red runs are recorded: the first, for AC3, against the old helper before the change; the
second, after review finding F5 of PR 2938, against a mutated copy of the new helper, so that both
maintained tests have a red run of their own code.

##### Red 1: a variant of the AC3 test against the old helper

At 2026-11-18 10:08:37 UTC, the AC3 test was first written against the old helper, which returned a
`Utf8PathBuf` and had no guard. This variant was a temporary step: it was replaced by the
maintained AC3 test in the same task and is in no commit, so its code is recorded here:

```rust
#[test]
fn it_should_remove_the_temporary_file_when_the_test_using_it_fails() {
    let mut path = None;
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let file = make_temp_file("failing-test", "content");
        path = Some(file.clone());
        panic!("a test that fails while it holds the file");
    }));
    assert!(result.is_err());
    assert!(!path.unwrap().exists());
}
```

```bash
T=$(mktemp -d)
TMPDIR=$T cargo +stable test -p torrust-tracker-axum-server --all-features --lib tls::tests
ls -A "$T"
```

Result: `it_should_remove_the_temporary_file_when_the_test_using_it_fails ... FAILED` at its final
assertion, `assert!(!path.unwrap().exists())` (line 105 of the file as it was during this run), 2
other tests passed, and `TMPDIR` held `failing-test-1794996525401726262.pem`.

##### Red 2: the maintained tests against a helper that keeps its file

<!-- cspell:ignore testeh Xiht -->

At 2026-11-18 11:54:15 UTC, after the change, `make_temp_file` was mutated so that its guard keeps
the file on drop, the same command ran, and the mutation was reverted with
`git checkout -- packages/axum-server/src/tls.rs`. The mutation:

```diff
         let mut file = Builder::new()
             .prefix(prefix)
             .suffix(".pem")
+            .disable_cleanup(true)
             .tempfile()
             .expect("it should create a temporary test file");
```

Result, with the temporary directory written as `<TMPDIR>` and the line numbers of the mutated file:

```text
test tls::tests::it_should_remove_the_temporary_file_when_its_guard_is_dropped ... FAILED
test tls::tests::it_should_error_on_missing_cert_or_key_paths ... ok
test tls::tests::it_should_remove_the_temporary_file_when_the_test_using_it_fails ... FAILED
test tls::tests::it_should_error_on_bad_tls_config ... ok
thread 'tls::tests::it_should_remove_the_temporary_file_when_its_guard_is_dropped' (2124036) panicked at packages/axum-server/src/tls.rs:113:9:
<TMPDIR>/droppedW7A3nD.pem should have been removed
thread 'tls::tests::it_should_remove_the_temporary_file_when_the_test_using_it_fails' (2124037) panicked at packages/axum-server/src/tls.rs:125:13:
thread 'tls::tests::it_should_remove_the_temporary_file_when_the_test_using_it_fails' (2124037) panicked at packages/axum-server/src/tls.rs:131:9:
<TMPDIR>/failing-testehXiht.pem should have been removed
test result: FAILED. 2 passed; 2 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
```

Both maintained tests failed at their final assertion (the panic at line 125 is the expected panic
of the AC3 test's stand-in failing test). `TMPDIR` held `bad-certEVyNZS.pem`, `bad-keySM8xfo.pem`,
`droppedW7A3nD.pem` and `failing-testehXiht.pem`. After the revert, at 2026-11-18 11:54:16 UTC, the
same command passed 4 tests and left `TMPDIR` empty.

#### Green After the Change

At 2026-11-18 10:09:01 UTC, the same command: 4 passed, 0 failed
(`it_should_error_on_bad_tls_config`, `it_should_error_on_missing_cert_or_key_paths` and the two new
tests), and `TMPDIR` was empty. `cargo +stable clippy -p torrust-tracker-axum-server
-p torrust-tracker-configuration --all-targets --all-features -- -D warnings` exited `0`.

The AC3 test prints the expected panic message of its stand-in failing test
(`a test that fails while it holds the file`) in the captured output; the test still passes. It does
not install a panic hook to hide the message, because a hook is process-global state.

#### Test-Design Review (Prose-First Arrange-Act-Assert)

`it_should_remove_the_temporary_file_when_its_guard_is_dropped`:

- Arrange: a temporary PEM file exists, held by the guard `make_temp_file` returned; remember its
  path.
- Act: drop the guard.
- Assert: nothing exists at that path any more.

`it_should_remove_the_temporary_file_when_the_test_using_it_fails`:

- Arrange: no file exists yet; prepare a place to remember the path the failing test creates.
- Act: run a stand-in test that creates a file through `make_temp_file` and panics while it holds
  the guard; catch the panic.
- Assert: the stand-in test did fail, and nothing exists at the path it created.

`it_should_error_on_bad_tls_config` (changed, not new): its Arrange still creates an invalid
certificate and key, its Act and Assert are unchanged; it only stops removing the files itself.

Result: the code states each paragraph, so the prose was removed. The causal initial state of each
new test is visible in the test: a held guard for the first, a panic while the guard is held for the
second. The helper hides only incidental mechanics (the builder, the write), and the expected result
("the path no longer exists") is checked against the file system, not derived from the helper.
`utf8_path` is a one-line conversion kept as a helper because the two `TlsConfig` fields need it
and its name says what it does.

### V2 - Recheck after the change

- Goal: one full test run with a fresh `TMPDIR` leaves no `test_config_*.toml` and no PEM file
  (AC1).
- Initial state: a fresh, empty `TMPDIR` made with `mktemp -d` in the session's scratch directory
  (`ls -A` printed 0 entries).
- Artifact under test: this issue's branch after the T4 commit
  (`test(axum-server): [#2907] remove the TLS test PEM files also when a test fails`), based on
  `develop` `90a7c7d3f`, with a clean working tree.
- Date and time (UTC): 2026-11-18 11:09:08 to 11:10:46
- Toolchain: `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1 (5f94df478 2026-08-27)`, Linux
  7.0.0-43-generic; the same command built the test binaries. No database driver test gates set.
- Status: `DONE`

#### Steps Performed

```bash
T=$(mktemp -d)
ls -A "$T" | wc -l
TMPDIR=$T cargo +stable test --tests --benches --examples --workspace --all-targets --all-features
find "$T" -maxdepth 1 -name 'test_config_*.toml' | wc -l
find "$T" -name '*.toml' | wc -l
find "$T" -name '*.pem' | wc -l
find "$T" -maxdepth 1 -name 'data_*.db' | wc -l
find "$T" -mindepth 1 -maxdepth 1 ! -name 'data_*.db' | wc -l
du -sh "$T"
rm -rf "$T"
```

Then, on the same tree: `linter all`.

#### Observed Result

| Measure | V1 repeated by T1 | V2 |
| --- | --- | --- |
| Suite exit status | `0` (4128 passed, 0 failed) | `0` (4152 passed, 0 failed) |
| `test_config_*.toml` left | 2 | **0** |
| Any `*.toml` left, at any depth | not counted | **0** |
| `*.pem` left, at any depth | 0 | **0** |
| `data_*.db` left (out of scope, #2924 and #2925) | 301 | 229 |
| Other entries left | 0 | 0 |
| Size of the directory | 15 MiB | 11 MiB |
| `linter all` | not run | exit `0` ("All linters passed") |

The test count grew and the `data_*.db` count fell because `develop` moved between the two runs
(`61e3fedcf` to `90a7c7d3f`): among other changes it merged #2923 (#2902), which keeps the
`tracker-core` test databases in temporary directories.

#### Conclusion

AC1 holds: the full suite leaves no `test_config_*.toml` and no PEM file in a fresh `TMPDIR`.

Two limits of this check, both covered elsewhere:

- The suite passes, so it never runs the failure path of a TLS test. AC3 is proven by
  `it_should_remove_the_temporary_file_when_the_test_using_it_fails` (red before, green after, T4).
- Because of the `figment::Jail::clear_env()` race (T3), a temporary file can be created under
  `/tmp` instead of `TMPDIR`, outside the counted directory. The new tests remove their files by the
  absolute path they created, so the race cannot make them leak, but a count of `TMPDIR` alone could
  not show a leak into `/tmp`. A search of `/tmp` for the old names during the run was not possible
  to isolate from other sessions on this machine, so it was not used.

## Failures and Follow-up

- `figment::Jail::clear_env()` empties the environment of the whole test process while a jail is
  active (T3); eighteen tests of `packages/configuration/src/lib.rs` call it. The #2394 audit's `inventory.md` classifies the `Jail` tests as isolated (G2) because
  they serialize on `Jail`'s lock, but tests in the same binary that do not use `Jail` are not
  serialized and can observe the cleared environment. Outside this issue's scope; reported to the
  EPIC #2392 coordinator on 2026-11-18.
- None for this issue: V2 passed (T5).

## Background Measurement

Not a controlled run, recorded for scale only: on the same machine on 2026-11-16 the shared `/tmp`
held 232,229 `data_*.db` files (about 10.6 GiB, modification dates from 2026-11-13 to
2026-11-16) and 1,590 `test_config_*.toml` files, counted with
`find /tmp -maxdepth 1 -name 'data_*.db' | wc -l` and the same command for the TOML files.
