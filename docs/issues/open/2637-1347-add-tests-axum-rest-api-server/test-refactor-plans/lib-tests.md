---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/lib.rs
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/lib.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
    - docs/pr-reviews/pr-2638-review/PR-REVIEW.md
---

# Crate Root File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/axum-rest-api-server/src/lib.rs`, ledger row 21.
It proposes no new test and one documentation correction. Status: done; the maintainer approved the plan on 2026-11-06.

## Current State

- **Unit-only coverage:** no executable lines. The file declares the modules, a clock type alias, the `API_LOG_TARGET`
  constant, the `InfoHashParam` and `Version` types (derives only) and carries the crate documentation.
- **Uses:** `InfoHashParam` is the path extractor of the whitelist handlers (real extraction is exercised by the whitelist
  integration tests; the handler unit tests build it directly). `API_LOG_TARGET` is the `tracing` target of the router and
  server logs. `Version` has no use in this repository (a search of all packages finds none).
- **Documentation defect (recorded by the review of #2638):** the crate documentation spells the TLS table
  `[http_api.tsl_config]` in two TOML examples and names "the `tsl_config` section" once (lines 30, 109 and 116). The v3
  `HttpApi` type defines `tls_config` and denies unknown fields, so a configuration copied from the documentation does not
  start the tracker.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| Module declarations, the clock aliases | No executable entries | No test. |
| `InfoHashParam` (`Deserialize` derive) | Collaborator-owned | No test: the derive belongs to `serde`, the value is validated by the handler (tested in S3) and the extraction is exercised by the whitelist integration tests. Removing the derive does not compile, so the compiler guards it. |
| `API_LOG_TARGET = "API"` | Logging-only | No test: a test would assert the literal against itself. A changed target changes log filtering for operators but not API behavior; recorded here as the decision, not pinned. |
| `Version` (`Serialize`, `Deserialize`, one variant) | Unused public API | No test: no consumer exists, so a wire-format test would pin a contract nobody uses. A candidate for removal in a separate decision (recorded in `lessons.md`, not a #2637 change). |
| The crate documentation | Documentation correction | Fix `tsl_config` to `tls_config` in the three places (the only change of this plan). The other `tsl_config` spellings of the repository (`docs/containers.md`, a skill) are outside this package and are recorded in `lessons.md` as follow-ups. |

The sentence at lines 130-134 (a dangling "options in the configuration file with the paths ...") reads badly after the link
to `HttpApi::tls_config`; fixing the wording is optional and is a question for the maintainer (default: leave it).

## Challenge Of The No-Change Conclusion

Named mutations to apply after approval, each alone and reverted exactly: L1 change `API_LOG_TARGET` to `"OTHER"` (expected
to survive, which confirms the decision (it did not: see Results, the integration tests pin the log target)); L2 remove the `Deserialize` derive of `InfoHashParam` (expected compile error:
guarded by the compiler); L3 rename `Version::V1` (expected to survive: unused). The documentation fix is verified by
`grep -n tsl_config packages/axum-rest-api-server/src/lib.rs` returning nothing, and by parsing the corrected TOML example
into the v3 configuration (a disposable script, not a committed test).

## Property-Test Decision

None.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No tests to refactor. | None. |
| R2 | SKIPPED | No tests added. | None. |
| R3 | SKIPPED | Integration increment: `none`. | None. |
| R4 | DONE | Fix the three `tsl_config` spellings in the crate documentation; run the mutations; re-run the documentation build (`cargo doc -p torrust-tracker-axum-rest-api-server --no-deps`). | The grep, the mutation results and a clean `cargo doc`. |

## Prose-First AAA Review

No test is added or changed.

## Results

The maintainer approved the plan on 2026-11-06. The three `tsl_config` spellings of the crate documentation are now `tls_config`; `grep -n tsl_config packages/axum-rest-api-server/src/lib.rs` returns nothing and `cargo doc -p torrust-tracker-axum-rest-api-server --no-deps` builds. A disposable program outside the repository (not committed) parsed the corrected example into the v3 `HttpApi`: `tls_config` parses and sets the TLS option, while the old `tsl_config` spelling is rejected with a TOML parse error (unknown field), which confirms the defect and the fix. No test was added.

Mutations (each alone, then `lib.rs` restored exactly): **L1** `API_LOG_TARGET` changed to `"OTHER"`: the plan expected a survivor and was wrong. The unit tests pass, but 21 of the 58 integration tests fail: the authentication and the auth-key, whitelist, stats and torrent failure cases assert that the error log contains a line with `ERROR`, `API` and the request id (`logs_contains_a_line_with`), so the target value is pinned end to end. **L2** removing the `Deserialize` derive of `InfoHashParam`: compile error (guarded by the compiler). **L3** renaming `Version::V1`: all 167 unit and 58 integration tests pass, so `Version` has no consumer, as recorded.

## Progress Log

- 2026-11-06 10:39 UTC - Claude Code - Plan drafted; waiting for the maintainer's approval.
- 2026-11-06 11:15 UTC - Claude Code - Plan approved by the maintainer; decision recorded, rows `DONE`.
