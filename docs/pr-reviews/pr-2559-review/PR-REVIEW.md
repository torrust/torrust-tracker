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
the review nor the ACK covers the commits that fix F1 to F7. The PR conversation comment
<https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6266681876>, posted at 11:59:32
UTC, is the consolidated response for this round. It names reviews 5570637563, 5570856899 and
5571038375 and every finding F1 to F7 with its disposition and resolution reference. Each row's
`Reply URL` is the per-thread reply, which is the durable evidence the validation script checks.

Round 3: da2ce7's review 5571393132 (APPROVED, 2026-10-19 12:16:29 UTC, on the head whose newest
commit is `docs(pr-reviews): [#2475] record the review rounds in the PR #2559 audit`) carries four
inline findings. da2ce7 numbered them F1 to F4, which collide with the rows above, so they are F8 to
F11 here with the original number kept as the reviewer finding ID. The review body is a
verification report that confirms the seven round-2 fixes and adds no other row. Its ACK comment
(<https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6266990302>) names that head.
Copilot made no review after 11:28:45 UTC. The branch was rebased onto `develop` to fix F8 and
force-pushed at 12:21:03 UTC, which dismissed the approval, so neither the review nor the ACK
covers the commits that fix F8 to F11. The rebase replaced the commit ids that the reviews of
rounds 2 and 3 cite, so this record identifies every cited commit by its subject, as the details
below do. The consolidated response for this round is
<https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6267088991>, posted at 12:21:54
UTC. It names review 5571393132 and every finding F8 to F11 with its disposition and resolution
reference.

Round 4: da2ce7's review 5571702121 (APPROVED, 2026-10-19 12:43:37 UTC, on the head whose newest
commit is `docs(pr-reviews): [#2475] record the third review round in the PR #2559 audit`) carries
two inline findings, which da2ce7 numbered F12 and F13. The numbers do not collide, so the audit
ids are the same. The review body is a verification report that adds no other row. Its ACK comment
(<https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6267492597>) names that head.
Copilot made no review after 11:28:45 UTC. The push that carries the fixes dismissed the approval
at 12:47:25 UTC, so neither the review nor the ACK covers the commits that fix F12 and F13. The
consolidated response for this round is
<https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6267577845>, posted at 12:48:15
UTC. It names review 5571702121 and both findings with their dispositions and resolution
references. It also records that my first reply on F12 gave an inaccurate list of the remaining
grep hits and that I edited the reply, so the reply text at its URL is the corrected one.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2559-f1` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2559-f2` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2559-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2559-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2559-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2559-f6` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2559-f7` | Human | Suggestion | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2559-f8` | Human | Major | other | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2559-f9` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2559-f10` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2559-f11` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2559-f12` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2559-f13` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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

### F8 - `EPIC.md` conflicts with `develop`, so the branch must be updated before it can merge

- PR number: 2559
- Source review ID: 5571393132
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313694186>
- Concern: a three-way merge with `develop` conflicts in two hunks of the EPIC file, the
  `last-updated-utc` line and the progress-log append. The fix for F7, which edits that file,
  introduced the conflict, and GitHub reported the PR as not mergeable.
- Solution: rebase onto the current `develop`, keeping every commit, and resolve the two hunks as the
  reviewer described: keep `2026-10-19 11:53` and both log entries in date order. The conflict is
  resolved inside the F7 fix commit, `docs(issues): [#2475] link the corrected coupling report and
  the new files from the spec and the EPIC`. The force-push used `--force-with-lease`, and the PR
  body now says the history was rewritten.
- Current-tree verification: `git merge-tree --write-tree torrust/develop HEAD` exits 0,
  `git log --format=%G? torrust/develop..HEAD` shows a signature (`U`, good signature, unknown
  trust) on every commit, and `gh pr view 2559 --json mergeable` reports `MERGEABLE`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6267088991>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313737658>

### F9 - One log entry, stamped 11:57 and labelled "Started audit", reports the whole round

- PR number: 2559
- Source review ID: 5571393132
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313694191>
- Concern: the 11:57 entry stands for events spread over ten minutes, gives no time for the thread
  resolutions, and says the validator ran after the replies, although the skill asks for a run
  before replying. The log may not be rewritten, so dated entries and a correction are due.
