---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/pr-reviews/pr-2811-review/PR-REVIEW.md
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/issues/open/2798-peer-ord-inconsistent-with-eq/ISSUE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2811

Covers the two review rounds up to the approval of da2ce7 (review 5847136183) and the fixes that
answered it. Written at the maintainer's request, relayed by the PR review retrospective process
session. Any review of this retrospective is recorded in the audit and not counted below.

## Purpose

Record a blameless, evidence-based review of how the pull request was reviewed and how its
findings were processed. This complements the audit record, which tracks what each finding was
and how it was resolved; the retrospective explains why the process cost what it did and what to
change. It does not replace the audit record and is not required for routine reviews.

## Review Summary

| Metric                                 | Value                                                                 |
| -------------------------------------- | --------------------------------------------------------------------- |
| Review rounds                          | 2 (round 1: da2ce7 and Copilot; round 2: da2ce7)                      |
| Audit findings                         | 14                                                                    |
| Re-raised findings (`RE_RAISE_OF`)     | 1 (F9 repeats F1)                                                     |
| Findings about the audit record itself | 2 (F13, F14); 10 more were about the spec and evidence the PR carries |
| Commits on the branch                  | 15 (6 audit-only), before this file                                   |
| First review to last thread resolution | 17:38 to 19:31 UTC on 2026-11-10 (about 1 h 53 min)                   |
| AI models used                         | Claude Sonnet 5.5 (author), unknown (reviewers)                       |

Numbers come from the audit record (rows and review IDs), `git rev-list --count
torrust/develop..HEAD` and a count of `docs(pr-reviews)` subjects in `git log`, and the
`created_at` of the first review comment and the last `resolveReviewThread` result in the audit's
Processing Log. The model is the one the session names.

## What Went Well

1. The code change itself drew one finding (F1, repeated as F9): the option was chosen with the
   maintainer after an investigation (T1) that showed production never orders a `Peer`.
2. A red run, a green run and a like-for-like recheck were recorded in the evidence, and da2ce7
   recomputed them. Round 2 raised only minor findings and approved.
3. Every reply was posted before its thread was resolved, and the audit validator passed after
   each round.

## What Made the Review Costly

Twelve of the 14 findings were about documents (spec, evidence, audit record); the other two (F1 and its repeat F9) were about the doctest, the only code change.
The table gives, per finding, what was flagged, the root cause, the type of problem and what
would have prevented it. Reviewer IDs are in the audit.

| Finding     | Round | Flagged                                                                          | Root cause                                                                                             | Type                    | What would have prevented it                                                 |
| ----------- | ----- | -------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ----------------------- | ---------------------------------------------------------------------------- |
| F1 (F9 dup) | 1     | The doctest guards `Ord` only while AC1 says neither trait                       | The test was written for the trait I removed, not derived from the AC text                             | Test gap                | Write each test from the AC wording and ask which broken state it would miss |
| F2          | 1     | Risks dropped the major-version note and gave a false reason (crate publishable) | I repeated "most packages are unpublished" from conversation and did not read `publish` or the version | Unverified claim        | Read the manifest before stating publishability                              |
| F3          | 1     | Two V2 sections, no code state named                                             | The template stub stayed, and the recheck ran on an uncommitted tree                                   | Process (evidence)      | Re-read the `fix-bug` evidence rules at completion; delete stubs             |
| F4          | 1     | No green output, stale test-boundary and closing lines                           | The decision changed from option C to A and I did not sweep the evidence for the old plan              | Doc/code mismatch       | Sweep every record for the old option after a decision change                |
| F5          | 1     | No task review, stale checkpoints and frontmatter                                | I skipped the pre-PR checkpoint pass of `review-task`                                                  | Process                 | Run the pre-PR checklist before opening the PR                               |
| F6          | 1     | A commit subject claims work that another commit carries                         | A pre-commit hook rejected the red doctest; the staged spec files rode into the next commit            | Tooling / process       | Check `git status` and `git diff --cached --stat` after a failed commit      |
| F7          | 1     | Cross-references still point at the old recommendation                           | Same cause as F4: no sweep after the decision                                                          | Doc/code mismatch       | The same sweep                                                               |
| F8          | 1     | "The only way" overstates; a derived `Ord` also agrees                           | I wrote a stronger claim than the analysis supported                                                   | Overstatement           | Review claims in the Decision for absolutes                                  |
| F10         | 2     | The strategy says a red run happened that did not (the earlier run used `Ord`)   | After changing the bound I described the new test's red run without running it                         | Statement beyond a run  | After changing a test, rerun it red and cite exactly that run                |
| F11         | 2     | "The versioning ADR above" points at nothing above                               | The F2 fix added the reference without reading the paragraph in context                                | Fix introduced a defect | Re-read each edit in its surroundings                                        |
| F12         | 2     | The retrospective rationale is not in the progress log and names one discovery   | I ticked a checkpoint without checking its literal condition; the second discovery was known           | Process (checkpoint)    | Tick a checkpoint only after reading its exact wording                       |
| F13         | 2     | The Processing Log cites branch commit ids                                       | I copied push output into a record whose rule forbids branch ids                                       | Process (record rule)   | A check for ids in the audit before commit                                   |
| F14         | 2     | The audit lacks the template's Completion Rules section                          | I built the audit from the header of an earlier audit instead of from the template                     | Process (tooling)       | Generate the audit from the template; have the validator check the section   |

