---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

# PR #2841 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2841.

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

Round 1 is Copilot's review `5875845096` (F13, numbered after the others) and da2ce7's review `5875864302` (F1 to F8), both submitted on the head whose last commit had the subject `docs(issues): [#2824] record the S2 coverage and the F6 finding`, and da2ce7's review `5876129658` (F9 to F12), submitted on the head whose last commit has the subject `test(primitives): [#2824] cover the ReadInfo role predicates for every state`. Round 2 is da2ce7's review `5878329038` (F14 to F20), submitted on the head whose last commit has the subject `docs(pr-reviews): [#2824] correct a log stamp of the PR 2841 audit`, and Copilot's review `5878380545` (F21), submitted on the head of the rebuilt history (last commit `docs(pr-reviews): [#2824] correct the PR 2841 audit facts after round 2`). Round 3 is da2ce7's approving review `5879377639` (F22 to F24) and Copilot's review `5879536305` (F25 to F27). The branch history was rebuilt after round 2 (see F14), so the subjects of the round 1 fix commits are those listed in each `Resolution reference` below.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2841-f1 | Human | Major | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2841-f2 | Human | Minor | testing | RE_RAISE_OF:F13 | FIXED | RESOLVED |
| F3 | review-finding:pr-2841-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2841-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2841-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2841-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2841-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2841-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2841-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2841-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2841-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2841-f12 | Human | Nit | documentation | ORIGINAL | NO_ACTION | RESOLVED |
| F13 | review-finding:pr-2841-f13 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2841-f14 | Human | Major | maintainability | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2841-f15 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2841-f16 | Human | Minor | maintainability | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2841-f17 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2841-f18 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2841-f19 | Human | Nit | documentation | RE_RAISE_OF:F8 | FIXED | RESOLVED |
| F20 | review-finding:pr-2841-f20 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2841-f21 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2841-f22 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2841-f23 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2841-f24 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2841-f25 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F26 | review-finding:pr-2841-f26 | Copilot | Minor | documentation | RE_RAISE_OF:F22 | FIXED | RESOLVED |
| F27 | review-finding:pr-2841-f27 | Copilot | Minor | documentation | RE_RAISE_OF:F22 | FIXED | RESOLVED |

## Finding Details

