---
semantic-links:
  skill-links:
    - process-pr-review
    - wait-for-pr-review
    - open-pull-request
    - create-issue
  related-artifacts:
    - docs/issues/open/2853-container-build-jobs-limit/ISSUE.md
    - docs/pr-reviews/pr-2854-review/PR-REVIEW.md
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md
    - .github/skills/dev/git-workflow/open-pull-request/SKILL.md
---

# Implementation Retrospective — Review of Pull Request #2854 (Issue #2853)

## Purpose

Record what made the review of pull request #2854 slow, and propose changes so that the next
review spends its rounds on the change and not on the records about the change. This is a blameless
review of the process. The author of the pull request was an AI agent (Claude Sonnet 5.5 for the
work and the first five review rounds, Claude Fable 5.2 for the last round and this document).

All counts below were taken on 2026-11-13 from `git log`, the GitHub API and the audit record
`docs/pr-reviews/pr-2854-review/PR-REVIEW.md`, before the commit that adds this file. Where a
number depends on my own classification, the text says so.

## Outcome

The pull request caps the Cargo jobs of the container image build with `CARGO_BUILD_JOBS`,
default 4. The shipped change is 61 changed lines in three files (`Containerfile`,
`contrib/dev-tools/containers/docker-build.sh`, `docs/containers.md`). The pull request as a whole
adds 1,816 lines in 14 files: the rest is the experiment (script, report and 832 lines of raw
samples), the issue specification, the evidence file and the 454-line review audit record.

The review took nine reviews (six by `da2ce7`, three by the automated reviewer) and eight pushes
after the pull request was opened. It produced 31 findings, all answered and resolved. From the
first review (10:20 UTC) to the approving review (11:55 UTC, which GitHub now shows as dismissed) it took 95 minutes. The last thread
was resolved at 13:21 UTC.

## What Went Well

1. The reviewer recomputed every figure from the raw samples. That is how the wrong per-process
   memory figure (F4) and the overwritten run summaries (F14) were found.
2. The measurement was safe. Builds ran one at a time with a memory watchdog, and the uncapped
   build was never run on the machine it had already crashed twice.
3. `validate-audit-record.py` caught every format error in the audit record (row shape, reply
   URLs, review ids, commit subjects) before the reviewer had to.
4. Severity labels were accurate. The reviewer marked the late findings Nit and wrote "no finding
   blocks", so the state of the pull request was never unclear.

## What Changed During Implementation

### Where the findings landed

Classification by me, from each finding's source thread and text:

| Target of the finding                                                    | Findings | Share |
| ------------------------------------------------------------------------ | -------- | ----- |
| The shipped change (`Containerfile`, `docker-build.sh`, `containers.md`) | 5        | 16%   |
| The experiment script (`measure.sh`)                                     | 3        | 10%   |
| The experiment report and raw data                                       | 10       | 32%   |
| The issue specification and the evidence file                            | 8        | 26%   |
| The review audit record and the review process itself                    | 5        | 16%   |

Two more views of the same 31 findings:

- By my classification, 16 were about content in the pull request as first opened. 15 were about content I wrote during
  the review: fixes, rewrites and the audit record.
- By severity (from the audit record plus the last finding): 4 Major, 16 Minor, 11 Nit.

So the review worked on the 61 shipped lines for 5 findings and on the supporting records for 26,
and almost half of all findings were created by the review rounds themselves.

### Frictions

Each friction lists what happened and the evidence.

1. **The review-processing skill was not started.** I replied to the first two threads without
   creating the audit record that `process-pr-review` requires before the first reply. The task
   brief and the maintainer's session instructions named `create-issue`, `open-pull-request` and the
   `wait-for-pr-review` poller, and I did not look for a skill about processing reviews. Finding
   F12; the record was back-filled one round late.

2. **Acting on one event instead of the whole review.** The poller returns on the first new review
   or comment. I fixed that one item and pushed. A review's other inline comments were in the same
   review but I had not listed them.
   - The review with F14 also held F15 to F20. I fixed and pushed F14 alone, then needed another
     round for the other five.
   - An automated review (R3, R4) stayed unanswered through two pushes because no step listed all
     open threads.

3. **Too many pushes.** Eight pushes for six reviews by the human reviewer. One review round alone
   took three pushes. Each push needs the maintainer to approve the workflow runs, and each push
   moves the head under a reviewer who is still reading. The container workflow completed on only
   three of the eight pushed heads; four have not been approved and one was cancelled. Two automated reviews examined a commit that was no longer the head when they
   were submitted.

