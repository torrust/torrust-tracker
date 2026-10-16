---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2413-2410-si-22-1-document-event-flows-and-draft-adr/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2553 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2553>.

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

Copilot's review 5546625913 carries two inline threads, F1 and F2. Its overview is a summary of
those two findings and adds no row. The inline comments have no `[Severity]` bracket, so their
severity is inferred as `Minor` from the overview's "Low severity" badge on each finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2553-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2553-f2 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The glossary promises that server drains also preserve running handlers

- PR number: 2553
- Source review ID: 5546625913
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293735143>
- Concern: the glossary's Drain entry said a drain does not interrupt a running handler, but the UDP
  server aborts unfinished request processors at its drain deadline and the HTTP server forces
  shutdown at its drain timeout. The guarantee holds only for the planned listener drain.
- Solution: the reviewer was right. The Drain and Drain timeout entries now separate a server drain
  deadline, which is a hard limit, from the planned listener admission timeout, which is checked
  only between events. They keep the shared-deadline abort exception.
- Current-tree verification: read from source, not run. `packages/udp-server/src/server/launcher.rs`
  aborts remaining processors with `tasks.abort_all()` after the drain deadline, and
  `packages/axum-server/src/signals.rs` calls `handle.shutdown()` when its drain timer fires.
  `docs/architecture/glossary.md` now has a "server drain deadline" and a "listener admission
  timeout" and no longer says every drain avoids interrupting a handler.
- Resolution reference: docs(architecture): [#2413] distinguish server drain deadlines from listener admission timeouts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293792829>

### F2 - The producer inventory omits the UDP receive loop

- PR number: 2553
- Source review ID: 5546625913
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293735240>
- Concern: `packages/udp-server/src/server/launcher.rs` also publishes `UdpRequestReceived`,
  `UdpRequestAborted`, `UdpRequestDiscarded` and `UdpRequestBanned` through an awaited helper, in
  the receive loop and not in the processor tasks, so the inventory omitted a producer execution
  path.
- Solution: the reviewer was right, and the miss was in the T1 method: it searched for `.send(`
  call sites, and the launcher reaches `send` through `publish_event_if_sender_available`. The
  UDP-server row now lists the receive loop and its four events, the producer send path section
  explains why a `.send(` search misses them and names both UDP execution paths, and a dated
  correction in the sub-EPIC `manual-verification-evidence.md` replaces the earlier "no unlisted
  producer" result. A re-sweep of every file that holds a bus sender found the launcher to be the
  only missing one.
- Current-tree verification: read from source, not run.
  `rg -n 'UdpRequestReceived|UdpRequestAborted|UdpRequestDiscarded|UdpRequestBanned' packages/udp-server/src/server/launcher.rs`
  shows the four publish sites at lines 385, 441, 472 and 499, all through the awaited helper at
  line 582. The inventory row and send-path section name them.
- Resolution reference: docs(shutdown): [#2413] add the UDP receive loop to the event producer inventory
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2553#discussion_r4293793074>

## Processing Log

- 2026-10-16 10:42 UTC - Fetched the review and both inline threads, classified them as Copilot
  findings F1 and F2, and re-derived each against the current tree. Both were valid.
- 2026-10-16 10:45 UTC - Fixed F1 and F2 in separate signed commits, and corrected the M1 evidence
  record in a third.
- 2026-10-16 10:47 UTC - Pushed, replied on both threads (and corrected a garbled phrase in the F1
  reply in place).
- 2026-10-16 10:48 UTC - Started this record, with a reply URL in every row.

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
