---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2370"
---

<!-- skill-link: process-pr-review -->

# PR #2382 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2382>.

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

Audit IDs `F1`-`F4` are the reviewer's IDs from Copilot review 5363659930 (round 1, submitted
2026-09-30 08:34 UTC at the head `docs(udp-server): [#2370] cite branch commits by subject`). No
earlier finding exists, so no ID collides. The review body is a summary with no independently
actionable assertion beyond the four inline threads.

Severity records the inline `[Severity]` bracket. The review overview's badge markup uses a
different scale: `alt="Medium severity"` for F3 and `alt="Low severity"` for F1, F2, and F4.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2382-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2382-f2` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2382-f3` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2382-f4` | Copilot | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ring capacity was documented as a simultaneous-request limit

- PR number: 2382
- Source review ID: 5363659930
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4142536018>
- Concern: `ACTIVE_REQUESTS_CAPACITY` was documented as "the number of UDP requests handled
  simultaneously". It limits only the abort handles the ring keeps. The new ADR's Known
  Trade-offs say that `force_push` can drop live handles, after which more than 50 processors
  can run.
- Solution: `docs(udp-server): [#2370] describe the request ring capacity, not simultaneous
  requests` now describes the constant as the number of processor abort handles kept for
  overload eviction, and says it is not an exact limit on running processors. The
  `ActiveRequests` struct doc made the same false claim ("at most 50 requests are handled
  concurrently"), so the same commit corrects it. The ring's behavior is unchanged (D8).
- Current-tree verification: `handled simultaneously|at most 50|handled concurrently` has no
  match in the workspace. `cargo test -p torrust-tracker-udp-server` and `linter clippy` pass.
  Warnings-denied `cargo +nightly doc` of the package, including private items, passes.
- Resolution reference: `docs(udp-server): [#2370] describe the request ring capacity, not simultaneous requests`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4143204076>

### F2 - The ADR claimed the approximate ring protects memory

- PR number: 2382
- Source review ID: 5363659930
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4142535925>
- Concern: "The fixed bound protects memory" contradicts the ADR's own Known Trade-offs. Live
  handles that the ring drops leave their processors running outside it.
- Solution: `docs(udp-server): [#2370] state that the request ring bounds bookkeeping, not
  memory` now says the ring bounds overload-control bookkeeping (the abort handles it keeps), and
  that it does not bound processor concurrency or task memory exactly. The sentence links to
  Known Trade-offs. The matching re-evaluation trigger now calls the bound approximate instead of
  implying that it protects the deployment. The ADR's decision is unchanged.
- Current-tree verification: `protects memory` has no match in the workspace. `linter markdown`,
  `linter cspell`, and `linter lychee` pass.
- Resolution reference: `docs(udp-server): [#2370] state that the request ring bounds bookkeeping, not memory`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4143204319>

### F3 - The drain deadline warning did not identify the listener

- PR number: 2382
- Source review ID: 5363659930
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4142535856>
- Concern: Every configured UDP binding drains at the same time on shutdown. The deadline
  warning in `drain_request_processors` had no listener field, and no enclosing span identifies
  the listener. Operators could not tell which binding reached its deadline.
- Solution: `fix(udp-server): [#2370] name the listener in the request drain deadline warning`
  passes the service binding into `drain_request_processors` and logs it as `service_binding`.
  The logging stays `tracing`-only (D4) and runs only on the shutdown path. The deadline is still
  the private constant (D1). New test: `it_should_name_the_listener_in_the_deadline_warning`. It
  uses paused Tokio time and a documentation-range binding that no other test logs, and reads the
  warning from the test-helpers log buffer. No production hook is added (D9). This package's unit
  tests install only that global subscriber, because the ephemeral test configuration turns off
  the configuration crate's logging setup.
- Current-tree verification: the test passes 5 of 5 isolated runs. Two hand mutations each made
  it fail and were then reverted by hand: removing the field, and logging the value as
  `local_addr`. `cargo test -p torrust-tracker-udp-server` passes (209 tests on the rebased
  branch), and so do `linter clippy` and warnings-denied `cargo +nightly doc` of the package.
- Resolution reference: `fix(udp-server): [#2370] name the listener in the request drain deadline warning`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4143204560>

### F4 - The new drain logs used local_addr instead of the canonical service_binding field

- PR number: 2382
- Source review ID: 5363659930
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4142535972>
- Concern: The new drain start and summary logs (lines 145 and 148) logged the binding URL as
  `local_addr`. The structured runtime logging convention names listener identity
  `service_binding`.
- Solution: `refactor(udp-server): [#2370] log request drain listener identity as
  service_binding` renames the drain helpers' parameter, so their shorthand field is now
  `service_binding`. The value is unchanged: `ServiceBinding`'s `Display` output is its URL. The
  receive-loop logs that already existed keep `local_addr`, because issue #2370's implementation
  constraints require existing receive-loop log messages to stay unchanged. The issue's manual
  verification evidence now notes that its captured drain logs predate the rename.
- Current-tree verification: the structured-runtime-logging skill names `service_binding` as the
  canonical listener field. A debug tracker stopped with `SIGTERM` logged
  `UDP request processors drained service_binding="udp://0.0.0.0:3000" completed=0 failed=0 aborted=0 evicted=0`
  and exited with code 0.
- Resolution reference: `refactor(udp-server): [#2370] log request drain listener identity as service_binding`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2382#discussion_r4143204866>

## Processing Log

- 2026-09-30 08:58 UTC - Fetched review 5363659930 and its 4 inline threads (all unresolved,
  none outdated). Read the severity badges in the raw review body.
- 2026-09-30 09:00 UTC - Committed the F1 fix.
- 2026-09-30 09:03 UTC - Committed the F2 fix.
- 2026-09-30 09:13 UTC - Committed the F4 fix.
- 2026-09-30 09:30 UTC - Committed the F3 fix and its regression test, proven by hand mutation.
- 2026-09-30 09:40 UTC - The branch was 57 commits behind `torrust/develop`. Rebased it with
  signed commits. Upstream had changed a `processor.rs` test assertion to an exact-context
  `assert_eq!` and added a response-sent test, which conflicted with
  `feat(udp-server): return a Result from UDP request processors`. The resolution keeps
  upstream's assertion and adds that commit's `Ok` result check, including in the new test.
  Tests, clippy, and all linters pass on the rebased branch.
- 2026-09-30 09:44 UTC - Force-pushed with lease after pre-push checks passed. Replied on all 4
  threads.
- 2026-09-30 09:51 UTC - `reply-status` reported 4 of 4 threads replied. Resolved all 4. A
  GraphQL refresh shows 4 of 4 threads resolved and none unresolved.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push. For a
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
