---
doc-type: file-test-plan
issue: 1348
package: torrust-tracker-udp-core
target-file: packages/udp-core/src/connection_cookie.rs
status: done
semantic-links:
  related-artifacts:
    - packages/udp-core/src/connection_cookie.rs
    - packages/udp-core/src/crypto/keys.rs
    - .github/skills/dev/maintenance/add-rust-dependency/SKILL.md
    - docs/issues/open/1348-1347-add-tests-udp-core/coverage-evidence.md
---

# UDP Core Connection Cookie File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/udp-core/src/connection_cookie.rs`.

## Current State

- **Unit-only coverage:** 130 / 135 lines (96.30%), 283 / 286 regions, 15 / 15 functions, at T1
  and unchanged since.
- **Uncovered lines:** 168-170, the `check` branch that rejects a decoded issue time that is not
  a normal `f64` (the forged-cookie defence); 334 and 352, `panic!` arms inside two existing tests.
- **Existing tests:** 8 example tests: a pinned cookie value, determinism, two difference
  tests, valid, expired, from-the-future, and wrong realistic fingerprint.
- **Module-owned decisions:** `make` rejects issue times that are not normal; `make`/`check`
  round-trip the issue time bit-exactly for the same fingerprint; `check` rejects non-normal
  decoded values, times before the range start, and times after the range end, and accepts both
  range ends; `check` panics when the range start exceeds its end; the fingerprint hashes the full
  socket address.
- **T1 hypothesis:** property candidate.

## Current Tests Review

Conclusion: `refactor`.

- No visible Arrange/Act/Assert sections.
- `it_should_reject_an_expired_cookie` and `it_should_reject_a_cookie_from_the_future` assert
  through `match ... panic!`; use `assert!(matches!(...))`. This also removes the two uncovered
  test lines.
- `it_should_make_a_connection_cookie` pins bytes "derived through experimentation". The pin is
  useful: it detects any change to the cookie wire encoding under the fixed test key. The name
  should say so (`it_should_keep_the_cookie_encoding_stable_for_the_test_key`).
- The remaining tests are clean.

## Coverage Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Forged cookie decoding to a non-normal issue time | 3A unit | C1: a cookie built through the private `assemble`/`encode` steps from zero, NaN, infinity, and a subnormal value is rejected with `ValueNotNormal`. |
| `make` input validation | 3A unit | C2: `make` rejects zero, NaN, infinity, and a subnormal issue time with `ValueNotNormal`. No direct test exists; the lines report as covered without one, which R4 will re-check. |
| Inclusive range ends | 3A unit | C3: a cookie issued exactly at the range start, and one issued exactly at the range end, are accepted. Pins the `<`/`>` comparisons. |
| Inverted range | 3A unit | C4: `check` panics when the range start exceeds its end (`#[should_panic]`), a documented contract. |
| Round trip for arbitrary fingerprints | Property | C5: for any `u64` fingerprint and any normal issue time `t`, `check(make(fp, t), fp, t..t)` returns `t` bit-exactly. The examples only use fingerprint `1_000_000`; large fingerprints exercise the `wrapping_add`/`wrapping_sub` overflow paths that examples miss. |
| Wrong fingerprint rejection | Not a property | The module docs state a cookie can coincidentally validate under another fingerprint, so "always rejected" is false. The realistic-address example stays. |
| Cipher key selection and Blowfish | Collaborator-owned | `crypto/keys.rs` (ledger order 5) and the `blowfish` crate own them. |

**Property-testing dependency:** add `quickcheck` and `quickcheck_macros` as `udp-core`
dev-dependencies, following the `add-rust-dependency` skill. `udp-protocol` already uses
`quickcheck 1` (locked at 1.1.0), so no new crate enters the workspace. R2 checks the latest stable
version before adding them. If C5 shows no value over the examples, it is removed with the
dependency and the reason recorded.

**Findings (no production change in this issue; `lessons.md` candidates):**

- The error messages say `expected > {min_value}` and `expected < {max_value}`, but `check`
  accepts both range ends. C3 pins the accepting behavior; the wording should say `>=`/`<=`.
- The module docs say special values are excluded "by checking `issue_time.is_finite()`"; the
  code checks `is_normal()`, which also excludes zero and subnormal values.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Add AAA sections, replace the two `match ... panic!` assertions, rename the pinned-encoding test. | 8 focused `connection_cookie` tests passed; nightly formatting, pedantic Clippy, and `git diff --check` clean; all hunks inside the test module. |
| R2 | DONE | Add C1-C4; add the dev-dependencies and C5. | 68 package tests passed; pedantic Clippy, `cargo machete`, nightly formatting, and `git diff --check` clean. `quickcheck 1` (1.1.0) and `quickcheck_macros 1` (1.2.0) are the latest stable versions and match `udp-protocol`. C5 alone caught a temporary wrapping-to-saturating arithmetic mutation that all 13 example tests passed. |
| R3 | SKIPPED | No integration increment: pure functions with no boundary. | Explicit rationale. |
| R4 | DONE | Re-run unit-only coverage; record before/after. | 130 / 135 to 171 / 171 lines; see Results. |

## Results

- **Unit-only coverage (test-inclusive file totals):** 130 / 135 (96.30%) to 171 / 171 (100.00%)
  lines; 283 / 286 to 372 / 376 regions; 15 / 15 to 22 / 22 functions.
- **Tests refactored:** 8 (AAA sections, two `matches!` assertions, one rename).
- **Tests added:** 6: C1-C4 as examples (C3 as two tests) and C5 as a `quickcheck` property.
- **Property testing kept:** yes, with mutation evidence above.
- **Findings:** recorded in [lessons.md](../lessons.md).
- **Integration selected:** No.
- **Completed-file review:** Approved by Jose Celano on 2026-10-06.

## Approval

Approved by Jose Celano on 2026-10-06: the R1 refactor, C1-C5, the `quickcheck` dev-dependencies,
and recording the two findings in `lessons.md`.

## Progress Log

- 2026-10-06 09:04 UTC - GitHub Copilot - Rebased onto `develop` (38 commits; none touched
  `udp-core` or `udp-protocol`). Created the plan from `connection_cookie.rs`, its module
  security notes, and the latest unit-only missing-line report. No Rust tests or production code
  changed. R1 and R2 await maintainer approval.
- 2026-10-06 10:06 UTC - GitHub Copilot - Plan approved. Completed R1 (the validation run waited
  about ten minutes on a Cargo lock held by IDE test discovery). R1 awaits the refactor-result
  review before R2.
- 2026-10-06 11:41 UTC - GitHub Copilot - R1 approved. Implemented R2, measured R4 (stable Rust
  1.99.0, nightly 1.101.0, `cargo-llvm-cov 0.6.16`), and created `lessons.md`. The plan awaits
  completed-file review.
- 2026-10-06 11:51 UTC - Jose Celano - Approved completed-file review. The file is ready for
  pre-commit and its signed per-file commit.
