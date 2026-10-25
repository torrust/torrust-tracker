---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p3
epic: null
github-issue: null
spec-path: docs/issues/drafts/rest-api-auth-ignores-unreadable-authorization-header/ISSUE.md
branch: "{issue-number}-rest-api-auth-ignores-unreadable-authorization-header"
related-pr: null
last-updated-utc: "2026-10-25 06:30"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
    - docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md
    - packages/axum-rest-api-server/src/v1/middlewares/auth.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - REST API authentication ignores an unreadable `Authorization` header

Found while planning the authentication middleware tests of #2637 (EPIC #1347). Not part of an EPIC; the
maintainer decides whether to attach it to one.

## Goal

Decide, and then make consistent and tested, how the REST API authentication middleware treats an
`Authorization` header whose value is not readable text (not visible ASCII).

## Background

`packages/axum-rest-api-server/src/v1/middlewares/auth.rs` takes the token from the `Authorization: Bearer
<token>` header or from the `token` query parameter, and documents that the header wins when both are
present. `extract_bearer_token_from_header` reads the header with
`header_value.to_str().ok()`. A value that is not visible ASCII (for example the bytes `Bearer \xff`) makes
`to_str()` fail, and `.ok()` turns that into "no header". The middleware then falls back to the query
parameter.

Observed behavior (a throwaway in-process test of the middleware with token `tok`, reverted; not a
maintained test):

| Request | Result |
| --- | --- |
| `Authorization: Bearer \xff` and `?token=tok` | `200`, the handler runs |
| `Authorization: Basic tok` and `?token=tok`, or `Authorization: bearer tok` | `500`, `unknown token provided` |
| `Authorization: Bearer tok` | `200` |

So an unreadable header is silently ignored, while a readable header with another scheme is rejected. A
caller that sends a garbled header together with a valid query token is authenticated.

The concern is consistency and the documented contract ("the `Authorization` header will be used" when both
are provided), not a bypass: the query token must still be valid, and a request without a valid token is
rejected in every case observed.

## Scope

### In Scope

- Decide the intended behavior (see Open Decisions).
- Make `extract_bearer_token_from_header` implement the decision.
- Add a unit test of the middleware for the decided behavior, in the in-file test module that #2637 creates.
- Update the module documentation of `auth.rs` if the contract text changes.

### Out of Scope

- Changing the response status or body of the failures (`500` with the legacy body, kept for backward
  compatibility).
- Any other part of #2637.

## Architectural Decisions

- Related ADRs: none known.
- ADRs to create: none known.

## Design and Ownership Review

The decision belongs to the `axum-rest-api-server` authentication middleware. No other package parses this
header for the REST API. The conflict with remaining work of #2637 is limited to the plan for
`v1/middlewares/auth.rs` (S1, ledger row 1): the fix adds a test case and changes the same function.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. **Analysis.** `to_str()` returns an error for any byte outside visible ASCII, and `.ok()` discards it, so the
   error path of the header check is the "header absent" path. Hypothesis: this was not a deliberate decision.
2. **Reproduction on the real artifact.** Start an isolated tracker (see the `run-tracker-locally` skill) and
   run `curl -H $'Authorization: Bearer \xff' 'http://127.0.0.1:1212/api/v1/stats?token=<valid>'`, then the same
   request with `Authorization: Basic x`. Record both in the issue-local `manual-verification-evidence.md`. If the
   real artifact cannot be used, record the attempted commands and the blocking constraint.
3. **Regression-test boundary.** A unit test of the middleware through a minimal router.
4. **Red evidence**, **fix**, **green evidence**, and a **like-for-like recheck** with the same `curl` commands.

## Regression Test Strategy

The smallest deterministic boundary is a unit test in `auth.rs` that sends the header `Bearer \xff` together
with a valid `token` query to a minimal router and asserts the decided status and body. The Arrange builds the
router and a recording handler, so the test can also assert that the handler ran or did not. Boundary rationale:
the decision is made inside the middleware and needs no socket. Link the red and green output and the final
recheck to `manual-verification-evidence.md`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Maintainer decides the intended behavior | Resolves the Open Decisions below. |
| T2 | TODO | Reproduce on the real tracker | `manual-verification-evidence.md` V1. |
| T3 | TODO | Add the failing regression test (red) | Test fails for the current behavior if the decision is to reject. |
| T4 | TODO | Fix `extract_bearer_token_from_header` and the documentation | Test passes (green). |
| T5 | TODO | Final like-for-like recheck and verification | `manual-verification-evidence.md` V2; `linter all`; package tests. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T3 | The regression test | Commit after focused validation and the prose-first Arrange-Act-Assert review. |
| T4 | The fix and the documentation | Commit after the test passes. |
| T5 | Evidence | Commit after the recheck. |

Use Conventional Commit messages (`fix(axum-rest-api-server): ...`) and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/rest-api-auth-ignores-unreadable-authorization-header/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-25 06:30 UTC - GitHub Copilot - Drafted this specification from the #2637 S1 plan review. The
  maintainer asked for a hand-off and a stop. The conflicting remaining work is the #2637 plan for
  `v1/middlewares/auth.rs`.

## Acceptance Criteria

- [ ] AC1: The behavior for an unreadable `Authorization` header is decided and documented in `auth.rs`.
- [ ] AC2: A unit test protects the decided behavior and fails for the opposite behavior.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-axum-rest-api-server` (stable Rust toolchain)
- `cargo +nightly fmt --all -- --check` (nightly Rust toolchain)

### Manual Verification Scenarios

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reproduce on the real tracker | Start an isolated tracker; `curl -H $'Authorization: Bearer \xff' 'http://127.0.0.1:1212/api/v1/stats?token=<valid>'` and with `Authorization: Basic x`. | Before the fix: `200` for the unreadable header, `500` for the other scheme. | TODO | `manual-verification-evidence.md` V1 |
| M2 | Like-for-like recheck | Repeat M1 after the fix. | The decided behavior. | TODO | `manual-verification-evidence.md` V2 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1 | TODO | The documentation diff and the decision record. |
| AC2 | TODO | Red and green test output. |

## Risks and Trade-offs

- Rejecting an unreadable header could break a client that sends a corrupted header and relies on the query
  token; such a client is already outside the documented contract, and the change is visible only to it.
- Keeping the behavior means documenting that an unreadable header is ignored, which is a less strict contract.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## Open Decisions (maintainer)

1. **Intended behavior.** Choose one:
   - **Reject.** Treat an unreadable header like another scheme and answer `unknown token provided`
     (consistent with `Basic <token>` and `bearer <token>`, and with "the header takes precedence").
   - **Keep and document.** Ignore an unreadable header and say so in the module documentation.
2. **Parent and priority.** Attach to an EPIC or leave it without a parent; confirm priority `p3`.

## References

- Found by: #2637 (EPIC #1347), plan `v1/middlewares/auth.rs`
- Code: `packages/axum-rest-api-server/src/v1/middlewares/auth.rs` (`extract_bearer_token_from_header`)
- Security record for the token comparison: `docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md`
