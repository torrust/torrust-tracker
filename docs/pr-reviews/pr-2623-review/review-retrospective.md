---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/pr-reviews/pr-2623-review/PR-REVIEW.md
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2553-review/review-retrospective.md
    - docs/issues/open/2503-2278-author-self-audit-gate/ISSUE.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2623

Covers the eight reviews of the pull request up to and including review 5631998994 (round 6, the
review of the head after the round 5 fixes). Written at the maintainer's request, because the
review took six human rounds and most findings concerned the author's prose, not the code. Any review
of this retrospective is recorded in the audit and not counted below.

## Purpose

Record a blameless, evidence-based review of how the pull request was reviewed and how its
findings were processed. This complements the audit record, which tracks what each finding was
and how it was resolved; the retrospective explains why the process cost what it did and what to
change. It does not replace the audit record and is not required for routine reviews.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 8 reviews (6 human, 2 bot); 7 produced findings (the first bot review was a summary with none) |
| Audit findings | 17 (16 human, 1 bot) |
| Re-raised findings (`RE_RAISE_OF`) | 0 |
| Findings about the audit record itself | 3 (F13, F15, F17) |
| Findings about the author's other prose (docs, evidence, specs, PR body) | 11 (F1, F2, F3, F5, F6, F7, F8, F9, F11, F12, F16) |
| Findings about code, the branch, or retained evidence | 3 (F4 a test assertion, F10 a merge conflict with `develop`, F14 retained logs) |
| Findings introduced by the author's own fix to an earlier finding | 8 (F7, F8, F9, F11, F13, F15, F16, F17) |
| Commits on the branch | 29 non-merge and 2 merge commits not on `torrust/develop` at the head this retrospective starts from; 4 audit-only (they touch nothing but `PR-REVIEW.md`) |
| First review to last thread resolution | 2026-10-22 07:47 UTC (first bot review) to 2026-10-23 17:46 UTC (F17 reply; the thread was resolved right after): about 34 hours |
| AI models used | Claude Sonnet 5.5 (author, per the maintainer; the progress-log stamps of this work say Claude Opus 5.5 and are unverified, see the correction entries); unknown (da2ce7 and the bot reviewer) |

Sources, all run at the time of writing:

- Reviews, their states and times: `gh api repos/torrust/torrust-tracker/pulls/2623/reviews`.
- Findings, rows and re-raises: the Findings table of `PR-REVIEW.md` (17 rows, all `ORIGINAL`).
- Dismissals: `gh api repos/torrust/torrust-tracker/issues/2623/timeline`, events `review_dismissed`:
  five of the six human reviews were dismissed by the push that fixed their findings; the round 4
  review requested changes and was not dismissed.
- Commits: `git rev-list --no-merges --count torrust/develop..HEAD` and
  `git rev-list --merges --count torrust/develop..HEAD`; 8 of the 29 were on the branch at the first
  reviewed head and 21 came after it; 3 of those 21 touch files under `src`, `packages`, or `tests`
  (`git rev-list --no-merges <first reviewed head>..HEAD ^torrust/develop`, then `git show --name-only`).
  The first reviewed head is the one of review 5610834925; it is not cited by id (repository rule).
- Added lines: `git diff --numstat torrust/develop...HEAD`; code and tests (`src`, `packages`,
  `tests`, `Cargo.toml`) 2078, everything else 2460 (documents and the retained evidence logs).
- "Introduced by the author's own fix" is read from the Concern of each finding in the audit and
  from the order of the commits: for each of the eight, the text came from a fix of the earlier
  finding named below.

## What Happened, Round by Round

| Round | Review | Findings | Cause of each |
| --- | --- | --- | --- |
| 1 | 5616192898 | F1 to F6 | F1 stale status row, F6 stale reference (not propagated); F2, F3, F5 wording that claimed more than was checked; F4 test printed the wrong snapshot |
| 2 | 5618477521 | F7 to F9 | F7 from the fix of F5; F8 and F9 from the evidence added for F2 (V7) |
| 3 | 5625229542 | F10, F11 | F10 `develop` moved (external); F11 from the fix of F8 |
| 4 | 5630959120, bot 5631096012 | F12, F13, F14 | F12 PR body still said what rounds 1 and 2 had corrected; F13 from the fix of F10; F14 asked for the logs to be retained |
| 5 | 5631621502 | F15, F16 | F15 from the fix of F13; F16 from the evidence added for F14 (V8) |
| 6 | 5631998994 | F17 | from the fix of F15 |

