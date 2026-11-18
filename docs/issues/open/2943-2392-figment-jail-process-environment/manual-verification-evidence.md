---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md
last-updated-utc: "2026-11-18 14:40"
---

# Manual Verification Evidence

## Purpose

Record the real-artifact reproduction made while drafting this issue (V1). The recheck (V2) is
added when the issue is implemented. This is evidence from commands actually performed; nothing
here is simulated.

## Preserving Verification Artifacts

The probe test was added to the working tree, run and reverted; it was never committed. Its code
is recorded verbatim below, with the commands and the relevant output lines.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-18, between 14:38 and 14:39
- Artifact under test: `develop` at `2f249d7e0` plus this spec branch, which adds documentation
  only, so the code of `packages/configuration` is `develop`'s. The only code change was the
  temporary probe below.
- Operating system / environment: Linux 7.0.0-43-generic, stable Rust toolchain
  `rustc 1.100.0 (0ef1bc386 2026-11-09)`, host `x86_64-unknown-linux-gnu`,
  `cargo 1.100.0 (3acc79c0c 2026-11-05)`
- Prerequisites and setup performed: `PATH` set by the shell; `TMPDIR` set to a fresh empty
  directory created with `mktemp -d` and passed on the command line. The probe asserts that both
  are set before it starts counting.

## Verification Processes

### V1 - Reproduce

- Goal: show that a test of the configuration lib binary that does not use `Jail` reads an unset
  `PATH` or `TMPDIR`, or another current directory, while the `Jail` tests run.
- Initial state: a clean working tree; `PATH` and `TMPDIR` set in the parent environment.
- Status: `DONE` (drafting baseline; T1 repeats it on its own base)
- Outcome (`fix-bug`): **Reproduced**. The wrong outcome itself was observed in 3 of 3 runs.

#### Steps Performed

1. Appended this module to `packages/configuration/src/lib.rs`:

   ```rust
   #[cfg(test)]
   mod jail_probe {
       use std::time::{Duration, Instant};

       #[test]
       fn it_reads_the_environment_outside_a_jail() {
           let initial_dir = std::env::current_dir().expect("the initial current directory");
           assert!(std::env::var_os("PATH").is_some(), "PATH must be set before the probe starts");
           assert!(std::env::var_os("TMPDIR").is_some(), "TMPDIR must be set before the probe starts");

           let (mut reads, mut path_unset, mut tmpdir_unset, mut dir_changed) = (0_u64, 0_u64, 0_u64, 0_u64);
           let start = Instant::now();
           while start.elapsed() < Duration::from_secs(2) {
               reads += 1;
               if std::env::var_os("PATH").is_none() {
                   path_unset += 1;
               }
               if std::env::var_os("TMPDIR").is_none() {
                   tmpdir_unset += 1;
               }
               if std::env::current_dir().ok().as_ref() != Some(&initial_dir) {
                   dir_changed += 1;
               }
           }

           eprintln!("PROBE reads={reads} path_unset={path_unset} tmpdir_unset={tmpdir_unset} dir_changed={dir_changed}");
           assert_eq!((path_unset, tmpdir_unset, dir_changed), (0, 0, 0));
       }
   }
   ```

2. Ran this command three times, one run after the other, with `D` a fresh `mktemp -d` directory:

   ```sh
   TMPDIR=$D cargo +stable test -p torrust-tracker-configuration --lib
   ```

3. Reverted the probe with `git checkout packages/configuration/src/lib.rs`; `git status --short`
   then printed nothing.

#### Observed Result

The `PROBE` line and the `test result` line of each run:

```text
test jail_probe::it_reads_the_environment_outside_a_jail ... FAILED
PROBE reads=2357147 path_unset=2814 tmpdir_unset=2738 dir_changed=8729
test result: FAILED. 159 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s
test jail_probe::it_reads_the_environment_outside_a_jail ... FAILED
PROBE reads=2292831 path_unset=2684 tmpdir_unset=2641 dir_changed=8230
test result: FAILED. 159 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s
test jail_probe::it_reads_the_environment_outside_a_jail ... FAILED
PROBE reads=2307090 path_unset=2969 tmpdir_unset=2842 dir_changed=8388
test result: FAILED. 159 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s
```

#### Conclusion

Reproduced. In every run the probe, which uses no `Jail`, read `PATH` unset, `TMPDIR` unset and a
changed current directory, each thousands of times in about 2.3 million reads. The 159 existing
tests passed. The current directory changed about three times as often as the variables were
unset, which fits the counts in the spec: every jail changes the directory, and 18 of the 51
call `clear_env()`.

Limits of this probe:

- It loops for 2 seconds, longer than the other tests take, so the counts depend on timing. They
  show that the reads happen, not how often a real test is affected.
- Its two start assertions can themselves fail if a jail is active at that moment. That did not
  happen in these three runs.

## Failures and Follow-up

None. V2 is recorded here when the issue is implemented.
