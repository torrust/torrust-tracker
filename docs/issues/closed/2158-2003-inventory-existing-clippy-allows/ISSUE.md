---
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2003
github-issue: 2158
spec-path: docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
branch: "2158-2003-inventory-existing-clippy-allows"
related-pr: 2259
last-updated-utc: 2026-09-18 14:40
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - AGENTS.md
    - Cargo.toml
    - .github/agents/clippy-fixer.agent.md
    - .github/agents/committer.agent.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - src/
    - packages/
    - console/
    - tests/
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

# Issue #2158 - Inventory Existing Clippy Allows

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Inventory and explain every existing Clippy `allow` attribute in this specification's
`clippy-allow-inventory.md` artifact, then remove unjustified suppressions or create stable
follow-up work for justified temporary ones.

## Background

The workspace currently has approximately 215 `allow(clippy::...)` attributes, including 25
crate-level attributes. Prospective enforcement alone cannot explain this historical baseline.
Each existing suppression needs an explicit decision: retain with rationale, remove after a fix, or
retain temporarily with a tracked follow-up.

## Scope

### In Scope

- Inventory every item- and crate-level Clippy allow in maintained Rust source.
- Store the human-readable inventory in this issue's folder as
  `clippy-allow-inventory.md`. It is the audit record and moves with the issue specification; it
  is not the stable machine-readable baseline for prospective enforcement.
- Record lint name, source location, scope, rationale category, evidence, owner, and disposition.
- Add nearby rationale comments for retained allows.
- Remove allows that are no longer necessary and fix resulting diagnostics.
- Update `ClippyFixer` to declare the `edit` tool so it can apply focused remediation and nearby
  rationale comments; continue to delegate commits to `Committer`.
- For each approved temporary suppression follow-up, create an issue-spec draft in
  `docs/issues/drafts/` that states the removal condition and links the inventory entry. Request
  user approval before creating the follow-up GitHub issue, then replace the draft reference with
  the assigned issue number.

> **Current investigation exception:** numeric-conversion draft material is retained under this
> issue's `numeric-conversion-follow-up-drafts/` folder at maintainer request while the complete
> inventory is classified. It is not yet a standalone issue specification and must be re-evaluated
> before promotion to `docs/issues/drafts/` or GitHub issue creation.

### Out of Scope

- Introducing the prospective enforcement mechanism for new allows; that belongs to the sibling policy issue.
- Using the issue-local Markdown inventory as the machine-readable prospective-enforcement
  baseline; the sibling policy issue owns that stable validation input.
- Broad refactoring unrelated to eliminating an unjustified suppression.
- Blanket suppression of Clippy warnings to finish the inventory.

## Architectural Decisions

- Related ADRs: None known.
- ADRs to create: None. Create an ADR only if the inventory reveals a new repository-wide lint policy decision.

## Implementation Plan

| ID  | Status | Task                              | Notes / Expected Output                                                                                                    |
| --- | ------ | --------------------------------- | -------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Create the issue-local inventory  | Add `clippy-allow-inventory.md` with source roots, generation method, and a row for every item- and crate-level attribute. |
| T2  | TODO   | Enable ClippyFixer remediation    | Grant its profile the `edit` tool while retaining Committer ownership of signed commits.                                   |
| T3  | TODO   | Classify each allow               | Retain, remove, or temporary follow-up with evidence.                                                                      |
| T4  | TODO   | Approve and create follow-up issues | Re-evaluate issue-local design inputs; promote only approved temporary work through the repository spec-first workflow. |
| T5  | TODO   | Remediate #2158 decisions in reviewable batches | Use ClippyFixer for focused `Remove` and `Retain` changes; group commits and PRs by package or rationale category. |
| T6  | TODO   | Validate final inventory          | Regenerate source entries, reconcile the inventory by ID, and ensure no allow lacks a disposition and relevant checks pass. |

### Classification and Remediation Sequencing

Perform a classification-first pass, grouping entries by rationale category while recording a
source-specific decision for every row. Then remediate one category and package-scoped batch at a
time. This avoids inconsistent decisions for repeated lint families and keeps source changes,
rationale comments, tests, and commits reviewable. The issue-local inventory is the authoritative
record of category order and entry-level classification evidence.

### Execution Sequence After Classification

Merge the completed classification inventory as a documentation-only checkpoint before remediation.
Then implement the inventory's `Remove` and `Retain` decisions as #2158 work in focused,
package-scoped branches and PRs from `develop`; do not create a follow-up issue merely to remove a
stale suppression or add a source-specific rationale. Each batch must update the inventory by ID
with its implementation and validation evidence. Do not maintain transient source line numbers
during every batch; regenerate and reconcile the complete source inventory once in T6.

