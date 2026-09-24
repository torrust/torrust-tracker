---
semantic-links:
  skill-links:
    - process-pr-review
    - open-pull-request
    - fix-bug
  related-artifacts:
      - docs/issues/closed/2314-preserve-udp-scrape-response-order/ISSUE.md
      - docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md
      - docs/templates/PR-REVIEW-RETROSPECTIVE.md
      - docs/templates/MANUAL-VERIFICATION-EVIDENCE.md
      - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
      - .github/skills/dev/git-workflow/open-pull-request/SKILL.md
      - .github/skills/dev/debugging/fix-bug/SKILL.md
      - contrib/dev-tools/git/github-merge.py
---

# PR Review Retrospective — PR #2320

## Purpose

Record why a two-file production fix with three regression tests needed seven human review rounds
after a Copilot round, why an approval was lost to a rebase whose `range-diff` was
byte-identical, and what to change so that rebasing an open pull request stops restarting its
review. This is a blameless review of the process, not of the reviewer or the author.

The `PR-REVIEW.md` audit record beside this file was first committed after round 4, once every
thread then open had been replied to and resolved through GitHub directly; the `process-pr-review`
workflow was not used while the rounds were in progress (cost item 8). The metrics below are derived
from the GitHub API and `git log`, and the audit record was built from the same data. This document
uses the reviewer's finding IDs (`F1`-`F17`); the audit record reassigns them to `F5`-`F21` because
the four Copilot findings, which came first, occupy `F1`-`F4` there.

Current as of round 7 (review `5296377033`, 2026-09-23 20:24 UTC). A round submitted after that
is not reflected in the counts, the timeline, or the finding list below; the audit record's
Processing Log is the place that tracks later rounds. Corrections that round 8 (review
`5297189747`) requested to existing text are applied in place; round 8 itself is not counted.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 8 (7 human, 1 Copilot) |
| Findings | 21 (4 Copilot, 17 human: 12 Minor, 1 Suggestion, 4 Nit) |
| Blocking findings | 10 (F1, F2 round 1; F9 round 3; F10 round 4; F11, F12 round 5; F13-F16 round 6) |
| Re-raised findings | 6 (F9 reintroduces the F1/F2 class; F11 restated in its thread at 19:47 after a push that left it unchanged; F13-F16 re-raised as still open in round 7 after a push that did not address them) |
| Human findings about issue-local evidence documents | 8 of 17 (F3 manifest comment; F10 skill rule; F11-F17 this folder) |
| Human findings about process changes made during the review | 8 (F10 on the rule added for F9; F11-F17 on this retrospective and the audit record) |
| Findings processed through `process-pr-review` while the review was open | 0 of 21 |
| Human findings about production code or tests | 0 |
| Commits on the branch | `git rev-list --count torrust/develop..HEAD` returned 15 at `docs(pr-reviews): source #2320 retrospective times and counts from git and the API` (4 original, 1 CI fix, 5 review-response, 5 retrospective or audit); later audit-record commits are not counted here |
| Rebases while the PR was open | 2 (one before the first human review, one between rounds 2 and 3) |
| Approval lifetime | 1 h 24 min (approved 16:00 UTC, changes requested 17:24 UTC on a pure rebase) |
| First review to last author reply | 2026-09-23 13:09 UTC to the round-7 reply (see Timeline) |

Derivation: review rounds, states, and timestamps from the pull request `reviews` GraphQL
connection; finding count and severities from the first comment of each `reviewThreads` node
(`[Severity][F<k>]` prefix for human findings); commit counts from
`git log torrust/develop..HEAD`; rebase times from the committer dates in that log. Round 3's
review body states `git range-diff` marked all seven patches `=`.

## Timeline

All times UTC, 2026-09-23. Head ids are quoted from the review headers and are pre-rebase ids
that later rebases rewrote; they resolve by URL, not from `develop` history.

