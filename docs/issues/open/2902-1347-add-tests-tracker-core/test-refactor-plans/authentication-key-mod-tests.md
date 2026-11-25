---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/authentication/key/mod.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/authentication/key/mod.rs
    - packages/tracker-core/src/authentication/handler.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/auth_key.rs
---

# Authentication Key Module File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 30,
`packages/tracker-core/src/authentication/key/mod.rs`: `generate_key`, `verify_key_expiration` and the key
`Error`.

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, S4 base
  `develop` at `39d6d7e1b`, measured 2026-11-25): 79 / 79 lines, 92 / 95 regions. The uncovered regions were the
  two `tracing::debug!` calls of `generate_key`: a test run has no logger. #3042 removed both calls (they logged the
  generated key), so after the rebase the file has no uncovered region.
- **Existing tests:** 7: an expiring key is displayed (lifetime 0 at the epoch); an expiring key verifies
  while valid and fails after its lifetime (+10 s ok, +20 s expired for a 19 s key); a permanent key is
  generated without expiry and always verifies; the database-error conversion and its text. Smells: the
  `the_permanent_peer_key::should_be_displayed` test **generates an expiring key** (a copy of the test of the
  other module), so no test displays a key made by `generate_permanent_key`; the expiry tests never test the
  boundary, so the comparison `valid_until < current_time` could become `<=` unnoticed.
- **Module-owned decisions** (read from the code):
  - A key is expired only when its expiry is **strictly before** now: at the exact expiry time it still verifies.
  - `generate_key(Some(lifetime))` expires at now plus the lifetime, to the nanosecond; `None` is permanent.
  - The error texts: owned by `develop` since #3042 (the secrets-redaction fix): its own table pins "Authentication key expired" and the "Missing authentication key" message, and the service tests pin "Authentication key not registered".

**Out of scope, found while planning:** `generate_key(Some(lifetime))` does `now_add(..).unwrap()`, so a lifetime
that overflows the clock panics; the REST add-key and generate-key endpoints reach it with `seconds_valid` of
`u64::MAX` (reproduced 2026-11-25). It is a bug, handled by its own issue and session (proposed to the
maintainer); this plan adds no test for it, and a regression test follows once the fix merges.

## Maintainer Decision

Plan approved on 2026-11-25.

## Test Design

Tests join the file's `tests` module; the clock is `clock::Stopped`.

| Test | Cases | Assert |
| --- | --- | --- |
| verifying an expiring key at the boundary (`rstest`) | now one nanosecond before the expiry; now equal to the expiry; now one nanosecond after | `Ok`; `Ok`; `Err(KeyExpired)` |
| a permanent key never expires | now at 0, at one year and at `u64::MAX` seconds after the epoch | `Ok` |
| expiry is exact (`quickcheck`) | any expiry and any now, both as seconds and nanoseconds | `verify_key_expiration` is `Err(KeyExpired)` exactly when the expiry is before now |
| a generated key expires at now plus its lifetime | now at 1,000 s; lifetime 500 s and lifetime 500 s 7 ns; no lifetime | `valid_until` is 1,500 s, 1,500 s 7 ns, and `None` |
| the permanent key display (replaces the misplaced test) | a key from `generate_permanent_key` | the text is ``key: `[REDACTED]`, permanent`` (a key's display is redacted since #3042) |

**Not in this plan:** the debug logs of `generate_key` that this plan was written against; #3042 removed them.

## Property-Test Decision

`quickcheck` for the third row, as the spec decided (Q5): the rule is a pure comparison of two durations;
the table covers the boundary cases, the property covers the rest of the range.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | the comparison `<` becomes `<=` | the equal-to-expiry case |
| M2 | the comparison `<` becomes `>` | the boundary table and the property |
| M3 | a permanent key fails verification (`Err(KeyExpired)` for `None`) | the permanent-key test |
| M4 | `generate_key` ignores the lifetime (`now_add(&Duration::ZERO)`) | the generated-expiry test |
| M5 | `generate_key` gives an expiring key no expiry (`valid_until: None`) | the generated-expiry test |
| M6 | `generate_key(None)` gives an expiry | the permanent display and generated-expiry tests |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| G1 | DONE | The tests; the misplaced display test fixed. | They pass. |
| G2 | DONE | Run M1-M7. | Each killed by its expected case. |
| G3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design, then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_expire_a_key_only_strictly_after_its_expiry_time` (cases) | The clock stopped at the case's time, and a key expiring at 1,000 s 500 ns. | Verify the key. | It is expired only when the clock is after the expiry. | Matches the code. |
| `it_should_never_expire_a_permanent_key` (cases) | The clock stopped at the case's time, and a permanent key. | Verify the key. | It verifies. | Matches the code. |
| `it_should_expire_a_key_exactly_when_its_expiry_is_before_now` (property) | The clock stopped at a generated time, and a key expiring at a generated time. | Verify the key. | It is expired exactly when the expiry is before now. | Matches the code. It has no AAA comments: the body is the property. |
| `it_should_generate_a_key_that_expires_at_now_plus_its_lifetime` (cases) | The clock stopped at 1,000 s. | Generate a key with the case's lifetime. | Its expiry is now plus the lifetime, or none. | Matches the code. |
| `the_permanent_peer_key::should_be_displayed` (fixed) | None. | Generate a permanent key. | It is displayed as permanent. | Matches the code. The old test built an expiring key. |

Smells found: none beyond the misplaced test, which the plan fixed. The property compares with the rule
spelled again, as a property does; its random times rarely hit equality, so the boundary table owns that.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage target
  directory, 2026-11-25): the file 79 / 79 lines to 96 / 96, regions 92 / 95 to 108 / 111, functions
  (with the test module) 17 / 17. The three regions left were the two `tracing::debug!` calls of
  `generate_key`, which #3042 removed after the rebase: the file now has no uncovered region. The package's unit-only lines
  5,326 / 6,074 (87.69%), regions 5,861 / 6,753 (86.79%), functions 930 / 1,122 (82.89%).
- **Tests:** 10 added (3 boundary cases, 3 permanent cases, 1 property, 3 generated-expiry cases) and one
  existing test fixed. A first draft also had a table of the key error texts (3 cases, mutant M7); after the
  rebase onto `develop` for #3042, which changed the texts and added its own table of them, it was dropped as a
  duplicate. Line coverage did not change what the file protects: the file was already fully covered by line.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the boundary case at the exact expiry **alone**: no other test of the package fails when `<` becomes `<=` |
  | M2 | Killed by the boundary table, the property and many authentication tests |
  | M3 | Killed by the permanent-key cases and the existing permanent tests |
  | M4 | Killed by the generated-expiry cases and existing tests |
  | M5 | Killed by the generated-expiry cases and existing tests |
  | M6 | Killed by the no-lifetime case and the permanent display test |
  | M7 | Dropped with the error-text table (see Tests); the text is pinned by `develop`'s own table |

- **Review outcome:** the maintainer approved the completed file on 2026-11-25.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the existing
  `axum::serve` warning), `cargo +nightly fmt --all -- --check` and `cargo machete --with-metadata` pass.
