---
doc-type: file-test-plan
issue: 2876
package: torrust-tracker-primitives
target-file: packages/primitives/src/peer.rs
status: done
last-updated-utc: "2026-11-16 16:05"
---

# `peer::fixture::qbittorrent_peer_id` Test Plan (T2)

The realistic-id helper decided in Q1 (in `peer::fixture`) and Q4 (`u32` input).

## The Helper

```rust
pub const fn qbittorrent_peer_id(number: u32) -> Id
```

It returns the Azureus-style prefix `-qB4650-` (qBittorrent 4.6.5, the id #2824's tests use)
followed by `number` as 12 zero-padded decimal digits: `qbittorrent_peer_id(42)` is
`-qB4650-000000000042`. `u32::MAX` needs 10 digits, so every input fits.

- It returns `Id`, like `Id::new` did, so the callers keep passing `&id` to
  `PeerBuilder::with_peer_id(&PeerId)` through `Deref`.
- It is a `const fn`, so the fixture's `const fn` builders (`seeder`, `leecher`) can use it for
  their own ids (Q6, T6).

## Behaviors and Tests

All three are unit tests in the `peer::test::qbittorrent_peer_id` module of `peer.rs`; the helper
decides each behavior itself.

| Behavior | Test |
| --- | --- |
| Exact format: prefix, then 12 zero-padded decimal digits | `it_should_write_the_number_as_twelve_zero_padded_decimal_digits_after_the_qbittorrent_prefix` (0, 42, `u32::MAX`) |
| The client registry knows every id as qBittorrent | `it_should_make_ids_that_the_client_registry_knows_as_qbittorrent` (quickcheck, any `u32`) |
| Distinct numbers give distinct ids | `it_should_make_distinct_ids_from_distinct_numbers` (quickcheck, any two different `u32`) |

## Red, Green and Mutations

Stable Rust toolchain, cargo 1.99.1, `cargo test -q -p torrust-tracker-primitives --lib qbittorrent_peer_id`.

- **Red 1:** tests written first, no helper: `error[E0432]: unresolved import
  crate::peer::fixture::qbittorrent_peer_id`.
- **Red 2:** a stub that returned `Id::new(number)` (today's ids): the format test failed
  (`peer id made from 0: "\0\0..."`) and the client test failed (`[quickcheck] TEST FAILED.
  Arguments: (0)`); 1 passed, 2 failed. The distinctness test passed: `Id::new` is distinct too, so
  its guard is shown by mutation M3.
- **Green:** 3 passed. The whole package: 111 passed (108 before).

Each mutation was applied to the working tree, checked to compile (no `error[` line; the only
`error:` line is cargo's "test failed"), and the file was restored with `cmp` against a saved copy.

| Mutation | Breaks | Failed tests |
| --- | --- | --- |
| M1: digits written from byte 8 onwards, least significant first (`42` gives `-qB4650-240000000000`) | the format only; still distinct and still qBittorrent | format |
| M2: prefix `-ZZ4650-` | the client name (and so the exact bytes) | client, format |
| M3: digit loop never runs (every number gives `-qB4650-000000000000`) | distinctness (and the bytes of 42 and `u32::MAX`) | distinct, format |

A first run of M1 did not compile (it left `PEER_ID_BYTES_LEN` unused, which `-D warnings`
rejects); it was rewritten to keep the constant in use and re-run, with the result above.

## Prose-First AAA Review

Format test:

- Arrange: three numbers that differ in how they are written: zero (no digit to write, all
  padding), an ordinary number, and the widest `u32` (ten digits); each with the exact id expected,
  written out by hand.
- Act: build the id from the number.
- Assert: the id is exactly the expected bytes; on failure the message shows the number and the id
  made.

The table did not say why these three numbers; a short comment on each case now does, and
`// Arrange`, `// Act`, `// Assert` mark the sections. The expected ids are literals, not derived
from the helper.

Property tests: each is one expression with the Act and the expected result in view (`Some("qBittorrent")`;
`!=` on the bytes, with equal inputs discarded). Prose added nothing, so none was kept.

Smells checked: no complex Arrange, no hidden Act, no production-derived expectation. The format
test asserts in a loop, but every assertion checks the same behavior and its message names the case.

## Maintainer Design Review

Approved by the maintainer in chat on 2026-11-16 ("Approve, continue"): the name
`qbittorrent_peer_id`, the signature `const fn(u32) -> Id` and the `-qB4650-` + 12-digit format,
before the callers move to it (T3).
