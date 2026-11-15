---
doc-type: file-test-plan
issue: 2824
package: torrust-tracker-primitives
target-file: packages/primitives/src/swarm_metadata.rs
status: done
last-updated-utc: "2026-11-14 22:49"
---

# `swarm_metadata.rs` File Test Plan

Follow the shared [guidance](README.md). Ledger row 7, slice S4. Status: **done**: approved by the maintainer on 2026-11-14 (first gate, as written)
and the completed file approved on 2026-11-14 (second gate).

## Current State

- **Unit-only coverage:** 3 / 12 lines, measured 2026-11-14 on `develop` after #2852 (package: 1,049 / 1,126 lines,
  105 unit tests). The file has no tests. The baseline row said 3 / 18: #2823 deleted the `AddAssign` impl of
  `AggregateActiveSwarmMetadata` (6 lines), so that group no longer exists.
- **Review of the existing tests:** none in the file. `zeroed` is covered only because the `scrape.rs` test builds a
  `ScrapeData::zeroed` and compares it with another value built from `SwarmMetadata::zeroed`, so its value is never
  checked: `zeroed` returning `new(1, 0, 0)` would pass every test of the package.
- **Hypothesis:** the risk is a mix-up of the three counts. The fields carry the scrape names (`downloaded`, `complete`,
  `incomplete`) and the accessors the domain names (`downloads`, `seeders`, `leechers`); a swap of `seeders` and
  `leechers` would report the wrong counts to every client, and no test in this package would see it. Seven calls in
  three other packages build the value with `new(downloaded, complete, incomplete)`, so the argument order is a
  contract too.

## Coverage And Decision Analysis

| Behavior group | Lines | Classification | Decision |
| --- | --- | --- | --- |
| `downloads`, `seeders`, `leechers` accessors | 39-41, 44-46, 49-51 | Unit | Three tests, one per accessor, on one value built with `new(3, 5, 7)` (three different counts): each accessor returns its own literal. A swap of two accessors, or of two fields in the struct (which reorders the `new` arguments), fails a test. |
| `zeroed` | 34-36 (covered, value unchecked) | Unit | One test: `zeroed()` equals the value built with `new(0, 0, 0)`. It is what a scrape answers for an unknown torrent. |
| `AggregateActiveSwarmMetadata` | none (derives only) | Unselected | Four public fields and derived traits, no executable line. The registry fills it (`get_aggregate_swarm_metadata`) and owns its sums. |
| Derived `Ord`, `Hash`, `Default`, `Constructor` | 17 | Unselected | Generated code. `Default` is reached through `zeroed`; I found no ordering of `SwarmMetadata` in the production code of other packages. |

No finding blocks tests and nothing is owned elsewhere (the `AddAssign` decision of F4 is closed by #2823).

## Property-Test Decision

None. Three independent copies of a field; a generated input would repeat the examples.

## Questions For The Maintainer

- None. The four tests check package-owned behavior and none pins a finding.

## Results (2026-11-14)

Toolchains: stable Rust 1.99.1 for tests and coverage, `cargo-llvm-cov 0.6.16`, nightly `cargo fmt`.

| Measure | Before | After |
| --- | ---: | ---: |
| Unit tests of the package | 105 | 109 (4 added, all in `swarm_metadata.rs`) |
| `swarm_metadata.rs` lines | 3 / 12 | 31 / 31 (the line count includes the new test module) |
| Package unit-only lines | 1,049 / 1,126 (93.16%) | 1,077 / 1,145 (94.06%) |
| Package unit-only regions | 1,659 / 1,752 (94.69%) | 1,698 / 1,782 (95.29%) |
| Package unit-only functions | 203 / 219 (92.69%) | 210 / 223 (94.17%) |

Applied as planned, no deviation. The file moved by +28 covered and +19 lines, which is the whole package change.
The baseline is 5 covered lines above the S3 close (1,044 / 1,121) with no source change in between: the S3 review
commit `test(primitives): [#2824] compare serialized JSON by value, not by string` changed tests in `announce.rs`,
`mode.rs` and `policy.rs` after the S3 figures were taken. A first baseline run in the shared target directory read
stale profile data (it listed lines 75-80 of this 70-line file, the deleted `AddAssign`); every figure here comes from
a separate `CARGO_LLVM_COV_TARGET_DIR` after `cargo llvm-cov clean --workspace`.

### Mutation sample (6 mutants, each applied alone and reverted exactly)

| Mutant | Result | Killed by |
| --- | --- | --- |
| `seeders` returns `incomplete` | killed | the seeders test |
| `leechers` returns `complete` | killed | the leechers test |
| `downloads` returns `complete` | killed | the downloads test |
| `zeroed` returns `new(1, 0, 0)` | killed | the zeroed test |
| `zeroed` returns `new(0, 0, 1)` | killed | the zeroed test |
| `complete` and `incomplete` swapped in the struct (reorders the `new` arguments) | killed | the seeders and leechers tests |

No survivor. `cmp` against a saved copy showed the file restored byte for byte after every mutant.

## Prose-First AAA Review (every added test)

Temporary prose per test was written and compared with the code; the code expresses it.

| Test | Arrange (the one causal state) | Act | Assert (literal, independent) | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_return_the_number_of_downloads` | `new(3, 5, 7)`, three different counts | `downloads()` | `3` | clean |
| `it_should_return_the_number_of_seeders` | `new(3, 5, 7)` | `seeders()` | `5` | clean |
| `it_should_return_the_number_of_leechers` | `new(3, 5, 7)` | `leechers()` | `7` | clean |
| `it_should_build_a_zeroed_value_with_every_count_at_zero` | none | `SwarmMetadata::zeroed()` | the value built with `new(0, 0, 0)` | clean; the expectation is built with `new`, not `default`, so it does not repeat the production path |

Test smells found and fixed while writing: none.

After review of #2864 (finding F5 of its audit record): the four tests separated the sections with blank lines only;
they now carry the `// Arrange`, `// Act` and `// Assert` comments the `write-unit-test` checklist asks for
(`zeroed` has no arrange step, so it has `// Act` and `// Assert`).

## Validation

`cargo test -p torrust-tracker-primitives` (109 + 2 + 1 passed); `cargo +nightly fmt --all -- --check`;
`cargo clippy -p torrust-tracker-primitives --all-targets`; `git diff --check`; the pre-commit gate (at commit);
unit-only coverage before and after in the table above.
