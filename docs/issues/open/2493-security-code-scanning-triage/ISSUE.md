---
schema-version: 1
doc-type: issue
issue-type: task
status: in-review
priority: p2
epic: null
github-issue: 2493
spec-path: docs/issues/open/2493-security-code-scanning-triage/ISSUE.md
branch: "2493-security-code-scanning-triage"
related-pr: null
last-updated-utc: "2026-10-08 17:08"
semantic-links:
  skill-links:
    - create-issue
    - triage-github-security-findings
    - catalog-security-vulnerabilities
  related-artifacts:
    - docs/security/README.md
    - docs/security/public-scanner-findings.md
    - docs/security/analysis/README.md
    - SECURITY.md
---

<!-- skill-link: create-issue -->

# Issue #2493 - Triage GitHub Security scanner findings and define the recurring process

## Goal

Define and execute the first recurrent review of the repository's public GitHub Security tab
findings (Code Scanning and Code Quality), then turn the validated clusters into individual,
reviewable issues instead of tracking all findings as one long-lived backlog.

## Background

The GitHub Security tab exposes a large set of public findings. In practice, many are duplicates
of the same underlying problem in multiple files or workflow jobs. Without a review process, the
backlog becomes noisy and difficult to fix in a controlled way.

The repository already has a public security-analysis process for CVEs and a confidential
vulnerability-remediation process for private reports. The missing piece is a specific process for
GitHub Security code-scanning and code-quality findings: cluster them, classify them, and create
separate issue specs by cause rather than by occurrence.

## Scope

### In Scope

- Define the repository process for reviewing GitHub Security code-scanning and code-quality
  findings.
- Add a focused AI skill that routes and executes this public scanner workflow without merging it
  with CVE, container-scan, or confidential-remediation workflows.
- Review the current findings and cluster repeated issues by root cause and affected subsystem.
- Record whether each cluster is affecting, non-affecting, or a hardening gap.
- Assign priority from the affected trust boundary, reachability, privileges, and production or
  release-supply-chain impact instead of copying scanner severity.
- Create one issue spec per root cause class, with the `Security` label and an appropriate
  priority, instead of using a single broad EPIC.
- Maintain an inventory of findings already triaged and accepted as non-applicable or already
  mitigated.
- Use the process as a repeatable template for future security-tab review cycles.

### Out of Scope

- Private or embargoed vulnerabilities.
- New dependency or tool decisions unrelated to a verified scanner triage result.
- Creating a general-purpose security dashboard or automation beyond the documented review
  workflow.

## Proposed Process

1. Refresh the public security tab for Code Scanning and Code Quality.
2. Export and review the complete list in a single inventory.
3. Deduplicate by rule name, code pattern, root cause, and affected subsystem.
4. Classify each cluster as affecting, hardening, non-affecting, or needs-investigation.
5. Assign the affected surface and project priority independently of scanner severity.
6. Open one issue per finding class with clear scope and acceptance criteria.
7. Keep a public inventory of accepted and non-applicable findings to avoid reanalyzing the
   same item from scratch.
8. Schedule the next review and keep the process lightweight enough to run regularly.

This workflow is documented in [docs/security/public-scanner-findings.md](../../../security/public-scanner-findings.md).

## Architectural Decisions

- Keep security workflows as multiple focused skills with explicit routing.
- Keep the process documents authoritative; skills provide discovery, routing, and concise
  executable steps rather than duplicating the full policy.
- Distinguish runtime and release-supply-chain risk from CI/build and local development risk.
  Scanner severity remains evidence and does not determine issue priority.
- Do not create an EPIC for the recurring alert backlog. The stable process and inventory provide
  continuity, while remediation is tracked through focused issues.
- ADRs to create: `None known`. Revisit only if security workflow ownership or confidentiality
  boundaries change materially.

## Design and Ownership Review

Not applicable. This task changes documentation, issue planning, and AI workflow guidance; it does
not introduce runtime resources or ownership boundaries.

## Bug-Fix Process

Not applicable. This task establishes and runs a triage process. Any confirmed defect discovered
during triage receives its own bug specification and follows the `fix-bug` skill.

