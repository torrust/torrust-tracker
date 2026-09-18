---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/pr-reviews/pr-2174-copilot-suggestions-legacy/PR-REVIEW.md
---

<!-- cspell:disable -->

# PR #2174 Review Feedback Tracking

Source: pull-request reviews and inline review comments for <https://github.com/torrust/torrust-tracker/pull/2174>.

## Purpose

Track Cameron's (`da2ce7`) maintainer reviews independently from the Copilot review-thread audit
in [pr-2174-copilot-suggestions-legacy/PR-REVIEW.md](../pr-2174-copilot-suggestions-legacy/PR-REVIEW.md), which
completed all six Copilot threads on 2026-09-09. Cameron submitted four `CHANGES_REQUESTED`
reviews; the recurring formatting blocker and validation-record findings span all four.

## Root Cause Note

Every recorded `cargo fmt --all -- --check ... passed` claim challenged by these reviews came from
**stable** rustfmt, which only warns about the repository's unstable
`imports_granularity`/`group_imports` options. CI and Cameron's environment use **nightly**
rustfmt, which enforces them. Commit `style(udp-server): fix rustfmt import grouping` applies the nightly formatting; validation and the
plan documents were corrected in `docs(udp-server): correct formatting validation records`.

## Commit Citation Note

Fix commits in this document are cited by their unique Conventional Commit subject instead of a
SHA. The branch was rebased onto `develop` twice on 2026-09-14 after the fix commits were created,
so any SHA recorded here or in the twenty pre-rebase inline thread replies (written 11:23-11:30
UTC, before the 12:27 and 16:02 UTC force-pushes) no longer resolves against the branch. Locate
any cited commit with `git log --oneline --fixed-strings --grep='<subject>'`; each subject matches
exactly one commit on this branch.

## Reviews

| Review ID  | Submitted at (UTC)  | Reviewer | State             | URL                                                                                 | Reviewed commit | Consolidated response URL                                                      | Response state |
| ---------- | ------------------- | -------- | ----------------- | ----------------------------------------------------------------------------------- | --------------- | ------------------------------------------------------------------------------ | -------------- |
| 5155500231 | 2026-09-09 14:11:57 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155500231> | `35294e14`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |
| 5155990517 | 2026-09-09 14:51:38 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155990517> | `f7b355d7`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |
| 5156330106 | 2026-09-09 15:21:06 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5156330106> | `954d4d20`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |
| 5181640522 | 2026-09-11 17:25:56 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5181640522> | `598e5f57`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593> | POSTED         |
| 5200096977 | 2026-09-14 16:12:46 | da2ce7   | CHANGES_REQUESTED | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5200096977> | `2c537182`      | <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5667238096> | POSTED         |

## Findings

