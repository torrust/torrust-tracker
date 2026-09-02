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
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/testing/README.md
    - docs/testing/refactoring-patterns/README.md
---

<!-- skill-link: create-issue -->

# EPIC #1347 - Overhaul: Packages Testing

## Goal

Improve maintainable automated test coverage across the current Torrust Tracker workspace packages, prioritizing critical behavior and making the published crates robust and reliable for consumers.

## Why This Is Needed

The repository was reorganized through package refactoring and extraction work. Its end-to-end suite provides valuable broad coverage, but packages also need focused unit tests that exercise their responsibilities in isolation. Test work should reveal design and refactoring opportunities while preserving readable tests that serve as behavioral contracts. The resulting package-local safety nets support contributors who make focused changes to independently publishable packages.

## Scope

### In Scope

- Establish and record a coverage baseline for each package addressed by a subissue, then aim to increase it by testing critical behavior. Record an issue-local, human-readable coverage-evidence document with the command, measurement scope, aggregate comparison, per-file results, and prioritized uncovered areas.
- Add maintainable, fast, responsibility-oriented unit tests close to the code they protect, using Arrange, Act, Assert (AAA) structure where appropriate.
- Add integration tests, runnable examples, or end-to-end tests when they provide valuable package-level regression protection.
- Use `tracker-core` as a reference for effective package-level test coverage.
- Create and track additional package-testing subissues when a maintainer or contributor identifies a need.
- Set qualitative, risk-based coverage objectives per subissue and document the rationale and exceptions; do not require a numeric percentage target.
- Review every test-producing increment before beginning the next one, then stop for maintainer review after the final test-producing increment and before final verification, committing, or opening a pull request.

### Out of Scope

- A uniform coverage-percentage threshold for every package.
- Replacing the existing end-to-end test suite.
- Unrelated production refactoring not justified by improving testability.
- Treating coverage percentage as proof of correct behavior.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue                                                 | Local Spec                                                            | Status | Notes                                                                                                                |
| ----- | ----------------------------------------------------- | --------------------------------------------------------------------- | ------ | -------------------------------------------------------------------------------------------------------------------- |
| 1     | #1348 - Add tests to the axum-http-server package     | `docs/issues/open/1348-1347-add-tests-axum-http-server/ISSUE.md`      | DONE   | Added fast package-local response, request-ID, and lifecycle tests; verification and final review evidence recorded. |
| 2     | #1349 - Add tests to the axum-rest-api-server package | `docs/issues/open/1349-1347-add-tests-axum-rest-api-server/ISSUE.md`  | TODO   | Existing subissue; package-level test work.                                                                          |
| 3     | Additional package-testing subissues                  | Create a folder-style spec when a concrete package need is identified | TODO   | Permitted but not required upfront; retain scope in this EPIC.                                                       |

## Package Coverage Tracking

Add a row only when work begins on a package subissue. Record its baseline before adding tests and
its latest measurement after implementation. Link each row to the subissue's issue-local
`coverage-evidence.md`, which remains the source of truth for measurement scope, per-file detail,
and prioritized gaps. These aggregate values show progress across the EPIC; they do not determine
whether a subissue has adequately covered critical behavior.

