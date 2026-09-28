---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p3
epic: 2003
github-issue: 2360
spec-path: docs/issues/open/2360-2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md
branch: "2360-2003-resolve-udp-protocol-empty-enums-allowance-scope"
related-pr: null
last-updated-utc: "2026-09-28 09:42"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/udp-protocol/src/lib.rs
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
    - docs/issues/closed/2261-2003-review-udp-protocol-clippy-baseline/ISSUE.md
    - review-finding:pr-2290-f4
---

<!-- skill-link: create-issue -->

# Issue #2360 - Resolve the Crate-Level Scope of the UDP Protocol `empty_enums` Allowance

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Make the UDP protocol crate's `clippy::empty_enums` allowance (inventory entry A159) meet the
Clippy exception decision framework, or change the framework through maintainer review, so that
the AC2 of #2261 and the framework no longer contradict the code.

## Background

Issue #2261 (PR #2290) removed eleven crate-level Clippy allowances from `packages/udp-protocol`. It
restored A159 after nightly Clippy began reporting `zerocopy` `FromBytes` derive expansion. The
source now reads:

```rust
#![allow(
    clippy::empty_enums,
    reason = "FromBytes derives generate empty helper enums for inhabited protocol wire structs"
)]
```

The post-merge review of PR #2290 (`review-finding:pr-2290-f4`, tracked by #2347) found that
this outcome matches none of the three outcomes #2261's AC2 permits: removed, narrowed to a
source-specific allowance, or retained temporarily with a stable removal condition. The
framework also says: "Broad crate-level suppression and statements that only say the warning is
intentional are insufficient." #2347 approved this as a follow-up because resolving it needs a
Rust change or a framework change.

Facts recorded during #2347 triage on `develop` at `478516cf`, with `lib.rs` restored after each
experiment:

- The allowance is needed only on nightly Clippy. Without it, `cargo +nightly clippy -p
  torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings` (Rust
  `1.100.0-nightly`, 2026-09-23) reports 18 unique `enum with no variants` diagnostics:
  `common.rs` 11, `announce.rs` 5, `connect.rs` 1, and `scrape.rs` 1. Stable `cargo clippy`
  (Rust `1.98.1`) reports none.
- An item-level `#[allow(clippy::empty_enums, reason = ...)]` on the `connect.rs` struct does not
  suppress its diagnostic. The derive's generated item is a sibling of the struct, so the
  narrowest attribute that works is probably a module-level `#![allow]`. This is unverified
  beyond that one struct.
- The derive comes from `zerocopy` `0.8.57` (`features = ["derive"]`).

## Scope

### In Scope

- Check upstream whether `clippy::empty_enums` firing inside `zerocopy` derive expansion is a
  known or fixed issue in `rust-clippy` or `zerocopy`, and whether a newer `zerocopy` avoids it.
- Present the maintainer with one of these outcomes, with evidence:
  - **Narrow**: replace the crate-level allowance with a module-level `#![allow]` carrying a
    native reason in each emitting module.
  - **Temporary**: keep the crate-level allowance, with a native reason naming a stable removal
    condition, such as an upstream issue link. This is the framework's temporary-exception
    outcome.
  - **Framework carve-out**: amend the framework, through maintainer review, to admit crate-level
    suppression for derive-generated code that no narrower attribute can reach.
- Apply the approved outcome, update inventory entry A159, and append a note to the #2261 spec
  recording how AC2 is now satisfied.

### Out of Scope

