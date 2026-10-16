---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p1
epic: null
github-issue: 2507
spec-path: docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
branch: "2507-stop-requiring-pull-request-rebases"
related-pr: 2526
last-updated-utc: "2026-10-16 08:30"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - open-pull-request
    - merge-pull-request
    - implement-workflow
  related-artifacts:
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/baseline-evidence.md
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/implementation-retrospective.md
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/agent-review-reports.md
    - "issue #2525"
    - "issue #2547"
    - docs/issues/open/2547-measure-effect-of-optional-rebases/ISSUE.md
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
    - .github/skills/dev/git-workflow/open-pull-request/SKILL.md
    - .github/skills/dev/git-workflow/merge-pull-request/SKILL.md
    - contrib/dev-tools/git/github-merge.py
    - contrib/dev-tools/git/README-github-merge.md
    - contrib/dev-tools/git/tests/test-github-merge-symlinks.py
    - .github/workflows/docs-lint.yaml
    - AGENTS.md
---

<!-- skill-link: create-issue -->

# Issue #2507 - Stop requiring pull-request rebases and reconcile repository rulesets

## Goal

Make rebasing a pull request onto `develop` a recommendation instead of a merge requirement, so a
merge no longer forces every other open pull request to be rebased and re-tested. Keep the
maintainer-signed merge workflow (`github-merge.py`) working, and leave the GitHub rulesets in a
state that is enforced, satisfiable, and documented.

## Background

### The rebase rule

Every pull request targeting `develop` has been rebased onto the latest `develop` before merge for
years. The rule was never recorded as a decision: no ADR, issue, or commit body explains it. It is
encoded in the `open-pull-request` skill (rebase before every push), the `merge-pull-request` skill
(rebase immediately before merging), and since PR #2229 in a stale-merge-base guard in
`contrib/dev-tools/git/github-merge.py`. `AGENTS.md` meanwhile says to "only rebase onto `develop`
to resolve conflicts", which contradicts the skills.

Rebasing has a real benefit: it detects integration problems before they reach `develop`. That
benefit is kept: rebasing remains recommended whenever it is cheap. The requirement is dropped only
for performance reasons, to reduce CPU consumption and runner costs. Most pull requests are now
prepared by AI agents, and the volume changed by an order of magnitude (full data in
[`baseline-evidence.md`](baseline-evidence.md)):

| Measure                                | June 2026 | October 2026 (1-13) |
| -------------------------------------- | --------: | ------------------: |
| Merged PRs                             |        67 |     67 (in 13 days) |
| Median open PRs waiting (daily)        |         0 |                   8 |
| Force-pushes per merged PR             |      0.39 |                1.58 |
| Merged PRs force-pushed at least once  |       34% |                 81% |
| PR CI runner-hours (estimate, 13 days) |         — |                 198 |

Each merge makes every other open PR stale, so a mandatory rebase causes:

1. **CI reruns.** A rebased code PR reruns about 92 runner-minutes of workflows; roughly 29% of
   PR CI time in early October went to re-testing rebased heads (estimate).
2. **Review noise.** Review records and evidence cite commit ids; rebases orphan them. About two
   dozen commits since September only repaired rebase fallout, and each repair push triggered
   another review and CI round.
