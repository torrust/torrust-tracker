---
semantic-links:
  skill-links:
    - process-pr-review-feedback
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md
    - docs/copilot-pr-reviews/pr-2174-copilot-suggestions.md
---

<!-- cspell:disable -->

# PR #2174 Review Feedback Tracking

Source: pull-request reviews and inline review comments for <https://github.com/torrust/torrust-tracker/pull/2174>.

## Purpose

Track Cameron's (`da2ce7`) maintainer reviews independently from the Copilot review-thread audit
in [pr-2174-copilot-suggestions.md](../copilot-pr-reviews/pr-2174-copilot-suggestions.md), which
completed all six Copilot threads on 2026-09-09. Cameron submitted four `CHANGES_REQUESTED`
reviews; the recurring formatting blocker and validation-record findings span all four.

## Root Cause Note

Every recorded `cargo fmt --all -- --check ... passed` claim challenged by these reviews came from
**stable** rustfmt, which only warns about the repository's unstable
`imports_granularity`/`group_imports` options. CI and Cameron's environment use **nightly**
rustfmt, which enforces them. Commit `14dc4066` applies the nightly formatting; validation and the
plan documents were corrected in `dccb06b5`.

## Reviews

| Review ID  | Submitted at (UTC)  | Reviewer | State             | URL                                                                                 | Reviewed commit | Consolidated response URL                                                      | Response state |
| ---------- | ------------------- | -------- | ----------------- | ----------------------------------------------------------------------------------- | --------------- | ------------------------------------------------------------------------------ | -------------- |
| 5155500231 | 2026-09-09 14:11:57 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155500231> | `35294e14`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |
| 5155990517 | 2026-09-09 14:51:38 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155990517> | `f7b355d7`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |
| 5156330106 | 2026-09-09 15:21:06 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5156330106> | `954d4d20`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |
| 5181640522 | 2026-09-11 17:25:56 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5181640522> | `598e5f57`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |

## Findings

