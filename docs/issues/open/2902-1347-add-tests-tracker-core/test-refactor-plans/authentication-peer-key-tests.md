---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/authentication/key/peer_key.rs
status: done
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/authentication/key/peer_key.rs
    - packages/tracker-core/src/authentication/key/mod.rs
    - packages/axum-http-server/src/v1/extractors/authentication_key.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/auth_key.rs
---

# Peer Key File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 29,
`packages/tracker-core/src/authentication/key/peer_key.rs`: `Key`, `PeerKey` and `ParseKeyError`. It also
settles finding F3.

## Current State

- **Unit-only coverage** (`--all-features --lib`, stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, S4 base
  `develop` at `8e89b382b`, measured 2026-11-25): 99 / 100 lines, 106 / 107 regions, 19 / 19 functions. The one
  uncovered line is 64, the `_ => false` arm of `PeerKey::eq`: a permanent key compared with an
  expiring one.
- **Existing tests:** 7: `Key` parses a valid string, is generated randomly (the test asserts nothing),
  has length 32, only alphanumeric characters, and returns its inner string; a `PeerKey` can be expiring
  or permanent and is displayed accordingly. **No test in the file rejects a bad key**: the length and
  character checks run only through the handler's invalid-key test.
- **Module-owned decisions** (read from the code):
  - `Key::new` accepts exactly 32 **bytes** of ASCII alphanumeric characters; the length is checked
    first, so a string with both faults is `InvalidKeyLength`; `FromStr` agrees with `new`.
  - `PeerKey` equality compares the key and the expiry **in whole seconds**; a permanent key never
    equals an expiring one.
  - `Key::random` always yields a valid key.

## F3: is the derived `Deserialize` of `Key` reachable from untrusted input?

`Key` derives `Deserialize` over its inner `String`, so deserializing skips the checks of `Key::new`.
Checked on 2026-11-25 (every use in `packages/`, `src/` and `console/`): **no code deserializes a `Key` or a
`PeerKey`**. Every external key arrives as a `String` and is parsed with `parse::<Key>()`: the HTTP path
extractor (`KeyParam(String)` then `parse_key`), the REST add-key form (`opt_key: Option<String>`, parsed
by the handler) and the REST path (`Key::from_str`); the database drivers parse the stored string with
`parse_valid_until` and `Key::from_str` (row 8). The REST resource `AuthKey` holds a `String`, not a `Key`. So F3 is
not a bug today; it is a latent bypass for a future caller.

## Maintainer Decision

Plan approved on 2026-11-25. F3: **record only** (no production change, no test for the derive, no new issue): the evidence above is the record, and the latent bypass is noted in the spec's findings table and log.

## Test Design

Tests join the file's `tests` module.

| Test | Cases | Assert |
| --- | --- | --- |
| a key is accepted or rejected (`rstest`) | 32 alphanumeric characters (upper, lower, digits); 31 and 33 characters; empty; 32 characters with a `-`; 32 characters with a space; 16 two-byte letters (32 bytes, not ASCII); 33 characters with a `-` (both faults) | `Ok` for the first; `InvalidKeyLength` for the length cases (including both faults); `InvalidChars` for the character cases; `FromStr` gives the same result as `new` |
| equality of two peer keys (`rstest`) | same key and same seconds; same key and same seconds with different nanoseconds; same key, different seconds; same key, one permanent and one expiring (both orders); different keys, same expiry; both permanent | equal for the first, second and last; not equal for the others |
| validity is exact (`quickcheck`) | a string of 31 to 33 characters from an alphabet that is mostly alphanumeric, with `-`, a space and a two-byte letter mixed in | `Key::new` is `Ok` exactly when the string is 32 ASCII-alphanumeric bytes, and then its value equals the input |
| a random key is valid | 100 draws of `Key::random()` | each passes `Key::new` |

The existing `should_be_generated_randomly` asserts nothing; the last row replaces its weight.

