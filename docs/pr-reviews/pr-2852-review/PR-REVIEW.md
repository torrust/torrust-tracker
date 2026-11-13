---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2852 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2852.

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

Round 1 is Copilot's review `5888862726` (no findings; approval recommended) and da2ce7's review
`5888917289` (F1 to F8), both submitted on the head whose last commit had the subject
`docs(issues): [#2824] record the S3 pull request number`. Round 2 is da2ce7's review `5889372966`
(F9 to F11), submitted on the head whose last commit had the subject
`docs(issues): [#2824] fix the S3 records after review round 1`, which fixed F1 to F8 without an
audit record (this record's own gap, F9).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2852-f1 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2852-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2852-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2852-f4 | Human | Minor | formatting | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2852-f5 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2852-f6 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2852-f7 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2852-f8 | Human | Suggestion | documentation | ORIGINAL | FOLLOW_UP | RESOLVED |
| F9 | review-finding:pr-2852-f9 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2852-f10 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2852-f11 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The description's mutation summary contradicts the records it ships with

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797045
- Concern: The description said one mutant survived and was equivalent, while the records list three
  survivors (`announce_interval.md`: the single-value bound slip and the `i64` read; `announce.md`: the
  `interval_min` serde default), and the description's claim that the property was kept under a
  pre-registered rule did not match the record: none of the three mutants named in advance met the
  rule, and the deciding band mutant was added afterward.
- Solution: Rewrote the PR description to name all three survivors with their status (one not
  equivalent, one equivalent, one equivalent by a different file), and to say that the deciding
  mutant was not one of the three pre-registered. Corrected the `announce_interval.md` property
  section to say the same.
- Current-tree verification: Read the rewritten description on the forge; read the "Property
  decision" and "Mutation sample" sections of `announce_interval.md`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562310596
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562310596

### F2 - The i64 mutant is not equivalent, and the record contradicts itself about it

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797067
- Concern: Reading an `i64` and casting to `u64` still rejects `-1`, but with a different,
  operator-visible error message naming a huge number; the test asserts only the failure, not the
  message, so it cannot tell the two forms apart. The record called this mutant equivalent and said a
  change here "would be caught", which do not agree.
- Solution: Changed the mutant's row to "survived, by design", explaining that the failure is the
  same but the message is not, and that only the failure is asserted. Corrected the file's negative-
  number test row to describe what it actually catches (a deserializer that clamps to zero).
- Current-tree verification: Read the mutation table and the "Deserializing a negative number" row of
  `announce_interval.md`.
- Resolution reference: docs(issues): [#2824] fix the S3 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562312344

### F3 - The per-file line figures do not reconcile with the package figures

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797088
- Concern: The per-file deltas summed to +197 / +127 while the package moved +196 / +126; two steps
  (`announce.rs`, `policy.rs`) disagreed with the package figure by one line each.
- Solution: Re-measured coverage at each step's own commit (`announce_interval.rs`, `announce.rs`,
  `policy.rs`) and found two causes: a clippy rewrite after the first `announce_interval.rs` measure
  removed one covered line, and one `announce.rs`-step run read `readiness.rs` (untouched by this
  slice) at 50 / 52 instead of 52 / 52. Corrected the three records' figures to the re-measured values
  and added a reconciliation note to `coverage-evidence.md` that shows every step agreeing and flags
  the `readiness.rs` variance for the S4 record.
- Current-tree verification: Re-ran `cargo llvm-cov` at the three commits
  (`test(primitives): [#2824] test the announce interval conversions and bound`,
  `test(primitives): [#2824] test the announce policy defaults and serde`,
  `test(primitives): [#2824] test the tracker policy defaults and serde`); summed the corrected
  per-file deltas against the package total in `coverage-evidence.md`.
- Resolution reference: docs(issues): [#2824] fix the S3 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562312845

### F4 - The description is hard-wrapped

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797093
- Concern: `open-pull-request` requires one continuous line per paragraph; the description wrapped at
  about 120 columns.
- Solution: Replaced the description with one written as continuous lines per paragraph.
- Current-tree verification: Read the description on the forge; no line break inside a paragraph.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562313222
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562313222

### F5 - A percentage does not recompute

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797101
- Concern: 1,406 / 1,553 is 90.534%, so 90.53%, not the 90.54% the record and `announce.md` both
  carried.
- Solution: Corrected both occurrences to 90.53%.
- Current-tree verification: Recomputed 1406 / 1553 = 0.905344...; grepped both records for the
  figure.
- Resolution reference: docs(issues): [#2824] fix the S3 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562313663

### F6 - The mode.rs line references are the base file's

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797107
- Concern: The doc-comment correction (Q1) adds one line, so at the head of the PR the `Default` impl,
  the serde attribute and the struct each start one line later than the record's Coverage And
  Decision Analysis table said; the same shift applies to `pagination.md`.
- Solution: Corrected the three ranges in `mode.md` and the one range in `pagination.md` to the
  current file's line numbers.
- Current-tree verification: `grep -n` the struct, the serde attribute and the `Default` impl of
  `mode.rs` and `pagination.rs` at the head, compared with the records.
- Resolution reference: docs(issues): [#2824] fix the S3 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562314358

### F7 - The issue frontmatter is not advanced

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797110
- Concern: `ISSUE.md`'s `last-updated-utc`, `branch` and `related-pr` still named the S1 slice,
  although S3 had added progress-log entries.
- Solution: Advanced `last-updated-utc`, set `branch` to
  `2824-1347-add-tests-primitives-defaults` and `related-pr` to 2852.
- Current-tree verification: Read the frontmatter block of `ISSUE.md`.
- Resolution reference: docs(issues): [#2824] fix the S3 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562314913

### F8 - The REST adapter's offset wording still speaks of pages

- PR number: 2852
- Source review ID: 5888917289
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4561797115
- Concern: `QueryParams::offset` in `packages/axum-rest-api-server/src/v1/context/torrent/handlers.rs`
  still says "The offset of the first page to return. Starts at 0.", although every consumer of
  `Pagination::offset` treats it as a number of items to skip; that file is outside this issue's
  scope.
- Solution: Recorded a `lessons.md` follow-up candidate naming the file, the line and the consumers,
  instead of changing a file outside this slice's ledger.
- Current-tree verification: Read the Follow-Up Candidates table of `lessons.md`.
- Resolution reference: docs(issues): [#2824] fix the S3 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562315531

### F9 - The round was processed without an audit record

- PR number: 2852
- Source review ID: 5889372966
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562139232
- Concern: Round 1's eight findings were fixed and pushed without this record, without a reply on any
  of the eight threads, and without resolving them; the records also cited findings as "review round
  1 of #2852, F3" instead of the skill's `review-finding:pr-2852-f3` reference, and the fix landed as
  one commit for seven independent findings (F1 to F7; F8 is a `lessons.md` row).
- Solution: Created this record with rows and detail entries for F1 to F11; switched the three
  in-record citations to the canonical `review-finding:pr-2852-fN` form; replying and resolving all
  eleven threads under this round's commit, per the Completion Rules (one combined commit, since the
  fixes were already pushed and are cited by subject here, not re-split).
- Current-tree verification: Ran the audit validator against this record after it was written.
- Resolution reference: docs(issues): [#2824] create the PR 2852 review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562316901

### F10 - The edited records keep their earlier last-updated-utc

- PR number: 2852
- Source review ID: 5889372966
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562139241
- Concern: The five records the round 1 fix commit edited kept their pre-round `last-updated-utc`
  stamps, the same defect F7 found in `ISSUE.md`.
- Solution: Advanced all five records' `last-updated-utc` to the fix commit's time (2026-11-13 09:44).
- Current-tree verification: `grep -n last-updated-utc` the five records.
- Resolution reference: docs(issues): [#2824] create the PR 2852 review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562317353

### F11 - The log entry wrongly credits the records with the survivor-count fix

- PR number: 2852
- Source review ID: 5889372966
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562139245
- Concern: The round-1 log entry said "the records were corrected (survivor count ...)", but no
  record's survivor count changed in that commit; only the description's count (outside the tree)
  changed.
- Solution: Split the entry into two clauses: the description was corrected for the survivor count
  and the property rule; the records were corrected for the remaining items.
- Current-tree verification: Read the corrected log entry in `ISSUE.md`.
- Resolution reference: docs(issues): [#2824] create the PR 2852 review audit record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562317666

## Processing Log

- 2026-11-13 09:50 UTC - Started this audit, after round 1 (F1 to F8) was already fixed and pushed
  without it (F9). Replying on all eleven threads and resolving them now.

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
