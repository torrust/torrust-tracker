---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore characterse -->

# PR #2994 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2994>.

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

Round 1 covers review 6012624473 (Copilot, COMMENTED, submitted 2026-11-22T14:05:07Z, a lite review) with seven inline comments on `docs/issues/open/2993-secrets-and-client-error-text/EPIC.md`. Its body is an overview with no finding beyond the inline comments; its warning that the full agentic review did not start is context and makes no row. The comments carry no `[Severity]` bracket. The audit severity scale has no medium or low level, so this audit maps the overview's `Medium severity` badge (F1 to F3) to `Minor (inferred)` and its `Low severity` badge (F4 to F7) to `Nit (inferred)`, as the PR #2990 audit does. The seven comments are three texts: F2 and F3 repeat F1 on other lines, and F6 and F7 repeat F5, so they are re-raises with their own rows.

Round 2 covers review 6012831750 (Copilot, COMMENTED, submitted 2026-11-22T14:40:22Z, a balanced review) with five inline comments, each with a `[Severity][FindingId]` first line. Its body is a verdict ("Changes recommended") and a list that repeats the five inline findings and the seven threads resolved since round 1; it makes no row. Copilot's IDs F8, F9 and F13 do not collide with F1 to F7 and are kept. Copilot used `F12` for two different findings: the first in source order (comment 4658541296, the SI-A draft) keeps F12, and the other (comment 4658541369, the EPIC evidence) is F14 here, the first number after the highest ID in use, with Copilot's ID in its detail entry. The rows are in source order. Severities are the comments' brackets. The overview's badges (Medium for F8, Low for the other four) differ from the brackets and are not used.

Round 3 covers review 6018472413 (Copilot, COMMENTED, submitted 2026-11-23T08:24:06Z, a lite review of the head that held the round 2 fixes, the merge of SI-4 and the inventory refresh) with six inline comments, which are four distinct findings: F15 (two threads, F19 is the second), F16, F17 and F18 (two threads, F20 is the second). Its body is an overview that repeats them, with a warning that the full agentic review did not start before the timeout. The overview marks F15 Medium and the others Low; they are recorded as Major and Minor.

