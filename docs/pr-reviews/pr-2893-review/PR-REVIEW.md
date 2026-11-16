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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2893-f1 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2893-f2 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2893-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2893-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2893-f5 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |

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
- Reply URL: pending: posted after the push of this record

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
- Reply URL: pending: posted after the push of this record

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
- Reply URL: pending: posted after the push of this record

## Processing Log

- 2026-11-16 18:51 UTC - Review 5925597715 (Copilot, `COMMENTED`) submitted on the head commit `docs(issues): [#2873] cite the branch commits by subject, not by id`: F1 and F2, both Minor.
- 2026-11-16 18:55 UTC - Committed the fix for F1 and F2 (18:55:25 UTC), with the spec, evidence and this record in a second commit. The fix keeps the hand-written raw reader instead of adding a YAML event parser as a dependency.
- 2026-11-16 19:00 UTC - Pushed both commits (push finished 19:00:39 UTC); the push dismissed da2ce7's review 5925607208, which had arrived at 18:52 UTC and had not been read yet. Replied to F1 and F2 (19:00:50 and 19:00:53 UTC).
- 2026-11-16 19:01 UTC - Resolved the review threads in a loop over every unresolved thread, which also resolved da2ce7's three threads before they were read or addressed. Read them and unresolved all three at 19:01:31 UTC. F1 and F2 stay resolved.
- 2026-11-16 19:10 UTC - Committed the F3 fix (rewording 16 descriptions) and the F4 and F5 fixes, after re-measuring the listing and the M3 prompts on the reworded commit (19:02 to 19:05 UTC).

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
