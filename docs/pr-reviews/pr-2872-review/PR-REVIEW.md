---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2872 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2872.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Round 1 is Copilot's review `5916561572` (commented, "Changes recommended") at the head whose last
commit is `docs(issues): [#2850] point #2838 and the EPIC #1347 table to PR #2872`. Its two inline
findings keep the reviewer's IDs F1 and F2; the review body only summarizes them.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2872-f1 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2872-f2 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The domain `NumberOfBytes` has no compile-fail test of its construction boundary

- PR number: 2872
- Source review ID: 5916561572
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583142394
- Concern: the positive doctest and the UDP rejection tests would still pass if the field of
  `torrust_tracker_primitives::NumberOfBytes` became public or a signed constructor were added, so
  nothing pins AC1 for this type. Add compile-fail doctests for a signed argument and for the
  tuple constructor, next to the existing successful example.
- Solution: added two doctests to `packages/primitives/src/number_of_bytes.rs`:
  `compile_fail,E0308` passes an `i64` to `NumberOfBytes::new`, and `compile_fail,E0423` builds
  the value with the tuple constructor from outside the crate. The existing successful example
  stays first.
- Current-tree verification: `cargo test --doc -p torrust-tracker-primitives number_of_bytes`
  ran 3 doctests, all passed. With the field changed to `pub u64` for a moment, the same command
  reported the tuple-constructor doctest as `FAILED`; the file was then restored.
- Resolution reference: test: [#2850] pin that a number of bytes cannot be built from a signed integer
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583186952

### F2 - The HTTP `NumberOfBytes` has no compile-fail test of its construction boundary

- PR number: 2872
- Source review ID: 5916561572
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583142335
- Concern: the new HTTP range tests exercise query parsing inside the module, so they cannot detect
  a tuple constructor exposed to other crates, and they do not pin the refusal of a signed
  constructor argument. Add compile-fail doctests for both, plus a successful unsigned
  construction example.
- Solution: added a doc comment to `NumberOfBytes` in
  `packages/http-protocol/src/v1/requests/announce.rs` with a successful construction doctest, a
  `compile_fail,E0308` doctest that passes an `i64` to `new`, and a `compile_fail,E0423` doctest
  that uses the tuple constructor from outside the crate.
- Current-tree verification: `cargo test --doc -p torrust-tracker-http-protocol NumberOfBytes` ran
  3 doctests, all passed; the pre-commit hook's `cargo test --doc --workspace` passed.
- Resolution reference: test: [#2850] pin that a number of bytes cannot be built from a signed integer
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2872#discussion_r4583187197

## Processing Log

- 2026-11-16 07:51 UTC - Started this audit for Copilot's review `5916561572` (submitted
  2026-11-16T07:47:32Z), before any reply. The fix for F1 and F2 is one commit, because the two
  findings ask for the same doctests on the two types.
- 2026-11-16 07:54 UTC - Round 1 processed. The fix was pushed by `git-bot-push` (head
  `test: [#2850] pin that a number of bytes cannot be built from a signed integer`); each of the
  two threads has a reply (posted 2026-11-16T07:54 UTC) and was resolved (`isResolved: true`); a
  fresh GraphQL fetch shows 0 unresolved threads. `validate-audit-record.py --pr-number 2872`
  returned `status: ok`, 2 rows, 0 failures, before the resolution.

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
