---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2740 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2740.

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

All three findings come from review 5789752911 by Copilot.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2740-f1 | Copilot | Major (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2740-f2 | Copilot | Major (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2740-f3 | Copilot | Minor (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Test server thread loops forever

- PR number: 2740
- Source review ID: 5789752911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2740#discussion_r4484215220
- Concern: The test TCP server loops on `listener.incoming()`, keeping a listener thread until process exit.
- Solution: `serve_status` now accepts one connection, answers and returns, in both server.rs files.
- Current-tree verification: Inspected `serve_status` in both files; `cargo test -p torrust-tracker-axum-rest-api-server -p torrust-tracker-axum-http-server health_endpoint` passed (2 per package).
- Resolution reference: test(axum-http-server): [#2733] serve one connection per health check test server
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2740#discussion_r4485099697

### F2 - Health check tests have no timeout

- PR number: 2740
- Source review ID: 5789752911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2740#discussion_r4484215286
- Concern: The tests await the health check job with no timeout, so a hang would stall the suite.
- Solution: `run_health_check` builds the client with a 5 s timeout and wraps the job in a 10 s `tokio::time::timeout`, in both files.
- Current-tree verification: Inspected both files; the same test command passed (2 per package).
- Resolution reference: test(axum-http-server): [#2733] bound the health check tests with timeouts
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2740#discussion_r4485100250

### F3 - Duplicated test helpers across the two crates

- PR number: 2740
- Source review ID: 5789752911
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2740#discussion_r4484215328
- Concern: `serve_status` and `run_health_check` are nearly identical in both crates; extract a shared helper.
- Solution: Declined: the spec keeps each server owning its check with no new dependency; sharing would need a new dev-only crate or a dependency between the two server crates for about 30 lines.
- Current-tree verification: Inspected both files: the helpers are small and independent; no shared test-support crate exists for these two packages.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2740#discussion_r4485100695
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2740#discussion_r4485100695

## Processing Log

- 2026-11-05 19:34 UTC - Started audit. Fixes F1 and F2 pushed, replies posted and threads resolved before this entry.

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
