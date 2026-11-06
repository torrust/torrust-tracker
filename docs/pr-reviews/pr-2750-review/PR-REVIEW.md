---
pr-number: 2750
pr-url: https://github.com/torrust/torrust-tracker/pull/2750
last-updated-utc: "2026-11-06 16:50"
---

# PR #2750 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2750.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: https://github.com/torrust/torrust-tracker/pull/2750#issuecomment-6689896342 (comment created 2026-11-06 16:30:47 UTC, approving the follow-up PR for round 2)

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

Round 1: Copilot review 5801009506 on the head `e1b8c40c8` (one inline finding). The finding is valid and fixed.

Round 2: Copilot review 5802998503 on the head `36cc80c49` (three inline findings), submitted 2026-11-06 16:00:51 UTC, 67 s before the merge at 16:01:58 UTC, so it was unprocessed at merge. All three are valid on the merged tree and are fixed in the follow-up PR https://github.com/torrust/torrust-tracker/pull/2756, which is open, so they are `FOLLOW_UP` until it merges.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2750-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2750-f2 | Copilot | Minor (inferred) | testing | ORIGINAL | FOLLOW_UP | OPEN |
| F3 | review-finding:pr-2750-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FOLLOW_UP | OPEN |
| F4 | review-finding:pr-2750-f4 | Copilot | Suggestion (inferred) | maintainability | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - The skill text implies an interruption becomes a timeout

- PR number: 2750
- Source review ID: 5801009506
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4493132447
- Concern: In `wait-for-pr-review/SKILL.md`, "If that happens" covered both budget expiry and signals, so the following sentences implied that an interruption becomes `timeout` or `deadline_exceeded`. In `wait.rs` only `Error::DeadlineExceeded` becomes a timeout; an interruption stays a failure.
- Solution: Valid. The text now says a signal always ends the run with an `interrupted` failure (exit `1`) and restricts the `timeout` versus `deadline_exceeded` explanation to budget expiry.
- Current-tree verification: `tool/src/wait.rs` line 141 maps only `Err(Error::DeadlineExceeded)` to `timeout()`; the edited paragraph in `SKILL.md` matches it.
- Resolution reference: docs(wait-for-pr-review): [#2712] limit the timeout result to budget expiry in the skill
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4493174889

### F2 - The fake gh descendant may be ended by SIGHUP

- PR number: 2750
- Source review ID: 5802998503
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4494558017
- Concern: The `sleep` that holds the pipes open in the fake `gh` of `tests/cli.rs` could be ended by SIGHUP in some shells, making the held-pipe condition unreliable.
- Solution: Valid. The descendant now starts as `(trap '' HUP; sleep 20 &)`.
- Current-tree verification: On develop at 35ec81000 the line reads `(sleep 20 &)`, so the concern was live; on the follow-up branch it reads `(trap '' HUP; sleep 20 &)` and `cargo test -p wait-for-pr-review` passes.
- Resolution reference: test(wait-for-pr-review): [#2712] make the fake gh descendant ignore SIGHUP
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2756
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4494962231

### F3 - `Low` is not a declared severity value

- PR number: 2750
- Source review ID: 5802998503
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4494558108
- Concern: The findings table of this audit used `Low`, which is not in the declared severity values.
- Solution: Valid. F1 now uses `Minor (inferred)`; the new rows use declared values only.
- Current-tree verification: `grep -n "| Copilot | Low" docs/pr-reviews/pr-2750-review/PR-REVIEW.md` finds nothing on the follow-up branch; the validation script reports 0 failures.
- Resolution reference: docs(pr-reviews): [#2712] use a declared severity value in the PR 2750 audit
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2756
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4494962828

### F4 - A temporary AtomicBool is passed by reference

- PR number: 2750
- Source review ID: 5802998503
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4494558186
- Concern: The reclaim path of `GhCli::run` passed `&AtomicBool::new(false)` twice, a reference to a temporary that is easy to misread.
- Solution: Valid, optional. One local `never_interrupted` is bound and passed to both calls.
- Current-tree verification: On develop at 35ec81000 `gh.rs` passes two temporaries; on the follow-up branch it binds `never_interrupted` once; `linter all` and the tests pass.
- Resolution reference: refactor(wait-for-pr-review): [#2712] bind the never-set interrupt flag to a local
- Follow-up PR URL: https://github.com/torrust/torrust-tracker/pull/2756
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2750#discussion_r4494963514

## Processing Log

- 2026-11-06 14:10 UTC - Started audit; round 1 (Copilot review 5801009506) processed: F1 fixed in the commit cited above, pushed, replied and resolved.
- 2026-11-06 16:50 UTC - Round 2 (Copilot review 5802998503, submitted before the merge, processed after it): maintainer approval comment recorded in Ownership; F2 to F4 fixed in the three commits cited above on the follow-up PR https://github.com/torrust/torrust-tracker/pull/2756, replied on the merged PR's threads; threads stay open and the findings stay `FOLLOW_UP` until the follow-up PR merges.

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