- Other Clippy allowances, including A156 (`cast_possible_truncation`, owned by #2245).
- Replacing `zerocopy` or the `FromBytes` derives.
- Rewriting historical #2261 or #2158 evidence. Corrections are appended.

## Architectural Decisions

- Related ADRs: `None known`.
- ADRs to create: `None known`. A framework carve-out is recorded in the framework document,
  which requires maintainer review, rather than in an ADR.

## Design and Ownership Review

Not applicable. The change is a lint attribute or a documentation change; it adds no process,
I/O, or fixture code.

## Bug-Fix Process

Not applicable. The runtime behaviour is correct; the finding concerns the scope of a lint
suppression and its documented justification.

## Regression Test Strategy

Not applicable. Clippy on the nightly toolchain is the guard. M1 and M2 record it.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Research upstream and verify the narrowest attribute | Upstream issue links or "none found"; confirmation of whether module-level `#![allow]` suppresses all 18 diagnostics on nightly. |
| T2 | TODO | Maintainer decision | One outcome (Narrow, Temporary, or Framework carve-out) approved and recorded in the progress log. |
| T3 | TODO | Apply the outcome | Source or framework change; A159 inventory entry updated; #2261 note appended. |
| T4 | TODO | Verify and record | M1-M2 recorded; automatic checks pass. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T3 | The allowance or framework change | One signed commit (`refactor(udp-protocol)` or `docs(quality)`). |
| T3 | Inventory and #2261 notes | One signed `docs(issues)` commit. |
| T4 | Evidence and tracking | One signed `docs(issues)` commit. |

T1 and T2 change no repository file; their results are recorded in the progress log.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-resolve-udp-protocol-empty-enums-allowance-scope/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created, linked as a sub-issue of #2003, and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-28 09:40 UTC - GitHub Copilot - Drafted as follow-up FU-A of #2347 (`review-finding:pr-2290-f4`), whose T3 approval routes this finding here: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>. The background facts come from #2347 triage and a working-tree spike on `develop` at `478516cf`; the files were restored afterwards.
- 2026-09-28 09:42 UTC - GitHub Copilot - Maintainer approved the specification. Created GitHub issue #2360, linked it as a sub-issue of #2003 (`parent_issue_url` verified), registered it as order 13 in the #2003 EPIC, and moved this specification to `docs/issues/open/`.

## Acceptance Criteria

- [ ] AC1: The A159 allowance either has the narrowest scope that suppresses the nightly
      diagnostics, or has a native reason naming a stable removal condition, or the framework
      admits its scope. The outcome is maintainer-approved.
- [ ] AC2: Nightly and stable Clippy pass for `torrust-tracker-udp-protocol` with `-D warnings`.
- [ ] AC3: Inventory entry A159 and an appended #2261 note record the outcome.
- [ ] AC4: When upstream issues exist, they are linked from the native reason or the inventory.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings` (stable Rust toolchain)
- `cargo +nightly clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings` (nightly Rust toolchain)
- `cargo test -p torrust-tracker-udp-protocol --all-targets --all-features` (stable Rust toolchain)
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | The allowance is still needed where it applies | In the working tree, remove the allowance in its new scope, run the nightly Clippy command, then restore the file. | The same 18 unique `enum with no variants` diagnostics as before, and none outside the allowance's scope. | TODO | `manual-verification-evidence.md` section V1 |
| M2 | The final tree is clean on both toolchains | Run the stable and nightly Clippy commands on the committed tree. | Both exit `0`. | TODO | `manual-verification-evidence.md` section V2 |

No disposable verification script is planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Progress log decision; source or framework diff |
| AC2 | TODO | M2 |
| AC3 | TODO | Inventory and #2261 diffs |
| AC4 | TODO | T1 result |

## Risks and Trade-offs

- A future nightly may stop emitting the diagnostics. If so, M1 shows no diagnostics, and the
  right outcome is to remove A159 rather than narrow it.
- Four module-level allowances are more lines than one crate-level allowance. The framework
  prefers source-specific scope, so the maintainer decides whether the trade is worth it.

## Implementation Completion Review

- Retrospective: `Not yet assessed`. Record in the progress log why no retrospective is needed,
  unless T1 finds a reusable lesson about derive-generated lints.

## References

- Source finding: `review-finding:pr-2290-f4` (<https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700056>).
- Routing issue: #2347.
- Framework: `docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md`.