Evaluate temporary entries only after the maintainer re-evaluates the issue-local design inputs.
Promote approved cross-package numeric conversion work to a dedicated EPIC with focused child
issues. The wire numeric-conversion follow-up exclusively owns A156 and A171. A distinct UDP
protocol baseline follow-up owns the remaining twelve crate-level allows, if that work is approved.
Do not create an EPIC, child issue, or GitHub issue from a design input before approval.

Use `#[expect(...)]` for remediated stable-toolchain lint findings where continued emission is
intended to be checked. Use `#[allow(..., reason = "...")]` for a toolchain-dependent,
macro-expansion finding when `expect` would fail on a supported toolchain that does not emit the
lint. Every retained attribute requires a native, source-specific `reason`; a temporary reason
must also state a stable removal condition or approved issue reference, consistent with #2157's
prospective validator.

### Handoff, Branch, and PR Plan

The tracked #2158 specification, its inventory, and approved issue specifications are the durable
source of truth. Use `.tmp/2158-remediation-plan.md` only as an ignored working board while
drafting, reviewing, and sequencing artifacts. It must link to durable artifacts and must not be
the only record of an approval, issue number, branch, PR, validation result, or disposition change.
Copy each completed decision to the relevant tracked specification and inventory row before moving
the working board to its next state.

| Order | Branch and PR | Scope | Completion gate |
| ----- | ------------- | ----- | --------------- |
| 1 | Current `2158-2003-inventory-existing-clippy-allows` documentation PR | Complete inventory and handoff plan only | Review and merge before all remediation work |
| 2 | `feat/numeric-conversion-remediation-plan` specification-only PR | One numeric-conversion EPIC plus the metric-aggregate, wire-validation, and domain-contract child specifications | Draft all four artifacts, obtain maintainer approval as one bundle, create the EPIC first and then its child issues, move specifications to `open/`, and merge the PR using `Related to #2158` |
| 3 | `fix/udp-protocol-clippy-baseline` specification-only PR | One approved follow-up for the twelve nonnumeric UDP protocol crate-level allows | Complete only after the numeric bundle is separated and maintainer approval is recorded |
| 4 | Package-scoped #2158 remediation branches and PRs | Direct `Remove` changes and retained rationale annotations | One package or coherent rationale batch per PR, linked to #2158; update evidence by inventory ID |
| 5 | Temporary follow-up implementation branches and PRs | Approved numeric child issues or UDP baseline issue | Branch from current `develop`, link only the issue the PR fully implements, and record removed/narrowed allows in the inventory |
| 6 | Final #2158 validation PR or closing update | Regenerated inventory and acceptance evidence | Reconcile every source allowance after all relevant remediation merges |

The numeric EPIC and all three child issues belong in one specification branch and one
documentation-only PR because they define one reviewed cross-package program. Create their GitHub
issues only after the maintainer has approved the complete bundle. Create the EPIC first so the
child issues can reference its assigned number. Do not combine that specification PR with code
remediation, the UDP baseline specification, or temporary source artifacts.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-inventory-existing-clippy-allows/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue #2158 created and issue number added to this spec
- [x] Issue-local `clippy-allow-inventory.md` created and completeness checked
- [x] ClippyFixer has the `edit` tool and its documented commit delegation remains intact
- [x] Draft specifications created and approved before every temporary-allow follow-up GitHub issue
- [x] Implementation completed and verified
- [x] Acceptance criteria reviewed after implementation and updated with evidence

### Progress Log

