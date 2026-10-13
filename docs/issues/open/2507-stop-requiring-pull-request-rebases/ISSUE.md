---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p1
epic: null
github-issue: 2507
spec-path: docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
branch: "2507-stop-requiring-pull-request-rebases-spec"
related-pr: 2508
last-updated-utc: "2026-10-13 17:30"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - open-pull-request
    - merge-pull-request
    - implement-workflow
  related-artifacts:
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/baseline-evidence.md
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
- Find the least disruptive way to refresh a stale GitHub merge ref before a maintainer merge
  (decision gate T1). GitHub's "Update branch" merge commit is an accepted fallback.
- Update `github-merge.py`, its tests, and `README-github-merge.md` so the stale-merge-ref failure
  recommends the chosen refresh instead of always "Rebase the pull request branch".
- Update the `open-pull-request` and `merge-pull-request` skills, and `AGENTS.md`, to the
  recommended-not-required rebase policy.
- Reconcile the repository rulesets: restore the force-push ban, redesign or remove required
  status checks so that they are satisfiable for code and documentation-only PRs and compatible
  with the maintainer merge, decide the Copilot review ruleset, delete rulesets that are no longer
  needed, and document the result.
- Re-measure force-pushes and PR CI time after the change.

### Out of Scope

- Replacing `github-merge.py` with GitHub's merge button or a merge queue (ADR option D).
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

