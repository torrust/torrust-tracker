---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
    - docs/issues/open/2525-ci-gate-required-check/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2526 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2526>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Review 5516553001 (round 1, `copilot-pull-request-reviewer`, state `COMMENTED`, submitted
2026-10-14 10:31 UTC on the head whose tip commit was
`docs(issues): [#2507] link the CI gate follow-up issue #2525`,
<https://github.com/torrust/torrust-tracker/pull/2526#pullrequestreview-5516553001>) left four inline
comments bracketed `[Blocker][F1]`, `[Major][F2]`, `[Minor][F5]`, and `[Major][F3]`; the rows keep
those IDs. Its body summarizes them and is not a further finding.

Review 5516589481 (round 1, `da2ce7`, state `CHANGES_REQUESTED`, submitted 2026-10-14 10:34 UTC on
the same head, <https://github.com/torrust/torrust-tracker/pull/2526#pullrequestreview-5516589481>)
left nine inline comments, `F1` to `F9`. Its IDs `F1`, `F2`, `F3`, and `F5` collide with the Copilot
rows, so they are recorded as `F10`, `F11`, `F12`, and `F13`, with the original ID in each detail
entry; `F4` and `F6` to `F9` keep their IDs. Three of them request the same change as an earlier
Copilot row and are recorded as re-raises. The body's per-area summary and "Checked, no finding"
list are not further findings; its note that a missing base printed `None` is handled with F13.

Reviews 5516638354, 5516638647, 5516638880, and 5516639123 (josecelano, `COMMENTED`, 10:39 UTC)
are the containers GitHub created for the replies to review 5516553001, and reviews 5516733580,
5516733881, 5516734204, 5516734623, 5516734988, 5516735345, 5516735687, 5516735962, and 5516736309
(josecelano, `COMMENTED`, 10:47 to 10:48 UTC) the containers for the replies to review 5516589481;
none carries a finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ---------------------------- | ------------ | ---------- | --------------- | -------------- | ----------- | ------------ |
| F1 | `review-finding:pr-2526-f1` | Copilot | Blocker | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2526-f2` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2526-f5` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2526-f3` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2526-f10` | Human | Major | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F11 | `review-finding:pr-2526-f11` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2526-f12` | Human | Minor | documentation | RE_RAISE_OF:F2 | FIXED | RESOLVED |
| F4 | `review-finding:pr-2526-f4` | Human | Minor | documentation | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F13 | `review-finding:pr-2526-f13` | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2526-f6` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2526-f7` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2526-f8` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2526-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The base refresh can change the target of a pull request that targets another branch

- PR number: 2526
- Source review ID: 5516553001
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269646931>
- Concern: The merge target comes from the wrapper (`develop`) or configuration, while the pull request may target `main` or a release branch. Its recorded base then never equals the `develop` tip, so the refresh would send `base=develop`, a real base change, and the run would continue towards merging that pull request into `develop`. The reviewer asked to check `base.ref` against the merge target and exit before any refresh, with a regression test.
- Solution: Reproduced first: a fixture pull request recording `base.ref=main` and a stale base made the tool edit the base and reach the signing step. `require_current_recorded_base` now checks the recorded target before any comparison or GitHub write and exits with code `4` and no edit. Two fixture cases cover a stale and a current recorded base; the README names the refusal.
- Current-tree verification: With the check replaced by `if False:`, both new cases fail; restored, `python3 contrib/dev-tools/git/tests/test-github-merge-symlinks.py` (Python 3.14.4) reports 49 tests OK at the fix commit; `git show HEAD:contrib/dev-tools/git/github-merge.py | grep -c "if recorded_ref != branch:"` returns `1`.
- Resolution reference: `fix(git): [#2507] refuse a pull request that targets another branch before refreshing its base`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269718552>

### F2 - AC3 still requires the stale merge-ref refusal that T2 removes

- PR number: 2526
- Source review ID: 5516553001
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269646999>
- Concern: AC3 required `github-merge.py` to refuse a merge built on a stale GitHub merge ref, which the completed T2 intentionally no longer does, so the specification contradicted its implementation. The comment also pointed at line 195, which at the reviewed head is the Copilot-ruleset row; the other outdated passages are the In Scope bullets.
- Solution: AC3 now requires refusing a pull request that targets another branch without editing it, refreshing a stale recorded base, refusing while it stays stale with the refresh command printed, and only skipping the GitHub-merge comparison on a stale merge ref. The In Scope bullets describe the recorded-base condition found by T1. Progress-log entries stay as historical records.
- Current-tree verification: `git show HEAD:docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md | grep -c "still refuses to sign or push a merge built on a stale GitHub merge"` and `... | grep -c "so the stale-merge-ref failure"` both return `0`.
- Resolution reference: `docs(issues): [#2507] align the scope and AC3 with the recorded-base check`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269718773>

### F5 - AC5 is ticked for a release branch without recorded evidence

- PR number: 2526
- Source review ID: 5516553001
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269647058>
- Concern: The AC5 row claimed a direct check of `releases/v3.0.0-rc.1`, but the cited evidence section held only the `develop` and `main` output.
- Solution: The check had been run, but only its result reached the specification. The evidence section now records the exact query and its output, observed at about 10:00 UTC and again at 10:36 UTC, and the AC5 row cites it.
- Current-tree verification: `git show HEAD:docs/issues/open/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md | grep -c "rules/branches/releases/v3.0.0-rc.1"` returns `1`.
- Resolution reference: `docs(issues): [#2507] record the release-branch ruleset check in the evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269718977>

### F3 - The #2525 override design has no source for cross-run state

- PR number: 2526
- Source review ID: 5516553001
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269647115>
- Concern: A `labeled` or `unlabeled` event starts a new workflow run that cannot read the earlier run's `needs` results, and later events do not identify who applied an existing label, so the planned override cannot both skip the heavy jobs and authorize the labeler. The reviewer asked for a task defining how results keyed by head SHA and label provenance are read, the permissions and trust boundary, and hosted tests for add, remove, and synchronize events.
- Solution: The #2525 specification gains decision gate T5 before the orchestrator: where the gate reads earlier results and label provenance, how it verifies the Maintain or Admin role, the token permissions on fork pull requests and the trust boundary of each input, and whether an override survives a new push, proved with hosted sandbox runs. Later tasks are renumbered, AC3 and the risks reference T5, and scenarios M6 (label removed) and M7 (push after an override) are added.
- Current-tree verification: `git show HEAD:docs/issues/open/2525-ci-gate-required-check/ISSUE.md | grep -c "Decision gate: cross-run state for the override"` returns `1`; `linter all` passed in the fix commit's pre-commit gate.
- Resolution reference: `docs(issues): [#2525] add a decision gate for the override's cross-run state`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269719167>

### F10 - The base refresh can change the target of a pull request that targets another branch

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677822>
- Concern: The same concern as F1, with the wrapper and configuration lines that make the merge target `develop` and the observation that the fixture hard-coded `base.ref=develop`, so no case could reach it.
- Solution: Fixed by the F1 change; the fixture now takes the recorded target branch as a parameter.
- Current-tree verification: As for F1.
- Resolution reference: `fix(git): [#2507] refuse a pull request that targets another branch before refreshing its base`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269797956>

### F11 - The ADR still names the merge ref in its required-update case, alternatives, and description

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677839>
- Concern: After the corrected constraint paragraph, the ADR's third required-update case, alternatives B and C, and its Description still named the merge ref, and `open-pull-request/SKILL.md` restated the case as "reports a stale merge base".
- Solution: Case 3 now reads "the pull request's recorded base is stale and the maintainer merge tool's own refresh is unavailable"; alternatives B and C and the transition note name the recorded base; the Description marks the merge-ref refusal as history. The skill's case 3 matches in a separate commit, `docs(skills): [#2507] name the recorded base as the merge precondition for a required update`.
- Current-tree verification: The two remaining "merge ref" lines of the ADR at HEAD are the marked history and the evidence sentence that a stale merge ref does not block; `git show HEAD:.github/skills/dev/git-workflow/open-pull-request/SKILL.md | grep -c "reports a stale merge base"` returns `0`.
- Resolution reference: `docs(adrs): [#2507] restate the remaining merge-ref passages around the recorded base`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269798220>

### F12 - AC3 and the Background still describe the merge-ref condition

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677847>
- Concern: The same AC3 and In Scope concern as F2, adding that the Background paragraph carried the superseded condition.
- Solution: AC3 and the In Scope bullets were fixed for F2. The Background keeps what was believed when the specification was written and now carries a "Correction from T1" naming `base.sha` as the condition.
- Current-tree verification: `git show HEAD:docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md | grep -c "Correction from T1"` returns `1`.
- Resolution reference: `docs(issues): [#2507] correct the background on the merge-tool constraint`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269798542>

### F4 - AC5 is ticked for a release branch without recorded evidence

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677861>
- Concern: The same concern as F5, marked not blocking because the recorded ruleset configuration supports the claim.
- Solution: Fixed by the F5 change.
- Current-tree verification: As for F5.
- Resolution reference: `docs(issues): [#2507] record the release-branch ruleset check in the evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269798877>

### F13 - A failed base refresh hides the reason and has no timeout

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677868>
- Concern: The `CalledProcessError` message did not include the error text the GitHub CLI wrote to stderr, so an authentication or permission failure gave no reason, and without a timeout a hung `gh` would stall the tool before its first prompt. The review summary also noted that a missing base printed `None`.
- Solution: A failed edit now prints the sanitized stderr of `gh`, the call is bounded by `GH_TIMEOUT_SECONDS = 60` and reports "gh timed out", and an unknown recorded base is named as "an unknown commit". Two `gh` request cases cover the reason and the timeout.
- Current-tree verification: Replacing the stderr use with `str(e)` fails `it_should_report_the_reason_gh_gives_when_the_edit_fails`; removing the timeout fails `it_should_give_up_when_gh_does_not_answer_in_time`; restored, 51 tests OK (Python 3.14.4).
- Resolution reference: `fix(git): [#2507] report why a base refresh failed and bound it with a timeout`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269799173>

### F6 - Progress-log stamps post-date their commits

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677876>
- Concern: The 06:52 and 07:30 entries post-dated their commits (06:47:59Z and 07:24:42Z), and the evidence frontmatter stamp 10:05 post-dated its commit (10:01:24Z).
- Solution: The two entries now read 06:47 and 07:24. The evidence stamp had already been superseded by 10:37, written with the 10:37 evidence commit, and later updates keep it current.
- Current-tree verification: `grep -n "^- 2026-10-14 0[67]"` on the specification at HEAD lists only the 06:47 and 07:24 entries.
- Resolution reference: `docs(issues): [#2507] correct two more progress-log stamps that post-date their commits`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269799483>

### F7 - The T1 evidence still reads pending after the decision

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677892>
- Concern: Two T1 status lines in the evidence still said the method was pending maintainer approval.
- Solution: Both lines now name the decision: option (e), run by the merge tool itself, chosen on 2026-10-14 and implemented by T2.
- Current-tree verification: `git show HEAD:docs/issues/open/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md | grep -c "pending maintainer approval"` returns `0`.
- Resolution reference: `docs(issues): [#2507] name the chosen refresh method in the T1 evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269799753>

### F8 - Two target-state rows still read as open choices

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677899>
- Concern: The 19245352 and 87098 rows of the ruleset target state still offered choices after both rulesets were deleted.
- Solution: Both rows now state the applied deletion and its reason.
- Current-tree verification: `git show HEAD:docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md | grep -c "Maintainer decides: enable it"` returns `0`.
- Resolution reference: `docs(issues): [#2507] record the applied decisions in the ruleset target state`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269799962>

### F9 - The saved ruleset JSON is not the output of the recorded command

- PR number: 2526
- Source review ID: 5516589481
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269677907>
- Concern: The evidence showed a bare `gh api ... > evidence/deleted-ruleset-<id>.json`, but the files lack response metadata, have reordered keys, and the created ruleset has no `updated_at`.
- Solution: The recorded commands now include the `--jq` filters that produced the files and the Prettier formatting step, and step 2 names the dropped metadata fields.
- Current-tree verification: `git show HEAD:docs/issues/open/2507-stop-requiring-pull-request-rebases/manual-verification-evidence.md | grep -c "jq '{id,name,target"` returns `2`.
- Resolution reference: `docs(issues): [#2507] record the filter that produced the saved ruleset definitions`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2526#discussion_r4269800240>

## Processing Log

- 2026-10-14 10:31 UTC - Review 5516553001 (`copilot-pull-request-reviewer`, `COMMENTED`) left
  inline comments 4269646931, 4269646999, 4269647058, and 4269647115.
- 2026-10-14 10:34 UTC - Review 5516589481 (`da2ce7`, `CHANGES_REQUESTED`) left nine inline
  comments; it was noticed only after the round-1 Copilot fixes, when the comment list was fetched
  again for this audit.
- 2026-10-14 10:35 UTC - Committed the F1 fix after seeing its regression cases fail first.
- 2026-10-14 10:36 UTC - Committed the F2 fix.
- 2026-10-14 10:37 UTC - Committed the F5 fix after re-running the release-branch query.
- 2026-10-14 10:38 UTC - Committed the F3 fix, pushed the four fixes, and posted the four Copilot
  replies by 10:39.
- 2026-10-14 10:43 UTC - Committed the F13 fix after seeing its two cases fail first.
- 2026-10-14 10:44 UTC - Committed the F11 fixes (ADR and skill) and the F12 Background correction.
- 2026-10-14 10:47 UTC - Committed the F6, F7, F8, and F9 fixes between 10:45 and 10:47.
- 2026-10-14 10:48 UTC - Pushed, re-verified the fixes against the pushed head, and posted the nine
  replies to review 5516589481, the F10, F12, and F4 replies naming the earlier fixes.
- 2026-10-14 10:49 UTC - Recorded all thirteen rows `FIXED`/`RESOLVED` and ran
  `validate-audit-record.py`.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
