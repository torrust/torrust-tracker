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
last-updated-utc: 2026-09-10 11:30
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - AGENTS.md
    - .github/agents/implementer.agent.md
    - .github/skills/dev/rust-code-quality/
    - contrib/dev-tools/
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
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
- Use native `reason = "..."` enforcement prospectively; defer workspace-wide
  `clippy::allow_attributes_without_reason` activation until #2158 remediates historical allows.
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
- Related ADR: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
- ADRs to create: None unless the selected baseline or enforcement mechanism changes repository-wide automation architecture.

## Implementation Plan

| ID  | Status | Task                                 | Notes / Expected Output                                                                             |
| --- | ------ | ------------------------------------ | --------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Define native rationale policy       | Uses `reason = "..."`; covers intentional, false-positive, and temporary cases.                     |
| T2  | DONE   | Select prospective baseline strategy | Merge-base diff excludes legacy allows without accepting changed attributes lacking native reasons. |
| T3  | DONE   | Implement and test Rust validation   | Pure `syn` module and narrow Git adapter remain independently testable and extractable.             |
| T4  | DONE   | Integrate and document               | Uses current validation tiers without choosing the final EPIC harness architecture.                 |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-require-documented-clippy-allows/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2157 created and issue number added to this spec
- [x] Revised specification approved and committed
- [x] Bash implementation removed in a separate commit
- [x] Replacement implementation completed and verified
- [x] Acceptance criteria reviewed after replacement implementation and updated with evidence

### PR #2177 Remediation

- [ ] Preserve, commit, or discard pre-existing uncommitted changes before review fixes.
- [x] Repair the current failed workflow and record verification against the branch tip.
- [ ] Process Cameron's maintainer feedback using
      `docs/pr-review-feedback/pr-2177-review-feedback.md` and the review-feedback workflow.
- [ ] Process Copilot suggestions using
      `docs/copilot-pr-reviews/pr-2177-copilot-suggestions.md` and the Copilot-suggestions workflow.
- [ ] Run final repository validation and independent reviews after all findings are processed.
- [ ] Rebase onto the latest `torrust/develop`, rerun required checks, and push with
      `--force-with-lease`.
- [ ] Confirm required CI checks pass and request refreshed maintainer review.

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as an immediately implementable EPIC #2003 child; existing-allow remediation is explicitly separate - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2157, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2157
- 2026-09-08 17:00 UTC - GitHub Copilot - Implemented the prospective merge-base validator, rationale policy, focused Git-fixture tests, pre-commit and CI integration - Pending final verification
- 2026-09-08 17:25 UTC - GitHub Copilot - Independent complexity and task reviews passed; `linter all`, focused tests, documentation tests, and all pre-commit steps passed - Ready to commit
- 2026-09-09 12:00 UTC - josecelano - Replaced the Bash approach with Rust and native lint reasons as the approved direction; the Bash implementation is superseded and will be deleted before replacement work begins - Chat decision
- 2026-09-09 12:20 UTC - GitHub Copilot - Removed the superseded Bash validator, tests, integrations, guidance, and obsolete evidence in dedicated reset commit `d1900cde` before starting the Rust replacement - Committed as `refactor(quality): remove superseded Bash Clippy guard`
- 2026-09-09 13:10 UTC - GitHub Copilot - Implemented native-reason prospective validation as a small Rust crate with a pure `syn` module, Git-diff adapter, unit tests, and end-to-end Git fixture - Pending full validation
- 2026-09-09 13:40 UTC - GitHub Copilot - Full lint and documentation tests plus independent complexity and acceptance reviews passed; restored prior append-only review evidence after the reset - Ready to commit
- 2026-09-09 14:05 UTC - GitHub Copilot - Corrected `clippy-allow-reasons` to the global CLI output contract: `no-stdout-result`, silent success, and NDJSON stderr failure diagnostics; added CLI contract tests - `linter all`, focused tests, strict Clippy, and documentation tests passed
- 2026-09-09 14:15 UTC - GitHub Copilot - Repaired PR #2177 container recipe context for the new workspace crate after `cargo chef prepare` failed in CI; focused uncached recipe build and `linter all` passed - Pending focused commit
- 2026-09-09 15:10 UTC - GitHub Copilot - Correcting Cameron's PR #2177 review findings: exclude the dev-only checker from nextest archives, parse CLI arguments before Git access, and append validation-evidence corrections - Pending validation
- 2026-09-09 15:20 UTC - GitHub Copilot - Completed Cameron's requested corrections: all four nextest archive commands exclude `clippy-allow-reasons`; invalid CLI arguments are parsed before Git access; corrected earlier lint-evidence claims - Focused tests, `linter all`, and `docker build --no-cache --target test_debug --file Containerfile .` passed
- 2026-09-10 09:00 UTC - GitHub Copilot - Started a structured PR #2177 remediation pass; the issue spec tracks merge readiness while separate Cameron and Copilot audit records track individual findings, commits, replies, and thread states - In progress
- 2026-09-10 09:20 UTC - GitHub Copilot - Fixed Cameron finding M25 / BB1: replaced five `assert_is_empty` assertions in the Rust validator library tests; nightly Clippy and `linter all` pass on the branch tip - Pending focused commit, reply, and thread resolution
- 2026-09-10 09:30 UTC - GitHub Copilot - Completed Cameron finding M25 / BB1 in `65a8d1b4`; nightly Clippy and `linter all` passed, reply posted, and review thread resolved - Done
- 2026-09-10 10:05 UTC - GitHub Copilot - Addressed Cameron findings M15 / BB2 and M19: pinned and fail-closed Git diff parsing; disposable Git fixtures disable signing and hooks; hostile configuration tests, nightly Clippy, and `linter all` passed - Pending focused commits, replies, and thread resolutions
- 2026-09-10 10:15 UTC - GitHub Copilot - Completed Cameron findings M15 / BB2 and M19 in `067f379b`; validation passed, replies posted, and both review threads resolved - Done
- 2026-09-10 10:30 UTC - GitHub Copilot - Addressed Cameron findings M16 / BB3 and M23: native-reason validation now covers changed direct and conditional Clippy `allow` and `expect` controls; the skill documents the macro token-body limitation - Pending focused commits, replies, and thread resolutions
- 2026-09-10 10:40 UTC - GitHub Copilot - Completed Cameron findings M16 / BB3 and M23 in `415f16f6` and `b2db0d1c`; validation passed, replies posted, and both review threads resolved - Done
- 2026-09-10 11:00 UTC - GitHub Copilot - Addressed Cameron finding M17: temporary-reason detection recognizes common intent wording and normalizes removal conditions without weakening required issue/removal evidence - Pending focused commits, reply, and thread resolution
- 2026-09-10 11:10 UTC - GitHub Copilot - Completed Cameron finding M17 in `bb1c7e45` and `856d3d33`; validation passed, reply posted, and the review thread resolved - Done
- 2026-09-10 11:30 UTC - GitHub Copilot - Addressed Cameron finding M20: diagnostic output failures now trigger a fixed NDJSON fallback and are covered by a failing-writer unit test - Pending focused commit, reply, and thread resolution
- 2026-09-10 11:20 UTC - GitHub Copilot - Addressed Cameron finding M18: CLI integration tests now exercise a real accepted changed allow plus method, statement, and multiline crate-level forms - Pending focused commits, reply, and thread resolution

