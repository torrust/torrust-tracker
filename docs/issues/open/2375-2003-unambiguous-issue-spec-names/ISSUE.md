---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 2003
github-issue: 2375
spec-path: docs/issues/open/2375-2003-unambiguous-issue-spec-names/ISSUE.md
branch: "2375-unambiguous-issue-spec-names-spec"
related-pr: null
last-updated-utc: "2026-09-29 16:29"
semantic-links:
  skill-links:
    - create-issue
    - cleanup-completed-issues
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
    - contrib/dev-tools/checks/frontmatter-validator
    - docs/issues/README.md
    - docs/issues/drafts/README.md
    - docs/issues/open/README.md
    - docs/issues/closed/README.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/open/2264-2003-refactor-semantic-link-conventions/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2375 - Define Unambiguous Issue Specification Directory Names

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Define a spec directory name that says which number is the issue's own and which is the parent
EPIC's, regardless of the folder (`drafts/`, `open/`, `closed/`) the spec lives in. Enforce it
with a deterministic check, and document what a draft spec means.

## Background

Spec directories currently follow `{issue}-{epic}-{slug}`, and either number may be missing:

| Folder    | Name                        | Meaning                                  |
| --------- | --------------------------- | ---------------------------------------- |
| `open/`   | `2264-2003-refactor-...`    | issue #2264, parent EPIC #2003           |
| `open/`   | `1843-migrate-git-hooks-...` | issue #1843, no parent EPIC              |
| `drafts/` | `1669-update-all-...`       | no issue yet, parent EPIC #1669          |
| `drafts/` | `generalize-error-events`   | no issue yet, no parent EPIC             |

