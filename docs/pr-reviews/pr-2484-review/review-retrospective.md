---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/pr-reviews/pr-2484-review/PR-REVIEW.md
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2320-review/review-retrospective.md
    - docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2484

Current as of review 5459306372 (round 4). Written at the maintainer's request after four human
rounds on a documentation-only spec PR, to find out why the rounds kept coming and what would
make the next spec PR converge in one or two.

## Purpose

Record a blameless, evidence-based review of how the pull request was reviewed and how its
findings were processed. This complements the audit record, which tracks what each finding was
and how it was resolved; the retrospective explains why the process cost what it did and what to
change. It does not replace the audit record and is not required for routine reviews.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 5 (4 human, 1 bot) |
| Audit findings | 14 (13 recorded; F14 is non-blocking and pending its row) |
| Re-raised findings (`RE_RAISE_OF`) | 0 |
| Findings about the audit record itself | 3 (F9, F10, F14) |
| Commits on the branch | 22 (5 audit-only) |
| First review to last thread resolution | 2026-10-08 07:23 UTC (Copilot) to 15:47 UTC (round 4 approval); 8 h 24 min |

Sources: `gh api --paginate repos/torrust/torrust-tracker/pulls/2484/reviews` for rounds, their
times and the head each reviewed; the Findings table of `PR-REVIEW.md` for counts and
relationships; `git log torrust/develop..<branch>` for commits, with `docs(pr-reviews)` subjects
counted as audit-only.

## What Happened, Round by Round

| Round | Head | Findings | What they were about |
| --- | --- | --- | --- |
| Copilot | `046ecb540` (the narrow spec) | 0 | Approval recommended |
| 1 (human) | `0438fa81b` (broadened spec, new inventory) | F1–F5 Minor/Nit, F6–F7 Suggestion | Five factual errors in the inventory (dates, a dependency, module visibility, an alias) and two additions. The only round about the reviewed content |
| 2 (human) | `c43fc9013` (task converted to sub-EPIC) | F8 Nit, F9 Major, F10 Minor, F11 Nit, F12 Suggestion | All five caused by the conversion commit: a live path to the deleted `ISSUE.md` in the audit (F9), line ranges into the deleted file (F10), the same event dated 12:33 in one EPIC and 12:53 in the other (F11), a section dropped without being listed (F12), a 103-column log line (F8) |
| 3 (human) | `542f1b293` (round-2 fixes) | F13 Nit | A log entry written while fixing F12 said a future spec "carries" a section. Approved with the Nit |
| 4 (human) | `feb0d8f24` (F13 fix) | F14 Minor, non-blocking | The F13 fix added two lines to `EPIC.md`, shifting line ranges the audit cited for F7 and F10. Approved |

Rounds 2 to 4 were entirely self-inflicted: each round's fix produced the next round's finding.
Rounds 3 and 4 also cost the approval that preceded them, because pushing to the PR dismisses
stale approvals (`dismiss_stale_reviews_on_push` is on for `develop`).

The content of the PR also changed twice while it was under review: from a task spec to a
broadened decision spec (before round 1), and from a task to a sub-EPIC (between rounds 1 and 2).
Those were maintainer scope decisions, not process failures, and they are not what this
retrospective proposes to prevent. What it proposes to prevent is the second-order cost: a
restructure pushed mid-review without the checks the reviewer was going to run.

## What Went Well

1. **Round 1 found real errors.** F1 to F5 were wrong facts in a document meant as a baseline
   (an inventory). Fixing them before the ADR work starts is the review doing its job.
2. **Severity was honest and the ACK protocol worked.** The reviewer approved at round 3 with a
   Nit outstanding and at round 4 with a Minor outstanding, each time naming the head. The
   author could have merged either time; the choice to fix F13 first was deliberate.
3. **One commit per finding, replies before resolution, and the audit record** were followed in
   every round. The validator reported 0 failures at every head.
4. **The reviewer states his checks.** Each review body lists what was recomputed (range-diff,
   link resolution, stamp order, line width, path liveness, the validator). That list is a
   specification of the pre-push check the author should have run; see Improvements.

## What Made the Review Costly

1. **A rename pushed without a reference sweep (F9, F10).** The conversion deleted `ISSUE.md`
   and created `EPIC.md`. The author re-pointed the EPIC, the report and the inventory, but not
   the audit record, which is outside `docs/issues/` and so outside the frontmatter validator's
   related-artifact check. The audit's two line-range citations into the deleted file were not
   re-derived either.
