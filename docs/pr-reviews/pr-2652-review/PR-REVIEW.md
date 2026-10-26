---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2652 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2652>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A (pre-merge review)

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

Round 1 has two reviews. Copilot's review `5649072698` (https://github.com/torrust/torrust-tracker/pull/2652#pullrequestreview-5649072698), submitted 2026-10-26 08:15 UTC, has one inline finding, F1. The maintainer da2ce7's review `5650838526` (https://github.com/torrust/torrust-tracker/pull/2652#pullrequestreview-5650838526), submitted 2026-10-26 10:48 UTC with `CHANGES_REQUESTED`, has two inline findings, F2 and F3; its body is a summary with the verdict and its checks and holds no further independently actionable assertion. The review `5652143371` is my own reply on the F1 thread and has no body. The reviewers' own IDs (`F1`, `F2`, `F3`) do not collide, so the audit keeps them.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2652-f1 | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2652-f2 | Human | Major | correctness | RE_RAISE_OF:F1 | FIXED | RESOLVED |
| F3 | review-finding:pr-2652-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Keep the environment restartable with a fixed bind port

- PR number: 2652
- Source review ID: 5649072698
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4375139248
- Concern: `start_with_cancellation()` registers the health-check binding in the caller's `registar`, `stop()` kept that registration in the returned environment, and a second `start()` on a nonzero port got `RegistrationError::DuplicateBinding` and panicked. The reviewer asked for a restart-safe registration that keeps caller-owned services, and a fixed-port stop/start regression test.
- Solution: `stop()` now calls `Registar::deregister` with the binding the server registered, after joining both tasks and also when a task failed. `Registar::deregister` was added to `torrust-server-lib` 0.4.0 (torrust/torrust-server-lib#2, merged and published), because the 0.3.0 registry had no remove operation. It removes only that binding, so other owners' services stay. A first reply, at 2026-10-26 12:45 UTC (https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4377335685), said FOLLOW_UP and documented the limitation with a `should_panic` test; that was reversed after F2, and the new reply below corrects it. The `should_panic` test is replaced by a green fixed-port restart test.
- Current-tree verification: `cargo test -p torrust-tracker-axum-health-check-api-server` passed 14 lib tests and 8 integration tests on the pushed tree, including `it_should_serve_requests_after_being_stopped_and_started_again_on_a_fixed_port`, `it_should_remove_only_its_own_registration_from_the_registar_when_stopped` and `it_should_remove_its_registration_even_when_stopped_with_a_failed_task`. Removing the `deregister` call failed those three tests, and removing the registration only after the join result is checked failed the last one (evidence V7).
- Resolution reference: fix(axum-health-check-api-server): [#2450] deregister the health-check test environment on stop
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4379919227

### F2 - Remove the stale health-check registration before a fixed-port restart

- PR number: 2652
- Source review ID: 5650838526
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4376385862
- Concern: A re-raise of F1 by the maintainer. The returned environment kept the caller's registry with the stale entry, so a fixed-port restart panicked; the restart test used port 0 and did not establish AC4. He asked for only the environment's own registration to be restart-safe, preserving caller-owned services, with a regression test on a dynamically selected nonzero port, and noted that copying the REST environment's fresh registry would discard caller services.
- Solution: The same change as F1. The fresh-registry approach was rejected for the reason he gave (spec decision D6), and AC4 now says a restart works "on an ephemeral or a fixed port" and AC8 covers the registration; both stay TODO for the maintainer to tick.
- Current-tree verification: the same test run as F1 (14 lib and 8 integration tests passed). `it_should_serve_requests_after_being_stopped_and_started_again_on_a_fixed_port` picks a free port by binding `127.0.0.1:0` and dropping the listener, restarts on it and gets HTTP 200. The registry test asserts that another owner's UDP binding is the only one left after `stop()`.
- Resolution reference: fix(axum-health-check-api-server): [#2450] deregister the health-check test environment on stop
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4379919713

### F3 - Append an evidence-backed correction to the completion review

- PR number: 2652
- Source review ID: 5650838526
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4376385876
- Concern: The independent-review entry in `agent-review-reports.md` said all seven acceptance criteria were met, which F2 contradicts; it lacked the template's Inputs, Evidence, Verdict and Follow-up actions; and it said every finding was resolved in the commit that recorded it, although the test change was in an earlier commit. The reviewer asked for an appended correction that keeps the earlier entry.
- Solution: Appended a correction entry that withdraws the AC4 conclusion, states the real resolution references and carries the missing template fields; the earlier entry is unchanged. A new independent review of the final tree is a separate, later entry.
- Current-tree verification: `awk` from the 16:10 heading to the end of `agent-review-reports.md` lists `Invocation scope`, `Inputs`, `Findings`, `Verdict` and `Follow-up actions` in both new entries, and the second entry's Evidence heading carries extra wording. The 07:38 entry is byte-for-byte unchanged in the diff (additions only). `linter all` exited 0.
- Resolution reference: docs(issues): [#2450] correct the independent review entry after the fixed-port finding
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4379920169

## Processing Log

- 2026-10-26 08:15 UTC - Copilot review `5649072698` submitted with one inline finding (F1).
- 2026-10-26 10:48 UTC - Review `5650838526` submitted by da2ce7 with `CHANGES_REQUESTED` and two inline findings (F2 re-raise of F1, F3).
- 2026-10-26 12:45 UTC - First reply on the F1 thread (https://github.com/torrust/torrust-tracker/pull/2652#discussion_r4377335685), recorded FOLLOW_UP; superseded by the 16:41 reply after the maintainer chose to fix it here.
- 2026-10-26 16:19 UTC - torrust/torrust-server-lib#2 merged (GitHub `merged_at`), then `torrust-server-lib` 0.4.0 was published.
- 2026-10-26 16:41 UTC - Replied on the three threads (reply times 16:41:57, 16:41:59 and 16:42:01 UTC, GitHub); the F3 reply was edited at 16:42:11 UTC to say "all six template fields".
- 2026-10-26 16:42 UTC - Each resolve mutation returned `isResolved: true`, and a GraphQL fetch showed the three threads resolved.
- 2026-10-26 16:43 UTC - Wrote this audit and validated it with `validate-audit-record.py --pr-number 2652 --base torrust/develop`, which exited 0 with 3 rows (command output at 16:43 UTC).

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
