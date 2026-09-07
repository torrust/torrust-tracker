---
doc-type: issue
issue-type: enhancement
status: planned
priority: p2
epic: 2003
github-issue: 2157
spec-path: docs/issues/open/2157-2003-require-documented-clippy-allows/ISSUE.md
branch: "2157-2003-require-documented-clippy-allows"
related-pr: null
last-updated-utc: 2026-09-07 11:20
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - AGENTS.md
    - .github/agents/implementer.agent.md
    - .github/skills/dev/rust-code-quality/
    - contrib/dev-tools/
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2157 - Require Documented Clippy Allows

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Require every newly introduced or modified Clippy `allow` attribute to have a nearby, specific
rationale, and add focused enforcement that does not block on pre-existing undocumented allows.

## Background

A Clippy suppression may reflect an intentional design choice, a false positive, or a temporary
deferral. Without a nearby explanation, future maintainers cannot determine why it exists or
whether it should be removed. The repository already contains approximately 215 Clippy allows, so
this policy needs a prospective baseline rather than silently expanding into a bulk remediation.

## Scope

### In Scope

- Define the accepted rationale format for item- and crate-level `allow(clippy::...)` attributes.
- Require temporary suppressions to include a stable issue reference or explicit removal condition.
- Add a focused, testable validator that detects undocumented additions or modifications.
- Establish and commit a baseline representing pre-existing attributes, or use another reviewed
  change-detection mechanism that cannot silently grandfather new undocumented attributes.
- Update relevant Rust code-quality guidance and agent instructions.

### Out of Scope

- Documenting or removing existing Clippy allows; that is the separate existing-allow inventory issue.
- Changing Clippy lint levels or remediating the underlying lint findings.
- Replacing the repository's existing linter runner or CI architecture.

## Architectural Decisions

- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: None unless the selected baseline or enforcement mechanism changes repository-wide automation architecture.

## Implementation Plan

| ID  | Status | Task                                  | Notes / Expected Output                                                          |
| --- | ------ | ------------------------------------- | -------------------------------------------------------------------------------- |
| T1  | TODO   | Define rationale policy               | Cover intentional, false-positive, and temporary cases.                          |
| T2  | TODO   | Select prospective baseline strategy  | Record how existing allows are excluded without admitting new undocumented ones. |
| T3  | TODO   | Implement and test focused validation | Support item and crate attributes with actionable diagnostics.                   |
| T4  | TODO   | Integrate and document                | Select a current validation tier without redesigning the runner.                 |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-require-documented-clippy-allows/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2157 created and issue number added to this spec
- [ ] Implementation completed and verified
- [ ] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as an immediately implementable EPIC #2003 child; existing-allow remediation is explicitly separate - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2157, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2157

## Acceptance Criteria

- [ ] Guidance defines nearby rationale requirements for all supported Clippy allow attribute forms.
- [ ] Temporary allows identify a removal condition or stable follow-up issue.
- [ ] A committed prospective baseline or reviewed equivalent excludes existing attributes without accepting new undocumented attributes.
- [ ] Focused tests prove undocumented new attributes fail and documented ones pass for item and crate forms.
- [ ] The enforcement runs in a documented existing validation tier and produces actionable diagnostics.
- [ ] `linter all` exits with code `0` and relevant tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- Focused validator tests
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario              | Command/Steps                                                                | Expected Result                                                          | Status | Evidence               |
| --- | --------------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------------------ | ------ | ---------------------- |
| M1  | Undocumented addition | Add an isolated undocumented fixture allow and run the validator.            | The validator fails and identifies the attribute and required rationale. | TODO   | Pending implementation |
| M2  | Documented exceptions | Run fixtures for intentional, false-positive, and temporary rationale types. | Each passes only with complete required information.                     | TODO   | Pending implementation |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence               |
| ----- | ---------------------- | ---------------------- |
| AC1   | TODO                   | Pending implementation |
| AC2   | TODO                   | Pending implementation |
| AC3   | TODO                   | Pending implementation |
| AC4   | TODO                   | Pending implementation |
| AC5   | TODO                   | Pending implementation |
| AC6   | TODO                   | Pending implementation |

## Risks and Trade-offs

- Text parsing can be brittle. Support a small explicit syntax and reject unrecognized forms visibly.
- A stale baseline can become a loophole. Version it, test it, and make its update reviewable.

## Implementation Completion Review

After implementation, record material findings about the baseline or validator in an issue-local
`implementation-retrospective.md`. If none occurred, add a concise progress-log entry explaining
why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2157
- Follow-up remediation: #2158 - Inventory Existing Clippy Allows
  (`docs/issues/open/2158-2003-inventory-existing-clippy-allows/ISSUE.md`)
