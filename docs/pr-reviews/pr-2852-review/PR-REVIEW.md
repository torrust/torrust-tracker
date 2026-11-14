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
audit record (this record's own gap, F9). Round 3 is two reviews submitted on the head whose last
commit had the subject `docs(issues): [#2824] create the PR 2852 review audit record`: Copilot's
review `5889997099` (F12 to F14; its agentic review did not start before its timeout, so this is
its lighter, non-agentic pass) and da2ce7's review `5889756621` (dismissed, but its five threads
stayed open and unanswered; its own F12 to F16, recorded here as F15 to F19 since F12 to F14 were
already used for Copilot's findings — the collision F20 raises). Round 4 is da2ce7's review
`5890207451` (its own F17 to F19, recorded here as F20 to F22), submitted on the head whose last
commit had the subject `test(primitives): [#2824] compare serialized JSON by value, not by string`.
Round 5 is da2ce7's review `5899160108` (approved, none blocking; its own F20 to F24, recorded here
as F23 to F27), submitted on the head whose last commit had the subject
`docs(issues): [#2824] fix the PR 2852 audit record for rounds 3 and 4`.

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
| F12 | review-finding:pr-2852-f12 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2852-f13 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2852-f14 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2852-f15 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2852-f16 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2852-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2852-f18 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2852-f19 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2852-f20 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2852-f21 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F22 | review-finding:pr-2852-f22 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2852-f23 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2852-f24 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2852-f25 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2852-f26 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F27 | review-finding:pr-2852-f27 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

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
- Concern: The per-file deltas summed to +197 / +127 while the package moved +196 / +126. At first
  run the `announce_interval.rs` step matched (package and file both +35 / +26), and the extra line
  was that file's own figure, 114 / 114; the steps that did not match were `announce.rs` (package
  +52 / +38 against file +55 / +39) and `policy.rs` (package +42 against file +40 covered lines).
- Solution: Re-measured coverage at each of the five steps' own commit (`announce_interval.rs`,
  `announce.rs`, `policy.rs`, `mode.rs`, `pagination.rs`) and found two causes: a clippy rewrite
  after the first `announce_interval.rs` measure removed one covered line, and one `announce.rs`-step
  run read `readiness.rs` (untouched by this slice) at 50 / 52 instead of 52 / 52. Corrected the
  records' figures to the re-measured values and added a reconciliation note to
  `coverage-evidence.md` that lists all five steps agreeing and flags the `readiness.rs` variance for
  the S4 record.
- Current-tree verification: Re-ran `cargo llvm-cov` at each of the five steps' commit
  (`test(primitives): [#2824] test the announce interval conversions and bound`,
  `test(primitives): [#2824] test the announce policy defaults and serde`,
  `test(primitives): [#2824] test the tracker policy defaults and serde`, and the `mode.rs` and
  `pagination.rs` commits, which already matched); summed the corrected per-file deltas against the
  package total in `coverage-evidence.md`.
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
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562315531
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

### F12 - The announce.rs serialization test is sensitive to non-semantic field-order changes

- PR number: 2852
- Source review ID: 5889997099
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562622273
- Concern: `it_should_serialize_to_the_three_configuration_keys` asserted the exact JSON string, which
  would fail on a non-semantic serialization change such as struct field reordering, even though the
  derived field order is deterministic today.
- Solution: Changed the assertion to `serde_json::to_value` compared against a `serde_json::json!`
  literal (structural equality, order-independent), keeping every key and value pinned. Ran the
  key-rename mutant (`max_peers_per_announce` renamed in JSON; new, not a repeat — `announce.md`'s
  own mutation sample has no key-rename mutant) and confirmed it still fails the test.
- Current-tree verification: `cargo test -p torrust-tracker-primitives --lib announce::`; applied the
  key-rename mutant and reverted with `cmp`.