- 2026-09-07 10:45 UTC - GitHub Copilot - Created as separate remediation work for the existing Clippy-allow baseline - This spec
- 2026-09-07 11:05 UTC - josecelano - Approved this subissue specification - Chat approval
- 2026-09-07 11:10 UTC - GitHub Copilot - Created GitHub issue #2158, linked it to EPIC #2003, and promoted this specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2158
- 2026-09-15 - GitHub Copilot - Started the source-root inventory with the generated first batch (A001-A025); baseline scan found 234 attributes and 244 lint allowances - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Completed and cross-checked the source-location baseline (A001-A234); classification remains pending - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Recorded a category-first classification and package-scoped remediation sequence; individual source decisions remain pending - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Began compatibility classification: retained 13 `derive_more` macro-expansion allowances with existing removal conditions and identified three `Into` implementations for focused removal - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Continued compatibility classification with source-specific decisions for six server lifecycle/API names and three standard conversion-trait allowances - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Classified macro-generated trait allowances and the remaining UDP server, peer-key error, and JSON serialization naming allowances; compatibility entries with the recorded lint families are now classified - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Classified all ten intentional command-line output and process-exit attributes as retained; each exposes a binary, example, benchmark, or shared console output contract - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Classified all 12 async-signature and lifecycle allowances as retained: two-phase server startup, Axum trait/handler boundaries, and uniform asynchronous benchmark calls provide source-specific evidence - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Added issue-local numeric-conversion follow-up drafts for metric aggregate safety, wire-width validation, and domain conversion contracts; defer GitHub issue or EPIC creation until the completed inventory is re-evaluated - `numeric-conversion-follow-up-drafts/README.md`
- 2026-09-15 - GitHub Copilot - Began trait/API ergonomics classification: retained ten `async_trait` macro-expansion `double_must_use` allowances with existing source evidence - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Classified Figment callback error types and ownership, standard-trait, driver-symmetry, and Tokio lock API allowances; identified three stale configuration `unnecessary_wraps` suppressions for focused removal - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Classified non-UDP-protocol documentation and panic-contract allowances: retained one deprecated public API field name and identified two missing panic-documentation suppressions for removal - `clippy-allow-inventory.md`
- 2026-09-15 - GitHub Copilot - Added an issue-local UDP protocol crate-baseline design input for the thirteen broad inherited allowances; defer GitHub issue or EPIC creation until the complete inventory is re-evaluated - `udp-protocol-clippy-baseline-draft.md`
- 2026-09-15 - GitHub Copilot - Completed the classification pass: retained explicit configuration, visibility, and error/composition-boundary contracts; identified six direct stale or mechanical removals - `clippy-allow-inventory.md`
- 2026-09-15 - josecelano - Approved the post-classification execution sequence: #2158 owns direct removals and retained-rationale remediation; only approved temporary design work becomes a follow-up issue or numeric-conversion EPIC - Chat decision
- 2026-09-15 - josecelano - Approved a durable handoff plan: use a local ignored coordination board while the tracked #2158 artifacts remain authoritative; create the approved numeric EPIC and child specifications in one documentation-only branch and PR - Chat decision
- 2026-09-15 - GitHub Copilot - Granted ClippyFixer the `edit` tool while retaining its required delegation of signed commits to Committer - `.github/agents/clippy-fixer.agent.md`
- 2026-09-18 - GitHub Copilot - Started direct `Remove` remediation with the configuration stale-suppression batch - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Removed HTTP announce parser and encoder suppressions by applying compact-peer chunk parsing and standard `From` conversions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Removed the fixture `PeerBuilder` manual default suppression by deriving `Default` - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Replaced the swarm registry test DTO `Into` implementation with `From` - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Documented the benchmark info-hash generator panic precondition and removed its suppression - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Removed UDP core seed-reference suppressions by making the test seed a static reference target - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Documented the tracker-client UDP checker sample-hash panic precondition and removed its suppression - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Completed direct `Remove` remediation; the anchored allow scan now finds 219 remaining attributes, matching the original 234 rows minus 15 removals - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained CLI and executable-output suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained `derive_more::Constructor` macro-expansion suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained async lifecycle, Axum extractor, and Axum handler suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained qBittorrent E2E staged-visibility suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained configuration ownership and serialized feature-switch suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained configuration Figment test-callback suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained module-name repetition suppressions that preserve public API and benchmark abstraction names - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained `async_trait` double-must-use suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained compatibility and API-shape suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained API-shape, standard trait, lock, benchmark, and UDP error-boundary suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native `reason` parameters for retained benchmark/test-data numeric suppressions - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native temporary `reason` parameters linking domain numeric suppressions to follow-up issue #2246 - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native temporary `reason` parameters linking wire numeric suppressions to follow-up issue #2245 - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Added native temporary `reason` parameters linking metric aggregate suppressions to follow-up issue #2244 - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Created follow-up issue #2261 for the nonnumeric UDP protocol baseline and added native temporary `reason` parameters for A157-A158 and A160-A168 - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Reconciled current source entry A235 for the benchmarking crate-level style baseline; removal probe exposed active diagnostics - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Removed A235 by applying behavior-preserving benchmarking repository style and lock-scope fixes - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Reconciled current source entry A236 for a retained swarm statistics collaboration-test gauge conversion - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Completed pre-review #2158 reconciliation: 236 inventory rows, 16 removed entries, 220 active inventory entries, 220 source attributes, and zero Clippy allows without native reasons; `linter all` passed - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Addressed PR review findings and reconciled A237-A239 for benchmark variants that intentionally preserve their outer read-lock span - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Addressed PR review finding and reconciled A240 for the torrent cleanup job `#[expect]` suppression - `clippy-allow-inventory.md`
- 2026-09-18 - GitHub Copilot - Archived the completed issue spec from `docs/issues/open/` to `docs/issues/closed/` after PR #2259 was merged and GitHub issue #2158 was closed - This specification

## Acceptance Criteria

