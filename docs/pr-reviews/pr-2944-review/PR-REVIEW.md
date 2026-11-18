---
pr-number: 2944
pr-url: https://github.com/torrust/torrust-tracker/pull/2944
last-updated-utc: "2026-11-18 14:45"
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
- Round 3: Copilot review 5958271471 (COMMENTED, at the round 1 fix head) with two findings without reviewer IDs, as audit findings F6 and F7.
- Round 4: da2ce7 review 5958625555 (posted as an approval at the round 3 audit head; the rebase push then dismissed it) with the reviewer's findings F4 and F5, neither blocking, as audit findings F8 and F9.
- Round 5: Copilot review 5958719165 (COMMENTED, at the rebased head) with three findings, as audit findings F10 to F12. The second comment carries the label `[Minor][F9]` in its own text.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2944-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2944-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2944-f3 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2944-f4 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2944-f5 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2944-f6 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2944-f7 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2944-f8 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2944-f9 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2944-f10 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2944-f11 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2944-f12 | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |

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

### F6 - The `Jail` usage is pinned to a commit id and claims that no other package uses it

- PR number: 2944
- Source review ID: 5958271471
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615293756
- Concern: The usage table cites a `develop` commit id and says that no other package uses `Jail`, a claim that can become stale and that a reader cannot recompute from the spec.
- Solution: The sentence now gives the drafting date and the command that produced the list of files, drops the commit id, and says that T1 repeats the counts.
- Current-tree verification: `grep -n 'grep -rl .Jail.' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds the command in the Background section, and `grep -c 'bc9d0590c' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` prints 0.
- Resolution reference: docs(issues): [#2943] record how the Jail usage was counted and require the toolchain version in the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615457059

### F7 - `cargo +stable` in V1 and V2 is a moving toolchain

- PR number: 2944
- Source review ID: 5958271471
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615293864
- Concern: V1 and V2 use `cargo +stable`, which moves, so a later recheck may run on another toolchain than the reproduction.
- Solution: The evidence policy now asks for the output of `rustc -Vv` and `cargo -V` with every recorded result, and for V2 to run on the same toolchain version as V1 or to record why it could not. The command keeps `+stable`, the form the repository's specs use.
- Current-tree verification: `grep -n 'rustc -Vv' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds the evidence policy.
- Resolution reference: docs(issues): [#2943] record how the Jail usage was counted and require the toolchain version in the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615457577

### F8 - A Processing Log entry was rewritten in place

- PR number: 2944
- Source review ID: 5958625555
- Reviewer finding ID: F4
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615597101
- Concern: A commit changed the 14:17 entry of this record's Processing Log in place to correct its refresh result. The log is append-only: the entry keeps its text and a new entry records the correction.
- Solution: The 14:17 entry has its first text again, and an appended 14:33 entry records the correction and names the commit that had rewritten it.
- Current-tree verification: `grep -n '^- 2026-11-18 14:17' docs/pr-reviews/pr-2944-review/PR-REVIEW.md` prints the entry with "found no unresolved thread" again, and `grep -n 'Correction of the 14:17 entry' docs/pr-reviews/pr-2944-review/PR-REVIEW.md` finds the appended entry.
- Resolution reference: docs(pr-reviews): [#2943] keep the PR 2944 audit log and the spec progress log append-only
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615673418

### F9 - A spec progress-log entry was extended with a later change

- PR number: 2944
- Source review ID: 5958625555
- Reviewer finding ID: F5
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615597110
- Concern: The 14:12 progress-log entry of the #2943 spec was extended with the changes of a commit made at 14:19, instead of a new entry stamped after that change.
- Solution: The 14:12 entry has its first text again, and a separate 14:33 entry records the usage-table and evidence-policy changes and the time of their commit.
- Current-tree verification: `grep -n '^- 2026-11-18 14:' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds the 14:12 and the 14:33 entries.
- Resolution reference: docs(pr-reviews): [#2943] keep the PR 2944 audit log and the spec progress log append-only
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615674076

### F10 - An absent `TMPDIR` would count as interference

- PR number: 2944
- Source review ID: 5958719165
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615676812
- Concern: The probe counts an unset `TMPDIR` as interference, but V1 did not require the variable to be set before the run, so a machine without `TMPDIR` would give a false reproduction and a failing V2.
- Solution: V1 runs with `TMPDIR` set to a fresh empty directory and `PATH` set, the probe asserts both at its start, and V2 repeats the same environment setup.
- Current-tree verification: `grep -n 'TMPDIR=<fresh empty dir>\|asserts that at its start' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds V1 and step 1 of the bug-fix process.
- Resolution reference: docs(issues): [#2943] record the drafting reproduction and make the recheck depend on the design
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615793881

### F11 - No reproduction attempt was recorded while drafting

- PR number: 2944
- Source review ID: 5958719165
- Reviewer finding ID: F9
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615676913
- Concern: `fix-bug` asks for a reproduction attempt before the spec review, recorded in the issue folder with one of three outcomes. The folder held only the spec, and the `/tmp` explanation was an inference.
- Solution: The reproduction was run: a probe test outside `Jail` read `PATH` unset, `TMPDIR` unset and a changed current directory in 3 of 3 runs. `manual-verification-evidence.md` records the probe's code, the commands, the toolchain and the output, with the outcome Reproduced. The spec's Background cites it, and T1 repeats the baseline.
- Current-tree verification: `grep -n 'Reproduced' docs/issues/open/2943-2392-figment-jail-process-environment/manual-verification-evidence.md` finds the outcome, and `grep -c '^PROBE reads=' docs/issues/open/2943-2392-figment-jail-process-environment/manual-verification-evidence.md` prints 3.
- Resolution reference: docs(issues): [#2943] record the drafting reproduction and make the recheck depend on the design
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615794640

### F12 - Under option A the unchanged probe cannot pass V2

- PR number: 2944
- Source review ID: 5958719165
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615676986
- Concern: Option A leaves `clear_env()` running in the probe's process, so the unchanged probe outside `Jail` still reads unset variables after the fix. The recommended design could not meet AC2 and V2 as written.
- Solution: AC2 and V2 use the recheck that T2 defines for the chosen design. The unchanged V1 probe is that recheck only for options B and C; step 3 of the bug-fix process says what the recheck is under A, and T2's output includes it.
- Current-tree verification: `grep -in 'the recheck T2 defines for the chosen design' docs/issues/open/2943-2392-figment-jail-process-environment/ISSUE.md` finds AC2 and V2.
- Resolution reference: docs(issues): [#2943] record the drafting reproduction and make the recheck depend on the design
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2944#discussion_r4615795155

## Processing Log

- 2026-11-18 13:54 UTC - Round 1: Copilot review 5957896114 with F1 and F2. Committed and pushed the fix (`docs(issues): [#2943] count G2 once in the AC4 evidence and cite the edition guide`), then replied to the two threads (13:53:25Z and 13:53:27Z).
- 2026-11-18 13:54 UTC - The validator on the worktree record reported 2 rows, 1 log entry, 0 failures. Resolved the two round 1 threads (about 13:54:38Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed these entries.
- 2026-11-18 14:16 UTC - Round 2: da2ce7 review 5958264981 with F3 to F5 (the reviewer's F1 to F3). Committed and pushed the fix (`docs(issues): [#2943] halve the Jail call counts, cite EPIC rule 5 and mark the /tmp file as inferred`), corrected the counts in the body of issue #2943, then replied to the three threads (14:16:01Z to 14:16:07Z).
- 2026-11-18 14:17 UTC - The validator on the worktree record reported 5 rows, 3 log entries, 0 failures. Resolved the three round 2 threads (about 14:17:01Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread. Committed these entries.
- 2026-11-18 14:22 UTC - Round 3: Copilot review 5958271471 with F6 and F7. Committed and pushed the fix (`docs(issues): [#2943] record how the Jail usage was counted and require the toolchain version in the evidence`) together with the round 2 entries and their correction (`docs(pr-reviews): [#2943] correct the round 2 refresh result in the PR 2944 audit`), then replied to the two threads (14:22:25Z and 14:22:28Z).
- 2026-11-18 14:22 UTC - Resolved the two round 3 threads (about 14:22:47Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh found no unresolved thread, and the review list showed no review newer than 5958271471. The validator ran on the worktree record after these entries. Committed these entries.
- 2026-11-18 14:27 UTC - Rebased the branch onto `develop` after PR #2947 (the #2907 close-out) merged, because both changed `docs/issues/open/2392-test-isolation/EPIC.md`. Kept both progress-log entries and the AC2 row of `develop`, and pointed the #2943 spec's link to the #2907 spec at `docs/issues/closed/`. No finding changes.
- 2026-11-18 14:33 UTC - Correction of the 14:17 entry. Its statement "a GraphQL refresh found no unresolved thread" was wrong: the refresh showed two more unresolved threads, from Copilot review 5958271471 posted at 14:10:52Z, which round 2 had not read; they became round 3. An earlier commit (`docs(pr-reviews): [#2943] correct the round 2 refresh result in the PR 2944 audit`) had rewritten that entry in place; this commit restores its original text, because the log is append-only, and records the correction here.
- 2026-11-18 14:37 UTC - Round 4: da2ce7 review 5958625555 with F8 and F9 (the reviewer's F4 and F5). Committed and pushed the fix (`docs(pr-reviews): [#2943] keep the PR 2944 audit log and the spec progress log append-only`), then replied to the two threads (14:37:19Z and 14:37:22Z) and resolved them (about 14:37:43Z). The GraphQL refresh then showed three unresolved threads, from Copilot review 5958719165 posted at 14:37:34Z; they are round 5. The commit `docs(pr-reviews): [#2943] audit review round 4 of PR 2944` added the F8 and F9 rows without this entry, which is written at 14:46 UTC.
- 2026-11-18 14:45 UTC - Round 5: Copilot review 5958719165 with F10 to F12. Ran the reproduction that F11 asks for (a temporary probe test, reverted, 3 of 3 runs) and recorded it in the spec folder. Committed and pushed the fix (`docs(issues): [#2943] record the drafting reproduction and make the recheck depend on the design`), then replied to the three threads (14:45:24Z to 14:45:29Z).
- 2026-11-18 14:45 UTC - Resolved the three round 5 threads (about 14:45:55Z), selecting only unresolved threads whose last comment is a `josecelano-bot` reply; a GraphQL refresh showed 2 unresolved thread(s). The validator ran on the worktree record after these entries. Committed these entries. This was the fifth round of fixes; further rounds wait for the maintainer.

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
