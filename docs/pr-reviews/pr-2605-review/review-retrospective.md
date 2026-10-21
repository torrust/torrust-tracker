---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - docs/templates/PR-REVIEW-RETROSPECTIVE.md
    - docs/pr-reviews/pr-2605-review/PR-REVIEW.md
    - docs/pr-reviews/pr-2590-review/PR-REVIEW.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR Review Retrospective — PR #2605

This retrospective covers pull request #2605 (the T5 sandbox evidence and design of #2525) and its
sibling #2610 (the T6a GitHub adapter of the same issue). Both were written by the same author in the
same session, were reviewed by the same reviewer in the same mode, and cost rounds for overlapping
reasons, so the causes are analyzed together. The audit of #2610 is on its own branch,
`docs/pr-reviews/pr-2610-review/PR-REVIEW.md`, and is cited by that path.

The maintainer asked why the author model needed so many rounds. The evidence answers that in part,
and the section "On the model hypothesis" says exactly how far.

## Purpose

Record a blameless, evidence-based review of how these pull requests were reviewed and how their
findings were processed. It complements the audit records, which track what each finding was and how
it was resolved, and explains why the process cost what it did.

## Review Summary

| Metric                                         | PR #2605                                    | PR #2610                                    |
| ---------------------------------------------- | ------------------------------------------- | ------------------------------------------- |
| Review rounds                                  | 7 (0 human, 7 bot)                          | 4 (0 human, 4 bot)                          |
| Audit findings                                 | 11                                          | 11                                          |
| Re-raised findings (`RE_RAISE_OF`)             | 5                                           | 3                                           |
| Findings about the audit record itself         | 6                                           | 0                                           |
| Commits on the branch                          | 19 before this document (7 audit-only)      | 22 (5 audit-only)                           |
| First review to last thread resolution         | 11:36 to 16:55 UTC, 5 h 19 min              | 12:33 to 16:51 UTC, 4 h 18 min              |
| Waiting for Copilot, summed over the rounds    | 233 min (mean 33, range 17 to 60)           | 119 min (mean 30, range 17 to 43)           |
| Copilot reviews that ran in degraded mode      | 7 of 7                                      | 4 of 4                                      |
| AI models used                                 | see below                                   | see below                                   |

The maintainer has been requested as a reviewer on both pull requests since the first round and has not
reviewed either, so the human count is zero.

Models: the author was Claude Haiku 5.5 until the maintainer switched to Claude Sonnet 5.5 at
17:30 local time (16:30 UTC) on 2026-10-21, as the maintainer stated in the session. Rounds 1 to 6 of
PR #2605 and rounds 1 to 3 of PR #2610 were handled before the switch, and round 7 of PR #2605 and round
4 of PR #2610 after it. The reviewer was the Copilot code review; its model is unknown. A model is
recorded here as an observation for later comparison, not as a finding.

The figures are as of 16:55 UTC. The review that was requested on #2610 at 16:51 UTC had not arrived, and no
further review was requested on #2605. See "Evidence" for how each number is derived.

### Findings by cause

This classification is mine, made after the fact, and is a judgement, not a measurement.

| Cause                                                             | PR #2605 | PR #2610 | Findings                                       |
| ----------------------------------------------------------------- | -------: | -------: | ---------------------------------------------- |
| A defect in an artifact the author wrote, and checkable by them   |        4 |        8 | #2605 F1, F2, F5, F9; #2610 F1 to F6, F10, F11 |
| A valid limit of a throwaway script, answered by documenting it   |        1 |        0 | #2605 F3                                       |
| One reviewer false positive, first raised and then re-raised      |        6 |        0 | #2605 F4, F6, F7, F8, F10, F11                 |
| A judgement about a trade-off, with a partial change              |        0 |        3 | #2610 F7, F8, F9                               |

A re-raise is counted with the cause of the finding it repeats, so #2610 F11 is with F10.

## What Went Well

