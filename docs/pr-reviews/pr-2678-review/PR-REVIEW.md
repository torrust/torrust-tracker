---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2668-1488-si-20-configure-shutdown-policy/ISSUE.md
    - docs/adrs/20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md
---

<!-- skill-link: process-pr-review -->

# PR #2678 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2678>.

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

One review with a finding was submitted: the review of `copilot-pull-request-reviewer` (5680948571), submitted at
10:10:20 UTC on the head whose subject is `docs(issues): link pull request 2678 and record the rebase for #2668`. It has
one inline thread and a body that only summarizes it, so the body adds no row. The reviewer labelled the finding
`R1`; it is kept as the `Reviewer finding ID` and the audit assigns `F1`. A second review object appears on the
pull request (5681638657, account `josecelano`, 11:07:31 UTC, empty body); it only carries the author's reply
to the thread, posted through the maintainer's GitHub account, and contains no finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------- | ------------ | -------- | ------------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2678-f1 | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Qualify which drain timeouts fail the shutdown

- PR number: 2678
- Source review ID: 5680948571
- Reviewer finding ID: R1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2678#discussion_r4399290776>
- Concern: the budget ADR said that a drain that reaches its budget is reported as a failed component, and the
  exit code section of `docs/containers.md` said exit `0` means every component finished or was cancelled. Only the
  HTTP-family drains produce a failed component; the UDP request drain and the event listener drain log a warning
  and still end as completed or cancelled, so exit `0` and the success message do not prove that no UDP request or
  queued event was discarded.
- Solution: the finding was right. The ADR now says that only the HTTP tracker, the REST API and the health check
  API end as a failed component when their budget is reached, that the UDP request drain aborts the processors it
  could not finish and logs how many, that a listener whose admission budget expires logs how many events it left
  unread, and that exit code `0` therefore does not prove nothing was discarded. `docs/containers.md` has the same
  statement after its exit code table. The sweep for the same overstatement in my own records found one more
  instance, in the exit-result table of the specification ("including a drain budget reached"), which now names the
  HTTP-family drain. The specification's current-state table, which says the HTTP, REST API and health-check
  components return an error, already had the right scope.
- Current-tree verification: `grep -n "HTTP-family drains" docs/adrs/20261027145446_budget_shutdown_in_two_chained_stages_inside_one_process_deadline.md`
  and `grep -n "Only an HTTP-family drain" docs/containers.md` find the new text. `supervise_receive_loop` in
  `src/bootstrap/jobs/udp_tracker.rs` ends with `Ok(ComponentCompletion::Cancelled)` after an `Ok(())` loop result;
  `drain_request_processors_on_shutdown` in `packages/udp-server/src/server/launcher.rs` only logs its outcome; the
  drain of `packages/events/src/drain.rs` reports `DrainOutcome::TimedOut` as a warning. `grep -n "HTTP-family drain
  budget reached" docs/issues/open/2668-1488-si-20-configure-shutdown-policy/ISSUE.md` finds the qualified table row.
- Resolution reference: `docs(shutdown): qualify which drain timeouts end a stop with exit code 1`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2678#discussion_r4399816677>

## Processing Log

- 2026-10-28 10:10 UTC - The review of `copilot-pull-request-reviewer` was submitted with one inline thread
  (GitHub `submitted_at` 10:10:20Z). The reviews of the pull request had been read at the end of the previous CI
  wait and listed no review; this one was found at the next read.
- 2026-10-28 10:24 UTC - The finding was checked against the current tree and confirmed. The fix was made in one
  signed commit, with author time 10:24:46Z, after `linter all` exited 0.
- 2026-10-28 11:07 UTC - The fix commit was pushed to the pull-request branch before the reply (the review that
  carries the reply is recorded on that head). The reply was posted on the thread (GitHub `created_at` 11:07:31Z)
  and the thread was resolved. A refreshed GraphQL fetch lists one thread, resolved and outdated, and no
  unresolved thread.

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
