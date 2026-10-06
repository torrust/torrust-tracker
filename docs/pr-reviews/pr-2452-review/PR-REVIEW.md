---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2452 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2452>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Copilot's inline comments carry no `[Severity]` bracket. Its review overview rates both findings
`Medium severity` in badge markup; as in the PR #2423 audit, they are recorded as
`Minor (inferred)`.

Human review 5428696523 (da2ce7, round 1) numbered its findings F1-F5, which collide with
Copilot's F1-F2 here. They are recorded as F3-F7, with the reviewer's IDs in each detail entry.
That review also called Copilot's two comments "the cleanup-timing point the merged #2440
settled", which agrees with declining F1 and F2.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2452-f1` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2452-f2` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F3 | `review-finding:pr-2452-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2452-f4` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2452-f5` | Human | Suggestion | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2452-f6` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2452-f7` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - EPIC row stays IN_PROGRESS although the PR closes #2245

- PR number: 2452
- Source review ID: 5428540203
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195414808>
- Concern: The EPIC #2243 subissue row for #2245 would be stale after merge if it stays `IN_PROGRESS`; set it to `DONE` here or change the closure timing.
- Solution: No change, agreed with the maintainer. The `cleanup-completed-issues` skill sets the parent EPIC row to `DONE`, together with the new `docs/issues/closed/` path, in the archive PR that follows the merge. #2246 followed that sequence (PR #2440 left the row `IN_PROGRESS`; archive PR #2442 set it to `DONE`), and the #2440 review confirmed it. Setting `DONE` here would leave the row pointing at an `open/` path.
- Current-tree verification: `EPIC.md` line 74 lists #2245 as `IN_PROGRESS` with its `docs/issues/open/` path.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195538965>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195538965>

### F2 - Spec status stays in-progress although the PR closes #2245

- PR number: 2452
- Source review ID: 5428540203
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195414923>
- Concern: `status: in-progress` will be inconsistent once the PR merges; set it to `done`, or drop `Closes #2245`.
- Solution: No change, agreed with the maintainer. `done` is invalid for a spec under `docs/issues/open/`: the frontmatter validator allows only `planned`, `in-progress`, `blocked`, or `in-review` there. The archive PR sets `status: done` when it moves the spec to `docs/issues/closed/`. `Closes #2245` stays because this PR completes the issue's work.
- Current-tree verification: with `status: done` set temporarily, `frontmatter-validator` reported `lifecycle-location-mismatch` and exited 1; the change was reverted and the tree is clean.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195539245>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195539245>

### F3 - Spec lacks the bug-fix sections for a reproduced defect

- PR number: 2452
- Source review ID: 5428696523
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195543332>
- Concern: A171 is a reproduced defect shipped as `fix(udp-server)`, but the spec omits the `Bug-Fix Process` and `Regression Test Strategy` sections that `fix-bug` and the v1 template require for bug work, whatever the `issue-type`.
- Solution: Added both sections after `Design and Ownership Review`: the six-step process (analysis, reproduction on `develop` at 10:19 UTC, boundary, red, fix, green plus M1 recheck), a note that the red run used mutate-then-restore because the test was added during review, and the chosen `build_response` boundary with its rationale. Spec text made stale by F5 and F6 was aligned in the same commit.
- Current-tree verification: both sections are present in `ISSUE.md`; the frontmatter validator passes.
- Resolution reference: `docs(issues): [#2245] add bug-fix process and regression test strategy`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4196691543>

### F4 - Evidence lacks the regression-test boundary and red/green output

- PR number: 2452
- Source review ID: 5428696523
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195543344>
- Concern: AC3 and the PR body claimed mutation-checked tests, but the evidence recorded only M1, with no boundary, red run, or green run for a reviewer to audit.
- Solution: Added a `Regression-Test Boundary` section with two mutate-then-restore red runs (call sites back to the original cast; wrapping helper) and the green run, each with its command, verbatim output, UTC time, and commit context. AC3 now cites it.
- Current-tree verification: the quoted outputs match the saved runs (13:18 UTC call sites and green; 14:15 UTC helper).
- Resolution reference: `docs(issues): [#2245] record the regression-test boundary and red/green runs`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4196691874>

