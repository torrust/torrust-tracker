---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2798-peer-ord-inconsistent-with-eq/ISSUE.md
last-updated-utc: 2026-11-10 13:34
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is evidence from commands or interactions actually performed against the artifact; do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

`.tmp/` and other git-ignored paths are not part of the repository, so a path there is not evidence once the run ends. Everything this evidence relies on is embedded below: the temporary test is reproduced verbatim in V1 and its output is quoted inline. No larger artifacts exist, so there is no `evidence/` subfolder.

## Defect Under Test

- Symptom: for a seeder `a` and a copy `b` with `left = 5`, `a == b` is `false` but `a.cmp(&b)` is `Ordering::Equal`.
- Hypothesis: `impl Ord for Peer` compares `peer_id` only, while the derived `PartialEq`/`Eq` compare all seven fields.
- Regression-test boundary (planned, see the spec): a unit test in `packages/primitives/src/peer.rs` asserting that `cmp` is `Equal` exactly when `==`. It is the smallest deterministic maintained test at the causal seam.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-10, steps stamped below.
- Artifact under test: `torrust/develop` at `1df1ecdf0`, plus the Conventional Commit `docs(issues): draft spec for Peer Ord inconsistent with Eq` and its follow-ups on the PR branch (these change no Rust code), with the temporary test below appended to `peer.rs` and reverted.
- Operating system / environment: Linux 7.0.0; rustc 1.99.1 (58cf8f9c3 2026-10-13), cargo 1.99.1, default toolchain.
- Prerequisites and setup performed: none. `Peer` is a library type with no CLI or service surface for ordering, so the test binary is the real artifact.

## Verification Processes

### V1 - Reproduce: `eq` false but `cmp` Equal

- Goal: observe the wrong outcome.
- Initial state: `git diff --quiet HEAD -- packages/primitives/src/peer.rs` exited 0 before step 1 started.
- Status: `DONE`
- Outcome: **Reproduced**

#### Steps Performed

Dependent steps ran one at a time, in this order, each starting after the previous one ended. Stamps are UTC, taken with `date -u +%T.%3N` on 2026-11-10 around each step.

1. Start 13:33:30.331, end 13:33:30.334: appended this temporary test to `packages/primitives/src/peer.rs`:

   ```rust
   #[cfg(test)]
   mod throwaway_repro {
       use std::cmp::Ordering;
       use crate::peer::fixture::PeerBuilder;
       use crate::NumberOfBytes;

       #[test]
       fn ord_is_inconsistent_with_eq() {
           let a = PeerBuilder::seeder().build();
           let mut b = a;
           b.left = NumberOfBytes::new(5);
           println!("a == b: {}", a == b);
           println!("a.cmp(&b): {:?}", a.cmp(&b));
           assert!(a != b);
           assert_eq!(a.cmp(&b), Ordering::Equal, "cmp is Equal although eq is false");
       }
   }
   ```

2. Start 13:33:31.339, end 13:33:31.692: ran `cargo test -p torrust-tracker-primitives --lib throwaway_repro -- --nocapture`.
3. Start 13:33:32.696, end 13:33:32.703: reverted with `git checkout -- packages/primitives/src/peer.rs`. After it ended, `git diff --quiet HEAD -- packages/primitives/src/peer.rs` exited 0, so the file equals `HEAD` again.

#### Observed Result

```text
running 1 test
a == b: false
a.cmp(&b): Equal
test peer::throwaway_repro::ord_is_inconsistent_with_eq ... ok
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 47 filtered out; finished in 0.00s
```

#### Conclusion

The test passes only because it asserts the buggy outcome: `a != b` yet `cmp` is `Equal`. This violates the `Ord`/`Eq` contract, so the bug is reproduced.

### V2 - Final like-for-like recheck

- Status: `TODO` (after the fix, on the final commit; to be named by its Conventional Commit subject).

## Regression Test Red Run (T4)

Recorded 2026-11-10 16:39 UTC, on the unfixed code (`Peer` still implements `Ord`).

- Test: `compile_fail` doctest on `Peer` in `packages/primitives/src/peer.rs`. It calls
  `fn assert_ord<T: Ord>() {}` with `Peer`, so it compiles only while `Peer: Ord`.
- Command: `cargo test -p torrust-tracker-primitives --doc`

```text
test packages/primitives/src/peer.rs - peer::Peer (line 117) - compile fail ... FAILED
Test compiled successfully, but it's marked `compile_fail`.
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out
```

Design review (Arrange-Act-Assert, prose): the one causal difference is whether `Peer` is `Ord`.
There is no fixture. The Act is the production type used as the `Ord` bound, and the expected
result (compilation must fail) is specified independently of the implementation. The red result
is the contract violation itself: an `Ord` exists that cannot agree with `==`.

## Failures and Follow-up

No failed or blocked process so far. Red and green regression-test output will be recorded here when T4 to T6 run.
