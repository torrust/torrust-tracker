---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/1348-1347-add-tests-udp-core/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2538 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2538>.

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

Two reviews were submitted. Copilot's review 5521914331 left no inline comments; its overview says
its full review did not start before a timeout, so it has no actionable assertion and no row.
da2ce7's review 5522366109 (changes requested) carries F1-F9 as inline threads; its body repeats
them as a summary, so the body adds no rows. The review read the head whose last commit was
"docs(udp-core): reconcile the statistics docs and records with #2517 and #2521" before the F1
history rewrite; the commit ids it cites no longer exist on the branch.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2538-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2538-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2538-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2538-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2538-f5 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2538-f6 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2538-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2538-f8 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2538-f9 | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - 25 commits cannot compile the package tests

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423647>
- Concern: two rebases left commits whose `udp-core` test target does not compile (a duplicate
  test module and removed APIs after #2517 and #2521; two-argument `make` calls after #2458), with
  the repairs added as later commits. The maintainer merge keeps every commit on `develop`, so
  bisecting the package's tests breaks.
- Solution: with the maintainer's approval, an interactive rebase folded each repair into the
  commit that introduced the code it repairs; the branch was then rebased onto the latest
  `develop`.
- Current-tree verification: in a temporary worktree, `cargo test -p torrust-tracker-udp-core
  --no-run` succeeded at every branch commit, 0 failures.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2538#issuecomment-6191642481>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282747072>

### F2 - The claim of a mutation proof for every new test is overstated

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423657>
- Concern: AC4 and the PR body claimed a mutation proof for every new test; the plans record about
  one per test-producing file.
- Solution: aligned AC4's evidence and the PR body with the retrospective's accurate wording (a
  proof per test-producing plan, not per test) instead of adding proofs.
- Current-tree verification: `ISSUE.md` AC4 row says "mutation proofs are recorded for every
  test-producing plan (not for every test)"; the PR body says the same.
- Resolution reference: docs(udp-core): reconcile the #1348 records after the PR #2538 review fixes
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282747389>

### F3 - The cause recorded for the unviable mutants is wrong

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423665>
- Concern: the records blamed `.cargo/config.toml` for denying unused code and said an empty
  `RUSTFLAGS` does not override it; the denies live in `[workspace.lints.rust]`.
- Solution: the mutation evidence, lessons, and retrospective name `[workspace.lints.rust]` in the
  root `Cargo.toml` and drop the override sentence.
- Current-tree verification: `grep` finds `[workspace.lints.rust]` in `mutation-evidence.md` and
  `lessons.md` and no empty-`RUSTFLAGS` sentence in the issue folder.
- Resolution reference: docs(udp-core): attribute the unviable mutants to the workspace lint table
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282747745>

### F4 - Several records were not reconciled after the final rebase

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423670>
- Concern: stale repository counts, retrospective wording, ledger count, spec timestamp, and
  doc-test count, and no record of the independent pre-PR Task Reviewer the spec requires.
- Solution: reconciled the five records. Ran the Task Reviewer: it failed because no plan
  recorded the prose-first AAA review (AC7); that review was done for every test-producing file,
  recorded in each plan, and the re-review passed. Both runs are in `agent-review-reports.md`.
- Current-tree verification: plan index and repository plan say 3 tests remain; retrospective
  names three dropped tests and #2458, #2505, #2517, #2521; AC2 says 18 `DONE`, 3 `REMOVED`;
  `last-updated-utc` matches the latest log entry; AC12 says 6 runnable doc tests and 3 `ignore`
  blocks; `agent-review-reports.md` ends with `REVIEW PASSED`.
- Resolution reference: docs(udp-core): reconcile the #1348 records after the PR #2538 review fixes
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282748032>

### F5 - A test-typed commit changes only records

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423682>
- Concern: the commit "test(udp-core): prove the statistics service reads both metric sources"
  contained only four Markdown files after the rebase.
- Solution: reworded as a `docs` commit during the F1 history rewrite.
- Current-tree verification: `git log torrust/develop..HEAD` has the subject below and no
  "prove the statistics service" subject.
- Resolution reference: docs(udp-core): record the statistics services file review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282748298>

### F6 - Repository and metrics tests skip the naming and AAA conventions

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423692>
- Concern: five new tests lacked the `it_should_` prefix, three lacked AAA sections, and S1 lost
  #2517's assertion message.
- Solution: renamed the five tests, added AAA sections, and restored the assertion message.
- Current-tree verification: the tests in `statistics/repository.rs`, `statistics/metrics.rs`, and
  `statistics/mod.rs` start with `it_should_` and have AAA sections; S1's `assert!` has the message.
- Resolution reference: test(udp-core): name and structure the statistics tests by the unit-test conventions
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282748534>

### F7 - The lock-file rationale is wrong

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423704>
- Concern: the manual-verification client is its own Cargo workspace, so the repository lock does
  not pin its versions, contrary to the `.gitignore` comment.
- Solution: the comment says the versions float between builds; the lock stays ignored for a
  disposable tool.
- Current-tree verification: the `.gitignore` first line reads "A separate Cargo workspace with no
  committed lock: its dependency versions float between builds."
- Resolution reference: chore(udp-core): correct the manual-verification client lock-file rationale
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282748823>

### F8 - The ban property rarely tests the boundary

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423714>
- Concern: P1 draws `limit` and `errors` independently over `u16`, so the boundary is rare and a
  `>=` mutant is caught by examples, not by P1.
- Solution: kept P1 for limit-dependent defects and added P2, which draws `errors` within two of
  `limit` with saturation; P2 was later renamed in the prose-first review.
- Current-tree verification: `services/banning.rs` has
  `it_should_ban_an_ip_exactly_when_its_errors_exceed_the_limit_for_counts_within_two_of_it`
  using `limit.saturating_add_signed(i32::from(offset % 3))`; 18 banning tests pass.
- Resolution reference: test(udp-core): add a ban threshold property that draws near the limit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282749144>

### F9 - Z1 checks the named type, not the held cipher

- PR number: 2538
- Source review ID: 5522366109
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4274423727>
- Concern: Z1 asserted `ZeroizeOnDrop` on a type spelled in the test, so a change of the wrapped
  cipher would leave it green.
- Solution: Z1 binds the bound to the field through a generic helper, as suggested.
- Current-tree verification: `crypto/cookie_cipher.rs` calls
  `wipes_itself_on_drop(&CookieCipher::fixed_for_testing().0)` with `T: ZeroizeOnDrop`.
- Resolution reference: test(udp-core): check key wiping on the cipher the cookie cipher holds
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2538#discussion_r4282749496>

## Processing Log

- 2026-10-15 07:23 UTC - Fixed F1-F9 in separate signed commits; F1 by an approved history
  rewrite. The audit file was cited in `ISSUE.md` but not yet written.
- 2026-10-15 12:08 UTC - Started this record. Ran the Task Reviewer for F4 (failed, then passed
  after the prose-first AAA review), rebased onto the latest `develop`, pushed, updated the PR body,
  posted the PR conversation response, verified every claim against the current tree, and replied
  on all nine threads. The threads are resolved after this audit is pushed.

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
