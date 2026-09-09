---
doc-type: issue
issue-type: enhancement
status: in-progress
priority: p2
epic: 2003
github-issue: 2157
spec-path: docs/issues/open/2157-2003-require-documented-clippy-allows/ISSUE.md
branch: "2157-2003-require-documented-clippy-allows"
related-pr: null
last-updated-utc: 2026-09-09 12:00
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

Require every newly introduced or modified Clippy `allow` attribute to use Rust's native, specific
`reason = "..."` parameter, with prospective enforcement that does not block on pre-existing
undocumented allows.

## Background

A Clippy suppression may reflect an intentional design choice, a false positive, or a temporary
deferral. Without a rationale, future maintainers cannot determine why it exists or whether it
should be removed. Rust supports a native `reason = "..."` parameter on lint-level attributes,
and Clippy's `allow_attributes_without_reason` lint detects absent reasons. The workspace MSRV is
1.88, so the native form is available for all maintained code. The repository already contains
approximately 215 Clippy allows, so this policy needs a prospective baseline rather than silently
expanding into a bulk remediation.

## Scope

### In Scope

- Define the native rationale form for item- and crate-level `allow(clippy::...)` attributes:
  `reason = "<specific rationale>"`.
- Enable `clippy::allow_attributes_without_reason` to make absent native reasons visible.
- Require temporary suppressions to include a stable issue reference or explicit removal condition
  inside their native reason string.
- Add a focused Rust validator that detects newly added or modified attributes lacking native
  reasons or the repository-specific temporary-removal information.
- Use a reviewed change-detection mechanism that cannot silently grandfather a newly added or
  modified undocumented attribute while #2158 remediates the historical inventory.
- Keep the validator's Rust module and command narrowly scoped, independently testable, and
  suitable for extraction into the later approved harness; do not define the EPIC's final harness
  architecture in this issue.
- Update relevant Rust code-quality guidance and agent instructions.

### Out of Scope

- Documenting or removing existing Clippy allows; that is the separate existing-allow inventory issue.
- Remediating historical allows or their underlying lint findings; that belongs to #2158.
- Replacing the repository's existing linter runner or CI architecture.
- Selecting the unified guardrail/sensor harness architecture planned by EPIC #2003.

## Architectural Decisions

- Related ADR: `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: None unless the selected baseline or enforcement mechanism changes repository-wide automation architecture.

## Implementation Plan

| ID  | Status | Task                                 | Notes / Expected Output                                                            |
| --- | ------ | ------------------------------------ | ---------------------------------------------------------------------------------- |
| T1  | TODO   | Define native rationale policy       | Use `reason = "..."`; cover intentional, false-positive, and temporary cases.      |
| T2  | TODO   | Select prospective baseline strategy | Exclude legacy allows without accepting a changed attribute with no native reason. |
| T3  | TODO   | Implement and test Rust validation   | Keep modules independently testable and suitable for later harness extraction.     |
| T4  | TODO   | Integrate and document               | Use current validation tiers without choosing the final EPIC harness architecture. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-require-documented-clippy-allows/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2157 created and issue number added to this spec
- [ ] Revised specification approved and committed
- [ ] Bash implementation removed in a separate commit
- [ ] Replacement implementation completed and verified
- [ ] Acceptance criteria reviewed after replacement implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as an immediately implementable EPIC #2003 child; existing-allow remediation is explicitly separate - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2157, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2157
- 2026-09-08 17:00 UTC - GitHub Copilot - Implemented the prospective merge-base validator, rationale policy, focused Git-fixture tests, pre-commit and CI integration - Pending final verification
- 2026-09-08 17:25 UTC - GitHub Copilot - Independent complexity and task reviews passed; `linter all`, focused tests, documentation tests, and all pre-commit steps passed - Ready to commit
- 2026-09-09 12:00 UTC - josecelano - Replaced the Bash approach with Rust and native lint reasons as the approved direction; the Bash implementation is superseded and will be deleted before replacement work begins - Chat decision

## Acceptance Criteria

- [ ] Guidance requires native `reason = "..."` for all supported Clippy allow attribute forms.
- [ ] Temporary native reasons identify a removal condition or stable follow-up issue.
- [ ] A reviewed prospective baseline excludes existing attributes without accepting changed attributes that lack native reasons.
- [ ] Focused Rust tests prove undocumented new attributes fail and documented item and crate forms pass.
- [ ] Enforcement runs in a documented existing validation tier and produces actionable diagnostics.
- [ ] `linter all` exits with code `0` and relevant tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- Focused Rust validator tests
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario              | Command/Steps                                                                | Expected Result                                               | Status | Evidence            |
| --- | --------------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------- | ------ | ------------------- |
| M1  | Undocumented addition | Add an isolated undocumented fixture allow and run the validator.            | The validator fails and identifies the missing native reason. | TODO   | Pending replacement |
| M2  | Documented exceptions | Run fixtures for intentional, false-positive, and temporary rationale types. | Each passes only with complete required native information.   | TODO   | Pending replacement |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                            |
| ----- | ---------------------- | ----------------------------------- |
| AC1   | TODO                   | Pending replacement implementation. |
| AC2   | TODO                   | Pending replacement implementation. |
| AC3   | TODO                   | Pending replacement implementation. |
| AC4   | TODO                   | Pending replacement implementation. |
| AC5   | TODO                   | Pending replacement implementation. |
| AC6   | TODO                   | Pending replacement implementation. |

## Risks and Trade-offs

- Native lint reasons prevent custom syntax for the rationale itself. Any temporary-policy check
  must inspect only the native reason string and reject unrecognized forms visibly.
- A stale baseline can become a loophole. Version it, test it, and make its update reviewable.
- A standalone Rust validator could prematurely become a harness. Keep its module and command
  boundary narrow, and defer its final location and interface to the EPIC decision.

## Implementation Completion Review

After replacement implementation, record material findings about the baseline or validator in an issue-local
`implementation-retrospective.md`. If none occurred, add a concise progress-log entry explaining
why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2157
- Follow-up remediation: #2158 - Inventory Existing Clippy Allows
  (`docs/issues/open/2158-2003-inventory-existing-clippy-allows/ISSUE.md`)
