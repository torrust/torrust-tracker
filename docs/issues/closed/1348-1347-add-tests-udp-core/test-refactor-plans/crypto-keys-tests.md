---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/crypto/keys.rs
status: superseded
semantic-links:
  related-artifacts:
    - packages/udp-core/src/crypto/keys.rs
    - packages/udp-core/src/crypto/ephemeral_instance_keys.rs
    - packages/udp-core/src/connection_cookie.rs
    - src/bootstrap/app.rs
    - docs/issues/closed/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Crypto Keys File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/udp-core/src/crypto/keys.rs`.

## Current State

- **Unit-only coverage:** 35 / 41 lines (85.37%) at T1 and unchanged since.
- **Uncovered lines:** 42-44, `Instance::get_cipher_blowfish` (the production keeper's cipher);
  77-79, the test-only `ZeroedTest` keeper's cipher method, which no test calls.
- **Existing tests:** 6: two keeper-level seed tests, three seed-alias tests, one cipher-alias test.
- **Module-owned decisions:** the `Current` facade resolves to the zeroed seed and cipher in test
  builds and to the random ones otherwise; the `Instance` keeper always provides the random seed
  and cipher.
- **T1 hypothesis:** test candidate; encrypt/decrypt round trip named as a property candidate.

## Current Tests Review

Conclusion: `refactor`.

- No visible Arrange/Act/Assert sections.
- The cipher-alias test is misnamed `it_should_default_to_zeroed_seed_when_testing`; it compares
  ciphers. Rename to `it_should_default_to_the_zeroed_cipher_when_testing`.
- The test-only `ZeroedTest` keeper exists only so one test can compare `Current::get_seed()` with
  `ZEROED_TEST_SEED`; its cipher method is dead. Compare with `ZEROED_TEST_SEED` directly and
  remove the keeper.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| The production keeper's cipher is not the zeroed test cipher | 3A unit | K1: encrypting the same block with `Instance::get_cipher_blowfish()` and with the zeroed test cipher gives different output. Covers lines 42-44 and, incidentally, the random-cipher initialization in `ephemeral_instance_keys.rs`. |
| `Current` resolves to the zeroed seed and cipher in test builds | 3A unit | Existing tests, kept after R1. |
| Encrypt/decrypt round trip | Not selected | The cookie round-trip property in `connection_cookie.rs` already exercises `encode`/`decode` through `Current` for arbitrary inputs. Blowfish correctness is owned by the `blowfish` crate. A second property here would add a dependency use without new protection. |
| Random seed and cipher generation | Ledger order 6 | Owned by `ephemeral_instance_keys.rs`. |

**Finding (a `lessons.md` candidate; no production change in this issue):** `RANDOM_CIPHER_BLOWFISH`
draws its own random key; it is not derived from `RANDOM_SEED`. In the package source, the seed
only reaches the production check `check_seed()` in `src/bootstrap/app.rs`, which compares the
`Current` and `Instance` seeds. That check does not cover the cipher. A build that wired the
cookie cipher to the zeroed key would therefore pass it, and anyone could forge cookies. Options:
derive the cipher from the seed, or extend `check_seed()` to compare ciphers.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | Add AAA sections, rename the cipher-alias test, remove the `ZeroedTest` keeper. | Superseded: #2458 deleted the file. |
| R2 | SKIPPED | Add K1. | Superseded: #2458's `crypto/cookie_cipher.rs` test `it_should_not_encrypt_like_the_fixed_test_key_when_the_key_is_random` covers the same property. |
| R3 | SKIPPED | No integration increment: static key facades with no boundary. | Explicit rationale. |
| R4 | SKIPPED | Re-run unit-only coverage; record before/after. | Superseded: the file no longer exists. |

## Results

Superseded. Issue #2458 replaced `crypto/keys.rs` and `crypto/ephemeral_instance_keys.rs` with an
injected `crypto/cookie_cipher.rs`. The finding this plan raised is resolved there; the new file
has its own row in the ledger.

## Approval

Approved by Jose Celano on 2026-10-06: the R1 refactor and K1. The cipher-seed finding became
Issue #2458 ([spec](../../../closed/2458-inject-udp-cookie-cipher/ISSUE.md)), which injects the cookie key;
this file is paused until it merges, because that
work will rewrite `crypto/keys.rs`. Re-plan R1/K1 against the new design when resuming.

## Progress Log

- 2026-10-06 12:32 UTC - GitHub Copilot - Rebased onto `develop` (53 commits; none touched
  `udp-core`). Created the plan from `crypto/keys.rs`, `ephemeral_instance_keys.rs`, the static
  initialization in `lib.rs`, the production seed check in `src/bootstrap/app.rs`, and the latest
  unit-only missing-line report. No Rust tests or production code changed. R1 and R2 await
  maintainer approval.
- 2026-10-06 13:38 UTC - GitHub Copilot - R1 and K1 approved. The maintainer chose to fix the
  finding by injecting one cookie key instead of the global statics, as a separate issue worked in
  parallel by another agent (draft spec and hand-off in the local, git-ignored
  `.tmp/inject-udp-cookie-cipher/`). Issue #1348 is paused until that work merges to avoid
  conflicts.
- 2026-10-07 10:59 UTC - GitHub Copilot - The follow-up is Issue #2458; its spec is on `develop`
  and its implementation is in review as PR #2470. Resume this plan after PR #2470 merges.
- 2026-10-09 07:30 UTC - GitHub Copilot - PR #2470 merged and deleted this file. Plan superseded;
  no Rust change made under it.
