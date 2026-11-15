---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/drafts/peer-id-test-helper/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2865 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2865>.

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

Findings from da2ce7's review 5912769583 (F1-F11, reviewer IDs kept) and Copilot's review 5912779850 (F12-F13, re-raises of F1 and F2), both on head `48858839e`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2865-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2865-f2 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2865-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2865-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2865-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2865-f6 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2865-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2865-f8 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2865-f9 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2865-f10 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2865-f11 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2865-f12 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2865-f13 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F2 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - Callers pass negative numbers and zero

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105181>
- Concern: The spec said the callers need only distinct ids for small positive numbers, but the `i32` wrappers are called with `-1` (`tests/entry/mod.rs:192,233,427`, `tests/repository/mod.rs:548`) and with a loop over `0..408`, and the test at `entry/mod.rs:427` depends on `-1` differing from the made peers. A `u32` helper (Q4) would change call sites, so the regression strategy's "unchanged apart from the helper call" did not hold.
- Solution: Added the wrapper call sites and their arguments to the caller inventory; removed the "small positive numbers" sentence; Q4 now weighs `i32` (sign in the first byte, no call-site changes) against `u32` with a named sentinel and `u32::try_from(i)` in the loops; the regression strategy says which call sites change under each option.
- Current-tree verification: `grep -rn -E 'a_(started|completed)_peer\(' packages/torrent-repository-benchmarking` lists the call sites recorded in the inventory table of `docs/issues/drafts/peer-id-test-helper/ISSUE.md`; `grep -n 'small positive' docs/issues/drafts/peer-id-test-helper/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580164578>

### F2 - M2 command keeps dev-dependency edges

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105185>
- Concern: M2 used `cargo tree -e features`, which keeps dev-dependency edges, so under Q2 the self dev-dependency and the consumers' `[dev-dependencies]` would show the `testing` feature and M2 could not pass. The repository already has `shipped-features-check`.
- Solution: M2 and the regression strategy now use `cargo run -p shipped-features-check -- --package torrust-tracker`, with `cargo tree -p torrust-tracker -e normal,build,features -i torrust-tracker-primitives` as the readable view, and explain why a plain `cargo tree -e features` does not work.
- Current-tree verification: On `develop` `abd7a10ff`, `cargo run -q -p shipped-features-check -- --package torrust-tracker` printed `"kind":"checked"` and exited 0; the `cargo tree` view lists only the `default` feature of `primitives`. `grep -n 'cargo tree -e features -i' docs/issues/drafts/peer-id-test-helper/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580164683>

### F3 - Uncovered lines are the last in peer.rs, not the package

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105191>
- Concern: The spec said the `Id::new` and fixture lines are the only uncovered lines left in the package; #2824's evidence says they are the only ones in `peer.rs`, and `swarm_metadata.rs` has 9 more.
- Solution: Reworded to "the only uncovered lines left in `peer.rs`" and added the 9 uncovered lines of `swarm_metadata.rs`, which F5 does not own.
- Current-tree verification: `docs/issues/open/2824-1347-add-tests-primitives/coverage-evidence.md` says `swarm_metadata.rs` is now 3 / 12 lines; the spec's Background bullet names `peer.rs`.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580164771>

### F4 - u64 fits in neither decimal nor hex

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105195>
- Concern: Q4 said a `u64` needs hex; after the 8-byte prefix only 12 bytes remain, and `u64::MAX` needs 20 decimal or 16 hex digits.
- Solution: Q4 now says a `u64` fits in neither form, so the choice is between two 32-bit inputs.
- Current-tree verification: `grep -n 'u64' docs/issues/drafts/peer-id-test-helper/ISSUE.md` shows only the corrected sentence in Q4.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580164876>

### F5 - ADR amendment precedent was wrong

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105201>
- Concern: The spec said the #2681 and #2699 amendments added ownership-map rows; they only added dated notes at the top of the ADR.
- Solution: The Architectural Decisions bullet now says a `peer::fixture` row would be the first row an amendment adds, and describes what the two amendments did.
- Current-tree verification: The ADR's two amendment notes (#2681, #2699) are at its top; its ownership map has no row added by either.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580164955>

