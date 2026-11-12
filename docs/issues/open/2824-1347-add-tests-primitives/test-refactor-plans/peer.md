---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/peer.rs
status: completed-file review pending
last-updated-utc: "2026-11-11 17:08"
---

# `peer.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 1, slice S2. Status: **approved by the maintainer on 2026-11-11** with the answers to Q1 to Q3 below; the tests are written and
await the completed-file review (second gate).

## Current State

- **Unit-only coverage:** 130 / 329 lines (39.51%), 191 / 428 regions, 22 / 66 functions, measured 2026-11-11 on
  `develop` after #2834. 10 unit tests: peer comparison (1), `ser_unix_time_value` (3), `PeerRole` JSON (2, S1),
  `peer::Id` conversion and hex (4).
- **Review of the existing tests: `clean`.** Names, visible Act and literal expectations follow the skill. Two
  `should_panic` tests pin the error variant by name (`NotEnoughBytes`, `TooManyBytes`) through `unwrap`; a
  `matches!` on the `Err` would say it more directly, but they have one reason to fail and stay.
- **Hypothesis:** the gap is almost all straight-line logic that no test calls: role handling, the `Peer` state
  predicates (three duplicated copies), the JSON shape of `Peer` and `Id`, and the `Id` conversions.

## Coverage And Decision Analysis

| Behavior group | Lines | Classification | Decision |
| --- | --- | --- | --- |
| `PeerRole::opposite`, `FromStr` (case-insensitive, unknown value) | 48-77 | Unit | Test both variants of `opposite` and its involution; `parse` for lower, mixed and upper case and for an unknown word (assert `raw_param`). **Not** the `location` field: owned by F3 / #2804. |
| `Peer` predicates `is_seeder`, `is_leecher`, `is_completed`, `role` | 269-290 | Unit | Table test of the seeder rule: `left = 0` seeder; `left = 1` leecher; `Stopped` with `left = 0` leecher; `Completed` seeder. See question Q1 for a negative `left`. |
| The same rule in `ReadInfo for Peer` and `ReadInfo for Arc<Peer>`, and the accessors `get_*` | 216-264 | Unit | One consistency test per entry point against the inherent `is_seeder`; no refactor of the triple copy (maintainability note, not a finding). |
| `Peer` mutators `ip`, `change_ip`, `mark_as_completed`, `into_completed`, `into_seeder` | 292-318 | Unit | Assert whole values: `change_ip` keeps the port; the `into_*` builders change only their own field. `ip(&mut self)` takes `&mut` needlessly: observation only. |
| JSON shape of `Peer` (`ser_peer_id`, `ser_announce_event`, `ser_number_of_bytes`, `Id` `Serialize`) | 176-204, 465-480 | Unit | One whole-`Peer` JSON test against a literal document (fixture not used; the value is built in the test), plus `Id` alone. |
| `Id`: `TryFrom<Vec<u8>>` success, `From<PeerId>`, `DerefMut`, `get_client_name` | 348-364, 402-405, 458-461 | Unit | `try_from` with 20 bytes gives the same bytes; `get_client_name` for `-qB...` is the qBittorrent client and `None` for an unknown prefix (collaboration with the `tdyne` registry, labelled; production reaches it through `rest-api-runtime-adapter`). `From` and `DerefMut` are compiler-guarded delegation: **unselected**. |
| `Id` `Display` `None` branch | 413 | Unit-unreachable | `to_hex_string` only returns `None` for invalid UTF-8, and hex output is ASCII: dead defensive branch, **unselected**, recorded. |
| `Id::new` | 369-382 | Owned elsewhere | Deferred: F5 moves it out of production code. No test until that issue decides. |
| `FromIterator<Peer> for Vec<P: Encoding>` | 487-495 | Unit | One test with a local `Encoding` type: order preserved, one element per peer. |
| `peer::fixture` (`PeerBuilder`) | 547-636 | Owned elsewhere | Deferred to #2482 (maintainer decision 2026-11-10). |

## Property-Test Decision

One `quickcheck` property (the approved seam): for any 20-byte vector, `Id::try_from` succeeds and
`to_hex_string` is `0x` plus the 40 lowercase hex digits of the same bytes; for any other length it fails.
It protects the encoding of every possible id, which the two example ids do not. Nothing else in the file has
a natural property (the role has two values).

## Decisions (2026-11-11)

- **Q1: pin the current behavior (option A).** `is_seeder` is `left <= 0 && event != Stopped`, so a negative
  `left` is a seeder. A characterization test with a name that states the rule is added. The maintainer wants the
  tracker to reject a negative `left` at the UDP boundary (HTTP already does: it parses a `u64`), which is outside
  `primitives`: finding F6, handed to issue #2838 (the maintainer approved rejecting a negative `left`, `uploaded` and `downloaded`), together with
  an ADR on tolerance of malformed inbound values (proposal `inbound-tolerance-adr`). The pinned test stays valid
  after that fix because it tests the signed domain type; its comment points at #2838.
