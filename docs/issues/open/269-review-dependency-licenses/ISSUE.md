---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: null
github-issue: 269
spec-path: docs/issues/open/269-review-dependency-licenses/ISSUE.md
branch: "269-first-dependency-license-review"
related-pr: null
last-updated-utc: 2026-08-28 11:10
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - deny.toml
    - .github/workflows/testing.yaml
    - contrib/dev-tools/git/hooks/pre-commit.sh
    - docs/issues/closed/1925-1669-si-31-configure-cargo-deny-for-layer-boundary-enforcement.md
    - docs/issues/open/269-review-dependency-licenses/evidence.md
    - docs/issues/open/269-review-dependency-licenses/initial-review.md
    - docs/issues/open/269-review-dependency-licenses/locked-license-inventory.json
    - docs/issues/open/269-review-dependency-licenses/preliminary-assessment.md
    - docs/issues/open/269-review-dependency-licenses/runtime-license-inventory.json
---

# Issue #269 - Review dependency licenses

## Goal

Identify actual and potential dependency-license conflicts, then establish a
maintainer-approved manual-first review process and evidence artifacts for the
complete resolved Rust workspace dependency graph. Use the initial review as
the repeatable template for a twice-yearly process; defer automated enforcement
until a future policy decision makes it useful.

## Background

Issue #269 was opened before repository issue specifications became the source
of truth. It identified `cargo license` as a way to list licenses but not to
evaluate whether they are compatible with the project's AGPL-3.0-only license.

The issue discussion also considered Snyk's commercial license-compliance
offering. Its required paid subscription makes it unsuitable as the default
repository enforcement mechanism without a separate funding decision.

The current `deny.toml` intentionally configures only `cargo deny check bans`
for workspace-layer enforcement. The `[licenses]` and `[advisories]` sections
explicitly state that they are not configured. Consequently, neither the
pre-commit hook nor CI currently performs dependency-license compliance checks.

The review will cover the entire resolved Cargo dependency graph, including
normal, build, development, target-specific, optional, and transitive
dependencies. The preliminary assessment in
[`preliminary-assessment.md`](preliminary-assessment.md) identifies a direct
GPL-2.0 dependency requiring urgent maintainer and qualified legal review. It
is technical triage, not a legal conclusion.

## Scope

### In Scope

- Define a simple approval process requiring agreement from all active project
  maintainers before the review protocol, its conclusions, or a later policy
  change is accepted.
- Perform and document the first full manual dependency-license review with AI
  assistance, using independently verifiable sources and tools rather than
  treating an agent's statement as evidence.
- Identify actual or potential conflicts among dependencies and between the
  project's license and dependency license obligations.
- Define and retain review artifacts analogous to security-analysis records:
  dated dependency-license inventories, decisions, identified conflicts,
  exceptions, and follow-up actions or linked issues.
- Define a twice-yearly review cadence and document the first review as the
  template for future reviews.
- Identify conditions that would justify a follow-up issue for automated checks
  when clear, maintainer-approved rules are available.

### Out of Scope

- Legal advice or a definitive legal opinion. Obtain qualified legal review when
  the policy decision requires it.
- Security advisory scanning, source-provenance checks, and general dependency
  updates; these are separate concerns from license compliance.
- Replacing the existing Cargo-deny layer-boundary bans configuration.
- Adding a mandatory license-compliance check to local hooks or CI. Such a check
  requires clear approved rules and is a possible follow-up, not a prerequisite
  for the initial review.
- Purchasing or making Snyk mandatory without a separate maintainer decision.
- Remediating a dependency solely because it is outdated when its license is
  policy-compliant.

## Open Decisions

The following decisions must be resolved through the initial review:

1. What simple mechanism will record unanimous active-maintainer approval for
   the review protocol, conclusions, and any later policy or exceptions?
2. Does the initial review establish an explicit license policy, or only record
   findings and decide whether a policy is needed for future enforcement?
3. When should qualified legal review be required before an approval decision?
4. What evidence sources and deterministic tools must AI-assisted review use to
   verify dependency license metadata, expressions, and identified conflicts?
5. What review artifact structure and storage location will preserve the
   inventory, analysis, decisions, actions, and next scheduled review date?
6. Which dependency or lockfile changes need an interim manual review before the
   next twice-yearly review?