### F1 - The commits do not match their messages

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530157
- Concern: The docs commit carried the `peer.rs` tests, the manifests and the lock, and the test commit carried only a TOML re-spacing, so history and blame would point at the wrong commits; the description repeated the mismatch and lacked a "Validation performed" part.
- Solution: Rewrote the unmerged branch (force-push with lease, with the maintainer's approval): the docs commit carried only the plan files and the test commit the code. Round 2 (F14, F16) found that the rewrite folded steps and put the record apart from its tests; the history was rebuilt from the pre-split commits (see F14).
- Current-tree verification: Listed the files of each rewritten commit with `git show --stat`; the test commit contains `peer.rs`, `packages/primitives/Cargo.toml` and `Cargo.lock`, the docs commit (before the round 2 rebuild) only the two plan files; after the rebuild it holds only `README.md` and `peer.md` is in the test commit.
- Resolution reference: test(primitives): [#2824] test the peer module
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553083185

### F2 - Each ReadInfo copy of the seeder rule is pinned on one side only

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530167
- Concern: The `ReadInfo` tests asserted only a leecher through one entry point and only a seeder through the other, so replacing the trait `is_seeder` by a constant or its `<=` by `==` survived.
- Solution: Added a case table (positive, zero and negative `left` with `Started`; zero and negative `left` with `Stopped`) asserting both `is_seeder` and `is_leecher` through the trait, on `Peer` and on `Arc<Peer>`, and ran the six standard mutants of both impls.
- Current-tree verification: Applied each mutant alone and ran `cargo test -p torrust-tracker-primitives --lib`: all six fail the case table; the file was restored byte for byte (`cmp`).
- Resolution reference: test(primitives): [#2824] cover the ReadInfo role predicates for every state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553083786

### F3 - The slice table and the EPIC row are not reconciled

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530175
- Concern: The S2 row said "PR number at open", S1 stayed `IN_PROGRESS`, the EPIC row still said S1 was in review and the spec had no S2 Progress Log entry.
- Solution: S1 is `DONE`, S2 names #2841, the EPIC row says S1 is merged and S2 in review, and the Progress Log has an S2 entry.
- Current-tree verification: Read the three places back in the tree: the slice table, the EPIC subissue row and the Progress Log.
- Resolution reference: docs(issues): [#2824] fix the S2 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553084277

### F4 - The EPIC log entry sits under References and is stamped after its commit

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530191
- Concern: The S2 EPIC entry was appended after the References list and was stamped 13:40 UTC, before the commit that carries it.
- Solution: Moved the entries under the Progress Log after the S1 entry; the rewritten commits are dated after every stamp (13:40 and 14:10 UTC).
- Current-tree verification: Compared the stamps with `git log --format=%cI` for the rewritten commits.
- Resolution reference: docs(issues): [#2824] fix the S2 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553084785

### F5 - peer::fixture is deferred to a closed EPIC

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530198
- Concern: Several records named #2482 (closed, did not move the fixture) as owner of `peer::fixture`, against the F5 row and the inventory.
- Solution: The README, `peer.md`, `coverage-evidence.md` and the ledger row now name the F5 follow-up (`peer-id-test-helper`) as the owner and cite #2482 only as closed.
- Current-tree verification: Searched the issue folder for `2482`: the remaining mentions say it is closed.
- Resolution reference: docs(issues): [#2824] fix the S2 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553085358

### F6 - The approved record marks From and DerefMut as unselected, but the PR tests them

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530203
- Concern: The approved record said `From` and `DerefMut` are unselected, yet the PR adds a test for each.
- Solution: Recorded the deviation and its reason in the record row: two cheap tests that pin the delegation.
- Current-tree verification: Read the row in `peer.md`.
- Resolution reference: docs(issues): [#2824] fix the S2 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553085818

### F7 - The record states say the completed-file review is still to come

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530220
- Concern: The frontmatter, the intro, the README index and the Validation section still described the completed-file review as pending.
- Solution: Set the status to `done` (first `completed`, changed after F17) and updated the intro, the README row and the Validation section; the second gate was passed on 2026-11-12.
- Current-tree verification: Searched `peer.md` and the README for `pending` and `are added at the completed-file review`: none left.
- Resolution reference: docs(issues): [#2824] fix the S2 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553086615

### F8 - The S2 dates disagree

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530228
- Concern: The results heading and the coverage evidence dated the same measurement differently, and the Decisions section dated the Q1 resolution before it happened.
- Solution: The results are dated 2026-11-12; the Decisions heading says Q1 was settled on 2026-11-12.
- Current-tree verification: Read the two headings and the coverage-evidence section.
- Resolution reference: docs(issues): [#2824] fix the S2 records after review round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553087247

### F9 - The mutation table "Killed by" column no longer matches the tests

- PR number: 2841
- Source review ID: 5876129658
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551735656
- Concern: After the case table was added, several mutants are also killed by it, so the recorded attributions were stale.
- Solution: Re-ran the affected mutants and updated the column; round 2 (F18) found three cells still differing, and they were corrected from a run of each mutant at one place.
- Current-tree verification: Mutated lines of `peer.rs` one at a time, listed the failing tests and restored the file (`cmp`).
- Resolution reference: docs(issues): [#2824] re-check the mutation attributions of the peer.rs record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553155043

### F10 - A review thread has a reply, but the branch has no review audit record

- PR number: 2841
- Source review ID: 5876129658
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551735667
- Concern: The first replies were posted before the audit record `PR-REVIEW.md` existed.
- Solution: Created this record from the template in its own commit and logged the late start in the Processing Log.
- Current-tree verification: The record exists at `docs/pr-reviews/pr-2841-review/PR-REVIEW.md` and has a row and a detail entry for every thread of the PR.
- Resolution reference: docs(pr-reviews): [#2824] record the review of PR 2841 round 1
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553155545

### F11 - The PR description still gives the pre-review figures

- PR number: 2841
- Source review ID: 5876129658
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551735678
- Concern: The description still said 31 tests, 80 unit tests and the first coverage figures, and listed three commits.
- Solution: Edited the description: 32 tests, 81 unit tests, 85.20% / 89.28% / 82.99%, the rewritten commit list and a "Validation performed" part; a PR conversation comment states the edit.
- Current-tree verification: Read the description back through the API after the edit.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2841#issuecomment-6829216025
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553156049

### F12 - The commit subject says "for every state", but the table has five cases

- PR number: 2841
- Source review ID: 5876129658
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551735687
- Concern: The subject of the commit that added the case table overstated what the five cases cover.
- Solution: No change. The first rewrite folded that commit, but the rebuild after round 2 restored it with its subject (F14 requires every step to be kept). The subject overstates the five cases, which are enough for the standard mutants of both impls; only the wording is off.
- Current-tree verification: `git log --format=%s` lists the restored commit with the subject `test(primitives): [#2824] cover the ReadInfo role predicates for every state`.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554227749
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553156550

### F13 - The ReadInfo tests leave the trait predicates unprotected

- PR number: 2841
- Source review ID: 5875845096
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551515165
- Concern: Copilot reported that the `ReadInfo` tests cover one positive-left peer and one zero-left completed shared peer, so a constant result or a removed stopped-event check survives; it asked for trait-qualified cases on `Peer` and `Arc<Peer>`.
- Solution: The case table described under F2 adds exactly those cases and asserts both seeder and leecher results.
- Current-tree verification: Same mutant run as F2: all killed.
- Resolution reference: test(primitives): [#2824] cover the ReadInfo role predicates for every state
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551572064

### F14 - The rewrite drops two steps the branch had, so the merged history hides a correction

- PR number: 2841
- Source review ID: 5878329038
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553379899
- Concern: The first rewrite folded the case-table commit and the TOML re-spacing step into the test commit, hiding a correction, and the description did not say the history was rewritten.
- Solution: Rebuilt the history from the pre-split commits: README alone, then the test commit with `peer.md`, the records commit, the case-table commit as its own commit, and the record fixes and audit commits above them. The TOML re-spacing step cannot be a separate commit because the pre-commit gate rejects the unformatted array; the test commit carries the formatted array and its body says so. The description says the history was rewritten.
- Current-tree verification: Listed the rebuilt branch with `git log --format=%s` and compared its final tree with the previous head before the round 2 fixes: identical.
- Resolution reference: test(primitives): [#2824] test the peer module
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553981820

### F15 - This log entry is stamped before the review it reports as fixed

- PR number: 2841
- Source review ID: 5878329038
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553379918
- Concern: A spec Progress Log entry was stamped before the review it reports, and the README frontmatter date was stale.
- Solution: Added a new Progress Log entry with the real times of the round without editing the old one, and updated the README date.
- Current-tree verification: Read the new entry and the frontmatter in the tree.
- Resolution reference: docs(issues): [#2824] correct the S2 record statuses, mutation cells and timing
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553982318

### F16 - The decision record sits in the docs commit, apart from its tests

- PR number: 2841
- Source review ID: 5878329038
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553379928
- Concern: The spec puts a file's decision record, tests and coverage in one commit, but the record was in the docs commit before the tests.
- Solution: The rebuild puts `peer.md` in the test commit; only the shared README is in the docs commit.
- Current-tree verification: `git show --stat` of the test commit lists `peer.md`.
- Resolution reference: test(primitives): [#2824] test the peer module
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553982792

### F17 - status: completed is a status no other file test plan uses

- PR number: 2841
- Source review ID: 5878329038
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553379936
- Concern: Every other file test plan says `status: done`.
- Solution: Changed the status to `done`.
- Current-tree verification: Read the frontmatter of `peer.md`.
- Resolution reference: docs(issues): [#2824] correct the S2 record statuses, mutation cells and timing
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553983267

### F18 - Three "Killed by" cells still differ from the tests that fail

- PR number: 2841
- Source review ID: 5878329038
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553379945
- Concern: Three mutation table cells did not list every test that fails; the audit repeated a count of 6.
- Solution: Re-ran each mutant at one place and recorded the failing tests; corrected the three cells and the audit text.
- Current-tree verification: Ran the nine mutants listed in the table one at a time and compared the failing tests with the cells; the file was restored byte for byte.
- Resolution reference: docs(issues): [#2824] correct the S2 record statuses, mutation cells and timing
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553983709

### F19 - The first run is dated before the decision that added its killing test

- PR number: 2841
- Source review ID: 5878329038
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553379960
- Concern: The note dated the first eight mutants to 2026-11-11, before the decision that added their killing test.
- Solution: The note no longer dates the runs by day.
- Current-tree verification: Read the note in `peer.md`.
- Resolution reference: docs(issues): [#2824] correct the S2 record statuses, mutation cells and timing
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553984173

### F20 - The audit misstates two facts that the forge records

- PR number: 2841
- Source review ID: 5878329038
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553379971
- Concern: The audit named the reviewed heads by subjects they did not have and marked F13 as the re-raise of F2 although F13 came first.
- Solution: Corrected the head subjects and the relationship (F2 is `RE_RAISE_OF:F13`) and logged the correction as a new entry.
- Current-tree verification: Compared the heads and comment times with the API output of the PR.
- Resolution reference: docs(pr-reviews): [#2824] correct the PR 2841 audit facts after round 2
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553984607

### F21 - Mutator fixture uses zero transfer totals

- PR number: 2841
- Source review ID: 5878380545
- Reviewer finding ID: F14
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553421890
- Concern: Both transfer counters of the fixture were zero, so the three mutator tests also pass if a mutator clears `uploaded` or `downloaded`.
- Solution: The fixture uses distinct nonzero totals; three mutants that clear a total are killed by their test and recorded in the mutation table.
- Current-tree verification: Applied each mutant alone and ran the package tests: each fails exactly its mutator test.
- Resolution reference: test(primitives): [#2824] use nonzero transfer totals in the peer mutator fixture
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553985041

### F22 - The audit still describes the history before the rebuild, and a correction misses it

- PR number: 2841
- Source review ID: 5879377639
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554219050
- Concern: Several audit fields still described the history before the rebuild (the case-table commit named wrongly, F12 saying the commit is gone, F1 and F7 text, the round 2 review list, F21 severity and the F19 relationship), and the round 2 correction entry named the wrong replies.
- Solution: Corrected the fields (F2 and F13 name the case-table commit, F12 is `NO_ACTION` with a correcting reply, F1, F7, the round 2 list, F21 `Minor`, F19 `RE_RAISE_OF:F8`) and added a new Processing Log entry for the correction, leaving the earlier entry as it was.
- Current-tree verification: Read each corrected field in the record and ran the validator against GitHub.
- Resolution reference: docs(pr-reviews): [#2824] record round 3 of the PR 2841 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554278875

### F23 - This entry is stamped before the rebuild it reports as done

- PR number: 2841
- Source review ID: 5879377639
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554219058
- Concern: A spec Progress Log entry was stamped 16:45 UTC and called the rebuild done while its commits were made until 16:47 UTC.
- Solution: Added a correcting entry with the real times and left the earlier entry as it was.
- Current-tree verification: Compared the stamps with `git log --format=%cI` of the rebuilt commits.
- Resolution reference: docs(issues): [#2824] point the F6 records at #2850 and fix the S2 stamps
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554279322

### F24 - The note puts the three new mutants in round 1, and the stamp was not moved

- PR number: 2841
- Source review ID: 5879377639
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554219069
- Concern: The mutation note put the last three mutants in review round 1 although they answer a round 2 review, and the file stamp was not moved after the fixture commit.
- Solution: The note says round 2 and the frontmatter stamp is moved.
- Current-tree verification: Read the note and the frontmatter in the tree.
- Resolution reference: docs(issues): [#2824] point the F6 records at #2850 and fix the S2 stamps
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554279740

### F25 - Cover the remaining non-completed events

- PR number: 2841
- Source review ID: 5879536305
- Reviewer finding ID: F22
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554347587
- Concern: The `is_completed` test rejected only `Started`, so a predicate that accepted every event but `Started` still passed; the swarm coordinator uses it to count downloads.
- Solution: The test also asserts `Stopped` and `None`; the mutant `event != Started` is killed and recorded in the mutation table.
- Current-tree verification: Applied the mutant to `is_completed` and ran the package tests: the test fails; the file was restored byte for byte.
- Resolution reference: test(primitives): [#2824] assert that Stopped and None peers are not completed
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554413503

### F26 - Cite the commit that adds the ReadInfo case table

- PR number: 2841
- Source review ID: 5879536305
- Reviewer finding ID: F23
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554347672
- Concern: The resolution references of F2 and F13 named the initial test commit, not the commit that adds the case table.
- Solution: Already fixed by the round 3 audit commit, which was pushed after the head this review covers: F2 and F13 cite the case-table commit.
- Current-tree verification: Read the Resolution reference of F2 and F13 in the record.
- Resolution reference: docs(pr-reviews): [#2824] record round 3 of the PR 2841 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554414010

### F27 - Reconcile F12 with the restored case-table commit

- PR number: 2841
- Source review ID: 5879536305
- Reviewer finding ID: F24
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554347733
- Concern: F12 claimed the subject is gone although the rebuild restored the commit.
- Solution: Already fixed by the round 3 audit commit: F12 is `NO_ACTION` with a solution, verification and reference that state the deliberate restoration (F14).
- Current-tree verification: Read the F12 row and detail in the record.
- Resolution reference: docs(pr-reviews): [#2824] record round 3 of the PR 2841 review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4554414426

## Processing Log

- 2026-11-12 14:14 UTC - Copilot's review `5875845096` (1 finding, F13) processed before this record existed: fixed in the test commit and replied on its thread.
- 2026-11-12 16:17 UTC - da2ce7's review `5875864302` (8 findings, F1 to F8) fixed: the branch history was rewritten and pushed (force-push with lease, approved by the maintainer). Replied on the eight threads and resolved them. This record did not exist yet.
- 2026-11-12 16:17 UTC - da2ce7's review `5876129658` (4 findings, F9 to F12) arrived on the earlier head; its four threads were resolved by mistake in the same sweep, before being read.
- 2026-11-12 16:24 UTC - Started this audit late, after the first replies. F9 (mutation attributions) and F10 (this record) are real: their threads were reopened, F9 was fixed and replied, F10 got a reply that announces this record. F11 (description) was fixed and F12 was folded by the rewrite; both replied. F9 to F12 and F13 are recorded in the rows above; the F9 and F10 threads are resolved after this record is pushed.
- 2026-11-12 16:26 UTC - Correction: the entry above stamped 16:24 UTC is later than its commit (16:23 UTC); its content is accurate. The F9 and F10 threads were then resolved, and the validator passed (13 rows).
- 2026-11-12 17:31 UTC - Round 2: da2ce7's review `5878329038` (7 findings, F14 to F20) and Copilot's review `5878380545` (1 finding, F21; reviewer label F14) processed. The history was rebuilt and force-pushed with the maintainer's approval; the record, mutation cell, status and audit facts were fixed in new commits; F21 fixed with a mutation check. Replied on the eight threads. Correction of an earlier entry: the round 1 fix replies of 16:17 UTC cite subjects that the rebuilt history no longer has; the `Resolution reference` fields above name the current subjects.
- 2026-11-12 17:58 UTC - Round 3: da2ce7's approving review `5879377639` (3 findings, F22 to F24) processed: record, stamp and audit corrections, plus the comment of the negative-left test and the F6 records now point at #2850. Replied on the three threads. Correction of the 17:31 UTC entry above: its last sentence says the round 1 fix replies cite subjects that the rebuilt history no longer has; the stale replies are the F1 and F12 replies (F12 got a correcting reply), and the round 1 replies cite no subject.
- 2026-11-12 18:09 UTC - Round 3, Copilot's review `5879536305` (3 findings, F25 to F27, reviewer labels F22 to F24) on the head before the round 3 audit commit: F25 fixed with a mutation check; F26 and F27 were already fixed by the round 3 audit commit and are recorded as re-raises of F22. Replied on the three threads.

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
