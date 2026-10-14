---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/benchmarking.md
    - docs/issues/closed/2479-fix-http-core-announce-benchmark/ISSUE.md
    - docs/issues/closed/2479-fix-http-core-announce-benchmark/implementation-retrospective.md
    - docs/issues/closed/2479-fix-http-core-announce-benchmark/manual-verification-evidence.md
---

<!-- skill-link: process-pr-review -->

# PR #2509 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2509>.

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

Copilot review 5508173139 (submitted 2026-10-13 18:29:37 UTC, before merge) left one inline
finding headed `[Minor][F1]`. The reviewer-provided ID `F1` does not collide with any earlier
audit finding, so it is used unchanged. The review overview repeats that finding and recommends
approval; it adds no other request, so it adds no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2509-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Attribute the connect_once layout shift to the control build, not the committed guard

- PR number: 2509
- Source review ID: 5508173139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2509#discussion_r4262490424>
- Concern: `docs/benchmarking.md` said that adding the routine guard to `connect_once`, "with
  nothing added to its timed loop", moved it from about 58 ns to about 65 ns. The committed guard
  does add `guard.track(...)` and an atomic increment to the timed loop; only the control build in
  the evidence kept the loop byte-identical. Readers could infer that the active guard has no
  timed-loop instrumentation.
- Solution: Step 3 of "Recording Before-and-After Evidence" now attributes the 58 to 65 ns shift to
  a control build that added the guard's code only outside the timed loop, and states that the
  committed guard, which does add `track` to the loop, measures the same about 65 ns. The issue
  retrospective had the same ambiguity and was corrected in the same commit.
- Current-tree verification: `docs/benchmarking.md` lines 81-85 describe the control build and
  the committed guard separately; `implementation-retrospective.md` ("What Changed During
  Implementation") does the same; the variant table in `manual-verification-evidence.md` already
  listed the control and the committed `inspect` variant as separate rows and is unchanged.
- Resolution reference: `docs(benchmarking): [#2479] attribute the connect_once layout shift to the control build`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2509#discussion_r4263284865>

## Processing Log

- 2026-10-13 19:47 UTC - Started audit: fetched review 5508173139 and its one unresolved thread with
  the `github-review-threads` tool.
- 2026-10-13 19:49 UTC - Pushed the signed fix commit for F1; the pre-push checks passed. Replied
  on the thread with the disposition, verification, and resolution reference.
- 2026-10-13 19:52 UTC - Recorded this audit; the thread is resolved after it is committed and
  pushed.

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