- Solution: append an entry, stamped 12:19 UTC, that names the 11:57 entry and lists each event
  with its time. A log entry cannot carry an earlier stamp than the entries before it, so the times
  are in the text. The entry also says why the validator could not run earlier on the new rows
  (each row's reply URL must exist first). For this round the validator ran before replying, on the
  existing rows.
- Current-tree verification: `grep -n "Correction to the entry above, per da2ce7" docs/pr-reviews/pr-2559-review/PR-REVIEW.md`
  finds the entry, and the validation script reports zero failures with the entries in order.
- Resolution reference: docs(pr-reviews): [#2475] list the round 2 events with their times and store the summary comment URL
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313738006>

### F10 - The summary comment's URL is not stored

- PR number: 2559
- Source review ID: 5571393132
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313694195>
- Concern: the round 2 summary comment names the reviews and findings, but no row or paragraph
  stores its URL, which step 8 of the workflow asks for.
- Solution: the Round 2 paragraph of the Findings section now stores the comment URL with its time,
  and says each row's `Reply URL` is the per-thread reply that the validator checks. The Round 3
  paragraph does the same for this round's comment, which was posted before this record was
  committed.
- Current-tree verification: `grep -n "issuecomment-6266681876" docs/pr-reviews/pr-2559-review/PR-REVIEW.md`
  finds the URL in the Findings section.
- Resolution reference: docs(pr-reviews): [#2475] list the round 2 events with their times and store the summary comment URL
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313738324>

### F11 - The "summary lines" are an edited excerpt

- PR number: 2559
- Source review ID: 5571393132
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313694203>
- Concern: the T3 block in the evidence file has four of the five `cargo test` result lines and
  trimmed `Running` lines, so it is an excerpt, although the file asks for output as observed.
- Solution: rerun the command and paste all five result lines verbatim, with the `Running` suffixes
  and the `Doc-tests` line, and say the block is the output filtered with
  `grep -E "Running|Doc-tests|test result"`.
- Current-tree verification: `grep -n "Doc-tests workspace_coupling" docs/issues/open/2475-*/manual-verification-evidence.md`
  finds the line, and `cargo test -p workspace-coupling` prints the same five result lines
  (11 + 9 tests).
- Resolution reference: docs(issues): [#2475] paste the T3 cargo test result lines verbatim
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313738651>

### F12 - "review finding F11" and "finding F8" are audit ids, not the review's

- PR number: 2559
- Source review ID: 5571702121
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313942368>
- Concern: a spec log entry cites "PR #2559 review finding F11" and "finding F8", but the review
  comments carry other numbers (F4 and F1), so the reader must look the mapping up. The
  repository's immutable form for citing a finding from another artifact is
  `review-finding:pr-<PR>-<id>`.
- Solution: cite every finding of this PR in the spec's progress log and in the EPIC's log entry
  by its immutable reference, not only the two the reviewer named, because "review finding F1"
  was ambiguous between rounds. The new log entry names `review-finding:pr-2559-f12`. The first
  reply on this thread said the remaining grep hits were three cites of PR #2462; the hits also
  include one cite of PR #2445 in the EPIC and the new entry that quotes the old wording. I edited
  the reply to the accurate list.
- Current-tree verification: `grep -n "review finding F" docs/issues/open/2475-*/ISSUE.md docs/issues/open/1669-overhaul-packages/EPIC.md`
  finds no cite of a PR #2559 finding.
- Resolution reference: docs(issues): [#2475] cite review findings by their immutable reference
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313980197>

### F13 - Which run does the `linter all` line come from?

- PR number: 2559
- Source review ID: 5571702121
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313942375>
- Concern: T3 gives two rerun times, 11:48 and 12:17 UTC, and the `linter all` line does not say
  which run it belongs to. If it is the 11:48 run, it predates the file's current content.
- Solution: rerun `linter all` on the branch after the rebase, and record its own time (12:45 UTC),
  the exit code and its last line as printed.
- Current-tree verification: `grep -n "12:45" docs/issues/open/2475-*/manual-verification-evidence.md`
  finds the run time and the pasted last line.
- Resolution reference: docs(issues): [#2475] give the T3 linter all result its own run time
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2559#discussion_r4313980512>

## Processing Log

- 2026-10-19 11:57 UTC - Started audit after da2ce7's review 5571038375. Copilot's two reviews had
  no finding. Fixed F1 to F7 in separate commits, pushed them, replied on the seven threads, ran
  the validation script (zero failures) and resolved the seven threads.
- 2026-10-19 12:19 UTC - Correction to the entry above, per da2ce7's review 5571393132. That one
  entry, stamped 11:57 UTC, stands for events spread over ten minutes, and its order is wrong
  against the skill. The log is append-only, so the events are listed here with their times from
  the commits, the GitHub timeline and the review threads, instead of rewriting the entry. The
  seven fix commits were authored from 11:47:48 to 11:53:40 UTC. The push that carries them, and
  the dismissal of da2ce7's review 5571038375 by it, was at 11:55:00 UTC. The seven replies were
  posted from 11:55:33 to 11:55:45 UTC. The validation script first ran at 11:57:05 UTC, after the
  replies, although the skill asks for a run before replying; for this round it could not run
  earlier on the new rows, because it requires each row's reply URL to exist. The seven threads
  were resolved at about 11:57:43 UTC, and the audit commit was authored at 11:57:50 UTC. The
  summary comment <https://github.com/torrust/torrust-tracker/pull/2559#issuecomment-6266681876> was
  posted at 11:59:32 UTC, after the audit commit, which is why its URL is stored only now (finding
  F10).
- 2026-10-19 12:22 UTC - Round 3: da2ce7's review 5571393132 (APPROVED and ACK on the head that
  ended with the Round 2 audit commit) carried four findings, F8 to F11 here. Ran the validation
  script on the existing rows before replying (zero failures). Rebased onto `develop` and
  resolved the `EPIC.md` conflict (F8); the force-push at 12:21:03 UTC dismissed the approval.
  Fixed F11 in one commit and F9 and F10 in a second one, both before the push, replied on the four threads, and
  posted the consolidated response at 12:21:54 UTC. F8 to F11 are recorded in this audit commit,
  and the threads are resolved after the validation script passes on the new rows.
- 2026-10-19 12:48 UTC - Round 4: da2ce7's review 5571702121 (APPROVED and ACK on the head that
  ended with the Round 3 audit commit) carried two findings, F12 and F13 here. Fixed each in its
  own commit and pushed them at about 12:47 UTC; the push dismissed the approval at 12:47:25 UTC.
  Ran the validation script on the existing rows at 12:47:31 UTC, before replying (zero failures),
  replied on the two threads, and posted the consolidated response at 12:48:15 UTC. F12 and F13
  are recorded in this audit commit, and the threads are resolved after the validation script
  passes on the new rows. The reply on F12 was edited after posting to correct a wrong list of
  grep hits, which the Round 4 paragraph and the F12 detail state.

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
