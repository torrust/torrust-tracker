---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2003
github-issue: 3073
spec-path: docs/issues/open/3073-2003-close-text-only-friction-register-rows/ISSUE.md
branch: "3073-close-text-only-friction-register-rows-spec"
related-pr: null
last-updated-utc: "2026-11-26 17:34"
semantic-links:
  skill-links:
    - create-issue
    - add-new-skill
    - cleanup-completed-issues
    - commit-changes
    - fix-bug
    - open-pull-request
    - process-pr-review
    - push-changes
    - reopen-issue
    - resolve-review-threads
    - review-pr
    - run-linters
    - run-pre-commit-checks
    - update-github-workflow-actions
    - write-markdown-docs
    - write-unit-test
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/run-linters/references/linters.md
    - AGENTS.md
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md
    - docs/discussions/AGENTS.md
    - docs/issues/closed/README.md
    - docs/release-process/workspace-packages.md
    - docs/skills/semantic-skill-link-convention.md
    - docs/templates/AGENT-REVIEW-REPORTS.md
    - docs/templates/EPIC.md
    - docs/templates/IMPLEMENTATION-RETROSPECTIVE.md
    - docs/templates/ISSUE.md
    - docs/templates/PR-REVIEW-TEMPLATE.md
    - tests/AGENTS.md
    - "issue #2003"
---

<!-- skill-link: create-issue -->

# Issue #3073 - Close the text-only rows of the EPIC #2003 friction register

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Close every open row of the EPIC #2003 friction register that a text-only edit can close and whose edit restates what the repository already decides: 72 rows in skills, templates and guides, in one implementation pull request with sixteen prepared commits and one evidence commit. No change adds a rule; each one makes a passage say what another passage, the code or settled practice already says.

## Background

The friction register records places where following a repository skill, guide or template literally and doing the work accurately diverged (EPIC specification, section Friction Register). It is kept in comments on #2003; the index comments list every row by its label, and the EPIC specification's Friction Register section links each index comment. A row is `OPEN` until a fix lands (`ADOPTED`), an open specification on `develop` names the fix as a task (`SPECIFIED`), or a later change removes its ground (`SUPERSEDED`).

With every disposition recomputed at `develop` `faf291a82`, 434 rows were `OPEN`:

| Kind of work | Rows |
| ------------ | ---- |
| A text-only edit closes the row | 80 |
| The row needs a tooling change | 66 |
| The row is a question for a maintainer | 286 |
| The friction no longer holds | 2 |

The edit of each of the 80 text-only rows was then checked against the tree at `b72c50218`, where none of the files the edits touch differs from `faf291a82`:

- 56 stand as proposed: the passage, code or practice they cite carries them.
- 17 stand once amended, so that they say no more than that basis (for example, a condition no passage states is dropped, or a sentence keeps "a reviewer" where the proposal narrowed it to one role).
- 6 would choose between two forms the repository already uses, which is a maintainer's decision, not a correction.
- 1 rests on practice that does not hold: recent commits of its kind use other forms than the one it would state.

Of the 73 rows whose edit stands, seven change `.claude/skills/review-pr/SKILL.md`, which pull request #3070 changed before this implementation branch was rebased onto its merge at `develop` `66c12ffe1`. All seven still hold against the merged text. One other row changes the EPIC specification and three specifications the EPIC holds; the remaining 72 rows land in one implementation pull request.

This issue falls under the EPIC's Scope exception for low-risk, additive, independently verifiable documentation, skill and template work: it changes prose and one shell comment, selects no runner, cache, enforcement platform or tool, and stays replaceable by the later design. Its integration point is the existing pre-commit hook and `linter all`.

## Scope

### In Scope

