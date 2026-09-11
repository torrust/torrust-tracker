---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2203
spec-path: docs/issues/open/2203-template-catalog-and-cargo-dependency-pr-template/ISSUE.md
branch: "2203-template-catalog-and-cargo-dependency-pr-template-spec"
related-pr: null
last-updated-utc: 2026-09-11 10:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/create-markdown-template/SKILL.md
    - .github/skills/dev/maintenance/update-dependencies/SKILL.md
    - docs/index.md
    - docs/templates/
---

<!-- skill-link: create-issue -->

# Issue #2203 - Catalog Markdown templates and add a Cargo dependency update PR template

## Goal

Make reusable repository Markdown templates discoverable through a concise catalog and provide a canonical PR-description template for Cargo dependency updates.

## Background

`docs/templates/` is already the canonical location for reusable repository Markdown templates, and several skills and agents link to templates stored there. The directory has no index that explains each template's purpose, intended output, or the workflow that uses it.

Cargo dependency updates have a stable pull-request description shape: a short summary, the affected lockfile, validation results, and the complete verbatim output captured in an invocation-specific `.tmp/<timestamp>-cargo-update.txt` file. The existing `update-dependencies` skill requires the output but repeats the surrounding authoring structure inline instead of linking to a canonical template. Its fixed `.tmp/cargo-update.txt` path can be overwritten when concurrent update workflows share a working tree.

The existing date-only branch prefix (`YYYYMMDD`) is also insufficient for concurrent runs. The workflow should derive a high-resolution timestamp once, for example `YYYYMMDD-HHMMSS`, and use that same value in both the branch and output filenames. Concurrent operations in one working tree remain unsafe because Git's index and branch checkout are shared; this change only prevents capture-file collisions.

GitHub-native contributor templates are a separate concern. Those may use `.github/PULL_REQUEST_TEMPLATE/` when GitHub's template picker is specifically required; this issue does not create that adapter.

## Scope

### In Scope

- Add `docs/templates/README.md` as a concise catalog of every reusable template currently in `docs/templates/`, including each template's purpose, intended destination, and primary workflow or agent.
- Add `docs/templates/CARGO-DEPENDENCY-UPDATE-PR.md` as the canonical authoring template for Cargo dependency-update PR descriptions.
- Update `docs/index.md` to link to the template catalog and the new dependency-update PR template.
- Update `.github/skills/dev/maintenance/update-dependencies/SKILL.md` to derive a single high-resolution timestamp (`YYYYMMDD-HHMMSS`) for the branch name and captured Cargo output filename, link to and use the canonical template, and retain the requirement to insert the captured output verbatim.
- Update `.github/skills/dev/planning/create-markdown-template/SKILL.md` so material template additions, removals, or repurposing maintain the catalog, and document the GitHub-native-template exception.
- Verify templates, skills, catalog links, and their Markdown/spelling quality.

### Out of Scope

- Moving the existing reusable templates out of `docs/templates/`.
- Creating a `.github/PULL_REQUEST_TEMPLATE/` adapter or changing GitHub's contributor-template picker behavior.
- Changing Cargo update behavior beyond deriving both the branch name and captured-output filename from one high-resolution timestamp, and replacing duplicated PR-body structure with a canonical template reference.
- Updating unrelated documentation templates or historical records.

## Architectural Decisions

Record architectural decisions that are already known when this specification is
drafted. Link existing ADRs and identify ADRs this issue is expected to create.

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: `None known`; the work applies the existing single-source-of-truth and portability policy.

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

For work involving child processes, asynchronous I/O, network readiness,
resource cleanup, or reusable test fixtures, define before implementation:

- the narrow public interface and each collaborator's responsibility;
- normal, failure, and drop-path ownership/lifetime invariants;
- the absolute deadline that bounds every awaited readiness operation; and
- a post-vertical-slice design-review checkpoint.

