---
pr-number: 2944
pr-url: https://github.com/torrust/torrust-tracker/pull/2944
last-updated-utc: "2026-11-18 14:17"
---

<!-- skill-link: process-pr-review -->

# PR #2944 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2944.

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

- Round 1: Copilot review 5957896114 (COMMENTED) with two findings without reviewer IDs, as audit findings F1 and F2.
- Round 2: da2ce7 review 5958264981 (CHANGES_REQUESTED, at the round 1 audit head) with the reviewer's findings F1 to F3, as audit findings F3 to F5.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2944-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2944-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2944-f3 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2944-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2944-f5 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The AC4 evidence counts G2 as isolated and then says it is not

- PR number: 2944
- Source review ID: 5957896114
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4614977060
- Concern: The AC4 evidence row of the EPIC summarizes the inventory as 29 isolated findings and then adds that G2, one of them, is not isolated, so the row contradicts itself.
- Solution: The row now counts G2 once, under "tracked by a subissue" (order 11), with 28 isolated findings, and says that the inventory itself still counts G2 as isolated until order 11 corrects its row.
- Current-tree verification: `grep -n 'isolated (28)' docs/issues/open/2392-test-isolation/EPIC.md` finds the AC4 row, and `grep -c 'counted here as isolated' docs/issues/open/2392-test-isolation/EPIC.md` prints 0.
- Resolution reference: docs(issues): [#2943] count G2 once in the AC4 evidence and cite the edition guide
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615054529

### F2 - The claim about `unsafe` environment functions has no reference or version

- PR number: 2944
- Source review ID: 5957896114
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4614977134
- Concern: The #2943 spec says that the 2024 edition marks `std::env::set_var` and `std::env::remove_var` as `unsafe`, without a reference or a Rust version that a reader can check.
- Solution: The sentence now names the version (2024 edition, stable since Rust 1.85) and links the "Newly unsafe functions" page of the Rust Edition Guide.
- Current-tree verification: `grep -n 'newly-unsafe-functions\|stable since Rust 1.85' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds both in the Background section.
- Resolution reference: docs(issues): [#2943] count G2 once in the AC4 evidence and cite the edition guide
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615055028

### F3 - The `Jail::expect_with` counts are doubled

- PR number: 2944
- Source review ID: 5958264981
- Reviewer finding ID: F1
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615288214
- Concern: The table's first column also counts the `reason` text of the `#[allow]` attribute above each call, so it says 36, 16 and 50 where there are 18, 8 and 25 calls. The spec then says 102 closures and that Option B moves about 100 tests, which doubles that option's cost in the table the maintainer decides from.
- Solution: The table now has 18, 8 and 25 calls and says how they are counted. The text says that 50 of the 51 calls are in `#[test]` functions and one is in a helper that two tests call, so 52 of the 159 tests run in a jail and 107 outside one. Option B says about 50 tests move. The body of GitHub issue #2943 has the corrected counts too.
- Current-tree verification: `grep -cE 'Jail::expect_with\(' packages/configuration/src/lib.rs packages/configuration/src/v2_0_0/mod.rs packages/configuration/src/v3_0_0/mod.rs` prints 18, 8 and 25; `grep -n '102\|about 100' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds only the progress-log entry that records the correction.
- Resolution reference: docs(issues): [#2943] halve the Jail call counts, cite EPIC rule 5 and mark the /tmp file as inferred
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615366339

### F4 - The Goal cites EPIC rule 1 where rule 5 applies, and the options do not weigh it

- PR number: 2944
- Source review ID: 5958264981
- Reviewer finding ID: F2
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615288230
- Concern: The rule the jails break is rule 5 (no process-global changes from a test that shares its process with other tests), not rule 1. With rule 5 cited, Option A needs a recorded exception and Option C is the rule's own remedy, which the options table did not show.
- Solution: The Goal cites and quotes rule 5. The options table has an "EPIC rule 5" column: A is not met and needs an exception recorded in the EPIC, B is not met to the letter and needs a narrower exception, C is met. The recommendation paragraph says that choosing A means recording that exception.
- Current-tree verification: `grep -n 'rule 5' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds the Goal, the table header and the recommendation, and `grep -n 'rule 1' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds only the progress-log entry that records the correction.
- Resolution reference: docs(issues): [#2943] halve the Jail call counts, cite EPIC rule 5 and mark the /tmp file as inferred
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615366986

### F5 - The `/tmp` file is presented as observed, but #2907 inferred it

- PR number: 2944
- Source review ID: 5958264981
- Reviewer finding ID: F3
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615288242
- Concern: Under "What #2907 observed" the spec says the other file went to `/tmp`. The #2907 evidence derives that from the `clear_env()` code and says a search of `/tmp` could not be isolated from other sessions.
- Solution: The sentence now says the other file most likely went to `/tmp`, that #2907 inferred it from the code, and that this is why AC1 asks for a reproduction.
- Current-tree verification: `grep -n 'most likely went to' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds the sentence in the Background section.
- Resolution reference: docs(issues): [#2943] halve the Jail call counts, cite EPIC rule 5 and mark the /tmp file as inferred
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615367609

## Processing Log

- 2026-11-18 13:54 UTC - Round 1: Copilot review 5957896114 with F1 and F2. Committed and pushed the fix (`docs(issues): [#2943] count G2 once in the AC4 evidence and cite the edition guide`), then replied to the two threads (13:53:25Z and 13:53:27Z).
- 2026-11-18 13:54 UTC - The validator on the worktree record reported 2 rows, 1 log entry, 0 failures. Resolved the two round 1 threads (about 13:54:38Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed these entries.
- 2026-11-18 14:16 UTC - Round 2: da2ce7 review 5958264981 with F3 to F5 (the reviewer's F1 to F3). Committed and pushed the fix (`docs(issues): [#2943] halve the Jail call counts, cite EPIC rule 5 and mark the /tmp file as inferred`), corrected the counts in the body of issue #2943, then replied to the three threads (14:16:01Z to 14:16:07Z).
- 2026-11-18 14:17 UTC - The validator on the worktree record reported 5 rows, 3 log entries, 0 failures. Resolved the three round 2 threads (about 14:17:01Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh showed two more unresolved threads, from a Copilot review posted at 14:10:52Z that this round had not read; they are round 3. Committed these entries.

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