- Resolution reference: test(primitives): [#2824] compare serialized JSON by value, not by string
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562655115

### F13 - The mode.rs serialization test is sensitive to non-semantic field-order changes

- PR number: 2852
- Source review ID: 5889997099
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562622326
- Concern: Same concern as F12, for `it_should_serialize_to_the_configuration_key` in `mode.rs`.
- Solution: Same fix as F12. Re-ran the key-rename mutant (`check_keys_expiration` renamed in JSON)
  and confirmed it still fails the test.
- Current-tree verification: `cargo test -p torrust-tracker-primitives --lib mode::`; re-applied the
  key-rename mutant and reverted with `cmp`.
- Resolution reference: test(primitives): [#2824] compare serialized JSON by value, not by string
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562655456

### F14 - The policy.rs serialization test is sensitive to non-semantic field-order changes

- PR number: 2852
- Source review ID: 5889997099
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562622378
- Concern: Same concern as F12, for `it_should_serialize_to_the_three_configuration_keys` in
  `policy.rs`.
- Solution: Same fix as F12. Re-ran the key-rename mutant (`remove_peerless_torrents` renamed in
  JSON) and confirmed it still fails the test.
- Current-tree verification: `cargo test -p torrust-tracker-primitives --lib policy::`; re-applied
  the key-rename mutant and reverted with `cmp`.
- Resolution reference: test(primitives): [#2824] compare serialized JSON by value, not by string
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562655772

### F15 - The Processing Log's stamp predates what the entry describes

- PR number: 2852
- Source review ID: 5889756621
- Reviewer finding ID: F12
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562435901
- Concern: The Processing Log's first entry was stamped "09:50 UTC", which matches no git or forge
  time and predates the review (10:21:40Z), the replies (10:42:22Z to 10:43:01Z) and the commit that
  carries the entry (10:44:04Z).
- Solution: Appended a Processing Log correction giving the entry's actual time, 10:44 UTC, the
  commit time of `docs(issues): [#2824] create the PR 2852 review audit record`. (The round-4 fix
  re-stamped the entry in place instead; F24 restored it.)
- Current-tree verification: `git log --fixed-strings --grep='create the PR 2852 review audit
  record' --format='%cI'` reads `2026-11-13T10:44:04Z`.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569432463

### F16 - F8's Resolution reference is a commit subject, not the durable reply URL the template asks for `FOLLOW_UP`

- PR number: 2852
- Source review ID: 5889756621
- Reviewer finding ID: F13
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562435907
- Concern: F8 is `FOLLOW_UP`, but its Resolution reference cited a commit subject instead of the
  durable reply URL the row already carries in its Reply URL field.
- Solution: Changed F8's Resolution reference to its own Reply URL
  (https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562315531).
- Current-tree verification: Read F8's row in this file.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for rounds 3 and 4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569432842

### F17 - The F3 entry does not match the other records of the same fix

- PR number: 2852
- Source review ID: 5889756621
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562435916
- Concern: F3's verification named three re-run commits, while `coverage-evidence.md` and the F3
  reply say every one of the five steps was re-measured at its own commit; F3's concern also said
  two steps disagreed "by one line each", while at first run `announce.rs` differed by 3 covered
  lines and 1 line (package +52 / +38 against file +55 / +39) and `policy.rs` by 2 covered lines
  (package +42 against file +40).
- Solution: Reworded F3's Concern, Solution and Current-tree verification to name all five steps and
  the per-step figures. The round-4 wording named the wrong step and left out `policy.rs`; F25
  corrected it.
- Current-tree verification: Read the corrected F3 entry and `coverage-evidence.md`'s reconciliation
  paragraph.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for rounds 3 and 4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569433162

### F18 - A citation was left in the superseded form

- PR number: 2852
- Source review ID: 5889756621
- Reviewer finding ID: F15
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562435925
- Concern: `lessons.md`'s Follow-Up Candidates table still cited "(review of #2852, F8)"; F9's
  solution said the three in-record citations were switched to the canonical
  `review-finding:pr-2852-fN` form, but this fourth one, outside the record itself, was missed.
- Solution: Switched the `lessons.md` citation to `` `review-finding:pr-2852-f8` ``.
- Current-tree verification: `grep -n "review of #2852"` the issue's docs; no match remains.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for rounds 3 and 4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569433427

### F19 - The `last-updated-utc` stamps lag the commit that edited them

- PR number: 2852
- Source review ID: 5889756621
- Reviewer finding ID: F16
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562435939
- Concern: The commit `docs(issues): [#2824] create the PR 2852 review audit record` (10:44:04Z)
  edited `ISSUE.md`'s round-1 log entry and
  `announce_interval.md`'s two citations, but both files' `last-updated-utc` stayed at their earlier
  stamps (09:41 and 09:44) — the F7 and F10 rule applied to this commit's own edits.
- Solution: Advanced `ISSUE.md`'s and `announce_interval.md`'s `last-updated-utc` to
  "2026-11-13 22:30", the time of the edit; the commit followed at 22:35:14Z.
- Current-tree verification: `grep -n last-updated-utc` both files.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for rounds 3 and 4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569433695

### F20 - Review `5889756621` is not in the record, and its finding IDs collide with the record's

- PR number: 2852
- Source review ID: 5890207451
- Reviewer finding ID: F17
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562757449
- Concern: Review `5889756621` (10:56:22Z), dismissed but with five unresolved threads, raised its
  own F12 to F16 on this commit's parent; none had a row or a reply. This record's own F12 to F14
  named Copilot's later findings instead, so the forge's F12 (the log-stamp concern) and this
  record's F12 (a JSON-serialization concern) were different findings under the same number.
- Solution: Added rows and detail entries F15 to F19 for review `5889756621`'s five findings (the
  next free audit IDs, since F12 to F14 were already used), named both round-3 reviews in the
  record's framing paragraph, and replied to and resolved all five threads.
- Current-tree verification: Read the Findings table and the round-3 paragraph above.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for rounds 3 and 4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569434649

### F21 - The round-3 fix commit carried both the test change and the audit update

- PR number: 2852
- Source review ID: 5890207451
- Reviewer finding ID: F18
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562757457
- Concern: `process-pr-review` asks for the audit to be committed separately from product fixes;
  the commit `test(primitives): [#2824] compare serialized JSON by value, not by string` carried
  both the three serialization-test changes and the F12 to F14 audit rows.
- Solution: No change. That commit is pushed and reviewed, and splitting it would rewrite published
  history; the later rounds are documentation-only commits and need no split. Recorded as
  `NO_ACTION` (F26; the round-4 fix had recorded it as `FIXED`).
- Current-tree verification: `git show --stat` of the two later round commits lists only `docs/`
  paths.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4572459439
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4572459439

### F22 - The audit says a key-rename mutant was re-run for `announce.rs`; its record lists none

- PR number: 2852
- Source review ID: 5890207451
- Reviewer finding ID: F19
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4562757463
- Concern: F12's Solution said the key-rename mutant was "re-run", but `announce.md`'s own seven-
  mutant sample has no key-rename mutant (unlike `mode.md` and `policy.md`, which both have one), so
  this run was new, not a repeat.
- Solution: Reworded F12's Solution from "Re-ran" to "Ran", noting it is new rather than a repeat,
  and amended the already-posted reply making the same claim. F12's Current-tree verification kept
  "re-applied" until F25 corrected it.
- Current-tree verification: Read the corrected F12 entry and `announce.md`'s mutation sample table.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for rounds 3 and 4
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569435327

### F23 - The record cites PR-branch commit ids

- PR number: 2852
- Source review ID: 5899160108
- Reviewer finding ID: F20
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569561786
- Concern: F15, F19 and F21 cited the round-3 and round-4 heads by short commit id, which a rebase
  can change; `process-pr-review` asks for the unique commit subject.
- Solution: Replaced the three ids with the commits' subjects; F15's command now finds the commit by
  its subject with `git log --fixed-strings --grep`.
- Current-tree verification: `grep -nE '\b[0-9a-f]{9}\b'` this record matches no commit id.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4572459601

### F24 - The Processing Log was rewritten in place, and the new entry's stamp predates its events

- PR number: 2852
- Source review ID: 5899160108
- Reviewer finding ID: F21
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569561792
- Concern: The round-4 fix re-stamped the first entry (09:50 to 10:44) and reworded the 11:20 entry;
  the template says "Append entries only". Its own 22:30 entry predates the replies (22:33:52Z to
  22:34:14Z) and the commit (22:35:14Z).
- Solution: Restored the first two entries to their earlier text and appended one correction that
  names both rewrites, gives the first entry's actual time and the 22:30 entry's forge times. The
  22:30 entry itself is left as written.
- Current-tree verification: Compared the first two entries with the record at the commit
  `test(primitives): [#2824] compare serialized JSON by value, not by string`.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4572459953

### F25 - Three statements in the round-4 entries go beyond the bytes

- PR number: 2852
- Source review ID: 5899160108
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569561798
- Concern: (1) F3 and F17 said the `announce_interval.rs` step was off and omitted `policy.rs`; at
  first run that step matched, and the mismatched steps were `announce.rs` and `policy.rs`. (2) F22
  said F12's Current-tree verification was reworded, but it still read "re-applied". (3) F19 said the
  stamps moved to "this commit's time"; they read 22:30 and the commit is 22:35:14Z.
- Solution: (1) Rewrote F3's and F17's concerns with the per-step figures. (2) Reworded F12's
  verification to "applied" and F22's solution to say so. (3) Reworded F19's solution to give the
  stamp and the commit time.
- Current-tree verification: Recomputed the first-run deltas from the figures the round-1 fix
  replaced (`announce.rs` 883 / 1,021 to 935 / 1,059; `policy.rs` 935 / 1,059 to 977 / 1,083); read
  F3, F12, F17, F19 and F22.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4572460261

### F26 - A no-change concern is recorded as `FIXED`

- PR number: 2852
- Source review ID: 5899160108
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569561801
- Concern: F21 changed nothing, yet was recorded `FIXED` with a commit subject as its reference; the
  status rules reserve `NO_ACTION` / `SUPERSEDED` for a no-change concern, with the reply URL as the
  reference.
- Solution: Recorded F21 as `NO_ACTION` / `SUPERSEDED`, posted the prescribed `Superseded by` reply on
  its thread, and cited that reply as its Resolution reference and Reply URL.
- Current-tree verification: Read F21's row and entry.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4572460484

### F27 - Three edited records keep their 09:44 stamp

- PR number: 2852
- Source review ID: 5899160108
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4569561806
- Concern: `announce.md`, `mode.md` and `policy.md` still read "2026-11-13 09:44", although the commit
  `test(primitives): [#2824] compare serialized JSON by value, not by string` (11:21:26Z) edited
  their prose-first rows.
- Solution: Advanced the three stamps to this round's edit time, taken right before the commit.
- Current-tree verification: `grep -n last-updated-utc` the three records.
- Resolution reference: docs(issues): [#2824] fix the PR 2852 audit record for round 5
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2852#discussion_r4572460709

## Processing Log

- 2026-11-13 09:50 UTC - Started this audit, after round 1 (F1 to F8) was already fixed and pushed
  without it (F9). Replying on all eleven threads and resolving them now.
- 2026-11-13 11:20 UTC - Round 3: Copilot's review `5889997099` (F12 to F14) fixed in one commit; replying on the three threads and resolving them now.
- 2026-11-13 22:30 UTC - Round 3 (da2ce7, dismissed review `5889756621`) and round 4 (da2ce7, review
  `5890207451`): fixed the numbering collision and the six other record-hygiene findings (F15 to
  F22) in one documentation-only commit; replying on all eight threads and resolving them now.
- 2026-11-14 08:08 UTC - Correction (F24): the round-4 fix rewrote two entries above in place, re-stamping
  the 09:50 entry to 10:44 and rewording the 11:20 entry; both are restored to their earlier text.
  The 09:50 entry's work was committed at 10:44:04Z. The 22:30 entry's replies were posted from
  22:33:52Z to 22:34:14Z and its commit is dated 22:35:14Z. Round 5 (da2ce7, review `5899160108`,
  approved): F23 to F27 fixed in one documentation-only commit and F21 changed to `NO_ACTION`; the six
  replies were posted from 08:07:44Z to 08:07:55Z and all five new threads are resolved.

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