### F6 - Coverage evidence parts missing

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105204>
- Concern: AC6 and T7 rest on coverage, but the spec lacked the parts `create-issue` requires when coverage is measured: an issue-local evidence file, the exact command and its scope, and separate unit-only and aggregate tables.
- Solution: Added a `Coverage Evidence` section (issue-local `coverage-evidence.md`, #2824's `cargo llvm-cov` command and its scope, unit-only and aggregate tables with baseline and final, per-file rows with owners, Markdown only); T7 and AC6 name the file.
- Current-tree verification: `grep -n '### Coverage Evidence' docs/issues/drafts/peer-id-test-helper/ISSUE.md` finds the section; T7 and AC6 cite `coverage-evidence.md`.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165116>

### F7 - T0, T1 and T9 had no commit point

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105208>
- Concern: T0, T1 and T9 change documentation but had no commit point.
- Solution: Added commit-point rows for T0, T1 and T9.
- Current-tree verification: The Commit Points table of `docs/issues/drafts/peer-id-test-helper/ISSUE.md` has rows T0, T1 and T9.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165250>

### F8 - Peer::default users wrongly said to be outside the packages above

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105213>
- Concern: `swarm-coordination-registry` is one of the packages listed above the `Peer::default()` paragraph.
- Solution: The paragraph now says "outside the files above" and notes that only `http-core` and the root crate are packages not listed above.
- Current-tree verification: The `Peer::default()` paragraph of `docs/issues/drafts/peer-id-test-helper/ISSUE.md` reads "outside the files above".
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165358>

### F9 - http-core bench uses a different literal

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105218>
- Concern: The `http-core` bench helper uses `-qB00000000000000000`, not `...01`.
- Solution: Out of Scope now names each literal with where it is used.
- Current-tree verification: `packages/http-core/benches/helpers/util.rs:159` has `-qB00000000000000000`; the Out of Scope bullet says so.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165446>

### F10 - 3.1.0 is the version of primitives, not the workspace

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105220>
- Concern: "The workspace version is 3.1.0" is wrong: 3.1.0 is `primitives`' own version.
- Solution: Changed to "Its version in the workspace is 3.1.0".
- Current-tree verification: `packages/primitives/Cargo.toml:15` has `version = "3.1.0"`.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165558>

### F11 - Timestamps later than the commit

- PR number: 2865
- Source review ID: 5912769583
- Reviewer finding ID: F11
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580105222>
- Concern: `last-updated-utc` and the progress-log stamp said 12:45 UTC, after the 12:43:02 UTC commit.
- Solution: The first entry now says 12:42 UTC (the last edit before that commit) with a correction note; the round-1 entry and `last-updated-utc` say 20:32 UTC, before the fix commit at 20:32:36 UTC.
- Current-tree verification: `git log --format=%cI` gives 2026-11-15T12:43:02Z and 2026-11-15T20:32:36Z for the two commits.
- Resolution reference: docs(issues): correct the peer-id-test-helper inventory and checks after review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165688>

### F12 - Signed benchmark inputs (re-raise)

- PR number: 2865
- Source review ID: 5912779850
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580112326>
- Concern: The inventory overlooks the signed `-1` inputs of the `i32` wrappers; Q4's `u32` helper needs more than a helper-call-only migration.
- Solution: Same concern as F1, fixed there.
- Current-tree verification: See F1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165836>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165836>

### F13 - Exclude dev-dependency edges in M2 (re-raise)

- PR number: 2865
- Source review ID: 5912779850
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580112355>
- Concern: `cargo tree -e features` includes dev-dependencies, so M2 would report `primitives/testing` from test-only dependencies.
- Solution: Same concern as F2, fixed there.
- Current-tree verification: See F2.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165965>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2865#discussion_r4580165965>

## Processing Log

- 2026-11-15 20:31 UTC - Review round 1: da2ce7 review 5912769583 (CHANGES_REQUESTED, F1-F11) and Copilot review 5912779850 (two comments) received on `48858839e`; every finding checked against the tree.
- 2026-11-15 20:32 UTC - Committed the fixes as `docs(issues): correct the peer-id-test-helper inventory and checks after review` and pushed it (head `b23f589ca`).
- 2026-11-15 20:44 UTC - Replied on all 13 threads (20:43:55 to 20:44:22 UTC). Late start: this audit record was created after the replies, not before the first reply as the workflow requires.
- 2026-11-15 20:45 UTC - Validator `validate-audit-record.py --pr-number 2865` exited 0 (13 rows, 0 failures); audit committed separately from the spec fix.

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