| Package                            | Subissue                                               | Baseline                                          | Latest                                            | Change                                                  | Evidence                                                                       |
| ---------------------------------- | ------------------------------------------------------ | ------------------------------------------------- | ------------------------------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------ |
| `torrust-tracker-axum-http-server` | [#1348](1348-1347-add-tests-axum-http-server/ISSUE.md) | Lines: 93.82%; regions: 91.66%; functions: 89.54% | Lines: 95.00%; regions: 93.10%; functions: 91.06% | Lines: +1.18 pp; regions: +1.44 pp; functions: +1.52 pp | [Coverage evidence](1348-1347-add-tests-axum-http-server/coverage-evidence.md) |

## Delivery Strategy

Implement independently reviewable, package-scoped subissues. When work begins on a package, add it to the Package Coverage Tracking table and record its starting coverage before adding tests. After implementation, update the row with the latest measurement and percentage-point change. Each subissue records its starting coverage, the critical responsibilities assessed, the coverage increase achieved where practical, verification evidence, and any explicitly justified exclusions. Store the coverage evidence in an issue-local human-readable document, rather than committing large raw coverage artifacts. State which source paths and code types the measurement includes, because test-inclusive totals are not production-only coverage. Use aggregate percentages only for navigation; prioritize behavior by examining per-file coverage and uncovered functions or regions.

Prioritize fast unit tests close to the code being changed, while retaining or adding integration, runnable-example, and end-to-end tests when they provide valuable regression protection. Coverage percentage informs the work but does not replace testing critical behavior. Record reusable test-design refactors in the [testing refactoring-pattern catalog](../../../testing/refactoring-patterns/README.md) so later subissues can apply proven patterns without restating their rationale.

For every test-producing task, apply this development loop:

1. Add the smallest behavior-focused test increment.
2. Review the changed tests before beginning the next test-producing task: remove duplication, extract only justified mechanical helpers, improve naming and AAA structure, and prefer expressive assertions.
3. Run focused tests and correct failures.
4. After the final test-producing task, stop and request maintainer review before final verification, committing, or opening a pull request.
5. Address review feedback, then complete verification and acceptance review.

For multi-input protocol behavior, scenarios should own every related artifact that describes the example, including selector request fields, domain input, and independently specified expected output. Builders may hide irrelevant fields of an individual artifact. Do not derive expected values by calling production mapping or serialization code under test. Keep the production-boundary invocation, concrete expected representation, and final actual-versus-expected assertion visible; helpers may encapsulate only repeated mechanics such as successful-response decoding.

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
- [x] Subissue statuses kept up to date in the `Subissues` table
- [x] For each implemented subissue: automatic checks completed and recorded
- [x] For each implemented subissue: manual verification completed and recorded
- [x] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-01 17:48 UTC - GitHub Copilot - Created repository-local EPIC specification from GitHub issue #1347 and recorded maintainer feedback: cover all current workspace packages, allow additional subissues as needs are identified, and use baselines with risk-based targets. - https://github.com/torrust/torrust-tracker/issues/1347
- 2026-09-01 18:00 UTC - User/maintainer - Approved the EPIC direction and clarified that each package should build a local safety net: establish and increase the coverage baseline, prioritize fast tests close to the code, and use unit, integration, or end-to-end tests whenever they provide valuable regression protection. - https://github.com/torrust/torrust-tracker/issues/1347
- 2026-09-01 - GitHub Copilot - Completed #1348 with fast package-local response-adapter, request-ID middleware, and registration-cleanup tests; automated checks, directly invoked real-server verification scenarios, and final review evidence are recorded in its subissue specification. - https://github.com/torrust/torrust-tracker/issues/1348

## Acceptance Criteria

- [ ] All required package-testing subissues are created and linked.
- [x] Implementation order and package-scoped delivery strategy are explicit.
- [x] Coverage policy requires a recorded baseline, an aim to increase it, and prioritization of critical behavior over an arbitrary percentage for each subissue.
- [ ] Dependencies, blockers, and remaining package needs are documented and current.
- [x] Epic status reflects actual state of linked subissues.
- [x] Every completed subissue includes automated verification evidence.
- [x] Every completed subissue includes manual verification evidence.
- [x] Every completed subissue includes post-implementation acceptance criteria review.
- [x] Documentation and governance updates are included when required.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                              |
| ----- | ---------------------- | --------------------------------------------------------------------------------------------------------------------- |
| AC1   | TODO                   | EPIC subissue table and GitHub issue links                                                                            |
| AC2   | DONE                   | Delivery strategy in this spec                                                                                        |
| AC3   | DONE                   | Scope and delivery strategy in this spec                                                                              |
| AC4   | TODO                   | Subissue specs and progress logs                                                                                      |
| AC5   | DONE                   | #1348 status in the subissues table and its specification                                                             |
| AC6   | DONE                   | #1348 automatic-verification records                                                                                  |
| AC7   | DONE                   | #1348 manual-verification records                                                                                     |
| AC8   | DONE                   | #1348 post-implementation acceptance-verification record                                                              |
| AC9   | DONE                   | #1348 specification and this EPIC update; no additional behavior, workflow, or governance documentation was required. |

## Risks and Trade-offs

- Coverage percentage can conceal critical low-coverage files behind strong aggregate results; mitigate it by maintaining per-file and uncovered-area evidence, then selecting behavior by risk rather than pursuing a percentage target.
- Raw coverage formats can be too large or tool-oriented for code review; mitigate this by committing a concise, human-readable issue-local evidence document and retaining the reproducible command instead.
- Testing may expose design seams that are difficult to isolate; make small testability refactorings only when justified and keep unrelated refactoring out of scope.
- New packages or package extractions can change the inventory during the EPIC; add concrete subissues as needs are identified and record deferrals explicitly before closing the EPIC.

## References

- GitHub EPIC: https://github.com/torrust/torrust-tracker/issues/1347
- Related issues: #1348, #1349
- Package inventory: `docs/packages.md`
- Reference package: `packages/tracker-core/`
- Coverage tooling: `cargo llvm-cov`
- Related historical work: #753, #1181, #1226, #1266
- Test-pattern catalog: `docs/testing/refactoring-patterns/README.md`
