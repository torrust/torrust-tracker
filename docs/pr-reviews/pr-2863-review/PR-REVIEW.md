---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2863 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2863>.

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

F1 to F5 come from the first da2ce7 review (APPROVED, findings not blocking), on the head ending in
`docs(issues): [#2393] record the T5 decisions for the ignored port-race tests`. The reviewer's own
IDs F1 to F5 are kept unchanged. F6 comes from the first Copilot review (COMMENTED), on the same head;
its own ID R1 is kept as `Reviewer finding ID`. The push of the F1 to F5 fixes dismissed the da2ce7
approval.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2863-f1 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2863-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2863-f3 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2863-f4 | Human | Nit | testing | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2863-f5 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2863-f6 | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Deletion premise says every other test binds port 0

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609096>
- Concern: The reason for deleting the nine environment tests said "Every other test binds port `0`", which is false: `tests/metrics/fixed_ports.rs` binds fixed ports and the CLI configuration tests use `reserve_free_ports`. The PR description repeated the sentence.
- Solution: Narrowed the premise to what holds: every user of the `testing` environments (the package tests and examples) binds port `0`, and the fixed-port suites start the main application, not these environments. Commit `docs(issues): [#2393] narrow the deletion premise to users of the testing environments`; the PR description was edited to match.
- Current-tree verification: `rg -n 'Every other test binds' docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing; the examples `http_only_public_tracker.rs` and `udp_only_public_tracker.rs` configure port `0`.
- Resolution reference: docs(issues): [#2393] narrow the deletion premise to users of the testing environments
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577631154>

### F2 - The Goal claims more than the spec allows

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609093>
- Concern: The Goal said the twelve tests come back "with rewrites that cannot race" and that "no test is left ignored for #2393", while the spec keeps a residual `0.0.0.0` collision and a non-Linux ignore.
- Solution: The Goal now splits the twelve into three rewrites that probe an address the test holds and nine restored on a unique loopback address "so they no longer race on `127.0.0.1`", and says no test is left ignored for #2393 "on Linux". Commit `docs(issues): [#2393] qualify the T5 goal to Linux and to 127.0.0.1 races`.
- Current-tree verification: `rg -n 'cannot race' docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): [#2393] qualify the T5 goal to Linux and to 127.0.0.1 races
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577631339>

### F3 - Non-Linux reason misstates Windows

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609097>
- Concern: "Other systems route only `127.0.0.1` by default" holds for macOS and the BSDs but not for Windows, which answers on the whole `127.0.0.0/8`.
- Solution: The reason now names macOS and the BSDs, notes that Windows answers on the whole range, and keeps the non-Linux ignore as a conservative choice because CI runs no tests off Linux. Commit `docs(issues): [#2393] state which systems lack the loopback range`; the PR description was edited to match.
- Current-tree verification: `rg -n 'Other systems route' docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): [#2393] state which systems lack the loopback range
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577631558>

### F4 - The T10 check matches the spec itself

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609102>
- Concern: `rg 'see #2393'` also matches the spec lines that quote the ignore reason, so the T10 check could never come back clean.
- Solution: Scoped the check to Rust sources: `rg -t rust 'see #2393'`. Commit `docs(issues): [#2393] scope the T10 ignore check to Rust sources`.
- Current-tree verification: `rg -n "rg -t rust 'see #2393'" docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints the T10 row.
- Resolution reference: docs(issues): [#2393] scope the T10 ignore check to Rust sources
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577631745>

### F5 - The word unignored is new to the tree

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609100>
- Concern: `unignored` in the T8 and T9 rows appears nowhere else in the tree and is not in `project-words.txt`.
- Solution: Both rows now say "run without `#[ignore]`". Commit `docs(issues): [#2393] avoid the word unignored in the T5 plan`. `linter cspell` had passed with the word, so this is wording only.
- Current-tree verification: `rg -c unignored docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing.
- Resolution reference: docs(issues): [#2393] avoid the word unignored in the T5 plan
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577631952>

### F6 - No test-design review per T5 test increment

- PR number: 2863
- Source review ID: 5909791104
- Reviewer finding ID: R1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577616907>
- Concern: V5 required only red-then-green results, so T8 and T9 could finish without the prose-first Arrange-Act-Assert design review that the `create-issue` skill requires after each passing test-producing increment, nor the maintainer review after the final increment.
- Solution: The Bug-Fix Process now requires the `write-unit-test` loop for T8 and T9 (smallest increments, a recorded prose-first Arrange-Act-Assert review after each and before the next, a maintainer stop after the final increment before T10, the final commit or the pull request). T8, T9, the workflow checkpoint, M7 and AC7 reference it as V7. Commit `docs(issues): [#2393] require a test-design review after each T5 test increment`.
- Current-tree verification: `rg -n 'prose-first|V7|AC7' docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints the Bug-Fix Process paragraph, the T8 and T9 rows, the checkpoint, AC7 and M7.
- Resolution reference: docs(issues): [#2393] require a test-design review after each T5 test increment
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577639763>

## Processing Log

- 2026-11-15 08:17 UTC - Started audit. da2ce7 reviewed the head ending in
  `docs(issues): [#2393] record the T5 decisions for the ignored port-race tests` at 08:12:28Z
  (APPROVED, five inline threads, none blocking). Each finding was fixed in its own commit.
- 2026-11-15 08:23 UTC - Copilot reviewed the same head at 08:15:41Z (COMMENTED, one inline thread,
  F6), after the review above was read. F1 to F5 were pushed with the head ending in
  `docs(pr-reviews): [#2393] create the PR 2863 review audit record`, and their replies were posted
  at 08:20:34Z to 08:20:44Z. Resolving the threads by a loop over every unresolved thread also
  resolved the F6 thread before it was read; it was reopened at once, read, fixed in its own
  commit, replied to at 08:22 and resolved again. All six threads are resolved.
