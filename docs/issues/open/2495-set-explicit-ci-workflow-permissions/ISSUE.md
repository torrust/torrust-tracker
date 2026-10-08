---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: null
github-issue: 2495
spec-path: docs/issues/open/2495-set-explicit-ci-workflow-permissions/ISSUE.md
branch: "2495-set-explicit-ci-workflow-permissions"
related-pr: null
last-updated-utc: "2026-10-08 18:10"
semantic-links:
  skill-links:
    - create-issue
    - implement-workflow
  related-artifacts:
    - "issue #2493"
    - .github/workflows/container.yaml
    - .github/workflows/coverage.yaml
    - .github/workflows/db-benchmarking.yaml
    - .github/workflows/db-compatibility.yaml
    - .github/workflows/docs-lint.yaml
    - .github/workflows/generate_coverage_pr.yaml
    - .github/workflows/labels.yaml
    - .github/workflows/os-compatibility.yaml
    - .github/workflows/testing.yaml
    - docs/security/analysis/github-security/README.md
---

# Issue #2495 - Set explicit permissions for CI workflows

## Goal

Declare minimum `GITHUB_TOKEN` permissions for the twenty validation and CI jobs currently relying
on repository defaults, while preserving fork safety, self-hosted-runner isolation, coverage, and
label automation.

## Background

CodeQL reports twenty missing-permission findings across validation workflows:

- alerts 77 and 103 in `container.yaml`;
- alert 69 in `coverage.yaml`;
- alerts 70, 72, and 74 in `db-benchmarking.yaml`;
- alerts 73 and 76 in `db-compatibility.yaml`;
- alert 83 in `docs-lint.yaml`;
- alerts 84, 86, 88, and 91 in `generate_coverage_pr.yaml`;
- alerts 87 and 90 in `labels.yaml`;
- alert 89 in `os-compatibility.yaml`;
- alerts 92–94 and 105 in `testing.yaml`.

The current repository token default and fork pull-request tokens are read-only. No present
write-token exploit was demonstrated. Explicit declarations prevent a future default change from
silently increasing CI privileges.

## Scope

### In Scope

- Determine the minimum permissions for each affected validation and CI job.
- Add explicit workflow- or job-level permissions for all twenty alerts.
- Preserve fork pull-request behavior and environment-secret boundaries.
- Review the self-hosted container job for shared-runner or cache pivots.
- Confirm the twenty CodeQL alerts close after merge.

### Out of Scope

- The seven publishing-job findings tracked by the release workflow draft.
- Redesigning self-hosted runner infrastructure.
- Changing coverage architecture, label semantics, or test matrices except where required to
  preserve behavior under explicit permissions.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None expected. Create one only if the work changes the CI trust boundary or
  self-hosted-runner security model.

## Design and Ownership Review

Follow the `implement-workflow` skill. Separate jobs needing no token (`permissions: {}`) from jobs
needing `contents: read` or a narrowly justified write permission. Pay special attention to
coverage publication, labels, generated coverage pull requests, and self-hosted runner jobs.

## Bug-Fix Process

Not applicable. This is defense-in-depth hardening under a currently read-only token default.

## Regression Test Strategy

Validate syntax and exercise representative pull-request, branch, scheduled, and manual triggers.
Fork behavior and jobs using self-hosted runners or repository mutation need explicit manual
evidence.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Build the CI permission matrix | Map twenty alerts to actual GitHub API and checkout requirements. |
| T2 | TODO | Add explicit CI permissions | Use the narrowest declarations and preserve trigger-specific behavior. |
| T3 | TODO | Verify sensitive CI paths | Check fork PRs, coverage, labels, generated PRs, and self-hosted container jobs. |
| T4 | TODO | Reconcile scanner alerts | Confirm all twenty scoped alerts close after merge. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Explicit permissions for CI workflows | Split implementation commits only when workflow groups have distinct behavior and validation. |
| T3-T4 | Verification and scanner evidence | Commit separately only if durable evidence files change. |

## Progress Tracking

### Workflow Checkpoints

- [x] Draft specification created from the #2493 triage
- [x] Draft reviewed and approved by maintainer
- [x] GitHub issue created and issue number added
- [ ] Workflow changes implemented
- [ ] Automatic and manual verification completed
- [ ] Twenty CodeQL alerts reconciled
- [ ] Independent review completed
- [ ] Issue closed and spec archived

### Progress Log

- 2026-10-08 16:55 UTC - Copilot - Drafted from the CI/build subset of GitHub Security cluster
  GSF-001.
- 2026-10-08 17:05 UTC - Maintainer - Approved the focused draft; Copilot created GitHub issue
  #2495 and promoted the specification to the open issue catalog.
- 2026-10-08 18:10 UTC - Copilot - Aligned the layout with `docs/templates/ISSUE.md` (Risks and Trade-offs,
  References, Acceptance Verification under Verification Plan) for PR #2498 review finding
  `review-finding:pr-2498-f8`.

## Acceptance Criteria

- [ ] All twenty affected CI jobs declare explicit minimum `GITHUB_TOKEN` permissions.
- [ ] Jobs that do not need a token receive no token permissions.
- [ ] Every non-empty permission grant has a documented consumer.
- [ ] Fork pull requests cannot access write permissions or protected secrets.
- [ ] Self-hosted runner and cache boundaries do not create an undocumented release pivot.
- [ ] CodeQL alerts 69, 70, 72, 73, 74, 76, 77, 83, 84, 86, 87, 88, 89, 90, 91,
  92, 93, 94, 103, and 105 close.
- [ ] Workflow linting and `linter all` pass.

## Verification Plan

### Automatic Checks

- Run repository YAML and workflow linting.
- Run workflow-specific validation required by `implement-workflow`.
- Run `linter all`.

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Verify fork safety | Inspect or run a fork pull-request path after explicit permissions are applied. | No write permission or protected secret is exposed. | TODO | `manual-verification-evidence.md` |
| M2 | Verify mutation jobs | Exercise labels and generated coverage PR behavior through the repository-approved safe path. | Required mutations succeed with only documented permissions. | TODO | `manual-verification-evidence.md` |
| M3 | Verify self-hosted boundary | Review effective permissions, cache access, and secrets for self-hosted container jobs. | No undocumented pivot to release publishing exists. | TODO | `manual-verification-evidence.md` |
| M4 | Verify CodeQL closure | Inspect the post-merge Code Scanning run. | All twenty scoped alerts close without replacement findings. | TODO | `manual-verification-evidence.md` |

### Acceptance Verification

Record the final permission matrix, representative run URLs, fork behavior, self-hosted runner
review, and CodeQL closure evidence.

## Risks and Trade-offs

- A permission set that is too narrow breaks coverage publication, labels, or generated coverage
  pull requests. Mitigation: build the permission matrix first (T1) and exercise each mutation
  job (M2).
- A broad workflow-level grant hides which job needs a permission. Mitigation: prefer job-level
  declarations and document every non-empty grant.

## Implementation Completion Review

Create a retrospective if the work reveals a new CI-to-release pivot or requires changes to the
self-hosted runner model. Otherwise record that explicit permissions preserved the existing
read-only boundary.

## References

- Related issues: #2493, #2496
- Related PRs: #2498 (specification)
- Related ADRs: None
