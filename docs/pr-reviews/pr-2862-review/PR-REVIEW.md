---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2862 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2862>.

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

Round 1 had two reviews on the head ending in `docs(issues): [#2392] sync the test isolation EPIC
after the #2393 patch`: da2ce7's review
5909721063 (findings F1 to F4, reviewer-provided IDs) and Copilot's review 5909726857 (three inline
comments, assigned F5 to F7). Each Copilot comment asks for the same current-tree change as one of
da2ce7's findings, so it is recorded as a re-raise. Neither review body has a separate actionable
assertion; da2ce7's "Checked, no finding" list and Copilot's overview are context only.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2862-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2862-f2 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2862-f3 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2862-f4 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2862-f5 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F4 | NO_ACTION | SUPERSEDED |
| F6 | review-finding:pr-2862-f6 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F1 | NO_ACTION | SUPERSEDED |
| F7 | review-finding:pr-2862-f7 | Copilot | Minor (inferred) | correctness | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The 150-run recheck is attributed to the final 21 ignored tests

- PR number: 2862
- Source review ID: 5909721063
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4577562483>
- Concern: the order-1 row and AC2 of the EPIC say 150 of 150 runs passed with 21 tests ignored, but the 150-run recheck (recheck 2) had 18 ignored; the final state with 21 ignored had only check 3, 20 runs. The Phase 1 state line reads correctly only if "the racing tests" means the 18 ignored at that head.
- Solution: the order-1 row, the Phase 1 state and the AC2 evidence now say 150 of 150 runs with the 18 tests ignored at that head, and 20 of 20 with the final 21.
- Current-tree verification: `manual-verification-evidence.md` of #2393 records recheck 2 as `150 runs: 1808 tests run: 1808 passed, 18 skipped` and check 3 as `20 runs: 1806 tests run: 1806 passed, 21 skipped`; `grep -n "18 tests ignored at that head" docs/issues/open/2392-test-isolation/EPIC.md` lists lines 103 and 244, and line 154 carries the Phase 1 wording.
- Resolution reference: docs(issues): [#2392] name the 18 and 21 ignored tests behind each #2393 recheck
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578310710>

### F2 - The shape list leaves out the released-port-probed-dead shape

- PR number: 2862
- Source review ID: 5909721063
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4577562487>
- Concern: three of the 21 ignored tests (`it_should_return_error_when_{api,http,udp}_service_was_stopped_after_registration`) stop a service and expect its released port to be dead, which is none of the three listed shapes; the #2394 Network ports row inherits the gap.
- Solution: added a fourth bullet, "Released port probed and expected dead", to the EPIC's shape list, and the matching clause to the #2394 Network ports row.
- Current-tree verification: `grep -n "probed and expected dead"` finds the bullet in `docs/issues/open/2392-test-isolation/EPIC.md` (line 136) and the clause in `docs/issues/open/2394-2392-audit-shared-test-resources/ISSUE.md` (line 53).
- Resolution reference: docs(issues): [#2392] add the released-port-probed-dead racing shape
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578310853>

### F3 - T2 leaves collisions between concurrent runs unaudited

- PR number: 2862
- Source review ID: 5909721063
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4577562492>
- Concern: T2 treated every Rust-test port as Covered by the #2393 inventory, which addresses races within one suite run, not collisions between concurrent runs on one host; `tests/metrics/fixed_ports.rs` (fixed `17091`-`17094`) and the `reserve_free_ports` users would get no disposition.
- Solution: T2 now marks Covered only the rows #2393 owns (its ignored and rewritten tests) and still audits Rust tests for collisions between concurrent runs, naming the fixed-port metrics suite and the `reserve_free_ports` users in `tests/configuration/`, besides E2E runners, Compose files, container test scripts and CI workflow steps.
- Current-tree verification: line 97 of `docs/issues/open/2394-2392-audit-shared-test-resources/ISSUE.md` carries the new T2 row; `tests/metrics/fixed_ports.rs` binds `0.0.0.0:17091` to `0.0.0.0:17094`.
- Resolution reference: docs(issues): [#2394] keep cross-run Rust port collisions in the T2 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578310998>

### F4 - Two of the four rewritten tests still bind a real port

- PR number: 2862
- Source review ID: 5909721063
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4577562495>
- Concern: the order-1 row said #2393 "rewrote 4 that needed no real port", but two of them bind a real port-`0` socket; the #2393 T4 row's wording is "never needed a racing port".
- Solution: the order-1 row now says "rewrote 4 that never needed a racing port".
- Current-tree verification: `grep -n "never needed a racing port" docs/issues/open/2392-test-isolation/EPIC.md` matches line 103, and "needed no real port" no longer occurs in the file.
- Resolution reference: docs(issues): [#2392] say the rewritten #2393 tests never needed a racing port
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578311102>

### F5 - Rewritten tests are described as needing no real port

- PR number: 2862
- Source review ID: 5909726857
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4577568080>
- Concern: the same as F4.
- Solution: no separate change; F4's fix covers it.
- Current-tree verification: as for F4.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578311215>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578311215>

### F6 - AC2 does not distinguish the 18-skip and 21-skip rechecks

- PR number: 2862
- Source review ID: 5909726857
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4577568131>
- Concern: the same as F1.
- Solution: no separate change; F1's fix covers it.
- Current-tree verification: as for F1.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578311332>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578311332>

### F7 - T2 leaves remaining shared Rust test ports unaudited

- PR number: 2862
- Source review ID: 5909726857
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4577568169>
- Concern: the same as F3.
- Solution: no separate change; F3's fix covers it.
- Current-tree verification: as for F3.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578311448>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2862#discussion_r4578311448>

## Processing Log

- 2026-11-15 12:08 UTC - Committed the F1 fix, then the F4, F2 and F3 fixes and the spec progress-log entries (last commit 12:11 UTC), one commit each.
- 2026-11-15 12:12 UTC - Replied on all seven threads (12:12:39 to 12:12:52 UTC). The audit record was not created before the first reply, as the skill requires; this is the late start.
- 2026-11-15 12:13 UTC - Started audit: created this record from the template with the reply URLs.
- 2026-11-15 12:17 UTC - Before the push, the PR-branch commit-id check flagged the round-1 head id in the Findings introduction; replaced it with that commit's subject.

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
