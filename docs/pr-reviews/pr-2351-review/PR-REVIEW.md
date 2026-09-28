---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2342-1488-si-14-migrate-udp-receive-reset-token-lifecycle/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2351 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2351>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2351-f1` | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2351-f2` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2351-f3` | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Legacy stop logs omit the UDP tracker log target

- PR number: 2351
- Source review ID: 5327633174
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2351#discussion_r4112947449>
- Concern: the global-signal, halt, and "Halting UDP Service" log lines in the legacy stop
  helpers had no `target`, unlike the rest of the UDP server logs, which can hide shutdown events
  from target-based log routing. Severity is inferred from Copilot's "Medium" overview marker.
- Solution: log all three lines with `target: UDP_TRACKER_LOG_TARGET`.
- Current-tree verification: `legacy_stop_requested` and `halt_requested` in
  `packages/udp-server/src/server/launcher.rs` pass `UDP_TRACKER_LOG_TARGET` on every log call;
  the two legacy launcher tests pass and clippy with `-D warnings` is clean on nightly Rust
  `1.100.0-nightly`.
- Resolution reference: `fix(udp-server): [#2342] log legacy stop events under the UDP tracker target`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2351#discussion_r4114597928>

### F2 - OwnedTask allows joining a task more than once

- PR number: 2351
- Source review ID: 5327633174
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2351#discussion_r4112947472>
- Concern: `OwnedTask::join(&mut self)` kept the handle, so a caller could await it again after
  completion; tokio 1.53.1 panics with "JoinHandle polled after completion"
  (`src/runtime/task/core.rs:427`). Severity is inferred from Copilot's "Medium" overview marker.
- Solution: `join` now consumes `self`. The handle stays inside the owner, which lives inside the
  join future, so dropping the component while it waits still aborts the task. Moving the handle
  out of the owner, as also suggested, would have detached the task on that drop path.
- Current-tree verification: `OwnedTask::join(mut self)` in `src/bootstrap/jobs/manager.rs`;
  `supervise_receive_loop` takes the owner by value. The new test
  `it_should_abort_the_receive_loop_when_the_component_is_dropped_while_waiting_for_it` passes
  and fails when the owner's abort-on-drop is removed; `cargo test -p torrust-tracker --lib -- udp`
  passes (12 tests).
- Resolution reference: `fix(bootstrap): [#2342] make OwnedTask::join consume its owner`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2351#discussion_r4114598006>

### F3 - Receive loop rebuilds ServiceBinding for every datagram

- PR number: 2351
- Source review ID: 5327633174
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2351#discussion_r4112947483>
- Concern: `ServiceBinding::new` is called on every loop iteration although the socket address is
  constant. Severity is inferred from Copilot's "Low" overview marker.
- Solution: no change in this PR, agreed with the maintainer. The code predates SI-14, whose spec
  excludes the request-loop refactor, and the hoist is already task T1 of the existing
  `simplify-udp-server-main-loop` draft. Keeping it out also keeps the SI-14 before/after
  throughput measurement comparable.
- Current-tree verification: `docs/issues/drafts/simplify-udp-server-main-loop/ISSUE.md` lists
  T1 "Hoist per-iteration `ServiceBinding` recreation"; `run_udp_server_main` is otherwise
  unchanged apart from the SI-14 stop condition and error mapping.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2351#discussion_r4114598064>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2351#discussion_r4114598064>

## Processing Log

- 2026-09-26 22:00 UTC - Copilot review 5327633174 submitted (F1, F2, F3). The review body is an
  overview with no independent request and has no row.
- 2026-09-27 06:15 UTC - Audit started; F1-F3 normalized before any fix. The maintainer chose to
  defer F3 to the `simplify-udp-server-main-loop` draft.
- 2026-09-27 06:21 UTC - `fix(udp-server): [#2342] log legacy stop events under the UDP tracker target`
  authored (F1).
- 2026-09-27 06:29 UTC - `fix(bootstrap): [#2342] make OwnedTask::join consume its owner`
  authored (F2).
- 2026-09-27 08:07 UTC - Replies posted on F1, F2, and F3.
- 2026-09-27 08:10 UTC - F1-F3 resolved after audit validation; a refreshed GraphQL fetch shows
  all three threads resolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
