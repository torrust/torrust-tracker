# PR #2607 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2607>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2607-f1 | Copilot | Major (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2607-f2 | Copilot | Major (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2607-f3 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Reproduction evidence is missing before spec review

- PR number: 2607
- Source review ID: 5598940189
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2607#discussion_r4336214961>
- Concern: The spec says the reproduction is recorded in `manual-verification-evidence.md`, but the
  issue folder held only `ISSUE.md`. The `fix-bug` skill requires the reproduction evidence before
  the maintainer reviews the spec. The reviewer asked for the commands, runtime versions, ref states,
  output, and classified outcome, with the final green recheck left pending.
- Solution: Ran the reproduction commands one at a time and recorded them in the new
  `manual-verification-evidence.md` (V1 and V2, outcome Reproduced). Updated T1, the Bug-Fix
  Process section, and the M1 and M2 rows. The green recheck stays pending.
- Current-tree verification: Read `manual-verification-evidence.md` in
  `docs/issues/open/2606-2278-validate-audit-record-merged-fix-subjects/` after the fix: it holds
  `python3 --version`, `gh --version`, the three ref states, the output of five commands, and the
  outcome "Reproduced"; the spec's T1 row reads `DONE`.
- Resolution reference: docs(issues): record the #2606 reproduction evidence before spec review
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2607#discussion_r4336496960>

### F2 - Test-design review gates are missing from the plan

- PR number: 2607
- Source review ID: 5598940189
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2607#discussion_r4336215050>
- Concern: T2 and T3 specified red and green runs but not the incremental test-design review the
  `create-issue` skill requires: the `write-unit-test` skill, behavior-sized increments, the
  prose-first Arrange-Act-Assert comparison, and a maintainer review before final verification and
  the commit. The red evidence and the production-fix step must stay separate.
- Solution: Rewrote the Implementation Plan and Commit Points into three test increments, each with
  kept red output and a recorded design review, separate production steps, and a maintainer review
  stop (T8) before final verification, the last commit, and the PR.
- Current-tree verification: Read the "Implementation Plan" and "Commit Points" sections of the spec
  after the fix: T2, T4, and T6 are test increments, T3, T5, and T7 are production steps, and T8 is
  the maintainer review stop; the introduction names the `write-unit-test` skill and the prose-first
  comparison.
- Resolution reference: docs(issues): require test-design review gates in the #2606 plan
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2607#discussion_r4336497368>

### F3 - Scenario M2 does not update local `develop` reliably

- PR number: 2607
- Source review ID: 5598940189
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2607#discussion_r4336215100>
- Concern: `git pull --ff-only torrust develop` updates the checked-out branch, so on the
  implementation branch it can fail or leave local `develop` stale, and two identical results would
  not support AC2. The reviewer asked to update `develop` separately and keep `HEAD` unchanged.
- Solution: M2 now updates `develop` with `git fetch torrust develop:develop` and records
  `git rev-parse HEAD`, `develop`, and `torrust/develop` before and after, so `HEAD` is shown unchanged.
- Current-tree verification: Read the M2 row of the spec's Manual Verification Scenarios table after
  the fix: it names `git fetch torrust develop:develop` and the before and after ref records.
- Resolution reference: docs(issues): update local develop separately in the #2606 M2 scenario
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2607#discussion_r4336497655>

## Processing Log

- 2026-10-21 12:00 UTC - Processed Copilot review 5598940189 (head a18cd04a7): three findings, F1 to F3.
  Fixed each in its own commit (named above), pushed, replied on each thread, and resolved the three
  threads. A GraphQL refresh then showed 0 unresolved threads, and no pending review exists.

Append entries only. If an entry was rewritten in place, restore the prior entries, then append a
correction that names the rewritten content; do not rewrite the record again.

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
