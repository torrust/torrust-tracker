---
semantic-links:
  skill-links:
    - process-pr-review
    - open-pull-request
    - fix-bug
  related-artifacts:
    - docs/issues/open/2314-preserve-udp-scrape-response-order/ISSUE.md
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

Record why a two-file production fix with three regression tests needed five review rounds and
five review-response commits, why an approval was lost to a rebase whose `range-diff` was
byte-identical, and what to change so that rebasing an open pull request stops restarting its
review. This is a blameless review of the process, not of the reviewer or the author.

No `PR-REVIEW.md` audit record exists for this pull request: every finding was processed as a
GitHub review thread and answered in place, without the `process-pr-review` skill. The metrics
below are therefore derived from the GitHub API and `git log`, not from an audit record.

## Review Summary

| Metric | Value |
| ------ | ----- |
| Review rounds | 5 (4 human, 1 Copilot) |
| Findings | 14 (4 Copilot, 10 human: 6 Minor, 1 Suggestion, 3 Nit) |
| Blocking findings | 4 (F1, F2 in round 1; F9 in round 3; F10 in round 4) |
| Re-raised findings | 1 (F9 is the same defect class as F1 and F2, reintroduced by their fix) |
| Human findings about issue-local evidence documents | 8 of 10 (F3 is a manifest comment; F10 is a skill rule) |
| Human findings about process changes made during the review | 1 (F10, on the rule added in response to F9) |
| Human findings about production code or tests | 0 |
| Commits on the branch | 11 (4 original, 1 CI fix, 5 review-response, 1 retrospective) |
| Rebases while the PR was open | 2 (one before the first human review, one between rounds 2 and 3) |
| Approval lifetime | 1 h 24 min (approved 16:00 UTC, changes requested 17:24 UTC on a pure rebase) |
| First review to last author reply | 2026-09-23 13:09 UTC to the round-4 reply (see Timeline) |

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
| after 17:52 | Author resolves all nine human threads at the maintainer's request. |
| 18:02 | Round 4 (human), `CHANGES_REQUESTED` at head `7e426ba2`: F8, F9 addressed; F10 blocks — the new pre-push grep returns 20 hits on this PR's own folder and its disposition list covers 10. |
| 18:16 | Author commits this retrospective (first version, before reading round 4). |
| after 18:16 | Author fixes F10 (fourth disposition, decimal filter, scope note), updates this retrospective, replies, resolves. |

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
   nine human findings, including all three blocking ones, are this one defect.
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
   three rounds until the maintainer asked for them to be resolved.
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

## Improvements for Future Reviews

Ordered by expected return on cost, best first. Each entry states its owner artifact and
disposition; substantial workflow changes are `PROPOSED` for maintainer decision, not applied here.

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

- Pros: would have prevented F4, F5, F7 (three of nine findings) at the cost of a few minutes.
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

## Avoiding Overcorrection

- Do not add fields to the evidence templates. The findings came from wrong values, not missing
  fields.
- Do not build the container in pre-push. It is minutes of CPU and gigabytes of memory on every
  push; the stub check in item 5 is the proportionate fix.
- Do not require a full-mix end-to-end rerun for every performance-sensitive PR. The
  microbenchmark is the precise instrument; the end-to-end leg is a gross-regression guard.
- Do not create a `PR-REVIEW.md` audit record retroactively for this PR. The GitHub threads and
  this retrospective are the record; a backfilled audit would be the kind of self-referential
  artifact the #2271 retrospective warned about.
- Do not relax GPG-signed maintainer merges to solve the rebase loop. Items 1 and 2 keep them.

## Evidence

- Pull request: <https://github.com/torrust/torrust-tracker/pull/2320>
- Review rounds: Copilot 13:09:22 UTC; human 15:16:12 UTC (`CHANGES_REQUESTED`), 16:00:09 UTC
  (`APPROVED`), 17:24:16 UTC (`CHANGES_REQUESTED`), 18:02:41 UTC (`CHANGES_REQUESTED`), all
  2026-09-23, from the `reviews` GraphQL connection.
- Findings: 4 Copilot threads plus F1-F10 from the `reviewThreads` GraphQL connection; severities
  from each thread's first comment.
- Branch commits: `git log --format='%h %aI %cI %s' torrust/develop..HEAD`; the 17:08 UTC
  committer date on the first seven commits is the round-3 rebase.
- Merge-tool behaviour: `contrib/dev-tools/git/github-merge.py`, `EXIT_STALE_MERGE_BASE` check
  and the ACK-collection function (matches `ACK` lines by the six-character head prefix).
- Container failure: Container workflow run 35867175758, step 8, `cargo chef cook` error
  `can't find udp_tracker_server_benchmark bench`.
- Related issue for item 5: `docs/issues/open/2298-rust-dev-tool-container-integration/ISSUE.md`.
