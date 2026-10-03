---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2003"
---

<!-- skill-link: process-pr-review -->

# PR #2428 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2428>.

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

Audit IDs `F1` to `F5` are the reviewer's own IDs from review 5400754664 by da2ce7 (round 1, an
`APPROVE` at the head `docs(issues): align EPIC #2264 and #2278 owner metadata`); each inline
comment carries a `[Nit]` bracket. The earlier Copilot review 5400634431 gives its one finding no
ID, so it takes the next free ordinal, `F6`, rather than `F1`: reserving the reviewer's IDs keeps
that review's body, its ACK comment, and its threads matching this record. The Copilot comment has
no severity bracket; the review overview rates it Medium, recorded as `Minor (inferred)` as the
`pr-2352-review` and `pr-2353-review` records do. F5 concurs with F6 on the same lines and asks
for the same change, so it is a re-raise of F6. The review body's verification sections and its
answers to the discussions' open questions request no change, so they have no rows.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2428-f1` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2428-f2` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2428-f3` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2428-f4` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2428-f5` | Human | Nit | correctness | RE_RAISE_OF:F6 | FIXED | RESOLVED |
| F6 | `review-finding:pr-2428-f6` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The folder guide's layout tree omitted the goals-and-boundaries discussion

- PR number: 2428
- Source review ID: 5400754664
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4173154651>
- Concern: The example tree in `docs/discussions/AGENTS.md` showed the folder as it stood before
  the goals-and-boundaries discussion was added, so it no longer matched the folder it described.
- Solution: Added the goals-and-boundaries folder to the tree and introduced the tree with "For
  example:", so the block does not have to track later discussions.
- Current-tree verification: `git grep -n -E "For example:|20261003-goals-and-boundaries/" HEAD -- docs/discussions/AGENTS.md`
  matches line 14 ("... then by start date and topic. For example:") and line 20
  ("├── 20261003-goals-and-boundaries/").
- Resolution reference: `docs(discussions): show both discussions in the folder guide layout`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4174495891>

### F2 - The new tables were not padded to their column widths

- PR number: 2428
- Source review ID: 5400754664
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4173154655>
- Concern: Rows of the Discussions table in `docs/index.md` overflowed the column widths its header
  set, unlike every other table in the index; three rows in the two discussion READMEs did the
  same.
- Solution: Padded every row of the Discussions table, the goals-and-boundaries header table, and
  the semantic-linking header and relationship tables to the widest cell in each column. The two
  link-corpus tables were already consistent and are unchanged.
- Current-tree verification: measuring every row of each table at the pushed head with
  `git show HEAD:<file> | awk` gives one length per table: 266 for `docs/index.md` from line 109,
  137 for the goals-and-boundaries header table, and 115 and 214 for the semantic-linking header
  and relationship tables.
- Resolution reference: `docs(discussions): pad the new discussion tables`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4174495961>

### F3 - The 148 labels already include the two CI findings

- PR number: 2428
- Source review ID: 5400754664
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4173154660>
- Concern: The semantic-linking README read as 148 friction labels plus two CI findings, while the
  register's 148 labels include the two CI findings.
- Solution: The commit that fixes F6 rewrote the sentence: the index held 148 labels, two of them
  CI findings rather than frictions.
- Current-tree verification: `git grep -n "148 labels" HEAD -- docs/discussions/2003-overhaul-guardrails-and-automation/20261003-semantic-linking-knowledge-graph/README.md`
  matches line 116 ("At their revision of 2026-10-03 10:50 UTC the index held 148 labels, two of
  them CI findings").
- Resolution reference: `docs(discussions): pin the EPIC #2003 thread evidence to its cutoff`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4174496049>

### F4 - "Most frequent defect class" was not what the register shows

- PR number: 2428
- Source review ID: 5400754664
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4173154662>
- Concern: Both discussions called two copies of one rule drifting apart the register's most
  frequent defect class; a keyword classification of the register's labels puts unstated or
  unchecked cases about threefold ahead.
- Solution: Both places now call it a recurring defect class, using the reviewer's wording.
- Current-tree verification: `git grep -n "most frequent" HEAD -- docs/discussions` finds nothing;
  "A recurring defect class" is on line 141 of the goals-and-boundaries README and line 145 of the
  semantic-linking README.
- Resolution reference: `docs(discussions): stop ranking the copy-drift defect class first`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4174496100>

### F5 - The comment count needed a time cutoff, not only a date

- PR number: 2428
- Source review ID: 5400754664
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4173154665>
- Concern: "All 54 comments on #2003 were read on 2026-10-03" stopped identifying the evidence once
  a 55th comment was posted the same day; the reviewer proposed a cutoff at the time the PR was
  opened.
- Solution: Fixed with F6. The section pins its evidence to the last comment it covers rather than
  to the PR-open time; see F6.
- Current-tree verification: `git grep -n -E "54 comments|5968442607|148 labels" HEAD -- docs/discussions/2003-overhaul-guardrails-and-automation/20261003-semantic-linking-knowledge-graph/README.md`
  matches lines 112, 113, and 116.
- Resolution reference: `docs(discussions): pin the EPIC #2003 thread evidence to its cutoff`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4174496202>

### F6 - The analysis of the mutable EPIC thread had no cutoff

- PR number: 2428
- Source review ID: 5400634431
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4173056409>
- Concern: The claim that all 54 comments were read was already stale: the thread had 55
  comments, and the 55th added five friction labels. The reviewer asked to pin the observation to
  its cutoff or update the analysis.
- Solution: Pinned rather than updated, because the register is edited in place and any count
  without a cutoff goes stale again. The section names the last comment it covers (5968442607,
  posted 2026-10-03 10:49 UTC) and the index revision its counts come from (2026-10-03 10:50 UTC),
  and states that later comments are not covered.
- Current-tree verification: `git grep -n -E "54 comments|5968442607|148 labels" HEAD -- docs/discussions/2003-overhaul-guardrails-and-automation/20261003-semantic-linking-knowledge-graph/README.md`
  matches lines 112, 113, and 116.
- Resolution reference: `docs(discussions): pin the EPIC #2003 thread evidence to its cutoff`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2428#discussion_r4174468146>

## Processing Log

- 2026-10-03 12:01 UTC - Copilot review 5400634431 posted one finding (F6).
- 2026-10-03 12:27 UTC - Review 5400754664 by da2ce7 approved the PR with five Nits (F1 to F5);
  ACK comment 5969156380 named the same head.
- 2026-10-03 19:05 UTC - Committed the F6 fix, which also fixes F3 and F5.
- 2026-10-03 19:06 UTC - Pushed it. GitHub dismissed the approval of review 5400754664 on that push.
- 2026-10-03 19:08 UTC - Replied on the F6 thread. The thread refresh that followed returned the
  five threads of review 5400754664, which the first fetch of this processing round had not
  returned.
- 2026-10-03 19:15 UTC - Committed the F1, F4, and F2 fixes, then pushed them.
- 2026-10-03 19:17 UTC - Replied on the F1 to F5 threads.
- 2026-10-03 19:20 UTC - `reply-status` reported every unresolved thread replied; resolved all six
  threads.
- 2026-10-03 19:21 UTC - Started this audit.

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