### F5 - The new clamp helper duplicates the scrape handler's

- PR number: 2452
- Source review ID: 5428696523
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195543351>
- Concern: `saturating_wire_i32` repeats the expression of the scrape handler's existing, tested `udp_counter_from_u32`.
- Solution: One `saturating_wire_i32` now lives in `handlers/mod.rs`, used by announce and scrape, with one set of boundary tests; the scrape copy, its two tests, and its stale `#1525` comment are gone. A rebase onto `develop` conflicted with a #2417 test appended to the same test module; both were kept.
- Current-tree verification: `grep -rn udp_counter_from_u32 packages/udp-server/src` finds nothing; `cargo test -p torrust-tracker-udp-server --lib` passes (213 tests); Clippy `-D warnings` is clean.
- Resolution reference: `refactor(udp-server): [#2245] share one BEP 15 i32 clamp between announce and scrape`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4196692262>

### F6 - No test pins the clamp at the build_response call sites

- PR number: 2452
- Source review ID: 5428696523
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195543355>
- Concern: The helper tests stay green if a `build_response` call site goes back to `I32::new(x as i32)`, the original defect's shape.
- Solution: Added `it_should_clamp_an_out_of_range_interval_and_peer_counts_for_both_address_families`, which calls `build_response` with `interval` and `incomplete` at `i32::MAX + 1` and `complete` at `u32::MAX`, for both address families. It fails with the call sites mutated back to the original cast, and serves as the regression test F3 and F4 describe.
- Current-tree verification: the focused test passes; its red output is in the evidence file.
- Resolution reference: `test(udp-server): [#2245] pin the announce wire clamp at every build_response call site`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4196692613>

### F7 - The deferred configuration-load rejection has no tracked artifact

- PR number: 2452
- Source review ID: 5428696523
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4195543361>
- Concern: Once #2245 closes and its spec is archived, the deferred fail-fast work would have no live record.
- Solution: The maintainer chose a note on the EPIC #2243 row for #2245 now, and keeps the follow-up specification draft local until this PR merges, to avoid switching branches mid-review.
- Current-tree verification: the EPIC row's Notes cell names the follow-up; the frontmatter validator passes.
- Resolution reference: `docs(issues): [#2245] note the configuration-load follow-up on the EPIC row`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2452#discussion_r4196692982>

## Processing Log

- 2026-10-06 12:46 UTC - Copilot review 5428540203 submitted findings F1 and F2.
- 2026-10-06 12:58 UTC - The maintainer agreed to decline F1 and F2 as conflicting with the archival lifecycle. Replied to both threads with the evidence and started this audit. No human review had been submitted at this time.
- 2026-10-06 14:42 UTC - Correction to the 12:58 entry: human review 5428696523 (da2ce7, round 1) had been submitted at 12:58:26 UTC. It was not visible when the entry was written and was found when the reply status was checked after the push. It requested changes with five findings, recorded here as F3-F7. Fixed all five in separate commits, rebased onto `develop` (resolving one test-module conflict with #2417), pushed after the pre-push suite passed, updated the PR body, and replied to each thread.
- 2026-10-06 15:09 UTC - Human review 5430228175 (da2ce7, round 2, 14:50 UTC, at `docs(issues): [#2245] note the configuration-load follow-up on the EPIC row`) approved with no new findings, verified F3-F7 fixed, and resolved their threads. Resolved the two replied Copilot threads (F1, F2); no unresolved thread remains.
- 2026-10-06 15:27 UTC - The force push of the round-1 record dismissed review 5430228175. Human review 5430644514 (da2ce7, round 3, 15:18 UTC, at `docs(pr-reviews): [#2245] record round-1 human review findings on #2452`) approved with no findings, confirming that the rebase changed only the base and that this record matches the posted reviews and replies.

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
