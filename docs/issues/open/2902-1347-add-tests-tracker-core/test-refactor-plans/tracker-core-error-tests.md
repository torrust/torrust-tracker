---
doc-type: file-test-plan
issue: 2902
package: torrust-tracker-core
target-file: packages/tracker-core/src/error.rs
status: done
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
| R1 | DONE | The table test. | It passes; lines 47-67 run. |
| R2 | DONE | Run M1-M4. | Each is killed by its expected case. |
| R3 | DONE | Record the results and ask for the completed-file review. | This record and the ledger. |

## Prose-First AAA Review

Temporary prose, written from this plan's Test Design when the review of #3009 found this section
missing (it belongs before the completed-file review), then compared with the code.

| Test | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_wrap_each_error_in_its_own_variant_and_name_its_area` (cases) | One source error per case: announce (database), scrape (whitelist), whitelist, authentication (key expired), already converted by `From`. | Read the variant and the message of the converted error. | The variant is the case's, and the message starts with the case's area prefix. | Matches the code. The conversion is built in the case expression. |

Smells found: none. The test has no AAA comments: a four-line body separated by blank lines.

## Results

- **Coverage** (stable Rust 1.100.0, `cargo-llvm-cov 0.9.1`, `--all-features --lib`, clean coverage
  target directory, 2026-11-23): the file 38 / 50 lines to 69 / 69, regions 33 / 45 to 58 / 58,
  functions 4 / 8 to 11 / 11 (the counts include the test module). The package's unit-only lines
  5,014 / 5,787 (86.64%) to 5,045 / 5,806 (86.89%), regions 5,528 / 6,441 (85.83%), functions
  876 / 1,072 (81.72%).
- **Tests:** 4 cases added, 4 existing tests unchanged: 267 to 271 unit tests in the package.
- **Mutations** (each applied alone and reverted, the file compared with its copy afterwards):

  | ID | Result |
  | --- | --- |
  | M1 | Killed by the announce case |
  | M2 | Killed by the scrape case |
  | M3 | Killed by the whitelist case |
  | M4 | Killed by the authentication case |

  As planned, no mutant of the conversions themselves exists: a `From` that wraps its source in another variant does not compile.
- **Review outcome:** the maintainer approved the completed file on 2026-11-23.
- **Validation:** `cargo clippy -p torrust-tracker-core --all-targets --all-features` (only the
  existing `axum::serve` warning), `cargo +nightly fmt --all -- --check` and
  `cargo machete --with-metadata` pass.