4. **Order of operations: reply before commit, log before event.** The skill says to stamp an event
   only after it happened. I broke that three times:
   - The first audit commit (10:52 UTC) contained a log entry saying the replies were posted. They
     were posted at 10:54 UTC.
   - Eleven replies were posted at 11:24 UTC for a commit made at 11:29 UTC. They could not cite its
     subject, so they said "the commit addressing this review round".
   - Three replies were posted at 11:42 UTC for a commit made after them.

5. **Hand-typed facts that tools already hold.** Timestamps, commit references, reply URLs and
   thread states were typed into the audit record and the spec's progress log by hand. Every one of
   these is available from `git log` or the GitHub API. Findings F8, F16, F19, F20, F26 and the
   last one (F28 in the audit record) are all about a hand-typed fact that was wrong: a time, a
   commit id where a subject was required, or the wrong commit named for a fix.

6. **"FIXED" recorded without running the check.** Three audit rows said FIXED with a verification
   line that the tree contradicted (F26). The commit-id check that would have caught one of them is
   documented in `open-pull-request`. I did not run it until the sixth round.

7. **Finding ids collided.** The reviewer numbers findings in each review. The audit record also
   assigns numbers to findings that arrive without one. Twice the same number meant two different
   findings (F21 to F23, then F25), and each collision cost a renumbering step and audit text to
   explain it.

8. **Fixes were rewrites, and rewrites made new claims.** To fix one sentence I often rewrote the
   paragraph and added explanation. The new sentences contained new unverified claims: a cause for
   a process-count spike, where `cargo binstall` downloads from, a runner measurement that did not
   exist yet (F15, F17, F25, F27, R3, R4).

9. **Raw data was edited.** To fix an arithmetic finding I regenerated the three run summary files
   in a new format with a one-off script. The script lost precision and wrote 32.9 where the run had
   printed 33.0 (F14). Raw output of a run must never be rewritten.

10. **The round limit and the stop rule were unclear.** The maintainer's rule is "5 rounds of fixes,
    then stop and ask". I stopped at the limit without first reading the newest review, which was an
    approval, and reported the remaining findings as the reviewer's problem. The rule does not say
    what a round is, and nothing says what to do when the only findings left are Nits about records.

11. **A small change carried the full weight of the specification process.** The spec template, the
    evidence file, the progress log, the experiment report and the audit record together are 29
    times the size of the shipped change, and they attracted 84% of the findings. The process does
    not have a lighter form for a small change.

12. **A failing check on the base branch.** `Test (Docker) (release)` fails on `develop` at the
    merge base in the step after the image build. The pull request cannot go green, and I reported
    the cause as "known and unrelated" before I had checked it.

## Root Cause

The process asks the author to keep several prose records in agreement with each other and with
facts that live in git and on GitHub: the spec's progress log, the evidence file, the audit
record's table, its detail entries and its processing log, the replies and the pull request
description. Each record repeats facts the others hold. Keeping them in agreement is done by hand,
under time pressure, between review rounds. Every hand copy is a place for an error, and each error
is a legitimate finding, which is answered with more prose in the same records.

The skills already state the right rules (stamp after the event, cite the subject, state only what
ran, sweep for the same mistake). I broke rules that were written down. More written rules would
not have helped. What was missing is a fixed sequence of commands that makes the wrong order
impossible and takes the facts from their source.

## Improvements for Future Work

Ordered by how much of this review each would have removed. Items 1 to 4 are the deterministic
process the maintainer asked for. They need tooling in the Tracker repository (the
`wait-for-pr-review` and `github-review-threads` tools are the natural home) and a short update of
the `process-pr-review` skill.

1. **One command per round, with a fixed order.** Add a `review-round` tool with these subcommands,
   and make the skill say "run them in this order and nothing else":
   1. `review-round start --pr N`: fetch every review and every thread, and write a work list file
      with one line per unresolved thread and per review-body finding. It refuses to run while a
      requested reviewer's review is still pending for the current head. This removes friction 2.
   2. The author fixes everything on the work list and commits. No push yet.
   3. `review-round check`: run `linter all`, the commit-id check, and the audit validator. It
      fails if any work list item has no commit assigned. This removes friction 6.
   4. `review-round publish`: push once, then post one reply per thread from a template, then
      resolve the threads, then confirm zero unresolved threads. The reply text takes the commit
      subject from `git log`, so a reply cannot exist before its commit and cannot cite an id. This
      removes frictions 3 and 4.