1. Every finding was checked against the tree before it was answered, and not all were accepted:
   the formatting finding on the audit table (#2605 F4 and its five re-raises) was rejected with
   evidence each time, and the finding about Unix-only tests (#2610 F7) was answered with a measured
   argument and a partial change that closed the real gap, not with a blanket yes or no.
2. Code findings were fixed test-first. Each new test was seen to fail on the old behavior, and the
   fixes were mutation-checked: 25 mutants of the adapter and its fixtures were applied and all were
   killed (`manual-verification-evidence.md`, V11).
3. The process rules held. Every fix has its own signed commit, every reply followed the push it
   describes, and the audit validator passed before every audit commit. It also failed once, on a draft
   with placeholder reply ids, and that stopped the commit.
4. Some problems were found by the author before any reviewer raised them: answering #2605 F3 showed
   that no test covered the pagination of the check suites in the adapter, and a test was added to
   #2610 in that round; a failing CI check on #2610 was traced to a macOS runner that was never
   acquired, with GitHub's own message as the evidence, and rerun, not assumed to be a code problem.
5. Errors in the record were corrected openly and on the record, not left: the free-form reply to #2605
   F4 was rewritten into the prescribed form, the disposition of #2605 F3 was changed from
   `FOLLOW_UP` to `FIXED`, a wrong row count in a reply was corrected in the thread, and the cause
   recorded for #2610 F5 was corrected while preparing this document.
6. After the second answer with evidence had failed to end the false positive, no further review of
   #2605 was requested when it came back a fifth time, and the maintainer was asked to decide. This
   applies lesson 4 below, and came late.

## What Made the Review Costly

Defects in the reviewed change are separated from defects in how the findings were processed.

### In the reviewed changes

1. **Claims written without deriving them from the artifact.** The most costly pattern, because each
   instance survived until a reviewer read the file.
   - #2605 F2, round 1: a log line was cut off with an ellipsis in a file described as verbatim.
   - #2605 F9, round 6: the saved sandbox workflow was the one that ended the experiment, not the one
     that ran in runs 1 to 4. Round 2 had even added the sentence "It is saved as it ran", which was
     false, and which stood for more than three hours. Writing the fix showed that the evidence also said
     that only run 3 used the old workflow, and run 4 had too.
   - #2605 F1 and F5: wording written for one place and shipped in another ("never merged upstream" in
     files that were then stored in the repository, and a relative folder name that is ambiguous from
     the repository root).
   - #2610 F1 and F4: documentation narrower than the behavior, and an error message that named
     `gh api` although the program runs `gh api --paginate`.
   - The same pattern, earlier on the same day and in the same series of pull requests: #2590 F2 and
     F3 were a wrong count (394 where the data has 392) and a rate pooled over two classes. See
     `docs/pr-reviews/pr-2590-review/PR-REVIEW.md`. No habit that would have prevented the repetition
     was adopted after it.
2. **The cheaper option was chosen on a finding that offered two.** #2610 F1 (round 1) pointed at the
   case where the events of a pull request cannot be read and offered to fix the implementation or to
   broaden the documentation. The documentation was broadened. In round 2, #2610 F5 asked for the
   implementation fix, so one defect cost two rounds. The cause was first recorded as a defect of the
   previous round's change, which was wrong; the unconditional read was in the first version of the
   adapter.
3. **Negative paths and helpers without a test, found one property per round.** The stand-in for `gh` in
   the end-to-end tests produced four findings in three of the four rounds of #2610, each a different
   property of the same helper: its permission path (F3, round 1), its directory name being shared
   (F6, round 2), and the same name going unchanged into a directory that is removed recursively (F10 and
   F11, round 4). One review of the helper's inputs, paths, uniqueness, and cleanup would have found
   them together. In F3 the helper took the actor but kept the permission path of one login: the edit that
   was meant to make the actor a parameter did not apply, because the formatter had wrapped the line and
   the string replacement matched nothing, and the tests still passed because every test used that
   login. Separately, #2610 F5 had no test for a missing label with an unreadable events request.
4. **A trade-off raised three times.** #2610 F7, F8, and F9 are one concern, that the end-to-end tests
   of the new command exist only on Unix, raised on three lines. It is a real trade-off and was answered
   as one, but it produced three threads and three audit rows.

### In how the findings were processed

1. **A false positive was answered, not escalated.** #2605 F4 claimed that the rows of the audit table
   start with a doubled leading pipe. They do not: the file fetched from the GitHub contents API has no
   such line, and its table rows all start with a single pipe. The claim came back in rounds 4, 5, 6, and 7
   (F6, F7, F8, F10, F11). Rounds 4, 5, and 7 contained nothing else, so three of the seven rounds, about
   120 minutes of waiting, were spent on it, and it was in five of seven rounds. The maintainer was first
   asked whether to dismiss it after the sixth, when the second answer with evidence had already failed.
2. **The first reply to the false positive was not in the prescribed form.** The skill asks for
   "Superseded by <FindingId>: <reason>." and the first reply was free-form. It was edited later. There
   is no evidence that the form would have changed the reviewer, but the rule was not followed.
3. **Small errors in the record, all caught before or soon after publishing.** Log times later than the
   clock three times in one audit, caught by hand; a wrong row count ("12" for 8) in a reply, corrected
   in the thread; and a disposition that did not match the skill. They did not cost a round, but each
   cost a commit or an edit.
4. **A half-finished sequence.** One long command was interrupted after the commit and before the push,
   the body update, the resolve, and the re-request. The state was re-checked before repeating anything,
   which avoided duplicates, but the loop stalled for the time it took to notice.
5. **Watchers that fired on the author's own threads.** Twice a wait script reported unresolved threads
   that had just been resolved. They cost attention and no round.

## Root Causes

System causes, ordered by how many findings they explain.

1. **No step that derives a claim from its source.** A number, "verbatim", and "as it ran" were typed
   from memory or from an earlier state, and the only check was a reviewer. This explains #2605 F2, F9
   and the 12-versus-8 slip, #2610 F1 and F4, and #2590 F2 and F3. The pattern was visible after #2590
   and nothing in the workflow made the lesson stick.
2. **Choosing between two fixes without asking which matches the intended behavior.** The skill says to
   process every finding but does not say how to choose when the reviewer offers both a code change and
   a documentation change. The default that costs less now is the documentation, and it can be the wrong
   one (#2610 F1 to F5).
3. **Edits by string replacement that can silently do nothing.** The scripts asserted that the old text
   was found in most cases, and failed loudly then, which is why most slips cost nothing. The helper
   edit in #2610 F3 had no such assertion.
4. **No threshold for escalating a repeated, already-answered finding.** The skill has a form for
   superseded findings and a rule to process every thread, and no rule for what to do when the same
   finding returns after an answer with evidence. Each re-request after a push starts another review of
   the latest tree, and a review that finds the same thing again looks like progress to the loop.
5. **The audit record is part of the reviewed change.** Each audit commit is a new surface to review, and
   six of the eleven findings of #2605 were about the audit's own table. The explanatory text of the
   rejected finding quoted the characters the reviewer had misread, and F8 of the fifth review and
   F10 of the sixth argue against that text. It is a hypothesis, not a proven cause, that quoting the
   characters primed the next review.
6. **The reviewer ran in degraded mode every time.** All eleven Copilot reviews say that the full agentic
   review could not run, and they arrive 17 to 60 minutes after the request. Findings are therefore
   spread over several rounds instead of arriving at once: #2605 had real findings in rounds 1, 2, 3,
   and 6, and #2610 in all four rounds. Nothing the author does changes this, and it multiplies the cost of every
   other cause.
7. **Two review loops at once.** #2605 and #2610 were worked in alternation, with a branch switch each
   time. One command ran against the wrong branch and found its file missing, and a watcher reported
   threads that had just been resolved. Neither cost a round, and the interrupted command described
   above was interrupted by the tool, not by the alternation.

### On the model hypothesis

The maintainer's hypothesis is that the author model lacks the discipline and organization that pull
request reviews require. The evidence supports part of it and does not support a conclusion.

- In favor: 12 of the 22 findings were defects in artifacts the author wrote and could check, and
  they share the same shape: a claim not derived, a fix chosen for cost, an edit not verified, a helper
  reviewed one property at a time. The slips in the record (future log times, the wrong count, the
  free-form reply, the wrong cause for F5) are the same kind.
- Against a conclusion: 6 of the 22 findings are one reviewer false positive that no author action
  prevented, and every review was degraded, which alone spreads findings over rounds. The two pull
  requests are documentation- and evidence-heavy, new work, written in one day, which raises the
  number of checkable claims per line. The sample is two pull requests, one author model for most of
  the rounds, and one reviewer mode, so model and task and process cannot be separated.
- After the switch there are two rounds, which is too few to say anything. #2610's fourth review had one
  valid finding, fixed test-first with a mutation check in one round; #2605's seventh was the same false
  positive and was answered without requesting another review. Small slips continued: a British spelling
  in a comment that the spell check caught, placeholder reply ids left in an audit until the validator
  flagged them, and a mutation script whose text had been reformatted and which stopped on its own
  assertion. None of them cost a round.
- What would settle it: the same kind of pull request, reviewed in the same way, with each finding
  classified by cause as in the table above, for each model. The rounds from the maintainer's switch
  at 16:30 UTC onwards are the start of such a comparison, and neither author model is a neutral judge
  of it.

## What We Learnt

1. A sentence that says "verbatim", "as it ran", or gives a count is a claim about a command's output.
   Paste the output of the command; do not retype it.
2. When a finding offers a code fix or a documentation fix, decide by the behavior the code should have,
   write the reason in the audit, and expect the other option to come back if the choice was only
   cheaper.
3. After a scripted edit, search for the old text and confirm that it is gone, and let a test fail on the
   old behavior before the fix is accepted, including for test helpers.
4. A finding that is wrong costs rounds only if the loop keeps asking the reviewer. After the second
   answer with evidence, ask the maintainer to decide.
5. Describe a rejected finding in words, not by quoting what the reviewer misread, in a file that the
   reviewer will read again. This is a hypothesis to test, not a rule yet.
6. Re-requesting a review has a cost of 17 to 60 minutes. A round should end with everything the author
   already knows to be open closed, so that the next review is not spent on it.

## Improvements for Future Reviews

All are `PROPOSED`: the template asks that substantial workflow changes are not applied inside the
reviewed pull request without maintainer approval.

1. Add a pre-push self-check to the review loop: every number, "verbatim", and "as it ran" in a changed
   document is produced by a command whose output is pasted, and every scripted edit is followed by a
   search for the old text — `process-pr-review` skill — PROPOSED
2. When a finding offers a code change or a documentation change, record in the audit which was chosen
   and the behavior that decided it — `process-pr-review` skill and `PR-REVIEW-TEMPLATE.md` — PROPOSED
3. Add an escalation rule: when a finding has been re-raised twice after answers with evidence, ask the
   maintainer whether to resolve it as a false positive instead of requesting another review, and record
   the decision — `process-pr-review` skill — PROPOSED
4. Try describing a rejected formatting finding in words in the audit, and measure whether the next
   review re-raises it — `process-pr-review` skill, as an experiment on the next occurrence — PROPOSED
5. Record in the audit header the author model and whether the reviews ran in degraded mode, so that a
   model comparison can use data and not recollection — `PR-REVIEW-TEMPLATE.md` — PROPOSED

## Avoiding Overcorrection

The evidence does not justify the following.

- A per-finding "cause" field in every audit. The classification above is a judgement made afterwards,
  and a field would make it look like a measurement.
- Moving the audit out of the reviewed pull request. It is the traceability record, and the cost was
  one false positive, not the audit as such.
- A checker for numbers in documents. Wrong numbers explain two findings of #2590 and one slip in a reply
  here, and a habit of pasting the output of the command that produced them is cheaper than a tool.
- Stopping the re-request of reviews. They found 12 valid defects or limits (#2605 F1, F2, F3, F5, F9;
  #2610 F1 to F6, F10), each in an artifact that had passed the repository's pre-commit and pre-push checks.
- Cross-platform end-to-end tests of the adapter. The pull-request workflows test on Linux only, and the
  real gap, the untested parsing of the output of `gh`, was closed with portable unit tests.
- Any conclusion about a model from this sample.

## Evidence

- Audit record: `docs/pr-reviews/pr-2605-review/PR-REVIEW.md`; for #2610,
  `docs/pr-reviews/pr-2610-review/PR-REVIEW.md` on its branch.
- Pull requests: <https://github.com/torrust/torrust-tracker/pull/2605> and
  <https://github.com/torrust/torrust-tracker/pull/2610>.
- Rounds, degraded mode, and thread counts: `gh api repos/torrust/torrust-tracker/pulls/<n>/reviews` (a
  review is degraded when its body says that the full agentic review or suite could not run) and the
  GraphQL `reviewThreads` with `pullRequestReview.databaseId`.
- Waiting time: the `review_requested` events whose reviewer is `Copilot` in
  `gh api repos/torrust/torrust-tracker/issues/<n>/timeline`, against the `submitted_at` of the next
  Copilot review, pairing each review with the earliest request that it had not yet answered, because a
  request made while Copilot was already working on the previous one belongs to the next review. #2605: 37,
  42, 17, 60, 43, 17, and 17 minutes. #2610: 43, 17, 17, and 43 minutes.
- Findings, re-raises, and the audit-record count: the tables of the two audits.
- Commits: `git rev-list --count torrust/develop..josecelano/2525-t5-design` (19 before this document, of
  which 7 start with `docs(pr-reviews)`) and, for #2610, the commits after the stacking point `7c230e2ec`
  on `josecelano/2525-ci-gate-github-adapter` (22, of which 5 start with `docs(pr-reviews)`).
- The macOS failure of #2610: the annotation "The job was not acquired by Runner of type hosted even
  after multiple attempts" on check run 119328209761, and the same workflow passing on the earlier head.
- The origin of #2610 F5: the first adapter commit `5bbed54f2`, whose `assess_override` already reads the
  events before looking for the label.
- The "Findings by cause" table and the section on the model hypothesis are the author's judgement.
