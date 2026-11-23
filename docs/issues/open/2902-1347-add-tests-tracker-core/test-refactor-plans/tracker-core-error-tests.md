---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/error.rs
status: approved
semantic-links:
  related-artifacts:
    - packages/tracker-core/src/error.rs
    - packages/http-core/src/services/error_mapping.rs
---

# Tracker Core Error File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger row 15,
`packages/tracker-core/src/error.rs`: the `TrackerCoreError` wrapper and its four `From`
conversions.

## Current State

- **Unit-only coverage:** 38 / 50 lines (76.00%), 33 / 45 regions, 4 / 8 functions, at the S2
  baseline and after rows 8 to 14 (`develop` at `83f3ecbbc`, stable Rust 1.100.0,
  `cargo-llvm-cov 0.9.1`, `--all-features --lib`). Uncovered: 47-49, 53-55, 59-61 and 65-67, the four
  `From` conversions into `TrackerCoreError`.
- **Existing tests:** four, all on the message of an error: a non-whitelisted torrent, and the three
  peer-key errors (overflow, bad key, failed persistence). None builds a `TrackerCoreError`.
- **Callers:** `http-core` maps each variant to a protocol error response and `udp-core` logs it; the
  conversions are applied by `?` in the announce and scrape handlers.
- **Module-owned decisions:** each conversion wraps its source in the variant of the same name, and
  each variant's message names its area ("Tracker core announce error", "scrape", "whitelist",
  "authentication") before the source's own message.

The conversions are type-safe: every source type is accepted by exactly one variant, so a mutant that
routes a conversion to another variant does not compile. What a test can protect is the message each
variant prints, which is the text the tracker logs for a failed announce or scrape.

## Maintainer Decision

Plan approved on 2026-11-23.

## Test Design

One table test in the file's `tests` module, over the four conversions.

| Case | Converted from | Assert |
| --- | --- | --- |
| announce | `AnnounceError::Database(a database error)` | the variant is `AnnounceError`; the message starts with `Tracker core announce error: Database error:` |
| scrape | `ScrapeError::Whitelist(a whitelist error)` | `ScrapeError`; `Tracker core scrape error: Whitelist error:` |
| whitelist | `WhitelistError::TorrentNotWhitelisted` | `WhitelistError`; `Tracker core whitelist error: The torrent:` |
| authentication | `authentication::key::Error::KeyExpired` | `AuthenticationError`; `Tracker core authentication error: Key has expired` |

## Property-Test Decision

No `quickcheck`: four fixed conversions.

## Named Mutations

| ID | Mutation | Expected killer |
| --- | --- | --- |
| M1 | the announce variant's message names the scrape area | the announce case |
| M2 | the scrape variant's message names the announce area | the scrape case |
| M3 | the whitelist variant's message drops its prefix | the whitelist case |
| M4 | the authentication variant's message names the whitelist area | the authentication case |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | TODO | The table test. | It passes; lines 47-67 run. |
| R2 | TODO | Run M1-M4. | Each is killed by its expected case. |
| R3 | TODO | Record the results and ask for the completed-file review. | This record and the ledger. |

## Results

To be filled in after R3.
