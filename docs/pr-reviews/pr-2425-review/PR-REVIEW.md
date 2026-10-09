---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md
---

<!-- skill-link: process-pr-review -->

<!-- cspell:ignore unsuffixed -->

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

Copilot review 5400517694 (`COMMENTED`, "Changes recommended", review effort "Balanced") left five inline comments, each with its own finding ID (F1-F5) and severity bracket; the audit keeps those IDs. The review body is an overview only, so it creates no additional finding. The overview's badges rate F2, F4, and F5 `Medium severity` and F1 and F3 `Low severity`; each row records its comment's own bracket, `[Minor]` for F1 and `[Major]` for F2-F5. The replies recorded in the Processing Log were posted on their threads before the threads were resolved; each `Reply URL` names the posted reply.

Maintainer review 5468099277 by josecelano (`CHANGES_REQUESTED`, 2026-10-09 09:12 UTC) left seven inline comments with finding IDs R1-R7, requesting R1-R3 and leaving R4-R7 optional; each row records its comment's bracket, `[Minor]` for R1-R3, `[Suggestion]` for R4-R5, and `[Nit]` for R6-R7. Its review body records the maintainer's verification and that the maintainer decisions requested in the PR body stay open, so it creates no additional finding. Rows R1-R7 record the thread state after their replies, recorded in the Processing Log, are posted and the threads resolved; their `Reply URL` fields are pending until then.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2425-f1` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2425-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2425-f3` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2425-f4` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2425-f5` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| R1 | `review-finding:pr-2425-r1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| R2 | `review-finding:pr-2425-r2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| R3 | `review-finding:pr-2425-r3` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| R4 | `review-finding:pr-2425-r4` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| R5 | `review-finding:pr-2425-r5` | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |
| R6 | `review-finding:pr-2425-r6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| R7 | `review-finding:pr-2425-r7` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Use the spec-only branch name

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956388>
- Concern: "This PR and D1 define a spec-only branch and reserve the unsuffixed name for implementation, but this metadata currently points to that reserved implementation branch. The spec-only workflow also requires the frontmatter branch to use the same `-spec` name (`.github/skills/dev/planning/create-issue/SKILL.md:285-288`)."
- Solution: Set `branch:` to `"{issue-number}-extend-strict-profiles-and-author-guidance-spec"`. The cited rule holds: the `create-issue` spec-only workflow names the branch with the `-spec` suffix, reserves the name without the suffix for the implementation branch, and sets the spec's frontmatter `branch:` to the same `-spec` name; D1 already follows that workflow.
- Current-tree verification: `grep -n '^branch:' docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md` prints line 10 with the `-spec` value; `.github/skills/dev/planning/create-issue/SKILL.md:286-288` states the rule.
- Resolution reference: `docs(issues): [#2264] name the spec-only branch in the row 3 draft`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173070969>

### F2 - Record the stale-guidance reproduction before review

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956345>
- Concern: "This step defers the reproduction to implementation, but the linked bug workflow requires attempting it while drafting, before maintainer review, and classifying the result as `Reproduced`, `Trigger only`, or `Infeasible` in issue-local `manual-verification-evidence.md` (`fix-bug` lines 125-134). The draft directory currently contains only `ISSUE.md`; add the evidence artifact now and update this section with the observed result."
- Solution: Accepted, decided together with F3. The draft invokes `fix-bug` for the stale convention field lists, and the skills support keeping that route rather than marking the bug sections `Not applicable`: `fix-bug` applies whenever work is substantively a bug, including stale or misleading behavior, even when the issue type says otherwise (`.github/skills/dev/debugging/fix-bug/SKILL.md:19-21`); `create-issue` states the same rule (`.github/skills/dev/planning/create-issue/SKILL.md:29-33`); and the ISSUE template allows `Not applicable` only when the work is not substantively a bug (`docs/templates/ISSUE.md:86`). Added issue-local `manual-verification-evidence.md` with section B1, which records two disposable specs written from the convention's list, the validator commands, and the outcome `Infeasible` while drafting, because the drafting environment has no Rust toolchain, with a field-by-field comparison against `profile.rs` as the substitute evidence. The recorded commands then ran unchanged in a Linux container with the workspace nightly toolchain and printed exactly the predicted records, so the follow-up commit `docs(issues): [#2264] record the reproduced stale-guidance run for row 3` moved B1 to `Reproduced`. Bug-Fix Process step 2 and a Workflow Checkpoint record the result.
- Current-tree verification: `git ls-files docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/` lists `ISSUE.md` and `manual-verification-evidence.md`; line 27 of the evidence file classifies the outcome as `Reproduced`, its Observed Result holds both records with exit `1`, and lines 154 and 204 of `ISSUE.md` state it.
- Resolution reference: `docs(issues): [#2264] record the stale-guidance reproduction attempt for row 3`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071020>

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
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071064>

### F4 - Exempt the template-exclusion test from AC3

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956354>
- Concern: "This criterion is impossible as written: `tests/cli.rs:683-708` currently asserts that `docs/templates/` is skipped in every command mode, while T6 and AC7 require reversing that behavior. Preserve the issue/EPIC outcome tests, but explicitly allow the exclusion test to change into the new template-mode coverage."
- Solution: AC3 now requires only the tests that assert issue and EPIC outcomes to pass unmodified. It names `it_should_skip_excluded_paths_in_every_mode` as the one existing test that changes: its `docs/templates/` case becomes template-mode coverage, while it keeps asserting that the crate fixtures are skipped in every mode. T6 records the change, and the Acceptance Verification row for AC3 names the reworked test.
- Current-tree verification: `contrib/dev-tools/checks/frontmatter-validator/tests/cli.rs:683-708` defines `it_should_skip_excluded_paths_in_every_mode` over both excluded prefixes; lines 175 and 232 of `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md` carry the T6 note and the new AC3.
- Resolution reference: `docs(issues): [#2264] exempt the template-exclusion test from row 3 AC3`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071118>

### F5 - Make the advisory-severity scenario trigger a finding

- PR number: 2425
- Source review ID: 5400517694
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4172956362>
- Concern: "Validating the existing directories does not exercise warning conversion: the tracked closed refactor plans have no `schema-version: 1` and therefore remain permissive, while valid closed issue records produce no structural finding. Deliberately introduce one invalid strict-profile field in each applicable historical location so the scenario can prove both warning severity and a successful exit."
- Solution: M6 now copies a record that passes its profile into each historical location that has a strict profile: a closed v1 spec, plus a refactor plan opted into its profile if that profile is approved. It injects one unprefixed unknown field, validates the copy, and expects exactly one `warning` record and exit `0`.
- Current-tree verification: `git grep -l '^schema-version:' -- docs/refactor-plans/closed` prints nothing, which confirms the reviewer's premise; line 269 of `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md` carries the new M6.
- Resolution reference: `docs(issues): [#2264] make the row 3 advisory-severity scenario trigger a finding`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071188>

### R1 - Remove the triaged draft from the unordered proposals list

- PR number: 2425
- Source review ID: 5468099277
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4228520177>
- Concern: "This section is defined as "Drafts proposed for this EPIC but not yet ordered in the table above", and whoever triages a draft places it in the table, merges it, or rejects it. This PR places the draft as row 2.3 but leaves it in the unordered list with a "Triaged on 2026-10-03" note, so the draft now appears both ordered and unordered." (full comment at the Source URL)
- Solution: Removed the entry from the EPIC's Draft Subissue Proposals list, which now reads "None pending.", and moved the triage reason into the 2026-10-03 Progress Log entry that records the triage, wrapped at the file's width like its neighbors.
- Current-tree verification: In `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md`, section "Draft Subissue Proposals" holds only "None pending.", and the Progress Log entry beginning "2026-10-03 10:56 UTC - da2ce7 - Triaged the multiple-related-PRs draft" carries the reason, wrapped within 100 columns.
- Resolution reference: `docs(issues): [#2264] order the row 2.3 draft once and align it with rows 3 and 4`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### R2 - Align the row 2.3 / row 3 relationship with the EPIC rationale

- PR number: 2425
- Source review ID: 5468099277
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4228520188>
- Concern: "This risk says "Neither issue depends on the other." The EPIC triage note in this PR justifies ordering row 2.3 before row 3 because doing so "lets the contract change settle before more profiles are built on the same model". That is at least a soft ordering dependency." (full comment at the Source URL)
- Solution: The order is a real but soft dependency, so the EPIC rationale stays and the draft's risk now states it: the entry "Ordering after EPIC row 2.3" says row 3 should preferably start after row 2.3 settles the issue profile, because both rows change `profile.rs` and the generated schema and D2 assumes the new profiles can join `schema-version: 1`; it is soft because none of row 3's profiles reads `related-pr`, so if row 2.3 is `BLOCKED` on row 4, row 3 proceeds. Out of Scope now points to that risk.
- Current-tree verification: In `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md`, Risks and Trade-offs holds "Ordering after EPIC row 2.3", Out of Scope points to it, and the phrase "Neither issue depends" no longer occurs.
- Resolution reference: `docs(issues): [#2264] order the row 2.3 draft once and align it with rows 3 and 4`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### R3 - D4 candidates assume `doc-type` values that templates do not emit

- PR number: 2425
- Source review ID: 5468099277
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4228520197>
- Concern: "D4 describes `implementation-retrospective`, `agent-review-reports`, and `security-report` as "produced by their templates". On `develop`, `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`, `AGENT-REVIEW-REPORTS.md`, and `SECURITY-REPORT.md` declare no `doc-type`. This spec's own baseline row says only 4 templates do: issue, epic, manual-verification-evidence, and refactor-plan. No tracked record uses `doc-type: agent-review-reports` or `doc-type: security-report`, and only 2 use `implementation-retrospective`." (full comment at the Source URL)
- Solution: D4 now states that only four templates declare a `doc-type` today and marks `adr`, `implementation-retrospective`, `agent-review-reports`, and `security-report` as proposed new values that their templates must start emitting (T4, AC8). T1 and AC1 now record a disposition for every file in `docs/templates/`, naming the templates D4 does not, and T4 and AC8 cover templates that emit no `doc-type` today.
- Current-tree verification: In `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md`, D4 begins its candidate list with "Only four templates declare a `doc-type` today", and the T1 row names `SECURITY-ANALYSIS.md`, `PR-REVIEW-RETROSPECTIVE.md`, `REVIEW-FINDINGS.md`, and `DISCUSSION.md`.
- Resolution reference: `docs(issues): [#2264] mark the D4 doc-types as proposed and pin the placeholder syntax`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### R4 - Say what happens to row 2.3 if it depends on row 4

- PR number: 2425
- Source review ID: 5468099277
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4228520206>
- Concern: "Row 2.3 is ordered before row 4 but may depend on row 4's versioning and migration policy if it needs a new `schema-version`. Consider stating the consequence here: in that case, row 2.3 moves after row 4, or is marked `BLOCKED` until row 4 is done. Otherwise the table order and the dependency can contradict each other." (full comment at the Source URL)
- Solution: Row 2.3's expected output now says that if it needs a new `schema-version`, it is marked `BLOCKED` until row 4 is done while row 3 proceeds without it; a new EPIC Progress Log entry records the change.
- Current-tree verification: In `docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md`, the Progressive Subissues row 2.3 ends with "is then marked `BLOCKED` until row 4 is done while row 3 proceeds without it".
- Resolution reference: `docs(issues): [#2264] order the row 2.3 draft once and align it with rows 3 and 4`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### R5 - Pin the enumerated-placeholder syntax for template mode

- PR number: 2425
- Source review ID: 5468099277
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4228520212>
- Concern: "Templates do not use one format for enumerated placeholders. `docs/templates/ISSUE.md` uses `<task|bug|feature|enhancement>`, while `docs/templates/SECURITY-REPORT.md` uses `<fixed | hardened | declined | non-affecting>`, with spaces around `|`. Since D6 compares enumerated placeholders with the profile's allowed values, consider defining the accepted placeholder syntax, including whether whitespace is allowed, as part of D6 or the T1 catalog. Then template mode either normalizes the format or reports the drift." (full comment at the Source URL)
- Solution: D6 now pins the accepted enumerated-placeholder syntax to the form `docs/templates/ISSUE.md` writes (values in angle brackets, separated by `|` with no surrounding whitespace, each listed once, in any order) and makes template mode report other spellings, such as the spaced form in `docs/templates/SECURITY-REPORT.md`, as drift rather than normalize them; T1 lists the templates whose placeholders need the change.
- Current-tree verification: In `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md`, D6 contains "Its accepted syntax is the form `docs/templates/ISSUE.md` writes" and "as drift instead of normalizing it".
- Resolution reference: `docs(issues): [#2264] mark the D4 doc-types as proposed and pin the placeholder syntax`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### R6 - Clarify the reproduction environment and toolchain wording

- PR number: 2425
- Source review ID: 5468099277
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4228520218>
- Concern: ""The loop's compute pod" is not explained anywhere in the repository. Per the AI-agent implementation-independence policy, consider describing it in portable terms, for example "a Linux container with the workspace nightly toolchain"." (full comment at the Source URL)
- Solution: Section B1 now describes the reproduction environment as a Linux container with the workspace nightly toolchain, step 3 names the workspace toolchain and points to the nightly one that was used, and a Provenance paragraph states which parts of the first observed record come from the transcript of the run and which were completed from the validator's `legacy_shape` string, citing the maintainer's independent rerun. The draft's Bug-Fix Process step 2 uses the same term, and a new Progress Log entry maps the earlier entry's wording to it.
- Current-tree verification: In `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/manual-verification-evidence.md`, section B1 contains "a Linux container with the workspace nightly toolchain" and the Provenance paragraph, and the phrase "compute pod" no longer occurs in the file.
- Resolution reference: `docs(issues): [#2264] state the B1 reproduction environment and toolchain`
- Follow-up PR URL: N/A
- Reply URL: Pending; the reply recorded in the Processing Log is posted before the thread is resolved, and this field then records its URL.

### R7 - The baseline counts have drifted on `develop`

- PR number: 2425
- Source review ID: 5468099277
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4228520224>
- Concern: "This isn't blocking, because T1 reproduces the baseline. For reference, on current `develop`:" (full comment at the Source URL)
- Solution: Added a note under the Background baseline table that the counts were measured on 2026-10-03 and that T1 re-measures every row before it proposes the catalog; the T1 row and AC1 say the same. The recount on current `develop` is recorded with this round's report, not in the spec.
- Current-tree verification: In `docs/issues/drafts/2264-extend-strict-profiles-and-author-guidance/ISSUE.md`, the paragraph after the baseline table begins "The counts in this specification were measured on 2026-10-03", and the T1 row begins "Re-measure the baseline counts".
- Resolution reference: `docs(issues): [#2264] mark the D4 doc-types as proposed and pin the placeholder syntax`
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
- 2026-10-03 11:49 UTC - da2ce7 - Correction: the spelling check rejects `unsuffixed`. Reworded the F1 Solution to "reserves the name without the suffix for the implementation branch", and the F1 reply recorded at 11:37 will be posted with that same reworded clause. The word stays in Copilot's verbatim F1 concern and in the 11:37 log entry, which are not rewritten, so this record carries a file-local `cspell:ignore` for it.
- 2026-10-03 11:51 UTC - da2ce7 - Followed up F2: the B1 commands ran on the loop's compute pod at 2026-10-03 11:44 UTC and printed exactly the predicted records, committed as `docs(issues): [#2264] record the reproduced stale-guidance run for row 3`; updated the F2 Solution and verification. The F2 row stays `FIXED` with its original resolution reference.
- 2026-10-03 11:51 UTC - da2ce7 - Recorded the updated F2 thread reply for posting, which replaces the 11:37 text: FIXED. The draft keeps invoking `fix-bug` for the stale convention field lists, because the skills require it: `fix-bug` applies whenever work is substantively a bug, including stale or misleading behavior, even when the issue type says otherwise (`.github/skills/dev/debugging/fix-bug/SKILL.md:19-21`), and the ISSUE template allows `Not applicable` only when the work is not substantively a bug. The draft folder now holds `manual-verification-evidence.md`. Its section B1 records the attempt made while drafting: two disposable specs written from the convention's list, and the exact validator commands. That attempt was **Infeasible**, because the drafting environment has no Rust toolchain, so a field-by-field comparison against `profile.rs` predicted the result. The recorded commands then ran unchanged on the loop's compute pod (a Linux container with the workspace nightly toolchain). They printed exactly the predicted `legacy-shape` and `missing-required-field` (`epic`) records, each with exit `1`, so B1 is now **Reproduced** (`docs(issues): [#2264] record the reproduced stale-guidance run for row 3`). Bug-Fix Process step 2 (line 154) and the Workflow Checkpoint (line 204) state the result. Resolution: `docs(issues): [#2264] record the stale-guidance reproduction attempt for row 3`.
- 2026-10-03 11:51 UTC - da2ce7 - The new draft Progress Log line moved AC3 from line 231 to 232 and M6 from 268 to 269; updated the F4 and F5 verification lines, and recorded the F4 and F5 replies for posting with the corrected line numbers, replacing the 11:37 texts.
- 2026-10-03 11:51 UTC - da2ce7 - Recorded the updated F4 thread reply for posting: FIXED. Confirmed: `contrib/dev-tools/checks/frontmatter-validator/tests/cli.rs:683-708` (`it_should_skip_excluded_paths_in_every_mode`) asserts that both `docs/templates/` and the crate fixtures are skipped in every mode. AC3 (line 232) now requires only the tests that assert issue and EPIC outcomes to pass unmodified. It names `it_should_skip_excluded_paths_in_every_mode` as the one test that changes: its `docs/templates/` case becomes template-mode coverage, while it keeps asserting the fixtures exclusion. T6 (line 175) records the change. Resolution: `docs(issues): [#2264] exempt the template-exclusion test from row 3 AC3`.
- 2026-10-03 11:51 UTC - da2ce7 - Recorded the updated F5 thread reply for posting: FIXED. Confirmed: no tracked file under `docs/refactor-plans/closed/` declares `schema-version`, and a valid closed v1 spec yields no structural finding, so the old M6 could not show a warning. M6 (line 269) now copies a passing record into each historical location that has a strict profile: a closed v1 spec, plus a refactor plan opted into its profile if that profile is approved. It injects one unprefixed unknown field, validates the copy, and expects exactly one `warning` record and exit `0`. Resolution: `docs(issues): [#2264] make the row 3 advisory-severity scenario trigger a finding`.
- 2026-10-03 12:08 UTC - da2ce7 - Recorded the five thread replies, posted at 2026-10-03 12:06 UTC with the texts last recorded above for posting: F1 <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173070969>, F2 <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071020>, F3 <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071064>, F4 <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071118>, F5 <https://github.com/torrust/torrust-tracker/pull/2425#discussion_r4173071188>. Set every thread state to `RESOLVED`: each concern is fixed and its reply is posted, and the five threads are resolved right after this commit reaches the PR.
- 2026-10-09 10:23 UTC - da2ce7 - Started round 2: read maintainer review 5468099277 (`CHANGES_REQUESTED`) and its seven unresolved inline threads R1-R7 from the captured review comments and thread data; R1-R3 are requested and R4-R7 optional.
- 2026-10-09 10:25 UTC - da2ce7 - Committed the R1, R2, and R4 fixes as `docs(issues): [#2264] order the row 2.3 draft once and align it with rows 3 and 4`.
- 2026-10-09 10:26 UTC - da2ce7 - Committed the R3, R5, and R7 fixes as `docs(issues): [#2264] mark the D4 doc-types as proposed and pin the placeholder syntax`.
- 2026-10-09 10:27 UTC - da2ce7 - Committed the R6 fix as `docs(issues): [#2264] state the B1 reproduction environment and toolchain`, and reworded the F2 Solution above to the same portable environment term; the earlier log entries keep their wording.
- 2026-10-09 10:30 UTC - da2ce7 - Recorded the R1 thread reply for posting: FIXED in `docs(issues): [#2264] order the row 2.3 draft once and align it with rows 3 and 4`. I removed the draft from the EPIC's Draft Subissue Proposals list, which now reads "None pending.", and moved the triage reason into the 2026-10-03 Progress Log entry that records the triage, wrapped like its neighbors. Verification: row 2.3 in the Progressive Subissues table is now the draft's only placement.
- 2026-10-09 10:30 UTC - da2ce7 - Recorded the R2 thread reply for posting: FIXED in `docs(issues): [#2264] order the row 2.3 draft once and align it with rows 3 and 4`. The order is a real soft preference, so I kept the EPIC rationale and rewrote the draft's risk as "Ordering after EPIC row 2.3": both rows change `profile.rs` and the generated schema, and D2 assumes `schema-version: 1`. If row 2.3 is `BLOCKED` on row 4, row 3 proceeds. Out of Scope points there. Verification: "Neither issue depends" no longer appears.
- 2026-10-09 10:30 UTC - da2ce7 - Recorded the R3 thread reply for posting: FIXED in `docs(issues): [#2264] mark the D4 doc-types as proposed and pin the placeholder syntax`. D4 now says only four templates emit a `doc-type`, and marks `adr`, `implementation-retrospective`, `agent-review-reports` and `security-report` as proposed new values that their templates must start emitting (T4, AC8). T1 and AC1 record a disposition for every file in `docs/templates/`. Verification: the T1 row names `DISCUSSION.md` and `SECURITY-ANALYSIS.md`.
- 2026-10-09 10:30 UTC - da2ce7 - Recorded the R4 thread reply for posting: FIXED in `docs(issues): [#2264] order the row 2.3 draft once and align it with rows 3 and 4`. Row 2.3's expected output in the EPIC's Progressive Subissues table now says that if it needs a new `schema-version`, it is marked `BLOCKED` until row 4 is done, while row 3 proceeds without it. The new EPIC Progress Log entry records this. Verification: the row 2.3 cell ends with "while row 3 proceeds without it".
- 2026-10-09 10:30 UTC - da2ce7 - Recorded the R5 thread reply for posting: FIXED in `docs(issues): [#2264] mark the D4 doc-types as proposed and pin the placeholder syntax`. D6 now pins the enumerated-placeholder syntax to the `docs/templates/ISSUE.md` form: values in angle brackets, separated by `|` with no surrounding whitespace, each listed once, in any order. Template mode reports other spellings, such as the spaced form in `SECURITY-REPORT.md`, as drift instead of normalizing them. Verification: D6 under Architectural Decisions.
- 2026-10-09 10:30 UTC - da2ce7 - Recorded the R6 thread reply for posting: FIXED in `docs(issues): [#2264] state the B1 reproduction environment and toolchain`. Section B1 of `manual-verification-evidence.md` now says "a Linux container with the workspace nightly toolchain", and step 3 points to the nightly toolchain that was used. A Provenance paragraph states what came from the run's transcript and what was completed from `legacy_shape`, and cites your rerun. Verification: "compute pod" no longer appears in B1.
- 2026-10-09 10:30 UTC - da2ce7 - Recorded the R7 thread reply for posting: FIXED in `docs(issues): [#2264] mark the D4 doc-types as proposed and pin the placeholder syntax`. Under the Background baseline table I added a note that the counts date from 2026-10-03 and that T1 re-measures every row before proposing the catalog; T1 and AC1 say the same. My recount on current `develop`: ADRs 37 (README/index excluded), `SKILL.md` 49, manual-verification-evidence 47, refactor-plan 8 (template excluded), templates 15.

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
