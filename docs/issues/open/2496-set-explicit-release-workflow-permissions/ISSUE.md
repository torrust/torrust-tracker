---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: null
github-issue: 2496
spec-path: docs/issues/open/2496-set-explicit-release-workflow-permissions/ISSUE.md
branch: "2496-set-explicit-release-workflow-permissions"
related-pr: null
last-updated-utc: "2026-10-08 17:05"
semantic-links:
  skill-links:
    - create-issue
    - implement-workflow
  related-artifacts:
    - "issue #2493"
    - .github/workflows/container.yaml
    - .github/workflows/deployment.yaml
    - .github/workflows/deployment-packages.yaml
    - docs/security/analysis/github-security/README.md
---

# Issue #2496 - Set explicit permissions for release publishing workflows

## Goal

Declare the minimum required `GITHUB_TOKEN` permissions for every release-publishing job currently
relying on repository defaults, without changing release behavior or credential access.

## Background

CodeQL reports seven missing-permission findings in jobs that publish crates or production-grade
container images:

- alerts 80 and 104 in `container.yaml`;
- alerts 78, 79, and 82 in `deployment-packages.yaml`;
- alerts 75 and 81 in `deployment.yaml`.

The repository default is currently read-only, so these findings do not demonstrate a present
write-token exploit. They are release-supply-chain hardening: a future repository-default change
must not silently broaden publishing-job permissions.

## Scope

### In Scope

- Determine the minimum `GITHUB_TOKEN` permissions required by each affected job.
- Add explicit workflow- or job-level permissions for all seven alerts.
- Preserve Cargo registry and Docker Hub publishing behavior.
- Verify that environment secrets remain available only to their intended publishing jobs.
- Confirm CodeQL closes the seven alerts after merge.

### Out of Scope

- The twenty CI/build permission findings tracked by a separate draft.
- Rotating Cargo or Docker Hub credentials without evidence of compromise.
- Changing release branch strategy, environment approvals, or publication semantics.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None expected. Create one only if implementation changes the release trust model
  rather than declaring existing minimum permissions.

## Design and Ownership Review

Follow the `implement-workflow` skill. Review permissions at the narrowest practical scope and map
each permission to the exact action or GitHub API operation that requires it.

## Bug-Fix Process

Not applicable. Current repository defaults are read-only; this is proactive release-supply-chain
hardening rather than correction of a demonstrated privilege escalation.

## Regression Test Strategy

Use workflow validation and real release-path evidence. Static YAML validation alone cannot prove
that effective permissions remain sufficient, so manual verification must inspect representative
workflow runs without publishing an unintended release.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Inventory permission needs | Map each affected job and action to required token permissions. |
| T2 | TODO | Add explicit release permissions | Use the narrowest workflow- or job-level declarations that preserve behavior. |
| T3 | TODO | Validate publishing paths | Verify representative container and crate release paths retain required access. |
| T4 | TODO | Reconcile scanner alerts | Confirm alerts 75, 78, 79, 80, 81, 82, and 104 close after merge. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Explicit permissions for the three release workflows | Commit together after workflow lint and permission review. |
| T3-T4 | Verification and scanner evidence | Commit separately only if durable evidence files change. |

## Progress Tracking

### Workflow Checkpoints

- [x] Draft specification created from the #2493 triage
- [x] Draft reviewed and approved by maintainer
- [x] GitHub issue created and issue number added
- [ ] Workflow changes implemented
- [ ] Automatic and manual verification completed
- [ ] Seven CodeQL alerts reconciled
- [ ] Independent review completed
- [ ] Issue closed and spec archived

### Progress Log

- 2026-10-08 16:55 UTC - Copilot - Drafted from the release-supply-chain subset of GitHub
  Security cluster GSF-001.
- 2026-10-08 17:05 UTC - Maintainer - Approved the focused draft; Copilot created GitHub issue
  #2496 and promoted the specification to the open issue catalog.

## Acceptance Criteria

- [ ] All seven affected publishing jobs declare explicit minimum `GITHUB_TOKEN` permissions.
- [ ] Every granted permission has a documented consumer.
- [ ] Cargo and container publishing behavior remains unchanged.
- [ ] No environment secret becomes available to an additional job or trigger.
- [ ] CodeQL alerts 75, 78, 79, 80, 81, 82, and 104 close.
- [ ] Workflow linting and `linter all` pass.

## Verification Plan

### Automatic Checks

- Run repository YAML and workflow linting.
- Run the workflow-specific validation required by `implement-workflow`.
- Run `linter all`.

### Manual Verification Scenarios

| ID | Scenario | Steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Review effective release permissions | Inspect each affected job and map declared permissions to actions and commands. | No job inherits repository defaults or receives an unexplained permission. | TODO | `manual-verification-evidence.md` |
| M2 | Exercise a safe release path | Use the repository-approved non-publishing or controlled verification path for each release workflow. | Jobs authenticate and prepare artifacts without unintended publication. | TODO | `manual-verification-evidence.md` |
| M3 | Verify CodeQL closure | Inspect the post-merge Code Scanning run. | All seven scoped alerts are closed and no replacement permission alert appears. | TODO | `manual-verification-evidence.md` |

## Acceptance Verification

Record the final permission matrix, validation run URLs, CodeQL results, and any environment
approval required for safe release-path verification.

## Implementation Completion Review

Create a retrospective if effective permissions differ materially from the expected read-only
baseline or reveal an undocumented release trust boundary. Otherwise record that explicit
declarations preserved the existing model.