| Time | Event |
| ---- | ----- |
| 12:51 | PR opened from a branch based on `e03b22f7` (the #2315 spec merge). |
| 13:06 | Author rebases onto `ef234623` and force-pushes; local pre-rebase ids are orphaned. |
| 13:09 | Copilot review, 4 findings on the reviewed head (bench cookie window x2, key naming, `u8` literals). |
| 13:25 | Author fixes all four in `fix(udp-server): address scrape review feedback`. |
| ~14:20 | Container workflow fails: `cargo chef cook` cannot find the new `[[bench]]` target's stub file. |
| 14:33 | Author adds the stub in `fix(container): stub UDP server benchmark target`. |
| 15:16 | Round 1 (human), `CHANGES_REQUESTED` at head `d8c82bec`: F1-F7. F1, F2 block. |
| 15:44 | Author fixes F1-F7 in `docs(issues): address #2320 review findings on scrape evidence`; replies at 15:47. |
| 16:00 | Round 2 (human), `APPROVED` at head `ae66bb69`: all seven addressed; F8 Nit advisory. |
| 16:00-17:08 | PR #2321 merges into `develop` (`a110200d`). |
| 17:08 | Author rebases onto `a110200d` and force-pushes; every branch id changes. |
| 17:24 | Round 3 (human), `CHANGES_REQUESTED` at head `305ddce1`: F9 blocks. The rebase itself is verified pure. |
| 17:49 | Author fixes F8, F9 and adds the process rule in two commits; replies at 17:52. |
| 18:02 | Round 4 (human), `CHANGES_REQUESTED` at head `7e426ba2`: F8, F9 addressed; F10 blocks — the new pre-push grep returns 20 hits on this PR's own folder and its disposition list covers 10. |
| after 18:02 | Author resolves the nine human threads F1-F9. The request came from the maintainer in the editor chat session, not in any GitHub comment; the round-2 body had said the opposite. |
| 18:08 | Author commits this retrospective (first version, written before reading round 4). |
| 18:13-18:15 | Author fixes F10 (`docs(skills): make the branch-id pre-push check clear its reference case`, 18:13), updates this retrospective (18:13), replies on F10 (18:15) and resolves it. |
| 18:26 | Round 5 (human), `CHANGES_REQUESTED` at head `c84a224a`: F10 addressed; F11 (three timeline rows contradicted by git and API timestamps) and F12 (commit count one short) block. |
| 18:27 | Author posts the replies the four Copilot threads never had, while creating `PR-REVIEW.md`. |
| 18:32 | Author commits `PR-REVIEW.md` and the retrospective update recording the skipped workflow. |
| 19:43 | Author commits the F11, F12 fix to this retrospective (`docs(pr-reviews): source #2320 retrospective times and counts from git and the API`); the first signing attempt at 19:39 failed on an expired GPG agent cache and was retried at the maintainer's instruction, given in the editor chat session and not in any GitHub comment. Replies at 19:49. The row was first written as "18:40", the editing time, and corrected before the head shipped. |
| 19:46 | Round 6 (human), `CHANGES_REQUESTED` at head `a1386c2f`: F13 (audit record claimed all threads resolved when two were open), F14 (two Processing Log entries wrong), F15 (this summary one round stale again), F16 (root cause 7 quoted the author's own words as the reviewer's). Not read by the author before the next push. |
| 20:12 | Author commits `docs(pr-reviews): record round 5 in the #2320 audit and retrospective` and pushes; F11, F12 resolved. F13-F16 untouched. |
| 20:24 | Round 7 (human), `CHANGES_REQUESTED` at head `69fc9030`: F13-F16 re-raised in their threads as still open; F17 (Nit) on the "commit that writes this row" anchor. |
| 21:25-21:27 | Author replies on F13-F17 (21:25), then commits their fix in both files (`docs(pr-reviews): record rounds 6 and 7 in the #2320 audit and retrospective`, 21:27), then resolves them (the resolution order is from the editor session; GitHub records no resolution time); the replies cite a subject that did not yet exist. The row was first written as "21:20", the editing time, and corrected after round 8. |

## What Went Well

1. The production change was never in question. The reviewer verified the fix, ran a mutant
   with only the `build_response` loop reverted, and confirmed it fails exactly the three new
   tests in every round; the fix commit was byte-identical from round 1 to the end.
2. Each human finding named the command, line, and commit that contradicted the text, so no
   round was spent disputing whether a defect existed, and each was fixed in one pass.
3. Round 3 was explicit that the rebase was pure (`range-diff` all `=`) and that the reviewer's
   own round-1 remedy had caused F9, which made the durable fix obvious.
4. Rebases were conflict-free every time, and the branch was on the `develop` tip at every
   review.
5. The Copilot round caught one real robustness issue (a strict connection cookie with a
   ±60 s window in a benchmark whose duration is configurable).

## What Made the Review Costly

1. **Evidence written with rebase-fragile identifiers (F1, F2, F9).** The evidence files named
   the artifact under test by local branch commit id and by "working tree". The pre-review rebase
   orphaned the V2 id (F2) and left the P1 row naming a `develop` commit that could not contain
   the benchmark (F1). The round-2 fix replaced them with the then-current branch ids, exactly as
   the reviewer's option A suggested; the round-3 rebase orphaned those too (F9). Three of the
   seventeen human findings, including the three blocking ones of rounds 1-3, are this one defect.
2. **An approval destroyed by an unrelated merge.** The PR was approved at 16:00. Because
   another PR merged first, the branch had to be rebased before the maintainer merge tool would
   accept it, and the rebased head had no approval. The reviewer performed a full byte-level
   re-derivation of a range whose `range-diff` was `=`, and it found F9. Without F9 the round
   would still have been necessary to re-approve the new head.
3. **Spec-versus-evidence drift found only in review (F4, F5, F7).** The end-to-end load test
   used the generic `docs/benchmarking.md` mix instead of the scrape-heavy mix the spec
   mandated, and the deviation was unrecorded; P1 and P2 results were committed together
   against the spec's own Commit Points; a templated checkpoint was narrowed. Each was a
   one-line fix, but none was caught before the PR opened.
4. **Container build failure after the PR opened.** Adding a `[[bench]]` target requires a stub
   line in the `Containerfile` recipe stage. Nothing local checks this; the pre-push hook does
   not build the container (and a local build is not feasible on every developer machine). The
   failure surfaced only in hosted CI, costing one commit and one CI cycle. This is the recurring
   fault tracked by issue #2298.
5. **Thread ownership was ambiguous in practice.** The author replied to each thread but left
   resolution to the reviewer. The reviewer's round-2 body treated that as correct; the
   repository skill says the author resolves after replying. The nine threads stayed open across
   three rounds until the maintainer asked for them to be resolved (in the editor chat session; see
   the `after 18:02` timeline row).
6. **A dependency pin without a written reason (F3).** `criterion 0.5.1` was chosen to match the
   two sibling bench packages while `0.8.2` is already in the lockfile; the policy requires the
   reason be written down, and the `add-rust-dependency` skill was not consulted because the
   dependency was "the same as next door".
7. **A process rule shipped without being run on its own reference case (F10).** The pre-push
   grep added for F9 was run once by the author with a *different* command (a classifying loop)
   and the skill text was written from memory of that result. Run as written, it returned 20
   hits on the #2314 folder: `[0-9a-f]{7,40}` matches seven-digit decimal config values, and the
   two test info-hashes and a recorded transaction id had no disposition. The reviewer's point
   that an info-hash and a git id are indistinguishable by pattern constrained the fix to a
   disposition clause plus a decimal filter, not a tighter regex.
8. **The repository's review-processing workflow was not used.** The first request was "address
   Copilot's suggestions"; the author read a third-party editor skill (`address-pr-comments`) and
   worked the GitHub threads directly, then kept that approach through four human rounds. The
   repository's own `process-pr-review` skill, which AGENTS.md Engineering Policy 8 says governs
   when the two conflict, makes the audit record the first step and the `Copilot Suggestions
   Handler` agent exists to run it. Consequences: the four Copilot threads were resolved without
   replies; F1-F10 had no normalized rows until after round 4; the first version of this
   retrospective contained a bullet arguing a backfilled audit was unnecessary, which contradicted
   `docs/pr-reviews/README.md`; and the record, once written, had to reassign the reviewer's IDs
   because the Copilot findings had never been numbered.
9. **This retrospective repeated the defect it describes (F11, F12).** Its first two versions
   wrote three timeline times and a commit count from recollection: the retrospective commit was
   dated 18:16 when `git log` says 18:08, two events were placed "after 18:16" when all had
   happened by 18:15, the thread resolution was placed after 17:52 when the round-4 body at
   18:02 shows the threads still open, and the commit count omitted the commit that wrote it.
   The `#2271` retrospective's lesson 4 ("record log events from `git log --format=%cI` and the
   GitHub `created_at` fields, not from recollection") was cited in this file and not applied
   to it. The "at the maintainer's request" clause was true but had no cited source: the request was made
   in the editor chat, which the GitHub record cannot show.
10. **Pushing without re-reading the PR (F13-F16 re-raised).** Round 6 landed at 19:46 while the
    author was fixing round 5; the author replied, committed and pushed at 19:49-20:12 without
    fetching threads again, so round 7 re-raised all four as still open and added F17. The
    `process-pr-review` skill's step 9 ("refresh GraphQL thread data and show no unresolved
    actionable thread") is the check that was skipped, at the one moment it mattered. Two files in
    one folder disagreed about one event because only one was corrected. F16 was a misattribution
    of the author's own words to the reviewer, the kind of error that a re-read of the cited
    review would have caught before commit.

## Root Causes

1. **The "no branch SHAs" rule existed only on the reviewer side.** `process-pr-review` and
   `PR-REVIEW-TEMPLATE.md` both say to cite a fix by its Conventional Commit subject, never by a
   branch SHA. Nothing in the evidence template, the `fix-bug` evidence requirements, or the
   `open-pull-request` rebase section said the same to the author writing evidence, so the author
   and the reviewer's suggested remedy both walked into it.
2. **The merge tool couples "up to date" with "reviewed".** `contrib/dev-tools/git/github-merge.py`
   exits with `EXIT_STALE_MERGE_BASE` unless GitHub's merge base equals the current `develop`
   tip, so every merge of any other PR forces a rebase of every open PR. It then matches ACKs by
   the abbreviated head commit id, so a rebase invalidates every ACK by construction. Together
   these make "approved" a state that any unrelated merge can revoke, and a review process that
   re-derives everything per head turns each revocation into a full round.
3. **No author-side reconciliation of spec against evidence before opening the PR.** The spec
   listed the load-test mix, the Commit Points, and the checkpoint text; the evidence deviated
   from all three and nothing prompted a comparison. The `open-pull-request` checklist checks the
   branch and the PR body, not the issue folder against its own spec.
4. **Target stubs in the `Containerfile` are a hand-maintained duplicate of Cargo metadata**
   (issue #2298). The `add-workspace-member` skill documents the manual step for new members, but
   adding a target to an existing member is not covered anywhere and has no automated check.
5. **Two written rules disagree about who resolves.** The skill says the author resolves after
   replying; the reviewer's stated expectation was reviewer-side resolution. Both are defensible;
   having both unstated in the same PR left the threads open.
6. **A verification command was documented without being executed verbatim.** The author's
   ad-hoc classification loop and the skill's one-liner differed in flags and post-processing,
   and only the loop was run. The same gap the #2271 retrospective named ("write the
   verification command and its output first") applies to writing a rule: a check belongs in a
   skill only after the exact text has been run on the case that motivated it.
7. **A third-party skill shadowed the repository skill and nothing flagged it.** The editor
   surfaces its own `address-pr-comments` skill for exactly the phrasing the maintainer used in the
   editor chat session, and
   its procedure (fetch threads, fix, resolve) looks complete. The repository rule that its own
   skills take precedence exists, but it relies on the agent noticing the conflict. No review
   round flagged the missing audit record; the gap was first written down by this retrospective's
   own first version, and the maintainer acted on it after that version was committed (in the
   editor chat session; no GitHub comment records it).
8. **Review state lives in five stores and nothing reconciles them.** GitHub threads (reply,
   resolution), the working tree (the fix), the branch (the commit whose subject a reply cites),
   the audit record (rows, log) and the retrospective (counts, timeline) must all agree at the
   head that ships, and each transition between them is a separate manual act: fix, commit, push,
   reply, resolve, record, commit again. The only thing keeping them in step is the author's
   memory, and the maintainer's own description of the failure, given in the editor chat session
   and not in any GitHub comment, is exact: remembering to reply, the audit is left behind;
   remembering the audit after a rebase, a reply or resolution is left pending. The existing
   validator covers two edges (rows against the REST comments, resolution subjects against the
   branch). Nine of the ten human findings after round 1 (F8-F17; F10 is the exception) are one
   store disagreeing with another; the reviewer finds them by recomputing every store from source
   on every round, which is the work item 9 proposes to give the author first.

## What We Learnt

1. Anything that will be merged may cite a PR-branch commit only by its Conventional Commit
   subject; commit ids are durable only once on `develop`, in a tag, or in another repository.
2. Before every post-rebase push, grep the issue folder for hex tokens and check each one is a
   `develop` commit, a tag, or an external reference.
3. When a reviewer offers a "name the commit" remedy for an unreachable reference, take the
   "name the patch" remedy instead; the first resets the clock, the second stops it.
4. Before opening the PR, diff the evidence against the spec's own prescriptions (configs,
   Commit Points, checklist text) and record every deviation as an anomaly rather than leaving it
   for the reviewer to find.
5. A pure rebase (`range-diff` all `=`) changes no reviewable content; the process should let the
   reviewer say so in one line instead of re-deriving the range.
6. Adding any target (`[[bench]]`, `[[bin]]`, example) to a workspace member is a `Containerfile`
   change until #2298 lands; check the recipe stage stub list before pushing.
7. Follow `add-rust-dependency` even when copying a sibling manifest; "matches the siblings" is a
   valid rationale only once written down.
8. Before committing a command into a skill, paste the exact text into a shell and run it on the
   artifact that motivated it; if the output needs a disposition the text does not give, the rule
   is not finished.
9. "Address the review comments" in this repository means `process-pr-review`: create or update
   `docs/pr-reviews/pr-<N>-review/PR-REVIEW.md` first, then fix, reply, resolve. When an editor or
   provider skill matches the request, check `.github/skills/` for the repository owner before
   following it.
10. A retrospective is a merged artifact and gets the F1/F2/F9 test: every time comes from
    `git log` or a GitHub `created_at`, every count is recomputed at the head that ships it (and
    includes the commit that writes it), and every attribution to a person names where the
    record shows it or says the record does not.
11. Refresh the PR's threads immediately before every push, not only before the first fix; a
    round that lands during the fix is otherwise shipped over, and the reviewer has to re-raise
    it. Anchor a snapshot document with "current as of round N" so later rounds do not falsify it.
12. Never quote a reviewer from memory; open the review by ID and copy the words, or do not use
    quotation marks.
13. Hand-written counts that move with the review are a liability, not a check: they are correct at
    the keystroke and wrong at the next push. A count is useful only when a tool recomputes it at the
    head that ships and fails the push when it disagrees with the source.

## Improvements for Future Reviews

Ordered by expected return on cost, best first. Each entry states its owner artifact and
disposition; substantial workflow changes are `PROPOSED` for maintainer decision, not applied here.
Items 9 and 10 are written in enough detail to be turned into issue specs without re-deriving
them from this PR; the maintainer stated the intent to open those issues from this document in the
editor chat session, and no GitHub comment records it.

### 1. Re-ACK protocol for pure rebases — `PROPOSED`

Owner: `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`, `docs/templates/PR-REVIEW-TEMPLATE.md`,
and the maintainer's review workflow.

When a push is a rebase, the author's push comment includes
`git range-diff <old-base>..<old-head> <new-base>..<new-head>` output. If every patch is `=`, the
reviewer replies `re-ACK <new-head>` citing the range-diff, without a new round. If any patch is
`!`, only those patches are re-reviewed. The merge tool already reads ACKs by head id, so the
re-ACK is what it needs.

- Pros: Bitcoin Core precedent for exactly this tooling; no code change; keeps ACK-by-head-id
  integrity; cost per rebase drops from a full round to one command and one line.
- Cons: still needs reviewer attention per rebase; a non-pure rebase still costs a round for
  the changed patches; relies on the author supplying an honest range-diff (the reviewer
  recomputes it, so this is cheap to verify).

### 2. Decouple "merge base is current" from "reviewed" in the merge tool — `PROPOSED`

Owner: `contrib/dev-tools/git/github-merge.py` (Python prototype; Rust port planned) and
`.github/skills/dev/git-workflow/merge-pull-request/SKILL.md`. Needs its own issue.

Let the maintainer merge an approved head onto the current `develop` tip when
`git merge-tree --write-tree` is clean and the PR's CI passed at that head, instead of exiting
with `EXIT_STALE_MERGE_BASE` and sending the author to rebase. The merge commit already records
the ACKed head id and the maintainer signs it.

- Pros: removes the author round-trip and the approval revocation entirely; the merge tool is
  the one place that can do this atomically; upstream Bitcoin Core's tool merges this way.
- Cons: CI ran on the PR head against the old base, not on the merged tree (mitigation: require
  GitHub's `mergeable` state and, for non-trivial base advances, run the pre-push profile on the
  local merge before signing); changes a shared safety-critical tool, so it should ride the Rust
  port rather than the prototype; loses the property that the merged range is exactly the
  reviewed range when the base advance touches the same files.

### 3. Rebase-resilient evidence rules — `APPLIED_IN_THIS_PR`

Owner: `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`, `.github/skills/dev/debugging/fix-bug/SKILL.md`,
`.github/skills/dev/git-workflow/open-pull-request/SKILL.md`.

Authors name PR-branch patches by Conventional Commit subject; a `git grep` for hex tokens over
the issue folder runs before every post-rebase push. After F10 the check filters purely decimal
tokens, lists four dispositions (`develop` commit, tag, external reference, non-commit data), and
states that the PR body and review replies are outside its reach. Run as written on the #2314
folder it returns seven tokens, each with a disposition.

- Pros: zero tooling; removes the entire F1/F2/F9 class; already in place.
- Cons: only removes one class of rebase-induced finding; a rebase still triggers a round until
  1 or 2 is adopted; the check is a manual read, so an info-hash and a branch id look the same
  until the reader classifies them.

### 4. Spec-versus-evidence reconciliation before opening the PR — `PROPOSED`

Owner: `.github/skills/dev/git-workflow/open-pull-request/SKILL.md` checklist and
`.github/skills/dev/debugging/fix-bug/SKILL.md`.

Add a checklist item: for each config, command, Commit Point, and checklist line the spec
prescribes, confirm the evidence either matches or records the deviation in Anomalies.

- Pros: would have prevented F4, F5, F7 (three of the seventeen human findings) at the cost of a
  few minutes.
- Cons: a checklist item is only as good as its execution; it cannot be mechanically checked
  without structured specs.

### 5. Automated `Containerfile` stub check — `FOLLOW_UP_ISSUE` (#2298)

Owner: `contrib/dev-tools/checks/` (new Rust check), wired into pre-commit.

Compare `cargo metadata` targets with the recipe-stage `mkdir`/`touch` lists and fail on a
missing stub. Until it exists, `add-workspace-member` should say that adding a target to an
existing member also requires a stub.

- Pros: turns a hosted-CI-only failure into a sub-second local check; #2298 already scopes it.
- Cons: parses `Containerfile` text; duplicated lists remain until a generator replaces them.

### 6. Clarify thread resolution ownership — `PROPOSED`

Owner: `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md`.

State one rule: the author replies and resolves; the reviewer reopens the thread on the next
round if the fix is insufficient. Or the opposite, but exactly one.

- Pros: one sentence; removes the "who resolves" wait.
- Cons: the reviewer's stated preference differs from the skill's current text, so this needs
  the reviewer's agreement, not just an edit.

### 7. Audit record created after the fact for this PR — `APPLIED_IN_THIS_PR`

Owner: `docs/pr-reviews/pr-2320-review/PR-REVIEW.md`.

When first committed after round 4: fourteen findings normalized with dispositions, current-tree
verification re-run at writing time, resolution references by commit subject, replies posted on
the four Copilot threads that had none, and `validate-audit-record.py` passing (14 rows, 0
failures). The record has been extended each round since; its Processing Log states when it was
created relative to the threads and tracks later rounds.

- Pros: the continuous-improvement purpose of the audit is served; the folder matches the README
  convention; the retrospective no longer contradicts it.
- Cons: a record written after resolution cannot show the "verify before reply" sequence the
  skill is designed to enforce; its value here is aggregation, not process evidence.

### 8. Make the repository review skill the one that gets picked — `PROPOSED`

Owner: `AGENTS.md` (Engineering Policy 8 wording) and `.github/agents/copilot-suggestions-handler.agent.md`.

Add the trigger phrases maintainers actually use ("address the comments", "process the review",
"resolve the threads") to the `process-pr-review` skill description and the handler agent's
description, so the repository skill outranks a third-party one on the same phrasing.

- Pros: a description edit; targets the exact failure that occurred.
- Cons: skill selection is still heuristic; it does not stop an agent that has already started
  down the third-party path.

### 9. Review-state reconciler: a tool that computes every count the review artifacts claim — `PROPOSED`

Owner: a new Rust check under `contrib/dev-tools/checks/` (working name `review-state-reconciler`),
replacing and extending `.github/skills/dev/pr-reviews/process-pr-review/scripts/validate-audit-record.py`;
wired into `process-pr-review` step 9 and into the pre-push hook when `docs/pr-reviews/pr-<N>-review/`
is touched. The Python validator stays as the behaviour reference until the Rust port lands, per
the repository's tooling-language policy.

**Problem it solves.** Review processing keeps state in five places that must agree at the head
that ships: GitHub threads, the working tree, the commits on the branch, the audit record, and
(when present) the retrospective. Nine of the ten human findings in this PR after round 1 (F8-F17;
F10 is the exception) are a disagreement between two of those stores. The existing validator
checks audit rows against the REST comments that back them, resolution subjects against the
branch, severities against their source brackets, and the Processing Log's order; the reviewer
named what it lacks in F13 ("nothing in it compares the row set against the thread set, so a
record can omit an open finding and still report `failures: 0`"). The maintainer's diagnosis,
given in the editor chat session and not in any GitHub comment, is the same: when the author
remembers to reply, the audit is not updated; when the audit is updated after a rebase, a reply or
a resolution is left pending. Human memory is the reconciler today, and it lost sync repeatedly.

**Contract.** Given a PR number, a base ref and the audit record path, fetch threads and reviews
through GraphQL (the authoritative source for thread identity and resolution state) and report
the following, each as a computed count with the offending identifiers listed when non-zero:

| Check | Source A | Source B | Blocks push |
| ----- | -------- | -------- | ----------- |
| Every thread has exactly one audit row (by source comment id) | `reviewThreads` | Findings table `Source URL` | yes |
| Every audit row has a live thread or is `NON_RESOLVABLE` | Findings table | `reviewThreads` | yes |
| `Thread state` equals GitHub `isResolved` (`RESOLVED`/`SUPERSEDED` vs resolved; `OPEN` vs unresolved) | Findings table | `reviewThreads` | yes |
| Every resolved thread has an author reply posted on that thread, and the row cites exactly that reply URL | `reviewThreads.comments` | detail entry `Reply URL` | yes |
| Every `FIXED` row's `Resolution reference` is a commit subject on `<base>..HEAD` | detail entry | `git log` | yes (existing) |
| Every `FIXED` row's fix commit author date precedes its reply timestamp | `git log --format=%aI` | reply `createdAt` | warn (the replies processing round 7 were posted 89-96 s before the commit they cite) |
| Every human review with a non-empty body has a Processing Log entry whose stamp equals `submittedAt` to the minute | `reviews` | Processing Log | yes |
| Processing Log is chronological and every stamp is at or before the event it records, where the event has a source timestamp | Processing Log | `reviews`, `git log`, `createdAt` | yes (order exists; stamp-vs-event is new) |
| Severity in the row equals the `[Severity]` bracket of the source comment | Findings table | comment body | yes (existing) |
| No branch commit id in `docs/issues/open/<folder>/` or `docs/pr-reviews/pr-<N>-review/` (the F9/F10 grep, with the four dispositions) | worktree | `git merge-base --is-ancestor` against `<base>` | yes |
| Retrospective `Current as of review <id>` is the latest human review id, or the retrospective is absent | retrospective | `reviews` | warn |
| Latest review `submittedAt` is earlier than the last fetch the tool performed | `reviews` | tool clock | yes — this is the "a round landed while you were fixing" gate that was missed at 20:12 |

Output is one line per check, `name=<count>` with identifiers, and a non-zero exit on any blocking
failure. The counts the audit and retrospective currently hand-write (rows, threads, unresolved,
rounds, findings by severity) become "as reported by the reconciler at `<commit subject>`", and the
tool's own output is pasted under it, so the numbers in the merged artifact are the tool's, not
the author's.

**Evidence that each check would have fired in this PR.** Row set vs thread set: F13 (16 threads,
14 rows), F13 re-raise (20 threads, 16 rows). Thread state vs `isResolved`: the round-4 body
recorded F1-F9 unresolved with one reply each. Reply on resolved thread: the four Copilot threads
(round 1 at 15:16 records all four resolved; replied 18:27). Review in log: F14 (round 5 at 18:26
missing), F14 re-raise (round 6 at 19:46 missing). Stamp vs event: F11 (18:16 vs 18:08), F14
(18:18 "posted now" vs 18:27). Branch ids: F2, F9. Retrospective staleness: F15. New-round gate:
once, the 20:12 push over round 6 (19:46). Within F5-F17 the tool would not catch F5 (commit
order against the spec's Commit Points), F6, F7, F8 ("same code state" against a git diff), F10,
F12 (the commit count, which no check computes), F16 and F17 (prose).

- Pros: makes the five stores mechanically consistent at every push; turns the reviewer's
  "recomputed from the bytes" method into something the author runs first; removes the incentive
  to hand-write counts; gives the maintainer a one-command answer to "is anything pending on this
  PR"; the same output can be pasted as the round's consolidated reply.
- Cons: needs `gh` and network at pre-push time, so it must degrade to a warning offline; GraphQL
  pagination and rate limits for PRs with hundreds of threads; prose claims (`Current-tree
  verification` sentences, misattributed quotations) remain unchecked; a Rust tool that shells out
  to `gh` or speaks GraphQL directly is a larger first slice than a Python extension, though the
  repository's stated direction is Rust for stateful, safety-relevant tooling.

### 10. A fixed per-push processing loop in `process-pr-review` — `PROPOSED`

Owner: `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` (the Workflow section) and
`docs/templates/PR-REVIEW-TEMPLATE.md` (Completion Rules).

**Problem it solves.** The skill's nine steps are correct individually but are written as a
sequence to run once. Reviews arrive while the author is mid-fix, fixes need commits before
replies can cite them, replies need to exist before audit rows can cite them, and the audit commit
cannot be the fix commit. Without an explicit loop with an explicit re-fetch gate, the author
improvises the order each round and drops a step; this PR dropped a different one each round
(replies on Copilot threads; the audit record itself; the re-fetch before the 20:12 push; the
Processing Log entry for round 5).

**Proposed order, run in full for every push to the PR branch:**

1. **Fetch.** Run the reconciler (item 9) or, until it exists, the `fetch-review-threads` scripts
   plus the `reviews` connection. Record the latest review id and the fetch time.
2. **Rows.** Add one audit row per new thread or independently actionable review-body assertion,
   disposition pending, `Thread state=OPEN`. Assign collision-safe IDs. Do not fix anything yet.
3. **Fix.** One signed Conventional Commit per independent concern. Fix commits touch the product
   or the evidence; they do not touch `docs/pr-reviews/`.
4. **Verify.** For every row, run the verification command and write the `Current-tree
   verification` sentence from its output. Write the `Resolution reference` as the subject now on
   the branch.
5. **Reconcile.** Run the reconciler. Every check must be zero except `rows_without_reply`, which
   equals the number of rows fixed in this pass.
6. **Re-fetch gate.** Run the reconciler's new-round check. If a review landed after step 1, go to
   step 1 with the new findings; do not push. (This is the gate the 20:12 push skipped.)
7. **Push** the fix commits.
8. **Reply, then resolve.** For each fixed row, reply on its thread with the disposition, the
   verification, and the resolution reference (which now exists on the remote); then resolve.
   `Superseded by <FindingId>: <reason>.` verbatim for duplicates.
9. **Record.** Fill `Reply URL` from the reply responses; append Processing Log entries whose
   stamps come from `git log --format=%aI` and the GitHub `created_at`/`submittedAt` fields,
   truncated to the minute; never from the clock at the time of writing.
10. **Reconcile again.** All checks zero, including `rows_without_reply` and thread state.
11. **Commit the audit** (and the retrospective, if present) in a separate `docs(pr-reviews)`
    commit. Run the re-fetch gate once more. Push.

The retrospective, when one exists, carries a `Current as of review <id>` line that step 9 updates
and the reconciler checks; a retrospective is otherwise a snapshot and is not re-derived per round.

- Pros: no new tooling required to adopt the order; the two reconcile points and the re-fetch gate
  are where every drift in this PR would have surfaced; the audit-lags-fix-by-one-commit shape
  matches what #2271 converged on in its round five; the loop is the same for Copilot, human and
  unknown authors.
- Cons: eleven steps per push is heavier than the current nine-once; without item 9 the reconcile
  steps are manual and will be skipped under time pressure, which is how this PR got here;
  reviewers who post several rounds in quick succession (rounds 5, 6 and 7 here were 18:26, 19:46
  and 20:24) can keep the author in step 6 indefinitely, which is a reviewer-cadence question the
  loop cannot solve.

### Alternatives considered and discarded

- **GitHub merge queue / auto-merge.** Rebases and tests automatically, but the merge commit is
  GitHub-authored and unsigned, drops the ACK trailers the merge tool writes, and `develop` is
  not currently protected. Too large a change in the trust model for this problem.
- **Maintainer-side merge ordering** (merge the oldest approved PR first; hold fast PRs). No
  tooling, but it slows every other PR to protect one and depends on discipline that does not
  scale.
- **Squashing the branch before merge** to reduce the surface a rebase can break. Rejected by
  `AGENTS.md`: intermediate commits are kept deliberately.
- **Re-running the end-to-end load test with the spec's scrape-heavy mix** to close F4 by
  measurement rather than by recording the deviation. Valid, but it needs release builds of two
  code states plus load-test runs on a machine that cannot run the container build; the
  microbenchmark already resolves the change. Recorded as a possible follow-up in the evidence.
- **Hand-maintained counters in the audit and retrospective** ("21 threads, 21 rows, 0 pending")
  as the check-before-reply the maintainer asked about in the editor chat session (no GitHub
  comment records it). Considered and rejected as a standalone measure: the hand-written counts
  that moved with the review were wrong within one round (F12, F15), and
  #2271 lesson 3 already says counts and universals rot. Counters help only as the *output* of item
  9, recomputed at the head that ships; as inputs typed by the author they add another store to
  keep in sync.
- **Extending the Python validator instead of writing the reconciler in Rust.** Faster first
  slice, and acceptable as a prototype if the issue says so, but the tool is stateful (fetch time,
  latest review id), safety-relevant (it gates pushes) and worth testing on its own, which is the
  repository's stated threshold for Rust. Recorded here so the issue can choose deliberately.

## Avoiding Overcorrection

- Do not add fields to the evidence templates. The findings came from wrong values, not missing
  fields.
- Do not build the container in pre-push. It is minutes of CPU and gigabytes of memory on every
  push; the stub check in item 5 is the proportionate fix.
- Do not require a full-mix end-to-end rerun for every performance-sensitive PR. The
  microbenchmark is the precise instrument; the end-to-end leg is a gross-regression guard.
- Do not relax GPG-signed maintainer merges to solve the rebase loop. Items 1 and 2 keep them.
- Do not treat the after-the-fact audit record as equivalent to one kept during the review; the
  Processing Log says when it was written, and that is enough.
- Do not add hand-written counter fields to the templates in response to F12 and F15. The
  reconciler (item 9) computes them; a typed counter is one more value to drift.
- Do not make the reconciler check prose. `Current-tree verification` sentences and quotations
  (F16) stay a human responsibility; lessons 8, 10 and 12 cover them.

## Evidence

- Audit record: `docs/pr-reviews/pr-2320-review/PR-REVIEW.md` (first committed after round 4 with
  14 rows; each later round is checked with
  `validate-audit-record.py --pr-number 2320 --base torrust/develop`).
- Pull request: <https://github.com/torrust/torrust-tracker/pull/2320>
- Review rounds: Copilot 13:09:22 UTC; human 15:16:12 UTC (`CHANGES_REQUESTED`), 16:00:09 UTC
  (`APPROVED`), 17:24:16 UTC, 18:02:41 UTC, 18:26:03 UTC, 19:46:44 UTC and 20:24:38 UTC (all
  `CHANGES_REQUESTED`), all 2026-09-23, from the `reviews` GraphQL connection.
- Findings: 4 Copilot threads plus F1-F17 from the `reviewThreads` GraphQL connection; severities
  from each thread's first comment.
- Branch commits: `git log --format='%h %aI %cI %s' torrust/develop..HEAD`; the 17:08 UTC
  committer date on the first seven commits is the round-3 rebase.
- Merge-tool behaviour: `contrib/dev-tools/git/github-merge.py`, `EXIT_STALE_MERGE_BASE` check
  and the ACK-collection function (matches `ACK` lines by the six-character head prefix).
- Container failure: Container workflow run 35867175758, step 8, `cargo chef cook` error
  `can't find udp_tracker_server_benchmark bench`.
- Related issue for item 5: `docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md`.
