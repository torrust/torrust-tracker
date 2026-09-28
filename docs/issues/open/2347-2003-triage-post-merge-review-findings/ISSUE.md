---
schema-version: 1
doc-type: issue
issue-type: task
status: in_progress
priority: p2
epic: 2003
github-issue: 2347
spec-path: docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
branch: "2347-2003-triage-post-merge-review-findings"
related-pr: null
last-updated-utc: "2026-09-28 09:35"
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
---

<!-- skill-link: create-issue -->

# Issue #2347 - Triage Post-Merge Review Findings on PRs #2290, #2293, #2300, #2313, and #2320

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Give every one of the 32 review findings posted after their pull requests merged a recorded,
maintainer-approved disposition in the owning audit, and a finding-specific reply on its thread,
so no late review feedback remains silently unprocessed.

## Background

Between 2026-09-22 and 2026-09-24, reviewer da2ce7 posted 32 inline findings on five pull
requests that had already merged. Each review body labels them post-merge findings, states that
nothing in it asks for remediation, and points to the `process-pr-review` section "Reviews
Submitted After Merge". None of the 32 threads has a reply, none is resolved, and no repository
artifact or GitHub issue tracks them. They were found on 2026-09-26 while verifying #2333, whose
`github-review-threads` tool now lists resolved and unresolved threads together.

| PR | Merged (UTC) | Review | Posted (UTC) | Findings | Audit record at `develop` |
| -- | ------------ | ------ | ------------ | -------- | ------------------------- |
| #2290 | 2026-09-22 10:12 | 5284003304 | 2026-09-22 21:30 | 7 | none |
| #2293 | 2026-09-22 12:33 | 5283816543 | 2026-09-22 21:09 | 8 | none |
| #2300 | 2026-09-22 16:51 | 5284011916 | 2026-09-22 21:31 | 8 | `docs/pr-reviews/pr-2300-review/PR-REVIEW.md` (F1-F4) |
| #2313 | 2026-09-23 10:48 | 5293099957 | 2026-09-23 15:31 | 7 | `docs/pr-reviews/pr-2313-review/PR-REVIEW.md` (F1-F3) |
| #2320 | 2026-09-24 06:50 | 5302919075 | 2026-09-24 10:07 | 2 | `docs/pr-reviews/pr-2320-review/PR-REVIEW.md` (to F27) |

The reviewer numbered the #2290, #2293, #2300, and #2313 findings `loop F<k>`, starting after the
IDs the existing audit already uses, so they do not collide. The #2320 findings reuse `F24` and
`F25`, which that audit already assigns to different findings, so they need new audit-local IDs,
with the reviewer's ID recorded in the detail entry.

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

- Before any repository change, post a short reply on each of the 32 threads naming this issue as
  its tracking record, after the maintainer approval is recorded on this issue. Threads stay
  unresolved.
- For each finding, verify against the current `develop` tree whether it is still live, and record
  the verification command or inspection.
- Normalize every finding into the owning PR's audit record, as `process-pr-review` requires:
  create `docs/pr-reviews/pr-2290-review/PR-REVIEW.md` and `pr-2293-review/PR-REVIEW.md`, and
  append rows to the #2300, #2313, and #2320 audits with collision-safe IDs, keeping the
  reviewer's ID in each detail entry.
- Propose one disposition per finding for maintainer approval: fix in this task (small,
  documentation-only corrections), `NO_ACTION` (not live, or declined by the maintainer), or
  `FOLLOW_UP` in a separate issue for anything larger or outside documentation.
- Apply only the approved fixes, in signed commits grouped per owning PR.
- After this task's pull request merges, close the loop following the PR #2276 precedent: a small
  pull request records the merge reference and final disposition for each finding this task fixed
  or declined; each of those threads then gets a reply with that disposition and a durable
  reference, and is resolved. A `FOLLOW_UP` finding keeps its thread open, with a reply naming its
  owning issue, until that issue's fix merges; that issue then updates the audit and resolves the
  thread.

### Out of Scope

- Findings the #2003 register or another open issue already owns; those threads are answered with
  a pointer to the owner.
