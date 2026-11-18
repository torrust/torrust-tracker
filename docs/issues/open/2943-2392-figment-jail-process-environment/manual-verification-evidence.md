---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md
last-updated-utc: "2026-11-18 15:43"
---

# Manual Verification Evidence

## Purpose

Record the real-artifact reproduction of this issue (V1). It was made during the review of the
spec pull request (PR #2944), after the maintainer approved the spec at 13:23 UTC, not while
drafting as `fix-bug` asks. The recheck (V2) is added when the issue is implemented. This is evidence from commands actually performed; nothing
here is simulated.

## Preserving Verification Artifacts

The probe test was added to the working tree, run and reverted; it was never committed. Its code
is recorded verbatim below, with the commands and the relevant output lines.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-18 15:43:22 to 15:43:29; each step's start and end are below. A
  first run on 2026-11-18 between 14:38 and 14:39, without step times, gave the same result; its
  output is in "Failures and Follow-up".
- Artifact under test: `develop` at `2f249d7e0` plus this spec branch, which adds documentation
  only, so the code of `packages/configuration` is `develop`'s. The only code change was the
  temporary probe below.
- Operating system / environment: Linux 7.0.0-43-generic, stable Rust toolchain. Output of
  `rustc +stable -Vv` and `cargo +stable -V`:

  ```text
  rustc 1.100.0 (0ef1bc386 2026-11-09)
  binary: rustc
  commit-hash: 0ef1bc386059596dada8fb65b50425aab2ea637a
  commit-date: 2026-11-09
  host: x86_64-unknown-linux-gnu
  release: 1.100.0
  LLVM version: 23.1.5
  cargo 1.100.0 (3acc79c0c 2026-11-05)
  ```

- Prerequisites and setup performed: `PATH` set by the shell; `TMPDIR` set to a fresh empty
  directory created with `mktemp -d` and passed on the command line. The probe asserts that both
  are set before it starts counting.

## Verification Processes

### V1 - Reproduce

- Goal: show that a test of the configuration lib binary that does not use `Jail` reads an unset
  `PATH` or `TMPDIR`, or another current directory, while the `Jail` tests run.
- Initial state: a clean working tree; `PATH` and `TMPDIR` set in the parent environment.
- Status: `DONE` (baseline made during the spec review; T1 repeats it on its own base)
- Outcome (`fix-bug`): **Reproduced**. The wrong outcome itself was observed in 3 of 3 runs.

#### Steps Performed

1. 15:43:22.902 to 15:43:22.907 - Appended this module to `packages/configuration/src/lib.rs`;
   `git status --short` then listed only `packages/configuration/src/lib.rs`, as modified:

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

2. Ran this command three times, one run after the other, with `D` a fresh `mktemp -d`
   directory. Run 1: 15:43:22.915 to 15:43:25.472. Run 2: 15:43:25.474 to 15:43:27.577. Run 3:
   15:43:27.580 to 15:43:29.684.

   ```sh
   TMPDIR=$D cargo +stable test -p torrust-tracker-configuration --lib
   ```

3. 15:43:29.686 to 15:43:29.694 - Reverted the probe with
   `git checkout packages/configuration/src/lib.rs`; `git status --short`, run after the
   revert, printed nothing. `D` was still empty and was removed.

#### Observed Result

The `PROBE` line and the `test result` line of each run:

```text
test jail_probe::it_reads_the_environment_outside_a_jail ... FAILED
PROBE reads=2237209 path_unset=2346 tmpdir_unset=2393 dir_changed=7224
test result: FAILED. 159 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s
test jail_probe::it_reads_the_environment_outside_a_jail ... FAILED
PROBE reads=2318669 path_unset=2326 tmpdir_unset=2294 dir_changed=7337
test result: FAILED. 159 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s
test jail_probe::it_reads_the_environment_outside_a_jail ... FAILED
PROBE reads=2337901 path_unset=2327 tmpdir_unset=2349 dir_changed=7253
test result: FAILED. 159 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.00s
```

#### Conclusion

Reproduced. In every run the probe, which uses no `Jail`, read `PATH` unset, `TMPDIR` unset and a
changed current directory, each thousands of times in 2.2 to 2.3 million reads. The 159 existing
tests passed. The current directory changed about three times as often as the variables were
unset, which fits the counts in the spec: every jail changes the directory, and 18 of the 51
call `clear_env()`.

Limits of this probe:

- It loops for 2 seconds, longer than the other tests take, so the counts depend on timing. They
  show that the reads happen, not how often a real test is affected.
- Its two start assertions can themselves fail if a jail is active at that moment. That did not
  happen in these three runs.

## Failures and Follow-up

No failed or blocked process. V2 is recorded here when the issue is implemented.

The first run (2026-11-18, between 14:38 and 14:39 UTC, same probe, command and toolchain, on the
same `develop` code) did not record the start and end of its steps, so the run above replaced it
as the evidence. Its output, for comparison:

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
