---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2458-inject-udp-cookie-cipher/ISSUE.md
last-updated-utc: 2026-10-06 16:17
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
- Artifact under test: uncommitted, reversible local mutation of `udp-core` on top of `develop`; no branch commit exists yet.
- Operating system / environment: Linux x86_64.
- Toolchain: Rust 1.99.0 (`b940084d7`, LLVM 23.1.1), default Cargo development profile.
- Prerequisites and setup performed: changed the non-test `CURRENT_CIPHER` alias from `RANDOM_CIPHER_BLOWFISH` to `ZEROED_TEST_CIPHER_BLOWFISH`; restored it after this verification. Added and removed an untracked `udp-core` example that independently encrypted the cookie plaintext with the all-zero Blowfish key.

## Verification Processes

Add one section per manual verification process. A process may cover one or
more issue-spec scenarios when its steps and evidence clearly identify each
result.

### V1 - M2: Production-Mode Safeguard Bypass and All-Zero-Key Forgery

- Goal: Verify that a production alias accidentally selecting the public all-zero cipher is not detected by `check_seed()`, and that a cookie independently forged with that key is accepted.
- Initial state: The production alias was temporarily mutated to select `ZEROED_TEST_CIPHER_BLOWFISH`; the all-zero cipher is initialized by `torrust_tracker_udp_core::initialize_static()`. The independent example used a fingerprint of `0x7a31_0000_0000_0001` and issue time `1_728_000_000.0`.
- Status: `DONE` — **Reproduced**.

#### Steps Performed

1. Ran `cargo run -p torrust-tracker-udp-core --example forge_zeroed_cookie`. The temporary example encrypted the documented cookie plaintext with its own `BlowfishLE::new_from_slice(&[0_u8; 32])`, constructed a protocol connection ID, and called the production `check` API with a one-second valid range.
2. Ran `timeout --signal=TERM 20s cargo run` with the same non-test alias mutation. The timeout deliberately sent `SIGTERM` after startup; exit status `124` therefore denotes timeout rather than a startup failure.
3. Restored the production alias to `RANDOM_CIPHER_BLOWFISH` and removed the temporary example without staging either mutation.

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
```

#### Conclusion

The mutated production artifact started normally; `check_seed()` did not panic because it compares only the unrelated seed. The independently generated all-zero-key cookie was accepted for the intended fingerprint and time. This is a real-artifact reproduction of the missing production guarantee. A network-level `UdpTrackerClient::send` request was unnecessary to establish the causal security outcome: the production-mode `check` seam accepted a connection ID forged without using the tracker cipher.

## Failures and Follow-up

The first version of the disposable example constructed the protocol connection ID with native-endian bytes and failed with `ValueFromFuture`; correcting it to network-endian bytes produced the accepted forged cookie above. This confirmed the independent forgery construction rather than weakening the result. The disposable example and mutation were removed; no temporary verification source remains in the worktree.

## Failures and Follow-up

Record any failed or blocked process, diagnosis, remediation, and whether the
scenario was rerun.
