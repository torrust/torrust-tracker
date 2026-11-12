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

Round 1 is Copilot's review `5875845096` (F13, numbered after the others), da2ce7's review `5875864302` (F1 to F8) and da2ce7's review `5876129658` (F9 to F12), submitted on the heads whose last commits have the subjects `docs(issues): [#2824] record the S2 coverage, mutation results and the F6 finding` and `docs(issues): [#2824] cover the ReadInfo role predicates for every state` (the latter is gone after the history rewrite described in F1).

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2841-f1 | Human | Major | maintainability | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2841-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2841-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2841-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2841-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2841-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2841-f7 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2841-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2841-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2841-f10 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2841-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2841-f12 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2841-f13 | Copilot | Minor (inferred) | testing | RE_RAISE_OF:F2 | FIXED | RESOLVED |

## Finding Details

### F1 - The commits do not match their messages

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530157
- Concern: The docs commit carried the `peer.rs` tests, the manifests and the lock, and the test commit carried only a TOML re-spacing, so history and blame would point at the wrong commits; the description repeated the mismatch and lacked a "Validation performed" part.
- Solution: Rewrote the unmerged branch (force-push with lease, with the maintainer's approval): the docs commit carries only `README.md` and `peer.md`; the test commit carries `peer.rs`, the manifests and the lock, with the taplo-formatted array in place; the description lists every commit and has the validation part.
- Current-tree verification: Listed the files of each rewritten commit with `git show --stat`; the test commit contains `peer.rs`, `packages/primitives/Cargo.toml` and `Cargo.lock`, the docs commit only the two plan files.
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
- Resolution reference: test(primitives): [#2824] test the peer module
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
- Resolution reference: docs(issues): [#2824] record the S2 coverage, mutation results and the F6 finding
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
- Resolution reference: docs(issues): [#2824] record the S2 coverage, mutation results and the F6 finding
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
- Resolution reference: docs(issues): [#2824] add the shared test plan guidance and the peer.rs decision record
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
- Resolution reference: docs(issues): [#2824] add the shared test plan guidance and the peer.rs decision record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553085818

### F7 - The record states say the completed-file review is still to come

- PR number: 2841
- Source review ID: 5875864302
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551530220
- Concern: The frontmatter, the intro, the README index and the Validation section still described the completed-file review as pending.
- Solution: Set the status to `completed` and updated the intro, the README row and the Validation section; the second gate was passed on 2026-11-12.
- Current-tree verification: Searched `peer.md` and the README for `pending` and `are added at the completed-file review`: none left.
- Resolution reference: docs(issues): [#2824] add the shared test plan guidance and the peer.rs decision record
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
- Resolution reference: docs(issues): [#2824] add the shared test plan guidance and the peer.rs decision record
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4553087247

### F9 - The mutation table "Killed by" column no longer matches the tests

- PR number: 2841
- Source review ID: 5876129658
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551735656
- Concern: After the case table was added, several mutants are also killed by it, so the recorded attributions were stale.
- Solution: Re-ran the three affected mutants and updated the column (the negative-left test and the role-case table; 6 tests; the role-case table and the shared-peer test); the table says which mutants were run on which day.
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
- Solution: The history rewrite folded that commit into the test commit, whose subject and body do not make the claim; the five cases are enough for the standard mutants of both impls.
- Current-tree verification: Listed the branch commits with `git log --format=%s`: the old subject is gone.
- Resolution reference: test(primitives): [#2824] test the peer module
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
- Resolution reference: test(primitives): [#2824] test the peer module
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2841#discussion_r4551572064

## Processing Log

- 2026-11-12 14:14 UTC - Copilot's review `5875845096` (1 finding, F13) processed before this record existed: fixed in the test commit and replied on its thread.
- 2026-11-12 16:17 UTC - da2ce7's review `5875864302` (8 findings, F1 to F8) fixed: the branch history was rewritten and pushed (force-push with lease, approved by the maintainer). Replied on the eight threads and resolved them. This record did not exist yet.
- 2026-11-12 16:17 UTC - da2ce7's review `5876129658` (4 findings, F9 to F12) arrived on the earlier head; its four threads were resolved by mistake in the same sweep, before being read.
- 2026-11-12 16:24 UTC - Started this audit late, after the first replies. F9 (mutation attributions) and F10 (this record) are real: their threads were reopened, F9 was fixed and replied, F10 got a reply that announces this record. F11 (description) was fixed and F12 was folded by the rewrite; both replied. F9 to F12 and F13 are recorded in the rows above; the F9 and F10 threads are resolved after this record is pushed.

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
