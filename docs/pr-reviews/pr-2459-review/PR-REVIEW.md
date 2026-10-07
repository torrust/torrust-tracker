---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/agent-review-reports.md
    - packages/udp-server/src/testing/environment.rs
---

<!-- skill-link: process-pr-review -->

# PR #2459 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2459>.

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

Copilot review 5431636272 ("Balanced" effort, 16:35 UTC, at the first pushed head) left one inline
finding tagged `[Major][F1]`, kept as F1. Its overview badge says `High severity`; the inline
bracket is recorded as given. The review body is an overview only.

da2ce7 review 5431735067 (round 1, CHANGES_REQUESTED, 16:44 UTC, same head) left three inline
findings numbered F1-F3. F1 already belongs to Copilot's finding, so they take the next free
audit IDs, F2-F4, with their original IDs kept in the detail entries. The body confirms Copilot's
F1 at the bytes and leaves it under that finding; it adds no other actionable assertion.

F4 changed only the PR title, which is not a repository artifact, so it is `NO_ACTION` with the
reply URL as its reference, as in the PR #2363 record.

da2ce7 review 5433631723 (round 2, approval at the first fix head, later dismissed by a push)
found nothing new. da2ce7 review 5434121120 (round 3, CHANGES_REQUESTED, 20:20 UTC) left two inline
findings on this record, numbered F4 and F5 in the reviewer's series; F4 is taken, so they become
F5 and F6. F5 needed only the thread resolution, so like F4 it is `NO_ACTION` with its reply URL.
da2ce7 review 5434318741 (round 4, APPROVED, 20:39 UTC) found nothing new.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2459-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2459-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2459-f3` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2459-f4` | Human | Nit | metadata | ORIGINAL | NO_ACTION | RESOLVED |
| F5 | `review-finding:pr-2459-f5` | Human | Minor | metadata | ORIGINAL | NO_ACTION | RESOLVED |
| F6 | `review-finding:pr-2459-f6` | Human | Minor | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Dropping a running environment no longer stops the UDP receive loop

- PR number: 2459
- Source review ID: 5431636272
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4197939279>
- Concern: the legacy path stopped the receive loop when the environment was dropped, because the
  launcher treats a closed halt channel as a stop request. The migrated running state kept a plain
  `CancellationToken`, which dropping does not cancel, and dropping the task handles detaches
  them, so a dropped environment (for example during panic unwinding) left its socket bound while
  the runtime lived. The spec's drop-path line still claimed today's behavior was kept.
- Solution: hold the token as a `DropGuard` in the running state; `stop()` disarms it, then
  cancels and joins as before. Correct the spec's drop-path line.
- Current-tree verification: new test
  `it_should_release_the_udp_socket_when_dropped_without_being_stopped` drops a started environment
  and polls the binding within the 10 s test deadline without shutting the runtime down. Before
  the fix it failed with `Elapsed`; after it, 8 environment tests and the package suites pass
  (219 unit, 12 integration).
- Resolution reference: `fix(udp-server): [#2448] cancel the UDP environment's tasks when it is dropped without stop()`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4199214726>

### F2 - T5 is still `TODO` while every T5 activity is recorded as done

- PR number: 2459
- Source review ID: 5431735067
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4198022632>
- Concern: the Implementation Plan left T5 `TODO` and the committer checkpoint unchecked, although
  the AC review, pre-push checks, and Task Reviewer report were recorded.
- Solution: mark T5 `DONE` with its evidence, tick the checkpoint, and log this review round.
- Current-tree verification: the spec's T5 row reads `DONE`; the committer checkpoint is ticked;
  the 18:03 UTC entry records this round.
- Resolution reference: `docs(issues): [#2448] mark SI-17 verification done and log the PR review`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4199214989>

### F3 - The Task Reviewer report cites commit ids that the rebase removed

- PR number: 2459
- Source review ID: 5431735067
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4198022647>
- Concern: the report's invocation scope named four PR-branch commit ids that the rebase before the
  PR removed; branch ids are not durable.
- Solution: name the four reviewed commits by their Conventional Commit subjects; keep the durable
  `develop` id.
- Current-tree verification: `rg` over the spec folder for every PR-branch id returns nothing.
- Resolution reference: `docs(issues): [#2448] name reviewed commits by subject in the SI-17 review report`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4199215275>

### F4 - `refactor` understates the PR title and the environment commit

- PR number: 2459
- Source review ID: 5431735067
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4198022662>
- Concern: the PR removes public fields, changes `stop()`, fixes a listener leak, and changes the
  example's behavior, so `feat` fits better than `refactor`, as in SI-16.
- Solution: the PR title, which becomes the merge subject, now starts `feat(udp-server)`. The
  environment commit keeps its subject; the branch is not rewritten.
- Current-tree verification: `gh pr view 2459 --json title` returns the `feat(udp-server)` title.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4199215559>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4199215559>

### F5 - F1's `Thread state` said `RESOLVED` before Copilot's thread was resolved

- PR number: 2459
- Source review ID: 5434121120
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4200046641>
- Concern: at the round-3 capture, Copilot's thread was still unresolved, so the F1 row recorded a
  state that had not happened yet.
- Solution: resolve the thread; the reply guard before resolving had found rounds 2 and 3, so
  they were read first. No commit was needed.
- Current-tree verification: GraphQL shows Copilot's thread resolved (20:25:02 UTC).
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4203606180>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4203606180>

### F6 - The Status Values and Completion Rules blocks are not the template's verbatim text

- PR number: 2459
- Source review ID: 5434121120
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4200046652>
- Concern: the record dropped template sentences and bullets that the template says to copy
  verbatim, including the ones that govern the `NO_ACTION` rows.
- Solution: copy both blocks from `docs/templates/PR-REVIEW-TEMPLATE.md` unchanged.
- Current-tree verification: `diff` against the template shows Completion Rules identical and
  Status Values differing only by the template's guidance comment.
- Resolution reference: `docs(pr-reviews): restore the template status and completion blocks in the #2459 record`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2459#discussion_r4203606338>

## Processing Log

- 2026-10-06 18:12 UTC - Fetched Copilot review 5431636272 and da2ce7 review 5431735067 (four
  inline threads). Wrote the F1 regression test first and saw it fail, then committed the F1 fix
  (17:52:13), F3 (18:02:28), and F2 (18:09:11); rebased onto `develop` (2 commits behind, 18:10);
  force-pushed (18:12:33) with the pre-push checks passing.
- 2026-10-06 18:49 UTC - Reply guard found no newer review; retitled the PR to `feat(udp-server)`
  (F4); replied on all four threads (18:48:50-18:48:55).
- 2026-10-06 19:33 UTC - Recorded this audit and ran the validator (0 failures); the threads are
  resolved after it is pushed.
- 2026-10-07 06:02 UTC - Late entry for 2026-10-06. After pushing this record, the reply guard
  found rounds 2 (approval, dismissed by the push; no findings) and 3 (F5, F6). Resolved Copilot's
  thread (20:25:02, F5); committed the F6 fix; rebased onto `develop` (5 commits behind);
  force-pushed (20:31:50). A combined command then ran the reply guard and posted the two round-3
  replies without stopping on its output, which already listed round 4 (APPROVED, 20:39:52, no
  findings); the replies landed at 06:00:56-06:00:57 on 2026-10-07, after that approval. da2ce7
  had resolved both round-3 threads himself.

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
