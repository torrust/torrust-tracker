---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md
last-updated-utc: "2026-11-10 14:20"
---

# Manual Verification Evidence

## Purpose

Drafting-time reproduction of the defect, recorded before the spec review as `fix-bug` requires. Verification of the fixes (M1-M3 in the specification) is not started and will be added here by the fix slices.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-10, about 14:10.
- Artifact under test: `torrust/develop` at the commit this spec branch was cut from, with no local changes except the temporary files named below.
- Operating system / environment: Linux 7.0.0-43-generic, x86_64.
- Toolchain: stable `rustc 1.99.1 (58cf8f9c3 2026-10-13)`, `cargo 1.99.1`. The workspace sets only `rust-version = "1.88"`.
- Prerequisites and setup performed: none beyond a built workspace.

## Drafting-Time Reproduction

### V1 - `PeerRole` parse reports a fixed library line

- Goal: observe the location reported for `"x".parse::<PeerRole>()` and `PeerRole::from_str("x")`.
- Initial state: `packages/primitives/src/peer.rs` unmodified.
- Status: `DONE`
- Outcome: **Reproduced** (the wrong location was observed in the error text a user of the type would see).

#### Temporary test (reverted, kept verbatim)

File `packages/primitives/tests/tmp_location_repro.rs`, deleted after the run:

```rust
use std::str::FromStr;

use torrust_tracker_primitives::peer::PeerRole;

#[test]
fn via_parse() {
    let e = "x".parse::<PeerRole>().unwrap_err();
    println!("parse: {e}");
}

#[test]
fn via_from_str() {
    let e = PeerRole::from_str("x").unwrap_err();
    println!("from_str: {e}");
}
```

#### Steps Performed

1. Ran on the unmodified code (stable `rustc 1.99.1`):

   ```sh
   cargo test -p torrust-tracker-primitives --test tmp_location_repro -- --nocapture --test-threads=1
   ```

2. Added `#[track_caller]` to `PeerRole::from_str` only (a temporary edit, reverted with `git checkout packages/primitives/src/peer.rs`) and ran the same command.

#### Observed Result

Step 1, no attribute:

```text
test via_from_str ... from_str: invalid param x in packages/primitives/src/peer.rs:73:27
test via_parse ... parse: invalid param x in packages/primitives/src/peer.rs:73:27
```

Step 2, `#[track_caller]` on `from_str`:

```text
test via_from_str ... from_str: invalid param x in packages/primitives/tests/tmp_location_repro.rs:13:13
test via_parse ... parse: invalid param x in /home/josecelano/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/str/mod.rs:2783:9
```

#### Conclusion

Without the attribute both call shapes report the library line `peer.rs:73:27`. With it, a direct call reports the caller's line but the call through `str::parse` reports a line inside the standard library, so the attribute alone is a partial fix. The std path and line depend on the toolchain. The HTTP and UDP client-visible reproductions (M1, M2) are not attempted yet: they belong to the fix slices.
