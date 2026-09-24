---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2324-1488-si-13-migrate-health-check-api-token-lifecycle/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2336 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2336>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2336-f1` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2336-f2` | Copilot | Nit (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Stop log uses the configured address instead of the bound address

- PR number: 2336
- Source review ID: 5307922912
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2336#discussion_r4096551508>
- Concern: the health-check component logged `Stopped server running on:` with the configured
  `bind_addr`, which is wrong when the listener binds an ephemeral port. Severity is inferred from
  Copilot's "Medium" overview marker.
- Solution: capture `server.local_addr` returned by `start_with_cancellation` before building
  `TokenAwareServerTask` and log it on stop.
- Current-tree verification: `start_job` in `src/bootstrap/jobs/health_check_api.rs` logs
  `local_addr` and no longer references the configured address after startup;
  `cargo test -p torrust-tracker --lib -- health_check_api` passes (5 tests) and clippy with
  `-D warnings` is clean on nightly Rust `1.100.0-nightly`.
- Resolution reference: `fix(bootstrap): [#2324] log the bound health-check address on stop`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2336#discussion_r4097669158>

### F2 - Listener-release test retry loop busy-waits

- PR number: 2336
- Source review ID: 5307922912
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2336#discussion_r4096551621>
- Concern: `wait_until_bindable` retried the bind with only `yield_now` between attempts, spinning
  the CPU while waiting. Severity is inferred from Copilot's "Low" overview marker.
- Solution: sleep a 10 ms `BIND_RETRY_INTERVAL` between attempts; the 5-second overall bound is
  unchanged.
- Current-tree verification: the `health_check_api` test module uses
  `tokio::time::sleep(BIND_RETRY_INTERVAL)` in `wait_until_bindable`;
  `cargo test -p torrust-tracker --lib -- health_check_api` passes (5 tests).
- Resolution reference: `test(bootstrap): [#2324] pace the health-check listener release retry`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2336#discussion_r4097669370>

## Processing Log

- 2026-09-24 17:33 UTC - Copilot review 5307922912 submitted (F1, F2). The review body is an
  overview with no independent request and has no row.
- 2026-09-24 19:28 UTC - Audit started; F1 and F2 normalized before the fixes.
- 2026-09-24 19:31 UTC - `fix(bootstrap): [#2324] log the bound health-check address on stop`
  authored (F1).
- 2026-09-24 19:32 UTC - `test(bootstrap): [#2324] pace the health-check listener release retry`
  authored (F2).
- 2026-09-24 19:35 UTC - Replies posted on F1 and F2.
- 2026-09-24 19:41 UTC - F1 and F2 resolved after audit validation; a refreshed GraphQL fetch
  reports no unresolved thread.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
