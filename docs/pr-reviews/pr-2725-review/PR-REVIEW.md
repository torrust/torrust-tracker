---
pr-number: 2725
pr-url: https://github.com/torrust/torrust-tracker/pull/2725
last-updated-utc: "2026-11-04 18:42"
---

# PR #2725 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2725.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Round 1: Copilot review 5773261511 on the head `6d0731d13` (two inline findings). Both are valid and fixed.

Round 2: Copilot review 5773741514 on the head `3c9771b0b` (one finding, raised on two threads on the same line; the second is a duplicate).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2725-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2725-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2725-f3 | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2725-f4 | Copilot | Minor (inferred) | metadata | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Define ownership and cleanup for the asynchronous startup path

- PR number: 2725
- Source review ID: 5773261511
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471316348
- Concern: The spec says the environment's lifecycle is unchanged, but task T4 adds a separate asynchronous startup path (the production-style example) without defining its task ownership, cleanup guarantees, absolute readiness deadlines or a design-review checkpoint, as `docs/templates/ISSUE.md` requires.
- Solution: Valid. The Design and Ownership Review section now defines the example's responsibilities (two cancellation tokens, the server join handle and the listener handles owned by `main`), the normal, failure and drop paths (server joined before listeners; listeners spawned only after a successful start; drop guards on both tokens), one absolute deadline bounding every await, and a design-review checkpoint after the first passing T4 slice.
- Current-tree verification: Read the section in `docs/issues/open/2724-2482-gate-axum-http-server-test-environment/ISSUE.md` after the fix; the four template bullets are each answered. `src/bootstrap/jobs/http_tracker.rs` uses `start_with_cancellation` with a cancellation token and a drain timeout, which the section cites.
- Resolution reference: docs(issues): [#2724] define ownership, cleanup and deadlines of the new example
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471489709

### F2 - Correct the recorded contract-test baseline before implementation

- PR number: 2725
- Source review ID: 5773261511
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471316438
- Concern: The spec records 8 integration tests for `axum-health-check-api-server`, but `tests/server/contract.rs` has 11, so AC2 and M3 would compare against a stale baseline.
- Solution: Valid. The first measurement was taken at `15da3a9c3`; develop then gained three readiness tests (#2697). T1 records both measurements (8, then 11 at `4e23c4356`) and T2 remeasures on the implementation base.
- Current-tree verification: `grep -c "#\[tokio::test\]" packages/axum-health-check-api-server/tests/server/contract.rs` printed 11; `cargo test -p torrust-tracker-axum-health-check-api-server` reported 15 unit and 11 integration tests, with and without `--all-features`.
- Resolution reference: docs(issues): [#2724] correct the recorded health check test baseline
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471490045

### F3 - `related-pr` is null and the spec status conflicts with the EPIC row

- PR number: 2725
- Source review ID: 5773741514
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471692593
- Concern: The spec front matter has `related-pr: null` although it is introduced by PR #2725, and `status: planned` conflicts with row 5 of the #2482 EPIC being `IN_PROGRESS`.
- Solution: Valid. `related-pr` is `2725` and `status` is `in-progress`, matching the EPIC row.
- Current-tree verification: `grep -n "^related-pr\|^status" docs/issues/open/2724-2482-gate-axum-http-server-test-environment/ISSUE.md` shows `status: in-progress` and `related-pr: 2725`; the pre-commit frontmatter validator accepted both values.
- Resolution reference: docs(issues): [#2724] set related-pr and align the spec status with the EPIC row
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471736077

### F4 - Duplicate of F3 on the same line

- PR number: 2725
- Source review ID: 5773741514
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471692663
- Concern: The same text as F3, posted a second time on the same line.
- Solution: Superseded by F3; no separate change.
- Current-tree verification: The two comment bodies are identical; the F3 fix covers both.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471736451
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2725#discussion_r4471736451

## Processing Log

- 2026-11-04 18:18 UTC - Started audit; round 1 (Copilot review 5773261511) processed: F1 and F2 fixed in the two commits cited above, pushed, replied and resolved; zero unresolved threads after refreshing GraphQL. Copilot is re-requested after the audit is pushed.
- 2026-11-04 18:42 UTC - Round 2 (Copilot review 5773741514): F3 fixed in the commit cited above, F4 superseded by F3; both replied and resolved after the push; zero unresolved threads after refreshing GraphQL. The maintainer's `ACK 6d0731d13` (comment 6637934790, 2026-11-04 17:54 UTC) is on a head older than the round 1 fixes and this round's fix, so it no longer matches the head.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
