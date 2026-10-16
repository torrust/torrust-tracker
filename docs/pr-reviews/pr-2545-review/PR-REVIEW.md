---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2545 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2545>.

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

Copilot's review 5539177165 (COMMENTED, 2026-10-15 20:03:35 UTC) is an overview with no findings, so it adds no row. da2ce7's review 5539892352 (APPROVED, 2026-10-15 21:02:29 UTC) carries four inline findings, F1-F4, all non-blocking. Its body raises nothing else; its "Checked, no finding" list is not a request. All four are fixed before merge and answered in one consolidated PR conversation response, <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771>, which names both reviews and every finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2545-f1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2545-f2` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2545-f3` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2545-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ADR's consistency rule is broader than the code: `tracker-core` keeps three per-value read accessors

- PR number: 2545
- Source review ID: 5539892352
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288246373>
- Concern: The Agreement said a caller that needs several values reads them in one closure, and rejected one accessor per value because each call takes the lock again. But `tracker-core` keeps `get_torrents_downloads_total()`, `get_torrents_downloads_in_session_total()` and `get_torrents_downloads_persisted_total()`, each one `read_metrics()` call, and the REST `Stats` reads all three with three acquisitions, so the ADR stated a rule its own example breaks.
- Solution: The rule is scoped to values one repository serves together, values from several repositories take one acquisition each, and the three accessors are named as an accepted exception, unchanged from `develop` and exposing no lock or field. The Alternatives bullet is reworded to match. The consolidated response is https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771.
- Current-tree verification: `docs/adrs/20261015181650_read_statistics_repositories_through_a_closure.md` contains the scoped rule, the exception bullet that names the three accessors, and the reworded Alternatives bullet.
- Resolution reference: docs(adrs): [#2536] scope the consistency rule to values one repository serves together
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288301755>

### F2 - Several stamps are later than the commits that carry them

- PR number: 2545
- Source review ID: 5539892352
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288246380>
- Concern: The spec's `last-updated-utc` and the 19:55 Progress Log entry, the evidence file's and the EPIC's stamps, and the 18:20 entry were later than the commits that carry them, and the commit titled for the ADR rewrote five earlier stamps in place. The audit skill treats a later stamp as a defect.
- Solution: The later stamps are corrected to their commits' time (T7 entry 18:18, T8 entry 19:48, evidence 19:48, EPIC entry and stamp 19:48) with a Progress Log entry. The five stamps rewritten in place stay in history because the branch history is kept; their values were already correct. Each stamp is now taken from `date -u` immediately before its commit. The consolidated response is https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771.
- Current-tree verification: In the spec, the T7 entry reads 18:18 UTC and the T8 entry 19:48 UTC; the evidence file reads `last-updated-utc: 2026-10-15 19:48`; the EPIC's entry reads 19:48 UTC. Every stamp is no later than its commit's UTC time.
- Resolution reference: docs(issues): [#2536] correct progress-log stamps that were later than their commits
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288301983>

### F3 - The acceptance boxes were ticked by the implementer, not a reviewer

- PR number: 2545
- Source review ID: 5539892352
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288246386>
- Concern: AC1-AC6 and the standard criteria were ticked by the implementer in the final spec commit, while the template's separate checkpoint "Reviewer validated acceptance criteria and updated checkboxes" stayed open, and no `agent-review-reports.md` records who validated them.
- Solution: The checkpoint is ticked on the basis of the maintainer's review 5539892352, which found AC1-AC6 met, and a Progress Log entry cites it and says no `agent-review-reports.md` exists because the review was posted on GitHub. The consolidated response is https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771.
- Current-tree verification: The spec's "Reviewer validated acceptance criteria and updated checkboxes" checkpoint is `[x]`, and the 21:06 UTC Progress Log entry cites the review URL.
- Resolution reference: docs(issues): [#2536] record the reviewer's validation of the acceptance criteria
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288302291>

### F4 - The PR body says #2414's spec mentions the removed accessors; it does not

- PR number: 2545
- Source review ID: 5539892352
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288246397>
- Concern: The PR body said open specs #2414 and #2518 mention the removed accessors. Only #2518 names them; #2414 names the listener files and `tracker-core/tests/common/test_env.rs`, whose tests call them. The merge tool copies the body into the merge commit.
- Solution: The PR description now says #2518 mentions the removed accessors and #2414 touches their callers (the listener files and the `tracker-core` test environment). The fix is the edit of the description, so the consolidated response https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771 is the resolution reference.
- Current-tree verification: `gh pr view 2545 --json body` reads "Open spec #2518 mentions the removed accessors, and open spec #2414 touches their callers (the listener files and the `tracker-core` test environment)".
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288302698>

## Processing Log

- 2026-10-15 21:09 UTC - Started audit after da2ce7's review 5539892352 on PR #2545. Fixed F1-F3 in three signed commits, corrected the PR description for F4, posted the consolidated response, replied on the four threads, and resolved them. Pushing the fixes dismissed the approval, so the review needs a fresh approval.

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
