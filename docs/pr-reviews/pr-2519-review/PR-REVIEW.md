---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/benchmarking.md
    - docs/issues/open/2518-http-first-announce-benchmark/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2519 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2519>.

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

Copilot review 5514492175 (submitted 2026-10-14 07:31:55 UTC, before merge) left two inline
findings without a `[<Severity>][<FindingId>]` header, so they are assigned the audit-local IDs
`F1` and `F2` in source order. Their severity is inferred as `Minor` from the "Low severity" badges
in the review overview. The overview repeats the two findings and adds no other request, so it adds
no row.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2519-f1` | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2519-f2` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Compare first-announce results only with each other

- PR number: 2519
- Source review ID: 5514492175
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4267966296>
- Concern: AC4 required every first-announce result to be above the repeat-announce
  (`announce_once`) result, but D1 and the Risks section say the two benchmarks use different
  timing strategies and must never be compared. The completion criteria contradicted the design.
- Solution: AC4 now requires only that the 74- and 1000-peer results are above the 0-peer result,
  or that the evidence explains why not, and states that the results are compared only with each
  other, never with `announce_once` (D1).
- Current-tree verification: AC4 in `ISSUE.md` no longer mentions the repeat-announce result; the
  remaining mentions of `announce_once` are D1's statement that the two are never compared, the
  Risks section, and the code-layout check that re-measures `announce_once` against itself.
- Resolution reference: `docs(issues): [#2518] compare first-announce results only with each other`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268395667>

### F2 - Record each run's Criterion medians in M2

- PR number: 2519
- Source review ID: 5514492175
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4267966392>
- Concern: M2 ran the benchmark three times without saying when to read the medians. Criterion
  overwrites `new/estimates.json` on every run, and `docs/benchmarking.md` requires reading the
  median immediately after each run, so M2 could keep only the last run and could not show
  run-to-run variation.
- Solution: M2 now records `uptime` before each run and, immediately after each run, reads the
  median of every swarm size from `target/criterion/http_tracker_first_announce/<N>/new/estimates.json`.
  Its expected result is a table of three medians per size with each run's load average and the
  toolchain (`rustc --version`), stating the variation per size.
- Current-tree verification: the M2 row of the Manual Verification Scenarios table in `ISSUE.md`
  contains these steps and the expected result; `linter markdown` and `linter cspell` pass.
- Resolution reference: `docs(issues): [#2518] record each run's Criterion medians in M2`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2519#discussion_r4268395950>

## Processing Log

- 2026-10-14 08:15 UTC - Started audit: fetched review 5514492175 and its two unresolved threads
  with the `github-review-threads` tool.
- 2026-10-14 08:20 UTC - Pushed one signed fix commit per finding (F1, F2); the pre-push checks
  passed. Replied on both threads with the disposition, verification, and resolution reference.
- 2026-10-14 08:22 UTC - Recorded this audit; the threads are resolved after it is committed and
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
