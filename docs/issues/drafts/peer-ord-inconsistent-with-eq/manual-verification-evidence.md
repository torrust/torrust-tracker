---
doc-type: manual-verification-evidence
issue-spec: docs/issues/drafts/peer-ord-inconsistent-with-eq/ISSUE.md
last-updated-utc: 2026-11-10 11:55
---

# Manual Verification Evidence

## Environment and Prerequisites

- Date and time (UTC): 2026-11-10 ~11:50
- Artifact under test: `torrust/develop` at `eaae0e487` (unmodified `peer.rs` plus the temporary
  test below)
- Operating system / environment: Linux 7.0.0, rustc 1.99.1, cargo 1.99.1 (default toolchain)
- Setup: none; `Peer` is a library type with no service surface, so the test binary is the artifact.

## V1 - Reproduce: `eq` false but `cmp` Equal

- Goal: observe the wrong outcome.
- Initial state: clean `develop`.
- Status: `DONE`
- Outcome: **Reproduced**

### Steps Performed

1. Appended this temporary test to `packages/primitives/src/peer.rs` (reverted afterwards):

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

1. `cargo test -p torrust-tracker-primitives --lib throwaway_repro -- --nocapture`

### Observed Result

```text
running 1 test
a == b: false
a.cmp(&b): Equal
test peer::throwaway_repro::ord_is_inconsistent_with_eq ... ok
```

### Conclusion

The test passes only because it asserts the buggy outcome: `a != b` yet `cmp` is `Equal`. This
violates the `Ord`/`Eq` contract. The temporary test was removed (`git status` clean for
`peer.rs`).

## V2 - Final like-for-like recheck

Status: `TODO` (after the fix).