- [x] A durable inventory accounts for every Clippy allow in the agreed source roots.
- [x] The inventory is stored as `clippy-allow-inventory.md` in this specification folder and
      identifies its source roots and generation/completeness method.
- [x] ClippyFixer declares the `edit` tool and can apply the focused remediation required by an
      inventory disposition while Committer retains commit responsibility.
- [x] Every retained allow has a nearby specific rationale and a matching inventory decision.
- [x] Every temporary allow has a stable removal condition and a linked GitHub follow-up issue
      created only after its folder-style draft specification was approved.
- [x] Every removed allow is accompanied by a focused fix and validation.
- [x] The inventory contains no unclassified entries and no blanket exceptions.
- [x] `linter all` exits with code `0` and relevant package tests pass.

## Verification Plan

### Automatic Checks

- `linter all`
- Inventory completeness check
- Relevant package tests for remediation batches
- Focused profile check for the ClippyFixer `edit` tool and Committer delegation

### Manual Verification Scenarios

| ID  | Scenario                           | Command/Steps                                                                                                           | Expected Result                                                                                            | Status | Evidence               |
| --- | ---------------------------------- | ----------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- | ------ | ---------------------- |
| M1  | Compare source and inventory       | Regenerate the source list and compare it with `clippy-allow-inventory.md`.                                             | Every allow or expect has one recorded decision.                                                           | DONE   | Review-adjusted reconciliation reports 240 inventory rows, 16 removed entries, 224 active entries, and 224 source attributes. |
| M2  | Sample retained and removed cases  | Review representative cases from each disposition category.                                                             | Rationale, source, and validation evidence agree.                                                          | DONE   | Retained, temporary, and removed evidence tables in `clippy-allow-inventory.md`; `missing_reason 0` scan. |
| M3  | Create a temporary-allow follow-up | Select a temporary entry, draft its folder-style follow-up specification, obtain approval, and create its GitHub issue. | The inventory records the resulting issue number; no GitHub issue is created before its draft is approved. | DONE   | Numeric follow-ups #2244, #2245, #2246; UDP protocol baseline follow-up #2261. |
| M4  | Use ClippyFixer for remediation    | Provide a removable or retainable fixture finding to ClippyFixer.                                                       | The agent can edit the focused files, documents the decision, and delegates the commit to Committer.       | DONE   | `.github/agents/clippy-fixer.agent.md` declares `edit`; remediation commits are signed and inventory-tracked. |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence               |
| ----- | ---------------------- | ---------------------- |
| AC1   | DONE                   | Review-adjusted reconciliation reports 240 inventory rows, 16 removed entries, 224 active entries, and 224 source attributes. |
| AC2   | DONE                   | `clippy-allow-inventory.md` is stored in this issue folder and records scope plus generation/completeness method. |
| AC3   | DONE                   | `.github/agents/clippy-fixer.agent.md` declares `edit`; signed remediation commits remain separate from agent-profile documentation. |
| AC4   | DONE                   | Anchored source scan reports `missing_reason 0`; inventory retains matching decisions for all active source `allow` and `expect` attributes. |
| AC5   | DONE                   | Temporary entries link to approved follow-up issues #2244, #2245, #2246, and #2261. |
| AC6   | DONE                   | All 16 `Remove` entries are implemented and validated, including current-source reconciliation entry A235. |
| AC7   | DONE                   | Inventory has 136 `Retain`, 88 `Temporary`, and 16 `Remove` entries; no `Pending` entries remain. |
| AC8   | DONE                   | `linter all` passed; relevant focused package Clippy/test commands are recorded in `clippy-allow-inventory.md`. |

## Risks and Trade-offs

- The inventory is substantial. Batch by package and preserve one complete machine-checkable inventory.
- Some suppressions may be macro-generated or technically required. Retain them only with concrete evidence.
- The issue-local inventory is intentionally a human audit record. Do not make the later
  prospective validator depend on a path that moves when this issue closes.
- Temporary follow-ups increase issue count, but they prevent suppressions from becoming permanent
  undocumented debt. Group only genuinely inseparable fixes in one follow-up specification.

## Implementation Completion Review

After implementation, record material inventory, remediation, or follow-up workflow lessons in an
issue-local `implementation-retrospective.md`. If none occurred, add a concise progress-log entry
explaining why no retrospective is needed.

## References

- Parent EPIC: #2003
- GitHub issue: #2158
- Prospective policy: #2157 - Require Documented Clippy Allows
  (`docs/issues/open/2157-2003-require-documented-clippy-allows/ISSUE.md`)
- Inventory artifact: `clippy-allow-inventory.md` in this issue specification folder
- Remediation agent: `.github/agents/clippy-fixer.agent.md`