3. **Lost approvals.** `dismiss_stale_reviews_on_push: true` dismisses approvals on every
   force-push, even a byte-identical rebase (PR #2320 retrospective).
4. **Serialized merging.** Draining a queue of N PRs needs N rebase-and-wait cycles.

The history benefit is small: the merge tool always creates `--no-ff` merge commits, so `develop`
was never linear, and `git log --first-parent develop` lists one merge per PR either way.

The technical constraint is in the merge tool. `github-merge.py` pushes a locally signed merge
commit, and GitHub's `pull_request` ruleset only accepted it (no `GH013`) when
`refs/pull/<n>/merge` was computed against the current `develop` tip. GitHub did not recompute
that ref for at least 48 minutes after `develop` advanced through locally pushed merges, even after
a mergeability query (baseline evidence section 3). Today a push to the PR head, in practice a
rebase, is what refreshes it.

Correction from T1: the sandbox experiment showed that the condition is the pull request's recorded
base (`base.sha`), not the freshness of `refs/pull/<n>/merge`; the paragraph above records what was
believed when this specification was written. T2 implements the corrected condition.

### The rulesets

Two repository rulesets are disabled, and neither state is documented (baseline evidence
sections 2.1 and 2.2):

- **94237 "Deny Force Push; Require Status Checks"** requires 16 status checks with
  `strict_required_status_checks_policy: true` and denies force-pushes. The maintainer disabled it
  around every merge on 2026-05-01, 05-04, 05-05, and 07-20; no merge was ever pushed while it was
  active. It cannot be satisfied: six required check names disappeared when the test jobs were
  collapsed on 2026-05-01, and since the documentation-only fast path (issue #1743) a Markdown-only
  PR reports only `Docs Lint`. It has been disabled since 2026-05-05 except for 16 minutes on
  2026-07-20, which also left `develop`, `main`, and `releases/*` without a force-push ban.
- **19245352 "Code Quality Copilot review for default branch"** requests Copilot reviews. GitHub's
  `ghost` actor created and disabled it, probably through the Code Quality feature. Maintainer PRs
  still get Copilot reviews through the authors' personal auto-review setting; external PRs do not.

Disabling a ruleset to merge a known-good PR, then forgetting to re-enable it, is a recurring
failure mode. The target configuration must not need toggling.

## Scope

### In Scope

- Record the decision in root ADR
  [`20261013161419_stop_requiring_pull_request_rebases.md`](../../../adrs/20261013161419_stop_requiring_pull_request_rebases.md).
- Find what GitHub requires before accepting a maintainer merge of a pull request that is behind
  `develop`, and the least disruptive way to satisfy it (decision gate T1). GitHub's "Update
  branch" merge commit is an accepted fallback. T1 found the condition is the pull request's
  recorded base (`base.sha`), not the freshness of its merge ref.
- Update `github-merge.py`, its tests, and `README-github-merge.md` so the tool checks and
  refreshes the recorded base instead of sending authors to rebase.
- Update the `open-pull-request` and `merge-pull-request` skills, and `AGENTS.md`, to the
  recommended-not-required rebase policy.
- Reconcile the repository rulesets: restore the force-push ban, delete the unsatisfiable
  required-checks ruleset, decide the Copilot review ruleset, delete rulesets that are no longer
  needed, and document the result.
- Draft the follow-up issue for a GitHub-required, always-reporting `CI gate` check (T8).
- Re-measure force-pushes and PR CI time after the change.

### Out of Scope

- Replacing `github-merge.py` with GitHub's merge button or a merge queue (ADR option D).
- Building the `CI gate` check, moving CI decision logic into Rust, and restructuring workflow path
  filters: delivered by the follow-up issue from T8.
- Changing which workflows run for documentation-only PRs (the issue #1743 fast path stays).
- Removing the "cite commits by subject" guidance from `fix-bug`, `process-pr-review`, and the
  evidence templates: rebases still happen, so it remains a safe default. A later issue may relax
  it with post-change evidence.
- Changing `dismiss_stale_reviews_on_push`.
- Tag rulesets and the `releases/*` and `staging/*` bypass rulesets, except where T5 finds them
  obsolete.

## Architectural Decisions

- Related ADRs:
  [`20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`](../../../adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md)
  (records the rebase-driven CI cost).
- ADRs to create: root ADR
  [`20261013161419_stop_requiring_pull_request_rebases.md`](../../../adrs/20261013161419_stop_requiring_pull_request_rebases.md),
  accepted through the spec-only PR so the recommendation applies immediately. If T1 or T5
  invalidates any agreement in it, update the ADR in the implementation PR.
- Possible additional ADR: if T5 moves required-check enforcement from GitHub rulesets into the
  merge tool, record that decision.

## Design and Ownership Review

Not applicable. The work changes contributor workflow documentation, one Python maintainer tool,
possibly one workflow job, and repository settings; it involves no child processes, asynchronous
I/O, or reusable test fixtures in the tracker.

## Bug-Fix Process

Not applicable. This is a policy and configuration change, not broken tracker behavior.

## Regression Test Strategy

Not applicable. The `github-merge.py` behavior change is covered by the existing deterministic
fixture tests (T2).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                       | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                               |
| --- | ------ | ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Decision gate: find a non-rewriting merge-ref refresh      | Maintainer-approved fork sandbox (evidence section T1). GitHub accepts the direct merge push when the PR `base.sha` equals the target tip; merge-ref freshness is irrelevant. Waiting is unreliable; close/reopen, Update branch, and a no-op base edit all refresh `base.sha` without rewriting commits. **Decision (maintainer, 2026-10-14):** the merge tool performs the no-op base edit itself.                                                                                                                                                                  |
| T2  | DONE   | Make `github-merge.py` check and refresh the recorded base | Guard compares the PR `base.sha` with the fetched target tip; a stale base is refreshed with the no-op base edit through `gh`, whose response supplies the new `base.sha`; refusal (exit `4`) prints the manual command. The GitHub-merge comparison is skipped when that merge has a stale base. Tests: 5 fixture scenarios plus 2 `gh` request tests; mutations (skip the guard, never refresh, always compare, wrong base argument) each failed at least one new test. Commit `fix(git): [#2507] check and refresh the recorded pull-request base before merging`. |
| T3  | DONE   | Update the `merge-pull-request` skill                      | Automatic refresh, `gh` prerequisite, manual command, and the `GH013` diagnosis now name the recorded base. Commits `docs(skills): [#2507] describe the automatic recorded-base refresh in the merge skill` and `docs(skills): [#2507] name the recorded base in the merge push-failure diagnosis`.                                                                                                                                                                                                                                                                   |
| T4  | DONE   | Update the `open-pull-request` skill and `AGENTS.md`       | Done in spec PR #2508. A search of `.github/`, `AGENTS.md`, `docs/templates/`, and `contrib/dev-tools/git/` for rebase mentions on 2026-10-14 found no remaining mandatory-rebase instruction; the remaining mentions describe the new rule or why rebases rewrite commit ids.                                                                                                                                                                                                                                                                                        |
| T5  | DONE   | Reconcile repository rulesets                              | Applied 2026-10-14 with maintainer approval: new always-active force-push ruleset 25068452; deleted 94237, 19245352, and 87098 (definitions saved under `evidence/`). Evidence section "T5 - Ruleset Changes Applied". M5 and M6 confirmed on 2026-10-15 by real code and documentation-only merges with no ruleset changed (evidence V5, V6). The required `CI gate` is the follow-up issue #2525 (T8).                                                                                                                                                              |
| T6  | DONE   | Document the ruleset configuration                         | New guide `docs/github-rulesets.md` (linked from `docs/index.md`): policy, every active ruleset and why it exists, deleted rulesets, and a drift check. The merge skill guardrails forbid disabling a ruleset to merge. Commits `docs(github): [#2507] document the active repository rulesets` and `docs(skills): [#2507] forbid disabling rulesets to merge`.                                                                                                                                                                                                       |
| T7  | DONE   | Post-change measurement                                    | Moved out of this issue by maintainer decision on 2026-10-15 so it can close without a two-week wait: follow-up issue #2547 measures the window 2026-10-15 to 2026-10-28 against [`baseline-evidence.md`](baseline-evidence.md) and records `post-change-evidence.md` in its own folder. No measurement is done here.                                                                                                                                                                                                                                                 |
| T8  | DONE   | Draft and create the CI gate follow-up issue               | Maintainer approved the draft on 2026-10-14; created #2525 with its specification under `docs/issues/open/2525-ci-gate-required-check/`. Commit `docs(issues): add issue specification for #2525`.                                                                                                                                                                                                                                                                                                                                                                    |

### Ruleset Target State

Decided by the maintainer on 2026-10-14 from the sandbox evidence (section T5): required checks are
enforced by GitHub on the maintainer's direct merge push, so they will be enforced by GitHub through
one always-reporting `CI gate` check. Building that gate is substantial CI work and moves to the
follow-up issue #2525 (drafted in T8); until it lands, no check is required by GitHub, which has been the
effective state since 2026-05-05. T5 applies the rest, each item after maintainer approval because
the settings are outside Git.

| Ruleset                                   | Action                                                                                                                                                                                                |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 94237 force-push ban (`non_fast_forward`) | Move into its own always-active ruleset covering the same branches, so it is never disabled together with status checks. Fork PR branches are unaffected.                                             |
| 94237 required status checks              | Delete with the ruleset. Issue #2525 adds a single required `CI gate` context with `strict_required_status_checks_policy: false`.                                                                     |
| 19245352 Copilot code review              | Deleted (maintainer decision 2026-10-14): maintainer PRs get Copilot reviews through personal auto-review settings, and `review_on_push` would add review noise and cost.                             |
| 87098 "Require Deployment: coverage"      | Deleted (2026-10-14): active with no rules since 2026-07-20, so it enforced nothing.                                                                                                                  |
| Temporary overrides                       | Never disable a ruleset to merge. The sandbox showed that a posted status cannot override a failing Actions check run, so the follow-up gate honors a maintainer-applied `ci-override` label instead. |

## Commit Points

| Task | Coherent change set                                                                 | Commit policy                                                                      |
| ---- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| T1   | `manual-verification-evidence.md` with the experiment results; ADR update if needed | Commit after the maintainer approves the chosen refresh method.                    |
| T2   | `github-merge.py`, its fixture test, `README-github-merge.md`                       | Commit after the merge-tool tests pass.                                            |
| T3   | `merge-pull-request` skill                                                          | Commit after `linter all` passes.                                                  |
| T4   | `open-pull-request` skill, `AGENTS.md`, and any other aligned instructions          | Commit after `linter all` passes.                                                  |
| T5   | Ruleset evidence (settings change itself is not in Git)                             | One commit per code change; commit evidence after the maintainer applies settings. |
| T6   | Ruleset documentation                                                               | Commit after `linter all` passes.                                                  |
| T7   | Follow-up measurement specification (moved to #2547)                                | Commit with the close-out evidence after maintainer review of the draft.           |
| T8   | Draft follow-up specification under `docs/issues/drafts/`                           | Commit after maintainer review of the draft.                                       |

Record a justified no-change decision in the task's evidence without creating an empty commit.
Use Conventional Commits with the narrow affected scope (`fix(git)`, `ci(workflows)`,
`docs(skills)`, `docs(issues)`), and sign every commit with GPG.

## Closure Plan

Recorded 2026-10-15 so the remaining steps are not forgotten. Follow them in order. The
two-week measurement no longer blocks closure, because it moved to #2547 (T7).

| Step | Status | Action                                                                                                                                                                                                                                                           |
| ---- | ------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C1   | DONE   | Record M1 to M6 and M8 from the real merges after PR #2526 (`manual-verification-evidence.md`, section "Merges Under the New Policy"), and tick AC1 to AC4 and AC6 with evidence.                                                                                |
| C2   | DONE   | Maintainer accepts AC6 as met while no check is required, with the always-reporting required `CI gate` tracked by #2525. #2507 does not wait for #2525.                                                                                                          |
| C3   | DONE   | Move T7 and AC9 to follow-up issue #2547, with its specification in `docs/issues/open/`.                                                                                                                                                                         |
| C4   | DONE   | Record the implementation completion review in [`implementation-retrospective.md`](implementation-retrospective.md).                                                                                                                                             |
| C5   | DONE   | Run the independent Task Reviewer on this specification and the close-out branch; record its result in `agent-review-reports.md` and fix its findings.                                                                                                           |
| C6   | TODO   | Commit, push, and open the close-out PR with `Related to #2507` (not `Closes`), then process its review with `process-pr-review`.                                                                                                                                |
| C7   | TODO   | After the close-out PR merges, close #2507 on GitHub with a comment that links the close-out PR, #2525, and #2547. Then archive this folder to `docs/issues/closed/` with the `cleanup-completed-issues` skill, which also repairs links such as those in #2547. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/stop-requiring-pull-request-rebases/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, merge-tool tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-13 16:20 UTC - AI assistant - Drafted spec, baseline evidence, and proposed root ADR
  from documentation, Git history, GitHub ruleset configuration, PR statistics, and CI run data -
  [`baseline-evidence.md`](baseline-evidence.md)
- 2026-10-13 16:58 UTC - AI assistant - Applied maintainer review: rebasing stays recommended
  (dropped as a requirement for performance only); "Update branch" accepted as fallback; ADR
  accepted in the spec PR; added ruleset version-history analysis and the ruleset reconciliation
  tasks (T5, T6) - [`baseline-evidence.md`](baseline-evidence.md) sections 2.1 and 2.2
- 2026-10-13 17:05 UTC - AI assistant - Maintainer approved the spec; created issue #2507, moved the
  spec to `docs/issues/open/`, and opened the spec-only PR from branch
  `2507-stop-requiring-pull-request-rebases-spec` (PR #2508) and requested review from da2ce7
- 2026-10-13 17:30 UTC - AI assistant - Processed Copilot review on PR #2508: cited the issue by
  number in the ADR frontmatter, and updated the `open-pull-request` and `merge-pull-request`
  skills and `AGENTS.md` in the spec PR so the immediately effective ADR has no conflicting
  workflow instruction (T3 and T4 in progress) - PR #2508 review audit in `docs/pr-reviews/`
- 2026-10-14 06:47 UTC - AI assistant - Spec PR #2508 merged (`d383d4b45`); started the
  implementation on branch `2507-stop-requiring-pull-request-rebases`. T1 option (a) rejected:
  open PR merge refs are recomputed on head pushes but not reliably after `develop` advances
  (#2500 stale for about 19 hours) - [`manual-verification-evidence.md`](manual-verification-evidence.md)
  section T1
- 2026-10-14 07:24 UTC - AI assistant - Ran the maintainer-approved fork sandbox (T1, T5).
  GitHub accepts the direct merge push when the PR `base.sha` equals the branch tip, not when the
  merge ref is fresh. Close/reopen, Update branch, and a no-op base edit all refresh it without
  rewriting commits; the no-op base edit also adds no commit, event, or CI run. Required status
  checks are enforced on the direct push; a commit status cannot override a failing Actions check
  run. Awaiting maintainer decisions on the refresh method and the required-checks design -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) sections T1 and T5
- 2026-10-14 09:10 UTC - AI assistant - Recorded maintainer decisions: the merge tool runs the no-op base edit itself
  (T1 done, T2 redefined); required checks are enforced by GitHub through one always-reporting
  `CI gate` with CI logic in Rust and thin, provider-agnostic workflows, overridable by a maintainer
  `ci-override` label; that gate is split into a follow-up issue (new T8), and T5 keeps the force-push
  ban, deletions, and the Copilot ruleset decision.
- 2026-10-14 09:22 UTC - AI assistant - T2, T3, and T4 done. Test design review (prose-first AAA):
  each new test states one causal difference in its name and Arrange (recorded base current, stale
  with a refresh that records the tip, fails, or leaves it stale; GitHub merge stale with a test
  command); the shared `open_pull_request_behind_the_target_branch` helper owns only the incidental
  history and returns the commits the Act and Assert use; the production Act (`merge`,
  `refresh_pr_base`) and independently specified results (exit code, refresh calls, messages, `gh`
  arguments) stay visible. `python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py`
  (Python 3.14.4): 47 tests OK; `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh`: passed.
- 2026-10-14 10:01 UTC - AI assistant - Maintainer decided to delete Copilot ruleset 19245352 and
  approved applying T5. Created force-push ruleset 25068452 and deleted 94237, 19245352, and 87098;
  M7, AC5, and AC7 done - [`manual-verification-evidence.md`](manual-verification-evidence.md)
  section "T5 - Ruleset Changes Applied"
- 2026-10-14 10:04 UTC - AI assistant - T6 done: added `docs/github-rulesets.md` and a merge-skill
  guardrail against disabling rulesets; AC8 done.
- 2026-10-14 10:11 UTC - AI assistant - T8 done: maintainer approved the CI gate draft; created
  issue #2525 and its specification.
- 2026-10-14 11:40 UTC - AI assistant - Processed the PR #2526 reviews (Copilot and da2ce7; audit in
  `docs/pr-reviews/pr-2526-review/PR-REVIEW.md`). Spec corrections: Scope and AC3 restated around
  the recorded base, `related-pr` set to 2526, AC5 cites the recorded release-branch check, a
  "Correction from T1" added to the Background, two stamps corrected, and the 19245352 and 87098
  target-state rows record the applied deletions. The merge tool also refuses a pull request that
  targets another branch and reports `gh` failures with a timeout.
- 2026-10-14 12:18 UTC - Maintainer - PR #2526 merged with the maintainer tool; implementation
  (T1 to T6, T8) is on `develop`.
- 2026-10-15 16:55 UTC - AI assistant - Close-out on branch
  `2507-record-verification-and-split-measurement`. Recorded M1 to M6 and M8 from the 18 real
  merges from PR #2526 to #2543: six behind pull requests merged through the recorded-base refresh (M1, by
  indirect evidence because the tool output was not captured), nine merged behind without
  rewriting commits (M2), every merged head reachable from `develop` (M3), #2532 and #2537 took
  the conflict rebase path (M4), code and documentation-only merges with no ruleset changed (M5,
  M6), and one first-parent merge per pull request (M8). Re-ran the merge-tool tests: 51 tests OK
  (Python 3.14.4) and the wrapper tests passed. Maintainer decisions: accept AC6 while no check is
  required, with the `CI gate` tracked by #2525, and move T7 and AC9 to a follow-up issue,
  drafted for review. Added the Closure Plan and
  [`implementation-retrospective.md`](implementation-retrospective.md) -
  [`manual-verification-evidence.md`](manual-verification-evidence.md) section "Merges Under the
  New Policy"
- 2026-10-16 06:12 UTC - AI assistant - Maintainer approved the follow-up draft; created issue #2547
  and moved its spec to `docs/issues/open/2547-measure-effect-of-optional-rebases/` (C3). The #2525
  Delivery Sequence was recorded separately in `docs/issues/open/2525-ci-gate-required-check/ISSUE.md`,
  whose own log entry is 2026-10-15 17:20 UTC.
- 2026-10-16 06:40 UTC - AI assistant - Task Reviewer reviewed the close-out (C5): verdict
  `REVIEW FAILED`, with five should-fix and three nit findings, which were fixed in the same pull
  request. Fixed its findings: weighed the alternative explanation in V1 and marked M1 as indirect,
  corrected the scope and initial state of the merge evidence, reworded AC9 to the verifiable
  outcome, corrected timestamps, and described #2541 as a heavy-CI rather than code PR -
  [`agent-review-reports.md`](agent-review-reports.md)
- 2026-10-16 08:30 UTC - AI assistant - Processed the PR #2550 reviews (Copilot and da2ce7; audit in
  `docs/pr-reviews/pr-2550-review/PR-REVIEW.md`). Recorded a re-review of the fixes, which first
  found residual "proven" wording for M1 and an incomplete V4 account, then passed at 08:30 UTC.
  Corrected the log entries above, labelled the force-push event commits in V4 and added the fetch
  step, classified all 11 force-pushes of the 18 merges, and amended the #2547 specification and
  issue body - [`agent-review-reports.md`](agent-review-reports.md)

## Acceptance Criteria

- [x] AC1: No skill, agent profile, or `AGENTS.md` instruction makes a rebase mandatory merely
      because `develop` advanced; the recommendation and the required cases match the ADR.
- [x] AC2: A behind, conflict-free PR can be merged with `merge-pull-request.sh` without rewriting
      its commits (T1 refresh), or with a rebase only when T1 finds no other method.
- [x] AC3: `github-merge.py` refuses, before building a merge and without editing the pull
      request, a pull request that targets another branch. It refreshes a stale recorded base
      (`base.sha`) with the no-op base edit, and refuses to sign or push while the recorded base is
      not the target tip, printing the refresh command. A stale GitHub merge ref alone does not
      block the merge; only the comparison with that merge is skipped.
- [x] AC4: Commit ids of a PR merged without rebase remain reachable from `develop`.
- [x] AC5: The force-push ban is active on `develop`, `main`, and `releases/*`.
- [x] AC6: Every active required-check mechanism can be satisfied by both a code PR and a
      documentation-only PR, and both were merged under it without disabling any ruleset.
- [x] AC7: Rulesets 94237, 19245352, and 87098 are each either active with a documented purpose
      or deleted; no ruleset is left disabled without a recorded reason.
- [x] AC8: The active ruleset configuration is documented in the repository (T6).
- [x] AC9: The measurement of post-change force-pushes per merged PR and PR CI runner-hours against
      the baseline is tracked by follow-up issue #2547. Originally this criterion required the
      measurement inside this issue; the maintainer moved it on 2026-10-15 (T7).
- [x] `linter all` exits with code `0`
- [x] Merge-tool tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py` (Python 3 runtime)
- `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- If a gate job is added: `yamllint -c .yamllint-ci.yml` and the `implement-workflow` skill checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                     | Human-oriented command/steps                                                                                                                                          | Expected Result                                                                                                        | Status | Evidence                                                                 |
| --- | -------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------ |
| M1  | Stale recorded base is refreshed             | Pick a PR whose `gh api repos/torrust/torrust-tracker/pulls/<n> --jq .base.sha` is not the `develop` tip and run `./contrib/dev-tools/git/merge-pull-request.sh <n>`. | Tool prints `Refreshed the recorded base of torrust/torrust-tracker#<n>` and continues to inspection without a rebase. | DONE   | `manual-verification-evidence.md` section V1 (indirect)                  |
| M2  | Merge a behind PR without rewriting commits  | Continue the M1 run: inspect, sign, and push.                                                                                                                         | Push accepted; PR shows as merged; reviewed commits keep their ids.                                                    | DONE   | `manual-verification-evidence.md` section V2                             |
| M3  | Reviewed ids survive the merge               | After M2, run `git merge-base --is-ancestor <reviewed head id> torrust/develop`.                                                                                      | Exit code `0`.                                                                                                         | DONE   | `manual-verification-evidence.md` section V3                             |
| M4  | Conflicting PR follows the rebase path       | Follow the updated `open-pull-request` skill for a PR GitHub reports as conflicting.                                                                                  | Skill directs a rebase with `--force-with-lease`; PR becomes mergeable.                                                | DONE   | `manual-verification-evidence.md` section V4                             |
| M5  | Code PR merges under the new rulesets        | Merge a real code PR with the maintainer tool after T5 is applied.                                                                                                    | Push accepted without disabling any ruleset.                                                                           | DONE   | `manual-verification-evidence.md` section V5                             |
| M6  | Documentation-only PR merges under new rules | Merge a real Markdown-only PR with the maintainer tool after T5 is applied.                                                                                           | Push accepted without disabling any ruleset; heavy workflows stay skipped.                                             | DONE   | `manual-verification-evidence.md` section V6                             |
| M7  | Force-push ban applies to `develop`          | Run `gh api repos/torrust/torrust-tracker/rules/branches/develop` and the same for `main`.                                                                            | A `non_fast_forward` rule is listed for both branches.                                                                 | DONE   | `manual-verification-evidence.md` section "T5 - Ruleset Changes Applied" |
| M8  | First-parent history unchanged               | `git log --first-parent --oneline -10 torrust/develop` after several merges under the new policy.                                                                     | One merge commit per PR in merge order.                                                                                | DONE   | `manual-verification-evidence.md` section V8                             |

Notes:

- These scenarios are executed during real maintainer merges; no throwaway PRs are required.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The baseline and post-change statistics use the `gh` and `git` commands recorded in
[`baseline-evidence.md`](baseline-evidence.md).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                                                                                                                                                            |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | T4 search of 2026-10-14 and a broader `git grep -niE rebase` on 2026-10-15 over `AGENTS.md`, `.github/agents`, `.github/skills`, `docs/*.md`, `docs/templates`, and `contrib/dev-tools/git/*.md`: only the optional-rebase rule, its required cases, and historical records remain. |
| AC2   | DONE                   | M1 and M2: nine PRs merged behind `develop` without a rebase at merge time. #2528, #2529, #2530, #2534, #2535, and #2541 merged through the recorded-base refresh (evidence V1, V2).                                                                                                |
| AC3   | DONE                   | T2 tests (51 tests OK, Python 3.14.4, re-run 2026-10-15) and the PR #2526 review; in production, the refresh path in V1.                                                                                                                                                            |
| AC4   | DONE                   | M3: `git merge-base --is-ancestor` exits `0` for all 18 merged heads (evidence V3).                                                                                                                                                                                                 |
| AC5   | DONE                   | `non_fast_forward` listed for `develop`, `main`, and `releases/v3.0.0-rc.1` (evidence section "T5 - Ruleset Changes Applied"; release branch command and output included).                                                                                                          |
| AC6   | DONE                   | No required-check mechanism is active after T5, so code PR #2529, heavy-CI PR #2541, and documentation-only PRs #2533, #2540, and #2543 merged with no ruleset changed (V5, V6). Maintainer accepted this on 2026-10-15; the required `CI gate` is #2525.                           |
| AC7   | DONE                   | 94237, 19245352, and 87098 deleted; definitions in `evidence/`. No disabled ruleset remains.                                                                                                                                                                                        |
| AC8   | DONE                   | `docs/github-rulesets.md`                                                                                                                                                                                                                                                           |
| AC9   | DONE                   | Issue #2547 exists, and its spec scopes the measurement against [`baseline-evidence.md`](baseline-evidence.md) for 2026-10-15 to 2026-10-28.                                                                                                                                        |

## Risks and Trade-offs

- **Untested combinations.** Two PRs that pass CI separately may break together. Mitigation:
  rebasing stays recommended when cheap; `push` workflows on `develop` test every merged state;
  reviewers may request an update when PRs overlap in Rust code or CI workflows; fix forward or
  revert.
- **"Update branch" merge commits.** They are signed by GitHub's web-flow key rather than the
  author and add `develop` merges to the PR history. The maintainer accepted this; most commits are
  agent-authored anyway, and `required_signatures` accepts them.
- **Required checks block the maintainer merge again.** Re-enabling checks without verifying the
  local-push behavior would recreate the disable-and-forget cycle. Mitigation: T5 verifies first
  and provides an override that does not disable rulesets.
- **Less linear full graph.** Overlapping merge bubbles in `git log --graph`; first-parent history
  is unaffected.
- **Agents trained on the old rule.** Stale instructions in skills or agent profiles would keep
  triggering rebases. Mitigation: T4 searches all instruction sources.

## Implementation Completion Review

- Retrospective: created, [`implementation-retrospective.md`](implementation-retrospective.md)
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related PRs: #2229 (stale merge-base guard), #2320 (review retrospective), #2000 (last merge that
  required disabling ruleset 94237)
- Related issues: #2022 (vendor and document the maintainer merge workflow), #1743
  (documentation-only CI fast path), #2525 (required CI gate follow-up), #2547 (post-change
  measurement follow-up)
- Related ADRs:
  [`20261013161419_stop_requiring_pull_request_rebases.md`](../../../adrs/20261013161419_stop_requiring_pull_request_rebases.md),
  [`20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`](../../../adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md)
- Evidence: [`baseline-evidence.md`](baseline-evidence.md),
  [`manual-verification-evidence.md`](manual-verification-evidence.md)
