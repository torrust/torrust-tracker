---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2446"
---

<!-- skill-link: process-pr-review -->

# PR #2447 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2447>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| SPEC-004 | `review-finding:pr-2447-spec-004` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| SPEC-001 | `review-finding:pr-2447-spec-001` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| SPEC-005 | `review-finding:pr-2447-spec-005` | Copilot | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| SPEC-002 | `review-finding:pr-2447-spec-002` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| SPEC-003 | `review-finding:pr-2447-spec-003` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2447-f1` | Human | Major | documentation | RE_RAISE_OF:SPEC-001 | FIXED | RESOLVED |
| F2 | `review-finding:pr-2447-f2` | Human | Major | documentation | RE_RAISE_OF:SPEC-002 | FIXED | RESOLVED |
| F3 | `review-finding:pr-2447-f3` | Human | Major | documentation | RE_RAISE_OF:SPEC-003 | FIXED | RESOLVED |
| F4 | `review-finding:pr-2447-f4` | Human | Major | correctness | RE_RAISE_OF:SPEC-004 | FIXED | RESOLVED |
| F5 | `review-finding:pr-2447-f5` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2447-f6` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2447-f7` | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### SPEC-004 - README audit package set is undefined

- PR number: 2447
- Source review ID: 5427394279
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194486395>
- Concern: T10 and its criterion promised the "current package set", but the workspace also has the root crate and six `contrib/dev-tools/` members, most without a README, while MV4 checked only `packages/` and `console/tracker-client`, so the audit could omit packages and still pass.
- Solution: Goal item 2 now defines the audited set as the 25 tracker packages (root crate, `console/tracker-client`, the 23 `packages/*` members), matching the EPIC Package Inventory, with a `missing` rating; the `contrib/dev-tools/` members are out of scope. In Scope, T10, AC4 and MV4 use the same set.
- Current-tree verification: `ls -d packages/*/` lists 23 folders and all 25 audited packages have a `README.md`; Goal item 2, In Scope, Out of Scope, T10, AC4 and MV4 in `docs/issues/open/2446-1669-establish-baseline-analysis/ISSUE.md` inspected; pre-commit passed.
- Resolution reference: `docs(issues): [#2446] define the README audit package set`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195044403>

### SPEC-001 - Commit Points section is missing