## Regression Test Strategy

Not applicable. Process validation uses inventory coverage, semantic-link review, Markdown
validation, and GitHub issue metadata checks.

## Example Cluster Types

The following are examples of the kinds of clusters we expect to review, not a claim that they
all currently exist in the repository:

- workflow permission hardening issues appearing across multiple GitHub Actions YAML files;
- repeated file-handle lifecycle findings generated in multiple paths or tests;
- common filesystem / environment handling issues that share the same root cause;
- scanner results that are valid, but already mitigated by project architecture or runtime
  constraints;
- false positives or intentionally accepted code patterns that are documented as non-applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Document the recurring security-tab triage process | Canonical policy in [`docs/security/public-scanner-findings.md`](../../../security/public-scanner-findings.md), linked from the security indexes. |
| T2 | DONE | Add and route the focused AI skill | Added `triage-github-security-findings`; narrowed the CVE catalog skill's discovery scope and routed GitHub findings correctly. |
| T3 | DONE | Review the current GitHub Security findings | Captured and reconciled 66 Code Scanning alerts and 2 Code Quality findings into 11 root-cause clusters. |
| T4 | DONE | Assign trust tier and priority | Recorded affected surface, reachability, privilege, pivot potential, and maintainer priority independently of scanner severity. |
| T5 | DONE | Draft and approve focused issue specs | Maintainer approved three independently deliverable specs. |
| T6 | DONE | Create approved GitHub issues | Created #2497, #2496, and #2495 for zlib, release permissions, and CI permissions respectively, then replaced temporary links. |
| T7 | DONE | Record non-applicable and accepted findings | Added the GitHub Security findings catalog, dated review, and runtime CVE bulk record. |
| T8 | DONE | Schedule the next review cycle | Next review due 2027-01-08, before the next release if earlier, or on any recheck trigger. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1-T2 | Canonical process, security-doc routing, and focused skill | Commit together after skill-link and documentation validation because the process and adapter must remain synchronized. |
| T3-T5, T7 | Initial alert inventory, classifications, priorities, catalog, and focused draft specs | Commit after coverage reconciliation proves every source alert maps to one cluster. |
| T6 | Approved GitHub issues and stable links | Commit after maintainer review and issue creation. |
| T8 | Review cadence and final evidence | Include with the catalog commit unless it becomes an independently reviewable process correction. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted and approved by the maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Process and focused AI skill completed
- [x] Current findings captured, clustered, and classified
- [x] Focused issue specs reviewed and created
- [x] Implementation completed
- [x] Automatic verification completed
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation
- [x] Evidence-based completion review recorded
- [ ] Reviewer validated acceptance criteria
- [x] Committer verified spec progress before commit
- [ ] Issue closed and spec moved to `docs/issues/closed/`

## Acceptance Criteria

- [x] The public triage process is documented and linked from the security docs.
- [x] A focused GitHub Security findings skill exists and routes safely among the repository's
  distinct security workflows.
- [x] The current security-tab backlog is reviewed and clustered by root cause.
- [x] Findings are classified as affecting, hardening, need-investigation, or non-affecting.
- [x] Every cluster records its affected security surface, reachability, privilege or credential
  impact, higher-tier pivot potential, and maintainer-assigned priority rationale.
- [x] A set of smaller issue specs is created for each real cluster rather than one umbrella issue.
- [x] The catalog records accepted and non-applicable findings so they are not lost.
- [x] The next review date and cadence are recorded.
- [x] `linter markdown`, `linter cspell`, and relevant semantic-link checks pass.

## Verification Plan

### Automatic Checks

- `linter markdown`
- `linter cspell`
- `linter all` before commit
- Repository searches confirm every declared skill link resolves to an existing skill and every
  child issue number, title, and specification path match GitHub metadata.

### Manual Verification Scenarios

