# Test Refactor Plan Guidance

This folder contains one proposed refactor plan for each test-bearing file in
`packages/axum-http-server/`. Create, review, and implement plans one file at a time. Do not begin
a plan for the next file until the current plan's approved work has been completed and reviewed.

Draft cross-file plans belong in `drafts/`. They assess a shared concern without authorizing a
cross-file extraction; promote one only after maintainer review establishes a cohesive common
responsibility.

## Plans

- [Shared handler test bootstrap assessment](drafts/shared-handler-test-bootstrap.md) — complete
- [Announce handler tests](announce-tests.md) — complete
- [Scrape handler tests](scrape-tests.md) — complete
- [Routes tests](routes-tests.md) — complete
- [Authentication-key extractor tests](authentication-key-extractor-tests.md) — complete
- [HTTP server tests](server-tests.md) — complete

## Shared Purpose

Each plan improves tests without changing production behavior. Its scope is deliberately limited to
the target file. Review and approve the plan before implementing any item.

## Shared Quality Goals

The refactoring must improve or preserve:

- **Expressiveness:** a test tells the reader the behavior and its relevant inputs.
- **Readability:** a reader can distinguish setup, action, actual result, and expected result.
- **Maintainability:** a behavior change has one intentional fixture or helper to update.
- **One behavior-focused contract:** each test asserts one observable contract; a failure should
  make that contract clear. This does not mean a wire-boundary test has literally one possible
  internal defect.
- **Coverage:** retain existing valuable coverage and add coverage only for identified,
  behavior-focused gaps.

The refactoring must reduce or avoid:

- **Flakiness:** no sleeps, retries, wall-clock dependencies, uncontrolled network I/O, or shared
  mutable state.
- **Duplication:** share only genuinely repeated mechanical setup or decoding.
- **Complexity:** helpers must not hide behavior selection, expected-value construction, or the
  system under test (SUT).

## Plan Structure

Every file plan must contain:

1. **Phase 1 — Identify Problems:** evidence-based, file-specific opportunities and strengths to
   preserve.
2. **Phase 2 — Proposed Refactorings:** items ordered from high-impact/low-effort to
   low-impact/high-effort, including behavior and abstraction guardrails.
3. **Progress Tracking:** a status checklist, progress log, and validation evidence for the plan.
4. **Non-Goals, validation, and completion criteria:** constraints that prevent speculative work
   and preserve the user-review checkpoint.

Use `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, or `DEFERRED` for each proposed refactoring. Update
one item at a time: record its review decision, implementation outcome, and focused validation
before beginning the next item. Do not mark the plan complete until the maintainer has reviewed all
approved changes.