- PR number: 2447
- Source review ID: 5427394279
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194486440>
- Concern: The spec has five documentation-changing tasks but no `Commit Points` section, which the issue template and `create-issue` workflow require.
- Solution: Added `## Commit Points` after the Implementation Plan, mapping T9 to T13 to one commit each and recording T12 as a justified no-change, without an empty commit, when nothing is found.
- Current-tree verification: the section and its five rows inspected; `linter markdown` and pre-commit passed.
- Resolution reference: `docs(issues): [#2446] add commit points to the baseline-analysis spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195043573>

### SPEC-005 - Framework name misspelled as Symphony

- PR number: 2447
- Source review ID: 5427394279
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194486480>
- Concern: The historical T8 trade-off row named the PHP framework "Symphony" instead of "Symfony".
- Solution: Corrected both occurrences and added `Symfony` to `project-words.txt` so cspell accepts the proper name.
- Current-tree verification: the T8 trade-off row inspected; `linter cspell` passed after the dictionary change.
- Resolution reference: `docs(issues): [#2446] correct the Symfony framework name`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195044623>

### SPEC-002 - Manual verification evidence is not trackable

- PR number: 2447
- Source review ID: 5427394279
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194486522>
- Concern: The manual-verification table had no status or evidence fields, and the spec never required results in an issue-local `manual-verification-evidence.md`.
- Solution: `### Manual Verification Scenarios` now follows the template, with command/steps, Status and Evidence columns, plus notes requiring the issue-local evidence file with inline commands, output and outcome.
- Current-tree verification: the MV1 to MV4 table and notes inspected; `linter markdown` and pre-commit passed.
- Resolution reference: `docs(issues): [#2446] track manual verification status and evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195043889>

### SPEC-003 - Completion evidence sections are missing

- PR number: 2447
- Source review ID: 5427394279
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194486576>
- Concern: The spec lacked `Acceptance Verification` and `Implementation Completion Review`, and its checklist did not require the post-implementation acceptance-criteria review.
- Solution: Numbered the remaining criteria AC1 to AC10, adding the manual-evidence (AC9) and post-implementation re-review (AC10) criteria; added `### Acceptance Verification` with a TODO evidence row per criterion, `## Implementation Completion Review`, and the template's post-implementation checkpoints.
- Current-tree verification: the Acceptance Criteria, Acceptance Verification, Implementation Completion Review and Workflow Checkpoints sections inspected; pre-commit passed.
- Resolution reference: `docs(issues): [#2446] add acceptance verification and completion review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195044131>

### F1 - Commit Points section is missing

- PR number: 2447
- Source review ID: 5427688438
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194720736>
- Concern: Same request as SPEC-001, citing `create-issue/SKILL.md:115-124` and the template.
- Solution: Fixed by the SPEC-001 change; the T12 no-change rule the reviewer asked for is included.
- Current-tree verification: as for SPEC-001.
- Resolution reference: `docs(issues): [#2446] add commit points to the baseline-analysis spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195047135>

### F2 - Manual verification has no status, evidence or evidence file

- PR number: 2447
- Source review ID: 5427688438
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194720745>
- Concern: Same request as SPEC-002, citing `create-issue/SKILL.md:135` and `:321-328`.
- Solution: Fixed by the SPEC-002 change.
- Current-tree verification: as for SPEC-002.
- Resolution reference: `docs(issues): [#2446] track manual verification status and evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195047342>

### F3 - Acceptance Verification and Implementation Completion Review are missing

- PR number: 2447
- Source review ID: 5427688438
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194720755>
- Concern: Same request as SPEC-003, also noting the missing template re-review criterion and post-implementation checkpoints.
- Solution: Fixed by the SPEC-003 change, which adds both the re-review criterion (AC10) and the checkpoints.
- Current-tree verification: as for SPEC-003.
- Resolution reference: `docs(issues): [#2446] add acceptance verification and completion review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195047533>

### F4 - README audit package set is undefined on both sides

- PR number: 2447
- Source review ID: 5427688438
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194720761>
- Concern: Same request as SPEC-004, widened: besides the inclusion set, T10 named only `rest-api-core` for removal, although the audit also rates six other packages no longer in the workspace.
- Solution: Fixed by the SPEC-004 change; T10 now lists all seven stale rows to remove (`clock`, `located-error`, `metrics`, `peer-id`, `server-lib`, `rest-tracker-api-core`, `contrib/bencode`).
- Current-tree verification: T10 inspected against the rows of `docs/issues/open/1669-overhaul-packages/readme-audit.md`; as for SPEC-004 otherwise.
- Resolution reference: `docs(issues): [#2446] define the README audit package set`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195047784>

### F5 - Background misdates e2e-tools and persistence-benchmark

- PR number: 2447
- Source review ID: 5427688438
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194720769>
- Concern: The Background said both packages were added after the June work and that all three outputs describe the June workspace, but the 2026-06-10 report and the diagram already include them; only the older README audit lacks them.
- Solution: The Background now says the README audit predates both reports (2026-05-18 workspace) and is the only output missing the two packages.
- Current-tree verification: `grep -c` finds both package names in the 2026-06-10 report (5) and the diagram (4), and none in `readme-audit.md`.
- Resolution reference: `docs(issues): [#2446] correct when e2e-tools and persistence-benchmark appeared`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195048073>

### F6 - T2 and T4 marked done against 27-package outputs

- PR number: 2447
- Source review ID: 5427688438
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194720777>
- Concern: T2 and T4 kept the draft's "27 packages" expected outputs while the 2026-05-19 report lists 29 packages and the README audit has 26 rows with a Summary table (27) that does not match its rows.
- Solution: The T2/T3 and T4 evidence bullets record the actual counts as deviations, and T10 now requires a Summary table that matches the rows.
- Current-tree verification: the 2026-05-19 report header reads "Workspace packages: 29"; the audit has 24 `packages/` rows plus `console/tracker-client` and `contrib/bencode`, rated 2 good, 10 minimal, 14 stub.
- Resolution reference: `docs(issues): [#2446] record actual T2 and T4 package counts`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195048331>

### F7 - SI-02 and SI-03 cannot be resolved from the spec

- PR number: 2447
- Source review ID: 5427688438
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4194720785>
- Concern: The promotion dropped the draft Background that defined SI-02 and SI-03, so the T5 row and the 2026-05-18 log entry cite names the EPIC no longer uses.
- Solution: The T5 row cites SI-02 (#1790) and SI-03 (#1793), and the T5 evidence bullet defines both. The 2026-05-18 log entry stays unchanged as append-only history.
- Current-tree verification: `gh issue view` confirms #1790 is the `DurationSinceUnixEpoch` move and #1793 the `DEFAULT_TIMEOUT` removal; the T5 row and evidence bullet inspected.
- Resolution reference: `docs(issues): [#2446] map SI-02 and SI-03 to their issues`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2447#discussion_r4195048576>

## Processing Log

- 2026-10-06 11:04 UTC - Copilot review 5427394279 posted five inline findings (SPEC-001 to SPEC-005). Its overview badges say "Low severity", but each comment carries a bracket severity, which the rows use.
- 2026-10-06 11:31 UTC - Human review 5427688438 (da2ce7, round 1) requested changes with seven inline findings: F1 to F4 re-raise SPEC-001 to SPEC-004 (F4 widened to the removal side), and F5 to F7 are new.
- 2026-10-06 12:14 UTC - Fixed all twelve findings in eight commits (one per distinct change; each re-raise shares its original's commit), pushed once after the pre-push suite passed, and replied on every thread before recording them here.

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
