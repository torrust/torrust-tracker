---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2809-review/PR-REVIEW.md
    - docs/issues/open/2808-workspace-package-release-process/ISSUE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2809

PR #2809 is the spec-only pull request of issue #2808 (the workspace-package release process). It adds one
issue specification and the audit record of its own review. This retrospective covers how the findings were
processed, not the content of the specification. It was requested by the maintainer and follows
`docs/templates/PR-REVIEW-RETROSPECTIVE.md`. The classification below is the author's judgement, made after
the fact, not a measurement.

## Purpose

Record a blameless, evidence-based review of how this pull request was reviewed and how its findings were
processed. It complements the audit record, which tracks what each finding was and how it was resolved, and
explains why the review cost what it did and what to change.

## Review Summary

| Metric                                   | Value                                                                                     |
| ---------------------------------------- | ----------------------------------------------------------------------------------------- |
| Review rounds                            | 7 (5 human `da2ce7` reviews, 3 Copilot reviews; 8 submissions, round 1 had both)          |
| Audit findings                           | 31 (24 `da2ce7`, 7 Copilot)                                                               |
| Re-raised findings (`RE_RAISE_OF`)       | 0                                                                                         |
| Findings about the process record itself | 24 (audit record, thread replies, progress-log entries); 7 about the specification        |
| Commits on the branch                    | 16 before the commit that adds this file (10 audit-only)                                  |
| First review to last thread resolution   | 2026-11-10 16:46:32 UTC to 21:26:47 UTC (includes waits for the maintainer and reviewers) |
| AI models used                           | Claude Opus 5.5 and Claude Sonnet 5.5 (author), unknown (reviewers)                       |

How the numbers were derived: rounds and findings from the `## Findings` table of the audit record (31 rows,
all `ORIGINAL`); review submissions from `gh api repos/torrust/torrust-tracker/pulls/2809/reviews`; commits
from `git log --format=%h torrust/develop..HEAD` and, for audit-only, the commits whose changed files are all
under `docs/pr-reviews/`. The model split comes from the `Co-Authored-By` trailers: Claude Opus 5.5 wrote the
first six commits (to the one dated 17:23 UTC) and Claude Sonnet 5.5 the rest, after the maintainer switched
the session model; Sonnet 5.5 also drafted the first progress-log entry. The reviewers' models are not stated
anywhere in the review text, so they are `unknown`. A model is recorded here as an observation, not a
finding.

## What Went Well

1. The maintainer's decisions were asked for before the PR (split the release document, wait for issue #2576,
   rename the packages document), which avoided conflicting edits with the package-README issue.
2. Two findings caught real contradictions in the specification: `release-new-version` steps 2 and 9 still
   say to bump every manifest (Copilot, F6), and the server-lib release document still repeats the steps
   (Copilot, F7).
3. The audit validator and the commit-id check of `open-pull-request` found real mistakes before a push,
   and no closing keyword reached a commit, title, or body.
4. Replies were posted after the fix was pushed and threads were resolved only after a reply.

## What Made the Review Costly

Seven of the 31 findings concern the specification. The other 24 concern prose the author wrote about the
earlier fixes. Every round of fixes added new log entries, audit rows and replies, and each new sentence
was a new place to be wrong. Each row gives the finding (audit ID, reviewer), what was flagged, the root
cause, the type of problem, and what would have prevented it.

