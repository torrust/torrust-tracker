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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2893-f1 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2893-f2 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

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
- Reply URL: pending: posted after the push of this record

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
- Reply URL: pending: posted after the push of this record

## Processing Log

- 2026-11-16 18:51 UTC - Review 5925597715 (Copilot, `COMMENTED`) submitted on the head commit `docs(issues): [#2873] cite the branch commits by subject, not by id`: F1 and F2, both Minor.
- 2026-11-16 18:55 UTC - Committed the fix for F1 and F2 (18:55:25 UTC), with the spec, evidence and this record in a second commit. The fix keeps the hand-written raw reader instead of adding a YAML event parser as a dependency.

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
