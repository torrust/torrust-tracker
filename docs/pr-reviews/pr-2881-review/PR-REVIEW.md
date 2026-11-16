---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2881 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2881.

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

The reviewer numbers the findings `F1` to `F11` across two reviews; the audit keeps those IDs.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2881-f1 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2881-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2881-f3 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2881-f4 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2881-f5 | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2881-f6 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2881-f7 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2881-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2881-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2881-f10 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2881-f11 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Add the discussion to the docs index

- PR number: 2881
- Source review ID: 5921158708
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586629281
- Concern: The new discussion had no row in the discussions table of `docs/index.md`, which lists every other discussion folder.
- Solution: Added a row for `discussions/2877-webtorrent-support/20261116-webtorrent-tracker-service/` naming issue #2877.
- Current-tree verification: `docs/index.md` has the row after the I2P discussion row; `linter lychee` passes.
- Resolution reference: docs(discussions): address review of the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586828766

### F2 - Do not present an edited research file as source material

- PR number: 2881
- Source review ID: 5921158708
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586629292
- Concern: The research file called itself source material but said it was edited for length, which the discussions rules do not allow for source material.
- Solution: The file now presents itself as the opening author's summary of the research, not source material kept as written.
- Current-tree verification: Lines 12-16 of `client-support-research.md` state that it condenses a research report and is not source material kept as written.
- Resolution reference: docs(discussions): address review of the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586829504

### F3 - Align the verification status of the 5.3 binaries

- PR number: 2881
- Source review ID: 5921158708
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586629299
- Concern: The README said "Not verified" for the 5.3 binaries while the research file said "Partly verified".
- Solution: The README now says "Partly verified" with the research file's qualifier.
- Current-tree verification: README lines 106-108 read "Partly verified" and name the maintainer's statement, the user's report and the unchecked binary.
- Resolution reference: docs(discussions): address review of the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586830220

### F4 - Qualify which qBittorrent 5.3 builds use libtorrent 2.1

- PR number: 2881
- Source review ID: 5921158708
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586629308
- Concern: The README said qBittorrent 5.3 uses libtorrent 2.1 without saying that 2.1 is the default and that the 1.2 builds have no WebTorrent.
- Solution: Used the reviewer's wording.
- Current-tree verification: README line 73 reads "libtorrent 2.1 by default; its libtorrent 1.2 builds have no WebTorrent".
- Resolution reference: docs(discussions): address review of the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586830821

### F5 - Link the I2P Q5 entry for the per-method grouping

- PR number: 2881
- Source review ID: 5921158708
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586629316
- Concern: Q2 attributed the one-group-per-addressing-method proposal to Q12 of the I2P discussion; it is proposed in the opening author's Q5 entry.
- Solution: Q2 now links the Q5 entry of the I2P design discussion.
- Current-tree verification: `linter lychee` resolves the fragment `#q5---jose-celano-josecelano-2026-10-19-pr-2562`.
- Resolution reference: docs(discussions): address review of the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586831590

### F6 - Drop unused spelling-ignore words

- PR number: 2881
- Source review ID: 5921158708
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586629322
- Concern: Both files had spelling-ignore words that do not occur in them.
- Solution: Kept only the words that occur in each file.
- Current-tree verification: Each listed word occurs in its file; `linter cspell` passes.
- Resolution reference: docs(discussions): address review of the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586832305

### F7 - Align the table rows

- PR number: 2881
- Source review ID: 5921158708
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586629334
- Concern: Some table rows had cell widths different from the header.
- Solution: Re-padded the tables in both files.
- Current-tree verification: Every row of the metadata table and the claims table has the header's widths.
- Resolution reference: docs(discussions): address review of the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586832902

### F8 - Add the review audit record

- PR number: 2881
- Source review ID: 5921530312
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586911092
- Concern: The round-1 threads were answered and resolved without the audit record that the process-pr-review skill requires.
- Solution: Added this record from the template, with rows for F1 to F11, and logged the late start.
- Current-tree verification: `validate-audit-record.py --pr-number 2881` exits 0 after the commit, before the push.
- Resolution reference: docs(pr-reviews): add the review audit for the WebTorrent discussion
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586948534

### F9 - Update the PR description

- PR number: 2881
- Source review ID: 5921530312
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586911114
- Concern: The PR description still called the research file source material and listed only two files.
- Solution: Edited the PR description: the Files section calls the research file the opening author's summary and lists `docs/index.md` and this audit record.
- Current-tree verification: `gh-bot pr view 2881 --json body` shows the four files and no "source material" wording.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586948908
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586948908

### F10 - Cite commit subjects, not ids, in replies

- PR number: 2881
- Source review ID: 5921530312
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586911133
- Concern: The seven round-1 replies cited a PR-branch commit id.
- Solution: Edited the seven replies to cite the commit subject only; this record and later replies cite subjects only.
- Current-tree verification: The seven replies on the round-1 threads cite only the subject.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586949332
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586949332

### F11 - Seven independent fixes landed in one commit

- PR number: 2881
- Source review ID: 5921530312
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586911150
- Concern: The fixes for F1 to F7 landed in one commit instead of one commit each.
- Solution: No change: the reviewer notes a rewrite is not needed; later fixes land one per commit.
- Current-tree verification: The reply states the decision.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586949830
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2881#discussion_r4586949830

## Processing Log

- 2026-11-16 14:14 UTC - Replied to and resolved the seven threads of review 5921158708 (F1 to F7) without an audit record.
- 2026-11-16 14:24 UTC - Edited the PR description (F9), edited the seven round-1 replies to cite the commit subject only (F10), and replied to the F8 to F11 threads of review 5921530312.
- 2026-11-16 14:25 UTC - Started audit late, after the replies above, as finding F8 asks; rows F1 to F11 recorded.

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
