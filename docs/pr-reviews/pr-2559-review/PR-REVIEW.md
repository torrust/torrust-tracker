---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2559 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2559>.

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

Round 1: Copilot's reviews 5570637563 (COMMENTED, 2026-10-19 11:07:54 UTC) and 5570856899
(COMMENTED, 2026-10-19 11:28:45 UTC) are overviews that recommend approval and report no open
issue. They carry no inline thread and no independently actionable assertion, so they add no row.

Round 2: da2ce7's review 5571038375 (submitted 2026-10-19 11:45:56 UTC on the PR's first head, whose
newest commit is `docs(issues): [#2475] regenerate the workspace coupling report`) carries seven
inline findings, F1 to F7, which da2ce7 numbered the same way. Its body is a verification report that
adds no other row. da2ce7 also left an ACK comment
(<https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6266450751>) that names that
first head. The review was dismissed at 11:55:00 UTC by the push that carries the fixes, so neither
the review nor the ACK covers the commits that fix F1 to F7.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2559-f1` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2559-f2` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2559-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2559-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2559-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2559-f6` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2559-f7` | Human | Suggestion | link-integrity | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Status stays `planned` and `related-pr` stays `null` while the work is in review

- PR number: 2559
- Source review ID: 5571038375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313415760>
- Concern: the spec's frontmatter still says `status: planned` and `related-pr: null`, although the
  implementation is complete and the change is in an open pull request.
- Solution: set `status: in-review` and `related-pr: 2559`.
- Current-tree verification: `grep -nE "^(status|related-pr):" docs/issues/open/2475-*/ISSUE.md`
  prints `status: in-review` and `related-pr: 2559`.
- Resolution reference: docs(issues): [#2475] set the spec status to in-review and link PR 2559
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313502046>

### F2 - The 12:40 stamps are later than the commit that writes them

- PR number: 2559
- Source review ID: 5571038375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313415768>
- Concern: the progress-log entry and the `last-updated-utc` of the spec and of the evidence file
  read 2026-10-17 12:40, later than the commit that wrote them (12:33:09 UTC).
- Solution: restamp the log entry to 12:33 UTC. The `last-updated-utc` of both files is restamped
  by each commit that edits them, to the minute of that commit. One stamp of this round was first
  written a minute after its commit (11:52 for a commit at 11:51:50); a later commit corrected it
  and the spec's log says so.
- Current-tree verification: `grep -n "2026-10-17 12:40" docs/issues/open/2475-*/*.md` prints
  nothing, and the spec's log entries read in chronological order.
- Resolution reference: docs(issues): [#2475] restamp the implementation log entry to the time of its commit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313502282>

### F3 - The checkboxes and the Acceptance Verification table disagree

- PR number: 2559
- Source review ID: 5571038375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313415776>
- Concern: the table marks AC1 to AC5 `DONE`, but their checkboxes, and the "Automatic
  verification completed" checkbox, were not ticked.
- Solution: tick the checkboxes of AC1 to AC5 and of "Automatic verification completed". AC6 stays
  not ticked and `TODO`, because its re-review is still pending.
- Current-tree verification: `grep -n "^- \[" docs/issues/open/2475-*/ISSUE.md` shows AC1 to AC5 and
  "Automatic verification completed" ticked, and AC6 not ticked.
- Resolution reference: docs(issues): [#2475] tick the acceptance-criteria checkboxes that the verification table marks done
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313502547>

### F4 - R2's per-edge listing and T3 have no command or output

- PR number: 2559
- Source review ID: 5571038375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313415788>
- Concern: the R1 listing script prints only "no references" lines, so it cannot have produced R2's
  per-edge listing, and T3 states the test counts and the `linter all` exit only as prose.
- Solution: rerun both and record the real commands and output. T3 now has the commands, the
  `cargo test` summary lines and the exit codes. R2 has the Python script that produced the listing,
  and its output. The rerun was made after the F6 tests, so the T3 counts are 11 and 9, and the
  evidence says so. `setdefault`, from the script, is added to `project-words.txt`.
- Current-tree verification: `grep -n "setdefault\|test result" docs/issues/open/2475-*/manual-verification-evidence.md`
  shows the script and the summary lines, and `linter cspell` passes.
- Resolution reference: docs(issues): [#2475] record the commands and output of the T3 and R2 verification
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313502756>

### F5 - The commit that marks T4 done lands before the report exists

- PR number: 2559
- Source review ID: 5571038375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313415802>
- Concern: `docs(issues): [#2475] record the red run, green run and R2 recheck` marks T4 `DONE`
  and logs the regenerated report, which only arrives in the next commit, so the step-by-step
  history does not follow the Commit Points table.
- Solution: do not rewrite history, because the repository keeps intermediate commits. Append a
  correction entry to the spec's progress log that names the 12:33 entry, the early commit and the
  commit that added the report.
- Current-tree verification: `grep -n "Correction to the 2026-10-17" docs/issues/open/2475-*/ISSUE.md`
  finds the entry.
- Resolution reference: docs(issues): [#2475] note that T4 was logged one commit before the report landed
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313503060>

### F6 - `rlib`, `dylib` and multi-kind libraries are untested

- PR number: 2559
- Source review ID: 5571038375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313415812>
- Concern: no test has a target of kind `rlib` or `dylib`, or a library with several kinds, so a
  mutant that drops either kind from the list passes every test.
- Solution: add `it_should_use_the_name_of_an_rlib_or_dylib_target` and
  `it_should_use_the_library_target_name_when_the_target_has_several_kinds` (`["cdylib", "rlib"]`).
- Current-tree verification: a mutant that shrinks the importable-kinds list to `lib` and
  `proc-macro` fails both new tests and passes the other nine; with the code restored,
  `cargo test -p workspace-coupling` passes (11 and 9 tests).
- Resolution reference: test(workspace-coupling): [#2475] cover rlib, dylib and multi-kind library targets
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313503364>

### F7 - Nothing outside the spec links the new report

- PR number: 2559
- Source review ID: 5571038375
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313415817>
- Concern: the spec's `related-artifacts` omit `lib.rs`, `tests/resolve_module_name.rs` and the new
  report, and the EPIC links only the 2026-10-06 report, whose finding 3 describes the bug.
- Solution: add the three files to `related-artifacts`; link the 2026-10-17 report from the EPIC's
  #2475 checklist item and Details table row; add an EPIC progress-log entry.
- Current-tree verification: `grep -n "workspace-coupling-report-2026-10-17" docs/issues/open/1669-overhaul-packages/EPIC.md`
  finds the three links, and `linter lychee` passes.
- Resolution reference: docs(issues): [#2475] link the corrected coupling report and the new files from the spec and the EPIC
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313503607>

## Processing Log

- 2026-10-19 11:57 UTC - Started audit after da2ce7's review 5571038375. Copilot's two reviews had
  no finding. Fixed F1 to F7 in separate commits, pushed them, replied on the seven threads, ran
  the validation script (zero failures) and resolved the seven threads.

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