| ID  | Review ID  | Source | Comment / thread ID     | URL                                                                           | Summary                                                                                       | Decision  | Independent fix commit | Validation                                        | Reply URL | Inline thread state | Status |
| --- | ---------- | ------ | ----------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | --------- | ---------------------- | -------------------------------------------------- | --------- | ------------------- | ------ |
| F1  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSne` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415173> | Blocker: rustfmt import grouping in `handlers/mod.rs`.                                          | ACTION    | `14dc4066`             | `cargo +nightly fmt --all -- --check`; full suite  | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004644462> | RESOLVED                | DONE |
| F2  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSnk` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415180> | Blocker: rustfmt import grouping in `server/request_buffer.rs`.                                 | ACTION    | `14dc4066`             | `cargo +nightly fmt --all -- --check`; full suite  | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690409> | RESOLVED                | DONE |
| F3  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSno` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415189> | Major: eviction test cannot detect over-eviction because retained tasks are checked after drop. | ACTION    | `6630575b`             | Reviewer's break-to-continue mutation now fails the test; full suite | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690616> | RESOLVED | DONE |
| F4  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSnx` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415205> | Nit: table-driven request-kind test vs prose-first AAA convention.                              | ACTION    | `0217a48e`             | Plan records the reviewed table-form rationale     | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690765> | RESOLVED                | DONE |
| F5  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSn3` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415214> | Minor: markdown format-on-save disabled silently in shared editor settings.                     | ACTION    | `688e1bb7`             | Reviewable comment added in `.vscode/settings.json` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690962> | RESOLVED                | DONE |
| F6  | 5155990517 | Inline | `PRRT_kwDOGp2yqc6gtUXI` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969816901> | Major: launcher R1 validation row records a formatting pass that did not hold.                  | ACTION    | `dccb06b5`             | Corrected row names the stable/nightly divergence and fix commit | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004693781> | RESOLVED | DONE |
| F7  | 5155990517 | Inline | `PRRT_kwDOGp2yqc6gtUXL` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969816911> | Nit: construction-named launcher fixture vs causal-state naming pattern.                        | NO_ACTION | N/A                    | Superseded: the fixture was renamed `UdpLauncherTestContext` in the reviewed final series; outdated thread | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694005> | RESOLVED | DONE |
| F8  | 5156330106 | Inline | `PRRT_kwDOGp2yqc6guDya` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3970108158> | Major: launcher R2 validation row repeats the false formatting pass.                            | ACTION    | `dccb06b5`             | Corrected row names the stable/nightly divergence and fix commit | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694159> | RESOLVED | DONE |
| F9  | 5156330106 | Inline | `PRRT_kwDOGp2yqc6guDyh` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3970108166> | Nit: fixture takes a `bind_address` argument that does no work.                                 | ACTION    | `c657dd21`             | Parameterless constant binding; focused launcher tests | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694327> | RESOLVED | DONE |
| F10 | 5156330106 | Inline | `PRRT_kwDOGp2yqc6guDym` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3970108172> | Nit: inert `Strict` policy argument reads as causal in port-zero tests.                         | ACTION    | `c657dd21`             | Guard-ahead-of-policy comments at both Acts        | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694560> | RESOLVED                | DONE |
| F11 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqZ1` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763351> | Blocker: third rustfmt violation in `statistics/event/handler/error.rs`.                        | ACTION    | `14dc4066`             | `cargo +nightly fmt --all -- --check`; full suite  | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004696631> | RESOLVED                | DONE |
| F12 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqZ4` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763356> | Blocker: error-metric R2 validation row recorded the false formatting pass.                     | ACTION    | `dccb06b5`             | Corrected row; plan README records the general correction | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004696864> | RESOLVED | DONE |
| F13 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqaA` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763364> | Blocker: handler-error R1/R2 validation row recorded the false formatting pass.                 | ACTION    | `dccb06b5`             | Corrected row; plan README records the general correction | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004697128> | RESOLVED | DONE |
| F14 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqaD` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763371> | Suggestion: bare `should_` prefixes in new tests vs `it_should_` skill rule.                    | ACTION    | `91a3e1e3`             | Six new tests renamed; plans updated; full suite   | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004697363> | RESOLVED                | DONE |
| F15 | 5155500231 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155500231> | Round-1 summary: formatting gate fails at head; items enumerated inline.                   | ACTION    | `14dc4066`             | See F1, F2                                          | N/A       | NOT_APPLICABLE      | DONE |
| F16 | 5155990517 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155990517> | Round-2 summary: standing formatting blocker plus new validation-record finding.           | ACTION    | `14dc4066`, `dccb06b5` | See F6                                              | N/A       | NOT_APPLICABLE      | DONE |
| F17 | 5156330106 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5156330106> | Round-3 summary: blocker unresolved four commits on; port-zero test credited.              | ACTION    | `14dc4066`, `dccb06b5` | See F8                                              | N/A       | NOT_APPLICABLE      | DONE |
| F18 | 5181640522 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5181640522> | Round-4 summary: blocker grew to three files; 25 false validation rows; pre-commit hook question. | ACTION | `14dc4066`, `dccb06b5` | Root cause (stable vs nightly rustfmt) recorded here and in plans README | N/A | NOT_APPLICABLE | DONE |

## Processing Log

- 2026-09-14 - Started Cameron review audit after completing the Copilot-suggestion workflow on
  2026-09-09. Fetched all four review IDs and the 14 unresolved inline threads by GraphQL.
- 2026-09-14 - Diagnosed the recurring blocker's root cause: stable rustfmt does not enforce the
  repository's unstable import-grouping options, so local gate runs reported false formatting
  passes while CI's nightly rustfmt failed. Recorded in the plans README and this audit.
- 2026-09-14 - Committed independent fixes: `14dc4066` (formatting, F1/F2/F11), `dccb06b5`
  (validation-record corrections, F6/F8/F12/F13), `6630575b` (eviction liveness assertion, F3),
  `c657dd21` (launcher fixture parameter and inert-policy comments, F9/F10), `91a3e1e3`
  (`it_should_` renames, F14), `0217a48e` (table-form rationale, F4), `688e1bb7`
  (settings comment, F5). `64571115` separately repaired stale archived EPIC links that failed
  the local lint gate during this session.
- 2026-09-14 - Replied to all fourteen inline threads with their fix commits, resolved every
  thread, and posted the single consolidated response covering all four reviews at
  <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593>. The eviction
  fix was verified by re-applying the reviewer's break-to-continue mutation and observing the
  strengthened test fail before reverting the mutation.