Not applicable. This is documentation and workflow maintenance with no child processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                       | Notes / Expected Output                                                                                                                                                                                                                                                        |
| --- | ------ | ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Inventory templates and current references | Confirm all current `docs/templates/` files and their live skill, agent, and documentation consumers.                                                                                                                                                                          |
| T2  | TODO   | Add template catalog                       | Create `docs/templates/README.md` with concise directory policy and an entry for each existing template.                                                                                                                                                                       |
| T3  | TODO   | Add dependency-update PR template          | Add `CARGO-DEPENDENCY-UPDATE-PR.md` with placeholders and an explicit verbatim Cargo-output section.                                                                                                                                                                           |
| T4  | TODO   | Link workflow and catalog                  | Update `docs/index.md`, `update-dependencies`, and `create-markdown-template` to point to the canonical catalog/template; make one high-resolution timestamp (`YYYYMMDD-HHMMSS`) name both the branch and the Cargo-output capture file; preserve the GitHub-native exception. |
| T5  | TODO   | Review and validate documentation          | Check links and frontmatter, run applicable linters, and review whether the catalog/template remove duplication without obscuring workflow requirements.                                                                                                                       |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task  | Coherent change set                               | Commit policy                                                        |
| ----- | ------------------------------------------------- | -------------------------------------------------------------------- |
| T1-T2 | Template inventory and catalog                    | Commit after focused Markdown and link validation.                   |
| T3    | Cargo dependency-update PR template               | Commit after reviewing placeholders and canonical-source boundaries. |
| T4-T5 | Skill and index integration plus final validation | Commit after `linter all` and independent review.                    |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, commit each reviewed test-design increment before starting the next planned
file or behavior area. Keep final verification and completion evidence separate when it improves
reviewability. Use a Conventional Commit message with the narrow affected scope, and sign every
commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec created in `docs/issues/open/2203-template-catalog-and-cargo-dependency-pr-template/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2203 created and issue number added to this spec
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

### Progress Log

Append one line per meaningful update.

- 2026-09-11 10:25 UTC - GitHub Copilot - Drafted the proposed task specification in the ignored `.tmp/` directory while the unrelated dependency-update PR CI retry runs. - `.tmp/template-catalog-and-cargo-dependency-pr-template/ISSUE.md`
- 2026-09-11 10:30 UTC - GitHub Copilot - After PR #2201 merged, copied the approved draft into the repository's numbered open-issue directory and linked it to the already-open issue #2203 on the dedicated spec branch. - `docs/issues/open/2203-template-catalog-and-cargo-dependency-pr-template/ISSUE.md`

## Acceptance Criteria

- [ ] AC1: `docs/templates/README.md` concisely explains the template directory and catalogs every reusable Markdown template currently present, with purpose, intended destination, and primary workflow or agent.
- [ ] AC2: `docs/templates/CARGO-DEPENDENCY-UPDATE-PR.md` provides a reusable Cargo dependency-update PR structure and explicitly requires the complete, unedited invocation-specific Cargo output in a fenced `text` block.
- [ ] AC3: `docs/index.md` links to the catalog and the new template.
- [ ] AC4: `update-dependencies` derives a single high-resolution timestamp (`YYYYMMDD-HHMMSS`) once and uses it at the beginning of both the branch name and `.tmp/<timestamp>-cargo-update.txt` output path, preventing ordinary concurrent invocations from overwriting another's captured output and keeping captured files easy to find in directory listings.
- [ ] AC5: `update-dependencies` links to the canonical template rather than repeating its full reusable PR-body structure, while its mandatory update/validation/commit workflow remains explicit.
- [ ] AC6: `create-markdown-template` requires catalog maintenance for material template lifecycle changes and records the exception for GitHub-native templates in `.github/PULL_REQUEST_TEMPLATE/`.
- [ ] AC7: No second independent canonical PR-description template is introduced under `.github/PULL_REQUEST_TEMPLATE/`.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Relevant tests for changed components
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                       | Human-oriented command/steps                                                                    | Expected Result                                                                                                                                                                        | Status | Evidence                                     |
| --- | ------------------------------ | ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Discover a template            | Read `docs/templates/README.md`, identify the Cargo dependency-update PR template, and open it. | A contributor can determine the template purpose and use it without searching the skills tree.                                                                                         | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Follow the dependency workflow | Read `update-dependencies` and the PR template together.                                        | The skill gives the process; the template supplies the reusable body; the high-resolution timestamped Cargo-output rule is unambiguous and prevents ordinary concurrent-run overwrite. | TODO   | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a
  real human-oriented use of the feature or reproduction of the bug fix, not a
  simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these
  scenarios. Record actual prerequisites, actions, commands, program output,
  relevant tracker logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

Temporary scripts may automate an issue-local verification scenario, but they
are neither maintained automatic tests nor manual-verification evidence. Before
creating one, record in the issue specification:

- why temporary automation is better for this concrete scenario than a
  maintained Rust automatic test;
- the script's issue-local path, what it verifies, and its intended removal or
  retention owner; and
- when using Python instead of Rust, why Rust is not suitable for that specific
  script.

Keep the script in the issue-specification folder so later reviewers can inspect
the verification performed. Promote durable product-behavior checks into Rust
automatic tests when practical, then remove the disposable script.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                               |
| ----- | ---------------------- | ------------------------------------------------------ |
| AC1   | TODO                   | Template catalog inspection and link checks            |
| AC2   | TODO                   | Template content review and manual scenario M2         |
| AC3   | TODO                   | `docs/index.md` link checks                            |
| AC4   | TODO                   | Skill review and concurrent-invocation path inspection |
| AC5   | TODO                   | Skill/template comparison                              |
| AC6   | TODO                   | Template-creation skill review                         |
| AC7   | TODO                   | Changed-files review                                   |

## Risks and Trade-offs

- The catalog can become stale when templates change. Mitigation: make catalog maintenance a material template-lifecycle requirement in `create-markdown-template`.
- Concurrent invocations can overwrite a fixed `.tmp/cargo-update.txt` capture. Mitigation: derive one high-resolution timestamp (`YYYYMMDD-HHMMSS`) per invocation and use `.tmp/<timestamp>-cargo-update.txt` consistently in the update, commit, and PR-description steps. This does not make concurrent Git operations in a shared working tree safe.
- The template could duplicate or weaken mandatory workflow rules. Mitigation: keep process requirements in `update-dependencies`; use the template only for PR-body structure and a clear placeholder for the required verbatim Cargo output.
- A GitHub-native adapter could become a second source of truth. Mitigation: keep it out of scope and document that it is an exception only when the GitHub picker is required.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

- Related issues: #2203
- Related PRs: https://github.com/torrust/torrust-tracker/pull/2201 (motivating Cargo dependency-update PR)
- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