2. **Generate the audit record; do not write it.** Replace the hand-written table, the reply URLs,
   the thread states and the Processing Log with output generated from the GitHub API and `git
   log`. The author writes one thing per finding in the work list: a disposition and a one-line
   solution. `review-round publish` writes the record. With no hand-typed time, id or URL, the
   findings in friction 5 cannot occur. This is the single largest saving: 5 findings were about
   the record itself and it is the largest file in the pull request.

3. **Take finding ids from GitHub.** Use the review comment id as the finding id in the audit
   record (for example `c4562180428`). Reviewers may keep their own labels in the comment text. No
   two findings can share an id, so the collision rule and friction 7 disappear.

4. **No hand-typed times in specifications.** In the issue template, replace the timed Progress Log
   with a list of commit subjects (which `git log` already orders and dates), and set
   `last-updated-utc` from a script or drop it. This removes the class of finding in F8.

5. **A fix changes the least text that resolves the finding.** Add one rule to `process-pr-review`:
   when a finding is about a wrong sentence, delete or correct that sentence; do not add
   explanation that was not asked for. Every added sentence is a new claim to verify. This targets
   friction 8, which produced six findings.

6. **Raw run output is read-only; derived figures come from a committed script.** In the experiment
   convention (`contrib/dev-tools/experiments/`), keep raw output under `results/` and never edit
   it after the run. Put every figure the report states in a script that reads `results/` and
   prints the table, so the author and the reviewer run the same computation. This removes
   friction 9 and the arithmetic findings (F4, F14, F25).

7. **A stop rule for the last mile.** Define a round as one verdict from the human reviewer. When
   that verdict says no finding blocks and only Nits about records remain, the author fixes them in
   one final push and the reviewer does not review again, or the maintainer defers them to the
   close-out. Before stopping at the round limit, the author must read the newest review. This
   removes friction 10.

8. **A small-change profile for specifications.** For a change under an agreed size (for example
   under 100 changed lines with no new behavior to design), allow a spec with Goal, Scope,
   Acceptance Criteria and Verification only: no progress log, no separate evidence file, and
   measured evidence attached to the issue instead of committed to the tree. Consider a separate
   pull request for an experiment and its report, so the fix can merge while the report is still
   being corrected. This targets friction 11.

9. **Name the skill where the agent will read it.** In the maintainer's session instructions, the
   paragraph on resolving reviews should name `process-pr-review` and say it starts before the
   first reply. The poller's result could print the same line. This removes friction 1.

10. **Check the base branch before blaming it.** When a check fails, `review-round start` should
    report whether the same job fails on the base branch at the merge base. The author then states
    "fails on `develop` too, run <id>" or investigates. This removes friction 12.

## Avoiding Overcorrection

- Do not add more prose rules about accuracy to the skills. The rules exist and were clear. The
  failures were of sequence and of copying, which tooling fixes and prose does not.
- Do not drop the audit record. A generated record still gives the aggregate data its purpose
  section describes (categories, recurring errors) at no cost to the author.
- Do not relax the reviewer's checks. Recomputing figures and checking claims against the tree
  found real errors, including the two Major findings about a false "done" and overwritten data.
- Do not conclude from this one pull request that experiments never belong in the tree. The cost
  here came from mixing the experiment with the fix in one review, not from keeping the data.

## Evidence

- Issue specification: `docs/issues/open/2853-container-build-jobs-limit/ISSUE.md`
- Review audit record with all 31 findings: `docs/pr-reviews/pr-2854-review/PR-REVIEW.md`
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2854>
- Commands used for the counts:
  - `gh api --paginate 'repos/torrust/torrust-tracker/pulls/2854/reviews?per_page=100'` (reviews,
    their authors, states, times and reviewed commits)
  - `gh api --paginate 'repos/torrust/torrust-tracker/pulls/2854/comments?per_page=100'` (threads
    and reply times)
  - `gh api --paginate 'repos/torrust/torrust-tracker/actions/runs?branch=2853-container-build-jobs-limit&per_page=100'`
    (workflow runs per pushed head)
  - `git log --reverse --format='%cI %s' torrust/develop..HEAD` (commits and their times)
  - `git diff --stat torrust/develop...HEAD` (lines per file)
