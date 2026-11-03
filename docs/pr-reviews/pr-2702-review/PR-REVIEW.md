---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2702 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2702>.

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

Round 1 is the Copilot review `5756948545` (submitted 2026-11-03 17:45 UTC) with five inline comments on two concerns:
the string-based call log of the port fakes (F1-F3, one per call-log site of the auth key fake) and the duplicated
response-reading helper (F4-F5, the two sites of the whitelist handlers). No comment carries a `[Severity][Id]` prefix,
so the finding IDs are audit-local and the severities are inferred. The maintainer decided on 2026-11-03 to fix both
concerns in this pull request (the response helper by extracting it, not deferring it).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2702-f1 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2702-f2 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2702-f3 | Copilot | Suggestion (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2702-f4 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2702-f5 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Record typed calls in the port fake

- PR number: 2702
- Source review ID: 5756948545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458391784
- Concern: The fake port recorded each call as a formatted string (auth_key/handlers.rs line 180), so the forwarding assertions depended on `Debug` and `Display` formatting; the reviewer suggested a typed `Call` enum.
- Solution: Both port fakes (auth key and whitelist) now record a test-local `Call` enum whose variants carry the forwarded inputs, and the forwarding assertions compare typed values.
- Current-tree verification: The forwarding mutations still fail: the fake that records nothing fails three tests in each fake, altering the generated seconds, the delete key, the whitelist add hash and the whitelist remove call each fail the forwarding test; the 106 unit and 58 integration tests pass.
- Resolution reference: test(axum-rest-api-server): [#2637] record typed calls in the port fakes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458612834

### F2 - Record typed calls in the port fake

- PR number: 2702
- Source review ID: 5756948545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458391853
- Concern: The fake port recorded each call as a formatted string (auth_key/handlers.rs line 281), so the forwarding assertions depended on `Debug` and `Display` formatting; the reviewer suggested a typed `Call` enum.
- Solution: Both port fakes (auth key and whitelist) now record a test-local `Call` enum whose variants carry the forwarded inputs, and the forwarding assertions compare typed values.
- Current-tree verification: The forwarding mutations still fail: the fake that records nothing fails three tests in each fake, altering the generated seconds, the delete key, the whitelist add hash and the whitelist remove call each fail the forwarding test; the 106 unit and 58 integration tests pass.
- Resolution reference: test(axum-rest-api-server): [#2637] record typed calls in the port fakes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458613225

### F3 - Record typed calls in the port fake

- PR number: 2702
- Source review ID: 5756948545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458391915
- Concern: The fake port recorded each call as a formatted string (auth_key/handlers.rs line 300), so the forwarding assertions depended on `Debug` and `Display` formatting; the reviewer suggested a typed `Call` enum.
- Solution: Both port fakes (auth key and whitelist) now record a test-local `Call` enum whose variants carry the forwarded inputs, and the forwarding assertions compare typed values.
- Current-tree verification: The forwarding mutations still fail: the fake that records nothing fails three tests in each fake, altering the generated seconds, the delete key, the whitelist add hash and the whitelist remove call each fail the forwarding test; the 106 unit and 58 integration tests pass.
- Resolution reference: test(axum-rest-api-server): [#2637] record typed calls in the port fakes
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458613714

### F4 - Share the response-reading test helper

- PR number: 2702
- Source review ID: 5756948545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458391996
- Concern: The `Observed`/`observed` response-reading helper (whitelist/handlers.rs line 128) repeats across the test modules and would grow in S4 and S5; the reviewer suggested one shared helper in `crate::test_support`.
- Solution: `test_support` holds `read_body`, `Observed` and `observed`, and the seven test modules that had their own copy use them; the `routes.rs` header checks keep a small struct built on the shared `read_body`.
- Current-tree verification: `grep -rn "MAX_BODY_BYTES\|to_bytes" packages/axum-rest-api-server/src` finds only `test_support.rs`; mutating the shared `observed` (status, content type or body) fails 48 to 62 unit tests; the 106 unit and 58 integration tests pass.
- Resolution reference: test(axum-rest-api-server): [#2637] share the response-reading test helper
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458614065

### F5 - Share the response-reading test helper

- PR number: 2702
- Source review ID: 5756948545
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458392046
- Concern: The `Observed`/`observed` response-reading helper (whitelist/handlers.rs line 184) repeats across the test modules and would grow in S4 and S5; the reviewer suggested one shared helper in `crate::test_support`.
- Solution: `test_support` holds `read_body`, `Observed` and `observed`, and the seven test modules that had their own copy use them; the `routes.rs` header checks keep a small struct built on the shared `read_body`.
- Current-tree verification: `grep -rn "MAX_BODY_BYTES\|to_bytes" packages/axum-rest-api-server/src` finds only `test_support.rs`; mutating the shared `observed` (status, content type or body) fails 48 to 62 unit tests; the 106 unit and 58 integration tests pass.
- Resolution reference: test(axum-rest-api-server): [#2637] share the response-reading test helper
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2702#discussion_r4458614364

## Processing Log

- 2026-11-03 17:45 UTC - Copilot review `5756948545` submitted with five inline findings. No human review existed at this point.
- 2026-11-03 17:52 UTC - Merged `develop` into the branch to pick up the fix of the unrelated `clippy::unnecessary_lazy_evaluations` failure in `http-protocol` that failed the testing workflow, and pushed.
- 2026-11-03 18:06 UTC - The maintainer decided to fix both concerns in this pull request. Fixed F4-F5 and F1-F3 in one signed commit each, pushed the branch (the commit subjects, not ids, are cited above), replied to the five threads (18:06:19 to 18:06:30 UTC) and resolved them at 18:06:39 UTC; each resolve mutation returned `isResolved: true` and a refreshed fetch lists no unresolved thread.

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
