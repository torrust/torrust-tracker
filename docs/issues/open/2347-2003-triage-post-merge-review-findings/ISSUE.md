---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: 2003
github-issue: 2347
spec-path: docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
branch: "2347-2003-triage-post-merge-review-findings"
related-pr: 2363
last-updated-utc: "2026-09-28 17:37"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
    - fetch-review-threads
  related-artifacts:
    - "issue #2003"
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - docs/pr-reviews/pr-2300-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2313-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2320-review/PR-REVIEW.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/triage.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/manual-verification-evidence.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/implementation-retrospective.md
    - docs/pr-reviews/pr-2290-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2293-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2363-review/PR-REVIEW.md
---

<!-- skill-link: create-issue -->

# Issue #2347 - Triage Post-Merge Review Findings on PRs #2290, #2293, #2300, #2313, and #2320

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

**Owner:** da2ce7, assignee since 2026-09-28 16:17 UTC ([hand-off](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520) and [acknowledgement](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5874065152) on #2003). josecelano owned the task until then and opened PR #2363, its implementation.

## Goal

Give every one of the 32 review findings posted after their pull requests merged a recorded, maintainer-approved disposition in the owning audit, and a finding-specific reply on its thread, so no late review feedback remains silently unprocessed.

## Background

Between 2026-09-22 and 2026-09-24, reviewer da2ce7 posted 32 inline findings on five pull requests that had already merged. Each review body labels them post-merge findings, states that nothing in it asks for remediation, and points to the `process-pr-review` section "Reviews Submitted After Merge". None of the 32 threads has a reply, none is resolved, and no repository artifact or GitHub issue tracks them. They were found on 2026-09-26 while verifying #2333, whose `github-review-threads` tool now lists resolved and unresolved threads together.

| PR | Merged (UTC) | Review | Posted (UTC) | Findings | Audit record at `develop` |
| -- | ------------ | ------ | ------------ | -------- | ------------------------- |
| #2290 | 2026-09-22 10:12 | 5284003304 | 2026-09-22 21:30 | 7 | none |
| #2293 | 2026-09-22 12:33 | 5283816543 | 2026-09-22 21:09 | 8 | none |
| #2300 | 2026-09-22 16:51 | 5284011916 | 2026-09-22 21:31 | 8 | `docs/pr-reviews/pr-2300-review/PR-REVIEW.md` (F1-F4) |
| #2313 | 2026-09-23 10:48 | 5293099957 | 2026-09-23 15:31 | 7 | `docs/pr-reviews/pr-2313-review/PR-REVIEW.md` (F1-F3) |
| #2320 | 2026-09-24 06:50 | 5302919075 | 2026-09-24 10:07 | 2 | `docs/pr-reviews/pr-2320-review/PR-REVIEW.md` (to F27) |

The reviewer numbered the #2290, #2293, #2300, and #2313 findings `loop F<k>`, starting after the IDs the existing audit already uses, so they do not collide. The #2320 findings reuse `F24` and `F25`, which that audit already assigns to different findings, so they need new audit-local IDs, with the reviewer's ID recorded in the detail entry.

### Finding Inventory

<!-- cspell:ignore misattributes -->

| PR | Reviewer ID | Severity | Thread | Summary |
| -- | ----------- | -------- | ------ | ------- |
| #2290 | loop F1 | Minor | [r4076700031](https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700031) | Both evidence files cite two commits that exist in no object in this repository. |
| #2290 | loop F2 | Minor | [r4076700041](https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700041) | The M1 PASS result is false at the head it ships in, and its recorded output does not reproduce. |
| #2290 | loop F3 | Minor | [r4076700048](https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700048) | Copilot's action-3 finding was fixed in the test name only; this prose still claims the opposite. |
| #2290 | loop F4 | Minor | [r4076700056](https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700056) | AC2 is checked, but A159's actual outcome matches none of AC2's three permitted outcomes. |
| #2290 | loop F5 | Minor | [r4076700063](https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700063) | Three Copilot threads were resolved with no reply and no audit record was created. |
| #2290 | loop F6 | Nit | [r4076700071](https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700071) | The module list for the `empty_enums` diagnostics omits `announce.rs`. |
| #2290 | loop F7 | Suggestion | [r4076700080](https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700080) | The merged PR description overstates the removal by one and misattributes ownership. |
| #2293 | loop F1 | Minor | [r4076532667](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532667) | The report-only check fails when the discovery job fails. |
| #2293 | loop F2 | Minor | [r4076532671](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532671) | This names a workflow that does not exist. |
| #2293 | loop F3 | Minor | [r4076532677](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532677) | The `semantic-links.related-artifacts` list is mis-indented. |
| #2293 | loop F4 | Minor | [r4076532684](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532684) | This reproduction instruction cannot be followed from the merged tree. |
| #2293 | loop F5 | Minor | [r4076532690](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532690) | The feature's central path was never verified on a hosted runner. |
| #2293 | loop F6 | Suggestion | [r4076532699](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532699) | No `permissions:` block, against the guidance this PR adds. |
| #2293 | loop F7 | Suggestion | [r4076532707](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532707) | The new skill ships without its semantic skill-link markers. |
| #2293 | loop F8 | Nit | [r4076532713](https://github.com/torrust/torrust-tracker/pull/2293#discussion_r4076532713) | Stale `last-updated-utc` in both issue-folder docs — **not live on `develop`**. |
| #2300 | loop F5 | Minor | [r4076706625](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706625) | Processing Log stamps precede the events they record. |
| #2300 | loop F6 | Minor | [r4076706631](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706631) | AC6 is checked but false at the head it ships in. |
| #2300 | loop F7 | Minor | [r4076706639](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706639) | V2's recorded grep output no longer reproduces, and M2 was not repeated after the review round that invalidated it. |
| #2300 | loop F8 | Minor | [r4076706646](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706646) | This ownership mark states something no audit record in the repository satisfies — including the one this PR ships. |
| #2300 | loop F9 | Minor | [r4076706651](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706651) | The Inputs line cites three branch SHAs that no longer resolve at this head. |
| #2300 | loop F12 | Minor | [r4076706663](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706663) | All four rows cite one commit subject that describes only F1's fix. |
| #2300 | loop F10 | Suggestion | [r4076706673](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706673) | The new pins cover 3 of the 19 roster names, so the skill's "must match … exactly" rule stays largely unenforced. |
| #2300 | loop F11 | Suggestion | [r4076706680](https://github.com/torrust/torrust-tracker/pull/2300#discussion_r4076706680) | This commit also rewrites a historical progress entry, beyond what the issue scoped for the EPIC. |
| #2313 | loop F4 | Minor | [r4084224040](https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224040) | A duplicate thread whose concern was fixed has two mandated dispositions at head; this row picks one while the log calls it a duplicate. |
| #2313 | loop F5 | Minor | [r4084224052](https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224052) | The template states the `Resolution reference` rule three ways at head, two of them contradicting. |
| #2313 | loop F6 | Minor | [r4084224062](https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224062) | F76 is named as reconciled, but at head the case F76 describes has no admissible `Resolution reference` at all. |
| #2313 | loop F7 | Minor | [r4084224083](https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224083) | A policy sentence was appended to the copied-verbatim `## Status Values` section, and this PR's own record does not copy it. |
| #2313 | loop F8 | Minor | [r4084224093](https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224093) | V1's recorded command cannot produce the Observed Result V1 asserts, and the fix changed the assertion rather than the procedure. |
| #2313 | loop F9 | Suggestion | [r4084224101](https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224101) | F79's conditional was resolved toward the condition, so the single-round consolidated response F79 named remains unruled. |
| #2313 | loop F10 | Nit | [r4084224113](https://github.com/torrust/torrust-tracker/pull/2313#discussion_r4084224113) | This file ships at head claiming a last-updated time 54 minutes before its last edit. |
| #2320 | F24 | Nit | [r4092362605](https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4092362605) | Record F24's verification under-reports its own command's output |
| #2320 | F25 | Nit | [r4092362620](https://github.com/torrust/torrust-tracker/pull/2320#discussion_r4092362620) | The head-id note is true for two of the seven head ids |

Each summary is the first line of the thread's first comment, generated from the capture; the thread is the source of truth.

## Scope

### In Scope

- Before any repository change, post a short reply on each of the 32 threads naming this issue as its tracking record, after the maintainer approval is recorded on this issue. Threads stay unresolved.
- For each finding, verify against the current `develop` tree whether it is still live, and record the verification command or inspection.
- Normalize every finding into the owning PR's audit record, as `process-pr-review` requires: create `docs/pr-reviews/pr-2290-review/PR-REVIEW.md` and `pr-2293-review/PR-REVIEW.md`, and append rows to the #2300, #2313, and #2320 audits with collision-safe IDs, keeping the reviewer's ID in each detail entry.
- Propose one disposition per finding for maintainer approval: fix in this task (small, documentation-only corrections), `NO_ACTION` (not live, or declined by the maintainer), or `FOLLOW_UP` in a separate issue for anything larger or outside documentation.
- Apply only the approved fixes, in commits grouped per owning PR.
- After this task's pull request merges, close the loop following the PR #2276 precedent: a small pull request records the merge reference and final disposition for each finding this task fixed or declined; each of those threads then gets a reply with that disposition and a durable reference, and is resolved. A `FOLLOW_UP` finding keeps its thread open, with a reply naming its owning issue, until that issue's fix merges; that issue then updates the audit and resolves the thread.

### Out of Scope

- Findings the #2003 register or another open issue already owns; those threads are answered with a pointer to the owner.
- Workflow, CI, or Rust behavior changes (for example #2293 loop F1, F5, and F6). If approved, each becomes its own issue.
- Rewriting historical audit or progress-log entries; corrections are appended.
- Changing the reviewer-side `review-pr` process.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- ADRs to create: `None known`.

## Approval Records

Each approval below was given in chat and then posted by josecelano as a comment on this issue. The audits cite these URLs as their durable approval references. When the comments are minimized, each URL still opens its comment and the text can be expanded. Minimizing a record means this section now carries its content; the record itself stays in force.

| Record | Posted (UTC) | What it approved or changed |
| ------ | ------------ | --------------------------- |
| [Post-merge workflow approval](https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5846100762) | 2026-09-26 12:03 | It approved tracking the 32 findings in this issue under `process-pr-review` "Reviews Submitted After Merge": a short reply on each thread naming this issue, every thread left unresolved, and the triage and audit work in this specification. It approved no per-finding disposition; those are proposed in T2 and decided in T3, before any fix, audit disposition, or thread resolution. |
| [T3 disposition approval](https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588) | 2026-09-28 07:45 (approved in chat at 07:44) | The T2 triage, with 14 `FIXED`, 9 `NO_ACTION` and 9 `FOLLOW_UP`, listed by audit ID. Changes from the T2 proposal: #2300 F7, #2300 F11 and #2320 F28 become `NO_ACTION` because no claim in the current tree is false, and #2293 F6 becomes `NO_ACTION`. Also approved: no audit rows for the 11 Copilot threads resolved before merge, a deliberate exception to `process-pr-review` 1.4; each fix in its own commit, and each PR's audit in one commit; `FOLLOW_UP` threads left open until their owning issue's fix merges; and each new follow-up issue drafted as a spec and approved before creation. |
| [Amendment 1](https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5867552349) | 2026-09-28 09:51 | #2293 F5 goes to the existing #2301 instead of a new issue: #2301's scope already covers the hosted `package-coverage-regression` evidence and the deferred scenarios of #2222, and T2 missed that owner. #2293 F1 goes to a new bug issue under #1347, limited to the report-only summary failing with a misleading JSON parse error when discovery fails or produces no output; the failure reproduces locally. #2313 F4, F5, F6, F7 and F9 still go to a new #2278 subissue, and F7 coordinates with #2278 order 8. |
| [Amendment 2](https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5868016556) | 2026-09-28 10:20 | #2293 F3 stays `FIXED`, but the fix is PR #2357's `docs(issues): [#2281] fix malformed frontmatter indentation in closed records`, which reached `develop` before this branch was rebased; the #2347 fix commit became empty and was dropped. The row is `FIXED`/`RESOLVED` citing that subject, and the thread was resolved at the reply step. Thirteen approved fixes land in this task and one in PR #2357. |
| [Process deviation accepted](https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5869786394) | 2026-09-28 12:25 | The T3 approval URL reached the #2300, #2313 and #2320 audits only in the T4 audit commits (10:44-10:54 UTC). That was after this task's earlier edits to the #2300 and #2320 audits (09:00 and 09:22 UTC) and after the disposition replies (10:28-10:30 UTC). The approval, which existed from 07:45 UTC, covered every one of those actions; only its citation came late. The deviation is accepted without rewriting the audits. The proposed `process-pr-review` clarification stays in the retrospective, and EPIC #2003 lists it as undecided improvement candidate 5. |

## Disposition Decisions (T3)

Approval record: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>, amended by <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5867552349> (#2293 F5 goes to the existing #2301, which T2 missed; #2293 F1's new issue is a bug) and by <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5868016556> (PR #2357 fixed #2293 F3 upstream). The maintainer approved the [`triage.md`](triage.md) proposal on 2026-09-28 with the changes below. The approval record and its amendments are the authority for each finding's disposition.

| Disposition | Findings |
| ----------- | -------- |
| `FIXED` (14) | #2290 F1, F2, F3, F5, F6; #2293 F2, F4, F7; #2300 F5, F6, F9; #2313 F8; #2320 F29; and #2293 F3, fixed upstream by PR #2357 |
| `NO_ACTION` (9) | #2290 F7; #2293 F6, F8; #2300 F7, F8, F11, F12; #2313 F10; #2320 F28 |
| `FOLLOW_UP` (9) | #2290 F4 (#2360, parent #2003); #2293 F1 (#2361, bug, parent #1347); #2293 F5 (existing #2301); #2300 F10 (#2278 order 8); #2313 F4, F5, F6, F7, F9 (#2362, #2278 order 11) |

Why we proceed this way:

- **Fix only what misleads a reader today.** A finding is fixed when a claim in the current tree is false or cannot be followed: an unreachable SHA, a false PASS or checked criterion, a wrong workflow name, broken YAML, or a test described wrongly. When a record was only imprecise and nothing false stands, the new audit row is the correction. This avoids edits to closed issue folders that add nothing. For this reason, #2300 F7 (a line number that moved while the check still passes), #2300 F11 (a dropped clause that was already stale), and #2320 F28 (an under-report with no false assertion) became `NO_ACTION`.
- **#2293 F6 is declined rather than tracked.** Pull requests to `develop` come from forks, and fork pull requests get a read-only token, so the missing `permissions:` block exposes nothing in practice. The gap also predates PR #2293, and 11 of 15 workflows share it. A new issue would add work in progress without adding protection. Repository-wide least-privilege hardening, if wanted, is a separate decision.
- **Larger or non-documentation work is tracked elsewhere.** Rust, workflow, and skill-contract changes stay out of this documentation task. They go to the issue that owns them. When no issue owns them yet, each new issue is drafted as a spec and created only after maintainer approval.
- **No backfilled rows for Copilot threads resolved before merge.** PR #2290 has 3 such threads and PR #2293 has 8, each resolved without a reply. Each row would need a new reply on an already-resolved thread, and that reply would carry nothing actionable. The reviewer confirmed that the three #2290 fixes landed, and the only partial one is loop F3. The #2293 threads whose fixes did not land are exactly loop F2-F4. So each new audit's Processing Log records those reviews as context: the review ID, its threads, which fixes landed, and which finding covers the rest. This is a deliberate exception to `process-pr-review` 1.4, which the approval record names. #2290 F5 is `FIXED` by creating the audit, and its detail entry declines the reply backfill.
- **One fix per commit.** Each `FIXED` finding gets its own commit, so its `Resolution reference` names exactly the change that fixed it. This follows `process-pr-review` step 6 and avoids the defect #2300 F12 describes. Audits stay one commit per PR.
- **Order of work** (maintainer-approved 2026-09-28, following the PR #2339 / PR #2344 precedent):
  1. Commit the fixes.
  2. Draft, approve, and create the follow-up issues. Their specs ride in this PR as separate commits.
  3. Open this PR as a draft.
  4. Post one disposition reply on each of the 32 threads.
  5. Commit one audit per PR, citing those replies.
  6. Complete T6, then mark the PR ready.

  The audits come after the PR because each row cites a reply on its own thread, and a reply that states the disposition needs the PR and the owning issue to exist. The T1 replies only say that the disposition is pending, so they cannot serve as resolution references.
- **Approved fixes stay `FOLLOW_UP` until merge.** `process-pr-review` "Reviews Submitted After Merge" forbids claiming `FIXED` on `develop` before the follow-up merges. In this PR, the 13 approved fixes that land here are therefore recorded as `FOLLOW_UP`/`OPEN`, with this PR as `Follow-up PR URL`. The T7 close-out changes them to `FIXED`, citing their commit subjects, and replaces each reply URL with the final one. #2293 F3 is the exception: PR #2357 fixed it upstream, so it is already `FIXED`/`RESOLVED`, and its thread was resolved after the disposition reply. NO_ACTION rows are final in this PR; their threads are resolved in T7. A thread fixed here therefore ends with three replies: the T1 tracking reply, the disposition reply, and the T7 reply. NO_ACTION and #2293 F3 threads need no T7 reply.

## Design and Ownership Review

Not applicable. This task processes review findings and edits documentation; it adds no process, I/O, or fixture code.

## Bug-Fix Process

Not applicable. The findings are review-process and documentation defects, handled under `process-pr-review` rather than as product bugs; any finding that turns out to be a product bug becomes its own bug issue.

## Regression Test Strategy

Not applicable, for the same reason.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Record approval and acknowledge threads | Maintainer approval posted on this issue; one tracking reply on each of the 32 threads. |
| T2 | DONE | Triage against `develop` | Per finding: live or not, with the command or inspection used; proposed disposition. Recorded in [`triage.md`](triage.md). |
| T3 | DONE | Maintainer disposition review | Maintainer approves or changes each proposed disposition; the decision is recorded in the progress log with a durable comment URL. See [Disposition Decisions (T3)](#disposition-decisions-t3). |
| T4 | DONE | Normalize into audits | Five audits hold all 32 findings with approved dispositions; `validate-audit-record.py` exits `0` for each PR. |
| T5 | DONE | Apply approved fixes | Approved documentation fixes applied; follow-up issues created for approved `FOLLOW_UP` items. |
| T6 | DONE | Verify and record completion | Automatic checks, evidence, and acceptance review recorded; implementation PR opened. Recorded so far: M1 and M3, the acceptance review, the retrospective, and the independent task review. Round 7 of the PR #2363 review (review 5341908562) approves the PR's head. PR #2363 merged into `develop` at 17:00:27Z. |
| T7 | DONE | Close the loop after merge | Close-out PR records final dispositions for findings fixed or declined here; those threads are replied to and resolved; each `FOLLOW_UP` thread stays open with a reply naming its owning issue. The full list is in [T7 Close-Out](#t7-close-out). The final replies and resolutions were posted by josecelano at 17:07 UTC, and the close-out PR records them. |

### T7 Close-Out

T7 runs in one close-out PR. PR #2363 merged on 2026-09-28 at 17:00 UTC as `bc90cde1b`; the list follows the #2003 hand-off:

- [x] Change the 13 audit rows fixed in PR #2363 from `FOLLOW_UP`/`OPEN` to `FIXED`, each citing its fix's commit subject, and replace each reply URL with the final one.
- [x] Post the final replies on those 13 threads, and resolve them together with the 9 `NO_ACTION` threads. The `NO_ACTION` threads need no new reply. josecelano posted the 13 replies (17:07:36Z-17:07:57Z) and resolved the 22 threads.
- [x] Keep the 9 `FOLLOW_UP` threads open until their owners' fixes merge:
  - #2360 for #2290 F4;
  - #2361 for #2293 F1;
  - #2301 for #2293 F5;
  - #2278 order 8 for #2300 F10;
  - #2362 for #2313 F4, F5, F6, F7 and F9.
- [x] Record PR #2363 review rounds 6 and 7 in `docs/pr-reviews/pr-2363-review/PR-REVIEW.md`, where rounds 4 and 5 are already recorded on `develop`:
  - Round 6 is review 5341713976, an approval (ACK comment 5874296059) of the head `docs(pr-reviews): [#2347] record Copilot's severity labels in the PR #2363 audit`. It raised F15 [Suggestion]: rounds 6-7 are not yet recorded in the audit.
  - GitHub dismissed round 6 automatically at 16:43:16Z, when `docs(pr-reviews): [#2347] record PR #2363 review rounds 4 and 5` was pushed.
  - Round 7 is review 5341908562, an approval (ACK comment 5874566969) of that head. F15 stands as non-blocking.

  Recording the rounds is F15's fix. Add its audit row, reply on its thread, and resolve it. josecelano replied on the F15 thread at 16:53:48Z and resolved it, and the close-out adds the row.
- [x] Run M2 and close AC4. M2 is the GraphQL `reviewThreads` query at 17:19 UTC; see `manual-verification-evidence.md` V2.
- [ ] Close #2347, then archive this specification in a separate step, since `cleanup-completed-issues` archives only the spec of an issue that is closed.

The round-5 correction that the #2003 acknowledgement carried into the close-out is F14, the Copilot severity labels. It is already fixed at PR #2363's head by `docs(pr-reviews): [#2347] record Copilot's severity labels in the PR #2363 audit`.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T4 | One owning PR's audit record | One `docs(pr-reviews)` commit per PR audit. |
| T5 | One approved documentation fix | One commit per `FIXED` finding, with the narrow affected scope (for example `docs(issues)`); see [Disposition Decisions (T3)](#disposition-decisions-t3). |
| T6 | Evidence and tracking | One `docs(issues)` commit. |
| T7 | Audit close-out after merge | One `docs(pr-reviews)` commit in the close-out PR. |

T1-T3 change no repository file; their evidence is the GitHub replies and comments, recorded in the progress log. No test code is planned.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-triage-post-merge-review-findings/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2003, and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all` and pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-26 11:02 UTC - GitHub Copilot - Drafted after read-only triage of the five merged PRs. The finding inventory was generated from `github-review-threads show --unresolved-only` captures, and review and merge times from the GitHub API. The maintainer approved creating this tracking task in chat; its approval as a GitHub comment follows issue creation (T1).
- 2026-09-26 12:20 UTC - GitHub Copilot - Maintainer approved the specification. Created GitHub issue #2347, linked it as a sub-issue of #2003 (`parent_issue_url` verified), and moved this specification to `docs/issues/open/`. T1 done ahead of the spec-only PR, as the maintainer approved: the approval record is <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5846100762>, and each of the 32 threads received one reply naming #2347 with its disposition pending. A GraphQL refetch shows all 32 threads still unresolved, each with that reply (7, 8, 8, 7, and 2 per PR). Spec-only PR pending.
- 2026-09-27 18:10 UTC - GitHub Copilot - The spec-only PR #2348 merged on 2026-09-27 06:11 UTC (`gh pr view 2348`). Created the implementation branch `2347-2003-triage-post-merge-review-findings` from `develop` at `478516cf`. The maintainer decided in chat: T2 results go in an issue-folder `triage.md` linked from this spec; the T3 approval follows the T1 precedent (approval in chat, then an approval record posted on this issue); each approved `FOLLOW_UP` gets a folder-style draft spec, a GitHub issue, and a sub-issue link, each approved first; the branch is pushed only at T6.
- 2026-09-27 19:05 UTC - GitHub Copilot - T2 done. Re-fetched all five PRs' threads (32 unresolved, each still with only the finding and the T1 reply) and checked each finding against `develop` at `478516cf`; results, verification commands, and proposed dispositions are in `triage.md`: 17 `FIXED`, 5 `NO_ACTION`, 10 `FOLLOW_UP` across four proposed follow-up issues and EPIC #2278 order 8. No disposition is approved yet; T3 is next.
- 2026-09-28 07:50 UTC - GitHub Copilot - T3 done. The maintainer approved the triage in chat with four changes (#2300 F7, #2300 F11, #2320 F28, and #2293 F6 become `NO_ACTION`), the no-backfill exception for the pre-merge Copilot threads, and one commit per fix. The approval record was posted at 2026-09-28 07:44 UTC: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>. Recorded the rationale in "Disposition Decisions (T3)" and changed the T5 commit policy from one commit per PR to one per fix. The result is 14 `FIXED`, 9 `NO_ACTION`, and 9 `FOLLOW_UP`. T5 fixes start next.
- 2026-09-28 09:35 UTC - GitHub Copilot - T5 documentation fixes committed: 13 fix commits (#2290 F5 is fixed by its audit in T4) and one commit refreshing `last-updated-utc` stamps that three earlier fix commits left stale. The maintainer approved opening this PR as a draft before the audits, following the PR #2339 precedent, and drafting the three follow-up issues first; see "Order of work" above. Next: the FU-A draft spec.
- 2026-09-28 09:42 UTC - GitHub Copilot - FU-A approved and created as #2360 under #2003 (order 13 in the #2003 EPIC); its spec rides in this PR.
- 2026-09-28 09:51 UTC - GitHub Copilot - While drafting FU-B, found that open #2301 ("Review package coverage rollout", #1347) already owns #2293 F5. The T2 triage had missed it. The maintainer approved routing F5 to #2301, and narrowing FU-B to #2293 F1 as a bug issue under #1347. Amendment record: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5867552349>. The #2278 register has no existing owner for #2313 F4-F7 or F9, so FU-D stands.
- 2026-09-28 10:14 UTC - GitHub Copilot - T5 done. The maintainer approved each follow-up spec before its issue was created: #2361 (bug, #1347 order 9) for #2293 F1, and #2362 (#2278 order 11) for #2313 F4, F5, F6, F7, and F9. Both specs ride in this PR. Added pointers in the owners that already existed: #2300 F10 in the #2278 order 8 row, and #2293 F5 in the #2301 progress log, with the hosted run evidence. Next: push and open this PR as a draft.
- 2026-09-28 10:20 UTC - GitHub Copilot - Rebased onto `develop` at `8a953724`, which had moved 84 commits. PR #2357 (#2281) had already fixed #2293 F3 in `docs(issues): [#2281] fix malformed frontmatter indentation in closed records`, so the rebase dropped this branch's now-empty F3 commit, and a follow-up commit restored the #2222 plan's stamp. The maintainer approved keeping F3 `FIXED` with that upstream subject, and resolving its thread at the reply step: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5868016556>. Resolved stamp conflicts in the #1347 EPIC and #2301 spec using `develop`'s quoted format. The new frontmatter validator rejected this spec's `status: in_progress`; it is now `in-progress`.
- 2026-09-28 10:24 UTC - GitHub Copilot - Pushed the branch (the pre-push hook passed) and opened draft PR #2363.
- 2026-09-28 10:29 UTC - GitHub Copilot - The maintainer reviewed the 32 drafted disposition replies, and they were posted from 10:28:56Z to 10:29:54Z. The #2293 F3 thread was resolved at 10:32 UTC.
- 2026-09-28 10:57 UTC - GitHub Copilot - T4 done: five audit commits, one per merged PR. `validate-audit-record.py` exits `0` for each: #2290 (7 rows), #2293 (8), #2300 (12), #2313 (10), and #2320 (29), with the first parent of each PR's merge commit as `--base`.
- 2026-09-28 11:00 UTC - GitHub Copilot - T6: M1 and M3 are recorded in `manual-verification-evidence.md`. M2 waits for T7. The acceptance criteria were re-reviewed: AC1, AC2, AC3, and AC5 are done, and AC4 waits for T7. `implementation-retrospective.md` records why 32 post-merge findings went unprocessed, and four improvement candidates for the maintainer to decide on. An independent task review comes next, before the PR is marked ready.
- 2026-09-28 11:48 UTC - GitHub Copilot - The independent task review (`agent-review-reports.md`) failed on documentation defects only. Corrections to earlier entries, which are kept:
  - The 2026-09-27 19:05 entry and `triage.md`'s original stamp were estimates. `docs(issues): [#2347] record T2 triage of post-merge findings` was authored at 20:06:33 UTC.
  - The 07:50 entry is later than the commit that carries it: `docs(issues): [#2347] record T3 disposition decisions` was authored at 07:46:57 UTC. Its approval record was posted at 07:45:06Z; 07:44 is the chat approval time.
  - The 10:29 entry also records the #2293 F3 resolution, which happened at 10:32, after its stamp.

  The approval URL reached the #2300, #2313, and #2320 audits only at T4, after their first #2347 edits and replies. The retrospective records this deviation for the maintainer.
- 2026-09-28 12:17 UTC - GitHub Copilot - Processed da2ce7's round-1 review of PR #2363 (review 5337742420, four findings) in `docs/pr-reviews/pr-2363-review/PR-REVIEW.md`. F2, F3, and F4 were already fixed by the pre-PR-review correction commit and the #2290 audit commit. F1 was live: the #2320 F29 correction had shifted the retrospective's later lines. It is fixed in `docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines`. All four threads were replied to and resolved.
- 2026-09-28 12:25 UTC - GitHub Copilot - The maintainer accepted the approval-URL timing deviation in the #2300, #2313, and #2320 audits, without rewriting them: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5869786394>. The proposed `process-pr-review` clarification stays in `implementation-retrospective.md` for a future skill change.
- 2026-09-28 14:58 UTC - GitHub Copilot - Processed round 2 of the PR #2363 review (da2ce7 review 5338643536, F5-F9, reconfirmed by round-3 review 5339061836) and Copilot review 5338724829 (four comments, audit F10-F13) in `docs/pr-reviews/pr-2363-review/PR-REVIEW.md`. F5-F8 are fixed in one signed commit each. F9 is fixed by the PR description edit. F10-F13 have no change: the three unquoted stamps are string-typed and follow their folders' convention, and the F28 sentence is grammatical. All nine threads were replied to and resolved.
- 2026-09-28 16:29 UTC - GitHub Copilot - Round 4 of the PR #2363 review approved `docs(issues): [#2347] map the pre-PR task review finding IDs` (da2ce7 ACK); GitHub dismissed it when the round-2 audit was pushed. Round 5 raised F14: the audit had recorded the Copilot severity labels wrongly. F14 is fixed in `docs(pr-reviews): [#2347] record Copilot's severity labels in the PR #2363 audit`, and its thread was replied to and resolved. The maintainer hands EPIC #2003 to da2ce7: <https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520>.
- 2026-09-28 16:59 UTC - da2ce7 - Assignee of this issue since 2026-09-28 16:17 UTC, taking over from josecelano. The hand-off and acknowledgement are on #2003: <https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520>, <https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5874065152>. PR #2363 review rounds after the 16:29 entry: round 6 (review 5341713976, an approval with ACK comment 5874296059) raised F15 [Suggestion], that rounds 6-7 are not yet recorded in the audit. GitHub dismissed round 6 at 16:43:16Z, when `docs(pr-reviews): [#2347] record PR #2363 review rounds 4 and 5` was pushed. Round 7 (review 5341908562, an approval with ACK comment 5874566969) approved that head, with F15 standing as non-blocking. T7 records both rounds and F15.
- 2026-09-28 17:00 UTC - da2ce7 - PR #2363 merged into `develop` as `bc90cde1b` (merged by josecelano at 17:00:27 UTC after seven review rounds; the last approval, review 5341908562, and its ACK stand on the merged head). T1-T6 are on `develop`; T7 starts.
- 2026-09-28 17:05 UTC - da2ce7 - Revised this specification for the T7 close-out PR. Added Approval Records (the five approval comments, with what each approved or changed), the owner line, and T7 Close-Out (the #2003 hand-off's list plus the review-round tail). Reflowed paragraphs to one line each. The issue body is re-derived from this file.
- 2026-09-28 17:15 UTC - da2ce7 - T7 phase 1 prepared the close-out without posting anything. It edits the five audits: the 13 findings fixed by PR #2363 become `FIXED`/`RESOLVED`, each citing its fix commit, and the 9 `NO_ACTION` threads become `RESOLVED`. It adds F15 and review rounds 6-7 to the PR #2363 audit. It drafts 12 replies (#2290 F1 already has its T7 reply, posted at 17:07:36Z), a resolve list of 22 threads plus F15, and the M2 block. Every `FOLLOW_UP` disposition reply already names its owner, so those 9 threads get no new reply and stay open. The rest of T7 follows the posting: reply URLs, resolution, M2, AC4.
- 2026-09-28 17:37 UTC - da2ce7 - T7 done in the close-out PR. josecelano posted the 13 T7 replies (17:07:36Z-17:07:57Z), resolved those threads and the 9 `NO_ACTION` threads, and had already replied on and resolved the PR #2363 F15 thread (16:53:48Z). The five audits now cite those replies as Reply URLs. The PR #2363 audit records F15, rounds 6-7 and the #2003 hand-off as its post-merge approval reference. M2 passes: the GraphQL `reviewThreads` query at 17:19 UTC shows only the 9 `FOLLOW_UP` threads unresolved, and AC4 is done. Closing #2347 and archiving this specification follow as a separate step.

## Acceptance Criteria

- [x] AC1: Each of the 32 findings has a row in its owning PR's audit record, with a collision-safe ID, the reviewer's ID, a live-on-`develop` verification, and a maintainer-approved disposition.
- [x] AC2: The maintainer approval of the dispositions is recorded as a durable GitHub comment URL before any fix or thread resolution. The URL appears in the Ownership section of the #2300, #2313, and #2320 audits when their rows are added, and of the #2290 and #2293 audits when those audits are created.
- [x] AC3: Every approved fix is applied, and every approved `FOLLOW_UP` item links a created issue.
- [x] AC4: Each of the 32 threads has a finding-specific final reply. Every thread whose finding this task fixed or declined is resolved; every `FOLLOW_UP` thread stays open, its reply naming the owning issue, until that issue's fix merges. A final GraphQL fetch confirms both.
- [x] AC5: `validate-audit-record.py` exits `0` for PRs #2290, #2293, #2300, #2313, and #2320.
- [x] `linter all` exits with code `0`.
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

## Verification Plan

### Automatic Checks

- `linter all`
- `python3 .github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py --pr-number <PR>` for each of the five PRs
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=text`
- Pre-push checks before opening the implementation pull request

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Tracking replies visible | Open each of the five PRs on GitHub after T1. | Every one of the 32 threads shows a reply naming this issue. | DONE | `manual-verification-evidence.md` section V1 (GraphQL fetch of all five PRs, 2026-09-28 10:56 UTC) |
| M2 | Thread states match dispositions | For each PR, run `github-review-threads fetch` and `list --unresolved-only` after T7. | Only `FOLLOW_UP` threads are unresolved, each with a reply naming its owning issue; every other of the 32 threads is resolved. | DONE | `manual-verification-evidence.md` section V2 (GraphQL `reviewThreads` query, 2026-09-28 17:19 UTC) |
| M3 | Live-status spot check | Re-run the recorded verification for three findings chosen at random, one per disposition kind. | Each re-run reproduces the recorded result. | DONE | `manual-verification-evidence.md` section V3 (#2290 F6, #2300 F8, #2290 F4) |

No disposable verification script is planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Audit records for the five PRs: 32 new rows, each with `Reviewer finding ID`, `Current-tree verification`, and an approved disposition |
| AC2 | DONE | Approved in chat at 07:44 UTC; the approval record was posted at 07:45:06Z, before the first fix commit (authored 08:30:39 UTC). The URL is in all five Ownership sections, but the three existing audits received it only at T4; see the retrospective. |
| AC3 | DONE | 12 fix commits in this PR plus the #2290 audit (F5); #2293 F3 fixed upstream by PR #2357; follow-up owners #2360, #2361, #2362, #2301, and EPIC #2278 order 8 |
| AC4 | DONE | M2 in `manual-verification-evidence.md` V2: at 17:19 UTC only the 9 `FOLLOW_UP` threads are unresolved, each with a reply naming its owner. The T7 replies and resolutions were posted by josecelano at 17:07 UTC. |
| AC5 | DONE | Validator output per PR in `manual-verification-evidence.md` Automatic Checks |

## Risks and Trade-offs

- Thirty-two findings are a lot for one pull request. Mitigation: one commit per fix, one audit commit per owning PR, and anything beyond documentation becomes its own issue.
- Some findings concern rules that EPIC #2278 is still reshaping (for example #2313 loop F4-F7 on the audit contract). Mitigation: T2 checks each one against the current `develop`, and one that an open #2278 subissue owns is pointed to that subissue rather than fixed twice.
- Replying before triage can look like a promise to fix. Mitigation: the T1 reply only names the tracking record and states that the disposition is pending.

## Implementation Completion Review

- Retrospective: created. [`implementation-retrospective.md`](implementation-retrospective.md) records why 32 post-merge findings went unprocessed (nothing surfaces threads opened after a merge), three recurring finding classes, and five improvement candidates for the maintainer to decide on. EPIC #2003's specification lists them as undecided improvement candidates, each with the home the #2003 hand-off suggests.
- When an independent reviewer receives this folder-style specification, record the result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.
- T7 close-out: the retrospective needs no update. The maintainer's replies and resolutions matched the recomputed lists exactly: 13 fixed threads replied to and resolved, 9 `NO_ACTION` threads resolved, and 9 `FOLLOW_UP` threads left open.

## References

- Parent EPIC: #2003.
- Governing workflow: `process-pr-review` "Reviews Submitted After Merge"; PR #2344 extends it to findings unprocessed at merge.
- Post-merge precedent: PR #2269 findings, follow-up PR #2271, close-out PR #2276.
- Related: #2333 (the thread tool that surfaced these), PR #2339.
- Ownership record: [hand-off](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5873655520) and [acknowledgement](https://github.com/torrust/torrust-tracker/issues/2003#issuecomment-5874065152) on #2003.
- Approval records: see [Approval Records](#approval-records).