2. **One event, two stamps (F11).** The conversion was logged in two files. The parent EPIC's
   entry was written first with the clock time at writing (12:33); the sub-EPIC's entry was
   written after a sync with another workspace, with a later clock time (12:53). Nothing compares
   the two.
3. **A section removed without being listed (F12).** The conversion's log entry listed the
   sections moved to the subissues but omitted one it had dropped. No check compares a rewrite's
   headings against the previous version.
4. **Line-number citations in prose (F10, F14).** Two audit verifications cited `EPIC.md` line
   ranges. Any edit above those lines invalidates them, so every later fix to the EPIC risked an
   audit finding, and one did (F14 followed directly from the F13 fix).
5. **Fixing a non-blocking Nit after an approval (round 4).** F13 was a one-word tense error the
   reviewer marked non-blocking. Fixing it dismissed the round-3 approval and bought one more
   round, which produced F14.
6. **A 100-column convention that no tool enforces (F8).** `MD013` is off, so a 103-column line
   passes `linter markdown`. The convention exists only in the reviewer's checks.
7. **Two workspaces processed round 1 at once.** The maintainer asked a second agent session to
   address round 1 while this session was doing the same. The second session's fixes were kept;
   this session's duplicates were discarded. The cost was a force-push rejection and a reset, not
   a review round, but the F11 stamp mismatch is a direct consequence of writing the same event
   from two clocks.

## Root Causes

- **The author's pre-push check is smaller than the reviewer's check.** Pre-commit runs
  linters, frontmatter validation and doc tests; the author also ran the audit validator. The
  reviewer additionally recomputes: every relative link, every path named in frontmatter
  anywhere in the tree (not just under `docs/issues/`), stamp order within and across files,
  prose line width, and whether cited line ranges still hold. Every round-2 to round-4 finding
  is in that difference.
- **Restructuring under review has no documented procedure.** `create-issue` covers promoting a
  draft to an open spec and names no step for converting an issue into an EPIC (the reviewer
  noted this in round 2). The author improvised: rename, rewrite, re-point the links he
  remembered. The sweep that was missing (grep the tree for the old path) is one command.
- **Timestamps are typed from the clock, not derived from events.** The process-pr-review skill
  derives audit stamps from `git log` and GitHub `created_at`; spec progress logs have no such
  rule, so the same event got two times.
- **Verification sentences are free prose.** The audit template asks for a "Current-tree
  verification" but does not say how to anchor it. Line numbers are the shortest anchor and the
  most fragile one.
- **The decision to fix or defer a non-blocking finding is made ad hoc.** The skill allows
  `FOLLOW_UP`, and the reviewer's approval made deferral the cheaper path, but nothing in the
  workflow says "after an approval, defer non-blocking findings by default".

## What We Learnt

1. Before pushing to a PR under review, run the reviewer's checks, not just the committer's: a
   tree-wide grep for every path the push deletes or renames, stamp order across every file the
   push touches, and line width.
2. Anchor verification claims to headings or quoted phrases, never to line numbers. A line number
   is correct for exactly one head.
3. Derive every timestamp from an event (`git log --format=%aI`, GitHub `created_at`) and write
   each event in one place first, then copy it. Never type the clock twice for one event.
4. After an approval, fix only blocking findings. Record non-blocking ones as `FOLLOW_UP` and fix
   them in the next PR that touches the file, unless the maintainer asks otherwise.
5. A rewrite that removes a section must list the removed section in its log entry, with where
   its content went, before it is pushed.
6. One workspace per PR branch. If a second session must take over, the first one stops first.

## Improvements for Future Reviews

Ordered by expected return. Each names its owner artifact and a disposition. The maintainer has
asked that the owner of EPIC #2003 (Automation Tools and AI Agent Guardrails) read this
retrospective and decide which items enter that EPIC or its sub-EPIC #2278 (Strengthen PR Review
Author Self-Audit). Items marked `PROPOSED` are therefore not applied here.

