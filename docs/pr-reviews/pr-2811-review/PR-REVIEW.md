---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2811 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2811.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2811-f1 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2811-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2811-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2811-f4 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2811-f5 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2811-f6 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2811-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2811-f8 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2811-f9 | Human | Suggestion | testing | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F10 | review-finding:pr-2811-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2811-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2811-f12 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2811-f13 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2811-f14 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Doctest misses a PartialOrd-only implementation

- PR number: 2811
- Source review ID: 5846498596
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528323166
- Concern: The `compile_fail` doctest bounds `Peer` by `Ord` only, so an id-only `PartialOrd` impl without `Ord` would keep it green although AC1 requires neither trait.
- Solution: Changed the bound to `PartialOrd` (`Ord` requires it), so one check guards both removals.
- Current-tree verification: Ran `cargo test -p torrust-tracker-primitives --doc`: passes. With a temporary id-only `impl PartialOrd for Peer` appended, the same command failed with "compiled successfully, but it's marked `compile_fail`"; the file was restored after.
- Resolution reference: test(primitives): [#2798] also guard against a PartialOrd-only Peer ordering
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528556055

### F2 - Restore the major-version note in Risks

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326644
- Concern: The Risks bullet dropped the version consequence and gave a reason that does not hold: `torrust-tracker-primitives` is publishable at 3.1.0.
- Solution: Restored the note (publishable crate, next publish is a major version per the versioning ADR) and corrected the acceptance reason in Risks and in the Decision.
- Current-tree verification: Read `packages/primitives/Cargo.toml` (`publish.workspace = true`, `version = "3.1.0"`) and the edited bullets.
- Resolution reference: docs(issues): [#2798] bring the Peer Ord spec up to date with the decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528556449

### F3 - Name the code state of V2 and drop the old V2 stub

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326661
- Concern: The evidence had two V2 sections with opposite statuses and V2 named no code state.
- Solution: Deleted the stub, named the code state of V2 and of the red run, and stated why T4 and T5 are one commit.
- Current-tree verification: Read the evidence file after the edit: one V2 section, with its code state.
- Resolution reference: docs(issues): [#2798] name the code state and green output in the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528557054

### F4 - Paste the green regression-test output and refresh stale evidence lines

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326666
- Concern: The evidence had a prose summary instead of the green output, and stale boundary, closing note and last-updated lines.
- Solution: Pasted the `--doc` output, replaced the stale boundary and closing lines, and updated `last-updated-utc`.
- Current-tree verification: Ran `cargo test -p torrust-tracker-primitives --doc` (1 passed) and read the edited file.
- Resolution reference: docs(issues): [#2798] name the code state and green output in the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528557618

### F5 - Record the spec's pre-PR state

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326682
- Concern: No task review was recorded, the completion review was not assessed, and the checkpoints and frontmatter were stale.
- Solution: Recorded that no Task Reviewer pass ran, ticked the checkpoints and criteria that ran, stated why no retrospective is needed, and updated `status`, `related-pr` and `branch`.
- Current-tree verification: Read the spec after the edit; `linter all` passed.
- Resolution reference: docs(issues): [#2798] bring the Peer Ord spec up to date with the decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528558218

### F6 - Commit messages point to the wrong commit

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326688
- Concern: The fix commit carried the spec and evidence updates while the last docs commit claimed them.
- Solution: Reworded the last docs commit subject and added a body paragraph to the fix commit, with one `--force-with-lease` rewrite and a note in the PR body.
- Current-tree verification: Read both messages with `git log --format=%B`; the push printed a forced update.
- Resolution reference: docs(issues): [#2798] mark T4 to T6 done and record the final recheck
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528558753

### F7 - Stale cross-references after the decision

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326701
- Concern: Three references still pointed at the recommendation, the pending decision, or an ADR revisit.
- Solution: Updated the options row, the T1 findings heading and the ADR line.
- Current-tree verification: Read the three lines after the edit.
- Resolution reference: docs(issues): [#2798] bring the Peer Ord spec up to date with the decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528559321

### F8 - Overstated claim about the only way

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326716
- Concern: The Decision called removal the only way to make the violation impossible, but a derived all-field `Ord` also agrees.
- Solution: Reworded to "the simplest way", with the reason that no ordering is left to keep in step.
- Current-tree verification: Read the Decision after the edit.
- Resolution reference: docs(issues): [#2798] bring the Peer Ord spec up to date with the decision
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528559785

### F9 - Doctest guards Ord only (repeat of F1)

- PR number: 2811
- Source review ID: 5846502890
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528326726
- Concern: The doctest guards `Ord` only while AC1 claims neither trait; suggests a sibling `PartialOrd` block.
- Solution: None beyond F1: the single `PartialOrd` bound covers both traits.
- Current-tree verification: Re-read the doctest in `peer.rs`: the bound is `PartialOrd`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528560253
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528560253

### F10 - Strategy records a red run that did not happen

- PR number: 2811
- Source review ID: 5847136183
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528803990
- Concern: The strategy said the red run of the `PartialOrd` doctest was recorded before the fix, but the recorded red run used the earlier `Ord` bound; the scope line still said `Peer` is not `Ord`.
- Solution: Recorded a red run of the final doctest against the reintroduced id-only impls (mutate, then restore), reworded the strategy to say which bound each red run used, and changed the scope line to neither trait.
- Current-tree verification: Ran the doctest against the mutated file (failed as `compile_fail`), restored the file (`git diff --quiet HEAD -- packages/primitives/src/peer.rs` exited 0), read the strategy and scope lines.
- Resolution reference: docs(issues): [#2798] state only the red runs that happened for the doctest
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4529494965

### F11 - Versioning ADR is cited above its first mention

- PR number: 2811
- Source review ID: 5847136183
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528804001
- Concern: The ADR line said "the versioning ADR above" but no ADR was cited above it, and Related ADRs said none known.
- Solution: Listed the versioning ADR under Related ADRs and wrote "the versioning ADR listed above".
- Current-tree verification: Read both lines and checked that the ADR file exists.
- Resolution reference: docs(issues): [#2798] list the versioning ADR under Related ADRs
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4529495517

### F12 - Retrospective rationale not in the progress log

- PR number: 2811
- Source review ID: 5847136183
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528804010
- Concern: The retrospective rationale named one discovery and was not in the progress log; Commit Points still planned T4 and T5 apart.
- Solution: Added a progress-log entry with the reason and both discoveries, updated the Retrospective line, and noted the T4/T5 merge in Commit Points.
- Current-tree verification: Read the progress log, the Retrospective line and the Commit Points table.
- Resolution reference: docs(issues): [#2798] record the retrospective rationale and the T4/T5 merge
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4529496092

### F13 - Processing Log cites branch commit ids

- PR number: 2811
- Source review ID: 5847136183
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528804020
- Concern: The Processing Log quoted three PR-branch commit ids from push output, which durable records must not carry.
- Solution: Rewrote the two entries to name the pushed states by commit subject and appended a correction entry naming the edit.
- Current-tree verification: Searched the Processing Log for 9-character ids (none) and ran `validate-audit-record.py --pr-number 2811` (ok, 0 failures).
- Resolution reference: docs(pr-reviews): [#2798] remove commit ids from the PR 2811 audit log
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4529496564

### F14 - Record omits the Completion Rules section

- PR number: 2811
- Source review ID: 5847136183
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4528804029
- Concern: The audit record lacked the template's `## Completion Rules` section.
- Solution: Appended the section copied verbatim from the template.
- Current-tree verification: Compared the section with `docs/templates/PR-REVIEW-TEMPLATE.md` using `diff`: identical.
- Resolution reference: docs(pr-reviews): [#2798] add the Completion Rules section to the PR 2811 audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2811#discussion_r4529497148

## Processing Log

- 2026-11-10 17:38 UTC - Review 5846498596 (Copilot, `COMMENTED`, submitted 17:38:22 UTC) raised F1; review 5846502890 (`da2ce7`, shown by GitHub as `DISMISSED`, submitted 17:38:41 UTC) raised F2 to F9 (its IDs F1 to F8). F9 repeats F1.
- 2026-11-10 17:59 UTC - Committed the fixes as "test(primitives): [#2798] also guard against a PartialOrd-only Peer ordering" (F1), "docs(issues): [#2798] name the code state and green output in the evidence" (F3, F4; commit time 17:47:14 UTC) and "docs(issues): [#2798] bring the Peer Ord spec up to date with the decision" (F2, F5, F7, F8; 17:47:53 UTC), reworded two earlier commits for F6, and pushed all with one `--force-with-lease` update (the push printed a forced update of the branch to the head carrying "docs(issues): [#2798] bring the Peer Ord spec up to date with the decision"). Edited the PR description at 17:59:00 UTC to correct the publishability sentence and note the history rewrite.
- 2026-11-10 17:59 UTC - Posted the replies on the nine threads (`created_at` 17:59:27 to 17:59:49 UTC); F9 got the `Superseded by F1` reply.
- 2026-11-10 18:05 UTC - Pushed the audit record (commit "docs(pr-reviews): [#2798] record the PR 2811 review audit", commit time 18:01:54 UTC; the push printed a fast-forward update to the head carrying that audit commit; `validate-audit-record.py --pr-number 2811` printed `"status": "ok"` with 0 failures), then resolved all nine threads, each returning `isResolved: true` (the command finished at 18:04:48 UTC). A refreshed GraphQL query showed 0 unresolved threads.
- 2026-11-10 18:22 UTC - Review 5847136183 (`da2ce7`, `APPROVED`, submitted 18:22:17 UTC) raised F10 to F14 (its IDs F9, F11, F12, F13, F14; its F10 was fixed by the round 1 audit record).
- 2026-11-10 19:23 UTC - Correction: the entries of 17:59 UTC and 18:05 UTC quoted three PR-branch commit ids from the push output; they were rewritten in place to name the pushed states by commit subject instead (review round 2, F13). No other content of those entries changed.
- 2026-11-10 19:27 UTC - Committed the fixes as "docs(issues): [#2798] state only the red runs that happened for the doctest" (F10; commit time 19:18:51 UTC), "docs(issues): [#2798] list the versioning ADR under Related ADRs" (F11; 19:21:30 UTC), "docs(issues): [#2798] record the retrospective rationale and the T4/T5 merge" (F12; 19:22:20 UTC), "docs(pr-reviews): [#2798] remove commit ids from the PR 2811 audit log" (F13; 19:23:25 UTC) and "docs(pr-reviews): [#2798] add the Completion Rules section to the PR 2811 audit" (F14; 19:24:06 UTC), pushed them in one fast-forward push, and posted the replies on the five threads (`created_at` 19:27:10 to 19:27:22 UTC).

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
