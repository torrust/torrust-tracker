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
IDs F1 to F5 are kept unchanged.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2863-f1 | Human | Minor | correctness | ORIGINAL | FIXED | OPEN |
| F2 | review-finding:pr-2863-f2 | Human | Minor | documentation | ORIGINAL | FIXED | OPEN |
| F3 | review-finding:pr-2863-f3 | Human | Nit | documentation | ORIGINAL | FIXED | OPEN |
| F4 | review-finding:pr-2863-f4 | Human | Nit | testing | ORIGINAL | FIXED | OPEN |
| F5 | review-finding:pr-2863-f5 | Human | Nit | formatting | ORIGINAL | FIXED | OPEN |

## Finding Details

### F1 - Deletion premise says every other test binds port 0

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609096>
- Concern: The reason for deleting the nine environment tests said "Every other test binds port `0`", which is false: `tests/metrics/fixed_ports.rs` binds fixed ports and the CLI configuration tests use `reserve_free_ports`. The PR description repeated the sentence.
- Solution: Narrowed the premise to what holds: every user of the `testing` environments (the package tests and examples) binds port `0`, and the fixed-port suites start the main application, not these environments. Commit `docs(issues): [#2393] narrow the deletion premise to users of the testing environments`; the PR description was edited to match.
- Current-tree verification: `rg -n 'Every other test binds' docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing; the examples `http_only_public_tracker.rs` and `udp_only_public_tracker.rs` configure port `0`.
- Resolution reference: pending reply
- Follow-up PR URL: N/A
- Reply URL: pending reply

### F2 - The Goal claims more than the spec allows

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609093>
- Concern: The Goal said the twelve tests come back "with rewrites that cannot race" and that "no test is left ignored for #2393", while the spec keeps a residual `0.0.0.0` collision and a non-Linux ignore.
- Solution: The Goal now splits the twelve into three rewrites that probe an address the test holds and nine restored on a unique loopback address "so they no longer race on `127.0.0.1`", and says no test is left ignored for #2393 "on Linux". Commit `docs(issues): [#2393] qualify the T5 goal to Linux and to 127.0.0.1 races`.
- Current-tree verification: `rg -n 'cannot race' docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing.
- Resolution reference: pending reply
- Follow-up PR URL: N/A
- Reply URL: pending reply

### F3 - Non-Linux reason misstates Windows

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609097>
- Concern: "Other systems route only `127.0.0.1` by default" holds for macOS and the BSDs but not for Windows, which answers on the whole `127.0.0.0/8`.
- Solution: The reason now names macOS and the BSDs, notes that Windows answers on the whole range, and keeps the non-Linux ignore as a conservative choice because CI runs no tests off Linux. Commit `docs(issues): [#2393] state which systems lack the loopback range`; the PR description was edited to match.
- Current-tree verification: `rg -n 'Other systems route' docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing.
- Resolution reference: pending reply
- Follow-up PR URL: N/A
- Reply URL: pending reply

### F4 - The T10 check matches the spec itself

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609102>
- Concern: `rg 'see #2393'` also matches the spec lines that quote the ignore reason, so the T10 check could never come back clean.
- Solution: Scoped the check to Rust sources: `rg -t rust 'see #2393'`. Commit `docs(issues): [#2393] scope the T10 ignore check to Rust sources`.
- Current-tree verification: `rg -n "rg -t rust 'see #2393'" docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints the T10 row.
- Resolution reference: pending reply
- Follow-up PR URL: N/A
- Reply URL: pending reply

### F5 - The word unignored is new to the tree

- PR number: 2863
- Source review ID: 5909778095
- Reviewer finding ID: F5
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2863#discussion_r4577609100>
- Concern: `unignored` in the T8 and T9 rows appears nowhere else in the tree and is not in `project-words.txt`.
- Solution: Both rows now say "run without `#[ignore]`". Commit `docs(issues): [#2393] avoid the word unignored in the T5 plan`. `linter cspell` had passed with the word, so this is wording only.
- Current-tree verification: `rg -c unignored docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md` prints nothing.
- Resolution reference: pending reply
- Follow-up PR URL: N/A
- Reply URL: pending reply

## Processing Log

- 2026-11-15 08:17 UTC - Started audit. da2ce7 reviewed the head ending in
  `docs(issues): [#2393] record the T5 decisions for the ignored port-race tests` at 08:12:28Z
  (APPROVED, five inline threads, none blocking). Each finding was fixed in its own commit.