1. **Author pre-push sweep for PRs under review** — `process-pr-review/SKILL.md` and EPIC #2278
   row 5 ("Add the author self-audit gate") — `PROPOSED`. A deterministic check run before every
   push to a reviewed branch, as a Rust tool under `contrib/dev-tools/checks/`:
   - no tracked file names a path that the push deletes or renames (the F9 grep, tree-wide,
     including `docs/pr-reviews/`);
   - in every changed spec, `last-updated-utc` is at or after its latest progress-log entry, and
     the log is chronological (the F11 check within one file);
   - no prose line over 100 columns in changed Markdown under `docs/issues/` and
     `docs/pr-reviews/` (F8), or alternatively turn `MD013` on for those folders with a
     100-column limit and table/link exemptions so `linter markdown` enforces it;
   - every `Current-tree verification` that names a file and a quoted phrase finds that phrase
     exactly once (replaces line-range checking; see item 2).
   This is the complement of the reviewer's stated checks. It belongs with #2278's gate rather
   than as a new tool, because that EPIC already owns author-side determinism.
2. **Ban line-number anchors in audit verifications** — `docs/templates/PR-REVIEW-TEMPLATE.md`
   and `process-pr-review/SKILL.md` — `APPLIED_IN_THIS_PR` for this record (the F14 fix cites
   sections and quoted phrases), `PROPOSED` for the rule. The verification sentence names the
   file and a heading or a quoted phrase; line numbers may be added only inside a parenthesis
   marked with the head they were true at.
3. **Defer non-blocking findings after an approval by default** — `process-pr-review/SKILL.md`
   step 7 and `merge-pull-request/SKILL.md` — `PROPOSED`. When the latest human
   review is an approval and a finding is marked non-blocking, record it as `FOLLOW_UP` with the
   PR that will carry it, reply so, and do not push. Pushing dismisses the approval and costs a
   round. The maintainer can still ask for an immediate fix.
4. **Derived timestamps for spec progress logs** — `docs/templates/ISSUE.md`,
   `docs/templates/EPIC.md` and `create-issue/SKILL.md` — `PROPOSED`. State that a progress-log
   stamp is the time of the event it records (commit author time for a committed change, the
   GitHub timestamp for a review or comment), and that one event logged in two files carries the
   same stamp. The audit template already has this rule; the spec templates do not.
5. **A documented step for restructuring a spec under review** — `create-issue/SKILL.md` —
   `PROPOSED`. Cover task-to-EPIC conversion and folder renames: rename with `git mv`, rewrite,
   list every removed section and its destination in the log entry, then run the item 1 sweep.
   The reviewer noted in round 2 that `create-issue` names no step for turning an issue into an
   EPIC.
6. **One session per PR branch** — `docs/agents/orchestration.md` or the Collaboration
   Principles in `AGENTS.md` — `PROPOSED`. Before an agent session processes a review on a branch
   it did not create, it checks the remote head against its local head and asks the maintainer
   which session owns the branch. This PR lost a force-push to that collision; the F11 mismatch
   came from it.

## Avoiding Overcorrection

- No new audit fields. Every round-2 to round-4 finding was a stale claim or a convention
  breach, not a missing field; the thirteen rows and seven log entries were enough to record them.
- No rule against changing a PR's scope under review. Both restructures were the maintainer's
  decisions, and the second (the EPIC conversion) is the result this PR exists to deliver. The
  cost to prevent is the unchecked push after a restructure, which item 1 covers.
- No per-round reconciler beyond what the #2320 retrospective already proposed (its items 9 and
  10). This PR had no state drift between threads, audit and tree: 0 re-raises, every thread
  replied and resolved, validator 0 failures at every head. Its failure mode is different (stale
  claims after the author's own edits) and is covered by items 1, 2 and 4 here. The two
  retrospectives are complementary inputs to EPIC #2278, not competing designs.
- No requirement to fix Nits before approval can be given. The reviewer's practice of approving
  with non-blocking findings is what makes item 3 possible.

## Evidence

- Audit record: `docs/pr-reviews/pr-2484-review/PR-REVIEW.md`
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2484>
- Reviews: Copilot 5453137624 (07:23 UTC); da2ce7 5454599321 (round 1, 09:38), 5457064358
  (round 2, 13:08), 5458896567 (round 3, 15:17, approved then dismissed by the F13 push),
  5459306372 (round 4, 15:47, approved)
- Commands: `gh api --paginate repos/torrust/torrust-tracker/pulls/2484/reviews`;
  `git log --format='%ad %h %s' --date=iso torrust/develop..<branch>`;
  `gh api repos/torrust/torrust-tracker/rules/branches/develop` for
  `dismiss_stale_reviews_on_push: true`
- Prior retrospective whose proposals this one builds on:
  `docs/pr-reviews/pr-2320-review/review-retrospective.md` (items 9 and 10)
- The EPIC expected to own the follow-ups:
  `docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md`