`1843-...` and `1669-...` have the same shape and mean different things. Only the containing
folder says whether the leading number is the issue or its parent EPIC. When a spec is quoted
out of context, in a link, a log, a branch discussion, or a tool's output, the name alone is
ambiguous. The `create-issue` skill has to explain the special case ("the prefix is the parent
EPIC number, not the future subissue number"), and the rename at issue creation changes the
meaning of an existing segment as well as adding one.

Drafts also use ad hoc ordinal segments that no document defines, such as
`1488-si-15-define-udp-active-request-policy` and `1669-01-establish-baseline-analysis`.

The slug is meant to follow GitHub's branch-name format for the issue title: lowercase, words
joined by hyphens.

`drafts/` is not only a short waiting room for maintainer approval. It also holds parked ideas
and incomplete tasks, a kind of backlog. What every draft has in common is that it lacks the
human review needed to start work, whether on scope, acceptance criteria, completeness, or
confirmation that an agent understood the task. The maintainer decided that frontmatter does
not need to distinguish these cases: even a thoroughly reviewed draft can go stale while higher
priority work proceeds.

## Scope

### In Scope

- Choose one directory-name format with no ambiguity across folders.
- Define the slug rule and whether ordinal segments are allowed.
- Update the `create-issue` and `cleanup-completed-issues` skills, the `docs/issues/` READMEs,
  and the templates' `spec-path` placeholders.
- Document that a draft lacks the human review needed to start work, and that a spec must be
  reviewed against the current code and context before its GitHub issue is created and again
  before implementation starts.
- A deterministic check that a spec directory name agrees with the spec's `github-issue` and
  `epic` frontmatter.
- Migrate existing names according to the maintainer's migration decision.

### Out of Scope

- Branch naming conventions.
- The spec file names inside the directory (`ISSUE.md`, `EPIC.md`, supporting artifacts).
- Legacy standalone specs, which #2159 migrates to folder-style.
- Renaming `closed/` specs; that folder is a temporary buffer emptied by the cleanup workflow.
- A frontmatter field or status value for draft readiness.

## Format Options

### Option A: Tagged Segments (Selected)

`[i{issue}-][e{epic}-]{slug}`, where each number carries a one-letter tag.

| State                        | Name                             |
| ---------------------------- | -------------------------------- |
| Draft, no parent             | `generalize-error-events`        |
| Draft, parent EPIC #2003     | `e2003-mine-pr-review-audit-records` |
| Issue 2400 (example), no parent | `i2400-generalize-error-events`  |
| Issue 2400 (example), parent #2003 | `i2400-e2003-mine-pr-review-audit-records` |
| EPIC #2278, parent EPIC #2003 | `i2278-e2003-strengthen-pr-review-author-self-audit` |

- Pros: unambiguous in any folder; issue creation only adds an `i` segment and never changes the
  meaning of an existing one; `ls | grep e2003-` lists an EPIC's children exactly; the tag grammar
  can later distinguish other GitHub artifacts that use numbers, such as `p` for a pull request or
  `d` for a discussion.
- Cons: changes every existing name; the letters are a new thing to learn; moving a subissue to a
  different EPIC requires a directory rename and live-link repair.

### Option B: Fixed Slots with a Placeholder

`{issue|x}-{epic|x}-{slug}`, where both slots are always present and `x` means "none yet" or
"none".

- Examples: `x-x-generalize-error-events`, `x-2003-mine-...`, `2400-x-...`,
  `2400-2003-mine-...`.
- Pros: unambiguous; close to the current shape.
- Cons: every independent issue carries `-x-` forever; a slug starting with a single letter or a
  number reads confusingly.

### Option C: Own Number Only; Parent in Frontmatter

`[{issue}-]{slug}`. The parent EPIC exists only in the `epic` frontmatter field.

- Pros: one source of truth for the parent (the field already exists); reassigning the parent
  needs no rename; the shortest names.
- Cons: the name no longer shows the parent; listing an EPIC's children needs a frontmatter
  query or the EPIC's subissue table.

### Decision

The maintainer selected Option A. It removes ambiguity without dropping the parent from the name,
which supports fast IDE searches and keeps an EPIC's subissues together in directory listings. The
duplication between the name and frontmatter is intentional and requires the T3 check to reject
disagreement. Reassigning a subissue to another EPIC is expected to be rare; when it happens, the
directory rename and live-link repair are the accepted maintenance cost.

Ordinals: do not encode them in names. Order belongs in the parent EPIC's subissue table, where
it can change without renames.

Migration: rename `drafts/` and `open/`; leave `closed/` unchanged because it is temporary.

Draft readiness: document it only. Do not add a frontmatter field or status value.

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: a root ADR recording the chosen format, because it is a repository-wide
  convention that tools and skills depend on.

### Coordination with EPIC #2264

This issue belongs to #2003, but it touches areas that sibling EPIC #2264 owns: document
metadata, semantic-link and path-reference conventions, and their typed validators.

- T3 adds a check to `frontmatter-validator`. Agree with #2264's owner whether it lives there or
  in #2264's planned validator structure, and follow that EPIC's diagnostics and staged
  enforcement model.
- T4 renames paths that `semantic-links.related-artifacts` entries and path references cite.
  Link repair must update those frontmatter paths too, using the forms #2264's frozen v1
  contract allows.
- If #2264 introduces a typed reference to an issue spec, the reference should resolve through
  the issue number, not the directory name, so a future rename cannot break it. Record the
  outcome in the root ADR.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable. This is a convention change, not a defect.

## Regression Test Strategy

Not applicable. The T3 check gets fixture tests.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                         | Notes / Expected Output                                                                                                              |
| --- | ------ | ---------------------------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Record the format decision   | Root ADR records Option A, the slug rule, no ordinal segments, and migration of `drafts/` and `open/` only.                         |
| T2  | TODO   | Update skills and docs       | `create-issue` owns the naming and review-freshness rules. `cleanup-completed-issues`, the `docs/issues/` READMEs, and template `spec-path` placeholders link to or follow them. |
| T3  | TODO   | Add a name-consistency check | `frontmatter-validator` checks `drafts/` and `open/` spec names against `github-issue` and `epic`, rejects an issue segment in drafts, and skips `closed/`. Fixture tests included. |
| T4  | TODO   | Migrate existing names       | Rename every `drafts/` and `open/` spec directory with `git mv` and repair every live reference, following the cleanup skill's link-repair procedure. |

## Commit Points

| Task | Coherent change set                                  | Commit policy                                        |
| ---- | ---------------------------------------------------- | ---------------------------------------------------- |
| T1   | Root ADR                                             | Commit after maintainer approval.                    |
| T2   | Skill, README, and template updates                  | Commit after focused validation and required review. |
| T3   | Name-consistency check and fixture tests             | Commit after focused validation and required review. |
| T4   | Mechanical renames plus link repair, in one commit   | Commit after `frontmatter-validator docs/issues/drafts docs/issues/open` and `linter all` pass. |

Sign every commit with GPG and use a Conventional Commit message with a narrow scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-29 12:20 UTC - GitHub Copilot - Drafted from the maintainer's description of the
  ambiguity between issue and parent EPIC numbers in draft and open spec names.
- 2026-09-29 15:59 UTC - GitHub Copilot - Recorded maintainer decisions: Option A, migrate
  `drafts/` and `open/` only, document draft readiness without frontmatter changes, and review a
  spec before issue creation and before implementation.
- 2026-09-29 16:11 UTC - GitHub Copilot - Maintainer chose #2003 as the parent EPIC and noted
  the impact on #2264; recorded the coordination boundary.
- 2026-09-29 16:29 UTC - GitHub Copilot - Maintainer approved the spec. Created issue #2375,
  linked it as a sub-issue of #2003, and moved the spec to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: The chosen format is recorded in a root ADR and described in the `create-issue` skill,
  and no other document restates it.
- [ ] AC2: A spec directory name alone tells a reader whether the spec has an issue number, a
  parent EPIC, both, or neither.
- [ ] AC3: `frontmatter-validator` fails on a `drafts/` or `open/` name that disagrees with the
  `github-issue` or `epic` frontmatter, skips `closed/`, and passes on every migrated spec.
- [ ] AC4: No live link points to a renamed path.
- [ ] AC5: The `create-issue` skill says that a draft lacks the human review needed to start
  work, and requires spec review against the current code and context before issue creation and
  before implementation starts. `docs/issues/drafts/README.md` links to that rule instead of
  restating it.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all` (lychee covers broken local links after renames)
- `cargo test -p frontmatter-validator`
- `frontmatter-validator docs/issues/drafts docs/issues/open` (`--all` also checks unrelated
  legacy specs and currently exits `1`)

### Manual Verification Scenarios

| ID  | Scenario                    | Human-oriented command/steps                                                   | Expected Result                                         | Status | Evidence                                     |
| --- | --------------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Draft to open lifecycle     | Create a draft under an EPIC, then follow `create-issue` to move it to `open/` | Only an issue segment is added; the check passes at each step | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Mismatch is caught          | Rename a scratch copy so its EPIC segment disagrees with `epic`, run the check | Actionable diagnostic naming both values; nonzero exit  | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Stale draft is re-reviewed  | Follow `create-issue` for an existing draft and for an open spec before work starts | The workflow requires review against the current code and context at both points | TODO   | `manual-verification-evidence.md` section V3 |

## Risks and Trade-offs

- **Link churn.** Renaming `open/` breaks links in EPIC tables, ADRs, and skills. Mitigation:
  one mechanical commit, lychee, and the cleanup skill's link-repair procedure.
- **In-flight branches.** Open PRs that add or edit specs will conflict with the renames.
  Mitigation: schedule T4 when few spec PRs are open.
- **Name and frontmatter drift** (Options A and B). Mitigation: the T3 check.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Parent EPIC: #2003
- Related issues: #2159 (folder-style issue specs), #2264 (frontmatter and semantic-link
  conventions)
