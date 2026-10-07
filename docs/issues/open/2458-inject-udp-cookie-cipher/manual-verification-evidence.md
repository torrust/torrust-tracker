---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2458-inject-udp-cookie-cipher/ISSUE.md
last-updated-utc: "2026-10-07 09:02"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the completed behavior. This is
evidence from commands or interactions actually performed against the artifact;
do not invent commands, output, logs, or results.

## Preserving Verification Artifacts

`.tmp/` and other git-ignored paths are not part of the repository, so a path
there is not evidence once the run ends. Record everything a reviewer needs in
this issue folder:

- Embed small artifacts, such as a tracker configuration file, verbatim in
  this document.
- Store larger artifacts, such as full logs or captured responses, in an
  `evidence/` subfolder of the issue folder and link them from here.
- Record runtime outputs (database rows, responses, relevant log lines) inline
  in each process.

A local `.tmp/` path may still be mentioned to show where a run wrote its
files, but never as the only copy of something the evidence relies on.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-06 16:14
- Artifact under test: `develop` at `9be79fc5a` ("Merge torrust/torrust-tracker#2452: fix(udp-server): [#2245] review numeric protocol wire conversions") with an uncommitted, reversible local mutation of `udp-core`. Both the 16:14 UTC run and the 17:06 UTC rerun used this code: the specification branch was created from `9be79fc5a`, and its own commits change only documentation.
- Operating system / environment: Linux x86_64.
- Toolchain: Rust 1.99.0 (`b940084d7`, LLVM 23.1.1), default Cargo development profile.
- Prerequisites and setup performed: changed the non-test `CURRENT_CIPHER` alias from `RANDOM_CIPHER_BLOWFISH` to `ZEROED_TEST_CIPHER_BLOWFISH`; restored it after this verification. Added and removed an untracked `udp-core` example that independently encrypted the cookie plaintext with the all-zero Blowfish key.

## Verification Processes

Add one section per manual verification process. A process may cover one or
more issue-spec scenarios when its steps and evidence clearly identify each
result.

### V1 - M2a: Production-Mode Safeguard Bypass and All-Zero-Key Forgery

- Goal: Verify that a production alias accidentally selecting the public all-zero cipher is not detected by `check_seed()`, and that a cookie independently forged with that key is accepted.
- Initial state: The production alias was temporarily mutated to select `ZEROED_TEST_CIPHER_BLOWFISH`; the all-zero cipher is initialized by `torrust_tracker_udp_core::initialize_static()`. The independent example used a fingerprint of `0x7a31_0000_0000_0001` and issue time `1_728_000_000.0`.
- Status: `DONE` — **Reproduced**.

#### Steps Performed

