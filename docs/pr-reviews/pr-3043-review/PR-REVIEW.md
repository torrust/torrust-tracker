---
pr-number: 3043
pr-url: https://github.com/torrust/torrust-tracker/pull/3043
last-updated-utc: "2026-11-24 18:31"
---

<!-- skill-link: process-pr-review -->

# PR #3043 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/3043.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (the findings were fixed before the merge).

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

Round 1, on the head `docs(issues): [#3031] record the manual verification evidence and complete the specification`: Greptile review 6044956173 (two threads, P2 badges) and CodeRabbit review 6044982742 (one thread). All three were fixed in one commit.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3043-f1 | Unknown | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3043-f2 | Unknown | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3043-f3 | Unknown | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The log capture helper writes `std::sync::Arc` in full

- PR number: 3043
- Source review ID: 6044956173
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684595423
- Concern: `packages/test-helpers/src/logging.rs` used `std::sync::Arc` three times without a name conflict, against the import rule of `AGENTS.md`.
- Solution: `Arc` is imported next to `Mutex` and used by its short name.
- Current-tree verification: `grep -n "std::sync::Arc" packages/test-helpers/src/logging.rs` prints nothing; `cargo clippy -p torrust-tracker-test-helpers --all-targets` printed nothing.
- Resolution reference: fix(statistics): [#3031] address review round 1 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684667777

### F2 - The tracker-level test ignores the exit status

- PR number: 3043
- Source review ID: 6044956173
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684595444
- Concern: `wait_for_exit()` returns `Ok` for a non-zero exit or a signal death, so a tracker that crashed before its listeners finished could pass the test with incomplete logs.
- Solution: The test asserts `exit.status.success()` before it reads the logs and prints the tracker output when it fails.
- Current-tree verification: `cargo test --test metrics-statistics-disabled-logs` passed (27 tests, the new one included) after the change.
- Resolution reference: fix(statistics): [#3031] address review round 1 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684668271

### F3 - The evidence file names a spec path that goes stale

- PR number: 3043
- Source review ID: 6044982742
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684617214
- Concern: `manual-verification-evidence.md` had `issue-spec: <path under docs/issues/open/>`, which is wrong once the specification moves to `closed/`; `write-markdown-docs` asks for `issue: #<number>` after the GitHub issue exists.
- Solution: The frontmatter has `issue: 3031`.
- Current-tree verification: `sed -n 1,5p docs/issues/open/3031-2387-si-1-statistics-disabled-warning-per-event/manual-verification-evidence.md` shows `issue: 3031`; the frontmatter check of `./contrib/dev-tools/git/hooks/pre-commit.sh` passed.
- Resolution reference: fix(statistics): [#3031] address review round 1 of the statistics-disabled fix
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/3043#discussion_r4684668708

## Processing Log

- 2026-11-24 18:21 UTC - Round 1: Greptile review 6044956173 (18:21:43) with two threads.
- 2026-11-24 18:23 UTC - Round 1: CodeRabbit review 6044982742 (18:23:35) with one thread.
- 2026-11-24 18:24 UTC - Committed the three fixes in one commit (committer time 18:24:53), then pushed it.
- 2026-11-24 18:28 UTC - Replied on the three threads after the push (18:27:58 to 18:28:03) and resolved them.
- 2026-11-24 18:29 UTC - CodeRabbit comment 4684687075 (review 6045065462, 18:29:41) confirms F3 is fixed and says it does not resolve the thread before this audit validation; it holds no new request, so it has no row.
- 2026-11-24 18:31 UTC - Created this audit record after the replies and the resolutions (the first reply was posted before the record existed); the validation script ran after this entry.

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
