---
schema-version: 1
doc-type: issue
issue-type: task
status: open
priority: p3
epic: 1347
github-issue: 2301
spec-path: docs/issues/open/2301-1347-review-package-coverage-rollout/ISSUE.md
branch: "2301-review-package-coverage-rollout"
related-pr: null
last-updated-utc: 2026-09-22 13:05
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/workflows/generate_coverage_pr.yaml
    - docs/issues/closed/2222-1347-package-coverage-regression-ci/ISSUE.md
    - docs/issues/closed/2222-1347-package-coverage-regression-ci/manual-verification-evidence.md
    - docs/issues/open/1347-overhaul-packages-testing/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2301 - Review Package Coverage Rollout After Testing EPIC Completion

Parent EPIC: #1347 - Overhaul: Packages Testing

## Goal

After #1347 is complete, review accumulated evidence from the report-only package coverage
comparison introduced by #2222. Decide whether to retain its current warning threshold and
report-only status, change it, or create a separately approved required-check proposal.

## Background

Issue #2222 introduced a non-blocking pull-request coverage comparison for directly changed
workspace packages. PR #2293 demonstrated the new-package and fork outcomes, but comparable
package and warning scenarios have not yet occurred in a hosted pull request. The remaining
package-testing work in #1347 provides the appropriate real changes from which to collect that
evidence, without creating artificial coverage regressions solely for workflow verification.

## Scope

### In Scope

- Collect hosted `package-coverage-regression` evidence from relevant #1347 package-testing PRs.
- Review comparable-package, unavailable-package, and warning outcomes that occur naturally.
- Review job duration, artifact handling, summary clarity, and matrix behavior.
- Reconcile the deferred M2 and M3 scenarios and completion records in #2222's local evidence.
- Record a maintainer decision to retain, revise, or retire the report-only behavior.

### Out of Scope

- Making the coverage report a required check without a separate approved implementation issue.
- Adding an absolute package coverage target.
- Creating disposable pull requests that deliberately reduce coverage only to produce evidence.
- Changing Codecov's existing project or patch reporting.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: None known. Create one only if a future decision changes the CI trust or
  permission boundary.

## Design and Ownership Review

Not applicable. This task reviews existing workflow behavior and documentation; it does not add
child-process, asynchronous I/O, network-readiness, resource-lifetime, or reusable-fixture logic.

## Bug-Fix Process

Not applicable. This task is a deferred rollout review, not a defect report.

## Regression Test Strategy

Not applicable. This task does not change production or maintained test behavior. Any later
workflow change requires its own issue and test strategy.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, `DEFERRED`.

| ID | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1 | TODO | Collect #1347 rollout evidence | Record hosted comparable and unavailable outcomes, plus any naturally occurring warning, from package-testing PRs. |
| T2 | TODO | Review operational behavior | Assess runtime, dynamic-matrix selection, artifacts, summary clarity, and fork safety using hosted evidence. |
| T3 | TODO | Make a maintainer decision | Record whether the check remains report-only and whether its threshold needs revision; create a separate issue for any behavioral change. |
| T4 | TODO | Reconcile completion records | Update #2222 evidence and the #1347 EPIC registration with the decision and evidence links. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Hosted rollout evidence record | Documentation-only commit after reviewed evidence is available. |
| T3-T4 | Decision and reconciled issue/EPIC records | Documentation-only commit after maintainer decision. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style draft created in `docs/issues/drafts/1347-review-package-coverage-rollout/ISSUE.md`.
- [x] Spec reviewed and approved by user/maintainer.
- [x] GitHub issue #2301 created as a subissue of #1347 and issue number added to this spec.
- [ ] #2222 receives a closing comment linking this deferred review.
- [ ] Hosted rollout evidence collected from #1347 package-testing pull requests.
- [ ] Maintainer decision recorded.
- [ ] #2222 and #1347 completion records reconciled.

### Progress Log

- 2026-09-22 13:05 UTC - GitHub Copilot - Drafted the deferred post-EPIC review after #2222 merged through PR #2293. The task intentionally relies on real #1347 package-testing pull requests rather than artificial coverage-regression PRs.
- 2026-09-22 13:05 UTC - GitHub Copilot - Created GitHub issue #2301 and attached it as a subissue of #1347 after maintainer approval.

## Acceptance Criteria

- [ ] AC1: Hosted evidence covers a comparable existing package and reports its base/head counts and delta.
- [ ] AC2: Available hosted exceptional outcomes, including the #2293 new-package result, are reviewed and linked.
- [ ] AC3: Runtime, artifact handling, matrix selection, summary clarity, and fork-safety evidence are reviewed.
- [ ] AC4: A maintainer decision records whether the report remains report-only and whether the warning threshold changes.
- [ ] AC5: Any proposal to make the report required is tracked by a separate approved issue.
- [ ] AC6: #2222 and #1347 documentation accurately record the rollout outcome and remaining follow-up.
- [ ] `linter all` exits with code `0` for documentation changes.
- [ ] Manual verification scenarios are executed and documented.

## Verification Plan

### Automatic Checks

- `linter all` after documentation updates.
- Relevant GitHub Actions workflow checks on the package-testing pull requests used as evidence.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented steps | Expected Result | Status | Evidence |
| --- | -------- | -------------------- | --------------- | ------ | -------- |
| M1 | Comparable package review | Open a completed #1347 package-testing PR and inspect its coverage workflow summary. | Selected package has exact base/head counts and a non-blocking delta outcome. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Rollout decision review | Review collected workflow summaries with the maintainer after #1347 completion. | A recorded decision retains, revises, or retires the report-only behavior without silently making it required. | TODO | `manual-verification-evidence.md` section V2 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Hosted package-testing pull request workflow summary |
| AC2 | TODO | #2222 manual verification evidence and hosted pull request links |
| AC3 | TODO | Hosted workflow logs and maintainer review record |
| AC4 | TODO | Maintainer decision in this issue |
| AC5 | TODO | GitHub issue links, if a follow-up is approved |
| AC6 | TODO | Reconciled #2222 and #1347 specifications |

## Risks and Trade-offs

- Waiting for real #1347 package changes may delay complete evidence, but it avoids contrived
  regression pull requests and makes the review representative of intended use.
- A report-only check can be ignored; retaining it requires explicit review of whether its signal
  remains useful before any required-check proposal.

## Implementation Completion Review

After implementation, compare the evidence and decision with this specification. Record material
findings or deviations in an issue-local retrospective when warranted; otherwise add a concise
progress-log entry explaining why no retrospective is needed.

## References

- Parent EPIC: #1347
- Related issue: #2222
- Related PR: #2293
