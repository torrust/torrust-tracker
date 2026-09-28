---
semantic-links:
  skill-links:
    - write-markdown-docs
    - process-pr-review
  related-artifacts:
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md
    - docs/issues/open/2347-2003-triage-post-merge-review-findings/triage.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# Implementation Retrospective — Issue #2347 Post-Merge Review Triage

## Purpose

Record evidence-based process improvements discovered while implementing #2347. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

All 32 post-merge findings on PRs #2290, #2293, #2300, #2313, and #2320 have a
maintainer-approved disposition, a disposition reply on their own thread, and a row in the owning
PR's audit record. Every audit passes `validate-audit-record.py`.

- 13 fixes land in PR #2363: 12 documentation fix commits, and the #2290 audit, which is itself
  the fix for #2290 F5.
- One fix (#2293 F3) landed upstream in PR #2357.
- 9 findings were declined.
- 9 are owned by other issues: the new #2360, #2361, and #2362; the existing #2301; and #2278
  order 8.

The close-out (T7) remains.

## What Went Well

1. Triage against the current tree before proposing dispositions. Three findings were already
   dead (#2293 F8, #2300 F8, and #2313 F10). The reviewer had flagged two of them; triage found
   #2300 F8. One had been fixed upstream by the time the branch was rebased (#2293 F3). The
   approval record was built on facts, not on the reviewer's liveness column alone.
2. One commit per fix. Each `FIXED` row can cite exactly the change that fixed it, which avoids
   the defect #2300 F12 describes.
3. Reproducing before routing. The working-tree Clippy experiments (#2290 F4) and the empty
   discovery run (#2293 F1) turned vague follow-ups into specs with concrete reproductions.

## What Changed During Implementation

- **Order of work.** The spec planned audits (T4) before fixes (T5). An audit row must cite a
  reply on its own thread, and a reply that states the disposition needs the follow-up PR and the
  owning issue to exist. So the order became: fixes, follow-up issues, draft PR, disposition
  replies, then audits (the #2339 / #2344 precedent). Fixes in the PR are `FOLLOW_UP`/`OPEN` until
  merge.
- **A missed owner.** T2 routed #2293 F5 to a new issue. While drafting it, open #2301 turned out
  to own exactly that scope. The routing was amended
  (<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5867552349>).
- **Upstream overlap.** `develop` moved 84 commits. PR #2357 fixed #2293 F3 identically, and the
  rebase dropped this branch's commit
  (<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5868016556>). PR #2357's
  new frontmatter validator also rejected this spec's `status: in_progress`.
- **Self-inflicted defects of the kinds under review.** Two stamps were estimated and ended up
  later than the events they recorded: the #2360 draft log said 09:50 for a 09:40 draft, and the
  first amendment comment said 09:55 for a 09:51 post. Three fix commits edited documents without
  refreshing their `last-updated-utc` (fixed by a follow-up commit). I corrected those before the
  pre-PR review. The F29 correction also moved the retrospective lines cited by three #2320 audit
  entries (F23, F24, and F26), but the F28 row disclosed only F24's shift. Re-wrapping the note to
  two lines restored all three after the pre-PR review, in round 1 of the PR #2363 review
  (`docs(pr-reviews): [#2347] keep the #2320 head-id note at two lines`). The
  pre-PR task review (`agent-review-reports.md`) also found more of the same kind:
  - three stale `last-updated-utc` stamps;
  - the spec's 2026-09-27 19:05 and 2026-09-28 07:50 entries, both estimates, one of them later
    than its own commit;
  - two batch log entries stamped at the batch's first event;
  - claims in the evidence file and PR body that the later commits had made stale.

  All were corrected by appended entries or in-place edits of this branch's unmerged text,
  before the PR was marked ready.
- **Approval URL timing in the existing audits.** `process-pr-review` step 2 asks for the
  approval URL in the original audit before any mutating action. The approval existed as a
  durable comment from 07:45 UTC. But the #2300 and #2320 audits received #2347 edits (the F5 log
  correction and the F29 note) at 09:00 and 09:22, and all 32 threads received replies at 10:29.
  The URL reached the #2300, #2313, and #2320 Ownership sections only in the T4 audit commits
  (10:44-10:54). The reply-first order made the audit commits come last, and the approval line
  moved with them instead of being written first.

## Root Cause

- **Unprocessed post-merge feedback.** Nothing surfaces review threads opened after a merge. The
  32 findings were found by chance while verifying #2333. The review bodies said "nothing below
  asks for remediation", which made silence look acceptable, and no tool lists unresolved threads
  on merged PRs.
- **Recurring finding classes.** The findings themselves come from three repeated habits, which
  `triage.md` groups under "Recurring Causes Observed":
  - citing branch SHAs in evidence and agent-review reports (#2290 F1, #2293 F4, #2300 F9);
  - not re-running recorded evidence after a later commit (#2290 F2, #2300 F6 and F7, #2313 F8);
  - resolving a thread without landing its fix (#2293 F2, F3, and F4).
- **Missed owner.** T2 searched the #2003 and #2278 registers but not the #1347 subissues, because
  the owner search followed the reviewer's framing (CI behaviour) rather than the affected folder's
  parent EPIC.
- **Wrong stamps.** Timestamps were typed from memory instead of taken from `date -u` or the
  event's own `created_at`.

## Improvements for Future Work

1. Add a periodic, read-only check that lists unresolved review threads on recently merged PRs,
   for example a `github-review-threads` mode or a scheduled report. This is a candidate for EPIC
   #2278 or #2003, proposed only if the maintainer wants it; the precedent now exists that such
   findings are otherwise found by chance.
2. In the `AGENT-REVIEW-REPORTS.md` template, state what `process-pr-review` already requires for
   audits: cite commits by subject, never by branch SHA. Three of the 32 findings are this one
   defect in evidence files.
3. In `process-pr-review` post-merge triage, search for an existing owner in the affected folder's
   parent EPIC (the `NNNN-EPIC-slug` prefix), not only in the EPIC the reviewer's topic suggests.
4. Take every log or comment stamp from `date -u` at writing time, or from the event's
   `created_at`. Never estimate. For a batch, stamp the entry at the batch's last event.
5. In `process-pr-review` "Reviews Submitted After Merge", say that when disposition replies
   must precede the audit rows, the Ownership approval line is still committed to each existing
   audit first, as its own commit.

## Avoiding Overcorrection

- Do not require re-running every recorded evidence command after every commit. The four
  "not re-run" findings were caught by review. Improvement 2 addresses the one mechanical class
  (SHAs), which a check can enforce.
- Do not backfill audits for pre-merge Copilot threads resolved without replies. The maintainer
  declined that for #2290 and #2293, and the late review showed their content was already
  accounted for.
- Improvement 1 is a proposal, not a decision. The maintainer chooses whether it becomes an issue.

## Evidence

- Spec: `docs/issues/open/2347-2003-triage-post-merge-review-findings/ISSUE.md`, including its
  "Disposition Decisions (T3)" section and progress log.
- Triage: `triage.md`. Verification: `manual-verification-evidence.md` V1 and V3.
- Approval and amendments:
  - <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>
  - <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5867552349>
  - <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5868016556>
- PR: <https://github.com/torrust/torrust-tracker/pull/2363>.
- Audits: `docs/pr-reviews/pr-2290-review/`, `pr-2293-review/`, `pr-2300-review/`,
  `pr-2313-review/`, and `pr-2320-review/`.
