---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2624 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2624>.

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

Round 1 (head: the audit-record commit pushed when the PR was opened): Copilot's review 5618588089 (COMMENTED, 2026-10-22 17:56:22 UTC) carries one inline thread, R1, and no independently actionable assertion in its body. da2ce7's review 5618664130 (APPROVED, 2026-10-22 18:02:53 UTC) carries ten inline threads, F1 to F10, and no independently actionable assertion in its body; da2ce7 also posted `ACK 487405af3...` at 18:02:59 UTC on the same head. The finding IDs are the reviewers' own; none collides with another. Severities are the bracketed labels of the source comments. F3 asks for the same change as R1, so it is a re-raise. The ACK names the head that existed before the fixes, so it is stale after the fix commits.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| R1 | review-finding:pr-2624-r1 | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F1 | review-finding:pr-2624-f1 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2624-f2 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2624-f3 | Human | Minor | correctness | RE_RAISE_OF:R1 | NO_ACTION | SUPERSEDED |
| F4 | review-finding:pr-2624-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2624-f5 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2624-f6 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2624-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2624-f8 | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2624-f9 | Human | Suggestion | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2624-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### R1 - Check direct dependency edges in the four environment drafts

- PR number: 2624
- Source review ID: 5618588089
- Reviewer finding ID: R1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351719925>
- Concern: the drafts' AC1 and M1 required the forced crates to disappear from `cargo tree -p <package> -e normal,features`, which prints the recursive tree. `tracker-core` depends on `swarm-coordination-registry` (`tracker-core/Cargo.toml:42`), `axum-server` on `configuration` (`axum-server/Cargo.toml:31`), and `rest-api-runtime-adapter` on five of the REST API server's six crates (`rest-api-runtime-adapter/Cargo.toml:23-28`), so a correct migration could not pass.
- Solution: AC1, M1 and the baseline task of rows 3 to 7 use `cargo tree -p <package> -e normal --depth 1` and say which crate stays reachable and through what; the separate check that the environment is not compiled without `testing` is kept.
- Current-tree verification: read the three `Cargo.toml` lines; after the fix `grep -c 'e normal --depth 1'` finds 3 matches in each of the five drafts and `grep -l 'normal,features'` lists only the row 9 draft, whose check is on a shipped binary's whole tree.
- Resolution reference: docs(issues): [#2482] check direct dependencies and add the audits to the migration drafts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351921371>

### F1 - The ADR calls `tracker-core/src/test_helpers.rs` private, but the module is `pub mod` and always compiled

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780823>
- Concern: `packages/tracker-core/src/lib.rs:132` declares `pub mod test_helpers;`; only its inner `tests` module is `#[cfg(test)] pub(crate)`. The ADR, DEC-18, the inventory and the EPIC said the module is `#[cfg(test)]` and private. The conclusion (nothing ships) holds.
- Solution: the ADR, DEC-18, the inventory, the EPIC and the #2618 spec Background say the module is a `pub mod` whose content is `#[cfg(test)]` and crate-private, empty in a non-test build.
- Current-tree verification: viewed `lib.rs:132` and `test_helpers.rs:3-4`; `grep -rn 'pub mod. whose content\|is a .pub mod'` over the five files finds the new wording.
- Resolution reference: docs(adrs): [#2618] correct the ADR statements on test_helpers, dependency counts and guards
- Further resolution reference: docs(issues): [#2618] align the spec and the retrospective with the review round
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351921631>

### F2 - V4c cannot show the `--tests` half and reads a stale binary

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780848>
- Concern: V4c's package had no integration test, so `cargo build --tests` and `cargo test --no-run` did not build the binary and the table printed the value the previous row left; the table recorded no exit codes.
- Solution: V4c was rerun on stable and 1.88.0 with an integration test that needs `CARGO_BIN_EXE_q`, with both binaries deleted before each row and exit codes recorded, plus a control without `tests/`; the ADR bullet now says `--tests` and `cargo test` enable the feature when the package has an integration test.
- Current-tree verification: reran the probe: `build` and `build --release` print false; `build --all-targets`, `build --tests`, `test --no-run` and `test` print true; the control built no binary for `--tests` and `--no-run`, as the reviewer said.
- Resolution reference: docs(issues): [#2618] rerun the production-build probe with an integration test
- Further resolution reference: docs(adrs): [#2618] correct the ADR statements on test_helpers, dependency counts and guards
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351922213>

### F3 - AC1 and M1 use the transitive tree and cannot pass after a correct move

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780855>
- Concern: the same request as R1 (check direct edges with `cargo tree -p <package> -e normal --depth 1`), listing the same three transitive paths and the four drafts.
- Solution: fixed once under R1; the reply says `Superseded by R1`.
- Current-tree verification: the same verification as R1.
- Resolution reference: docs(issues): [#2482] check direct dependencies and add the audits to the migration drafts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351922541>

### F4 - "Nine normal dependency edges" is true only for workspace dependencies, and `axum-http-server` has two more test-only crates

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780840>
- Concern: the count counts workspace dependencies and treats an intra-doc link as use, which the ADR did not say. In `axum-http-server`, `torrust-clock` (`src/server.rs:664` inside `mod tests`, and `src/testing/environment.rs:249`) and `torrust-info-hash` (`src/lib.rs:324`, three extractor and handler test modules, and `src/testing/environment.rs:5`) are normal dependencies used only by test code; row 5 made only the registry optional.
- Solution: the ADR says "nine normal workspace dependency edges", names the two crates, and Rule A.2 covers workspace and third-party dependencies (optional behind `testing`, or a dev-dependency); DEC-18 repeats the wording; the inventory has a note; every environment draft's scope audits all normal dependencies, and row 5 names the two crates.
- Current-tree verification: `grep -rn 'torrust_clock\|torrust_info_hash' packages/axum-http-server/src` and `Cargo.toml` (both normal dependencies); after the fix `grep -n 'torrust-clock'` finds the ADR, the row 5 draft and the inventory.
- Resolution reference: docs(adrs): [#2618] correct the ADR statements on test_helpers, dependency counts and guards
- Further resolution reference: docs(issues): [#2482] check direct dependencies and add the audits to the migration drafts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351921919>

### F5 - `last-updated-utc` stamps are stale after later edits

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780862>
- Concern: the evidence file's stamp (2026-10-21 18:50) predates V4b and V4c, and the tracker-core draft's stamp and progress log predate its 2026-10-22 execution-order edit.
- Solution: the evidence file's stamp is 2026-10-22 18:08; the tracker-core draft's is 2026-10-22 18:10 with a log entry, and the other drafts edited in the same round have the same stamp and a log entry.
- Current-tree verification: `sed -n 4p` of the evidence file and `sed -n 12p` of the draft show the new stamps.
- Resolution reference: docs(issues): [#2618] rerun the production-build probe with an integration test
- Further resolution reference: docs(issues): [#2482] check direct dependencies and add the audits to the migration drafts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351922798>

### F6 - The spec frontmatter does not match the body

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F6
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780877>
- Concern: `status: planned`, the branch of the spec PR, and the "Implementation completed" checkpoint ticked while T3 is `IN_PROGRESS` and AC2 waits for the maintainer's approval.
- Solution: `status: in-review`, `branch` names this PR's head branch, `related-pr: 2624`, and the checkpoint is no longer ticked until the ADR is approved.
- Current-tree verification: `sed -n 5p` and `sed -n 10,11p` of the spec show the new values; the checkpoint line reads `- [ ] Implementation completed`.
- Resolution reference: docs(issues): [#2618] align the spec and the retrospective with the review round
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351923091>

### F7 - The retrospective repeats the ordering wording the first review flagged

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780891>
- Concern: "then 4 to 7 as their #1488 gates clear" where the EPIC now states the fixed sequence 1, 2, 8, 9, 3, 4, 5, 6, 7.
- Solution: the retrospective uses the EPIC's sequence and says rows 4 to 7 also wait for their #1488 gates.
- Current-tree verification: `grep -n 'fixed sequence'` finds the new text at line 26 of the retrospective.
- Resolution reference: docs(issues): [#2618] align the spec and the retrospective with the review round
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351923326>

### F8 - Making `mockall` optional also requires gating its imports and the `Mock*` re-exports

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F8
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780906>
- Concern: `use mockall::automock;` is unconditional in the four `databases/traits/` files, and the `Mock*` re-exports at `databases/mod.rs:66` and `databases/traits/mod.rs:14` are unconditional, so a default build would not compile.
- Solution: row 3's scope names the four imports and the two re-exports, and adds the dependency audit.
- Current-tree verification: `grep -n 'use mockall' packages/tracker-core/src/databases/traits/*.rs` and the two re-export lines; after the fix the row 3 draft names them.
- Resolution reference: docs(issues): [#2482] check direct dependencies and add the audits to the migration drafts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351923643>

### F9 - The `deny.toml` guard covers less than rule B.1 says

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F9
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780916>
- Concern: `deny.toml` has `wrappers` entries for the HTTP and UDP protocol crates, the server crates, `http-core` and `udp-core`, but none for `torrust-tracker-client-lib` or `tracker-core`, so `test-helpers` could depend on them again without the check failing, while the ADR said the guard keeps out "protocol and client dependencies".
- Solution: rule B.3 states what the guard covers and that for `tracker-core` and the other domain crates rule B.1 is a convention until row 2 decides; row 2's scope and AC3 add a ban entry for `client-lib`.
- Current-tree verification: read `deny.toml`; after the fix `grep -n 'client-lib'` finds the ADR rule and the row 2 draft.
- Resolution reference: docs(adrs): [#2618] correct the ADR statements on test_helpers, dependency counts and guards
- Further resolution reference: docs(issues): [#2482] check direct dependencies and add the audits to the migration drafts
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351923911>

### F10 - "Without it, only the unit tests run" does not match V4

- PR number: 2624
- Source review ID: 5618664130
- Reviewer finding ID: F10
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351780929>
- Concern: under O1a a plain `cargo test -p X` fails to compile with exit 101 and runs nothing; only `--lib` passes, and only with the `any(test, ...)` gate.
- Solution: the ADR bullet says exactly that.
- Current-tree verification: compared with the V4 table row `test -p X` and `test -p X --lib`.
- Resolution reference: docs(adrs): [#2618] correct the ADR statements on test_helpers, dependency counts and guards
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2624#discussion_r4351924243>

## Processing Log

- 2026-10-22 17:56 UTC - Copilot submitted review 5618588089 with one inline thread (R1). GraphQL data at the 17:57 UTC poll: 1 thread, unresolved.
- 2026-10-22 18:02 UTC - da2ce7 submitted review 5618664130 (APPROVED) with ten inline threads (F1 to F10) and posted an ACK for the then-current head. GraphQL data at the 18:05 UTC poll: 11 threads, 11 unresolved.
- 2026-10-22 18:17 UTC - Verified each claim against the tree, committed the fixes (commit times 18:08, 18:12, 18:13 and 18:14 UTC), pushed them, and replied on every thread (reply times 18:16:56 to 18:17:16 UTC).
- 2026-10-22 18:18 UTC - Resolved all 11 threads; the reply check exited 0 before resolving, and a GraphQL query at 18:19 UTC showed 11 threads, 0 unresolved. da2ce7's ACK names the head before the fixes; a new ACK is needed on the head that carries this audit record.

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
