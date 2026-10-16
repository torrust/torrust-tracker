---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2552 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2552>.

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

Round 1: Copilot's review 5545198548 (COMMENTED, 2026-10-16 08:44:03 UTC, on the PR's first head)
carries two inline findings, F1 and F2. Its body is an overview and adds no further row. Copilot
marked F1 with a medium and F2 with a low severity icon, so both severities are inferred.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2552-f1` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2552-f2` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - `f64::EPSILON` is not a general absolute tolerance for the gauge assertions

- PR number: 2552
- Source review ID: 5545198548
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4292605809>
- Concern: the two assertions this PR changed in `packages/udp-server/src/statistics/repository.rs`
  compare the gauge with `1000.0` and `2000.0` using `f64::EPSILON`, the machine epsilon around
  1.0. That is not a generally correct absolute tolerance, so the test could fail on a value that
  is merely very close.
- Solution: compare with an absolute tolerance of `1e-9` instead. `assert_eq!` was not used because
  `clippy::float_cmp` is denied. The unchanged `f64::EPSILON` assertions elsewhere in the file
  predate this PR and were left alone.
- Current-tree verification: `grep -n 1e-9 packages/udp-server/src/statistics/repository.rs` shows
  both assertions, and `cargo test -p torrust-tracker-udp-server --lib statistics::repository`
  passes (16 tests).
- Resolution reference: test(udp-server): [#2244] use an absolute tolerance for the gauge assertions
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293675643>

### F2 - The cast reasons say the cast saturates above 2^53

- PR number: 2552
- Source review ID: 5545198548
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4292605882>
- Concern: the `#[expect]` reason of each `whole_total` helper said the `f64` to `u64` cast is
  exact below 2^53 and saturates above it. A float to integer cast saturates only for NaN,
  infinities, negative values, and values beyond `u64::MAX`. Between 2^53 and `u64::MAX` the `f64`
  may already have been rounded by the earlier conversion. The reviewer asked for the same fix in
  the HTTP core and UDP core helpers, which use the same text.
- Solution: the reasons in the HTTP core, UDP core, and UDP server helpers now say the cast is
  exact up to 2^53, that beyond that the `f64` may already be rounded, and that the cast itself
  saturates only above `u64::MAX`.
- Current-tree verification: `grep -rn "saturates above it" packages --include=*.rs` finds nothing,
  the new wording is in all three helpers, and Clippy with `-D warnings` is clean for the three
  packages, so each `#[expect]` still fires.
- Resolution reference: docs(statistics): [#2244] correct the float-to-integer cast reasons
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2552#discussion_r4293675987>

## Processing Log

- 2026-10-16 10:40 UTC - Started audit after Copilot's review 5545198548. Fixed F1 and F2 in
  separate commits, pushed them, and replied on both threads.

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