1. Ran `cargo run -p torrust-tracker-udp-core --example forge_zeroed_cookie`. The temporary example encrypted the documented cookie plaintext with its own `BlowfishLE::new_from_slice(&[0_u8; 32])`, constructed a protocol connection ID, and called the production `check` API with a one-second valid range.
2. Ran `timeout --signal=TERM 20s cargo run` with the same non-test alias mutation. The timeout deliberately sent `SIGTERM` after startup; exit status `124` therefore denotes timeout rather than a startup failure.
3. Restored the production alias to `RANDOM_CIPHER_BLOWFISH` and removed the temporary example without staging either mutation.
4. On 2026-10-06 17:06 UTC, in response to review finding `review-finding:pr-2460-f1`, recreated the exact mutation and example shown in [Temporary Code](#temporary-code-verbatim) and ran the example again, then restored the alias and ran it once more as a control. Both temporary changes were reverted afterwards, and `git status --short` was empty.

#### Temporary Code (Verbatim)

The non-test alias mutation in `packages/udp-core/src/crypto/keys.rs`, as `git diff` printed it on the rerun, without the file header lines and without the hunk's last context line, which is blank (line 135). The hunk header is adjusted from `-129,7 +129,7` to match the six lines shown:

```diff
@@ -129,6 +129,6 @@ mod detail_seed {
 mod detail_cipher {
     #[allow(unused_imports)]
     #[cfg(not(test))]
-    pub use crate::crypto::ephemeral_instance_keys::RANDOM_CIPHER_BLOWFISH as CURRENT_CIPHER;
+    pub use crate::crypto::ephemeral_instance_keys::ZEROED_TEST_CIPHER_BLOWFISH as CURRENT_CIPHER;
     #[cfg(test)]
     pub use crate::crypto::ephemeral_instance_keys::ZEROED_TEST_CIPHER_BLOWFISH as CURRENT_CIPHER;
```

The disposable example `packages/udp-core/examples/forge_zeroed_cookie.rs`. It builds the cookie plaintext the way `connection_cookie::cookie_builder::assemble` does, encrypts it with its own all-zero-key Blowfish instance, so it never uses the tracker cipher, and passes the result to the production `check` function:

```rust
use blowfish::BlowfishLE;
use cipher::{Block, BlockCipherEncrypt, KeyInit};
use torrust_tracker_udp_core::connection_cookie::check;
use torrust_tracker_udp_protocol::ConnectionId;

fn main() {
    let fingerprint = 0x7a31_0000_0000_0001_u64;
    let issue_at = 1_728_000_000.0_f64;
    let plaintext = (i64::from_ne_bytes(issue_at.to_ne_bytes()).wrapping_add(fingerprint as i64)).to_ne_bytes();
    let mut ciphertext = Block::<BlowfishLE>::from(plaintext);
    let cipher = BlowfishLE::new_from_slice(&[0_u8; 32]).expect("the fixed test key is a valid Blowfish key");
    cipher.encrypt_block(&mut ciphertext);
    let forged_cookie = ConnectionId::new(i64::from_be_bytes(ciphertext.as_slice().try_into().expect("a cookie is eight bytes")));

    let accepted_issue_time =
        check(&forged_cookie, fingerprint, (issue_at - 1.0)..(issue_at + 1.0)).expect("the forged cookie should validate");

    println!("forged cookie accepted for issue time {accepted_issue_time}");
}
```

`ConnectionId` stores a network-endian `I64`, so the ciphertext bytes are read with `i64::from_be_bytes` to keep them unchanged when `check` reads them back.

#### Observed Result

```text
$ cargo run -p torrust-tracker-udp-core --example forge_zeroed_cookie
Finished `dev` profile [optimized + debuginfo] target(s) in 0.89s
Running `target/debug/examples/forge_zeroed_cookie`
forged cookie accepted for issue time 1728000000

$ timeout --signal=TERM 20s cargo run
2026-10-06T16:14:06.606127Z  INFO ... HTTP TRACKER: Started on: http://0.0.0.0:7171
2026-10-06T16:14:06.606293Z  INFO ... API: Started tracker API service_binding=http://0.0.0.0:1212/
2026-10-06T16:14:06.606433Z  INFO torrust_tracker: Tracker shutdown signal handlers installed.
2026-10-06T16:14:15.650958Z  INFO torrust_tracker: Torrust tracker shutting down (SIGTERM) ...

$ # 2026-10-06 17:06 UTC rerun with the verbatim code above, alias mutated
$ cargo run -q -p torrust-tracker-udp-core --example forge_zeroed_cookie
forged cookie accepted for issue time 1728000000

$ # Control: alias restored to RANDOM_CIPHER_BLOWFISH, same example
$ cargo run -q -p torrust-tracker-udp-core --example forge_zeroed_cookie
thread 'main' (2400272) panicked at packages/udp-core/examples/forge_zeroed_cookie.rs:16:80:
the forged cookie should validate: ValueExpired { expired_value: -1.2038401961111392e-290, min_value: 1727999999.0 }
```

#### Conclusion

The mutated production artifact started normally; `check_seed()` did not panic because it compares only the unrelated seed. The independently generated all-zero-key cookie was accepted for the intended fingerprint and time. The control run shows that the same cookie is rejected when production uses the random cipher, so the acceptance comes from the mutation, not from a flaw in the example. This is a real-artifact reproduction of the missing production guarantee. A network-level `UdpTrackerClient::send` request was unnecessary to establish the causal security outcome: the production-mode `check` seam accepted a connection ID forged without using the tracker cipher.

## Regression-Test Design

The selected boundaries and their rationale are in the specification's Regression Test Strategy.
This section records the runs.

### R2 - Red Before the Fix (T3)

- Date and time (UTC): 2026-10-07 08:59.
- Code under test: `develop` at `7836471b3` ("Merge torrust/torrust-tracker#2461: docs(issues): [#2458] add cookie-cipher injection specification"), with the temporary doctests below added to the module documentation of `packages/udp-core/src/crypto/ephemeral_instance_keys.rs`. No production code was changed.
- Toolchain: stable Rust 1.99.0 (`b940084d7`), Cargo 1.99.0.
- Why a doctest: rustdoc builds the library without `cfg(test)`, as any production build or downstream crate sees it. The expected error code is pinned (`E0432`, unresolved import, which is what removing the public static produces) so an unrelated compile error, such as a typo, cannot make the test pass. The companion doctest compiles against the production key, so the pair shows the test can tell the two keys apart.

Temporary doctests, verbatim:

````rust
//! A production build must not be able to reach the fixed test key:
//!
//! ```rust,compile_fail,E0432
//! use torrust_tracker_udp_core::crypto::ephemeral_instance_keys::ZEROED_TEST_CIPHER_BLOWFISH;
//!
//! let _test_key = &*ZEROED_TEST_CIPHER_BLOWFISH;
//! ```
//!
//! A production build can reach the random key:
//!
//! ```rust
//! use torrust_tracker_udp_core::crypto::ephemeral_instance_keys::RANDOM_CIPHER_BLOWFISH;
//!
//! let _production_key = &*RANDOM_CIPHER_BLOWFISH;
//! ```
````

Command and output (cargo's file-lock wait lines removed):

```text
$ cargo test --doc -p torrust-tracker-udp-core ephemeral_instance_keys
   Compiling torrust-tracker-udp-core v0.1.0 (.../packages/udp-core)
    Finished `test` profile [optimized + debuginfo] target(s) in 1.33s
   Doc-tests torrust_tracker_udp_core

running 1 test
test packages/udp-core/src/crypto/ephemeral_instance_keys.rs - crypto::ephemeral_instance_keys (line 16) ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.00s


running 1 test
test packages/udp-core/src/crypto/ephemeral_instance_keys.rs - crypto::ephemeral_instance_keys (line 8) - compile fail ... FAILED

failures:

---- packages/udp-core/src/crypto/ephemeral_instance_keys.rs - crypto::ephemeral_instance_keys (line 8) stdout ----
Test compiled successfully, but it's marked `compile_fail`.

failures:
    packages/udp-core/src/crypto/ephemeral_instance_keys.rs - crypto::ephemeral_instance_keys (line 8)

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

all doctests ran in 0.75s; merged doctests compilation took 0.54s
error: doctest failed, to rerun pass `-p torrust-tracker-udp-core --doc`
```

Conclusion: **red**. The fixed test key is reachable from a production build today. The temporary
doctests were then removed with `git checkout`, so no production file changed. They are not
committed in red form, because the pre-commit gate runs `cargo test --doc`. The fix (T4) commits R2
against the new key type: the `compile_fail` doctest then names the test-only constructor, and the
companion doctest names the production constructor.

## Failures and Follow-up

The first version of the disposable example built the protocol connection ID with `i64::from_ne_bytes` instead of `i64::from_be_bytes` and failed with `ValueFromFuture`. Correcting it to network-endian bytes produced the accepted forged cookie above. This confirmed the independent forgery construction rather than weakening the result. The disposable example and mutation were removed; no temporary verification source remains in the worktree. Their final code is recorded verbatim in [Temporary Code](#temporary-code-verbatim).
