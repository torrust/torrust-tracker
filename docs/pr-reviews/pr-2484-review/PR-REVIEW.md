---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md
    - docs/issues/open/2482-1669-reorganize-shared-test-support/test-support-inventory.md
---

<!-- skill-link: process-pr-review -->

# PR #2484 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2484>.

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

Copilot review 5453137624 (07:23 UTC) reported "0 open findings" and makes no request, so it gets
no row.

Human review 5454599321 by `da2ce7` (`CHANGES_REQUESTED`, round 1, 09:38 UTC) supplied the finding
IDs `F1` to `F7` with bracketed severities; all seven are kept. Its body summarizes the same seven
threads and lists the rest under "Checked, no finding", so it adds no other row. F1 and F2 block;
F3 to F7 do not.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2484-f1` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2484-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2484-f3` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2484-f4` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2484-f5` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2484-f6` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2484-f7` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2484-f8` | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2484-f9` | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2484-f10` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | `review-finding:pr-2484-f11` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2484-f12` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F13 | `review-finding:pr-2484-f13` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F14 | `review-finding:pr-2484-f14` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The inventory dated all five test-helpers dependencies to one event

- PR number: 2484
- Source review ID: 5454599321
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4217324872>
- Concern: The inventory said the `http` and `udp` modules took `test-helpers` from one workspace dependency to five on 2026-07-29, but `primitives` arrived on 2026-10-02 with the #2406 scrape helpers.
- Solution: Dated the growth in two steps: `client-lib` and the two protocol crates on 2026-07-29, and `primitives` on 2026-10-02.
- Current-tree verification: `test-support-inventory.md:115-117` carry the two dates; `git log -- packages/test-helpers/Cargo.toml` shows `8151e920c` adding the three crates and `4b61e1cca` adding `primitives`.
- Resolution reference: docs(issues): [#2482] date the test-helpers dependency growth in two steps
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219053331>

### F2 - Not every workspace package is planned to be published

- PR number: 2484
- Source review ID: 5454599321
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4217324885>
- Concern: The spec said every workspace package is planned to be published and made "every package stays publishable" a criterion, but nine of the 31 members set `publish = false` and the cited ADR keeps an "Unpublished tooling" tier; the timing was sourced only by the GitHub issue body.
- Solution: Scoped both sentences to members without `publish = false`, which include every crate in Patterns A and B, named issue #2482 as the source of the timing, and linked the ADR and its tier. A follow-up commit rewrapped a 121-character line the first commit created.
- Current-tree verification: In `EPIC.md`, the "Why This Is Needed" paragraph starting "Every workspace member without `publish = false`" and the Criteria bullet "Every package without `publish = false` stays publishable" carry the scoped wording (cited by line as `ISSUE.md:69-75` and `:92` before the sub-EPIC conversion, then as `EPIC.md:66-71` and `:109`); `cargo metadata --no-deps` lists 9 of 31 members with `publish = []`; `linter lychee` exits 0.
- Resolution reference: docs(issues): [#2482] scope the publishing plan to members without publish = false
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219053683>

### F3 - The udp row listed torrust-info-hash

- PR number: 2484
- Source review ID: 5454599321
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4217324902>
- Concern: `udp.rs` uses `torrust_tracker_udp_protocol::common::InfoHash` and never imports `torrust_info_hash`; only `http.rs` does.
- Solution: Dropped `torrust-info-hash` from the `udp` row.
- Current-tree verification: The `udp` row (`test-support-inventory.md:113`) no longer names it; `udp.rs:90`, `:144` and `:293` use the protocol crate's type.
- Resolution reference: docs(issues): [#2482] drop torrust-info-hash from the udp module's dependencies
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219054062>

### F4 - peer_tests adds no public module

- PR number: 2484
- Source review ID: 5454599321
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4217324912>
- Concern: `peer_tests.rs` starts with the inner attribute `#![cfg(test)]`, which removes the module from non-test builds; only `test_helpers` and `whitelist::test_helpers` leave empty public modules.
- Solution: Said that two of the three `pub mod` declarations leave empty public modules, and why `peer_tests` does not.
- Current-tree verification: `test-support-inventory.md:128-132` carry the corrected sentence; `packages/tracker-core/src/peer_tests.rs:1` is `#![cfg(test)]`.
- Resolution reference: docs(issues): [#2482] say which tracker-core test modules stay public
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219054445>

### F5 - Only two environments define Unstarted

- PR number: 2484
- Source review ID: 5454599321
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4217324916>
- Concern: The inventory said every server package aliases `Unstarted` and `Started`, but only `axum-http-server` and `udp-server` define `Unstarted`.
- Solution: Said all four alias `Started` and only those two also alias `Unstarted`.
- Current-tree verification: `test-support-inventory.md:52-55` carry the corrected sentence; `Unstarted` appears only at `axum-http-server/src/testing/environment.rs:19` and `udp-server/src/testing/environment.rs:28`.
- Resolution reference: docs(issues): [#2482] say only two environments alias Unstarted
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219054774>

### F6 - axum-http-server to configuration is a candidate tenth edge

- PR number: 2484
- Source review ID: 5454599321
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4217324924>
- Concern: Outside test code, `axum-http-server` uses `torrust-tracker-configuration` only in an intra-doc link, so the inventory should either list the edge with that note or say that doc links count as production use.
- Solution: Took the second option: the inventory states that intra-doc links count as production use, so the edge is not listed and the nine-edge counts stay, and notes that under O1 it becomes a tenth candidate if the link is reworded.
- Current-tree verification: `test-support-inventory.md:87-91` carry the rule; the other three uses in `src/` sit under `#[cfg(test)]` (`scrape.rs:118`, `announce.rs:132`, `server.rs:524`).
- Resolution reference: docs(issues): [#2482] state how the inventory counts intra-doc links
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219055096>

### F7 - Name the open #1488 subissues that change these environments

- PR number: 2484
- Source review ID: 5454599321
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4217324931>
- Concern: Three open #1488 specs (#2449, #2450, #2471) change the `Environment` types this plan would move, and the spec did not name them.
- Solution: Added a Risks bullet that names them and asks the migration plan to order each move after them or record why the order does not matter, and linked their specs from References.
- Current-tree verification: In `EPIC.md`, the Risks bullet "Three open subissues of EPIC #1488 change the environments", the References entry "Shutdown overhaul EPIC: #1488" with its three spec links, and the "Coordination with EPIC #1488" rule under "How we work in this EPIC" (cited by line as `ISSUE.md:280-283` and `:310-313` before the sub-EPIC conversion, then as `EPIC.md:330-332`, `:347-350` and `:204-207` until the 15:20 entry shifted them); `linter lychee` exits 0.
- Resolution reference: docs(issues): [#2482] name the open #1488 subissues that change the environments
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219055480>

### F8 - The round-1 log entry overruns the 100-column wrap

- PR number: 2484
- Source review ID: 5457064358
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219319086>
- Concern: Two lines of the 12:40 progress-log entry in `EPIC.md` were 103 and 101 characters, the only prose lines over the file's 100-column wrap.
- Solution: Rewrapped the entry to 100 columns, keeping `#1488` mid-line so no continuation starts with `#<digits>` (MD018), and logged the rewrap.
- Current-tree verification: An `awk` width check on `docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md` finds no prose line over 100 columns, only table rows and link-only reference lines; `linter markdown` exits 0.
- Resolution reference: docs(issues): [#2482] rewrap the round-1 log entry to 100 columns
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4220601650>

### F9 - The audit's related-artifacts names the deleted ISSUE.md

- PR number: 2484
- Source review ID: 5457064358
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219319065>
- Concern: The sub-EPIC conversion replaced `ISSUE.md` with `EPIC.md`, but this audit's `related-artifacts` still named `ISSUE.md`, the only live path to it in the tree.
- Solution: Pointed the entry at `EPIC.md`.
- Current-tree verification: The frontmatter of this record names `EPIC.md`; `rg 'reorganize-shared-test-support/ISSUE.md'` outside dated prose finds nothing.
- Resolution reference: docs(pr-reviews): [#2482] point the PR #2484 audit at EPIC.md
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4220600920>

### F10 - Two audit verifications cite ISSUE.md lines that no longer exist

- PR number: 2484
- Source review ID: 5457064358
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219319077>
- Concern: The F2 and F7 current-tree verifications cited `ISSUE.md` line ranges that the conversion removed.
- Solution: Re-pointed both to the matching `EPIC.md` lines, keeping the earlier `ISSUE.md` citations marked as before the conversion.
- Current-tree verification: In `docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md`, the "Why This Is Needed" paragraph starting "Every workspace member without `publish = false`", the Criteria bullet "Every package without `publish = false` stays publishable", the "Coordination with EPIC #1488" rule, the Risks bullet naming #2449, #2450 and #2471, and the References entry for #1488 hold the cited text (cited by line as `:66-71`, `:109`, `:204-207`, `:330-332` and `:347-350` at `542f1b293`); the validator reports 0 failures.
- Resolution reference: docs(pr-reviews): [#2482] point the PR #2484 audit at EPIC.md
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4220601330>

### F11 - EPIC #1669 records the conversion at 12:33, EPIC.md at 12:53

- PR number: 2484
- Source review ID: 5457064358
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219319097>
- Concern: EPIC #1669's log entry and stamp dated the conversion 12:33, while the sub-EPIC's log and stamp said 12:53 and its commit was authored at 12:54:52Z.
- Solution: Dated the #1669 entry and its `last-updated-utc` at 12:53.
- Current-tree verification: `docs/issues/open/1669-overhaul-packages/EPIC.md` stamp and progress-log entry both read `2026-10-08 12:53`.
- Resolution reference: docs(issues): [#2482] date the sub-EPIC conversion at 12:53 in EPIC #1669
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4220602012>

### F12 - The ownership rule from Design and Ownership Review was dropped

- PR number: 2484
- Source review ID: 5457064358
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4219319106>
- Concern: The conversion removed Design and Ownership Review, so its ownership rule for the environments appeared nowhere in the tree, although `create-issue` requires an ownership map for reusable test fixtures.
- Solution: Made the ownership map part of D2's expected output (the package whose server an environment starts owns it; other packages use its published test API or their own `tests/` code) and logged that subissue 1's spec carries the full section.
- Current-tree verification: The D2 row and the 14:48 progress-log entry in `docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md` inspected.
- Resolution reference: docs(issues): [#2482] carry the environment ownership rule into D2
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4220602429>

### F13 - The 14:48 entry says a spec that does not exist yet carries the section

- PR number: 2484
- Source review ID: 5458896567
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4220793533>
- Concern: The 14:48 progress-log entry said subissue 1's spec "carries" the full Design and Ownership Review, but that spec does not exist yet.
- Solution: The entry now says the spec "will carry" the section, and a 15:20 entry records the correction.
- Current-tree verification: The 14:48 and 15:20 entries and the `last-updated-utc` stamp in `docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md` inspected; pre-commit passed.
- Resolution reference: docs(issues): [#2482] say subissue 1's spec will carry the ownership review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4220965544>

### F14 - Two verifications cite EPIC.md ranges that the 15:20 entry shifted

- PR number: 2484
- Source review ID: 5459306372
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4221129255>
- Concern: The F7 and F10 verifications cited `EPIC.md:330-332` and `:347-350`; the 15:20 log entry added two lines above them, so the ranges no longer pointed at the cited text. Non-blocking.
- Solution: The F2, F7 and F10 verifications now cite the EPIC by section and quoted phrase, keeping the old ranges in parentheses with the head they held at, so later log entries cannot move them.
- Current-tree verification: Each quoted phrase ("Every workspace member without `publish = false`", "Every package without `publish = false` stays publishable", "Coordination with EPIC #1488", "Three open subissues of EPIC #1488 change the environments", "Shutdown overhaul EPIC: #1488") occurs exactly once in `docs/issues/open/2482-1669-reorganize-shared-test-support/EPIC.md` (`grep -c`); the validator reports 0 failures.
- Resolution reference: docs(pr-reviews): [#2482] cite the EPIC passages by section in the PR #2484 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2484#discussion_r4221516086>

## Processing Log

- 2026-10-08 12:34 UTC - Started audit. Fetched the seven review threads with GraphQL (all
  unresolved, none outdated) and both review bodies; normalized them into F1 to F7.
- 2026-10-08 12:42 UTC - Committed the seven fixes, one per finding, plus a rewrap and a progress-log
  entry, after the pre-commit gate passed; rebased onto `develop`, which had moved 13 commits, and
  pushed.
- 2026-10-08 12:43 UTC - Re-checked every claim against the pushed tree and replied on all seven
  threads.
- 2026-10-08 13:08 UTC - Human review 5457064358 (da2ce7, round 2) requested changes at the sub-EPIC
  conversion head `c43fc9013`: F1 to F7 carried as fixed, and five new findings, F8 to F12.
- 2026-10-08 15:01 UTC - Committed one fix per finding (F9 and F10 share the audit edit), rebased
  onto `develop`, which had moved 11 commits, pushed after the pre-push suite passed, and replied
  on the five threads before recording them here.
- 2026-10-08 15:17 UTC - Human review 5458896567 (da2ce7, round 3) approved head `542f1b293`,
  confirmed F8 to F12 and their audit rows, and raised one non-blocking Nit, F13.
- 2026-10-08 15:32 UTC - At the maintainer's request, fixed F13 rather than defer it, accepting that
  the push dismisses the approval; pushed after the pre-push suite passed and replied on the thread
  before recording it here.
- 2026-10-08 15:47 UTC - Human review 5459306372 (da2ce7, round 4) approved head `feb0d8f24`,
  confirmed F13 and its row, and raised one non-blocking Minor, F14.
- 2026-10-08 16:24 UTC - Fixed F14 by anchoring the citations to sections, added
  `review-retrospective.md` at the maintainer's request, rebased onto `develop` (2 commits ahead),
  pushed after the pre-push suite passed, and replied on the thread before recording it here.

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
