---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2003
github-issue: 2261
spec-path: docs/issues/open/2261-2003-review-udp-protocol-clippy-baseline/ISSUE.md
branch: "2261-2003-review-udp-protocol-clippy-baseline"
related-pr: null
last-updated-utc: 2026-09-18 10:05
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/open/2158-2003-inventory-existing-clippy-allows/ISSUE.md
    - docs/issues/open/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
    - docs/issues/open/2158-2003-inventory-existing-clippy-allows/udp-protocol-clippy-baseline-draft.md
    - packages/udp-protocol/src/lib.rs
---

<!-- skill-link: create-issue -->

# Issue #2261 - Review UDP Protocol Clippy Baseline

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Review the remaining nonnumeric `udp-protocol` crate-level Clippy baseline, remove suppressions
that can be fixed without changing wire behavior, and narrow or document any retained exceptions
with native `reason` parameters.

## Background

Issue #2158 inventoried the historical Clippy allowance baseline and classified the broad
`packages/udp-protocol/src/lib.rs` crate-level suppressions as temporary. The UDP protocol crate
originated from vendored `aquatic_udp_protocol` code, so its baseline mixes independent concerns:
style, documentation, API ergonomics, imports, macro expansion, and protocol conversion behavior.

The numeric `cast_possible_truncation` crate-level allowance is already owned by #2245 as #2158
entry A156. This issue owns only the remaining nonnumeric UDP protocol baseline entries. Its job is
to replace the broad crate-level baseline with the narrowest clear outcome for each lint: remove,
move to a focused source location with a native rationale, or retain temporarily only with a stable
removal condition.

## Scope

### In Scope

- #2158 entries A157-A168 in `packages/udp-protocol/src/lib.rs`.
- The following current crate-level Clippy allowances:
  - `default_trait_access`
  - `doc_markdown`
  - `empty_enums`
  - `explicit_iter_loop`
  - `legacy_numeric_constants`
  - `match_same_arms`
  - `missing_errors_doc`
  - `missing_panics_doc`
  - `must_use_candidate`
  - `needless_pass_by_value`
  - `semicolon_if_nothing_returned`
  - `wildcard_imports`
- UDP protocol source files that trigger those lints after the crate-level allows are removed.
- Focused protocol tests when changing parsing, encoding, or public API behavior.
- Reconciliation back to #2158's inventory evidence for every owned entry.

### Out of Scope

- A156, the UDP protocol numeric `cast_possible_truncation` allowance owned by #2245.
- A171 and any other numeric protocol wire conversion work owned by #2245.
- Metric aggregate conversions owned by #2244.
- Domain numeric conversions owned by #2246.
- Broad protocol redesign or BEP behavior changes unrelated to eliminating or documenting the
  existing Clippy baseline.
- Changing public UDP protocol APIs solely to satisfy a stylistic lint without maintainer review.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`

- ADRs to create: None expected. Create one only if the review changes public protocol API policy,
  vendored-code modernization policy, or BEP compatibility behavior.

## Design and Ownership Review

This work is limited to UDP protocol code and its local tests. It does not introduce child
processes, asynchronous I/O, network readiness, resource cleanup, or reusable test fixtures.

Ownership boundary:

- `udp-protocol` owns wire types, parsing, and serialization behavior.
- #2245 owns numeric wire conversion boundaries and is intentionally excluded here.
- #2158 remains the historical inventory and handoff record; this issue owns the implementation
  outcome for A157-A168.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | TODO | Expose the real diagnostics | Temporarily remove or narrow each owned crate-level allow and record the concrete diagnostics by lint family. |
| T2 | TODO | Classify each lint family | For A157-A168, choose remove, narrow-with-reason, or retain temporarily with a stable removal condition. |
| T3 | TODO | Apply focused fixes | Implement behavior-preserving Clippy fixes or source-level `reason` annotations in small batches. |
| T4 | TODO | Validate protocol behavior | Run focused UDP protocol checks and tests for every batch that changes protocol source. |
| T5 | TODO | Reconcile #2158 evidence | Update the #2158 inventory with each final outcome and validation command. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1-T2 | Diagnostic and classification evidence in this specification | Commit after maintainer review of the classification. |
| T3-T4 | One lint family or tightly related group of UDP protocol changes | Commit after focused validation. |
| T5 | #2158 inventory reconciliation for A157-A168 | Commit after final focused validation. |

Record a justified no-change decision in the task's evidence without creating an empty commit.
Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-review-udp-protocol-clippy-baseline/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec moved to `docs/issues/open/` with the assigned issue number
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-18 00:00 UTC - GitHub Copilot - Drafted from #2158's UDP protocol
  baseline design input; awaiting maintainer review before GitHub issue creation - This specification
- 2026-09-18 10:05 UTC - josecelano - Approved the draft specification and confirmed it should remain a narrow #2003 child task - Chat decision
- 2026-09-18 10:05 UTC - GitHub Copilot - Created GitHub issue #2261 and promoted the specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2261

## Acceptance Criteria

- [ ] Every #2158 entry A157-A168 has a recorded diagnostic, outcome, and validation result.
- [ ] Every owned crate-level UDP protocol allowance is removed, narrowed to a source-specific
      allowance with a native `reason`, or retained temporarily with a stable removal condition.
- [ ] A156 remains owned by #2245 and is not broadened by this issue.
- [ ] Changed UDP protocol parsing or serialization behavior is covered by focused tests.
- [ ] #2158's inventory records the final outcome for A157-A168.
- [ ] `linter all` exits with code `0` and relevant UDP protocol tests pass.

## Verification Plan

### Automatic Checks

- `cargo clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings`
- `cargo test -p torrust-tracker-udp-protocol --all-targets --all-features`
- `linter all`

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Review #2158 reconciliation | Compare A157-A168 in #2158's inventory against the final UDP protocol source and this issue's evidence. | Every entry has a final outcome and no owner ambiguity remains. | TODO | `manual-verification-evidence.md` section M1 |

### Disposable Verification Scripts

No disposable verification script is planned. Prefer focused Rust tests and repository linters.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | Pending implementation. |
| AC2 | TODO | Pending implementation. |
| AC3 | TODO | Pending implementation. |
| AC4 | TODO | Pending implementation. |
| AC5 | TODO | Pending implementation. |
| AC6 | TODO | Pending validation. |

## Risks and Trade-offs

- Removing crate-level suppressions may expose generated-code diagnostics from `zerocopy` or
  other derives. Narrow generated-code exceptions rather than hiding unrelated lint families.
- Behavior-preserving Clippy fixes in protocol code can still affect wire compatibility if applied
  mechanically. Validate parsing and encoding boundaries after any source change.
- Retaining broad crate-level allowances would keep the original problem alive. Prefer item-level
  evidence or documented follow-up conditions.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- Create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`
  in this folder if the work invalidates an assumption, changes design materially, or yields a
  reusable lesson; otherwise add a progress-log entry stating why none is needed.

## References

- Parent EPIC: #2003
- Related issues: #2158, #2245
- Related PRs: #2259
- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