| ID  | Reviewer | Flagged                                                     | Root cause                                                          | Type                | Would have prevented it                                           |
| --- | -------- | ----------------------------------------------------------- | ------------------------------------------------------------------- | ------------------- | ----------------------------------------------------------------- |
| F1  | da2ce7   | `git diff -M` cannot show a split that keeps the old path   | Wrote git behaviour from memory, never simulated the split          | Unverified claim    | Run a command on a simulated change before naming it as evidence  |
| F2  | da2ce7   | Bare `issue #7` links this repository's issue 7             | Used repo-local shorthand for a cross-repo issue                    | Link integrity      | Qualify every cross-repo reference                                |
| F3  | da2ce7   | A past log entry was rewritten in place                     | A repo-wide replace for a rename also hit a past entry              | Append-only record  | Treat logs as append-only; replace only outside log lines         |
| F4  | da2ce7   | Times labelled UTC were later than their commits            | Stamped from a guess, not from `date -u` or `git log`               | Timestamp           | Stamp after the event from the command or forge time              |
| F5  | da2ce7   | T2-T6 read as ready while the work waits for #2576          | Added the blocker to the text, not to the plan table                | Spec consistency    | Sweep every section that restates status when a decision lands    |
| F6  | Copilot  | Steps 2 and 9 of `release-new-version` still bump all       | Saw the step 6 drift only; did not read the whole skill for it      | Scope miss          | Search the whole artifact for the same contradiction              |
| F7  | Copilot  | M2 expects no duplication in docs the issue cannot edit     | Did not read the server-lib release document before writing M2      | Unread dependency   | Read the counterpart document first                               |
| F8  | da2ce7   | Correction said times were "one hour or more" ahead         | Generalized from one entry; offsets were mixed                      | Unverified claim    | Compute the offset of each entry                                  |
| F9  | da2ce7   | A grep did not print the commit point it was cited for      | Wrote the verification without running it                           | Unverified claim    | Run each verification command before citing it                    |
| F10 | da2ce7   | A reply cites a branch commit by id                         | Used `git show <id>` in a reply for brevity; rebases invalidate it  | Commit id in record | Cite the commit subject                                           |
| F11 | da2ce7   | Two log entries record events later than their stamps       | Wrote entries before the events happened                            | Timestamp           | Write an entry after the event; stamp from the forge or `git log` |
| F12 | Copilot  | The 16:10 entry says "anchor", against the final decision   | Reviewer read a dated historical entry as current (declined)        | Reviewer-side       | None needed; later entries record the change                      |
| F13 | Copilot  | "audit F8" is not defined in the audit                      | Reviewer read a head before F8 existed (declined)                   | Reviewer-side       | None needed; the finding existed at the current head              |
| F14 | da2ce7   | No-change threads lack the prescribed reply form            | Invented "No action (...)" instead of copying the prescribed form   | Prescribed form     | Copy the string from the skill's step 7                           |
| F15 | da2ce7   | A reply cites a branch commit by id (second time)           | The F10 sweep only covered replies I remembered                     | Commit id in record | Script the commit-id sweep over every reply after each edit       |
| F16 | da2ce7   | "Review replies are not rewritten" contradicted by edits    | Justification written from belief, not checked against my own edits | Unverified claim    | Check a stated rule against the record before citing it           |
| F17 | da2ce7   | F10's resolution cites a commit, not the reply that fixed   | The validator accepts any PR subject; I took the permissive path    | Prescribed form     | Read the "fix outside the tree" rule; validator could warn        |
| F18 | da2ce7   | The audit itself cites a branch commit by id                | Named the head by its id while describing the fix                   | Commit id in record | Run the commit-id check on the audit before every push            |
| F19 | Copilot  | `related-pr: null` although the PR adds the spec            | Convention is undocumented; sibling spec PRs keep `null` (declined) | Convention          | State the `related-pr` convention in `create-issue`               |
| F20 | Copilot  | `PR 6` and `PR 8` are not linkable                          | Avoided backlinks on purpose; the ambiguity was real                | Link integrity      | Qualify cross-repo PR references                                  |
| F21 | Copilot  | Use a Markdown link for the audit's `Source:` URL           | Style suggestion; the template uses the bare form (declined)        | Style               | None needed                                                       |
| F22 | da2ce7   | F19 and F21 lack the form applied to F12 and F13            | Fixed two siblings, did not sweep the other no-change rows          | Incomplete sweep    | Sweep every row of the same kind after fixing one                 |
| F23 | da2ce7   | Audit names the wrong head for Copilot's round 5            | Inferred the head from timing, not from the forge `commit_id`       | Unverified claim    | Read `commit_id` from the review before naming a head             |
| F24 | da2ce7   | The 17:42 entry was rewritten in place (again)              | The F23 fix edited a log entry; second occurrence of F3's class     | Append-only record  | Fix log statements with an appended correction                    |
| F25 | da2ce7   | A grep prints lines the verification did not name           | Described the output from memory                                    | Unverified claim    | Run the grep at the final head and list its lines                 |
| F26 | da2ce7   | A reply quotes a grep whose pattern is a commit id          | The "no reply cites an id" sweep ran before the last reply edit     | Commit id in record | Run the sweep after the final edit, not before                    |
| F27 | da2ce7   | `related-pr` "names the implementation PR" is overstated    | Generalized from two specs without searching all of them            | Unverified claim    | Search all specs before stating a convention                      |
| F28 | da2ce7   | F23's grep prints the lines that quote its own pattern      | The verification text sat in the file the grep searches             | Unverified claim    | Describe the expected lines; re-run the grep after adding rows    |
| F29 | da2ce7   | A reply still says the restored entry uses the new wording  | Restored the entry, did not sweep replies that described it         | Incomplete sweep    | Sweep replies and notes after reverting a statement               |
| F30 | da2ce7   | A reply gives a stamp as 19:19 instead of 19:18             | Typed the stamp from memory                                         | Unverified claim    | Copy stamps from the file with grep                               |
| F31 | da2ce7   | A spec fix rode in an audit commit; body misnumbers a round | Batched a spec correction into an audit commit to save a push       | Commit hygiene      | One commit per concern, as step 9 of the skill asks               |