Round 1 mostly had one source: the option decision and its late documentation, then the hook
accident. Round 2 findings were mostly my own repair work (F10, F11, F12) and two record-hygiene
slips (F13, F14).

## Root Causes

1. **No sweep after a decision change.** Choosing A after drafting for C left stale text in the
   spec and evidence (F4, F7) and produced the overstatement in the rewrite (F8).
2. **Statements went further than what ran or what I read.** F2, F10, F12 and F13 are claims or
   ticks that the file or the run did not support.
3. **Repairs were not re-read.** F11 and F10 came from fixes made in round 1.
4. **A failed hook left state behind.** The pre-commit doctest step rejected a red-state commit
   and the staged files joined the next commit (F6).
5. **The audit was generated from a model, not from the template** (F14), and no check covered the
   rules the reviewer later enforced by hand (F13, F14).

## What We Learnt

1. After changing a decision, grep every record on the branch for the old option before pushing.
2. A claim about the world (publishable, red, green) needs a command or file read, named in the
   record; a conversation remark is not evidence.
3. Tick a checkpoint only after reading its literal text.
4. After a rejected commit, look at what is staged before committing again.
5. Re-read a fix in its context, since it can break an adjacent sentence.

## Improvements for Future Reviews

1. Add a decision-change sweep step to the `fix-bug` evidence checklist — `fix-bug` skill — PROPOSED
2. Have `validate-audit-record.py` check that the audit ends with the template's Completion Rules
   and that the Processing Log has no PR-branch commit id — `validate-audit-record.py` — PROPOSED
3. Say in the audit-creation step to start from `PR-REVIEW-TEMPLATE.md`, not a prior audit —
   `process-pr-review` skill — PROPOSED
4. Note in the commit steps to inspect the index after a hook rejection — `open-pull-request`
   skill — PROPOSED

## Avoiding Overcorrection

The disk-quota stop and the denied force push (which needed maintainer approval, then succeeded)
cost time but are not process gaps. The red-state commit blocked by the pre-commit doctest step is
correct behaviour; the only change worth making is to record why T4 and T5 share a commit, which
was done. No new checkpoint field is justified by one PR.

## Evidence

- Audit record: `docs/pr-reviews/pr-2811-review/PR-REVIEW.md`
- Pull request: https://github.com/torrust/torrust-tracker/pull/2811
- Reviews: 5846498596 (Copilot), 5846502890 (da2ce7, round 1), 5847136183 (da2ce7, round 2,
  approved).
- `git rev-list --count torrust/develop..HEAD` and `git log --format=%s torrust/develop..HEAD`
  support the commit counts.
