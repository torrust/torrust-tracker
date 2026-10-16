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

Copilot's review 5539177165 (COMMENTED, 2026-10-15 20:03:35 UTC) is an overview with no findings, so it adds no row. da2ce7's review 5539892352 (APPROVED, 2026-10-15 21:02:29 UTC) carries four inline findings, F1-F4, all non-blocking. Its body raises nothing else; its "Checked, no finding" list is not a request. All four are fixed before merge and answered in one consolidated PR conversation response, <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771>, which names both reviews and every finding. da2ce7's second review 5540236537 (APPROVED, 2026-10-15 21:35:58 UTC, at the pushed head) carries four more inline `Nit` findings. Its own IDs F1-F4 collide with the rows above, so they are recorded as F5-F8 with the reviewer's ID kept in each entry. Its body raises nothing else. The four later reviews by the PR author are reply wrappers and add no row. All four are fixed and answered in a second consolidated response, <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6210195811>. da2ce7's third review 5544427728 (APPROVED, 2026-10-16 07:34:32 UTC, at the pushed head) carries one more inline `Nit` finding, whose own ID F1 collides with the first row, so it is recorded as F9. Its body raises nothing else. It is fixed and answered in a third consolidated response, <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6211150310>. da2ce7's fourth review 5545068167 (APPROVED, 2026-10-16 08:32:40 UTC, at the pushed head) and its ACK comment carry no finding, so they add no row. Copilot's second review 5545105221 (COMMENTED, 2026-10-16 08:35:45 UTC, Lite effort after an agentic-review timeout) carries one inline finding, F10. Its body only summarizes that finding and a timeout warning, so it adds no other row. F10 is a no-change concern, answered in a fourth consolidated response, <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6213345067>. da2ce7's fifth review 5546973741 (APPROVED, 2026-10-16 11:09:54 UTC, at the pushed head) carries two inline `Nit` findings, whose own IDs F1 and F2 collide with earlier rows, so they are recorded as F11 and F12. Its body raises nothing else. Both are fixed and answered in a fifth consolidated response, <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6214298167>.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2545-f1` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2545-f2` | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2545-f3` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2545-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2545-f5` | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2545-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2545-f7` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2545-f8` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2545-f9` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F10 | `review-finding:pr-2545-f10` | Copilot | Minor (inferred) | other | ORIGINAL | NO_ACTION | SUPERSEDED |
| F11 | `review-finding:pr-2545-f11` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F12 | `review-finding:pr-2545-f12` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |

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
- Solution: The later stamps are corrected to their commits' time (T7 entry 18:18, T8 entry 19:48, evidence 19:48, EPIC log entry 19:48); the `last-updated-utc` of the spec and of the EPIC were set to 21:05, the time of that commit with a Progress Log entry. The five stamps rewritten in place stay in history because the branch history is kept; their values were already correct. Each stamp is now taken from `date -u` immediately before its commit. The consolidated response is https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771.
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
- Solution: The checkpoint is ticked on the basis of da2ce7's review 5539892352, which found AC1-AC6 met, and a Progress Log entry cites it and says no `agent-review-reports.md` exists because the review was posted on GitHub. The consolidated response is https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6202317771.
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

### F5 - Two pointers still name the stamps that the F2 correction changed

- PR number: 2545
- Source review ID: 5540236537
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288526741>
- Concern: The spec's retrospective pointer named "the 2026-10-15 19:55 UTC progress-log entry" and the AC1 evidence row named "progress log, 2026-10-15 18:20 UTC", but the F2 correction had changed those entries to 19:48 and 18:18, so each pointer named an entry that no longer existed.
- Solution: Both pointers now name the corrected entries. The consolidated response is <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6210195811>.
- Current-tree verification: In the spec, the retrospective line reads 19:48 UTC and the AC1 row reads 18:18 UTC; the only 19:55 and 18:20 mentions left are in the 21:05 entry that describes the correction.
- Resolution reference: docs(issues): [#2536] point the spec's pointers at the corrected progress-log stamps
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4291876039>

### F6 - The EPIC's `last-updated-utc` is recorded as 19:48; it reads 21:05

- PR number: 2545
- Source review ID: 5540236537
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288526749>
- Concern: The spec's 21:05 entry and my round-1 reply, consolidated response and audit F2 Solution said the EPIC's entry and `last-updated-utc` went from 19:55 to 19:48. The EPIC's `last-updated-utc` reads 21:05, set by that commit; only its log entry reads 19:48.
- Solution: The 21:05 spec entry and the F2 Solution above now say the EPIC's log entry went to 19:48 and that the `last-updated-utc` of the spec and the EPIC were set to 21:05. The earlier GitHub reply and consolidated response cannot be edited reliably, so the second consolidated response <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6210195811> corrects that claim.
- Current-tree verification: `EPIC.md` line 9 reads `last-updated-utc: "2026-10-15 21:05"`; the spec's 21:05 entry and the F2 Solution state the same.
- Resolution reference: docs(issues): [#2536] state the last-updated stamps the F2 correction actually set
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4291876194>

### F7 - The entry calls review 5539892352 "the maintainer's review"; it is da2ce7's

- PR number: 2545
- Source review ID: 5540236537
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288526756>
- Concern: In the spec, "the maintainer" chose the design, approved the spec and reviewed T3-T7, so calling da2ce7's review "the maintainer's review" in the 21:06 entry and in the audit's F3 Solution read as if the same person validated the acceptance criteria.
- Solution: Both places now say "da2ce7's review". The consolidated response is <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6210195811>.
- Current-tree verification: The 21:06 spec entry reads "da2ce7's review"; `grep "maintainer's review"` on the spec prints nothing; the F3 Solution above reads "da2ce7's review 5539892352".
- Resolution reference: docs(issues): [#2536] name da2ce7's review where the spec called it the maintainer's
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4291876391>

### F8 - The Ownership section omits the template's post-merge approval line

- PR number: 2545
- Source review ID: 5540236537
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4288526763>
- Concern: The template copies the Ownership section verbatim, ending with the `- Post-merge workflow approval:` line, and this audit omitted it.
- Solution: The line is added with the value `N/A`, since the review took place before merge. The consolidated response is <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6210195811>.
- Current-tree verification: The Ownership section of this record ends with `- Post-merge workflow approval: N/A`.
- Resolution reference: docs(pr-reviews): [#2536] add the post-merge approval line to the PR #2545 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4291876557>

### F9 - The 2026-10-16 corrections have no Progress Log entry

- PR number: 2545
- Source review ID: 5544427728
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4292016761>
- Concern: three commits on 2026-10-16 changed the spec's two pointers and rewrote its 21:05 and 21:06 entries in place without appending a Progress Log line, so those entries carry text written the next morning under their old stamps. The template asks for one line per meaningful update.
- Solution: a 2026-10-16 entry names the three corrections by commit subject and says the 21:05 and 21:06 entries carry wording written the next morning. The third consolidated response is <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6211150310>.
- Current-tree verification: the spec's Progress Log ends with a 2026-10-16 entry after the 21:06 one, naming the three corrections, and the spec's `last-updated-utc` equals that entry's stamp.
- Resolution reference: docs(issues): [#2536] record the 2026-10-16 spec corrections in the progress log
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4292339883>

### F10 - Same pattern: `format!` allocates while holding the read lock

- PR number: 2545
- Source review ID: 5545105221
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4292535346>
- Concern: in `packages/http-core/src/statistics/event/handler.rs`, `format!` allocates inside the `read_metrics` closure, which holds the read lock. Copilot suggests moving `tracing::debug!` into the closure to avoid the allocation and shorten the lock hold. The review's overview labels the finding Medium severity and its text only says "Consider moving"; the audit scale has no Medium, so the row records `Minor (inferred)`, the nearest value, rather than `Suggestion`, which would set the reviewer's label aside.
- Solution: no change. Moving `tracing::debug!` into the closure would take the read lock on every event even when `debug` is disabled, which the spec's Risks section ("Eager debug logging") and AC4 forbid and the ADR's "Debug logging stays lazy" rule rules out. As written, the `read_metrics(...)` call and its `format!` are inside the macro's arguments, so neither runs unless `debug` is enabled. The reply uses the prescribed `Superseded by F10:` form, naming the finding itself, as for an original no-change finding in the PR #2334 audit. The fourth consolidated response is <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6213345067>.
- Current-tree verification: the handler's `tracing::debug!` arguments contain `stats_repository.read_metrics(|metrics| format!("{metrics:?}")).await`; the runtime check in `manual-verification-evidence.md` (V1) logged 0 `stats: Metrics` lines at `info` and 9 at `debug`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4293529526>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4293529526>

### F11 - The row records `Suggestion (inferred)`; the reviewer's own label is Medium

- PR number: 2545
- Source review ID: 5546973741
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4294017462>
- Concern: The F10 row recorded `Suggestion (inferred)`, but the review's overview labels the finding Medium and the round summary lists it as "F10 (Medium, ...)". No rule maps the bot's labels onto the five severity values, so the row disagreed with both with no stated reason.
- Solution: The F10 row now records `Minor (inferred)`, and the F10 entry says the scale has no Medium and `Minor` is the nearest value. The fifth consolidated response is <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6214298167>.
- Current-tree verification: The F10 row in this record reads `Minor (inferred)`, and the F10 entry's Concern states the mapping.
- Resolution reference: docs(pr-reviews): [#2536] map the PR #2545 F10 severity from the reviewer's Medium label
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4294067924>

### F12 - The intro omits da2ce7's fourth review, so "Round 4" reads as the reviewer's fourth round

- PR number: 2545
- Source review ID: 5546973741
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4294017474>
- Concern: The intro named every review on the PR except da2ce7's review 5545068167 (APPROVED at the pushed head, no findings), and the log's "Round 4" labels Copilot's second review while Rounds 2 and 3 were da2ce7's second and third, so the review history was incomplete.
- Solution: The intro now says review 5545068167 and its ACK comment carry no finding and add no row. The Processing Log is append-only, so "Round 4" stays; the Round 5 entry below says the round numbers count processing rounds, not one reviewer's reviews. The fifth consolidated response is <https://github.com/torrust/torrust-tracker/pull/2545#issuecomment-6214298167>.
- Current-tree verification: The intro names review 5545068167 and says it adds no row; the Round 5 log entry explains the numbering.
- Resolution reference: docs(pr-reviews): [#2536] record da2ce7's no-finding approval in the PR #2545 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2545#discussion_r4294068140>

## Processing Log

- 2026-10-15 21:09 UTC - Started audit after da2ce7's review 5539892352 on PR #2545. Fixed F1-F3 in three signed commits, corrected the PR description for F4, posted the consolidated response, replied on the four threads, and resolved them. Pushing the fixes dismissed the approval, so the review needs a fresh approval.
- 2026-10-16 07:15 UTC - Round 2: da2ce7's review 5540236537 (APPROVED) raised four Nit findings, recorded as F5-F8. Fixed each in its own signed commit, posted the second consolidated response, replied on the four threads, corrected the round-1 F2 and F3 Solution texts in this record, and resolved the threads. Round-1 rows are otherwise unchanged.
- 2026-10-16 08:15 UTC - Round 3: da2ce7's review 5544427728 (APPROVED) raised one Nit finding, recorded as F9. Fixed it in a signed commit, posted the third consolidated response, replied on the thread, and resolved it. Earlier rows are unchanged.
- 2026-10-16 10:20 UTC - Round 4: Copilot's review 5545105221 (COMMENTED) raised one inline finding, recorded as F10. Re-derived the claim against the current tree, posted the fourth consolidated response, replied on the thread with the prescribed `Superseded by F10:` form, and resolved it. No code or spec change; earlier rows are unchanged.
- 2026-10-16 11:15 UTC - Round 5: da2ce7's review 5546973741 (APPROVED) raised two Nit findings, recorded as F11 and F12. Fixed each in its own signed commit, posted the fifth consolidated response, replied on both threads, and resolved them. Round numbers in this log count processing rounds, not one reviewer's reviews: rounds 2, 3 and 5 processed da2ce7's second, third and fifth reviews, and round 4 processed Copilot's second review; da2ce7's fourth review 5545068167 had no finding. Earlier rows are unchanged except F10's severity, which F11 corrected.

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
