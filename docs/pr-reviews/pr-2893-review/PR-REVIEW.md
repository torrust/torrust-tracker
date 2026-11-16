---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2893 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2893.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

<!-- Copied verbatim into each audit record. -->

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

F1 and F2 are the two inline comments of Copilot's review 5925597715 (`COMMENTED`), submitted at 18:51:32 UTC on the head commit `docs(issues): [#2873] cite the branch commits by subject, not by id`. Copilot numbered them itself; the IDs are its own. Each comment says `[Minor]`; the review overview calls both "Medium severity", and this record keeps the comments' own label.

F3, F4 and F5 are da2ce7's review 5925607208, submitted at 18:52:17 UTC on the same head, in that source order. It was submitted as `APPROVED` ("None blocks"); the push of 19:00 UTC dismissed it, so its state is now `DISMISSED`. His IDs F1, F2 and F3 collide with Copilot's F1 and F2, so they are numbered F3, F4 and F5, and his IDs are kept as the reviewer finding IDs. The fix commit for F3 cites `review-finding:pr-2893-f1` "in his numbering"; it means this record's F3.

F6 to F9 are da2ce7's review 5925869377 (`CHANGES_REQUESTED`), submitted at 19:10:37 UTC on the head commit `docs(pr-reviews): [#2873] record Copilot's first review of PR #2893`, in that source order. His IDs F4 to F7 collide with F4 and F5 above, so they are numbered F6 to F9, and his IDs are kept as the reviewer finding IDs. The fix commit for F6 cites "his F4"; it means this record's F6.

F10 and F11 are da2ce7's review 5925989673 (`CHANGES_REQUESTED`), submitted at 19:18:18 UTC on the head commit `docs(issues): [#2873] address da2ce7's first review of PR #2893`, in that source order. His IDs F8 and F9 collide with F8 and F9 above, so they are numbered F10 and F11, and his IDs are kept as the reviewer finding IDs.

F12 and F13 are da2ce7's review 5926183635 (`COMMENTED`), submitted at 19:30:45 UTC on the head commit `docs(issues): [#2873] address da2ce7's third review of PR #2893`, in that source order. His IDs F10 and F11 collide with F10 and F11 above, so they are numbered F12 and F13, and his IDs are kept as the reviewer finding IDs. F13 follows up F11: its fix explained one gap in the totals and left another.

F14 is the one finding of da2ce7's review 5926341211 (`COMMENTED`), submitted at 19:41:31 UTC on the head commit `docs(issues): [#2873] address da2ce7's fourth review of PR #2893`. His ID F12 collides with F12 above, so it is numbered F14, and his ID is kept as the reviewer finding ID.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2893-f1 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2893-f2 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2893-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2893-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2893-f5 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2893-f6 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2893-f7 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2893-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2893-f9 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2893-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2893-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2893-f12 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2893-f13 | Human | Nit | documentation | RE_RAISE_OF:F11 | FIXED | RESOLVED |
| F14 | review-finding:pr-2893-f14 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Recognize white space and quoted description keys

