---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/crypto/cookie_cipher.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/crypto/cookie_cipher.rs
    - packages/udp-core/src/connection_cookie.rs
    - packages/udp-core/Cargo.toml
    - docs/issues/closed/2458-inject-udp-cookie-cipher/ISSUE.md
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Cookie Cipher File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/crypto/cookie_cipher.rs`, added by Issue #2458.

## Current State

- **Unit-only coverage:** 49 / 49 lines (100.00%) after #2458; not in the T1 inventory.
- **Existing tests:** 3 unit tests from #2458 (a random key does not encrypt like the fixed test
  key; two random keys differ; `Debug` is redacted) and 2 doc tests (production can build a
  random key; a `compile_fail` test proving `fixed_for_testing` does not exist outside this
  crate's unit tests).
- **Module-owned decisions:** production keys come only from the cryptographically secure RNG; the
  fixed all-zero key exists only in this crate's unit-test build; `Debug` never prints key bytes;
  the key schedule is wiped on drop.
- **T1 hypothesis:** none (new file). Rank 1 (security).

## Current Tests Review

Conclusion: `refactor` (one small change).

- All three unit tests have visible Arrange/Act/Assert sections, clear names, and independently
  stated expectations.
- `it_should_redact_the_key_when_formatted_for_debugging` asserts the exact output
  `CookieCipher([REDACTED])` and then also asserts that the output does not contain the key bytes.
  The second assertion cannot fail when the first passes, so it is a redundant second reason to
  fail. R1 removes it.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Random keys differ from the fixed test key and from each other | 3A unit | Existing tests; kept. |
| `Debug` redaction | 3A unit | Existing test; kept after R1. |
| Fixed key unreachable outside unit tests | Doc test | Existing `compile_fail` doc test; kept. It runs in the doc-test gate, not in unit-only coverage. |
| Key schedule wiped on drop | 3A unit (compile time) | Z1: assert that `BlowfishLE` implements `ZeroizeOnDrop`. The wipe exists only because `Cargo.toml` enables `blowfish`'s `zeroize` feature; if that feature were removed, the module docs would silently become false. Z1 turns that into a compile error. |
| Encrypt/decrypt round trip | Not selected | Owned by the `blowfish` crate; the cookie round-trip property in `connection_cookie.rs` already exercises it through `encrypt`/`decrypt`. |
| Property-based testing | Not selected | No property beyond the round trip above. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Remove the redundant second assertion in the redaction test. | Focused tests passed; the exact-output assertion is kept as the stronger guarantee (any key encoding, not only `5a`, would break it). |
| R2 | DONE | Add Z1. | 68 package unit tests and the doc tests passed; pedantic Clippy, nightly formatting, and `git diff --check` clean. With the `zeroize` feature temporarily removed from `Cargo.toml`, Z1 failed to compile with `E0277` (`Blowfish<LittleEndian>: ZeroizeOnDrop` not satisfied); the feature was restored by hand. |
| R3 | SKIPPED | No integration increment: a value type with no boundary. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 49 / 49 to 51 / 51 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 49 / 49 to 51 / 51 (100.00%) lines; 88 / 88
  regions; 12 / 12 functions.
- **Tests refactored:** 1 (redundant assertion removed).
- **Tests added:** 1 compile-time guarantee (Z1).
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-10.

## Approval

Approved by Jose Celano on 2026-10-10: R1 (one test with only the exact-output assertion, after
discussing splitting it into two) and Z1.

## Progress Log

- 2026-10-09 12:19 UTC - GitHub Copilot - Created the plan from `crypto/cookie_cipher.rs`, its doc
  tests, the `blowfish` `zeroize` feature in `Cargo.toml`, and the post-#2458 unit-only report.
  `develop` was 5 commits ahead with no `udp-core` change. No Rust tests or production code
  changed. R1 and R2 await maintainer approval.
- 2026-10-10 09:25 UTC - GitHub Copilot - Plan approved. Implemented R1 and R2, measured R4
  (stable Rust 1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`). The plan awaits completed-file
  review.
- 2026-10-10 10:10 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