- The 72 rows listed under T1 to T15 in [Rows This Issue Closes](#rows-this-issue-closes), in one implementation pull request with sixteen prepared commits across 30 files, plus the T16 evidence commit and its two files (17 commits and 32 files, 33 if either conditional issue-local file is required, or 34 if both are required): one row commit per file family, plus a separate formatter-path correction.
- One comment line in `contrib/dev-tools/git/hooks/pre-commit.sh`, inside the `STEPS` array, which changes no step. Every other change is Markdown prose.
- For each row, a record of the passage, code or commit its change rests on.
- One comment on #2003, after the implementation merges, that lists each closed label with the file and line on `develop` and the landing commit, so the register index can record it as `ADOPTED`.

### Out of Scope

- The six rows that would choose between forms the repository already uses: `create-issue-commit-subject-example-off-practice`, `commit-changes-body-hash-line-unwarned`, `git-workflow-upstream-remote-idiom-split`, `link-convention-issue-key-loads-null`, `create-issue-completion-review-vs-closing-keyword` and `implementation-retrospective-template-guidance-unmarked`. Each needs a maintainer's choice first.
- `actions-update-commit-subject-fixed-or-illustrative`: the recent Actions-update commits use more than one subject form, so practice does not fix the subject it would state.
- `epic-2003-known-issue-hold-no-exit`: it edits the EPIC's own record and the three paused specifications #1768, #1774 and #1843, and is left to the EPIC owner.
- Repairing `contrib/dev-tools/checks/tests/test-format-project-words.sh`: its project-root path stops at `contrib/`, so fixture copies look under `contrib/contrib/dev-tools/` and the test exits before exercising the formatter. The script repair belongs in a separate issue.
- The 66 rows that need a tooling change and the 286 questions for a maintainer, including text-only pieces that ride on such a row without closing it (for example the list indent of `markdownlint-list-continuity-unchecked`).
- The two rows whose friction no longer holds: `run-pre-commit-checks-doc-build-skips-libraries` and `triage-github-security-findings-sarif-source-unrouted`.
- Changing how the edited passages are wrapped. Each keeps its file's current wrapping; how prose is wrapped is the open question `write-markdown-docs-line-wrap-rule-unstated`, best answered by its own sweep afterwards.
- Editing the register itself. The index is edited in place by whoever files to it; this issue reports what landed.

## Decisions

1. **Amended text lands, not the first proposal.** Where a row below is marked *amended*, the amended edit is the one applied; the note says what the amendment keeps or drops.
2. **Two pairs of rows share lines and land as one hunk each.** `process-pr-review-validator-limit-sentence-inverted` and `audit-validator-concern-unchecked-against-source` both rewrite `.claude/skills/process-pr-review/SKILL.md` lines 321-323: the corrected sentence lands with the clause "nor that a `Concern` states what its source comment reports" added. `create-issue-open-status-value-unnamed` and `create-issue-move-skips-old-path-search` both restate `.claude/skills/create-issue/SKILL.md` lines 300-303 and add sentences that agree: one hunk carries both.
3. **One wording for the lint-level location in `AGENTS.md`.** `run-linters-lint-config-location-stale` and `agents-md-rustflags-location-stale` fix the same `AGENTS.md` lines. The `agents-md-rustflags-location-stale` wording lands, in T6; the duplicate `AGENTS.md` hunk of the other row is dropped.
4. **The commit-id snippet keeps `UPSTREAM_REMOTE=torrust`.** With `git-workflow-upstream-remote-idiom-split` out of scope, the T3 snippet edit keeps the snippet's first line as it is, so no edit undoes another.
5. **No skill version changes.** No edit that stands changes a skill's `metadata.version`: the one that proposed a bump was amended to drop it, because no document requires one and the latest change to that skill kept its version. Whether skills carry a version rule at all is the open question `add-new-skill-no-version-bump-rule`.
6. **The issue-link paragraph of `open-pull-request` does not decide the discussion case.** The T3 paragraph names the `Related to #N` form for a pull request that informs an issue or delivers part of it, without "for example a discussion": the form for a discussion round is the open question `open-pull-request-type-check-vs-spec-scope`, which `review-pr-discussion-round-mode-missing` also leaves open.
7. **Specification first, then one implementation pull request.** The specification-only pull request uses `Related to #3073` and no closing keyword. The implementation pull request delivers all 72 rows and closes the issue. Its commit subjects use `[#3073]`, following `develop` practice.

## Rows This Issue Closes

<!-- cspell:ignore reearned unexcluded unpointed unrouted unsourced unwarned -->

Tick a row when its commit is on the branch. The T1 to T14 line numbers are at `develop` `b72c50218`; the T15 line numbers are at the merged #3070 text on `develop` `66c12ffe1`.

### T1 - Review-audit skill and template (12)

`.claude/skills/process-pr-review/SKILL.md` and `docs/templates/PR-REVIEW-TEMPLATE.md`:

- [ ] `process-pr-review-validator-limit-sentence-inverted` (SKILL.md 321-323): the hand-check sentence names the property rather than the defect: each Processing Log stamp is the time of the event it records and no later than the commit that adds the entry; one hunk with the next row (Decision 2).
- [ ] `audit-validator-concern-unchecked-against-source` (SKILL.md 322-323): the same sentence names a `Concern` that does not state what its source comment reports as another point to check by hand.
- [ ] `process-pr-review-fix-reply-deferred-to-resolution` (SKILL.md 103-104): step 7 replies once the fixes are pushed, in the order of the review loop of `wait-for-pr-review`.
- [ ] `process-pr-review-author-reply-reviews-unexcluded` (SKILL.md 60): step 1 processes every review except one that carries only the author's own thread replies, as the review loop does.
- [ ] `process-pr-review-pr-body-refresh-unruled` (SKILL.md 102), *amended*: step 6 links the Post-creation Validation of `open-pull-request`, under which the body keeps matching the diff and history after each fix; the amendment keeps the rule unconditional.
- [ ] `audit-validator-requires-repository-root` and `audit-validator-comments-file-capture-point-unstated` (SKILL.md 298-301), *amended*: the Validation Script section says to capture a comments file after the last reply a row cites is posted, and to run the script from the root of the checkout whose `HEAD` carries the audited branch; the amendment drops the version change (Decision 5).
- [ ] `process-pr-review-checklist-omits-commit-id-check` (SKILL.md, insert after 378): a checklist item for the commit-id check that `open-pull-request` already requires before each push that changes the audit.
- [ ] `pr-review-template-reviewer-id-placeholder-lacks-na` (template 96): the placeholder takes the `_OR_NA` form of the other `N/A` fields.
- [ ] `pr-review-template-resolution-reference-cell-grammar-unstated` and `pr-review-template-review-id-placeholder-unexplained` (template 105-109): the `Resolution reference` holds references only, several separated by semicolons, and `Source review ID` holds the ID of the submitted review, not a comment ID, written in full in every entry, as the validator reads them.
- [ ] `pr-review-template-log-append-point-ambiguous` (template 117-118): new entries go at the end of the Processing Log, above `## Completion Rules`.

### T2 - Thread resolution (2)

`.claude/skills/resolve-review-threads/SKILL.md`:

- [ ] `resolve-review-threads-resolve-before-push-unchecked` (21-25, 48): a precondition asks that every commit a reply cites is pushed to the pull request, with the command that lists the pushed subjects, and the resolution rule says "pushed to the pull request".
- [ ] `resolve-review-threads-single-resolve-skips-reply-check` (52, 126): resolving one thread first runs the `reply-status` command of the batch pattern and checks the thread's `has_reply`; the checklist item covers a single thread as well as a batch.

### T3 - Pull-request skill (7)

`.claude/skills/open-pull-request/SKILL.md`:

- [ ] `open-pull-request-commit-id-check-blob-stderr`, `open-pull-request-id-check-exit-123-when-clean` and `open-pull-request-commit-id-check-stale-develop` (84-92 inside the fence at 83-93, and 95-101): the snippet fetches `develop` first and discards the error a blob or tree id prints, and the paragraph says that the output, not the exit status, is the result, because `xargs` exits 123 when no changed file holds a hexadecimal token; the snippet keeps its `UPSTREAM_REMOTE=torrust` line (Decision 4).
- [ ] `open-pull-request-link-table-lacks-informing-form` and `open-pull-request-type-table-closes-sliced-issue` (190-193, insert after 244, 293), *amended*: a Related row gives `Related to #N` for a pull request that informs an issue or implements part of it, a paragraph after the type list says when `Closes #N` holds, and the amendment changes the Post-creation Validation item at 293 to pair the keywords the same way. No discussion example (Decision 6).
- [ ] `open-pull-request-closing-keyword-adjective-unwarned` (202): `closed` used as an adjective before an issue reference closes the issue too.
- [ ] `commit-changes-subject-reword-unnamed` (122-124): rewording a commit message that does not describe its diff is named among the allowed history rewrites.

### T4 - Commit skill (4)

`.claude/skills/commit-changes/SKILL.md` and the commit lines of `AGENTS.md`:

- [ ] `commit-changes-hook-step-list-stale` (SKILL.md 140-144): a link to "Automated Checks" of `run-pre-commit-checks`, which lists every hook step in order, replaces the stale three-step copy.
- [ ] `commit-changes-format-project-words-path-stale` (SKILL.md 200-203): the formatter's path is `./contrib/dev-tools/checks/format-project-words.sh`, the one the hook runs.
- [ ] `agents-md-commit-scope-reads-mandatory` and `agents-md-commit-type-list-diverges` (`AGENTS.md` 166-168): a link to `commit-changes`, which states the message structure, the types and when to name a scope, replaces the type list and the scope sentence. Whether `revert` and `build` join that skill's table stays the open question `commit-changes-type-table-omits-revert`.

### T5 - Hook and check skills: copies replaced by links (5)

`.claude/skills/push-changes/SKILL.md`, the hook section of `.claude/skills/commit-changes/SKILL.md`, `.claude/skills/run-pre-commit-checks/SKILL.md` and `contrib/dev-tools/git/hooks/pre-commit.sh`:

- [ ] `git-hook-setup-section-copied-in-four-skills` (commit-changes 113-124, push-changes 40-60): each copied hook-setup section becomes a short paragraph that links "Git Hook (Recommended Setup)" of `run-pre-commit-checks` or `run-pre-push-checks`; each Quick Reference keeps its one-line install command.
- [ ] `pre-push-step-list-copied-in-four-guides` (push-changes 79-90, run-pre-commit-checks 192-197), *amended*: `push-changes` links the step list of `run-pre-push-checks` instead of copying it; the amendment keeps the heading and the recommendation of "Before Opening a PR" in `run-pre-commit-checks` and replaces only its copied commands with a link to the two documentation-build steps.
- [ ] `pre-commit-linter-binary-unexplained` (run-pre-commit-checks 92; `pre-commit.sh`, one comment line before 62 inside `STEPS`): the linter step names the torrust-linting binary and the `install-linter` skill, and the hook carries the same pointer as a comment line, as `pre-push.sh` already does.
- [ ] `run-pre-commit-checks-gate-example-double-run` (106-108): the scripted-gate example applies only when the hook is not installed, and it tests the script's own exit code.
- [ ] `run-pre-commit-checks-git-index-file-unmentioned` (insert after 94), *amended*: the staged checks read the index that `GIT_INDEX_FILE` names, which `git commit --all` and `--only` point at a prepared index, so a check added to the hook keeps that variable set; the amendment decides nothing about other variables a check clears.

### T6 - Lint-level location (2)

`.claude/skills/run-linters/references/linters.md`, `.claude/skills/run-linters/SKILL.md` and the configuration lines of `AGENTS.md`:

- [ ] `run-linters-lint-config-location-stale` (linters.md 17-29, SKILL.md 116): the skill and its reference name `[workspace.lints]` in `Cargo.toml`, which every member crate inherits with `[lints] workspace = true`, and the lint levels it denies; its `AGENTS.md` hunk is dropped (Decision 3).
- [ ] `agents-md-rustflags-location-stale` (`AGENTS.md` 37-39, 60-63): the configuration paragraph and the Code Style item name `[workspace.lints]`, and `.cargo/config.toml` as the home of the aliases only.

### T7 - Issue-spec skills (9)

`.claude/skills/create-issue/SKILL.md` and `.claude/skills/reopen-issue/SKILL.md`:

- [ ] `create-issue-bug-rule-lacks-observed-qualifier` (create-issue 29-33): the bug rule uses the definition of `fix-bug`, which qualifies the behaviour as observed.
- [ ] `create-issue-init-list-omits-conditional-sections` (150-152, and a paragraph after 152): Task, Bug and Feature drafts keep the template's Design and Ownership Review, Bug-Fix Process and Regression Test Strategy sections, filled or marked `Not applicable` as the template instructs.
- [ ] `create-issue-bug-draft-checklist-omits-reproduction` (194-199): the reproduction is attempted while drafting, before review, and its outcome recorded, with a link to "Reproduce Before Review" of `fix-bug`.
- [ ] `create-issue-autolink-check-cross-repo-unstated` (254): the autolink check says to write a reference into another repository as `owner/repo#NUMBER` or a full URL.
- [ ] `create-issue-branch-key-vs-epic-template` (278-281, 324-325): the `branch:` sentences apply to issue specifications; an EPIC specification has no `branch:` field, and the frontmatter validator rejects one.
- [ ] `create-issue-open-status-value-unnamed` and `create-issue-move-skips-old-path-search` (300-303, one hunk; Decision 2): Step 4 names the status values a specification in `open/` may take (`planned`, `in-progress`, `blocked`, `in-review`), links the validator's migration checklist, and asks for the search for live references to the former draft path that every other specification move makes.
- [ ] `closing-keyword-rule-restated-without-scan` (create-issue 58-59 and 335-339; reopen-issue 140), *amended*: each restatement asks for `Related to`, no closing keyword followed by a reference to the issue in the title, body or commit messages, and links the Issue Linking Rules of `open-pull-request`; the amendment bans that pattern, not every closing keyword.
- [ ] `reopen-issue-issue-body-link-timing-unstated` (reopen-issue 68-70): the issue body's link to the `open/` path resolves on `develop` only after the pull request that moves the specification back merges.

### T8 - Specification templates (5)

`docs/templates/ISSUE.md` and `docs/templates/EPIC.md`:

- [ ] `issue-template-retrospective-trigger-mismatch` (ISSUE.md 144): the checkpoint names the three retrospective triggers of the skills and the EPIC template: reusable lessons, material design changes and meaningful deviations from the plan.
- [ ] `issue-template-independent-reviewer-undefined` (ISSUE.md 146): the checkpoint points to the Artifact Ownership table of `docs/agents/orchestration.md`, which names the independent reviewers.
- [ ] `issue-template-progress-log-stamp-source-unstated` (ISSUE.md, between 154 and 156), *amended*: a pointer to "Statements in Your Own Records" of `process-pr-review`, which already covers specification progress logs, without widening it.
- [ ] `issue-template-ac-ticking-authority-unstated` (ISSUE.md 158-160, and a paragraph after the heading), *amended*: a reviewer ticks a verified criterion and the author of the work leaves it unticked; the amendment keeps "a reviewer", with the task reviewer of `review-task` as an example.
- [ ] `epic-template-log-append-point-ambiguous` (EPIC.md 92): a progress-log line is appended at the end of the list, not at the end of the document.

### T9 - Record templates (2)

`docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` and `docs/templates/AGENT-REVIEW-REPORTS.md`:

- [ ] `implementation-retrospective-lesson-scope-unprompted` (IMPLEMENTATION-RETROSPECTIVE.md 42-45), *amended*: Improvements for Future Work asks, for each improvement, for the document that owns the rule it changes and every other document that governs the same work, so the change lands in the owner and the others link to it; the amendment keeps the template's 80-column wrap.
- [ ] `agent-review-reports-vs-branch-commit-id-rule` (AGENT-REVIEW-REPORTS.md 12-13): the opening note restates the `open-pull-request` rule to name pull-request-branch commits by their subject, because commit ids are durable only on `develop`, in tags or in other repositories; the sentence the report-contract check pins stays.

### T10 - Archive skill and the closed-issues index (4)

`.claude/skills/cleanup-completed-issues/SKILL.md` and `docs/issues/closed/README.md`:

- [ ] `cleanup-completed-issues-supplementary-stamp-bump-misplaced` (189-194): Step 3 says that a supplementary file whose frontmatter names the `open/` path is an affected document under Step 4, so its `semantic-links` and `last-updated-utc` are updated.
- [ ] `cleanup-completed-issues-archive-log-entry-placement` (196-199): the archival entry is appended at the end of the specification's Progress Log, in its template's line format. Its residual point on the entry's content stays with the maintainer questions.
- [ ] `cleanup-completed-issues-pr-body-omits-required-items` (292-297), *amended*: the Step 7 example body carries the items `open-pull-request` requires, the issue link as `Related to`, the files touched and the validation performed; the amendment says "the moved spec", which also covers a legacy single-file one.
- [ ] `issues-closed-readme-legacy-skill-paths` (README.md 7, 36, 37): the skill paths are the `.claude/skills/` ones its sibling indexes use.

### T11 - Link convention and the Markdown guide (3)

`docs/skills/semantic-skill-link-convention.md` and `.claude/skills/write-markdown-docs/SKILL.md`:

- [ ] `link-convention-issue-spec-rule-restated-thrice` (convention 28, 301-302), *amended*: the marker table and the maintenance workflow link Issue-spec lifecycle instead of restating it; the amendment names the class as a document that keeps the key, which covers both kinds the rule names, those in the issue's folder and those whose template prescribes the key.
- [ ] `semantic-link-convention-skill-placement-unstated` (convention, insert after 95): an Agent Skill or agent profile nests the block under `metadata`, and the frontmatter validator reads only that nested block in those files.
- [ ] `write-markdown-docs-checklist-requires-optional-frontmatter` (write-markdown-docs 132): the checklist asks for frontmatter where the document type requires it, as the skill's own Frontmatter section does.

### T12 - Actions-update skill (3)

`.claude/skills/update-github-workflow-actions/SKILL.md`:

- [ ] `actions-update-commit-allowlist-condition-conflict` (63): the quick-reference comment includes both allowlists only when step 5 required an organization policy update, as the Delivery Policy and step 9 say.
- [ ] `actions-update-skill-link-step-reads-as-add` (80): step 6 keeps the `skill-link` comment each workflow carries and adds it only where one is missing.
- [ ] `actions-update-pr-text-refresh-unruled` (85), *amended*: step 10 rechecks the pull-request body against the diff and history after any later push; the amendment drops "title", which no rule asks to match.

### T13 - Other skills (3)

`.claude/skills/add-new-skill/SKILL.md`, `.claude/skills/fix-bug/SKILL.md` and `.claude/skills/write-unit-test/SKILL.md`:

- [ ] `add-new-skill-governance-review-unreferenced` (add-new-skill, insert after 163): a pointer to the review cadence of the AI-agent governance ADR, which makes adding, removing or materially changing a skill a review event.
- [ ] `fix-bug-plan-vs-mutate-then-restore` (fix-bug 153-154, 162-163): the plan rule and the pre-review self-check admit mutate-then-restore when the fix already exists, as step 4 of the Required Sequence does.
- [ ] `write-unit-test-stopped-clock-default-misstated` (write-unit-test 461, 469, 471): the stopped clock starts at `Duration::ZERO` only in the clock crate's own tests and at the application start time in a crate that depends on it, and a test that needs the Unix epoch sets it first, as the tests of `tracker-core` and `swarm-coordination-registry` do.

### T14 - Guides, ADR and release notes (4)

`tests/AGENTS.md`, `docs/discussions/AGENTS.md`, `docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md` and `docs/release-process/workspace-packages.md`:

- [ ] `tests-agents-md-structure-tree-stale` (tests/AGENTS.md 181-202): the structure tree lists every file `git ls-files tests` tracks.
- [ ] `discussions-index-registration-step-missing` (discussions/AGENTS.md 42-44): the opening round adds the discussion's row to the Discussions table of `docs/index.md`, as every discussion so far did.
- [ ] `testing-features-adr-build-check-unnamed` (ADR 216-217): names `shipped-features-check` and the workflow step of `.github/workflows/testing.yaml` that runs it, without changing the decision.
- [ ] `workspace-packages-shared-precondition-workspace-only` (73-79), *amended*: the precondition to raise every dependent's version requirement is scoped to a workspace package; the amendment drops the added bullet on dependents in other repositories.

### T15 - Reviewer skill (7)

`.claude/skills/review-pr/SKILL.md`, re-anchored to the text merged by #3070:

- [ ] `review-pr-discussion-round-mode-missing` (insert after 14): a pointer to the discussion rounds of `docs/discussions/AGENTS.md`.
- [ ] `review-pr-description-item-unanchored`, `review-pr-issue-link-forms-omit-related-to` and `review-pr-closes-keyword-vs-spec-state-unchecked` (31-32, one hunk): the description item points to `open-pull-request` for the issue-link forms, `Related to` included, and the closing rule.
- [ ] `review-pr-unwrap-rule-unsourced` (40, 169-170): the unwrap rule is sourced from `handle-errors-in-code`, not from `AGENTS.md`.
- [ ] `review-pr-log-correction-form-unpointed` (insert after 156): a pointer to the review audit's Processing Log correction form in the audit template and `process-pr-review`.
- [ ] `review-pr-ack-not-reearned-after-force-push` (insert after 162), *amended*: a new head needs a new ACK, as `merge-pull-request` says; the amendment scopes the listing command to an ACKed commit that is still an ancestor of the head, and says that pushing the same head again stales nothing.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md` (repository-owned skills are the workflow authority); `docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md`, which T14 corrects for accuracy, as it has been corrected since it merged.
- ADRs to create: none. Every change restates something the repository already decides; none selects among alternatives.

## Design and Ownership Review

Not applicable. The work changes prose and one shell comment; no process, I/O or fixture code changes.

## Bug-Fix Process

Not applicable. Several rows correct stale or misleading text, which the bug rule of `fix-bug` names, but that rule concerns observed behaviour: no program, hook or check behaves differently before or after this work, so there is nothing to reproduce. The documentation corrections of #2347 were handled the same way.

## Regression Test Strategy

Not applicable, for the same reason. The one shell change is a comment, and M2 confirms the hook runs the same steps.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Review-audit skill and template | The 12 T1 rows applied. Verified by `linter all` and the report-contract check, which pins sentences in both files. No dependency. |
| T2 | TODO | Thread resolution | The 2 T2 rows applied. Verified by `linter all`. No dependency. |
| T3 | TODO | Pull-request skill | The 7 T3 rows applied, with Decisions 4 and 6. Verified by `linter all`. No dependency. |
| T4 | TODO | Commit skill | The 4 T4 rows applied. Verified by `linter all`. No dependency; its `commit-changes` hunks and T5's do not overlap. |
| T5 | TODO | Hook and check skills | The 5 T5 rows applied. Verified by `linter all` and M2's comparison of the hook steps. Either order with T4. |
| T6 | TODO | Lint-level location | The 2 T6 rows applied, with Decision 3; a separate final commit corrects the dictionary-formatter path and closes no row. Verified by `linter all`. No dependency. |
| T7 | TODO | Issue-spec skills | The 9 T7 rows applied, with Decision 2. Verified by `linter all`. No dependency. |
| T8 | TODO | Specification templates | The 5 T8 rows applied. Verified by `linter all` and the frontmatter validator of the hook. No dependency. |
| T9 | TODO | Record templates | The 2 T9 rows applied. Verified by `linter all` and the report-contract check. No dependency. |
| T10 | TODO | Archive skill and closed-issues index | The 4 T10 rows applied. Verified by `linter all`. No dependency. |
| T11 | TODO | Link convention and Markdown guide | The 3 T11 rows applied. Verified by `linter all`. No dependency. |
| T12 | TODO | Actions-update skill | The 3 T12 rows applied. Verified by `linter all`. No dependency. |
| T13 | TODO | Other skills | The 3 T13 rows applied. Verified by `linter all`. No dependency. |
| T14 | TODO | Guides, ADR and release notes | The 4 T14 rows applied. Verified by `linter all`. No dependency. |
| T15 | TODO | Reviewer skill | The 7 T15 rows applied in the same implementation pull request, against the text merged by #3070. Verified by `linter all` and M1 on those rows. |
| T16 | TODO | Verification and evidence | M1 to M3 recorded in `manual-verification-evidence.md`; acceptance criteria re-reviewed; `implementation-retrospective.md` included if the completion-review conditions require it, and `agent-review-reports.md` included if an independent reviewer receives this folder-style specification. Depends on T1 to T15. |
| T17 | TODO | Report to the register | One comment on #2003 listing each closed label with its file and line on `develop` and its landing commit. Depends on the implementation merge. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 to T14 | One file family, as listed under its task | One commit per task, after the pre-commit hook passes. The body lists the labels the commit closes and, for each, the basis its change rests on: a passage or code at the implementation branch head (`path:line`), or an earlier revision (`rev:path:line`) or commit that shows the practice. |
| T15 | The reviewer skill | One commit after T1 in the implementation pull request, with the same body. |
| T6 formatter-path correction | The run-linters skill and its reference | One separate commit after T15, last among the sixteen prepared commits. It closes no register row and carries no `Friction:` line. |
| T16 | `manual-verification-evidence.md`, this specification's tracking sections, plus whichever of `implementation-retrospective.md` and `agent-review-reports.md` their conditions require | One `docs(issues)` commit in the implementation pull request. |
| T17 | None | A GitHub comment; no repository change. |

Each commit uses a Conventional Commit message with the narrow affected scope. No task adds tests, so the progressive test-development loop of `create-issue` does not apply.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-close-text-only-friction-register-rows/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2003, and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`
- [ ] After the implementation merge, report each closed label to #2003 with its file, line on `develop` and landing commit (T17)

### Progress Log

- 2026-11-26 15:01 UTC - Torrust Contributors - Drafted from the friction register's text-only rows, with dispositions recomputed at `develop` `faf291a82` and every edit checked against `b72c50218`. The 65 rows of T1 to T14, the 7 rows of T15 and the out-of-scope rows are listed by label.
- 2026-11-26 15:21 UTC - Torrust Contributors - The maintainer approved the specification, including Decision 6. Issue #3073 was created with the label `task` and linked as a sub-issue of #2003 (`parent_issue_url` verified). Moved the specification to `docs/issues/open/3073-2003-close-text-only-friction-register-rows/` on the spec-only branch `3073-close-text-only-friction-register-rows-spec`, set `status: planned`, set `branch:` to that branch and filled in the issue number. Corrected the descriptions of T1 to T14 against the fourteen commits prepared for the first pull request, which change 29 files; the 65 rows, the decisions and the acceptance criteria are unchanged. A search of the tree found no reference to the former draft path. The spec-only pull request comes next.
- 2026-11-26 16:22 UTC - Torrust Contributors - Pull request #3070 merged at `develop` `66c12ffe1`. Re-anchored T15 against the merged reviewer skill; all seven rows still hold. The implementation branch carries 72 rows in sixteen prepared commits across 30 files, including the separate formatter-path correction. Updated the delivery plan and acceptance criteria for one implementation pull request. The earlier entries describe the branch before this merge.
- 2026-11-26 16:46 UTC - Torrust Contributors - Clarified the implementation pull request as sixteen prepared commits plus one evidence commit across 32 files; the review audit correction is scoped to its Processing Log, bases resolve at the branch head unless a revision is named, and the register report follows the merge.
- 2026-11-26 17:12 UTC - Torrust Contributors - Assigned the separate formatter-path correction to T6, allowed an issue-local retrospective when the completion review requires it, and corrected the AC5 evidence wording after checking the T14 discussion-guide change.
- 2026-11-26 17:22 UTC - Torrust Contributors - Ran the dictionary-formatter test; it exited 1 after fixture copies looked under `contrib/contrib/dev-tools/`. T5 now uses `linter all` and M2 for verification, and the test-script repair is outside this issue.
- 2026-11-26 17:34 UTC - Torrust Contributors - Allowed `agent-review-reports.md` in T16 and AC3 when an independent reviewer receives the folder-style specification: 32 files in the base plan, 33 with either conditional issue-local file, and 34 with both.

## Acceptance Criteria

- [ ] AC1: Each of the 72 labels of T1 to T15 is closed by the implementation pull request, in the commit of its task, with the amended text where its row says *amended* and with Decisions 2 to 4 applied.
- [ ] AC2: No change states a rule the repository does not already state. Each commit body names, for every label it closes, the passage, code or commit its change rests on, and M1 finds every changed passage carried by that basis.
- [ ] AC3: The implementation pull request changes only the 30 files named under T1 to T15, plus this specification and its issue-local evidence file under T16 (32 files), `implementation-retrospective.md` if the completion-review conditions require it, and `agent-review-reports.md` if an independent reviewer receives this folder-style specification (33 files with either conditional file, 34 with both). In `contrib/dev-tools/git/hooks/pre-commit.sh` it adds one comment line inside `STEPS` and nothing else, and the hook runs the same steps (M2).
- [ ] AC4: No skill's `metadata.version` changes (Decision 5).
- [ ] AC5: None of the rows under Out of Scope is applied; the commit-id snippet of `open-pull-request` keeps its `UPSTREAM_REMOTE=torrust` line.
- [ ] AC6: The specification-only pull request uses `Related to #3073` without an issue-closing keyword; the implementation pull request closes the issue (Decision 7).
- [ ] AC7: The 7 labels of T15 land in the implementation pull request, each re-anchored to the text merged by #3070 with its basis read again there.
- [ ] `linter all` exits with code `0`
- [ ] T5's hook comment is verified by M2's comparison of the pre-commit steps; no formatter-test result is required for these text-only edits
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh` before each commit, which runs `linter all` and the frontmatter validator on the staged files
- `cargo run --quiet --package agent-review-report-contract` after T1 and T9, which edit files whose sentences it pins; no hook runs it
- Pre-push checks before each push

Record the version of the `linter` binary, and the Rust toolchain of the `cargo` commands, with each result.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Every change rests on its basis | For each label, read the changed passage on the implementation branch next to its commit body's basis; an unqualified `path:line` resolves at the branch head, and an explicit `rev:path:line` resolves at that revision | Each change says nothing its basis does not; a label whose change goes further is reworded before review | TODO | `manual-verification-evidence.md` section V1, one row per label |
| M2 | The hook runs the same steps | Run the pre-commit hook on the commit before T5 and on T5's commit, and compare the printed step lists and exit codes | The same thirteen steps in the same order, both runs exit `0` | TODO | `manual-verification-evidence.md` section V2 |
| M3 | New links lead where they say | Open each changed file on the pushed branch on GitHub and follow every link and anchor the pull request adds | Each opens the heading or passage its sentence names | TODO | `manual-verification-evidence.md` section V3 |

No disposable verification script is planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Ticked rows under Rows This Issue Closes; commit list of the implementation pull request |
| AC2 | TODO | Commit bodies; M1 |
| AC3 | TODO | `git diff --stat develop...HEAD`; `git diff develop...HEAD -- contrib/dev-tools/git/hooks/pre-commit.sh`; M2 |
| AC4 | TODO | `git diff develop...HEAD -- .claude/skills` shows no `version:` line |
| AC5 | TODO | The passages the out-of-scope rows ask about keep their current text at the branch head; in-scope edits may touch the same lines |
| AC6 | TODO | The closing-keyword scan of `open-pull-request` on both pull requests |
| AC7 | TODO | The implementation pull request; M1 rows for T15 |

## Risks and Trade-offs

- **A large documentation pull request.** 72 rows in 30 row-series files, plus the specification and evidence file and either conditional issue-local file if required, is a lot to review. Mitigation: one commit per file family, each readable in one sitting, and each commit body naming its labels and their bases.
- **An open question may later reword a changed passage.** `create-issue-branch-key-vs-epic-template` keeps the `-spec` branch sentence that `create-issue-branch-field-spec-vs-template` may overturn; `discussions-index-registration-step-missing` edits the lines `discussions-round-and-outcome-cases-unruled` asks about; `process-pr-review-validator-limit-sentence-inverted` may need the clock `process-pr-review-stamp-source-not-rebase-stable` chooses. Mitigation: each edit keeps the current text where the question is open and adds nothing on it, so an answer edits it once more rather than undoing it.
- **An accepted ADR changes in place.** T14 corrects `docs/adrs/20261021190318_share_workspace_test_support_through_testing_features.md` for accuracy; whether editorial changes to an accepted ADR are allowed is `create-adr-editorial-change-unruled`. Mitigation: the ADR has been corrected for accuracy since it merged, and the change touches no decision.
- **A tooling row may move a fixed snippet.** If the commit-id check becomes a tool, the three T3 snippet fixes move into it. Mitigation: they are small, and the tool's own specification takes them over.
- **Order with tooling work.** The audit-validator changes edit the failure list of `process-pr-review` that T1 edits, `skills-redirect-stubs-mask-stale-paths` first meets the lines T10 fixes, and `merge-pull-request-stale-base-exit-shared` edits lines next to a T3 hunk. Mitigation: the implementation pull request lands before those changes.
- **The merged reviewer skill has different line numbers.** T15 was re-anchored to the text merged by #3070, and each basis was read again there.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if a row's change could not be carried by its basis as planned, if a decision above changed, or if the work found a reusable lesson; otherwise add a progress-log entry explaining why none was needed.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2003; the register's index comments are linked from its specification, section Friction Register.
- Related PRs: #3070 (changes `.claude/skills/review-pr/SKILL.md`)
- Related issues: #1768, #1774, #1843 (the paused specifications of the row left to the EPIC owner)