- PR number: 2893
- Source review ID: 5925597715
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590040033
- Concern: The raw reader matched only `description:`, so `description : ...` and `"description": ...` were not read; YAML cuts both at a comment, yet neither rule reported them.
- Solution: The key is matched plain or quoted (double or single), with optional white space before the `:`. Unit tests for the three forms; mutations N1 and N2 in the evidence.
- Current-tree verification: `description_field_value` in `contrib/dev-tools/checks/frontmatter-validator/src/skills.rs`; `it_should_report_a_cut_plain_description_however_its_key_and_properties_are_written` cases 1 to 3.
- Resolution reference: fix(frontmatter-validator): [#2873] find the description key and its value start as YAML does
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590139698

### F2 - Determine the scalar style after comments, tags and anchors

- PR number: 2893
- Source review ID: 5925597715
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590039964
- Concern: The style was read from the first character after the colon, so `!!str "..."`, an anchor before a quoted or block value, or a comment ending the key's line before a block value made a valid description look plain and cut, which the hook and CI would reject.
- Solution: The value's start skips `!` tags, `&` anchors and a comment on the key's line before its style is read; an alias (`*`) is not plain. Unit tests for a tag, an anchor and a comment, before both plain and quoted or block values; mutations N3 and N4 in the evidence.
- Current-tree verification: `plain_description` and `without_node_properties` in `src/skills.rs`; `it_should_accept_a_whole_description_however_its_key_and_properties_are_written` cases 1 to 3 and the reporting cases 4 to 6.
- Resolution reference: fix(frontmatter-validator): [#2873] find the description key and its value start as YAML does
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590140260

### F3 - Thirteen descriptions do not end in `Use when`

- PR number: 2893
- Source review ID: 5925607208
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590047503
- Concern: The AC2 evidence says every new description is a "what" sentence then `Use when`, but 13 use `Use for`, `Use after`, `Use before`, `Use to` or `Use only when`, or no "when" (the two redirects), and four keep `project`, which the Trigger words rule drops.
- Solution: The 16 descriptions are reworded to the rule, so the Shape rule, the AC2 row and the area commit bodies hold as written. The listing and the eight M3 prompts were measured again on the new wording.
- Current-tree verification: every parsed description contains `Use when` and none has the word `project`; `description-measurements.md`, evidence V1 step 4 and V3 step 4.
- Resolution reference: docs(skills): [#2873] word every skill description to the Use when shape
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590259101

### F4 - The completion review leaves out the skipped review stop

- PR number: 2893
- Source review ID: 5925607208
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590047515
- Concern: The progress-log entry that is the implementation completion review gives only the two discoveries as its reason for no retrospective, so the issue folder reads as if the plan was followed; the stop for maintainer review after the last test increment was skipped.
- Solution: The 18:14 UTC entry now names that deviation and why no retrospective was written.
- Current-tree verification: the 2026-11-16 18:14 UTC progress-log entry of the spec.
- Resolution reference: docs(issues): [#2873] address da2ce7's first review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590259656

### F5 - Quoted texts render altered

- PR number: 2893
- Source review ID: 5925607208
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590047521
- Concern: `torrust-<crate>`, `it*should*\*` and the bare `#1, #2, #3` in quoted "Before" texts are Markdown-active, so the rendered record shows altered text.
- Solution: Every quoted description text in `description-measurements.md` is a code span, with double backticks where the text holds a backtick.
- Current-tree verification: each `Before` and `After` item in `description-measurements.md` is a code span.
- Resolution reference: docs(issues): [#2873] address da2ce7's first review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590260155

### F6 - A description that is only a comment is no longer reported

- PR number: 2893
- Source review ID: 5925869377
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590246278
- Concern: After the F1 and F2 fix the reader skips a comment on the key's line, finds no value and returns nothing, so `description: #NUMBER ...` (or that text on the next indented line, or after a tag), which YAML reads as null, passes both rules, the hook and Docs Lint.
- Solution: The reader tells a value that is only a comment from an empty or non-plain one, and the rule reports it. Three reporting cases; mutation O1 in the evidence.
- Current-tree verification: `RawDescription::OnlyAComment` in `src/skills.rs`; `it_should_report_a_cut_plain_description_however_its_key_and_properties_are_written` cases 7 to 9; `plain_after_a_comment` stays accepted.
- Resolution reference: fix(frontmatter-validator): [#2873] report a skill description that is only a comment
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590351624

### F7 - The N5 row records tests its mutation cannot fail

- PR number: 2893
- Source review ID: 5925869377
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590246291
- Concern: The N5 row says all 11 reporting tests failed, but a mutation that matches the key without its `:` cannot fail them; the recorded run was of a mutation that stopped every key from matching.
- Solution: N5 was run again as its row states; only `another_key_with_the_same_start` fails, and the row says so, with a note on the first, mislabeled run.
- Current-tree verification: the N5 and O1 rows of the evidence's T2 section.
- Resolution reference: docs(issues): [#2873] address da2ce7's second review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590352056

### F8 - The audit record covers only the bot review

- PR number: 2893
- Source review ID: 5925869377
- Reviewer finding ID: F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590246301
- Concern: At the reviewed head the record's two Reply URLs said `pending`, and review 5925607208's three findings had no row and no reply.
- Solution: The push of 19:11 UTC added F3 to F5 with replies; this round fills in every Reply URL. Each finding of every review has a row and a reply.
- Current-tree verification: the Findings table and every `Reply URL` field of this record.
- Resolution reference: docs(issues): [#2873] address da2ce7's second review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590352460

### F9 - The PR description gives a stale test count

- PR number: 2893
- Source review ID: 5925869377
- Reviewer finding ID: F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590246317
- Concern: The PR description says 243 tests pass and does not mention the reader fix; the merge tool copies the description into the merge commit.
- Solution: The PR description gives the test count from this head (257) and lists the reader fixes and the rewording.
- Current-tree verification: the PR description, edited at 19:15 UTC.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590352901
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590352901

### F10 - Stamps later than their commit, and the rewording put after the re-measure

- PR number: 2893
- Source review ID: 5925989673
- Reviewer finding ID: F8
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590334716
- Concern: The commit `docs(issues): [#2873] address da2ce7's first review of PR #2893`, made at 19:07:29 UTC, stamps `last-updated-utc` 19:10 and 19:08 and two entries 19:10; and this record said the rewording was committed after the re-measure, which ran on a checkout of it.
- Solution: The spec's progress-log entry and the measurements file give 19:07; this record's entry is split into the rewording commit (19:02), the re-measure (19:05) and the commit of the numbers (19:07), and says it was corrected.
- Current-tree verification: the 2026-11-16 19:07 UTC progress-log entry of the spec; `last-updated-utc` of `description-measurements.md`; the 19:02, 19:05 and 19:07 entries of this record's Processing Log.
- Resolution reference: docs(issues): [#2873] address da2ce7's third review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590412489

### F11 - The after-rewording total is unexplained

- PR number: 2893
- Source review ID: 5925989673
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590334733
- Concern: The after-rewording run totals 32.9k, the same as the before run, while the first after run totals 30.7k and Skills moved only 0.1k.
- Solution: The total was read from that run's output, not copied. The difference is the MCP rows: the 19:02:47 UTC run lists `MCP tools` (559) and `MCP server instructions` (1.6k), the 18:04:51 UTC run lists neither. The evidence's V1 now records this.
- Current-tree verification: the paragraph after the after-rewording numbers in the evidence's V1.
- Resolution reference: docs(issues): [#2873] address da2ce7's third review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590412952

### F12 - The F9 row's resolution reference is neither a commit subject nor a URL

- PR number: 2893
- Source review ID: 5926183635
- Reviewer finding ID: F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590473132
- Concern: The F9 row's resolution reference said "PR description edit, 2026-11-16 19:15 UTC", which the audit validator rejects; for a fix outside the tree the `process-pr-review` skill asks for the durable URL of the response that states it.
- Solution: The F9 row now cites its reply, which states the PR description edit. The F10 and F11 Reply URLs are filled in too.
- Current-tree verification: the `Resolution reference` of F9 and the `Reply URL` of F10 and F11 in this record.
- Resolution reference: docs(issues): [#2873] address da2ce7's fourth review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590544327

### F13 - The before run's total is unexplained

- PR number: 2893
- Source review ID: 5926183635
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590473144
- Concern: With the MCP rows in both, the before run and the reworded run total 32.9k although Skills differs by 2.0k, and nothing names the category that makes up the difference.
- Solution: `System tools` is 13.1k in the before run and 15.2k and 15.1k in the after runs; V1 names it and says the totals depend on the session, so the conclusions compare only the Skills category and its rows.
- Current-tree verification: the paragraph after the after-rewording numbers in the evidence's V1.
- Resolution reference: docs(issues): [#2873] address da2ce7's fourth review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590544893

### F14 - The paragraph introducing F12 and F13 appears twice

- PR number: 2893
- Source review ID: 5926341211
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590593053
- Concern: The commit for review 5926183635 added the paragraph that introduces F12 and F13 twice, almost word for word.
- Solution: The shorter copy is removed; one paragraph remains.
- Current-tree verification: the Findings section of this record has one paragraph starting `F12 and F13 are`.
- Resolution reference: docs(pr-reviews): [#2873] address da2ce7's fifth review of PR #2893
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2893#discussion_r4590649499

## Processing Log

- 2026-11-16 18:51 UTC - Review 5925597715 (Copilot, `COMMENTED`) submitted on the head commit `docs(issues): [#2873] cite the branch commits by subject, not by id`: F1 and F2, both Minor.
- 2026-11-16 18:55 UTC - Committed the fix for F1 and F2 (18:55:25 UTC), with the spec, evidence and this record in a second commit. The fix keeps the hand-written raw reader instead of adding a YAML event parser as a dependency.
- 2026-11-16 19:00 UTC - Pushed both commits (push finished 19:00:39 UTC); the push dismissed da2ce7's review 5925607208, which had arrived at 18:52 UTC and had not been read yet. Replied to F1 and F2 (19:00:50 and 19:00:53 UTC).
- 2026-11-16 19:01 UTC - Resolved the review threads in a loop over every unresolved thread, which also resolved da2ce7's three threads before they were read or addressed. Read them and unresolved all three at 19:01:31 UTC. F1 and F2 stay resolved.
- 2026-11-16 19:02 UTC - Committed the F3 fix, rewording 16 descriptions (19:02:09 UTC).
- 2026-11-16 19:05 UTC - Re-measured the listing and the M3 prompts in a fresh checkout of that commit (19:02:47 to 19:05:27 UTC).
- 2026-11-16 19:07 UTC - Committed the F4 and F5 fixes and the new numbers (19:07:29 UTC). This entry was first stamped 19:10 UTC, after its commit, and put the F3 commit after the re-measure; F10 corrected both.
- 2026-11-16 19:10 UTC - Review 5925869377 (da2ce7, `CHANGES_REQUESTED`) submitted on the head commit `docs(pr-reviews): [#2873] record Copilot's first review of PR #2893`: F6 (Major) to F9.
- 2026-11-16 19:11 UTC - Pushed the F3 to F5 fixes (push finished 19:11:36 UTC), replied to their threads and resolved those three by ID. A refresh then showed review 5925869377's four threads.
- 2026-11-16 19:15 UTC - Committed the F6 fix and the F7 to F9 fixes; edited the PR description for F9.
- 2026-11-16 19:18 UTC - Review 5925989673 (da2ce7, `CHANGES_REQUESTED`) submitted on the head commit `docs(issues): [#2873] address da2ce7's first review of PR #2893`: F10 and F11, both Nits.
- 2026-11-16 19:19 UTC - Pushed the F6 to F9 fixes (push finished 19:19:33 UTC), replied to their four threads and resolved them by ID; a refresh showed review 5925989673's two threads.
- 2026-11-16 19:21 UTC - Committed the F10 and F11 fixes, with the F6 to F9 Reply URLs.
- 2026-11-16 19:25 UTC - Pushed them (push finished 19:25:12 UTC), replied to the two threads and resolved them by ID; no thread was unresolved.
- 2026-11-16 19:30 UTC - Review 5926183635 (da2ce7, `COMMENTED`) submitted on the head commit `docs(issues): [#2873] address da2ce7's third review of PR #2893`: F12 (Minor) and F13 (Nit), neither blocking; the audit validator, run offline by him, failed on F9's resolution reference and the pending F10 and F11 Reply URLs.
- 2026-11-16 19:32 UTC - Committed the F12 and F13 fixes, with the F10 and F11 Reply URLs.
- 2026-11-16 19:37 UTC - Pushed them (push finished 19:37:04 UTC), replied to the two threads and resolved them by ID.
- 2026-11-16 19:37 UTC - Filled in the F12 and F13 Reply URLs in a separate commit, so no row is left pending.
- 2026-11-16 19:41 UTC - Review 5926341211 (da2ce7, `COMMENTED`) submitted on the head commit `docs(issues): [#2873] address da2ce7's fourth review of PR #2893`: F14, a Nit; the two pending Reply URLs it notes were filled in by the commit pushed at 19:41:32 UTC.
- 2026-11-16 19:42 UTC - Committed the F14 fix.
- 2026-11-16 19:46 UTC - Pushed it (push finished 19:46:38 UTC), replied to the thread and resolved it.
- 2026-11-16 19:46 UTC - Filled in the F14 Reply URL in a separate commit.

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
