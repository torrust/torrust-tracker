---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2347-2003-triage-post-merge-review-findings/ISSUE.md
    - docs/issues/closed/2261-2003-review-udp-protocol-clippy-baseline/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2290 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2290>.

PR #2290 merged at 2026-09-22 10:12 UTC. Review 5284003304 was submitted after the merge, at
2026-09-22 21:30 UTC, and labels its findings post-merge. #2347 tracks them, and this record is
created on its approved follow-up branch.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: <https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5865646588>

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

Audit IDs `F1`-`F7` correspond to the reviewer's `loop F1`-`loop F7` from review 5284003304. No
earlier audit existed, so no ID collides. Findings fixed in follow-up PR #2363 stay `FOLLOW_UP`
and `OPEN` until it merges. The close-out then records them as `FIXED`, citing their commit
subjects. `NO_ACTION` threads are resolved in the close-out.

Copilot review 5275400698 left three inline threads before the merge. They were resolved without
replies, and they get no rows here, by maintainer decision (approval record above). Review
5284003304 confirms all three fixes landed, and the only partial one is F3 below.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2290-f1` | Human | Minor | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2290-f2` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2290-f3` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2290-f4` | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | OPEN |
| F5 | `review-finding:pr-2290-f5` | Human | Minor | other | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2290-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2290-f7` | Human | Suggestion | documentation | ORIGINAL | NO_ACTION | RESOLVED |

## Finding Details

### F1 - Both evidence files cite two commits that exist in no object in this repository

- PR number: 2290
- Source review ID: 5284003304
- Reviewer finding ID: loop F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700031>
- Concern: the #2261 retrospective's Evidence list and the agent-review-reports Inputs line cite
  `7e6f5425` and `d75276ff`. A pre-merge rebase discarded both ids, so neither resolves.
- Solution: `docs(issues): [#2347] cite #2261 evidence commits by subject` replaces the ids in the
  retrospective with their subjects. It appends a dated correction entry to
  `agent-review-reports.md`, which is append-only, and leaves the 06:31 entry unchanged.
- Current-tree verification: on `develop` at `478516cf`, `git cat-file -t 7e6f5425` and
  `git cat-file -t d75276ff` print `fatal: Not a valid object name`. GitHub still serves both
  objects (`gh api repos/torrust/torrust-tracker/commits/<sha>`), with subjects
  `docs(quality): define Clippy exception decisions` and
  `refactor(udp-protocol): remove Clippy baseline`. Each subject names exactly one commit on
  `develop`.
- Resolution reference: `docs(issues): [#2347] cite #2261 evidence commits by subject`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4124956431>

### F2 - The M1 PASS result is false at the head it ships in, and its recorded output does not reproduce

- PR number: 2290
- Source review ID: 5284003304
- Reviewer finding ID: loop F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700041>
- Concern: M1's Result says no owned suppression remains in the final source, but A159 was
  restored. Its recorded command output no longer reproduces, and the revalidation section does
  not retract the PASS.
- Solution: `docs(issues): [#2347] correct the #2261 M1 result for retained A159` appends
  "M1 Post-Merge Correction", with the re-run output and the final outcome.
- Current-tree verification: M1's first command, re-run as recorded on `develop` at `478516cf`,
  prints `packages/udp-protocol/src/lib.rs` and `15:    clippy::empty_enums,`. Its second
  command prints lines 9, 10, and 16.
- Resolution reference: `docs(issues): [#2347] correct the #2261 M1 result for retained A159`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4124956636>

### F3 - Copilot's action-3 finding was fixed in the test name only; this prose still claims the opposite

- PR number: 2290
- Source review ID: 5284003304
- Reviewer finding ID: loop F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700048>
- Concern: the #2261 spec says Arrange "creates each supported action", but the loop covers action
  codes 0-3, and code 3 is rejected as an invalid action.
- Solution: `docs(issues): [#2347] describe the #2261 parser test action codes` rewords the
  sentence to "each action code 0-3, including the unsupported code 3".
- Current-tree verification: `packages/udp-protocol/src/request.rs` has
  `for action in 0i32..4` in
  `it_should_not_panic_when_parsing_all_action_codes_at_all_packet_lengths`, and
  `_ => Err(RequestParseError::unsendable_text("Invalid action"))`.
- Resolution reference: `docs(issues): [#2347] describe the #2261 parser test action codes`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4124956882>

### F4 - AC2 is checked, but A159's actual outcome matches none of AC2's three permitted outcomes

- PR number: 2290
- Source review ID: 5284003304
- Reviewer finding ID: loop F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700056>
- Concern: A159 is retained permanently at crate scope. AC2 admits only removed, narrowed to a
  source-specific allowance, or temporary with a removal condition, and the exception framework
  calls broad crate-level suppression insufficient.
- Solution: follow-up in #2360 (EPIC #2003). It decides between a module-level allowance, a
  temporary allowance with a removal condition, and a maintainer-reviewed framework carve-out.
  Each option needs a Rust or framework change, which is outside #2347's documentation scope.
- Current-tree verification: on `develop` at `478516cf`, `lib.rs` carries the crate-level
  `#![allow(clippy::empty_enums, reason = ...)]`. With it removed in the working tree,
  `cargo +nightly clippy -p torrust-tracker-udp-protocol --all-targets --all-features -- -D warnings`
  (`1.100.0-nightly`, 2026-09-23) reports 18 unique `enum with no variants` diagnostics, and
  stable Clippy 1.98.1 reports none. An item-level `#[allow]` on a `connect.rs` struct does not
  suppress its diagnostic. The files were restored afterwards.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4121100844>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4121100844>

### F5 - Three Copilot threads were resolved with no reply and no audit record was created

- PR number: 2290
- Source review ID: 5284003304
- Reviewer finding ID: loop F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700063>
- Concern: Copilot review 5275400698's three threads were resolved with no reply, and
  `docs/pr-reviews/pr-2290-review/PR-REVIEW.md` did not exist.
- Solution: `docs(pr-reviews): [#2347] add PR #2290 post-merge review audit` creates this record.
  The maintainer declined backfilling rows and replies for the three Copilot threads (approval
  record in the Ownership section): all three fixes landed, and the partial one is F3.
- Current-tree verification: before this record, `docs/pr-reviews/pr-2290-review/` was absent
  on `develop` at `478516cf`. `github-review-threads show` lists the three Copilot threads as
  resolved, each with one comment.
- Resolution reference: `docs(pr-reviews): [#2347] add PR #2290 post-merge review audit`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4124957142>

### F6 - The module list for the `empty_enums` diagnostics omits `announce.rs`

- PR number: 2290
- Source review ID: 5284003304
- Reviewer finding ID: loop F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700071>
- Concern: the M1 revalidation names `common.rs`, `connect.rs`, and `scrape.rs`, but `announce.rs`
  also carries `FromBytes` derive sites.
- Solution: `docs(issues): [#2347] name all four #2261 empty_enums modules` appends a correction
  with the per-module counts.
- Current-tree verification: the same nightly Clippy run as F4 reports `common.rs` 11,
  `announce.rs` 5, `connect.rs` 1, and `scrape.rs` 1 unique diagnostics.
- Resolution reference: `docs(issues): [#2347] name all four #2261 empty_enums modules`
- Follow-up PR URL: <https://github.com/torrust/torrust-tracker/pull/2363>
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4124957429>

<!-- cspell:ignore misattributes -->

### F7 - The merged PR description overstates the removal by one and misattributes ownership

- PR number: 2290
- Source review ID: 5284003304
- Reviewer finding ID: loop F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4076700080>
- Concern: the PR description says twelve #2261-owned allowances were removed. Eleven were, and
  `empty_enums` was never #2261-owned.
- Solution: no action, approved by the maintainer. The merged description is delivery history,
  and the in-tree records already say "Eleven". An edit made outside the repository has no
  admissible `FIXED` resolution reference under the current contract (`review-finding:pr-2313-f6`,
  #2362), so the reply records the correction on the thread.
- Current-tree verification: the #2261 retrospective's Outcome begins "Eleven nonnumeric UDP
  protocol crate-level Clippy allowances were removed"; the PR body still reads "Remove all twelve
  nonnumeric UDP protocol crate-level Clippy allowances owned by #2261."
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4121101621>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2290#discussion_r4121101621>

## Processing Log

- 2026-09-26 12:03 UTC - The maintainer's approval to track the late review in #2347 was recorded (<https://github.com/torrust/torrust-tracker/issues/2347#issuecomment-5846100762>).
- 2026-09-26 12:04 UTC - Posted a tracking reply naming #2347 on all seven threads (the first at `created_at` 12:04:47Z), with the disposition pending.
- 2026-09-28 07:44 UTC - The maintainer approved the dispositions (Ownership section).
- 2026-09-28 10:24 UTC - Follow-up PR #2363 opened as a draft with the F1, F2, F3, and F6 fix commits.
- 2026-09-28 10:28 UTC - Posted a disposition reply on each of the seven threads (`created_at` 10:28:56Z-10:29:07Z). Threads stay unresolved until the close-out.
- 2026-09-28 10:33 UTC - Started this audit, fixing F5. Copilot review 5275400698 (three threads, resolved before merge without replies) is recorded as context only, without rows, as the maintainer approved.
- 2026-09-28 11:48 UTC - Correction: the 10:28 UTC entry is stamped at the first disposition reply; the last was posted at 10:29:07Z.
- 2026-09-28 17:07 UTC - Follow-up PR #2363 merged into `develop` at `bc90cde1b` (17:00 UTC); its tree is identical to the reviewed head `30f7419c4`. Re-derived each fix on `develop`, changed F1, F2, F3, F5, and F6 to `FIXED`/`RESOLVED` with their commit subjects as resolution references, and posted a final reply on each (`created_at` 17:07:36Z-17:07:42Z).
- 2026-09-28 17:11 UTC - Resolved the fixed threads and the F7 (`NO_ACTION`) thread. A GraphQL refetch at 17:12 UTC shows 10 threads, 1 unresolved: F4, owned by #2360.

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
