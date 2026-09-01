---
doc-type: epic
status: open
github-issue: 1347
spec-path: docs/issues/open/1347-overhaul-packages-testing/EPIC.md
epic-owner: josecelano
last-updated-utc: 2026-09-01 17:48
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
---

<!-- skill-link: create-issue -->

# EPIC #1347 - Overhaul: Packages Testing

## Goal

Improve maintainable automated test coverage across the current Torrust Tracker workspace packages, prioritizing critical behavior and making the published crates robust and reliable for consumers.

## Why This Is Needed

The repository was reorganized through package refactoring and extraction work. Its end-to-end suite provides valuable broad coverage, but packages also need focused unit tests that exercise their responsibilities in isolation. Test work should reveal design and refactoring opportunities while preserving readable tests that serve as behavioral contracts. The resulting package-local safety nets support contributors who make focused changes to independently publishable packages.

## Scope

### In Scope

- Establish and record a coverage baseline for each package addressed by a subissue, then aim to increase it by testing critical behavior.
- Add maintainable, fast, responsibility-oriented unit tests close to the code they protect, using Arrange, Act, Assert (AAA) structure where appropriate.
- Add integration tests, runnable examples, or end-to-end tests when they provide valuable package-level regression protection.
- Use `tracker-core` as a reference for effective package-level test coverage.
- Create and track additional package-testing subissues when a maintainer or contributor identifies a need.
- Set qualitative, risk-based coverage objectives per subissue and document the rationale and exceptions; do not require a numeric percentage target.

### Out of Scope

- A uniform coverage-percentage threshold for every package.
- Replacing the existing end-to-end test suite.
- Unrelated production refactoring not justified by improving testability.
- Treating coverage percentage as proof of correct behavior.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue                                                 | Local Spec                                                            | Status | Notes                                                          |
| ----- | ----------------------------------------------------- | --------------------------------------------------------------------- | ------ | -------------------------------------------------------------- |
| 1     | #1348 - Add tests to the axum-http-server package     | `docs/issues/open/1348-1347-add-tests-axum-http-server/ISSUE.md`      | TODO   | Existing subissue; package-level test work.                    |
| 2     | #1349 - Add tests to the axum-rest-api-server package | `docs/issues/open/1349-1347-add-tests-axum-rest-api-server/ISSUE.md`  | TODO   | Existing subissue; package-level test work.                    |
| 3     | Additional package-testing subissues                  | Create a folder-style spec when a concrete package need is identified | TODO   | Permitted but not required upfront; retain scope in this EPIC. |

## Delivery Strategy

Implement independently reviewable, package-scoped subissues. Each subissue records its starting coverage, the critical responsibilities assessed, the coverage increase achieved where practical, verification evidence, and any explicitly justified exclusions. Prioritize fast unit tests close to the code being changed, while retaining or adding integration, runnable-example, and end-to-end tests when they provide valuable regression protection. Coverage percentage informs the work but does not replace testing critical behavior.

For each subissue implementation, the completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run and record manual verification scenarios.
3. Re-review acceptance criteria against observed behavior and update evidence.

### Phase 1

- Outcome: Existing package-testing issues have repository-local specs with clear scope, baselines, and qualitative risk-based coverage objectives.
- Exit criteria: Specs for #1348 and #1349 are approved, and their status is represented accurately in this EPIC.

### Phase 2

- Outcome: Package-scoped testing improvements are delivered incrementally through implementation PRs.
- Exit criteria: Each completed subissue has passing automated-check, manual-verification, and acceptance-review evidence.

### Phase 3

- Outcome: The EPIC’s package coverage objective is assessed and any remaining package needs are represented by tracked subissues or explicit, documented deferrals.
- Exit criteria: All required package-testing work is completed, deferred with rationale, or tracked by a successor issue.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec created in `docs/issues/open/` for existing GitHub issue #1347
- [x] Initial epic-scope, subissue-policy, and risk-based coverage-target feedback collected from user/maintainer
- [x] Epic spec reviewed and approved by user/maintainer
- [x] Existing GitHub epic issue number added to this spec
- [x] Existing subissues linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-01 17:48 UTC - GitHub Copilot - Created repository-local EPIC specification from GitHub issue #1347 and recorded maintainer feedback: cover all current workspace packages, allow additional subissues as needs are identified, and use baselines with risk-based targets. - https://github.com/torrust/torrust-tracker/issues/1347
- 2026-09-01 18:00 UTC - User/maintainer - Approved the EPIC direction and clarified that each package should build a local safety net: establish and increase the coverage baseline, prioritize fast tests close to the code, and use unit, integration, or end-to-end tests whenever they provide valuable regression protection. - https://github.com/torrust/torrust-tracker/issues/1347

## Acceptance Criteria

- [ ] All required package-testing subissues are created and linked.
- [x] Implementation order and package-scoped delivery strategy are explicit.
- [x] Coverage policy requires a recorded baseline, an aim to increase it, and prioritization of critical behavior over an arbitrary percentage for each subissue.
- [ ] Dependencies, blockers, and remaining package needs are documented and current.
- [ ] Epic status reflects actual state of linked subissues.
- [ ] Every completed subissue includes automated verification evidence.
- [ ] Every completed subissue includes manual verification evidence.
- [ ] Every completed subissue includes post-implementation acceptance criteria review.
- [ ] Documentation and governance updates are included when required.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                   |
| ----- | ---------------------- | ------------------------------------------ |
| AC1   | TODO                   | EPIC subissue table and GitHub issue links |
| AC2   | DONE                   | Delivery strategy in this spec             |
| AC3   | DONE                   | Scope and delivery strategy in this spec   |
| AC4   | TODO                   | Subissue specs and progress logs           |
| AC5   | TODO                   | Subissue verification records              |
| AC6   | TODO                   | Subissue verification records              |
| AC7   | TODO                   | Subissue acceptance-verification records   |
| AC8   | TODO                   | Relevant PRs and documentation             |

## Risks and Trade-offs

- Coverage percentage can incentivize low-value tests; mitigate it by recording it as a baseline and prioritizing critical responsibilities and valuable regression protection.
- Testing may expose design seams that are difficult to isolate; make small testability refactorings only when justified and keep unrelated refactoring out of scope.
- New packages or package extractions can change the inventory during the EPIC; add concrete subissues as needs are identified and record deferrals explicitly before closing the EPIC.

## References

- GitHub EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- Related issues: #1348, #1349
- Package inventory: `docs/packages.md`
- Reference package: `packages/tracker-core/`
- Coverage tooling: `cargo llvm-cov`
- Related historical work: #753, #1181, #1226, #1266
