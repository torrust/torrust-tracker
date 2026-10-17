---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2558 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2558>.

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

PR #2558 is the S1 pull request for #1349 and was open when its review was processed, so this is the
normal pre-merge workflow. Copilot's review 5557264376 (COMMENTED, 2026-10-17 09:46:28 UTC, at the
head `docs(issues): [#1349] record the S1 pull request #2558 in the specification`) has one inline
finding, F1. The reviewer wrote `[Minor][F1]` in the comment, and the overview marks it "Low
severity"; the comment's own tag is used. Its "Approval recommended" verdict and change summary are
non-actionable and add no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2558-f1` | Copilot | Minor | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The precedence test needs an external IP in the policy the plan refactors

- PR number: 2558
- Source review ID: 5557264376
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4302250313>
- Concern: the announce plan's test review treated the precedence test as having only a
  predicate-built expectation and an `is_ok()` assertion, but the test builds its service with
  `PeerIpSelectionPolicy::enabled()`, whose `external_ip` is `None`, so the external IP in its
  fixture never reaches the service and the test cannot detect the external IP taking precedence.
- Solution: the plan's test review now records the missing setup, and R1 gains a step that gives
  the test a policy with overrides enabled and a distinct external IP, and confirms it fails under
  a temporary change that prefers the external IP. This amends the approved plan a second time; the
  plan log asks the maintainer to confirm it when reviewing the R1 result.
- Current-tree verification: `PeerIpSelectionPolicy::enabled()` sets `external_ip: None`
  (`announce.rs` lines 89-93) and the test passes it at line 859; the plan's review row and the R1
  step both name the setup (`grep -n "external IP"` on the plan).
- Resolution reference: docs(issues): [#1349] add the external-IP setup to the precedence test in the announce plan
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2558#discussion_r4302644616>

## Processing Log

- 2026-10-17 09:46 UTC - Copilot submitted review 5557264376 with the one inline finding.
- 2026-10-17 09:48 UTC - Found the review while polling every two minutes; verified the claim
  against `announce.rs`.
- 2026-10-17 09:49 UTC - Edited the plan to record the smell and the R1 step.
- 2026-10-17 12:21 UTC - Committed the fix after a GPG passphrase timeout was retried with the
  maintainer's approval.
- 2026-10-17 12:23 UTC - Pushed, replied on the thread and resolved it.
- 2026-10-17 12:24 UTC - Recorded this audit.

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