Round 4 covers review 6020791615 (Copilot, COMMENTED, submitted 2026-11-23T11:21:07Z, a lite review of the head that held the round 3 fixes) with three inline comments, which are two distinct findings: F21 (two threads, F22 is the second) and F23. Its body is an overview that repeats them, with the same warning as round 3 that the full agentic review did not start before the timeout. All three are Low in the overview and are recorded as Minor.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2994-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2994-f2 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F3 | review-finding:pr-2994-f3 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2994-f4 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2994-f5 | Copilot | Nit (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2994-f6 | Copilot | Nit (inferred) | documentation | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2994-f7 | Copilot | Nit (inferred) | documentation | RE_RAISE_OF:F5 | NO_ACTION | SUPERSEDED |
| F8 | review-finding:pr-2994-f8 | Copilot | Major | security | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2994-f12 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2994-f9 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2994-f13 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2994-f14 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2994-f15 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2994-f19 | Copilot | Major | correctness | RE_RAISE_OF:F15 | NO_ACTION | SUPERSEDED |
| F16 | review-finding:pr-2994-f16 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2994-f17 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2994-f18 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2994-f20 | Copilot | Minor | documentation | RE_RAISE_OF:F18 | NO_ACTION | SUPERSEDED |
| F21 | review-finding:pr-2994-f21 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2994-f22 | Copilot | Minor | documentation | RE_RAISE_OF:F21 | NO_ACTION | SUPERSEDED |
| F23 | review-finding:pr-2994-f23 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - A10 and A11 are missing from the A table and the "A1 to A9 were reproduced" sentence is inaccurate

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421443
- Concern: The inventory numbering is inconsistent: A10 and A11 are introduced later but are not in the A table, and "A1 to A9 above were reproduced on a running tracker" is inaccurate because A9 is documentation and A10 and A11 are also part of the verified set. Copilot suggests adding A10 and A11 to the table and making the verification bullet match what was reproduced.
- Solution: The A table has rows A10 and A11. The verification bullet says which items were reproduced on a running tracker (A1, A3, A4; A2 and A5 in the next item; A10 and A11 below), which were confirmed by reading the code (A6, A7, A8), and that A9 is documentation. Copilot's example wording "A1–A8, A10–A11" was not used because A6 to A8 were not reproduced on a running tracker.
- Current-tree verification: `grep -c '^| A1[01] |' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 2; `grep -c 'A1 to A9 above were reproduced' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 0; the verification bullet reads "A1, A3 and A4 were reproduced on a running tracker (A2 and A5 in the next item, A10 and A11 below). A6, A7 and A8 were confirmed by reading the code, and A9 is documentation."
- Resolution reference: `docs(issues): [#2993] list A10 and A11 in the inventory table and state what was reproduced`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463288

### F2 - Same inventory-numbering comment on the first verification bullet

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421459
- Concern: Same text as F1, anchored on original line 76 of the reviewed commit (the verification bullet).
- Solution: No separate change: F1's fix covers it.
- Current-tree verification: Same as F1: `grep -c '^| A1[01] |' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 2; `grep -c 'A1 to A9 above were reproduced' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 0; the verification bullet reads "A1, A3 and A4 were reproduced on a running tracker (A2 and A5 in the next item, A10 and A11 below). A6, A7 and A8 were confirmed by reading the code, and A9 is documentation."
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463401
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463401

### F3 - Same inventory-numbering comment on the A10 and A11 bullets

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421468
- Concern: Same text as F1, anchored on the A11 bullet (line 84 of the current file).
- Solution: No separate change: F1's fix covers it.
- Current-tree verification: Same as F1: `grep -c '^| A1[01] |' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 2; `grep -c 'A1 to A9 above were reproduced' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 0; the verification bullet reads "A1, A3 and A4 were reproduced on a running tracker (A2 and A5 in the next item, A10 and A11 below). A6, A7 and A8 were confirmed by reading the code, and A9 is documentation."
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463527
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463527

### F4 - "Key must be have 32 chars" reads like an error in the spec

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421476
- Concern: The excerpt "Key must be have 32 chars" reads like a grammatical error. Copilot asks to mark it as verbatim (a note or "[sic]") if it is the system's output, or to correct it if paraphrased, and suggests naming this message quality issue in the motivation of SI-B.
- Solution: The A2 reproduction says the text is verbatim, including the grammar slip "must be have", cites the source line, and says SI-A replaces the message. The suggestion to add it to SI-B's motivation was not taken: the message is about a peer key, and In Scope item 2 of the SI-A draft (`docs/issues/drafts/2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md`) owns the new key messages.
- Current-tree verification: `grep -c 'The text is verbatim, including the grammar slip "must be have"' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 1; `packages/tracker-core/src/authentication/key/peer_key.rs:195` reads `#[error("Invalid key length. Key must be have 32 chars")]`; line 54 of the SI-A draft proposes `Invalid authentication key: expected 32 characters, got <n>`.
- Resolution reference: `docs(issues): [#2993] mark the quoted key-length message as verbatim`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463632

### F5 - 'Subissues', 'sub-issue' and 'subissue' are used inconsistently

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421494
- Concern: The document uses 'Subissues', 'sub-issue' and 'subissue'. Copilot recommends one form (for example 'sub-issue' / 'Sub-issues') throughout.
- Solution: No change. Every line with "Subissues" or "subissue" (the `## Subissues` heading, the Delivery Strategy sentence, the Workflow Checkpoints and the Acceptance Criteria items) is unchanged text of the repository template `docs/templates/EPIC.md`, which other EPIC specifications share. The prose written for this specification uses "sub-issue". Changing the template wording is outside this specification-only pull request.
- Current-tree verification: `grep -ci 'subissue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 14 (lines 178, 193, 220 to 225, 261 and 264 to 268), and `grep -qFx` finds each of those 14 lines unchanged in `docs/templates/EPIC.md`; `grep -c 'sub-issue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 11, none of them a template line. `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` has the same `## Subissues` heading.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463773
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463773

### F6 - Same terminology comment on the sub-issue drafts sentence

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421514
- Concern: Same text as F5, anchored on original line 185 of the reviewed commit.
- Solution: No separate change: same disposition as F5.
- Current-tree verification: Same as F5: `grep -ci 'subissue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 14 (lines 178, 193, 220 to 225, 261 and 264 to 268), and `grep -qFx` finds each of those 14 lines unchanged in `docs/templates/EPIC.md`; `grep -c 'sub-issue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 11, none of them a template line. `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` has the same `## Subissues` heading.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463980
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658463980

### F7 - Same terminology comment on the Delivery Strategy sentence

- PR number: 2994
- Source review ID: 6012624473
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658421527
- Concern: Same text as F5, anchored on line 193 ("For each subissue implementation in this EPIC, ...").
- Solution: No separate change: same disposition as F5. Line 193 is a template line.
- Current-tree verification: Same as F5: `grep -ci 'subissue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 14 (lines 178, 193, 220 to 225, 261 and 264 to 268), and `grep -qFx` finds each of those 14 lines unchanged in `docs/templates/EPIC.md`; `grep -c 'sub-issue' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 11, none of them a template line. `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` has the same `## Subissues` heading.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658464189
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658464189

### F8 - A database error can hold a secret and no sub-issue owns its removal

- PR number: 2994
- Source review ID: 6012831750
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658541274
- Concern: Driver diagnostics can contain secrets: adding an existing peer key to MySQL's unique `keys.key` column produces `Duplicate entry '<peer key>' ...`, and `databases/error.rs:130-133` keeps that source. Copilot asks to assign the sanitization of secret-bearing database errors to SI-A, to add a MySQL duplicate-key regression that covers retained errors, replies and logs, and to make SI-B's Out of Scope item and T4 keep only non-secret driver detail.
- Solution: The case is inventory item A12, owned by SI-A. It was reproduced on MySQL 8.0 (section V8 of `docs/issues/open/2993-secrets-and-client-error-text/manual-verification-evidence.md`): the 500 reply and one DEBUG `torrust_located_error` log line hold the key; the PostgreSQL reply and log do not, and the SQLite reply of V5 does not. EPIC: row A12 in the A table, the In Scope item of SI-A, the acceptance criterion and a sentence in maintainer decision 4. SI-A draft: In Scope item 8, a MySQL regression test in the Regression Test Strategy, task T7b, AC9 and manual scenario M7. SI-B draft: the Out of Scope bullet and T4 say that non-secret driver text stays and that SI-A removes messages that hold a secret. The same commit also holds lines of F12 and F13 (see those entries). The background paragraph of the SI-A draft still opens with "items A1 to A11" while it names A12 two sentences later; this was seen during this audit and is not changed in this round.
- Current-tree verification: `git show HEAD:docs/issues/open/2993-secrets-and-client-error-text/EPIC.md | grep -n 'A12'` finds lines 66 (table row), 151 (In Scope), 242 (decision 4) and 271 (acceptance criterion); the same grep on the SI-A draft finds lines 40, 60 (In Scope item 8) and 114 (T7b), with the regression test on line 95, AC9 on line 162 and M7 on line 189; on the SI-B draft, lines 75 (Out of Scope) and 110 (T4) say "non-secret driver text". The evidence file's V8 quotes the MySQL reply and the DEBUG log line with `Duplicate entry 'TestPeerKey000000000000000000003' for key 'keys.key'` and says the PostgreSQL log held 0 lines with the key.
- Resolution reference: `docs(issues): [#2993] assign database errors that hold a secret to SI-A as inventory item A12`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658603687

### F12 - The SI-A draft does not require the incremental test-design review

- PR number: 2994
- Source review ID: 6012831750
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658541296
- Concern: T2 of the SI-A draft batches every regression test and records only red output; nothing requires the recorded prose-first Arrange-Act-Assert review after each passing increment, which `docs/templates/ISSUE.md:119-128` and `create-issue/SKILL.md:180-189` require. Copilot asks for incremental red/green work, the design-review evidence and an explicit maintainer checkpoint.
- Solution: The Implementation Plan of the SI-A draft has a "Test-development loop" paragraph (the `write-unit-test` skill, one behavior area at a time, a recorded test-design review before the next area, a commit per reviewed increment, a stop for maintainer review after the last increment). T2 is a test plan, T8b is the maintainer checkpoint, and AC10 requires the recorded reviews and the checkpoint. Not every line of this fix is in the commit named below. When the round's changes were split into commits by hunk, these lines of the SI-A draft landed in `docs(issues): [#2993] assign database errors that hold a secret to SI-A as inventory item A12` because they are next to A12 lines: the AC10 line, the AC10 row of the Acceptance Verification table, and the reworded cells of the Commit Points table (with the removal of its T2 row).
- Current-tree verification: `git show HEAD:docs/issues/drafts/2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md` has the paragraph on line 101, T2 "Test plan" on line 108, T8b on line 116, AC10 on line 163 and its Acceptance Verification row on line 210. `git show -U0` of the commit named below shows the paragraph, the T2 row and the T8b row for this file; `git show -U0` of the A12 commit shows the AC10 line, the AC10 row and the Commit Points cells.
- Resolution reference: `docs(issues): [#2993] require the incremental test-design review and maintainer checkpoint in both sub-issue drafts`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658603862

### F9 - A duplicate insert cannot reach the database-error reply of the HTTP and UDP trackers

- PR number: 2994
- Source review ID: 6012831750
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658541332
- Concern: The announce handlers of the HTTP and UDP trackers update the in-memory repository and only read persisted download counts (`announce_handler.rs:184-204`); a duplicate-key insert is a REST operation. Copilot asks the SI-B draft to specify a failing persisted-downloads read for the two tracker protocols, with persistence enabled and a torrent absent from memory.
- Solution: The Regression Test Strategy of the SI-B draft has one item for the REST boundary (a duplicate key insert) and one for the HTTP and UDP trackers: `core.tracker_policy.persistent_torrent_completed_stat = true`, an info-hash that is not in memory, and a read that fails in `load_downloads_metric_if_needed` (announce) or `persisted_downloads.load_many` (scrape). Manual scenario M4 names the same setting and an info-hash that is not in memory.
- Current-tree verification: `git show HEAD:docs/issues/drafts/2993-si-b-client-error-text-policy/ISSUE.md` has the two items on lines 94 and 95 and M4 on line 182; `git show HEAD:packages/tracker-core/src/announce_handler.rs | grep -n load_downloads_metric_if_needed` finds the call on line 189 and the function on line 196.
- Resolution reference: `docs(issues): [#2993] reach the tracker database error through a failing persisted-downloads read in SI-B`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658603993

### F13 - The SI-B draft does not require the incremental test-design review

- PR number: 2994
- Source review ID: 6012831750
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658541352
- Concern: The SI-B plan records baseline and red evidence and then goes through the production changes, without the prose-first Arrange-Act-Assert design review after each passing test increment that `docs/templates/ISSUE.md:119-128` and `create-issue/SKILL.md:180-189` require. Copilot asks for the incremental red/green loop, the recorded design review before the next behavior area and a final maintainer checkpoint.
- Solution: The Implementation Plan of the SI-B draft has the same "Test-development loop" paragraph as the SI-A draft, for areas T4 to T7. T3 is "Baseline reproduction and test plan", T8b is the maintainer checkpoint, the Commit Points cells name the red and green runs and the recorded review, and AC7 requires the recorded reviews and the checkpoint. Not every line of this fix is in the commit named below. When the round's changes were split into commits by hunk, the reworded T3 row of the SI-B task table ("Baseline reproduction and test plan") landed in `docs(issues): [#2993] assign database errors that hold a secret to SI-A as inventory item A12`, because it is next to the T4 row that the A12 change touches.
- Current-tree verification: `git show HEAD:docs/issues/drafts/2993-si-b-client-error-text-policy/ISSUE.md` has the paragraph on line 101, T3 on line 109, T8b on line 115, AC7 on line 159 and its Acceptance Verification row on line 201. `git show -U0` of the commit named below shows the paragraph, T8b, the Commit Points rows, AC7 and its row for this file; `git show -U0` of the A12 commit shows the T3 row.
- Resolution reference: `docs(issues): [#2993] require the incremental test-design review and maintainer checkpoint in both sub-issue drafts`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658604115

### F14 - The reproductions have no issue-local manual verification evidence file

- PR number: 2994
- Source review ID: 6012831750
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658541369
- Concern: The claimed reproductions have example output but no issue-local `manual-verification-evidence.md` with the commands, configuration and relevant logs, although SI-A relies on this session as its baseline. Copilot asks to keep the runtime and backend details, commands, redacted results and outcomes in the EPIC's evidence file and to link both drafts to it.
- Solution: `docs/issues/open/2993-secrets-and-client-error-text/manual-verification-evidence.md` records a new run of the reproductions on a fresh debug build whose code equals `develop` at `98fa27df2`, with `evidence/reproduce.sh`, `evidence/udp_probe.py` and `evidence/output.txt`. The EPIC and both drafts link it. The commit also adds the EPIC's `evidence/*.txt` to `ignorePaths` in `cspell.json`. Limits, as the evidence file states them: A6, A7 and A8 were confirmed by reading the code, not run; the expired-key reply was not requested; the script was edited for ShellCheck after the run and the edited script was not run again. The link in the background paragraph of the SI-A draft is in `docs(issues): [#2993] assign database errors that hold a secret to SI-A as inventory item A12`; the commit named below links the file from the SI-A Regression Test Strategy, the SI-B background and the EPIC.
- Current-tree verification: `git ls-tree -r --name-only HEAD docs/issues/open/2993-secrets-and-client-error-text` lists `EPIC.md`, the three `evidence/` files and `manual-verification-evidence.md`. `git merge-base --is-ancestor 98fa27df2 torrust/develop` succeeds. The evidence file names the build on lines 44 and 45, the script edit on lines 21 to 23 and the two limits under "Not Reproduced" (lines 233 to 236). `git show HEAD:cspell.json | grep -n 2993` finds line 28. `grep -c 'manual-verification-evidence.md'` on `git show HEAD:` of the EPIC, the SI-A draft and the SI-B draft prints 1, 13 and 10.
- Resolution reference: `docs(issues): [#2993] record the reproductions of the EPIC inventory as manual verification evidence`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4658604272

### F15 - The UDP probe unpacks a str when the connect reply is missing

- PR number: 2994
- Source review ID: 6018472413
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663340881
- Concern: When `rx()` hits a socket error or timeout it returns a `str`, but the probe slices and unpacks the result as bytes, which raises, and it also raises on a reply shorter than 16 bytes. Copilot asks for `rx()` to always return bytes or raise, for a length check before unpacking the connection id, and for a clear failure message.
- Solution: `rx()` returns `None` on a socket error, the probe exits with a message that names the address and what it received when the connect reply is `None` or shorter than 16 bytes, and later replies print `(no reply)` when none arrives. The evidence file says the probe was edited after the run and run again.
- Current-tree verification: `python3 -I -B docs/issues/open/2993-secrets-and-client-error-text/evidence/udp_probe.py 46999` with no tracker prints `no valid connect response from 127.0.0.1:46999 (got None)` and exits 1. Against a local public tracker (debug build of `develop` at the merge of PR #2995) on 2026-11-23 08:27 UTC it printed the three replies of V6 and exited 0. The note is in the evidence file under "Preserving Verification Artifacts".
- Resolution reference: `docs(issues): [#2993] fix the review round 3 findings in the evidence files and the EPIC`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663415273

### F19 - Same UDP probe comment on the second thread

- PR number: 2994
- Source review ID: 6018472413
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663340931
- Concern: Same text as F15, posted a second time on the same code.
- Solution: No separate change: F15's fix covers it.
- Current-tree verification: Same as F15.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663415596
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663415596

### F16 - The proposed message example hard-codes `got 40`

- PR number: 2994
- Source review ID: 6018472413
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663340995
- Concern: The EPIC table proposes `Invalid authentication key: expected 32 characters, got 40`, which implies a specific wrong length. Copilot asks for a placeholder as in the SI-A draft.
- Solution: The EPIC table row reads `got <n>`.
- Current-tree verification: `grep -c 'got <n>' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` is at least 1 and `grep -c 'got 40' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 0.
- Resolution reference: `docs(issues): [#2993] fix the review round 3 findings in the evidence files and the EPIC`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663415872

### F17 - The usage comment of reproduce.sh names run.sh

- PR number: 2994
- Source review ID: 6018472413
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663341048
- Concern: The first line of `evidence/reproduce.sh` says `Usage: run.sh`, but the file is `reproduce.sh`.
- Solution: The usage comment says `reproduce.sh`.
- Current-tree verification: `head -2 docs/issues/open/2993-secrets-and-client-error-text/evidence/reproduce.sh` shows `Usage: reproduce.sh <tracker-binary> <work-dir>`.
- Resolution reference: `docs(issues): [#2993] fix the review round 3 findings in the evidence files and the EPIC`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663416140

### F18 - The quoted reply ends in `characterse` and reads like a typo

- PR number: 2994
- Source review ID: 6018472413
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663341105
- Concern: The evidence file quotes a failure reason ending in `characterse`. Copilot asks to mark the quote as verbatim and add `[sic]`, as the EPIC does for the `must be have` slip.
- Solution: Not `[sic]`: the text is the tracker's reply, and the final `e` is the bencode terminator, not a typo. The evidence file now says so under that reply, so readers do not take it for an error of the spec.
- Current-tree verification: `grep -n 'bencode terminator' docs/issues/open/2993-secrets-and-client-error-text/manual-verification-evidence.md` finds the note under the reply of V1.
- Resolution reference: `docs(issues): [#2993] fix the review round 3 findings in the evidence files and the EPIC`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663416450

### F20 - Same quoted-reply comment on the second thread

- PR number: 2994
- Source review ID: 6018472413
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663341155
- Concern: Same text as F18, posted a second time on the same text.
- Solution: No separate change: F18's note covers it.
- Current-tree verification: Same as F18.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663416793
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4663416793

### F21 - The evidence file's last-updated-utc is older than its latest edit

- PR number: 2994
- Source review ID: 6020791615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4665103196
- Concern: `last-updated-utc` in the frontmatter of `manual-verification-evidence.md` is older than the edits the document describes (the 2026-11-23 re-run note of `udp_probe.py`). Copilot asks for the timestamp to match the latest modification.
- Solution: The frontmatter timestamp is the time of the fix commit (2026-11-23 11:22 UTC, read from the clock).
- Current-tree verification: `sed -n 4p docs/issues/open/2993-secrets-and-client-error-text/manual-verification-evidence.md` prints `last-updated-utc: "2026-11-23 11:22"`.
- Resolution reference: `docs(issues): [#2993] fix the review round 4 findings in the EPIC and the evidence frontmatter`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4665168699

### F22 - Same last-updated-utc comment on the second thread

- PR number: 2994
- Source review ID: 6020791615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4665103259
- Concern: Same text as F21, posted a second time on the same line.
- Solution: No separate change: F21's fix covers it.
- Current-tree verification: Same as F21.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4665169122
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4665169122

### F23 - The cross-reference list names A10 and A11 but A12 is also reproduced

- PR number: 2994
- Source review ID: 6020791615
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4665103290
- Concern: The EPIC verification bullet says "A10 and A11 below", but A12 is also reproduced, in the database-message section. Copilot asks to add A12 or to point at the sections.
- Solution: The bullet reads "A10 and A11 below, A12 in section V8 of the evidence file".
- Current-tree verification: `grep -c 'A12 in section V8 of the evidence file' docs/issues/open/2993-secrets-and-client-error-text/EPIC.md` prints 1; `grep -n '^### V8' docs/issues/open/2993-secrets-and-client-error-text/manual-verification-evidence.md` finds "Database server messages (inventory A11, A12)".
- Resolution reference: `docs(issues): [#2993] fix the review round 4 findings in the EPIC and the evidence frontmatter`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2994#discussion_r4665169556

## Processing Log

- 2026-11-22 14:16 UTC - Started audit. Round 1: review 6012624473 (Copilot, COMMENTED, submitted 14:05:07Z) with seven inline comments, F1 to F7. Fetched the seven review threads with GraphQL as `josecelano-bot`: all unresolved, `viewerCanResolve` true for each. The fixes of F1 and F4 were already committed (committer times 14:09:00Z and 14:10:03Z) and on the pull request head. Checked each disposition against the current tree with the commands in the detail entries.
- 2026-11-22 14:17 UTC - Replied on the seven threads as `josecelano-bot` (reply `created_at` 14:17:20Z to 14:17:34Z) and recorded the reply URLs.
- 2026-11-22 14:18 UTC - A copy of the audit validator that calls `gh-bot` exits 0 (7 rows, 0 failures). After it, `github-review-threads reply-status` reported `all_replied` (7 of 7) and the seven threads were resolved as `josecelano-bot`. A new GraphQL fetch shows 7 threads, 0 unresolved. This record is not committed yet.
- 2026-11-22 14:55 UTC - Round 2: review 6012831750 (Copilot, COMMENTED, submitted 14:40:22Z) with five inline comments, recorded as F8, F12, F9, F13 and F14. Fetched the twelve review threads with GraphQL as `josecelano-bot`: the seven of round 1 resolved, the five new ones unresolved with `viewerCanResolve` true. The four fix commits were already on the pull request head (committer times 14:47:25Z to 14:49:05Z). Checked each disposition against the head commit with `git show HEAD:<path>`, because the working tree held an unfinished merge of `develop` that this audit did not start.
- 2026-11-22 14:56 UTC - Replied on the five threads of round 2 as `josecelano-bot` (reply `created_at` 14:56:46Z to 14:56:55Z) and recorded the reply URLs.
- 2026-11-22 14:57 UTC - The copy of the audit validator that calls `gh-bot` exits 0 (12 rows, 0 failures). After it, `github-review-threads reply-status` reported `all_replied` (5 of 5) and the five threads were resolved as `josecelano-bot`. A new GraphQL fetch shows 12 threads, 0 unresolved. The round 2 additions to this record are not committed yet: the working tree holds an unfinished merge of `develop` with a conflict in `cspell.json`.
- 2026-11-23 08:26 UTC - Round 3: review 6018472413 (Copilot, COMMENTED, submitted 08:24:06Z) with six inline comments, recorded as F15, F19, F16, F17, F18 and F20. Fetched the 18 review threads with GraphQL as `josecelano-bot`: the twelve of rounds 1 and 2 resolved, the six new ones unresolved. The same head ran its workflow checks after approval: 14 passed, 5 skipped and `Test (Docker) (release)` failed in the qBittorrent MySQL end-to-end test (the tracker container was unhealthy); this pull request changes only documentation, so the cause was not found here and a new run of the head will show whether it repeats.
- 2026-11-23 08:33 UTC - Merged `torrust/develop` (four commits, including the archive move of the #2916 spec), fixed the findings in one signed commit, ran the pre-commit gate (exit 0) and pushed with `git-bot-push`. Replied on the six threads as `josecelano-bot` (reply `created_at` 08:33:33Z to 08:33:45Z) and recorded the reply URLs.
- 2026-11-23 08:35 UTC - The copy of the audit validator that calls `gh-bot` exits 0 (18 rows, 0 failures). The six threads were then resolved as `josecelano-bot`. A new GraphQL fetch shows 18 threads, 0 unresolved.
- 2026-11-23 11:24 UTC - Round 4: review 6020791615 (Copilot, COMMENTED, submitted 11:21:07Z, on the head of round 3) with three inline comments, recorded as F21, F22 and F23. The poller reported it after the specifications of #2999 and #3000 had been pushed. Fetched the 21 review threads with GraphQL as `josecelano-bot`: the eighteen of rounds 1 to 3 resolved, the three new ones unresolved. Fixed the findings in one signed commit, ran the pre-commit gate (exit 0) and pushed with `git-bot-push`.
- 2026-11-23 11:27 UTC - Replied on the three threads as `josecelano-bot` (reply `created_at` 11:27:36Z to 11:27:41Z) and recorded the reply URLs. The reply on F22 first named F22 as the finding to supersede it; it was edited to name F21.
- 2026-11-23 11:28 UTC - The copy of the audit validator that calls `gh-bot` exits 0 (21 rows, 0 failures). The three threads were then resolved as `josecelano-bot`. A new GraphQL fetch shows 21 threads, 0 unresolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
