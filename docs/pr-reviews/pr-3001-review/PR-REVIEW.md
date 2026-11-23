---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #3001 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3001.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

F1 and F2 come from Copilot's review 6021313664 (COMMENTED on the first head, "docs(issues): [#2917] record the SI-5 verification evidence and the configuration 4.0.0 breaking change"), in source order. Copilot noted that its full agentic review did not start before its timeout; the two findings are its inline threads. F3 to F6 come from Copilot's review 6022021456 (COMMENTED on the round-1 head, "docs(pr-reviews): [#2917] add the review audit of PR #3001", with ACK comments of the maintainer on the first head and on that head), in source order; its overview lists F1 and F2 as resolved.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3001-f1 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3001-f2 | Copilot | Nit | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-3001-f3 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3001-f4 | Copilot | Minor | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F5 | review-finding:pr-3001-f5 | Copilot | Minor | testing | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-3001-f6 | Copilot | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The authentication classification test repeats the error's display string as a literal

- PR number: 3001
- Source review ID: 6021313664
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665512621
- Concern: Copilot (rated Medium, recorded as Minor) noted that `it_should_classify_an_authentication_error` in `packages/udp-server/src/event.rs` repeats the `AuthRequired` display string as a literal, although the conversion now uses `error.to_string()`; it suggested comparing against `Error::AuthRequired.to_string()` so a wording change does not need edits at several sites.
- Solution: The assertion now compares with `ErrorKind::TrackerAuthentication(Error::AuthRequired.to_string())`. The wording stays pinned once, in `it_should_not_hold_a_source_path_when_an_authentication_required_error_is_displayed` in `error.rs`.
- Current-tree verification: `grep -n "AuthRequired" packages/udp-server/src/event.rs` shows the comparison with `Error::AuthRequired.to_string()`; `cargo test -p torrust-tracker-udp-server --lib` (stable Rust toolchain) passes 226 tests.
- Resolution reference: test(udp-server): [#2917] compare the authentication classification with the error's own message
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665567589

### F2 - The issue status is `in-progress` while the task table is `DONE`

- PR number: 3001
- Source review ID: 6021313664
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665512674
- Concern: Copilot (rated Low, recorded as Nit) read the front-matter `status: in-progress` of the spec as inconsistent with the `DONE` tasks and acceptance criteria and suggested `done`.
- Solution: No change. The tasks and acceptance criteria describe the implementation, which is finished; the workflow checkpoints for review validation, the committer check and the archive stay unticked until the PR merges. The repository's close-out step (as in SI-2 to SI-4) moves the spec to `docs/issues/closed/` and sets `status: done` then; setting it earlier would claim a closed issue while the PR is open.
- Current-tree verification: `grep -n "^status:" docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` prints `status: in-progress`; `grep -n "Reviewer validated" docs/issues/open/2917-2912-si-5-startup-and-udp-server-errors/ISSUE.md` shows that checkpoint unticked.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665567937
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4665567937

### F3 - The severity values of the findings table are not the documented ones

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666073875
- Concern: Copilot (rated Medium, recorded as Minor) noted that the findings table of this audit used `Medium` and `Low`, which are not in the documented set (`Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`).
- Solution: The table now uses the documented values, mapping Copilot's Medium to Minor and Low to Nit as the audit of PR #2991 did, and the detail entries say so. A sweep of the same mistake found no other non-documented value in this audit.
- Current-tree verification: `grep -n "^| F" docs/pr-reviews/pr-3001-review/PR-REVIEW.md` lists only `Minor` and `Nit`; `validate-audit-record.py --pr-number 3001` (run with the bot identity) reports 0 failures.
- Resolution reference: docs(pr-reviews): [#2917] record round 2 of the review audit of PR #3001
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666135737

### F4 - The `axum-server` path check lacks backslash variants for the registry and standard-library paths

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666073945
- Concern: Copilot (rated Medium, recorded as Minor) noted that the `assert_names_no_source_path` copy in `packages/axum-server/src/tls.rs` forbids `.cargo/registry` and `rustlib/src` but not their Windows spellings `.cargo\\registry` and `rustlib\\src`.
- Solution: No change. A Windows registry or standard-library source path always continues with a `src\\` component (`...\\.cargo\\registry\\src\\<index>\\<crate>\\src\\...`, `...\\rustlib\\src\\rust\\library\\...`), and the helper already forbids `src\\`, so both spellings are caught. The list is the one the SI-5 specification fixes (Regression Test Strategy, "Portable path check") and the same as the shared helper that SI-8 (#2996) moves into `torrust-tracker-test-helpers`.
- Current-tree verification: `grep -n 'src' packages/axum-server/src/tls.rs` shows `"src\\"` in the helper's list.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136252
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136252

### F5 - The `configuration` path check lacks the same backslash variants

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666074007
- Concern: The same request as F4 for the copy of `assert_names_no_source_path` in `packages/configuration/src/lib.rs`, where it is the first of two copies reported by Copilot.
- Solution: No change, for the reason given on F4: `src\\` is in the list and catches the Windows registry and standard-library paths.
- Current-tree verification: `grep -n 'src' packages/configuration/src/lib.rs` shows `"src\\"` in the helper's list.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136697
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666136697

### F6 - `error @ Error::AuthRequired` is an unnecessary binding

- PR number: 3001
- Source review ID: 6022021456
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666074088
- Concern: Copilot (rated Low, recorded as Nit) noted that the `ErrorKind` conversion in `packages/udp-server/src/event.rs` binds the unit variant with `error @` only to call `to_string()` on it, and asked for the direct pattern.
- Solution: The arm now matches `Error::AuthRequired` and builds the message with `Error::AuthRequired.to_string()`.
- Current-tree verification: `grep -n "AuthRequired" packages/udp-server/src/event.rs` shows the direct pattern; `cargo test -p torrust-tracker-udp-server --lib` (stable Rust toolchain) passes 226 tests and `cargo clippy -p torrust-tracker-udp-server --all-targets -- -D warnings` is clean.
- Resolution reference: refactor(udp-server): [#2917] match the unit variant AuthRequired directly
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3001#discussion_r4666137078

## Processing Log

- 2026-11-23 12:00 UTC - Copilot's review 6021313664 (COMMENTED, submitted 12:00:15Z on the first head) arrived with two inline findings, F1 (Medium, recorded as Minor) and F2 (Low, recorded as Nit). The audit was started before the first reply. F1 fixed in "test(udp-server): [#2917] compare the authentication classification with the error's own message", pushed at 12:04 UTC (pre-push hook passed). F2 declined with a reason (the spec `status` becomes `done` at close-out).
- 2026-11-23 12:06 UTC - Replied on both threads (replies posted 12:05:34 UTC by `date -u`) and resolved both (`resolveReviewThread` returned `true` for each; `date -u` after the resolve commands: 12:06:00). GraphQL refresh: zero unresolved threads on PR #3001.
- 2026-11-23 12:53 UTC - Copilot's review 6022021456 (COMMENTED, submitted 12:53:25Z on the round-1 head) arrived with four inline findings, recorded as F3 to F6; its overview lists F1 and F2 as resolved. All CI checks of that head passed. F6 fixed in "refactor(udp-server): [#2917] match the unit variant AuthRequired directly", pushed at 12:58 UTC (pre-push hook passed). F3 fixed in this audit's round-2 commit; F4 and F5 declined with a reason.
- 2026-11-23 13:00 UTC - Replied on the four threads (replies posted 12:59:28 UTC by `date -u`). The threads are resolved after the audit commit is pushed.
