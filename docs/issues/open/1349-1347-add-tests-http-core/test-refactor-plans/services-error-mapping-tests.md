---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/services/error_mapping.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/services/error_mapping.rs
    - packages/tracker-core/src/error.rs
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Error Mapping File Test Plan

Follow the shared [guidance](README.md). This plan covers only
`packages/http-core/src/services/error_mapping.rs`. The announce plan,
[services-announce-tests.md](services-announce-tests.md), deliberately left the failure-reason text of
`TrackerCoreError` to this plan.

## Current State

- **Unit-only coverage:** 0 / 18 lines (0.00%), 0 / 13 regions, 0 / 1 function
  ([coverage evidence](../coverage-evidence.md), `87690b15e`).
- **Existing tests:** none in the file. `services/announce.rs` has one test that asserts the whitelist
  prefix of this mapping, to show that `HttpAnnounceError` delegates here; it does not pin the text.
- **Module-owned decision:** the client-visible failure reason of each `TrackerCoreError` variant: a
  fixed package-owned prefix naming the failing area, followed by the wrapped error's own message.
  The function is `pub` and is called by `services/announce.rs` and `services/scrape.rs`.
- **T1 hypothesis:** test candidate.

## Current Tests Review

There are no tests in the file to review, so R1 is skipped.

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Four named arms: `AnnounceError`, `ScrapeError`, `WhitelistError` and `AuthenticationError` (lines 6-17) | 3A unit | R2: one table with a case per variant. Each case builds the variant from one equal construction of its source and expects the literal prefix (`Tracker announce error:`, `Tracker scrape error:`, `Tracker whitelist error:`, `Tracker authentication error:`), a space, and the same source's own text. The prefix is package-owned and written as a literal. The wrapped text is the collaborator's: it contains a `Location`, so the test builds the expected suffix from a second, equal construction instead of copying production code. |
| Wildcard arm for an unknown variant, `Tracker error: {unknown}` (lines 18-20) | Collaborator-owned | Not testable here. `TrackerCoreError` is `#[non_exhaustive]` and has exactly four variants, so no code outside `tracker-core` can build a fifth. The arm is the forward-compatibility path added by #2454. It stays uncovered (lines 18-20) and is named in the Results. Revisit it when `tracker-core` adds a variant: then add a fifth case. |
| Source texts and their `Location` values | Collaborator-owned | Not asserted beyond being appended; the owners are `tracker-core` (`TrackerCoreError`, `AnnounceError`, `ScrapeError`, `WhitelistError`) and its authentication module. |

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No existing tests. | None. |
| R2 | DONE | Add the table of four cases in an in-file `#[cfg(test)]` module, with `rstest` (already a dev-dependency). | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review, and the named mutations E1 and E2. Done on 2026-10-20 (see Results). |
| R3 | DONE | Select an integration increment, or `none`. | `none`: the function is pure and has no boundary of its own. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results, below. `coverage-evidence.md` is updated with the completed-file review. |

## Named Mutations

Each is a temporary hand-made change that its test must catch. It is reverted after the run, and the
result is recorded in Results. Lines are those of `services/error_mapping.rs` at `87690b15e`.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| E1 | The announce case | The `AnnounceError` arm (line 6) | Use the prefix `Tracker scrape error:` instead of the announce one. |
| E2 | The authentication case | The `AuthenticationError` arm (line 15) | Drop the wrapped text: change the pattern to `{ source: _ }` and the reason to `"Tracker authentication error".to_string()`. Leaving `source` unused does not compile under the workspace lints (`unused variable`); that would be a compile error, not a caught mutant, unless `RUSTFLAGS="--cap-lints=warn"` is set. |

Gates: this plan needs the maintainer's approval first; the completed file needs the maintainer's
completed-file review before its one signed commit.

## Prose-First AAA Review

Temporary prose, written before comparing with the code.

