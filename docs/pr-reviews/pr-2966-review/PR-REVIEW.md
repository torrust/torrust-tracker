---
pr-number: 2966
pr-url: https://github.com/torrust/torrust-tracker/pull/2966
last-updated-utc: "2026-11-19 12:13"
---

<!-- skill-link: process-pr-review -->

# PR #2966 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2966.

<!-- Copied verbatim into each audit record after placeholder replacement. -->

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

Round 2 is da2ce7's review `5974080148` (APPROVED, two inline findings, F10 and F11, on the head with the first version of this record).
Round 1 is da2ce7's review `5973096324` (APPROVED, seven inline findings, F1 to F7) and Copilot's review `5973108503` (COMMENTED, one finding, F8 here, a re-raise of F5), both on the first head, the commit `docs(issues): [#2768] close out the tracker_client JSON-only issue`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2966-f1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2966-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2966-f3` | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2966-f4` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2966-f5` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2966-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2966-f7` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2966-f8` | Copilot | Nit | formatting | RE_RAISE_OF:F5 | FIXED | RESOLVED |
| F10 | `review-finding:pr-2966-f10` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2966-f11` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The AC2 row cites G1 for 72 tests, but G1 records 71

- PR number: 2966
- Source review ID: 5973096324
- Reviewer finding ID: da2ce7 F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627251314
- Concern: The AC2 row cites G1 for 72 tests, but G1 records 71.
- Solution: AC2 now cites G1 for 71 and the PR 2956 audit (finding F1) for the 72-test run.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(issues): [#2768] correct the close-out after review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627882652

### F2 - The retrospective repeats the claim that one module owns every write, which review corrected in PR 2956

- PR number: 2966
- Source review ID: 5973096324
- Reviewer finding ID: da2ce7 F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627251328
- Concern: The retrospective repeats the claim that one module owns every write, which review corrected in PR 2956.
- Solution: Reworded the Outcome (the tracing subscriber writes its own events) and added the correction to What Changed.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(issues): [#2768] correct the close-out after review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627883919

### F3 - The prose-first test comparison is not recorded for the ten tests

- PR number: 2966
- Source review ID: 5973096324
- Reviewer finding ID: da2ce7 F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627251337
- Concern: The prose-first test comparison is not recorded for the ten tests.
- Solution: Recorded as a deviation in the retrospective and the spec's Completion Review; not reconstructed after review.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(issues): [#2768] correct the close-out after review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627883048

### F4 - The PR title keeps [#2768] for an already closed issue

- PR number: 2966
- Source review ID: 5973096324
- Reviewer finding ID: da2ce7 F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627251347
- Concern: The PR title keeps [#2768] for an already closed issue.
- Solution: Edited the PR title and wrote `Related to #2768` in the body.
- Current-tree verification: the title and body were read back after the edit (`gh pr view`); the fix is outside the tree.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627884832
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627884832

### F5 - Duplicated word "now" across a line break

- PR number: 2966
- Source review ID: 5973096324
- Reviewer finding ID: da2ce7 F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627251363
- Concern: Duplicated word "now" across a line break.
- Solution: Removed the duplicate.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(issues): [#2768] correct the close-out after review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627882250

### F6 - The template's closing Evidence section is missing

- PR number: 2966
- Source review ID: 5973096324
- Reviewer finding ID: da2ce7 F6
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627251378
- Concern: The template's closing Evidence section is missing.
- Solution: Added the Evidence section.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(issues): [#2768] correct the close-out after review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627884441

### F7 - Name the owner of improvement 1 and of the deferred schema version

- PR number: 2966
- Source review ID: 5973096324
- Reviewer finding ID: da2ce7 F7
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627251388
- Concern: Name the owner of improvement 1 and of the deferred schema version.
- Solution: Named the skill and template as owner of improvement 1 and said the maintainer decides on an issue; same for the schema version.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(issues): [#2768] correct the close-out after review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627883488

### F8 - Duplicated word "now" (same as F5)

- PR number: 2966
- Source review ID: 5973108503
- Reviewer finding ID: Copilot F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627260863
- Concern: Duplicated word "now" (same as F5).
- Solution: Same fix as F5.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(issues): [#2768] correct the close-out after review round 1`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627885338

### F10 - The record cites the PR-branch commit by id

- PR number: 2966
- Source review ID: 5974080148
- Reviewer finding ID: da2ce7 F10
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627996700
- Concern: The record cites the PR-branch commit by id.
- Solution: Named the first head by its commit subject.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(pr-reviews): [#2768] cite the first head by subject and F4 by its reply in the PR 2966 audit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4628043207

### F11 - F4's Resolution reference names a commit that does not carry the fix (a title and body edit)

- PR number: 2966
- Source review ID: 5974080148
- Reviewer finding ID: da2ce7 F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4627996704
- Concern: F4's Resolution reference names a commit that does not carry the fix (a title and body edit).
- Solution: F4's Resolution reference is the URL of the reply that states the edit.
- Current-tree verification: checked in the tree after the push of the fix commit; see the reply.
- Resolution reference: `docs(pr-reviews): [#2768] cite the first head by subject and F4 by its reply in the PR 2966 audit`
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2966#discussion_r4628043578

## Processing Log

- 2026-11-19 11:13 UTC - Started the audit after the replies of round 1, not before them (late start): the record was written once the reply URLs existed.
- 2026-11-19 11:13 UTC - Round 1: the reviews above (da2ce7 at 10:17:01Z, Copilot at 10:17:52Z). One commit fixed all eight (pushed, `push_exit=0`); the PR title and body were edited; eight replies were posted, then the eight threads were resolved (GraphQL: 0 unresolved). `validate-audit-record.py` was not run (it uses plain gh).
- 2026-11-19 11:24 UTC - Round 2: da2ce7's APPROVED review `5974080148` on the head with this record, with two inline findings (F10, F11 in his numbering; both Minor, about this record). Fixed in one commit: the first head is cited by subject, and F4 cites the reply that states the title edit. The threads were answered and resolved after the push. Fix round 2 of 5. Correction (added later): this line was committed before the two replies existed (they were posted at 11:27Z); F10 and F11 had no rows until the next round.
- 2026-11-19 12:13 UTC - Round 3: da2ce7's COMMENTED review `5974228751` (11:34:31Z) on head `docs(pr-reviews): [#2768] cite the first head by subject and F4 by its reply in the PR 2966 audit` with one Minor finding (F12 in his numbering): F10 and F11 had replies and resolved threads but no rows. Rows and details for F10 and F11 were added here, after their replies. The row for F12 follows once its reply exists. Fix round 3 of 5.

<!-- Copied verbatim into each audit record. -->

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
