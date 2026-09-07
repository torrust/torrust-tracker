# UDP Server Test Refactor Plan Guidance

This folder contains one proposed refactor plan for each test-bearing source or integration-test
file selected during this issue's T2 review. Create, review, and implement plans one file at a
time. Do not begin a plan for the next file until the current plan's approved work is completed
and reviewed.

Draft cross-file plans belong in `drafts/`. They may assess a shared concern, but do not authorize
a cross-file extraction unless maintainer review establishes a cohesive common responsibility.

## Plans

- [Request-buffer tests](request-buffer-tests.md) — proposed; awaiting maintainer approval.

## Shared Purpose

Each plan improves test code without changing production behavior. It applies only to its target
file and must be reviewed and approved before any proposed item is implemented.

## Shared Quality Goals

The refactoring must improve or preserve:

- **Expressiveness:** each test states its behavior and the initial state that causes it.
- **Readability:** Arrange, Act, actual result, expected result, and Assert are distinguishable.
- **Maintainability:** common mechanics have one intentional update site, while behavior-specific
  values remain near the test.
- **One behavior-focused contract:** a failure identifies the observable contract; a wire-boundary
  contract may still have more than one possible internal cause.
- **Coverage:** retain valuable existing coverage and add only behavior-focused coverage gaps.

The refactoring must reduce or avoid:

- **Flakiness:** no sleeps, retry loops, wall-clock dependencies, uncontrolled networking, or
  shared mutable state.
- **Duplication:** share only genuinely repeated mechanics such as deterministic packet decoding
  or non-behavioral fixture setup.
- **Complexity:** helpers must not hide the causal state, production Act, expected outcome, or
  final assertion.
- **Scope leakage:** do not duplicate `udp-protocol`, `udp-core`, root composition, or #1488
  shutdown work.

## Plan Structure

Every file plan must contain:

1. **Phase 1 — Identify Problems:** evidence-based, file-specific strengths and opportunities.
2. **Phase 2 — Proposed Refactorings:** items ordered from high-impact/low-effort to
   low-impact/high-effort, with behavior and abstraction guardrails.
3. **Progress Tracking:** status checklist, progress log, and validation evidence.
4. **Non-Goals, validation, and completion criteria:** constraints that prevent speculative churn
   and preserve the maintainer-review checkpoint.

Use `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`, or `DEFERRED` for each item. Update one item at a
time: record its review decision, implementation outcome, and focused validation before starting
the next item. Do not mark a plan complete until the maintainer has reviewed all approved changes.