- **Q2: yes.** Correct the doc comment of `to_hex_string` (output has the `0x` prefix). Documentation only.
- **Q3: yes.** Add `quickcheck` as a dev-dependency (the first property test), following `add-rust-dependency`.

## Results (2026-11-11)

Toolchains: stable Rust 1.99.1 for tests and coverage, `cargo-llvm-cov 0.6.16`, nightly `cargo fmt`.

| Measure | Before | After |
| --- | ---: | ---: |
| Unit tests of the package | 49 | 80 (31 added, all in `peer.rs`) |
| `peer.rs` lines | 130 / 329 | 486 / 554 |
| Package unit-only lines | 468 / 746 (62.73%) | 824 / 971 (84.86%) |
| Package unit-only regions | 743 / 1,057 (70.29%) | 1,302 / 1,464 (88.93%) |
| Package unit-only functions | 94 / 158 (59.49%) | 160 / 193 (82.90%) |

The line totals include the in-file test modules (the baseline did too), so the percentage rises partly because
the added tests are covered code. The 68 uncovered lines of `peer.rs` are exactly the groups the plan leaves
alone: `Id::new` (12 lines, F5), the dead `Display` `None` branch (1 line) and `peer::fixture` (55 lines, #2482).

All the planned groups are tested (`PeerRole`, the `Peer` predicates and mutators, `ReadInfo` for `Peer` and
`Arc<Peer>`, the JSON of `Peer` and `Id`, the `Id` conversions, the collection, one `quickcheck` property with two
tests). Decisions applied: Q1 pinned with `it_should_treat_a_negative_number_of_bytes_left_as_a_seeder` (comment
points at #2838); Q2 corrected the `to_hex_string` doc comment; Q3 added `quickcheck` and `quickcheck_macros` as
dev-dependencies (same requirements as `udp-core`; both already locked at their latest stable versions, 1.1.0 and
1.2.0; `cargo machete --with-metadata` is run by the pre-commit gate).

**A fixture lesson (for F5 and #2482).** The id the existing `PeerBuilder` fixture uses, `-qB00000000000000000`, is
not a valid Azureus-style id (the character after the four version characters must be `-`), so the client
registry reports no client for it. The new tests use `-qB4650-abcdefghijkl`, which the registry knows as
qBittorrent. This confirms the maintainer's concern that test ids should look like real ones.

### Mutation sample (8 mutants, each applied alone to `peer.rs` and reverted exactly)

| Mutant | Result | Killed by |
| --- | --- | --- |
| `is_seeder` `left <= 0` to `left == 0` | killed | the negative-`left` test (the only guard of `<=`) |
| `is_seeder` `!= Stopped` to `== Stopped` | killed | 4 tests (state tests) |
| `ReadInfo for Arc<Peer>` `<=` to `<` | killed | the shared-peer read-info test |
| `opposite` of the seeder returns the seeder | killed | the opposite-role test |
| `change_ip` resets the port | killed | the keep-the-port test |
| hex string without `0x` | killed | 6 tests (incl. the property) |
| `get_client_name` always `None` | killed | 3 tests |
| `FromStr` case sensitive | killed | the case-insensitive parse test |

No survivor. Run with `cargo test -p torrust-tracker-primitives --lib` per mutant; `cmp` against a saved copy
showed the file restored byte for byte.

## Prose-First AAA Review (every added test)

Temporary prose per section was written for each group and compared with the code; the code expresses it, so no
prose is kept as a comment, except where a comment states a rule that the test name cannot (F6).

| Group | Arrange (the one causal state) | Act | Assert (literal, independent) | Outcome |
| --- | --- | --- | --- | --- |
| `peer_role` | none: the role or the text is the input | `opposite()` / `parse` | the other role; the parsed role; `raw_param` | clean |
| `peer_state` | a peer from `a_peer_with(left, event)`: only `left` and `event` vary, everything else is fixed | the predicate or mutator | `is_seeder`, `is_leecher` and `role` agree; whole-value `assert_eq!` against the expected peer | clean; helper owns only the incidental fields |
| `peer_state::read_info` | a leecher, and a completed seeder in an `Arc` | each trait accessor | literal id, state; fields compared with the peer's own | clean |
| `peer_serialization` | the peer built in the test with distinct values per field | `serde_json::to_value` | one whole JSON document written in the test; one event per test | clean |
| `peer_id` | an id from a realistic prefix, or all zeros | conversion, `get_client_name`, JSON | bytes, client name or `None`, JSON literal | clean |
| `peer_id_encoding` | any bytes (property) | `try_from`, `to_hex_string` | expected hex built independently with `format!("{byte:02x}")` | clean |
| `peer_collection` | three peers with distinct ports; a local `Encoding` type | `collect` | order and one element per peer | clean |

Test smells found and fixed while writing: none left. Existing tests judged `clean` in the plan are unchanged.

## Validation

`cargo test -p torrust-tracker-primitives`; `cargo +nightly fmt --all -- --check`; `git diff --check`; the
pre-commit gate; unit-only coverage re-run with before and after recorded here. Prose-First AAA Review rows and
the mutation proofs are added at the completed-file review.