Counts by primary type (31 findings): unverified claim 9 (F1, F8, F9, F16, F23, F25, F27, F28, F30), commit
id in a durable record 4 (F10, F15, F18, F26), prescribed form 2 (F14, F17), incomplete sweep 2 (F22, F29),
timestamp 2 (F4, F11), append-only record 2 (F3, F24), link integrity 2 (F2, F20), specification
consistency, scope or unread dependency 3 (F5, F6, F7), reviewer-side or declined 4 (F12, F13, F19, F21),
commit hygiene 1 (F31). Four findings were declined with a reply (F12, F13, F19, F21); none was a false
positive that cost a re-raise.

## Root Causes

1. **Claims written before the command ran.** Nine findings are a statement about output, a stamp, or a
   convention that the author had not checked at the final head. Several fixes were themselves such
   statements, so the next round found the next one.
2. **Records that quote what they search.** F25 and F28 are the same defect: a verification line inside the
   file it greps, so adding rows changed its output.
3. **A rule fixed in one place, not everywhere.** The commit-id rule failed four times (F10, F15, F18, F26),
   and the in-place edit of a log twice (F3, F24), because each fix swept only what the author remembered.
   The skill's "sweep for the same mistake" rule exists; nothing ran it mechanically.
4. **No author self-audit gate.** Stamps later than commits, ids in replies, and verification greps are all
   checkable by a script. The skill points to issue #2503 for that gate, which was not in place.
5. **Permissive tooling.** The audit validator accepts any PR commit subject as a resolution reference, so
   F17 passed it.
6. **Fix rounds on records multiply.** 24 of 31 findings were about records, and every fix round produced a
   new record. A round of fixes that adds text will add findings.

## What We Learnt

1. Write a claim about output only after running the command at the final head, and quote nothing else.
2. A log is append-only. Correct it with a new entry, never an edit, even for the author's own wording.
3. Never put a branch commit id in a durable record or in a reply; name the commit by its subject.
4. After fixing one instance, search for the class before replying, and run the search again after the last
   edit.
5. A verification text should say what to expect (which lines), and should not be searchable by its own
   pattern.
6. Batch fixes, but keep one concern per commit; a spec fix does not belong in an audit commit.

## Improvements for Future Reviews

None was applied in this pull request; the maintainer decides which become follow-up issues.

1. An author self-audit script that checks stamps against `git log`, runs the commit-id check over the
   files and over thread replies, and re-runs the verification greps — `process-pr-review` and issue #2503
   — PROPOSED
2. Extend the commit-id check of `open-pull-request` to review replies, which it does not scan today —
   `open-pull-request` — PROPOSED
3. Make the audit validator warn when a resolution reference for a fix outside the tree is a commit subject
   (F17) — `validate-audit-record.py` — PROPOSED
4. State the `related-pr` convention (spec-only PR or implementation PR) — `create-issue` and
   `docs/templates/ISSUE.md` — PROPOSED
5. State the reply form for a no-change thread that no earlier finding covers (F22 had to name the audit
   template instead of a finding ID) — `process-pr-review` — PROPOSED

## Avoiding Overcorrection

- Four findings were reviewer-side (F12, F13, F19, F21) and one declined cheaply each; no rule or tool is
  justified by them.
- The review cost came from the author's own prose, not from missing process. Writing fewer claims in the
  record, and only after the command ran, is cheaper than more fields or more tooling.
- Findings F6 and F7 were real content defects and justify no process change.

## Evidence

- Audit record: `docs/pr-reviews/pr-2809-review/PR-REVIEW.md`
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2809>
- Issue specification: `docs/issues/open/2808-workspace-package-release-process/ISSUE.md`
- Counts of findings and rows: `grep -c '^| F[0-9]* ' docs/pr-reviews/pr-2809-review/PR-REVIEW.md` (31) and
  `grep -c 'RE_RAISE_OF' docs/pr-reviews/pr-2809-review/PR-REVIEW.md` over the table rows (0).
- Review submissions: `gh api repos/torrust/torrust-tracker/pulls/2809/reviews`.
- Model trailers: `git log --format='%h %(trailers:key=Co-Authored-By,valueonly)' torrust/develop..HEAD`.
