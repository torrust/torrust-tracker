---
doc-type: issue
issue-type: task
status: done
priority: p2
epic: 2003
github-issue: 2261
spec-path: docs/issues/closed/2261-2003-review-udp-protocol-clippy-baseline/ISSUE.md
branch: "2261-2003-review-udp-protocol-clippy-baseline"
related-pr: 2290
last-updated-utc: 2026-09-22 10:18
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/ISSUE.md
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-allow-inventory.md
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md
    - docs/issues/closed/2158-2003-inventory-existing-clippy-allows/udp-protocol-clippy-baseline-draft.md
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

## Maintainer Decisions

- The first commit records maintainer feedback before diagnostics and classification work begins.
- The upcoming 4.0.0 major release permits breaking UDP protocol API changes when they produce a
  clearer design; preserving the current API is not a reason to retain an allowance.
- Apply the shared [Clippy exception decision framework](../../closed/2158-2003-inventory-existing-clippy-allows/clippy-exception-decision-framework.md).
  A permanent, evidence-backed exception may remain indefinitely. A temporary exception requires
  a concrete refactor or external-change condition and must not persist because the refactor was
  forgotten.

## Diagnostic and Classification Evidence

The owned crate-level controls were removed together and checked with stable and nightly Clippy.
Eleven outcomes remove the crate-level allowance. A159 remains as a permanent documented
generated-code exception because the current nightly toolchain emits it from `FromBytes` derive
expansion.

| Inventory ID | Lint | Diagnostic outcome | Final outcome |
| ------------ | ---- | ------------------ | ------------- |
| A157 | `default_trait_access` | `AnnounceResponse::empty` initialized `Vec` through `Default::default`. | Replaced with `Vec::default`. |
| A158 | `doc_markdown` | No stable or nightly diagnostic. | Removed unused crate-level allowance. |
| A159 | `empty_enums` | Nightly Rust 1.100.0 (2026-09-21) emits 37 diagnostics from `FromBytes` derive expansion for inhabited protocol wire structs. | Retained at crate scope with a native generated-code rationale. |
| A160 | `explicit_iter_loop` | Test generators iterated with `.iter_mut()`. | Replaced with direct mutable-array iteration. |
| A161 | `legacy_numeric_constants` | No stable or nightly diagnostic. | Removed unused crate-level allowance. |
| A162 | `match_same_arms` | A test generator had two equivalent arms. | Merged the equivalent patterns. |
| A163 | `missing_errors_doc` | Public wire read, parse, and write APIs lacked error contracts. | Added specific `# Errors` documentation. |
| A164 | `missing_panics_doc` | `Request::parse_bytes` used `unwrap` after slicing action bytes. | Replaced with a fallible conversion. |
| A165 | `must_use_candidate` | Constructors and error factories discard meaningful values if ignored. | Added `#[must_use]`. |
| A166 | `needless_pass_by_value` | Round-trip test helpers did not consume their request or response. | Removed the helper and inlined each test's conversion and assertion. |
| A167 | `semicolon_if_nothing_returned` | Test-only assignment calls omitted semicolons. | Added semicolons. |
| A168 | `wildcard_imports` | Four protocol modules used wildcard imports. | Replaced with explicit production and test-only imports. |

Focused validation: `cargo clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings`
and `cargo test -p torrust-tracker-udp-protocol --all-targets --all-features` both exit with code `0`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| -- | ------ | ---- | ----------------------- |
| T1 | DONE | Expose the real diagnostics | Removed the owned crate-level controls and recorded the results by lint family. |
| T2 | DONE | Classify each lint family | A157-A158 and A160-A168 are removed; A159 is a permanent documented generated-code exception. |
| T3 | DONE | Apply focused fixes | Applied behavior-preserving fixes, public API annotations, explicit imports, and documentation. |
| T4 | DONE | Validate protocol behavior | Focused Clippy and all nine UDP protocol tests pass. |
| T5 | DONE | Reconcile #2158 evidence | Updated the inventory entries with final outcomes and focused validation. |

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
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-18 00:00 UTC - GitHub Copilot - Drafted from #2158's UDP protocol
  baseline design input; awaiting maintainer review before GitHub issue creation - This specification
- 2026-09-18 10:05 UTC - josecelano - Approved the draft specification and confirmed it should remain a narrow #2003 child task - Chat decision
- 2026-09-18 10:05 UTC - GitHub Copilot - Created GitHub issue #2261 and promoted the specification to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2261
- 2026-09-21 00:00 UTC - josecelano - Approved the implementation branch and defined the shared
  Clippy exception decision policy: prefer fixes or clearer 4.0.0 API changes; retain only
  evidence-backed permanent exceptions; require a concrete tracked refactor or removal condition
  for temporary exceptions - Chat decision