| ID  | Status      | Task                                                  | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                                                                                                |
| --- | ----------- | ----------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO        | Decision gate: find a non-rewriting merge-ref refresh | During normal maintainer merges of behind, conflict-free PRs, try in order: (a) wait and re-query; (b) close and reopen the PR; (c) GitHub "Update branch" (merge commit signed by GitHub). For each, record whether `refs/pull/<n>/merge^1` becomes the `develop` tip and whether the `github-merge.py` push is accepted. Choose the first reliable method; fall back to a rebase only if none works. Maintainer approves the choice. |
| T2  | TODO        | Update `github-merge.py` stale-merge-ref recovery     | Guard still refuses a stale merge ref; message names the chosen refresh first and a rebase as an alternative. Fixture test asserts the new message. `README-github-merge.md` updated.                                                                                                                                                                                                                                                  |
| T3  | IN_PROGRESS | Update the `merge-pull-request` skill                 | Spec PR #2508 already replaced the pre-emptive "rebase immediately before the interactive merge" step with "refresh only when the tool reports a stale merge base (rebase or Update branch)". Remaining: document the T1 refresh procedure; keep `GH013` diagnosis accurate; document how to merge a known-good PR whose checks failed for an unrelated reason without disabling a ruleset (per T5).                                   |
| T4  | IN_PROGRESS | Update the `open-pull-request` skill and `AGENTS.md`  | Spec PR #2508 already replaced the mandatory rebase pre-flight and section with the ADR recommendation and aligned `AGENTS.md` (Copilot review finding F1). Remaining: verify the wording after T1/T2 and search `.github/agents/` and other skills for remaining "always rebase" instructions.                                                                                                                                        |
| T5  | TODO        | Reconcile repository rulesets                         | See [Ruleset target state](#ruleset-target-state). Maintainer applies the changes; evidence records before/after `gh api` output and one real merge of a code PR and of a documentation-only PR under the new rulesets.                                                                                                                                                                                                                |
| T6  | TODO        | Document the ruleset configuration                    | Add a short, version-controlled description of each active ruleset and why it exists (location decided during implementation, for example a section in `docs/release_process.md` or a new `docs/github-repository-settings.md`), so a future disabled or obsolete rule is detectable by comparison.                                                                                                                                    |
| T7  | TODO        | Post-change measurement                               | After at least two weeks under the new policy, rerun the baseline commands for force-pushes per merged PR, PR CI runner-hours, and rebase-fallout commits; record the comparison in `post-change-evidence.md`.                                                                                                                                                                                                                         |

### Ruleset Target State

T5 proposes, verifies, and applies the following. Each item needs maintainer approval because the
settings are outside Git.

| Ruleset                                   | Proposed action                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                     |
| ----------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| 94237 force-push ban (`non_fast_forward`) | Move into its own active ruleset (or into 87085) so it is never disabled together with status checks. Fork PR branches are unaffected.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                              |
| 94237 required status checks              | Do not re-enable as is. Choose one design: **(a)** a single always-reporting gate check (for example a job in `docs-lint.yaml`, which runs for every PR, that succeeds when the heavy workflows succeeded or were legitimately skipped by the documentation-only path filter), required with `strict_required_status_checks_policy: false`; or **(b)** no GitHub-required checks, with `github-merge.py` verifying the PR head's check results before signing and offering an explicit, logged override. First verify whether GitHub accepts a locally pushed merge commit under a required-checks rule at all; if it does not, only (b) is viable. |
| 94237 remainder                           | Delete after the replacement is active.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                             |
| 19245352 Copilot code review              | Maintainer decides: enable it so every PR, including external ones, gets a Copilot review independently of personal settings (consider `review_on_push: false` to avoid a new review on every rebase or fix push), or delete it and keep relying on personal settings. Confirm no organization-level ruleset already does this.                                                                                                                                                                                                                                                                                                                     |
| 87098 "Require Deployment: coverage"      | Active but has no rules; delete unless a reason is found.                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                           |
| Temporary overrides                       | Replace "disable the ruleset" with a mechanism that cannot be forgotten: a bypass entry scoped to the maintainer role for design (a), or the merge-tool override for design (b).                                                                                                                                                                                                                                                                                                                                                                                                                                                                    |

## Commit Points

| Task | Coherent change set                                                                 | Commit policy                                                                      |
| ---- | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------- |
| T1   | `manual-verification-evidence.md` with the experiment results; ADR update if needed | Commit after the maintainer approves the chosen refresh method.                    |
| T2   | `github-merge.py`, its fixture test, `README-github-merge.md`                       | Commit after the merge-tool tests pass.                                            |
| T3   | `merge-pull-request` skill                                                          | Commit after `linter all` passes.                                                  |
| T4   | `open-pull-request` skill, `AGENTS.md`, and any other aligned instructions          | Commit after `linter all` passes.                                                  |
| T5   | Gate job or merge-tool check (if chosen), plus ruleset evidence                     | One commit per code change; commit evidence after the maintainer applies settings. |
| T6   | Ruleset documentation                                                               | Commit after `linter all` passes.                                                  |
| T7   | `post-change-evidence.md`                                                           | Commit after the measurement window; may land after the implementation PR.         |

Record a justified no-change decision in the task's evidence without creating an empty commit.
Use Conventional Commits with the narrow affected scope (`fix(git)`, `ci(workflows)`,
`docs(skills)`, `docs(issues)`), and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/stop-requiring-pull-request-rebases/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, merge-tool tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
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

## Acceptance Criteria

- [ ] AC1: No skill, agent profile, or `AGENTS.md` instruction makes a rebase mandatory merely
      because `develop` advanced; the recommendation and the required cases match the ADR.
- [ ] AC2: A behind, conflict-free PR can be merged with `merge-pull-request.sh` without rewriting
      its commits (T1 refresh), or with a rebase only when T1 finds no other method.
- [ ] AC3: `github-merge.py` still refuses to sign or push a merge built on a stale GitHub merge
      ref, and its message recommends the T1 refresh.
- [ ] AC4: Commit ids of a PR merged without rebase remain reachable from `develop`.
- [ ] AC5: The force-push ban is active on `develop`, `main`, and `releases/*`.
- [ ] AC6: Every active required-check mechanism can be satisfied by both a code PR and a
      documentation-only PR, and both were merged under it without disabling any ruleset.
- [ ] AC7: Rulesets 94237, 19245352, and 87098 are each either active with a documented purpose
      or deleted; no ruleset is left disabled without a recorded reason.
- [ ] AC8: The active ruleset configuration is documented in the repository (T6).
- [ ] AC9: Post-change force-pushes per merged PR and PR CI runner-hours are measured and compared
      with the baseline.
- [ ] `linter all` exits with code `0`
- [ ] Merge-tool tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py` (Python 3 runtime)
- `bash contrib/dev-tools/git/tests/test-merge-pull-request.sh`
- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
- If a gate job is added: `yamllint -c .yamllint-ci.yml` and the `implement-workflow` skill checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                     | Human-oriented command/steps                                                                                                                       | Expected Result                                                            | Status | Evidence                                     |
| --- | -------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Stale merge ref is detected                  | With a behind PR whose `refs/pull/<n>/merge^1` is not the `develop` tip, run `./contrib/dev-tools/git/merge-pull-request.sh <n>`.                  | Tool exits before signing with the new recovery message.                   | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Merge a behind PR without rewriting commits  | Apply the T1 refresh, confirm `git log -1 --format=%p` of the fetched merge ref starts with the `develop` tip, run the merge tool, sign, and push. | Push accepted; PR shows as merged; reviewed commits keep their ids.        | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Reviewed ids survive the merge               | After M2, run `git merge-base --is-ancestor <reviewed head id> torrust/develop`.                                                                   | Exit code `0`.                                                             | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | Conflicting PR follows the rebase path       | Follow the updated `open-pull-request` skill for a PR GitHub reports as conflicting.                                                               | Skill directs a rebase with `--force-with-lease`; PR becomes mergeable.    | TODO   | `manual-verification-evidence.md` section V4 |
| M5  | Code PR merges under the new rulesets        | Merge a real code PR with the maintainer tool after T5 is applied.                                                                                 | Push accepted without disabling any ruleset.                               | TODO   | `manual-verification-evidence.md` section V5 |
| M6  | Documentation-only PR merges under new rules | Merge a real Markdown-only PR with the maintainer tool after T5 is applied.                                                                        | Push accepted without disabling any ruleset; heavy workflows stay skipped. | TODO   | `manual-verification-evidence.md` section V6 |
| M7  | Force-push ban applies to `develop`          | Run `gh api repos/torrust/torrust-tracker/rules/branches/develop` and the same for `main`.                                                         | A `non_fast_forward` rule is listed for both branches.                     | TODO   | `manual-verification-evidence.md` section V7 |
| M8  | First-parent history unchanged               | `git log --first-parent --oneline -10 torrust/develop` after several merges under the new policy.                                                  | One merge commit per PR in merge order.                                    | TODO   | `manual-verification-evidence.md` section V8 |

Notes:

- These scenarios are executed during real maintainer merges; no throwaway PRs are required.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The baseline and post-change statistics use the `gh` and `git` commands recorded in
[`baseline-evidence.md`](baseline-evidence.md).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |

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

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory.
- If no retrospective is needed, add a concise progress-log entry explaining why.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related PRs: #2229 (stale merge-base guard), #2320 (review retrospective), #2000 (last merge that
  required disabling ruleset 94237)
- Related issues: #2022 (vendor and document the maintainer merge workflow), #1743
  (documentation-only CI fast path)
- Related ADRs:
  [`20261013161419_stop_requiring_pull_request_rebases.md`](../../../adrs/20261013161419_stop_requiring_pull_request_rebases.md),
  [`20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md`](../../../adrs/20260926142648_adopt_self_hosted_hetzner_runner_for_container_tests.md)
- Evidence: [`baseline-evidence.md`](baseline-evidence.md)