Round 1 found real defects and its six findings were all legitimate. From round 2 on, 8 of 11
findings were text the author wrote while fixing the previous round. The chains are F5, F7;
F2, F8, F11 and F9; F10, F13, F15, F17; and F14, F16.

## What Went Well

1. **Round 1 caught false statements in the delivery.** F2 and F3 were claims ("cannot be held in
   flight", "cannot leave detached tasks") that the code contradicted; both were also in the PR
   body (F12).
2. **The findings are specific and reproducible.** F17's comment, for example, gives the exact
   `git grep` that prints the line it names, so the author could confirm it with one command.
3. **The processing mechanics held.** One signed commit per fix, a reply on every thread after the
   push, the validator at 0 failures before each audit commit, zero re-raises, and no unresolved
   thread after each round.
4. **Severity was honest and gave the maintainer choices.** Five of the six human reviews
   approved with non-blocking findings and named the head, so the maintainer could have merged
   after any of them; the one `CHANGES_REQUESTED` was a real Major (F12, the PR body).

## What Made the Review Costly

1. **Prose with several copies.** The same fact was stated in the spec, the sub-EPIC, the evidence
   file, the audit, a retrospective, replies, and the PR body. A correction made in one place and
   not in the others produced F1, F6, F8, F12 and F16 (five propagation failures).
2. **Claims that went beyond what had run.** F2, F3, F5, F7, F9, F11, F13, F15 and F17 are
   statements of "cannot", "only", "finds nothing", a count, or an attribution that the author had
   not derived by a command, or derived with a command that matched itself (F17, and the F2 and F9
   verification lines). Nine of the seventeen findings.
3. **A fix is new text, and new text is new findings.** Eight findings were in the text of the
   fix to an earlier finding. Three of them (F13, F15, F17) are in the audit's own entries, the
   part meant to record the earlier fix.
4. **A new head restarts the review.** Each push dismissed the approval (five dismissals), so
   a nit in a fix costs a full review of the new head even though the reviewer had approved.
5. **The reviewer finds new things at each head.** F12 (the PR body) was stale since rounds 1 and 2
   corrected the records and was first raised in round 4; the reviewer noted in F7 that his own F5
   comment carried the wording F7 corrects. This
   is expected for a reviewer who recomputes each head, not a defect of his process, and it means the
   author cannot assume that an approved head has no more findings.

## Root Causes

1. **The author rules from the previous retrospective were prose, and prose did not hold.** The
   retrospective of PR 2553 produced three rules, now in `process-pr-review`: stamp after the event,
   state only what ran, and sweep for the same mistake. They were in the session's instructions during
   this PR. The same three failure classes appeared again: a count not derived (F11, F13), a
   verification that did not do what it said (F2, F9, F17), and fixes that created the next finding
   (F13, F15 and F17: each came from the fix of the one before). A rule that depends on the
   author remembering it at every sentence is not a control.
2. **No mechanical self-audit.** Nothing compared the added prose with the tree before the push.
   `linter all` and the pre-push script check form, not truth. Issue #2503 plans the gate and is still open.
3. **No propagation register.** When a statement was corrected, nothing listed the other places that
   held the old statement, so the sweep was a search the author composed each time, and each
   composed search could be too narrow (F13 then F15) or match itself (F17).
4. **A large self-authored prose surface.** 2460 added non-code lines against 2078 code and test
   lines. The reviewer recomputes all of it at every head, so every imprecise sentence is a round.
5. **Unverified provenance.** The author stamped "Claude Opus 5.5" in 28 progress-log entries (17 in the spec,
   11 in the sub-EPIC) by copying earlier entries; counted with `grep` for the stamp in entries dated
   2026-10-21 or later in the spec and in the sub-EPIC entries that mention #2416. The model of the session was not known from inside it, and the stamps
   claimed what could not be checked: the same class of problem as the findings, in the logs.

## Model Observation

The maintainer's hypothesis is that the findings come from the author's model (Claude Sonnet 5.5)
being less strict than the reviewer's, and that a more expensive model could cost the same in total
because it would finish in fewer rounds. This PR cannot test it:

- Only one author model is known, and no run of this PR with another model exists.
- PR 2553 had the same author model and a similar profile (6 reviews, 5 with findings, 24 findings,
  most about the author's records), which is consistent with the hypothesis but is two data points
  from one author model, not a comparison.
- The failure classes above are not specific to a model: each has a mechanical check, and a stricter
  author model would still have to remember the rule at every sentence.
- The repository holds no token or cost data per session, so "cheap model for longer" against
  "expensive model for less time" cannot be computed from it.

This is recorded as an observation. Engineering policy 7 treats provider choices as optional
adapters, so no rule names a model. To compare, run the same kind of change with each author model
and record, per run, the number of rounds to the first approval with no findings, findings per
round, the share of findings introduced by the author's fixes (8 of 17 here), and the session's
token or cost figure if the harness can report it.

## What We Learnt

1. A sentence the author did not derive by a command is a finding waiting for a reviewer; fewer,
   exact sentences cost fewer rounds than many careful ones.
2. A fix to a statement is incomplete until every other copy of the old statement is found, and the
   search must be written to find the old phrase, never to quote it.
3. A verification that quotes its own pattern matches itself; use a pattern that cannot match its own line.
4. Keep one source for each fact and link to it; every copy is a thing to keep in step.
5. Record the model from the harness, not from the previous log entry.

## Improvements for Future Reviews

List of changes with an owner artifact and a disposition. None is applied in this pull request:
each changes the review workflow and needs the maintainer's approval.

1. **Pre-push claims diff**: print the lines added to `docs` since the last reviewed head that
   contain a digit or an absolute (`only`, `never`, `cannot`, `all`, `none`, `finds`, `returns`), so
   the author derives each one before pushing — author self-audit gate, issue #2503 —
   `FOLLOW_UP_ISSUE` (that issue is open; this item is input for it).
2. **Executable verification lines**: a `Current-tree verification` that names a search or count
   is a command with its expected output, run by the validator and failed when the output differs
   (this catches the self-matching grep and the count that was never run) — `validate-audit-record.py` —
   `PROPOSED`.
3. **Stale-phrase register**: when a finding corrects a statement, record the old phrase once; a
   check searches the records and the PR body for it — `process-pr-review` skill and checker —
   `PROPOSED`.
4. **PR body consistency check**: the highest V number in the evidence file, the commit counts,
   and the single closing keyword, compared with the body — a checker run before each push —
   `PROPOSED`.
5. **A second reading of the round diff before pushing**: the `Task Reviewer` agent or another
   model reads only the lines added in this round against a fixed checklist (counts, absolutes,
   attribution, stale copies). This is where a stricter model is cheap, because the input is a
   small diff — `review-task` skill — `PROPOSED`.
6. **Fewer copies of each fact**: spec, evidence, and PR body link to the one record that
   states it instead of restating it — spec and evidence templates — `PROPOSED`.
7. **Harness-recorded model**: the model id comes from the harness (for example a commit trailer)
   and not from the author's text — harness and the progress-log rule — `PROPOSED`.
8. **Model comparison**: the experiment described above — maintainer decision — `PROPOSED`.

## Avoiding Overcorrection

- No rule naming a model, and no conclusion from one PR.
- No limit on review rounds or on fixing non-blocking findings: each fix in this PR was correct,
  and round 1 found real defects.
- No new reviewer duty: the reviewer's checks were sound; the improvements are on the author side.
- No checker for code quality: only 3 of 17 findings were about code, the branch, or retained logs.
- Not every statement needs a command: only counts, absolutes, attributions and "finds nothing" claims.

## Evidence

- Audit record: `docs/pr-reviews/pr-2623-review/PR-REVIEW.md`
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2623>
- Reviews: 5610834925 and 5631096012 (bot); 5616192898, 5618477521, 5625229542, 5630959120,
  5631621502, 5631998994 (da2ce7), from `gh api repos/torrust/torrust-tracker/pulls/2623/reviews`.
- Dismissal times: `gh api repos/torrust/torrust-tracker/issues/2623/timeline`, events
  `review_dismissed` (2026-10-22 17:16 UTC; 2026-10-23 07:12, 15:38, 17:06 and 17:45 UTC).
- Earlier precedent: `docs/pr-reviews/pr-2553-review/review-retrospective.md`.