- 2026-09-21 00:00 UTC - GitHub Copilot - Removed all A157-A168 crate-level controls and
  applied their focused source fixes; stable Clippy and all UDP protocol tests pass - Pending
  repository-wide validation
- 2026-09-22 06:23 UTC - GitHub Copilot - Completed M1 reconciliation: A157-A168 are absent
  from source and recorded as removed, while A156 remains assigned to #2245; `linter all` passed
  - `manual-verification-evidence.md`
- 2026-09-22 06:23 UTC - GitHub Copilot - Independent task review found missing M1, completion,
  full-linter, and prose-first test evidence. Added the evidence and refactored the modified
  round-trip tests to expose Arrange, Act, and Assert - Pending refreshed independent review
- 2026-09-22 06:43 UTC - GitHub Copilot - Reconciled superseded #2158 evidence, made parser
  boundary tests behavior-focused with visible assertions, and passed the final independent task
  review; `linter all` passed - Ready for PR
- 2026-09-22 07:49 UTC - GitHub Copilot - Updated nightly Rust to 1.100.0 (2026-09-21) after CI
  began reporting `empty_enums` from `FromBytes` derive expansion. Restored A159 as a documented
  permanent generated-code exception; current-nightly focused Clippy, focused tests, and `linter all` passed
- 2026-09-22 10:18 UTC - GitHub Copilot - PR #2290 merged and closed GitHub issue #2261 as
  completed. Archived this specification under `docs/issues/closed/` - https://github.com/torrust/torrust-tracker/pull/2290

## Acceptance Criteria

- [x] Every #2158 entry A157-A168 has a recorded diagnostic, outcome, and validation result.
- [x] Every owned crate-level UDP protocol allowance is removed, narrowed to a source-specific
      allowance with a native `reason`, or retained temporarily with a stable removal condition.
- [x] A156 remains owned by #2245 and is not broadened by this issue.
- [x] Changed UDP protocol parsing or serialization behavior is covered by focused tests.
- [x] #2158's inventory records the final outcome for A157-A168.
- [x] `linter all` exits with code `0` and relevant UDP protocol tests pass.

## Verification Plan

### Automatic Checks

- `cargo clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings`
- `cargo test -p torrust-tracker-udp-protocol --all-targets --all-features`
- `linter all`

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| -- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Review #2158 reconciliation | Compare A157-A168 in #2158's inventory against the final UDP protocol source and this issue's evidence. | Every entry has a final outcome and no owner ambiguity remains. | DONE | `manual-verification-evidence.md` section M1 |

### Disposable Verification Scripts

No disposable verification script is planned. Prefer focused Rust tests and repository linters.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | DONE | Diagnostic and Classification Evidence records all A157-A168 outcomes. |
| AC2 | DONE | Eleven allowances were removed; A159 remains with a native generated-code reason. |
| AC3 | DONE | The #2245 `cast_possible_truncation` allowance remains unchanged. |
| AC4 | DONE | Existing request and response round-trip tests pass after parsing and serialization changes. |
| AC5 | DONE | #2158 inventory entries A157-A168 record final outcomes and focused validation evidence. |
| AC6 | DONE | Current-nightly focused UDP protocol Clippy and tests plus `linter all` pass. |

## Risks and Trade-offs

- Removing crate-level suppressions may expose generated-code diagnostics from `zerocopy` or
  other derives. Narrow generated-code exceptions rather than hiding unrelated lint families.
- Behavior-preserving Clippy fixes in protocol code can still affect wire compatibility if applied
  mechanically. Validate parsing and encoding boundaries after any source change.
- Retaining broad crate-level allowances would keep the original problem alive. Prefer item-level
  evidence or documented follow-up conditions.

## Implementation Completion Review

- Retrospective: `implementation-retrospective.md` records the reusable decision-framework and
  test-review lessons.

### Prose-First Test Evidence

The seven modified request and response round-trip property tests now state their behavior in
their names. Arrange converts the generated wire value into its request or response enum and
creates an output buffer; Act writes the value and parses the resulting bytes; Assert compares the
independently parsed value with the original. The scrape-request property first discards the
invalid empty-info-hash state, then follows the same flow. The final code retains only the
Arrange-Act-Assert markers because the temporary prose adds no irreducible context.

The request parser boundary tests use the same review: Arrange creates each supported action at
each packet length or a scrape request without info hashes; Act calls `Request::parse_bytes`; Assert
checks that parsing does not panic or returns an error, respectively. Their names state those
observable contracts.

## References

- Parent EPIC: #2003
- Related issues: #2158, #2245
- Related PRs: #2259
- Related ADRs: `docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md`