| Test (cases) | Arrange prose | Act prose | Assert prose | Outcome |
| --- | --- | --- | --- | --- |
| `it_should_report_each_tracker_core_error_with_the_prefix_of_its_own_area` (announce, scrape, whitelist, authentication) | A tracker-core error of the case's variant, wrapping a not-whitelisted error or a missing-key error; and the text expected for it: the area's literal prefix and a space, then the wrapped error's own text, built from a second equal construction. | Map the error to a protocol response. | The failure reason equals the expected text. | Matches the code. The Arrange lives in the case attributes (rstest), so the body holds only the Act and the Assert; the two helpers, `not_whitelisted` and `missing_key`, name the one fact each case varies. |

## Results

### R2-R4 (2026-10-20, stable Rust 1.99.0, nightly `rustfmt`)

- `cargo test -p torrust-tracker-http-core`: 77 unit tests pass (4 new cases; the other 73 are
  unchanged) and the 3 doc tests pass. `cargo +nightly fmt --all -- --check`, `cargo clippy -p
  torrust-tracker-http-core --all-targets --all-features -- -D warnings` and `git diff --check` pass.
  The diff only adds lines after the function: production code is unchanged.
- E1 (the announce arm uses the scrape prefix): fails the `announce` case only. E2 (the authentication
  arm drops the wrapped text): fails the `authentication` case only. Both were reverted.
- The wildcard arm (lines 18-20) is the only uncovered production code, as the plan decided: it
  cannot be reached while `TrackerCoreError` has four variants. It is `tracker-core`'s to unlock.

Unit-only coverage (`cargo llvm-cov clean --workspace`, then `cargo llvm-cov -p
torrust-tracker-http-core --all-features --lib`, `cargo-llvm-cov 0.6.16`, stable Rust 1.99.0):

| Scope | Before (`87690b15e`) | After |
| --- | ---: | ---: |
| `services/error_mapping.rs` lines | 0 / 18 (0.00%) | 26 / 29 (89.66%) |
| `services/error_mapping.rs` regions | 0 / 13 | 17 / 19 (89.47%) |
| `services/error_mapping.rs` functions | 0 / 1 | 3 / 3 (100.00%) |

The package totals after slice S2 are in the
[coverage evidence](../coverage-evidence.md#package-after-slice-s2); they include the later announce
cases.

The totals include the test module. Of the file's 29 lines, 18 are production lines (the same 18 as
before: 15 now covered and the 3 wildcard lines uncovered) and 11 are test-module lines, all covered.

## Approval

Approved by Jose Celano on 2026-10-20 (13:21 UTC): R1 skipped, R2 as one table of four cases, R3 `none`, mutations E1 and E2, and the wildcard arm recorded as `tracker-core`-owned and not testable.

## Progress Log

- 2026-10-20 13:01 UTC - GitHub Copilot - Created the plan from `services/error_mapping.rs`, the S2
  re-measurement at `87690b15e`, and `TrackerCoreError`. The wildcard arm added by #2454 cannot be
  reached by a test, which corrects the earlier note that suggested adding one. No Rust test or
  production code changed. The plan awaits maintainer approval.
- 2026-10-20 13:21 UTC - Jose Celano - Approved the plan. R2 starts.
- 2026-10-20 13:24 UTC - GitHub Copilot - R2-R4 done: four cases, mutations E1 and E2 each fail their
  own case, the file is at 26 / 29 lines with only the wildcard arm uncovered. The completed file awaits
  the maintainer's review.
- 2026-10-20 14:56 UTC - Jose Celano - Approved the completed file. The plan is `done`.
- 2026-10-20 16:05 UTC - GitHub Copilot - The S2 Task Reviewer found that mutation E2 as written does
  not compile under the workspace lints (LOW). Reproduced: a plain unused `source` fails with `unused
  variable`. The run recorded above used the `{ source: _ }` form, which compiles, so its result stands;
  the Named Mutations row now says so.