| ID | Scenario | Human-oriented steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Verify skill routing | Compare the four security finding sources against the routing table in the new skill and process document. | GitHub source findings, public CVEs, image scans, and confidential reports each route to the correct workflow without overlap. | DONE | `manual-verification-evidence.md` V1 |
| M2 | Verify initial inventory coverage | Reconcile the dated review rows against the open Code Scanning and Code Quality lists. | Every source alert maps to exactly one cluster, with no unexplained duplicate or omission. | DONE | `manual-verification-evidence.md` V2 |
| M3 | Verify issue granularity | Review each proposed cluster against root cause, remediation, owner, effort, and verification boundaries. | Each focused issue is independently deliverable; oversized clusters are split before creation. | DONE | `manual-verification-evidence.md` V3 |

## Acceptance Verification

Verified on 2026-10-08:

- Sources: 66 Code Scanning alerts plus 2 Code Quality findings.
- Coverage: all 68 findings map exactly once to 11 clusters.
- Dispositions: 27 hardening, 2 needs investigation, 39 non-affecting, and 0 confirmed affecting.
- Follow-up issues: #2497 for native zlib reachability, #2496 for release workflow permissions,
  and #2495 for CI workflow permissions.
- Catalog: `docs/security/analysis/github-security/README.md`.
- Dated evidence: `docs/security/analysis/github-security/reviews/2026-10-08.md`.
- Next review: 2027-01-08, before the next release if earlier, or on a catalog recheck trigger.
- Validation: exact alert-set reconciliation, v1 frontmatter validation, `linter markdown`,
  `linter cspell`, `linter lychee`, and the complete pre-commit gate all passed.

## Implementation Completion Review

No separate `implementation-retrospective.md` is required. The first cycle validated the
classification and skill-routing model. Two reusable corrections were incorporated directly into
the canonical process and evidence:

- durable decisions use the repository's established **catalog** term, while dated source coverage
  is a **review inventory**;
- package presence alone is insufficient, but a previous non-affecting verdict must be reopened
  when its own recheck condition is demonstrably true, as occurred with native zlib linkage.

## Progress Log

- 2026-10-08 16:18 UTC - Copilot - Drafted the repeatable GitHub security triage process and
  the first issue spec to execute the initial review cycle.
- 2026-10-08 16:24 UTC - Maintainer - Approved the no-EPIC approach, recurring process, initial
  review, and focused issue creation; requested an AI skill architecture review before triage.
- 2026-10-08 16:29 UTC - Copilot - Added the focused-skill design: canonical policy remains in
  security documentation, while separate skills route public GitHub findings, CVEs, image scans,
  and confidential reports.
- 2026-10-08 16:30 UTC - Maintainer - Clarified that priority must follow the affected trust
  boundary: shipped runtime and production dependencies first, CD and release injection at the
  same high tier, CI in the middle unless it provides a higher-tier pivot, and local development
  or test tooling normally lower.
- 2026-10-08 16:45 UTC - Copilot - Created the issue branch from current upstream `develop`;
  completed the canonical process, trust-tier priority model, security workflow routing, focused
  skill, and documentation indexes. Targeted Markdown, spelling, link, and whitespace checks pass.
- 2026-10-08 16:58 UTC - Copilot - Exported all 68 open findings through the GitHub APIs,
  reconciled them into 11 clusters, independently reviewed security impact, cataloged 39
  non-affecting findings, identified one zlib investigation cluster, and drafted separate release
  and CI permission-hardening specs.
- 2026-10-08 17:05 UTC - Maintainer - Approved all three focused specifications. Copilot created
  #2497 for native zlib reachability, #2496 for release workflow permissions, and #2495 for CI
  workflow permissions, then replaced temporary catalog links with stable issue references.
- 2026-10-08 17:08 UTC - Copilot - Committed the catalog, dated review, CVE corrections, manual
  evidence, and three approved issue specs in `d68281286`. The complete pre-commit gate passed,
  including frontmatter validation, dependency checks, nightly formatting, all linters,
  Containerfile linting, and workspace documentation tests.

## Notes

The intent is to keep this workflow recurring and efficient. Using a lightweight review process
and per-cluster issue specs avoids a large EPIC with poor reviewability while preserving a public
record of accepted findings and actual remediation tasks.