Potential future automation is explicitly conditional. For example, `cargo deny
check licenses` can deterministically compare the resolved graph's declared or
detected SPDX expressions against a configured policy. It cannot decide the
policy itself, provide legal advice, or compare previous and updated
`Cargo.lock` files to report a license change as a distinct event.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None known. Create an ADR if the accepted policy introduces a
  long-lived dependency-governance model or a material new CI enforcement
  boundary.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                               | Notes / Expected Output                                                                                                                                                                  |
| --- | ----------- | ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE        | Define manual review protocol      | Recorded the review protocol, evidence standards, legal-review boundaries, unanimous-approval process, cadence, and interim-review triggers in [`initial-review.md`](initial-review.md). |
| T2  | DONE        | Run preliminary technical triage   | Record initial high-risk and metadata findings in [`preliminary-assessment.md`](preliminary-assessment.md); do not treat it as a legal conclusion.                                       |
| T3  | DONE        | Gather independently verified data | Produced a dated locked-graph inventory and source-manifest evidence in [`evidence.md`](evidence.md).                                                                                    |
| T4  | IN_PROGRESS | Analyze license conflicts          | Recorded material findings in [`initial-review.md`](initial-review.md); `bloom` remains blocked pending qualified legal review.                                                          |
| T5  | IN_PROGRESS | Record decisions and actions       | Produced the initial blocked review report and resolved the internal metadata gap; unresolved findings need dispositions.                                                                |
| T6  | BLOCKED     | Approve review outcome             | Requires qualified legal disposition for `bloom`, other finding dispositions, and explicit unanimous active-maintainer approval.                                                         |
| T7  | DONE        | Publish future-review template     | The initial report and evidence ledger define the twice-yearly template and schedule the next review for 2027-02-28.                                                                     |
| T8  | DONE        | Assess automation follow-up        | Deferred: no approved SPDX policy or exception process yet justifies enforcement.                                                                                                        |

## Progress Tracking

### Workflow Checkpoints

- [x] Existing GitHub issue reviewed and its discussion incorporated into this spec.
- [x] Specification converted to folder layout with issue-local evidence artifacts.
- [x] Specification reviewed and approved by user/maintainer.
- [x] Unanimous-maintainer approval process, review protocol, and scope decisions recorded.
- [x] Spec-only PR merged into `develop` before implementation.
- [ ] Implementation completed; legal and unanimous-maintainer approval remain blocked.
- [x] Documentation validation completed (`linter all`).
- [x] Initial technical manual-review scenario executed and recorded in `evidence.md`.
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-08-28 09:30 UTC - agent - Reviewed GitHub issue #269 and its three comments; drafted this source-of-truth specification from the existing issue and current repository tooling.
- 2026-08-28 09:30 UTC - agent - Confirmed that Cargo-deny currently enforces workspace bans only; no dependency-license check is configured in local hooks or CI.
- 2026-08-28 09:30 UTC - agent - Created local branch `269-review-dependency-licenses` for the planned spec-only PR. Awaiting user review and policy decisions.
- 2026-08-28 09:45 UTC - user - Confirmed that the review must cover the complete resolved dependency graph, that defining the initial policy and a simple unanimous-maintainer process are in scope, and that review artifacts should follow the established security-analysis model.
- 2026-08-28 10:00 UTC - user - Directed a manual-first approach: perform and preserve an AI-assisted full verification as the template for twice-yearly reviews. Automation is not required now and may be reconsidered only after clear rules exist.
- 2026-08-28 10:20 UTC - agent - Created the preliminary assessment artifact from current Cargo metadata and package manifests. It identifies the direct `bloom` GPL-2.0 dependency as requiring urgent qualified legal review and does not state a final compatibility conclusion.
- 2026-08-28 10:35 UTC - agent - Installed `cargo-license` 0.7.0 at the user's request and incorporated its production-oriented inventory into the preliminary assessment. The new inventory corroborates, but does not resolve, the GPL-2.0, LGPL-3.0, and non-routine-license findings.
- 2026-08-28 11:10 UTC - user - Approved the issue specification and preliminary assessment; authorized a spec-only PR targeting `develop`.
- 2026-08-28 - agent - Completed the first locked-graph technical inventory and evidence ledger at merged commit `c30fbff4`. The `workspace-coupling` metadata gap is resolved; `bloom` remains blocked pending qualified legal review and active-maintainer approval.

## Acceptance Criteria

