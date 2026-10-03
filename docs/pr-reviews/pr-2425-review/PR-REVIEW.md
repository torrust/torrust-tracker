---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2425 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2425>.

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

Copilot review 5400517694 (`COMMENTED`, "Changes recommended", review effort "Balanced") left five inline comments, each with its own finding ID (F1-F5) and severity bracket; the audit keeps those IDs. The review body is an overview only, so it creates no additional finding. The overview's badges rate F2, F4, and F5 `Medium severity` and F1 and F3 `Low severity`; each row records its comment's own bracket, `[Minor]` for F1 and `[Major]` for F2-F5. The replies are recorded in the Processing Log and are posted before the threads are resolved, so every thread is `OPEN` and every `Reply URL` is pending until then.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2425-f1` | Copilot | Minor | metadata | ORIGINAL | FIXED | OPEN |
| F2 | `review-finding:pr-2425-f2` | Copilot | Major | testing | ORIGINAL | FIXED | OPEN |
| F3 | `review-finding:pr-2425-f3` | Copilot | Major | testing | ORIGINAL | FIXED | OPEN |
| F4 | `review-finding:pr-2425-f4` | Copilot | Major | correctness | ORIGINAL | FIXED | OPEN |
| F5 | `review-finding:pr-2425-f5` | Copilot | Major | testing | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - Use the spec-only branch name

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956388>
- Concern: "This PR and D1 define a spec-only branch and reserve the unsuffixed name for implementation, but this metadata currently points to that reserved implementation branch. The spec-only workflow also requires the frontmatter branch to use the same `-spec` name (`.github/skills/dev/planning/create-issue/SKILL.md:285-288`)."
- Solution: Set `branch:` to `"{issue-number}-extend-strict-profiles-and-author-guidance-spec"`. The cited rule holds: the `create-issue` spec-only workflow names the branch with the `-spec` suffix, reserves the unsuffixed name for implementation, and sets the spec's frontmatter `branch:` to the same `-spec` name; D1 already follows that workflow.
- Current-tree verification: `grep -n '^branch:' docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md` prints line 10 with the `-spec` value; `.github/skills/dev/planning/create-issue/SKILL.md:286-288` states the rule.
- Resolution reference: `docs(issues): [#2264] name the spec-only branch in the row 3 draft`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### F2 - Record the stale-guidance reproduction before review

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956345>
- Concern: "This step defers the reproduction to implementation, but the linked bug workflow requires attempting it while drafting, before maintainer review, and classifying the result as `Reproduced`, `Trigger only`, or `Infeasible` in issue-local `manual-verification-evidence.md` (`fix-bug` lines 125-134). The draft directory currently contains only `ISSUE.md`; add the evidence artifact now and update this section with the observed result."
- Solution: Accepted, decided together with F3. The draft invokes `fix-bug` for the stale convention field lists, and the skills support keeping that route rather than marking the bug sections `Not applicable`: `fix-bug` applies whenever work is substantively a bug, including stale or misleading behavior, even when the issue type says otherwise (`.github/skills/dev/debugging/fix-bug/SKILL.md:19-21`); `create-issue` states the same rule (`.github/skills/dev/planning/create-issue/SKILL.md:29-33`); and the ISSUE template allows `Not applicable` only when the work is not substantively a bug (`docs/templates/ISSUE.md:86`). Added issue-local `manual-verification-evidence.md` with section B1, which records two disposable specs written from the convention's list, the validator commands, and the outcome `Infeasible`, because the drafting environment has no Rust toolchain; a field-by-field comparison against `profile.rs` is the substitute evidence. Bug-Fix Process step 2 and a Workflow Checkpoint record the result. Running the recorded commands with the toolchain moves B1 to `Reproduced` in a follow-up commit.
- Current-tree verification: `git ls-files docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/` lists `ISSUE.md` and `manual-verification-evidence.md`; line 27 of the evidence file classifies the outcome as `Infeasible`, and lines 154 and 204 of `ISSUE.md` state it.
- Resolution reference: `docs(issues): [#2264] record the stale-guidance reproduction attempt for row 3`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### F3 - Split the bug regression sequence into explicit tasks

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956425>
- Concern: "T7 combines the stale-guidance fix into one task and does not require a red regression result before the edit or a separate green plus like-for-like recheck afterward. The linked `fix-bug` workflow requires those phases to be explicit in the implementation plan (`.github/skills/dev/debugging/fix-bug/SKILL.md:141-144`), so split T7 or add ordered subtasks and map their evidence before implementation begins."
- Solution: Split T7 into three tasks. T7a adds a maintained crate test that validates every issue and EPIC frontmatter example in the semantic-link convention in the D6 template mode, and records its red run against the unchanged convention in section B2. T7b fixes the convention and the other guidance. T7c records the green run and the like-for-like recheck through M7. The Bug-Fix Process now follows the six-step required sequence of `fix-bug`, and Regression Test Strategy, Commit Points, and AC9 match it.
- Current-tree verification: `grep -n -E '^\| T7[abc] ' docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md` lists T7a, T7b, and T7c at lines 176-178; `.github/skills/dev/debugging/fix-bug/SKILL.md:141-144` states the rule.
- Resolution reference: `docs(issues): [#2264] split the row 3 stale-guidance fix into red, fix, and green tasks`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### F4 - Exempt the template-exclusion test from AC3

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956354>
- Concern: "This criterion is impossible as written: `tests/cli.rs:683-708` currently asserts that `docs/templates/` is skipped in every command mode, while T6 and AC7 require reversing that behavior. Preserve the issue/EPIC outcome tests, but explicitly allow the exclusion test to change into the new template-mode coverage."
- Solution: AC3 now requires only the tests that assert issue and EPIC outcomes to pass unmodified. It names `it_should_skip_excluded_paths_in_every_mode` as the one existing test that changes: its `docs/templates/` case becomes template-mode coverage, while it keeps asserting that the crate fixtures are skipped in every mode. T6 records the change, and the Acceptance Verification row for AC3 names the reworked test.
- Current-tree verification: `contrib/dev-tools/checks/frontmatter-validator/tests/cli.rs:683-708` defines `it_should_skip_excluded_paths_in_every_mode` over both excluded prefixes; lines 175 and 231 of `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md` carry the T6 note and the new AC3.
- Resolution reference: `docs(issues): [#2264] exempt the template-exclusion test from row 3 AC3`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### F5 - Make the advisory-severity scenario trigger a finding

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956362>
- Concern: "Validating the existing directories does not exercise warning conversion: the tracked closed refactor plans have no `schema-version: 1` and therefore remain permissive, while valid closed issue records produce no structural finding. Deliberately introduce one invalid strict-profile field in each applicable historical location so the scenario can prove both warning severity and a successful exit."
- Solution: M6 now copies a record that passes its profile into each historical location that has a strict profile: a closed v1 spec, plus a refactor plan opted into its profile if that profile is approved. It injects one unprefixed unknown field, validates the copy, and expects exactly one `warning` record and exit `0`.
- Current-tree verification: `git grep -l '^schema-version:' -- docs/refactor-plans/closed` prints nothing, which confirms the reviewer's premise; line 268 of `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md` carries the new M6.
- Resolution reference: `docs(issues): [#2264] make the row 3 advisory-severity scenario trigger a finding`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

## Processing Log

- 2026-10-03 11:28 UTC - da2ce7 - Started audit: read review 5400517694 and its five unresolved inline threads from the GraphQL thread data and REST review comments captured at 2026-10-03 11:26 UTC; the PR has no human review and no conversation comment.
- 2026-10-03 11:30 UTC - da2ce7 - Committed the F1 fix.
- 2026-10-03 11:32 UTC - da2ce7 - Committed the F2 fix with issue-local `manual-verification-evidence.md` section B1 (reproduction `Infeasible` in the drafting environment).
- 2026-10-03 11:33 UTC - da2ce7 - Committed the F3, F4, and F5 fixes, one commit each.
- 2026-10-03 11:37 UTC - da2ce7 - Recorded the F1 thread reply for posting: FIXED. The cited rule holds: `.github/skills/dev/planning/create-issue/SKILL.md:286-288` names the spec-only branch `{issue-number}-{short-description}-spec`, reserves the unsuffixed name for implementation, and sets the spec's frontmatter `branch:` to the same `-spec` name. Line 10 of the draft now reads `branch: "{issue-number}-extend-strict-profiles-and-author-guidance-spec"`, matching D1. Resolution: `docs(issues): [#2264] name the spec-only branch in the row 3 draft`.
- 2026-10-03 11:37 UTC - da2ce7 - Recorded the F2 thread reply for posting: FIXED. The draft keeps invoking `fix-bug` for the stale convention field lists, because the skills require it: `fix-bug` applies whenever work is substantively a bug, including stale or misleading behavior, even when the issue type says otherwise (`.github/skills/dev/debugging/fix-bug/SKILL.md:19-21`), and the ISSUE template allows `Not applicable` only when the work is not substantively a bug. The draft folder now holds `manual-verification-evidence.md`. Its section B1 records the attempt made while drafting: two disposable specs written from the convention's list, the exact validator commands, and the outcome **Infeasible**, because the drafting environment has no Rust toolchain. A field-by-field comparison against `profile.rs` is the substitute evidence. Bug-Fix Process step 2 (line 154) and a new Workflow Checkpoint (line 204) state the result. Running the recorded commands with the toolchain moves B1 to **Reproduced** in a follow-up commit. Resolution: `docs(issues): [#2264] record the stale-guidance reproduction attempt for row 3`.
- 2026-10-03 11:37 UTC - da2ce7 - Recorded the F3 thread reply for posting: FIXED. The cited rule holds (`.github/skills/dev/debugging/fix-bug/SKILL.md:141-144`). T7 is now three tasks (lines 176-178): T7a adds a maintained crate test that validates every issue and EPIC frontmatter example in the semantic-link convention in the D6 template mode, and records its red run against the unchanged convention in evidence section B2; T7b fixes the convention and the other guidance; and T7c records the green run and repeats the B1 authoring procedure like-for-like (M7). The Bug-Fix Process now follows the six-step required sequence, and Regression Test Strategy, Commit Points (the test and the fix land together after the red run is recorded) and AC9 match it. Resolution: `docs(issues): [#2264] split the row 3 stale-guidance fix into red, fix, and green tasks`.
- 2026-10-03 11:37 UTC - da2ce7 - Recorded the F4 thread reply for posting: FIXED. Confirmed: `contrib/dev-tools/checks/frontmatter-validator/tests/cli.rs:683-708` (`it_should_skip_excluded_paths_in_every_mode`) asserts that both `docs/templates/` and the crate fixtures are skipped in every mode. AC3 (line 231) now requires only the tests that assert issue and EPIC outcomes to pass unmodified. It names `it_should_skip_excluded_paths_in_every_mode` as the one test that changes: its `docs/templates/` case becomes template-mode coverage, while it keeps asserting the fixtures exclusion. T6 (line 175) records the change. Resolution: `docs(issues): [#2264] exempt the template-exclusion test from row 3 AC3`.
- 2026-10-03 11:37 UTC - da2ce7 - Recorded the F5 thread reply for posting: FIXED. Confirmed: no tracked file under `docs/refactor-plans/closed/` declares `schema-version`, and a valid closed v1 spec yields no structural finding, so the old M6 could not show a warning. M6 (line 268) now copies a passing record into each historical location that has a strict profile: a closed v1 spec, plus a refactor plan opted into its profile if that profile is approved. It injects one unprefixed unknown field, validates the copy, and expects exactly one `warning` record and exit `0`. Resolution: `docs(issues): [#2264] make the row 3 advisory-severity scenario trigger a finding`.

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
