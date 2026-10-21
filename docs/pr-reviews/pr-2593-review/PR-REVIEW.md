---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
    - docs/issues/open/1349-1347-add-tests-http-core/test-refactor-plans/container-tests.md
---

<!-- skill-link: process-pr-review -->

# PR #2593 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2593>.

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

Copilot's review 5596527098 (COMMENTED, 2026-10-21 07:50:54 UTC) read the head whose subject is
"docs(issues): [#1349] record the S4 pull request in the specification". It carries one inline thread,
F1. Its body says "Changes recommended", labels the finding with a medium-severity icon and repeats
the inline thread, so the body adds no row. No other review had been submitted when this audit started.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2593-f1 | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Test event delivery when usage statistics are disabled

- PR number: 2593
- Source review ID: 5596527098
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2593#discussion_r4334306916>
- Concern: the delivery test of `HttpTrackerCoreServices::initialize_from` ran with `Core::default()`,
  whose `tracker_usage_statistics` is `true`, so it could not tell unconditional event publication from
  publication gated by that flag. The comment at the production code says metrics policy must not
  suppress events, so the test should cover both values and require delivery in both cases.
- Solution: the finding is right and exposes a mutant the plan had not hunted. The delivery test is now a
  table over the usage-statistics policy (enabled and disabled), applied to the tracker core and to the
  first HTTP tracker. The three container publication tests that exercise the same contract (the announce
  and scrape services of the container composed from services, and the bus of the container composed from
  tracker core) are tables over it too, each with an enabled and a disabled case. No production code
  changed.
- Current-tree verification: three mutations were applied and reverted. Creating the services bus from the
  core's usage-statistics flag fails the two disabled cases (the delivery test and the bus test) and passes
  the enabled ones, so the old test could not have caught it. Giving the announce or the scrape service a
  sender only when the HTTP tracker's flag is true fails the disabled cases of that service's publication
  test and the bus test. After the revert, the production code of `container.rs` is byte-identical to the
  merge of #2586, and `cargo test -p torrust-tracker-http-core` passes with 144 unit tests, with
  `clippy -D warnings` and nightly `rustfmt` clean.
- Resolution reference: test(http-core): [#1349] cover event publication with usage statistics disabled
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2593#discussion_r4334366085>

## Processing Log

- 2026-10-21 07:50 UTC - Copilot submitted review 5596527098 with one inline thread. Created this audit
  from GraphQL review-thread data: 1 thread, unresolved, marked outdated only after the fix push.
- 2026-10-21 07:58 UTC - Fixed F1 in the commit named above and pushed it. Replied on the thread, then
  resolved it. GraphQL data shows 0 unresolved threads.

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