- [ ] AC1: The documented manual review protocol, approved by all active
      maintainers through the defined process, covers the complete dependency graph,
      evidence requirements, legal-review escalation, and twice-yearly cadence.
- [ ] AC2: An initial dated review report inventories all resolved dependencies
      and records the sources and deterministic tools used to validate the data used
      by AI-assisted analysis.
- [ ] AC3: The initial review identifies actual and potential dependency-license
      conflicts, ambiguities, and exceptions; each has an approved rationale,
      follow-up action, or qualified legal-review escalation.
- [ ] AC4: The initial review report is retained as the documented template for
      recurring twice-yearly reviews and interim dependency-change reviews.
- [ ] AC5: The report records whether sufficiently clear approved rules exist to
      justify a separate automation issue, without adding mandatory automated
      license enforcement in this issue.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior or workflow changes.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Documentation Checks

- `cargo deny check bans` to preserve existing layer-boundary validation.
- `linter all`.

### Evidence Requirements for AI-Assisted Review

- Treat AI output as analysis, not authoritative license or compatibility fact.
- Record the exact commands, tool versions, input lockfile revision, and output
  used to build the dependency inventory.
- Verify package license metadata against primary package manifests, license
  files, and authoritative upstream sources where metadata is missing, custom,
  or ambiguous.
- Distinguish deterministic evidence about declared license expressions from a
  maintainer or qualified legal conclusion about compatibility and obligations.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                   | Command/Steps                                                                                                                                                  | Expected Result                                                                                    | Status      | Evidence                                                                                                            |
| --- | -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ----------- | ------------------------------------------------------------------------------------------------------------------- |
| M1  | Initial full review        | Run the documented inventory commands against the complete resolved graph; verify non-trivial metadata with the required sources; record analysis and actions. | A dated, reproducible review report is produced with evidence for every finding.                   | IN_PROGRESS | Technical evidence is complete; each finding needs a maintainer-approved disposition or qualified legal escalation. |
| M2  | Maintainer approval        | Present the initial report and review protocol to every active maintainer using the defined approval process.                                                  | Unanimous approval or recorded unresolved objections; unresolved matters are escalated or tracked. | TODO        | Pending defined approval process.                                                                                   |
| M3  | Recurring-review rehearsal | Use the initial report structure to plan the next review and an interim dependency-update review.                                                              | The report functions as a clear reusable template with a next-review date and triggers.            | DONE        | [`initial-review.md`](initial-review.md) schedules 2027-02-28 and records interim-review triggers.                  |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                          |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | TODO                   | Protocol is documented; unanimous active-maintainer approval is pending.                                                                          |
| AC2   | DONE                   | [`initial-review.md`](initial-review.md), [`evidence.md`](evidence.md), and the retained inventory artifacts record the locked graph and sources. |
| AC3   | TODO                   | Material findings are recorded; `bloom` requires qualified legal review and other dispositions remain pending.                                    |
| AC4   | DONE                   | The report and ledger are the recurring template; next review is scheduled for 2027-02-28.                                                        |
| AC5   | DONE                   | The report defers automation because approved deterministic rules do not yet exist.                                                               |

## Risks and Trade-offs

- License compatibility can depend on how a dependency is linked, distributed,
  and used; automated SPDX matching is a deterministic policy guardrail, not
  legal advice.
- Checking all lockfile packages offers earlier detection but may require policy
  decisions for platform, build, and test-only transitive dependencies.
- License text or metadata can be incomplete or non-standard. Such cases need
  an explicit review path rather than an unexamined global bypass.
- Unanimous approval improves legitimacy for policy decisions but can delay
  dependency updates. The process should state how to identify active
  maintainers, request approval, and record a non-response without weakening
  the unanimity requirement.

## References

- GitHub issue: #269
- Preliminary evidence: [`preliminary-assessment.md`](preliminary-assessment.md)
- Issue comment: [Snyk license-compliance suggestion](https://github.com/torrust/torrust-tracker/issues/269#issuecomment-1749443211)
- Existing Cargo-deny bans spec: `docs/issues/closed/1925-1669-si-31-configure-cargo-deny-for-layer-boundary-enforcement.md`
- Current configuration: `deny.toml`
- Cargo-deny license-check documentation: <https://embarkstudios.github.io/cargo-deny/checks/licenses/index.html>
- Cargo-deny license configuration: <https://embarkstudios.github.io/cargo-deny/checks/licenses/cfg.html>
