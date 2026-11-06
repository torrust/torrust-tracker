---
doc-type: file-test-plan
issue: 2637
package: torrust-tracker-axum-rest-api-server
target-file: packages/axum-rest-api-server/src/testing/
status: done
semantic-links:
  related-artifacts:
    - packages/axum-rest-api-server/src/testing/environment.rs
    - packages/axum-rest-api-server/src/testing/mod.rs
    - docs/issues/open/2745-2482-gate-axum-rest-api-server-test-environment/ISSUE.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/coverage-evidence.md
---

# Test Support File Test Plan

Follow the shared [guidance](README.md). This plan covers ledger rows 19 (`testing/environment.rs`) and 20
(`testing/mod.rs`), the two files that are test support, not production behavior. It is one plan for the pair because the
decision is the same and no test is proposed. Status: done; the maintainer approved the plan on 2026-11-06.

## Current State

- **Unit-only coverage:** `testing/environment.rs` 301 / 332 lines (90.66%) at the S5 re-measurement; `testing/mod.rs` has no
  executable entry (a module declaration and documentation). The figures include the file's own in-file tests (8 unit tests
  added by #2449 are the reason the file is mostly covered).
- **What the files are:** a test environment that wires the containers and starts and stops the REST API server for the
  integration tests of this package and for other packages (`axum-health-check-api-server`). It is exported unconditionally
  today; the open spec #2745 (under EPIC #2482) gates it behind a `testing` feature, and #2449 (closed) changed its lifecycle.
- **Module-owned decisions:** none of the package's behavior: it only composes what other files decide.

## Coverage And Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| The 31 uncovered lines of `testing/environment.rs` (start and stop failure arms, the failed-task panic paths and similar) | Collaborator-owned / out of scope | No test. The EPIC does not test test code, and the file is owned by #2482 (gating) and #2449 (lifecycle); a test now would be rewritten or discarded by the gating change. The environment is exercised, as its job requires, by every integration test of the package. |
| `testing/mod.rs` | No executable entries | No test: documentation and `pub mod environment;`. |

Per the spec ([Out of Scope](../ISSUE.md#out-of-scope)): do not test `src/testing/`. This plan records the decision with its
owner so the ledger rows can close.

## Challenge Of The No-Change Conclusion

Named mutations on the real code would only show that the environment's own tests (or the integration tests that use it)
fail when it is broken, which is the intended use and says nothing about the REST API server. No mutation is applied to
these files. If #2745 lands before S6 is merged, re-measure at the slice close and keep the decision.

## Property-Test Decision

None.

## Dependencies

None.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | SKIPPED | No tests to refactor under this decision. | None. |
| R2 | SKIPPED | No tests added. | None. |
| R3 | SKIPPED | Integration increment: `none`. | None. |
| R4 | DONE | Record the figures at the slice close. | `coverage-evidence.md`. |

## Prose-First AAA Review

No test is added or changed.

## Results

The maintainer approved the out-of-scope decision on 2026-11-06: no test, no mutation, no change of the two files. `testing/environment.rs` stays at 301 / 332 unit-only lines (S6 baseline) and `testing/mod.rs` has no executable entry. The decision stays valid if #2745 (gating behind a `testing` feature) lands; the figures are re-measured at the slice close.

## Progress Log

- 2026-11-06 10:39 UTC - Claude Code - Plan drafted; waiting for the maintainer's approval.
- 2026-11-06 11:13 UTC - Claude Code - Plan approved by the maintainer; decision recorded, rows `DONE`.