- Workflow, CI, or Rust behavior changes (for example #2293 loop F1, F5, and F6). If approved, each
  becomes its own issue.
- Rewriting historical audit or progress-log entries; corrections are appended.
- Changing the reviewer-side `review-pr` process.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`.
- ADRs to create: `None known`.

## Disposition Decisions (T3)

Approval record: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>.
The maintainer approved the [`triage.md`](triage.md) proposal on 2026-09-28 with the changes
below. The approval record is the authority for each finding's disposition.

| Disposition | Findings |
| ----------- | -------- |
| `FIXED` (14) | #2290 F1, F2, F3, F5, F6; #2293 F2, F3, F4, F7; #2300 F5, F6, F9; #2313 F8; #2320 F29 |
| `NO_ACTION` (9) | #2290 F7; #2293 F6, F8; #2300 F7, F8, F11, F12; #2313 F10; #2320 F28 |
| `FOLLOW_UP` (9) | #2290 F4 (new issue, parent #2003); #2293 F1, F5 (new issue, parent #1347); #2300 F10 (#2278 order 8); #2313 F4, F5, F6, F7, F9 (new #2278 subissue) |

Why we proceed this way:

- **Fix only what misleads a reader today.** A finding is fixed when a claim in the current tree is
  false or cannot be followed: an unreachable SHA, a false PASS or checked criterion, a wrong
  workflow name, broken YAML, or a test described wrongly. When a record was only imprecise and nothing
  false stands, the new audit row is the correction. This avoids edits to closed issue folders that
  add nothing. For this reason, #2300 F7 (a line number that moved while the check still passes),
  #2300 F11 (a dropped clause that was already stale), and #2320 F28 (an under-report with no
  false assertion) became `NO_ACTION`.
- **#2293 F6 is declined rather than tracked.** Pull requests to `develop` come from forks, and
  fork pull requests get a read-only token, so the missing `permissions:` block exposes nothing
  in practice. The gap also predates PR #2293, and 11 of 15 workflows share it. A new issue
  would add work in progress without adding protection. Repository-wide least-privilege
  hardening, if wanted, is a separate decision.
- **Larger or non-documentation work is tracked elsewhere.** Rust, workflow, and skill-contract
  changes stay out of this documentation task. They go to the issue that owns them. When no issue
  owns them yet, each new issue is drafted as a spec and created only after maintainer approval.
- **No backfilled rows for Copilot threads resolved before merge.** PR #2290 has 3 such threads
  and PR #2293 has 8, each resolved without a reply. Each row would need a new reply on an
  already-resolved thread, and that reply would carry nothing actionable. The reviewer confirmed
  that the three #2290 fixes landed, and the only partial one is loop F3. The #2293 threads whose
  fixes did not land are exactly loop F2-F4. So each new audit's Processing Log records those
  reviews as context: the review ID, its threads, which fixes landed, and which finding covers
  the rest. This is a deliberate exception to `process-pr-review` 1.4, which the approval record
  names. #2290 F5 is `FIXED` by creating the audit, and its detail entry declines the reply
  backfill.
- **One fix per commit.** Each `FIXED` finding gets its own signed commit, so its
  `Resolution reference` names exactly the change that fixed it. This follows `process-pr-review`
  step 6 and avoids the defect #2300 F12 describes. Audits stay one commit per PR.
- **Order of work** (maintainer-approved 2026-09-28, following the PR #2339 / PR #2344 precedent):
  1. Commit the fixes.
  2. Draft, approve, and create the follow-up issues. Their specs ride in this PR as separate
     commits.
  3. Open this PR as a draft.
  4. Post one disposition reply on each of the 32 threads.
  5. Commit one audit per PR, citing those replies.
  6. Complete T6, then mark the PR ready.

  The audits come after the PR because each row cites a reply on its own thread, and a reply
  that states the disposition needs the PR and the owning issue to exist. The T1 replies only say
  that the disposition is pending, so they cannot serve as resolution references.
- **Approved fixes stay `FOLLOW_UP` until merge.** `process-pr-review` "Reviews Submitted After
  Merge" forbids claiming `FIXED` on `develop` before the follow-up merges. In this PR, the 14
  approved fixes are therefore recorded as `FOLLOW_UP`/`OPEN`, with this PR as `Follow-up PR URL`.
  The T7 close-out changes them to `FIXED`, citing their commit subjects, and replaces each reply
  URL with the final one. NO_ACTION rows are final in this PR; their threads are resolved in T7.
  Each thread therefore ends with three replies: the T1 tracking reply, the disposition reply,
  and the T7 reply. A NO_ACTION thread needs no T7 reply.

## Design and Ownership Review

Not applicable. This task processes review findings and edits documentation; it adds no process,
I/O, or fixture code.

## Bug-Fix Process

Not applicable. The findings are review-process and documentation defects, handled under
`process-pr-review` rather than as product bugs; any finding that turns out to be a product bug
becomes its own bug issue.

## Regression Test Strategy

Not applicable, for the same reason.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Record approval and acknowledge threads | Maintainer approval posted on this issue; one tracking reply on each of the 32 threads. |
| T2 | DONE | Triage against `develop` | Per finding: live or not, with the command or inspection used; proposed disposition. Recorded in [`triage.md`](triage.md). |
| T3 | DONE | Maintainer disposition review | Maintainer approves or changes each proposed disposition; the decision is recorded in the progress log with a durable comment URL. See [Disposition Decisions (T3)](#disposition-decisions-t3). |
| T4 | TODO | Normalize into audits | Five audits hold all 32 findings with approved dispositions; `validate-audit-record.py` exits `0` for each PR. |
| T5 | IN_PROGRESS | Apply approved fixes | Approved documentation fixes applied; follow-up issues created for approved `FOLLOW_UP` items. |
| T6 | TODO | Verify and record completion | Automatic checks, evidence, and acceptance review recorded; implementation PR opened. |
| T7 | TODO | Close the loop after merge | Close-out PR records final dispositions for findings fixed or declined here; those threads are replied to and resolved; each `FOLLOW_UP` thread stays open with a reply naming its owning issue. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T4 | One owning PR's audit record | One signed `docs(pr-reviews)` commit per PR audit. |
| T5 | One approved documentation fix | One signed commit per `FIXED` finding, with the narrow affected scope (for example `docs(issues)`); see [Disposition Decisions (T3)](#disposition-decisions-t3). |
| T6 | Evidence and tracking | One signed `docs(issues)` commit. |
| T7 | Audit close-out after merge | One signed `docs(pr-reviews)` commit in the close-out PR. |

T1-T3 change no repository file; their evidence is the GitHub replies and comments, recorded in the
progress log. No test code is planned.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-triage-post-merge-review-findings/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2003, and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all` and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-26 11:02 UTC - GitHub Copilot - Drafted after read-only triage of the five merged PRs. The finding inventory was generated from `github-review-threads show --unresolved-only` captures, and review and merge times from the GitHub API. The maintainer approved creating this tracking task in chat; its approval as a GitHub comment follows issue creation (T1).
- 2026-09-26 12:20 UTC - GitHub Copilot - Maintainer approved the specification. Created GitHub issue #2347, linked it as a sub-issue of #2003 (`parent_issue_url` verified), and moved this specification to `docs/issues/open/`. T1 done ahead of the spec-only PR, as the maintainer approved: the approval record is <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5846100762>, and each of the 32 threads received one reply naming #2347 with its disposition pending. A GraphQL refetch shows all 32 threads still unresolved, each with that reply (7, 8, 8, 7, and 2 per PR). Spec-only PR pending.
- 2026-09-27 18:10 UTC - GitHub Copilot - The spec-only PR #2348 merged on 2026-09-27 06:11 UTC (`gh pr view 2348`). Created the implementation branch `2347-2003-triage-post-merge-review-findings` from `develop` at `478516cf`. The maintainer decided in chat: T2 results go in an issue-folder `triage.md` linked from this spec; the T3 approval follows the T1 precedent (approval in chat, then an approval record posted on this issue); each approved `FOLLOW_UP` gets a folder-style draft spec, a GitHub issue, and a sub-issue link, each approved first; the branch is pushed only at T6.
- 2026-09-27 19:05 UTC - GitHub Copilot - T2 done. Re-fetched all five PRs' threads (32 unresolved, each still with only the finding and the T1 reply) and checked each finding against `develop` at `478516cf`; results, verification commands, and proposed dispositions are in `triage.md`: 17 `FIXED`, 5 `NO_ACTION`, 10 `FOLLOW_UP` across four proposed follow-up issues and EPIC #2278 order 8. No disposition is approved yet; T3 is next.
- 2026-09-28 07:50 UTC - GitHub Copilot - T3 done. The maintainer approved the triage in chat with four changes (#2300 F7, #2300 F11, #2320 F28, and #2293 F6 become `NO_ACTION`), the no-backfill exception for the pre-merge Copilot threads, and one commit per fix. The approval record was posted at 2026-09-28 07:44 UTC: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>. Recorded the rationale in "Disposition Decisions (T3)" and changed the T5 commit policy from one commit per PR to one per fix. The result is 14 `FIXED`, 9 `NO_ACTION`, and 9 `FOLLOW_UP`. T5 fixes start next.
- 2026-09-28 09:35 UTC - GitHub Copilot - T5 documentation fixes committed: 13 fix commits (#2290 F5 is fixed by its audit in T4) and one commit refreshing `last-updated-utc` stamps that three earlier fix commits left stale. The maintainer approved opening this PR as a draft before the audits, following the PR #2339 precedent, and drafting the three follow-up issues first; see "Order of work" above. Next: the FU-A draft spec.

## Acceptance Criteria

- [ ] AC1: Each of the 32 findings has a row in its owning PR's audit record, with a collision-safe ID, the reviewer's ID, a live-on-`develop` verification, and a maintainer-approved disposition.
- [ ] AC2: The maintainer approval of the dispositions is recorded as a durable GitHub comment URL before any fix or thread resolution. The URL appears in the Ownership section of the #2300, #2313, and #2320 audits when their rows are added, and of the #2290 and #2293 audits when those audits are created.
- [ ] AC3: Every approved fix is applied, and every approved `FOLLOW_UP` item links a created issue.
- [ ] AC4: Each of the 32 threads has a finding-specific final reply. Every thread whose finding this task fixed or declined is resolved; every `FOLLOW_UP` thread stays open, its reply naming the owning issue, until that issue's fix merges. A final GraphQL fetch confirms both.
- [ ] AC5: `validate-audit-record.py` exits `0` for PRs #2290, #2293, #2300, #2313, and #2320.
- [ ] `linter all` exits with code `0`.
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`.
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.

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
| M1 | Tracking replies visible | Open each of the five PRs on GitHub after T1. | Every one of the 32 threads shows a reply naming this issue. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Thread states match dispositions | For each PR, run `github-review-threads fetch` and `list --unresolved-only` after T7. | Only `FOLLOW_UP` threads are unresolved, each with a reply naming its owning issue; every other of the 32 threads is resolved. | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Live-status spot check | Re-run the recorded verification for three findings chosen at random, one per disposition kind. | Each re-run reproduces the recorded result. | TODO | `manual-verification-evidence.md` section V3 |

No disposable verification script is planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Audit records for the five PRs |
| AC2 | TODO | Ownership sections; approval comment URL |
| AC3 | TODO | Fix commits; follow-up issue links |
| AC4 | TODO | M2; thread replies |
| AC5 | TODO | Validator output per PR |

## Risks and Trade-offs

- Thirty-two findings are a lot for one pull request. Mitigation: commits are grouped per owning PR,
  and anything beyond documentation becomes its own issue.
- Some findings concern rules that EPIC #2278 is still reshaping (for example #2313 loop F4-F7 on
  the audit contract). Mitigation: T2 checks each one against the current `develop`, and one that an
  open #2278 subissue owns is pointed to that subissue rather than fixed twice.
- Replying before triage can look like a promise to fix. Mitigation: the T1 reply only names the
  tracking record and states that the disposition is pending.

## Implementation Completion Review

- Retrospective: `Not yet assessed`.
- Create `implementation-retrospective.md` if triage shows a recurring cause of unprocessed
  post-merge feedback worth a guardrail; otherwise record why none was needed.
- When an independent reviewer receives this folder-style specification, record the result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2003.
- Governing workflow: `process-pr-review` "Reviews Submitted After Merge"; PR #2344 extends it to
  findings unprocessed at merge.
- Post-merge precedent: PR #2269 findings, follow-up PR #2271, close-out PR #2276.
- Related: #2333 (the thread tool that surfaced these), PR #2339.
