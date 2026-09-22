---
doc-type: test-refactor-plan-guidance
issue: 2283
package: torrust-tracker-udp-server
status: done
---

# UDP Server Test Plan Guidance

This folder contains concise decision records for source files selected by Issue #2283. The
[coverage inventory](../coverage-evidence.md) is authoritative for the whole package; a decision
record authorizes neither test changes nor work on another file until the maintainer approves it.

## Order And Status

Group selected files by cohesive subsystem and complete the current subsystem before moving to an
unrelated one. Within a subsystem, only one file may be `IN_PROGRESS`. Its record must become
`DONE`, with focused validation and completed-file review, before another file begins.

The current queue contains only the top-level error-adapter subsystem:

| Order | Record | Status | Boundary |
| ---: | --- | --- | --- |
| 1 | [Error adapter tests](error-tests.md) | DONE | `error.rs` conversions from UDP-core service errors |

## Required Two-Phase Sequence

1. Review the current tests for readability, duplication, causal state, visible Arrange-Act-Assert
   structure, deterministic execution, and independently specified expected results. Implement and
   validate every approved cleanup before adding coverage. Record a no-change decision when no
   concrete cleanup is justified.
2. Add only the approved behavior-focused unit increment. Keep the source error, conversion Act,
   and typed expected result visible. Do not derive expected values with production conversion code.
3. Perform the prose-first Arrange-Act-Assert review, run focused validation, update the record,
   and obtain completed-file review before another file begins.

## Shared Guardrails

- Unit-only coverage is the primary metric. Integration tests require a documented reason that a
  real-loopback boundary is clearer or necessary.
- Do not add tests merely for coverage, reopen #2149's completed parse-error contracts, duplicate
  `udp-core` service behavior, or cross into response serialization, event classification, or
  lifecycle policy.
- Use the nightly Rust toolchain for formatting evidence: `cargo +nightly fmt --all -- --check`.
- Record the toolchain/runtime with every validation result. Run `git diff --check` after each
  increment and `linter markdown` plus `linter cspell` after changing these records.
- Shared helpers require an approved cohesive responsibility; do not create generic fixtures for a
  one-file conversion seam.

## Reconciliation

Before final verification, reconcile every record status with `coverage-evidence.md`, `ISSUE.md`,
and EPIC #1347. Do not cite branch SHAs as evidence until the implementation is merged; use unique
Conventional Commit subjects instead.
