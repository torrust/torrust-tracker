---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2410-1488-si-22-process-queued-events-before-listeners-stop/EPIC.md
---

<!-- skill-link: process-pr-review -->

# PR #2421 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2421>.

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

Copilot review 5395349362 (round 1, "Lite" effort; its full agentic review timed out) left five
inline comments. The review body is an overview only, so it creates no additional finding. Its
badge markup rates F1-F3 `Medium severity` and F4-F5 `Low severity`; as in the PR #2397 audit,
they are recorded as `Minor (inferred)` and `Nit (inferred)`.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2421-f1` | Copilot | Minor (inferred) | link-integrity | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2421-f2` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2421-f3` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2421-f4` | Copilot | Nit (inferred) | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F5 | `review-finding:pr-2421-f5` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - SI-3 draft links to an SI-16 evidence file that may not exist

- PR number: 2421
- Source review ID: 5395349362
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168639962>
- Concern: The superseded SI-3 `verification.md` links SI-16's `manual-verification-evidence.md`;
  the reviewer assumed only `ISSUE.md` exists in the SI-16 folder and suggested linking it instead.
- Solution: no change. The target exists: this PR renamed the SI-16 draft's `verification.md` to
  `docs/issues/open/2412-1488-si-16-migrate-standalone-http-environment/manual-verification-evidence.md`.
- Current-tree verification: `git ls-files docs/issues/open/2412-1488-si-16-migrate-standalone-http-environment/`
  lists `ISSUE.md` and `manual-verification-evidence.md`; `linter lychee` passes.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168818203>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168818203>

### F2 - #2411 workflow checkpoints contradict the issue's state

- PR number: 2421
- Source review ID: 5395349362
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168640029>
- Concern: The #2411 checklist still showed issue creation, #324 linking, and sub-issue linking as
  pending, although the frontmatter and Subissues section show them done.
- Solution: Ticked the completed checkpoints after verifying them on GitHub, with a progress-log
  entry. The same stale "issue created" checkpoint in #2417 and #2418 was fixed in the same commit.
- Current-tree verification: `gh api repos/torrust/torrust-tracker/issues/324` and `.../2417` report
  parent #2411; #2418's issue body references #1488. The three checklists show the items ticked.
- Resolution reference: `docs(issues): mark completed issue creation checkpoints in #2411, #2417, #2418`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168818475>

### F3 - #2417 acceptance criteria out of order

- PR number: 2421
- Source review ID: 5395349362
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168640092>
- Concern: AC4 appeared before AC3 in the #2417 acceptance criteria.
- Solution: Moved AC3 before AC4 without renumbering, so existing references stay valid.
- Current-tree verification: `grep -n -E '^- \[.\] AC[0-9]' docs/issues/open/2417-2411-verify-http-scrape-info-hash-limit/ISSUE.md`
  lists AC1, AC2, AC3, AC4 in order.
- Resolution reference: `docs(issues): order #2417 acceptance criteria numerically`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168818706>

### F4 - Reproducer readiness depends on an exact log message

- PR number: 2421
- Source review ID: 5395349362
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168640143>
- Concern: `reproduce-lost-completions.sh` waits for the log line
  `Tracker shutdown signal handlers installed.`, which is brittle across versions; the reviewer
  suggested relying on the health check alone or making the log check optional.
- Solution: no change. `main` starts all jobs, including the health-check server, before
  `wait_for_shutdown_signal` installs the SIGTERM handler and logs that line. A SIGTERM sent after
  health is ready but before the handler exists takes the default action and bypasses graceful
  shutdown, invalidating the measurement. If the text changes, the script fails with an explicit
  timeout pointing at the log rather than producing a wrong result.
- Current-tree verification: `src/main.rs` calls `app::start_with_explicit_config_toml_path` and
  then `wait_for_shutdown_signal`, whose `select!` logs the line after creating the SIGTERM stream.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168818923>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168818923>

### F5 - #2418 regression test section contradicts itself

- PR number: 2421
- Source review ID: 5395349362
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168640195>
- Concern: The section said "Not applicable" and then listed unit and integration tests.
- Solution: Stated that no bug regression test applies to a performance change and named the
  planned tests that protect behavior: aggregation unit tests (T2) and integration tests for
  persisted totals after a burst and a shutdown flush (T3, T4).
- Current-tree verification: `docs/issues/open/2418-batch-persisted-download-writes/ISSUE.md`,
  section `Regression Test Strategy`, contains the new wording.
- Resolution reference: `docs(issues): state the #2418 test plan instead of not applicable`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2421#discussion_r4168819147>

## Processing Log

- 2026-10-02 18:41 UTC - Fetched review 5395349362 and its five inline threads with the
  `github-review-threads` tool; no human reviews.
- 2026-10-02 18:49 UTC - Committed fixes for F2 (18:47), F3, and F5 separately; pushed.
- 2026-10-02 18:52 UTC - Replied on all five threads; recorded the audit.

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
