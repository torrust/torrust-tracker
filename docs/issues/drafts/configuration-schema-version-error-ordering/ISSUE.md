---
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/configuration-schema-version-error-ordering/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-09-10 09:45
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/configuration/src/lib.rs
    - docs/issues/open/2190-maintenance-frictions-cleanup/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Report an unsupported configuration schema version instead of an unknown-field error

**Parent EPIC:** None. This issue is standalone; see the reasoning in [EPIC #2190](../../open/2190-maintenance-frictions-cleanup/EPIC.md).

## Goal

Make a configuration file written for an older schema version report that its schema version is unsupported, rather than complaining about a section name the current schema does not know.

## Background

Verified at revision `f6b73e29` on 2026-09-09. In `packages/configuration/src/v3_0_0/mod.rs`, `Configuration::load` runs the strict extract at line 392 before checking `metadata.schema_version` at line 395. Every configuration section carries an attribute rejecting unknown fields, so a version-2 configuration file fails on the first section name the version-3 schema does not know, and the user is told there is an unknown field named `tracker`.

The error that exists precisely to explain this case, `Error::UnsupportedVersion`, is defined at `packages/configuration/src/lib.rs:816` and returned at line 396, but the strict extract has already failed by then, so it is unreachable for exactly the input it was written for.

The user-visible result is a spelling complaint where the real problem is that the file is a schema version behind, which sends people looking for a typo in a file that has none. This was found while verifying the maintenance inventory in EPIC #2190 and is recorded there as the one item that is a behavioural change rather than clean-up.

## Scope

### In Scope

- Report an unsupported schema version for a configuration file whose declared version is not the one this binary supports, whatever else the file contains.
- Decide between reading the metadata section separately before the strict extract and relaxing the extract enough to reach the version check, and record the reasoning.
- Add a regression test that loads a version-2 file with the version-3 loader and asserts the unsupported-version error.
- Check the same ordering in the version-2 loader at `packages/configuration/src/v2_0_0/mod.rs:362` and record whether it has the same defect.

### Out of Scope

- Supporting or migrating older configuration schemas. The goal is a correct error, not compatibility.
- Relaxing the unknown-field policy in general. Unknown fields inside a file of the right schema version must still be rejected.
- Any other change to configuration loading, defaults, or environment overrides.

## Architectural Decisions

The choice between a separate metadata probe and a relaxed extract has consequences for every future schema change, so record it. If the chosen approach constrains how later schema versions detect themselves, create an ADR under `docs/adrs/`; if it is contained inside one loader function, a note in this specification is enough.

- Related ADRs: `None`
- ADRs to create: Configuration schema-version detection order, if the chosen approach constrains future schema versions

## Design and Ownership Review

`Not applicable` for process, I/O, and cleanup concerns: loading is synchronous and owns nothing beyond the parsed value. The one interface question is whether a metadata probe becomes a second public entry point on the configuration loader or stays a private step inside `load`; prefer the private step unless a caller genuinely needs the version without the rest of the file.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Reproduce the defect with a failing test | A test loading a version-2 file with the version-3 loader fails, asserting the unsupported-version error and observing the unknown-field error instead. |
| T2 | TODO | Choose and record the detection approach | The chosen approach and the rejected alternative are recorded, with the reason. |
| T3 | TODO | Implement the chosen approach | The failing test passes; unknown fields in a correctly versioned file are still rejected. |
| T4 | TODO | Check the version-2 loader for the same ordering | The version-2 loader is confirmed correct, or its defect is recorded and either fixed here or given its own issue. |
| T5 | TODO | Final verification and acceptance review | `linter all` exits 0, the configuration package tests pass, and every acceptance criterion is re-reviewed against observed behaviour. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | The failing regression test. | Commit the failing test first so the defect is recorded as a test before it is fixed. |
| T2, T3 | The detection change. | Commit once the regression test passes and the unknown-field rejection still holds. |
| T4 | The version-2 loader finding. | Commit the fix if it is the same one-line ordering; otherwise record a justified no-change decision and open a separate issue. |
| T5 | Completion evidence. | Keep separate from the fix so the verification record is reviewable on its own. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use a Conventional Commit message with the narrow affected scope.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/configuration-schema-version-error-ordering/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Specification moved from `docs/issues/drafts/` to `docs/issues/open/` under its assigned number
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

- 2026-09-10 09:30 UTC - Specification author - Drafted from the verified evidence in EPIC #2190; inventory item R1, the one behavioural defect rather than clean-up - https://github.com/torrust/torrust-tracker/issues/2190

## Acceptance Criteria

- [ ] AC1: Loading a configuration file that declares an older schema version reports that the schema version is unsupported, and names the version found.
- [ ] AC2: The reported error no longer names a section as an unknown field when the real cause is the schema version.
- [ ] AC3: A configuration file of the correct schema version that contains a genuinely unknown field is still rejected as such.
- [ ] AC4: A regression test covers both cases and fails if the ordering is reintroduced.
- [ ] AC5: The version-2 loader is either confirmed correct or its equivalent defect is recorded.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- The configuration package tests
- The workspace test suite for the packages that depend on configuration loading

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | An old configuration file explains itself | Run the tracker against a configuration file declaring the version-2 schema and read the error printed to the console | The message says the schema version is unsupported and names the version found, not an unknown field | TODO | `manual-verification-evidence.md` section V1 |
| M2 | A genuine typo is still a typo | Run the tracker against a correctly versioned configuration file containing one misspelled field and read the error | The message still identifies the unknown field | TODO | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a real human-oriented use of the repository's tooling, not a simulated result and not merely running automated tests.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these scenarios. Record actual prerequisites, actions, commands, program output, relevant logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None is planned. Both scenarios are direct runs of the binary against a small configuration file, and the regression coverage belongs in the package's own tests.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | M1 |
| AC2 | TODO | M1 |
| AC3 | TODO | M2 |
| AC4 | TODO | Automatic checks |
| AC5 | TODO | {PR link} |

## Risks and Trade-offs

- Relaxing the strict extract to reach the version check could weaken unknown-field rejection for correctly versioned files. Mitigation: an acceptance criterion and a manual scenario both assert that rejection still holds, and the alternative approach avoids the relaxation entirely.
- A separate metadata probe parses the file twice. Mitigation: configuration loading happens once at startup, so the cost is negligible next to the clarity of the error.
- The same ordering may exist in other loaders. Mitigation: T4 checks the version-2 loader explicitly rather than assuming.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions, material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory if the chosen detection approach constrains how future schema versions are detected, or the version-2 loader turns out to have a materially different defect.
- If none of that occurs, add a concise progress-log entry explaining why the work had no material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: #2190 (the inventory that recorded this defect)
- Related PRs: #2193 (the EPIC specification that produced this draft)
- Related ADRs: `None` yet; see Architectural Decisions
