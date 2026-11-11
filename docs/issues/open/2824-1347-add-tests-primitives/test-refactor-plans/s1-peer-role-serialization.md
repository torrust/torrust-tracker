---
doc-type: test-design-review
issue-spec: docs/issues/open/2824-1347-add-tests-primitives/ISSUE.md
scope: slice S1, the two `PeerRole` serialization tests in `packages/primitives/src/peer.rs`
last-updated-utc: "2026-11-11 14:37"
---

# S1 test design review: `peer_role_serialization`

Review recorded after round 1 of PR #2834 (Copilot's third finding), for the two tests added by the
commits `test(primitives): [#2824] pin the lowercase JSON form of PeerRole` and
`test(primitives): [#2824] compare the PeerRole JSON with its Display form`. The `peer.rs` decision
record of slice S2 reuses this review for these two tests and covers the rest of the file.

## The comparison (prose-first Arrange-Act-Assert)

The temporary prose was written for each test and the code was compared with it:

| Section | Prose | Code that expresses it |
| --- | --- | --- |
| Arrange | The only initial-state difference between the two tests is the role: a seeder in one, a leecher in the other. Nothing else is needed. | The variant is named inline in the Act: `PeerRole::Seeder` / `PeerRole::Leecher`. No fixture. |
| Act | Serialize the role to a JSON value, the way any serde consumer would. | `serde_json::to_value(PeerRole::<role>).unwrap()` |
| Assert | The value is the lowercase string for that role, written as a literal, and the same string `Display` prints. | `assert_eq!(serialized, json!("seeder"))` and `assert_eq!(serialized, json!(PeerRole::Seeder.to_string()))` |

Result of the comparison:

- **Causal initial state:** the role is the one difference that explains each expected value, and it
  is visible in the Act line.
- **Fixture:** none. There is no helper, because nothing incidental needs hiding.
- **Production Act and independent expectation:** the Act calls production code (`Serialize`
  derived on `PeerRole`); the literal is specified independently of the code under test, and the
  `Display` comparison ties the JSON to the other observable form of the role.
- **Redundant prose removed:** the temporary prose is not kept as comments, because the test names
  (`it_should_serialize_a_seeder_in_lowercase_like_its_display_format` and the leecher
  equivalent) and the two assertions carry it.
- **Two functions, not one parameterized test:** with two cases, one function per variant keeps
  each failure message pointing at the role that failed.

## Result

Both tests are `clean` after the review; no change to the tests came from it.