## Property-Test Decision

`quickcheck` for the third row, as the spec decided (Q5): the validity rule is a pure predicate over
strings, which the fixed table cannot enumerate. A first draft used an arbitrary `String`; coverage showed
it never reached the accepted branch (a random string is almost never 32 valid bytes), so the generator was
replaced by one that builds 31 to 33 characters from a mostly valid alphabet, and the second draft's separate
"valid strings" property folded into it.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | `Key::new` length check `!=` becomes `<` | the 33-character cases |
| M2 | the character check uses `any` instead of `all` | the `-` and space cases |
| M3 | the two checks swap order | the both-faults case |
| M4 | `FromStr` skips the validation (`Self::new(s)?` becomes `drop(Self::new(s))`, because `let _ =` on a `Result` is denied) | the `FromStr` agreement of the table and the property |
| M5 | `PeerKey::eq` mixed arm `_ => false` becomes `_ => true` | the permanent-versus-expiring cases |
| M6 | `PeerKey::eq` compares nanoseconds as well | the nanosecond case |
| M7 | `PeerKey::eq` ignores the key | the different-keys case |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| K1 | DONE | The four tests; settle F3 as decided. | They pass; line 64 runs. |
| K2 | DONE | Run M1-M7. | Each killed by its expected case. |
| K3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design, then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_accept_only_thirty_two_ascii_alphanumeric_bytes` (cases) | One candidate string per case: valid, too short or long, empty, with a hyphen, a space or two-byte letters, and one with both faults. | Parse it with `Key::new` and with `FromStr`. | Both give the case's result: accepted, a length error or a character error. | Matches the code. The Arrange is the case argument. |
| `it_should_accept_a_string_exactly_when_it_is_thirty_two_ascii_alphanumeric_bytes` (property) | A string of 31 to 33 characters built from the generated bytes. | Parse it with `Key::new`. | It is accepted exactly when it is 32 ASCII-alphanumeric bytes, and then keeps its value. | Matches the code. The oracle spells the rule again; that is the property. |
| `it_should_generate_only_valid_random_keys` | None. | Generate a random key, 100 times. | Each passes `Key::new`. | Matches the code. The loop is the Arrange and Act of each draw. |
| `it_should_compare_the_key_and_the_expiry_in_whole_seconds` (cases) | Two peer keys built from the case's keys and expiries. | Compare them. | They are equal exactly when the case says. | Matches the code. |

Smells found: none. The property function takes its generated arguments by value, which `clippy::needless_pass_by_value`
flags; it carries an `expect` with that reason because `quickcheck` requires it.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage target
  directory, 2026-11-25): the file 99 / 100 lines to 129 / 129, regions 106 / 107 to 136 / 136, functions
  19 / 19 to 25 / 25 (the counts include the test module). Line 64, the permanent-versus-expiring arm, runs.
  The package's unit-only lines 5,309 / 6,057 (87.65%), regions 5,845 / 6,737 (86.76%), functions
  928 / 1,120 (82.86%).
- **Tests:** 18 added (9 key cases, 1 property, 1 random-key test, 7 equality cases): 290 to 308 unit tests.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the 33-character cases, the property and `length_should_be_32` |
  | M2 | Killed by the hyphen, space and symbol cases and by the property |
  | M3 | Killed by the both-faults case alone |
  | M4 | Killed by every rejecting case of the table and by the handler's invalid-key tests |
  | M5 | Killed by the two permanent-versus-expiring cases |
  | M6 | Killed by the nanoseconds case (and by `run_sqlite_driver_tests`) |
  | M7 | Killed by the another-key case |

- **F3:** not reachable from untrusted input today (evidence above); recorded, no change.
- **Review outcome:** the maintainer approved the completed file on 2026-11-25.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the existing
  `axum::serve` warning), `cargo +nightly fmt --all -- --check` and `cargo machete --with-metadata` pass.