| ID  | Review ID  | Source | Comment / thread ID     | URL                                                                           | Summary                                                                                       | Decision  | Independent fix commit | Validation                                        | Reply URL | Inline thread state | Status |
| --- | ---------- | ------ | ----------------------- | ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------- | --------- | ---------------------- | -------------------------------------------------- | --------- | ------------------- | ------ |
| F1  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSne` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415173> | Blocker: rustfmt import grouping in `handlers/mod.rs`.                                          | ACTION    | `style(udp-server): fix rustfmt import grouping`             | `cargo +nightly fmt --all -- --check`; full suite  | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004644462> | RESOLVED                | DONE |
| F2  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSnk` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415180> | Blocker: rustfmt import grouping in `server/request_buffer.rs`.                                 | ACTION    | `style(udp-server): fix rustfmt import grouping`             | `cargo +nightly fmt --all -- --check`; full suite  | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690409> | RESOLVED                | DONE |
| F3  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSno` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415189> | Major: eviction test cannot detect over-eviction because retained tasks are checked after drop. | ACTION    | `test(udp-server): assert retained tasks survive eviction`             | Reviewer's break-to-continue mutation now fails the test; full suite | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690616> | RESOLVED | DONE |
| F4  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSnx` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415205> | Nit: table-driven request-kind test vs prose-first AAA convention.                              | ACTION    | `docs(udp-server): record table-form test rationale`             | Plan records the reviewed table-form rationale     | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690765> | RESOLVED                | DONE |
| F5  | 5155500231 | Inline | `PRRT_kwDOGp2yqc6gsSn3` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969415214> | Minor: markdown format-on-save disabled silently in shared editor settings.                     | ACTION    | `chore(vscode): explain markdown format-on-save opt-out`             | Reviewable comment added in `.vscode/settings.json` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004690962> | RESOLVED                | DONE |
| F6  | 5155990517 | Inline | `PRRT_kwDOGp2yqc6gtUXI` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969816901> | Major: launcher R1 validation row records a formatting pass that did not hold.                  | ACTION    | `docs(udp-server): correct formatting validation records`             | Corrected row names the stable/nightly divergence and fix commit | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004693781> | RESOLVED | DONE |
| F7  | 5155990517 | Inline | `PRRT_kwDOGp2yqc6gtUXL` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3969816911> | Nit: construction-named launcher fixture vs causal-state naming pattern.                        | NO_ACTION | N/A                    | Superseded: the fixture was renamed `UdpLauncherTestContext` in the reviewed final series; outdated thread | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694005> | RESOLVED | DONE |
| F8  | 5156330106 | Inline | `PRRT_kwDOGp2yqc6guDya` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3970108158> | Major: launcher R2 validation row repeats the false formatting pass.                            | ACTION    | `docs(udp-server): correct formatting validation records`             | Corrected row names the stable/nightly divergence and fix commit | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694159> | RESOLVED | DONE |
| F9  | 5156330106 | Inline | `PRRT_kwDOGp2yqc6guDyh` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3970108166> | Nit: fixture takes a `bind_address` argument that does no work.                                 | ACTION    | `test(udp-server): clarify inert admission test inputs`             | Parameterless constant binding; focused launcher tests | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694327> | RESOLVED | DONE |
| F10 | 5156330106 | Inline | `PRRT_kwDOGp2yqc6guDym` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3970108172> | Nit: inert `Strict` policy argument reads as causal in port-zero tests.                         | ACTION    | `test(udp-server): clarify inert admission test inputs`             | Guard-ahead-of-policy comments at both Acts        | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004694560> | RESOLVED                | DONE |
| F11 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqZ1` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763351> | Blocker: third rustfmt violation in `statistics/event/handler/error.rs`.                        | ACTION    | `style(udp-server): fix rustfmt import grouping`             | `cargo +nightly fmt --all -- --check`; full suite  | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004696631> | RESOLVED                | DONE |
| F12 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqZ4` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763356> | Blocker: error-metric R2 validation row recorded the false formatting pass.                     | ACTION    | `docs(udp-server): correct formatting validation records`             | Corrected row; plan README records the general correction | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004696864> | RESOLVED | DONE |
| F13 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqaA` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763364> | Blocker: handler-error R1/R2 validation row recorded the false formatting pass.                 | ACTION    | `docs(udp-server): correct formatting validation records`             | Corrected row; plan README records the general correction | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004697128> | RESOLVED | DONE |
| F14 | 5181640522 | Inline | `PRRT_kwDOGp2yqc6hkqaD` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r3991763371> | Suggestion: bare `should_` prefixes in new tests vs `it_should_` skill rule.                    | ACTION    | `test(udp-server): apply it_should naming convention`             | Six new tests renamed; plans updated; full suite   | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4004697363> | RESOLVED                | DONE |
| F15 | 5155500231 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155500231> | Round-1 summary: formatting gate fails at head; items enumerated inline.                   | ACTION    | `style(udp-server): fix rustfmt import grouping`             | See F1, F2                                          | N/A       | NOT_APPLICABLE      | DONE |
| F16 | 5155990517 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5155990517> | Round-2 summary: standing formatting blocker plus new validation-record finding.           | ACTION    | `style(udp-server): fix rustfmt import grouping`, `docs(udp-server): correct formatting validation records` | See F6                                              | N/A       | NOT_APPLICABLE      | DONE |
| F17 | 5156330106 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5156330106> | Round-3 summary: blocker unresolved four commits on; port-zero test credited.              | ACTION    | `style(udp-server): fix rustfmt import grouping`, `docs(udp-server): correct formatting validation records` | See F8                                              | N/A       | NOT_APPLICABLE      | DONE |
| F18 | 5181640522 | Review body | N/A                | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5181640522> | Round-4 summary: blocker grew to three files; 25 false validation rows; pre-commit hook question. | ACTION | `style(udp-server): fix rustfmt import grouping`, `docs(udp-server): correct formatting validation records` | Root cause (stable vs nightly rustfmt) recorded here and in plans README | N/A | NOT_APPLICABLE | DONE |
| F19 | 5200096977 | Inline | `PRRT_kwDOGp2yqc6iL8un` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007168064> | Major: audit commit ids unreachable after rebases; replies carry stale ids.                     | ACTION    | `docs(review): cite fix commits by stable subject` | SHAs replaced by unique subjects; provenance note added for the pre-rebase replies | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007286903> | RESOLVED | DONE |
| F20 | 5200096977 | Inline | `PRRT_kwDOGp2yqc6iL8uw` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007168076> | Major: sixth bare-named test (`receiver.rs`) missed; round-4 thread resolved on a false claim. | ACTION    | `test(udp-server): rename bare receiver test prefix` | Renamed test and plan row; focused receiver test passes | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007287125> | RESOLVED | DONE |
| F21 | 5200096977 | Inline | `PRRT_kwDOGp2yqc6iL8u1` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007168083> | Suggestion: uncorrected validation rows do not self-describe the stable-rustfmt caveat.        | ACTION    | `docs(udp-server): mark stable-rustfmt validation rows` | Pointer added under every affected plan's Validation Evidence heading | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007287315> | RESOLVED | DONE |
| F22 | 5200096977 | Inline | `PRRT_kwDOGp2yqc6iL8u7` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007168089> | Nit: `RawRequest` `PartialEq`/`Eq` derive is a public API addition unmentioned in plans.       | ACTION    | `docs(udp-server): record RawRequest derive decision` | Receiver plan records the derive as a deliberate public API addition | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007287572> | RESOLVED | DONE |
| F23 | 5200096977 | Inline | `PRRT_kwDOGp2yqc6iL8vT` | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007168120> | Credit: eviction fix verified independently; doc comment prevents regression.                   | NO_ACTION | N/A | Reviewer confirmation; no change requested | <https://github.com/torrust/torrust-tracker/pull/2174#discussion_r4007287784> | RESOLVED | DONE |
| F24 | 5200096977 | Review body | N/A            | <https://github.com/torrust/torrust-tracker/pull/2174#pullrequestreview-5200096977> | Round-5 summary: rounds 1-4 substance closed; two record-accuracy Majors remain (F19, F20).     | ACTION    | See F19-F22 | Rebase-stable citations, receiver rename, row pointers, derive record | N/A | NOT_APPLICABLE | DONE |

## Processing Log

- 2026-09-14 - Started Cameron review audit after completing the Copilot-suggestion workflow on
  2026-09-09. Fetched all four review IDs and the 14 unresolved inline threads by GraphQL.
- 2026-09-14 - Diagnosed the recurring blocker's root cause: stable rustfmt does not enforce the
  repository's unstable import-grouping options, so local gate runs reported false formatting
  passes while CI's nightly rustfmt failed. Recorded in the plans README and this audit.
- 2026-09-14 - Committed independent fixes: `style(udp-server): fix rustfmt import grouping` (formatting, F1/F2/F11), `docs(udp-server): correct formatting validation records`
  (validation-record corrections, F6/F8/F12/F13), `test(udp-server): assert retained tasks survive eviction` (eviction liveness assertion, F3),
  `test(udp-server): clarify inert admission test inputs` (launcher fixture parameter and inert-policy comments, F9/F10), `test(udp-server): apply it_should naming convention`
  (`it_should_` renames, F14), `docs(udp-server): record table-form test rationale` (table-form rationale, F4), `chore(vscode): explain markdown format-on-save opt-out`
  (settings comment, F5). `docs(issues): repair archived issue links in EPIC` separately repaired stale archived EPIC links that failed
  the local lint gate during this session.
- 2026-09-14 - Replied to all fourteen inline threads with their fix commits, resolved every
  thread, and posted the single consolidated response covering all four reviews at
  <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5663269593>. The eviction
  fix was verified by re-applying the reviewer's break-to-continue mutation and observing the
  strengthened test fail before reverting the mutation.
- 2026-09-14 - Round 5 (review 5200096977) processed: rebase-stable subject citations and a
  Commit Citation Note replace all SHA references; the missed `receiver.rs` bare-named test was
  renamed with its plan row; all sixteen plans carry a standalone stable-rustfmt pointer; the
  `RawRequest` derive is recorded as the issue's one deliberate public API addition. Fixes pushed
  fast-forward, all five threads replied and resolved, consolidated response posted at
  <https://github.com/torrust/torrust-tracker/pull/2174#issuecomment-5667238096>.

## Migration Note

Moved to the unified audit directory by issue #2219. The companion
`pr-2174-copilot-suggestions-legacy.md` preserves the separately completed Copilot audit.
