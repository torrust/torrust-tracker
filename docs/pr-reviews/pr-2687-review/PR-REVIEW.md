---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2687 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2687>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`. `OPEN` applies prospectively
  to audits created or updated for approved follow-up work; historical audits remain valid without
  bulk migration.
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Round 1 is the Copilot review `5686737218` (submitted 2026-10-28 17:13 UTC). Neither inline comment carries a
`[Severity][Id]` prefix, so the finding IDs are audit-local and the severities are inferred.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2687-f1 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2687-f2 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Bound the response bodies the test helpers read

- PR number: 2687
- Source review ID: 5686737218
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2687#discussion_r4403739901
- Concern: A test helper read the response body with `usize::MAX` as the limit, which removes any bound on the buffered size, and the reviewer asked to apply the same change to the other helpers that do so.
- Solution: Five helpers read bodies with `usize::MAX`: three in this pull request (`routes.rs`, `v1/routes.rs`, the health check test) and two in the already merged S1 files (`v1/middlewares/auth.rs`, `v1/responses.rs`). One `MAX_BODY_BYTES` (1 MiB) now lives in the crate-private `test_support` module and all five use it, with an expect message that names the limit.
- Current-tree verification: `grep -rn 'usize::MAX' packages/axum-rest-api-server/src` finds nothing after the change. With the limit temporarily set to 1 byte, 56 unit tests failed with `LengthLimitError`; the restored value passed the 76 unit and 58 integration tests.
- Resolution reference: test(axum-rest-api-server): [#2637] bound the response bodies the tests read
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2687#discussion_r4403849003

### F2 - Select the first tracker without indexing

- PR number: 2687
- Source review ID: 5686737218
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2687#discussion_r4403739958
- Concern: `api_container` indexed the first HTTP and UDP tracker with `[0]`, which panics out of bounds with a misleading message when the list is empty, and the reviewer asked for `.first()` with an explicit `expect` and no clone of the whole list.
- Solution: The helper selects the tracker with `.as_ref().and_then(|trackers| trackers.first()).expect(...)` and clones only that element, as its `# Panics` documentation says.
- Current-tree verification: Read `packages/axum-rest-api-server/src/test_support.rs` after the change; the 76 unit tests that build the container pass.
- Resolution reference: test(axum-rest-api-server): [#2637] select the first tracker without indexing in the test container helper
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2687#discussion_r4403849394

## Processing Log

- 2026-10-28 17:13 UTC - Copilot review `5686737218` submitted with two inline findings. No human review existed at this point.
- 2026-10-28 17:23 UTC - Started audit; fixed F2 and F1 in one signed commit each and pushed the branch (the commit subjects, not ids, are cited above), replied to both threads (17:23:46 and 17:23:48 UTC) and resolved them; each resolve mutation returned `isResolved: true`.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
