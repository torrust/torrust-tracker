---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2333-2278-fetch-all-review-threads/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2339 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2339>.

PR #2339 merged at 2026-09-25 08:26 UTC with these findings unprocessed; they were submitted at
2026-09-24 21:42 UTC. This record is created on the approved post-merge follow-up branch.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/pull/2339#issuecomment-5844522320>

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

Audit IDs match the reviewer's IDs `F1`-`F3` from review 5310586223. Copilot review 5309860249
reports `Findings: None`; its overview line about coverage of the live query's `line` and
`resolvedBy` fields contains no separate request and is the concern da2ce7 recorded as F1, so it
has no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2339-f1` | Human | Suggestion | testing | ORIGINAL | FOLLOW_UP | OPEN |
| F2 | `review-finding:pr-2339-f2` | Human | Suggestion | testing | ORIGINAL | FOLLOW_UP | OPEN |
| F3 | `review-finding:pr-2339-f3` | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | OPEN |

## Finding Details

### F1 - No test pins that the live query requests `line` and `resolvedBy`

- PR number: 2339
- Source review ID: 5310586223
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4098786012>
- Concern: every projection test reads a static fixture, and `line` and `resolved_by` are `Option`
  fields that serde reads as `None` when missing. Dropping either field from
  `REVIEW_THREADS_QUERY` would pass every test and report `null` at runtime, which looks the same
  as GitHub's legitimate "unknown". The fallback-query parity with the skill (#2333 AC1) held only
  by hand.
- Solution: `test(dev-tools): pin review-thread query fields and explicit nulls` adds
  `it_should_request_the_line_and_resolver_of_each_thread`, which requires `path line resolvedBy
  { login }` as consecutive tokens of the query, and
  `it_should_document_the_same_fallback_query_the_tool_sends`, which compares the query with the
  `fetch-review-threads` skill's fallback block token for token.
- Current-tree verification: on the follow-up branch, `cargo test --package github-review-threads`
  passes 15 unit and 7 CLI tests (stable Rust 1.98.1). In the working tree, deleting the
  thread-level `line` from `REVIEW_THREADS_QUERY` failed both new tests; deleting `line` from the
  skill fallback failed only the parity test; both files were restored before committing.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4111070029>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2344>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4111070029>

### F2 - The explicit-null assertions also pass when the key is absent

- PR number: 2339
- Source review ID: 5310586223
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4098786020>
- Concern: `thread["resolvedBy"]` and `thread["line"]` index a `serde_json::Value`, which returns
  `Null` for a missing key, so the two tests named for explicit nulls would pass if `ShownThread`
  skipped `None` fields. Nothing else pins `ShownThread`'s nulls.
- Solution: the same commit changes both assertions to `thread.get(...)` equal to
  `Some(&Value::Null)`.
- Current-tree verification: adding `#[serde(skip_serializing_if = "Option::is_none")]` to
  `ShownThread`'s `resolved_by` and `line` in the working tree failed
  `it_should_keep_a_missing_resolver_as_an_explicit_null` and
  `it_should_keep_the_missing_line_of_an_outdated_thread_as_an_explicit_null`; the file was
  restored before committing.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4111070186>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2344>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4111070186>

### F3 - Evidence step V3.1 does not record the projection its observed result shows

- PR number: 2339
- Source review ID: 5310586223
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4098786022>
- Concern: V3.1 printed a four-key array in its observed block, but the recorded command prints
  the full `threads` object; V1.2, V2, and V3.2 named `jq` or a redirect without the filter; and
  V1.1 and V3.1 run from a terminal stop with `tty_refusal`.
- Solution: `docs(issues): record the exact #2333 evidence commands` records each V1-V4 step with
  its pipe or redirect and exact `jq` filter, explains the `tty_refusal` reason, and appends a
  dated correction note.
- Current-tree verification: re-running the recorded V1-V4 commands verbatim against PR #2320 at
  2026-09-26 09:35 UTC reproduced every observed block unchanged (29/27/2/21 counts, the same
  resolver, path, and line examples, the two unresolved IDs, `reply-status` exit `1`, and the
  resolver's two dry-run IDs with exit `0`).
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4111070337>
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2344>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2339#discussion_r4111070337>

## Processing Log

- 2026-09-26 10:19 UTC - Started audit. Read-only triage on 2026-09-25 found three da2ce7 findings
  from review 5310586223 with no reply, unresolved, and live on `develop` at `a20e8f3a`, plus Copilot
  review 5309860249 with no independent finding. The maintainer approved a follow-up; the
  approval is recorded on the PR (Ownership section).
- 2026-09-26 10:19 UTC - The fixes are committed on the follow-up branch and opened as PR #2344.
  Replied on all three threads with the follow-up disposition and fixing commit; the threads are
  not resolved, because the fixes are not yet on `develop`.

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
