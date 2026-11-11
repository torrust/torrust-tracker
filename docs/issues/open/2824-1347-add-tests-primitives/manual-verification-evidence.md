---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2824-1347-add-tests-primitives/ISSUE.md
last-updated-utc: "2026-11-11 14:35"
---

# Manual Verification Evidence

## Purpose

Record real verification of the slice S1 fix (finding F2, `PeerRole` JSON casing). Nothing
here is invented: every output below was copied from the runs listed, on 2026-11-11.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-11, runs between 14:30:15 and 14:30:21.
- Artifact under test: the patches `test(primitives): [#2824] pin the lowercase JSON form of
  PeerRole` and `fix(primitives): [#2824] serialize PeerRole in lowercase`, and the follow-up
  `test(primitives): [#2824] compare the PeerRole JSON with its Display form` (patches named by
  subject; each run used a detached checkout of the commit with that subject).
- Operating system / environment: Linux 7.0.0-43-generic; `rustc 1.99.1 (58cf8f9c3 2026-10-13)`,
  `cargo 1.99.1 (5f94df478 2026-08-27)` (stable toolchain).
- Prerequisites: the runs shared one cargo target directory; the red and green runs used a
  temporary detached `git worktree` that was removed afterwards.

## Verification Processes

### V1 - F2 reproduction at the nearest seam

- Goal: show that `PeerRole` serialized as `"Seeder"` and `"Leecher"` while `Display` and the
  `peer_role` metric label (`swarm-coordination-registry` `statistics/event/handler.rs`) use
  `seeder` and `leecher`.
- Classification: nothing in production serializes a `PeerRole` (no struct in the workspace holds
  one; `grep` over `packages` and `src` finds `PeerRole` only in `peer.rs` and that handler), so the
  reproduction is the nearest-seam unit test on `serde_json::to_value`. A throwaway test on
  2026-11-10 (not kept) gave the same `"Seeder"` result.
- Status: `DONE`

#### Steps Performed

1. `git checkout --detach` the commit with subject `test(primitives): [#2824] pin the lowercase
   JSON form of PeerRole` (the old attribute `#[serde(rename_all_fields = "lowercase")]` is still
   present), then `cargo test -p torrust-tracker-primitives --lib peer_role_serialization`
   (started 14:30:15.347, ended 14:30:15.814).

#### Observed Result

```text
== red: test(primitives): [#2824] pin the lowercase JSON form of PeerRole

running 2 tests
test peer::test::peer_role_serialization::it_should_serialize_a_seeder_in_lowercase_like_its_display_format ... FAILED
test peer::test::peer_role_serialization::it_should_serialize_a_leecher_in_lowercase_like_its_display_format ... FAILED
...
  left: String("Seeder")
 right: String("seeder")
...
  left: String("Leecher")
 right: String("leecher")
...
test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 47 filtered out; finished in 0.00s
```

#### Conclusion

Both tests fail on the test commit for the reason F2 describes, so they reproduce the defect.

### V2 - The fix turns both tests green

- Goal: show the one-attribute change fixes the reproduction without breaking other unit tests.
- Status: `DONE`

#### Steps Performed

1. `git checkout --detach` the commit with subject `fix(primitives): [#2824] serialize PeerRole in
   lowercase`, then `cargo test -p torrust-tracker-primitives --lib` (started 14:30:20.151, ended
   14:30:20.435).

#### Observed Result

```text
== green: fix(primitives): [#2824] serialize PeerRole in lowercase
test peer::test::peer_role_serialization::it_should_serialize_a_leecher_in_lowercase_like_its_display_format ... ok
test peer::test::peer_role_serialization::it_should_serialize_a_seeder_in_lowercase_like_its_display_format ... ok
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

#### Conclusion

Both regression tests pass and all 49 unit tests pass (47 before the slice, plus the 2 new ones).

### V3 - Recheck at the branch head

- Goal: confirm the result still holds after the review fix that also compares the JSON value
  with the `Display` string.
- Status: `DONE`

#### Steps Performed

1. On the branch head, the commit with subject `test(primitives): [#2824] compare the PeerRole JSON
   with its Display form`, run `cargo test -p torrust-tracker-primitives` (ended 14:30:20.814).

#### Observed Result

```text
test peer::test::peer_role_serialization::it_should_serialize_a_leecher_in_lowercase_like_its_display_format ... ok
test peer::test::peer_role_serialization::it_should_serialize_a_seeder_in_lowercase_like_its_display_format ... ok
test result: ok. 49 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

#### Conclusion

The unit tests (49) and the doctests (2 + 1) pass at the head.

## Failures and Follow-up

None. The EPIC "Latest" values and the slice coverage measurement are recorded in
`coverage-evidence.md` and the EPIC for this slice.