## Acceptance Criteria

- [x] Temporary native reasons identify a removal condition or stable follow-up issue.
- [x] A reviewed prospective baseline excludes existing attributes without accepting changed attributes that lack native reasons.
- [x] Focused Rust tests prove undocumented new attributes fail and documented item and crate forms pass.
- [x] Enforcement runs in a documented existing validation tier and produces actionable diagnostics.
- [x] `linter all` exits with code `0` and relevant tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- Focused Rust validator tests
- `cargo test --doc --workspace`

### Manual Verification Scenarios

| ID  | Scenario              | Command/Steps                                                                | Expected Result                                               | Status | Evidence                |
| --- | --------------------- | ---------------------------------------------------------------------------- | ------------------------------------------------------------- | ------ | ----------------------- |
| M1  | Undocumented addition | Add an isolated undocumented fixture allow and run the validator.            | The validator fails and identifies the missing native reason. | DONE   | End-to-end Git fixture  |
| M2  | Documented exceptions | Run fixtures for intentional, false-positive, and temporary rationale types. | Each passes only with complete required native information.   | DONE   | Focused Rust unit tests |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                  |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | Rust code-quality guidance requires native reasons for changed Clippy allows.                                                             |
| AC2   | DONE                   | Unit tests cover native reasons with issue references and non-empty removal conditions.                                                   |
| AC3   | DONE                   | The Rust command validates only changed attribute spans from the Git merge-base diff.                                                     |
| AC4   | DONE                   | Unit and end-to-end Git-fixture tests cover missing, empty, item, crate, and temporary native reasons.                                    |
| AC5   | DONE                   | Pre-commit and CI run the Rust command; CI fetches history for merge-base computation and it follows the `no-stdout-result` CLI contract. |
| AC6   | DONE                   | `linter all`, focused Rust tests including CLI output contracts, strict Clippy, and `cargo test --doc --workspace` passed.                |

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

## Validation Evidence Corrections

- 2026-09-09 - The 13:40 and 14:05 entries predate the current branch's final lint state and
  incorrectly state that `linter all` passed. The current failure is
  `clippy::assert_is_empty` in the new CLI tests. The assertion style was corrected in response
  to PR #2177 review; `linter all` now passes. Do not treat those earlier entries as final
  acceptance evidence.

## References

- Parent EPIC: #2003
- GitHub issue: #2157
- Follow-up remediation: #2158 - Inventory Existing Clippy Allows
  (`docs/issues/open/2158-2003-inventory-existing-clippy-allows/ISSUE.md`)
