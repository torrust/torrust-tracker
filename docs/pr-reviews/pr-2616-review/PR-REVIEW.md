---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/1349-1347-add-tests-http-core/ISSUE.md
    - docs/issues/open/1349-1347-add-tests-http-core/coverage-evidence.md
    - docs/issues/open/1349-1347-add-tests-http-core/manual-verification-evidence.md
---

<!-- skill-link: process-pr-review -->

# PR #2616 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2616>.

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

Copilot's review 5602395027 (COMMENTED, 2026-10-21 15:50:49 UTC) read the pull-request head
"docs(issues): [#1349] record the S6 pull request in the specification". Its body only warns that the
agentic review did not start before the timeout and carries no independently actionable assertion; it
has six inline threads, F1 to F6. No other review had been submitted when this audit started. All six are
valid and were fixed.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2616-f1 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2616-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2616-f3 | Copilot | Nit (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2616-f4 | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2616-f5 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2616-f6 | Copilot | Nit (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The ledger records `lib.rs` as 27 / 27 lines while the evidence says 41 / 41

- PR number: 2616
- Source review ID: 5602395027
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338894863>
- Concern: the ledger row 11 and the S6 evidence give different baselines for `lib.rs`.
- Solution: both are right for different points in time. The ledger column is the S2 baseline (as its
  introduction says), while `lib.rs` grew to 41 lines when S3 added the shared `RecordingSender`. The row now
  states both figures and the cause.
- Current-tree verification: `git log -S RecordingSender -- packages/http-core/src/lib.rs` lists the S3 scrape
  commit as the one that added it; ledger row 11 now reads "27 / 27 (100.00%; 41 / 41 at the S6 start, after S3
  added the shared `RecordingSender`)".
- Resolution reference: docs(issues): [#1349] reconcile the ledger, the uncovered-lines table and the approval times
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338936281>

### F2 - The uncovered-lines row names two lines but counts three

- PR number: 2616
- Source review ID: 5602395027
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338894959>
- Concern: the row for `statistics/event/handler.rs` lists locations "20 and 25" but a count of 3.
- Solution: the count came from the `llvm-cov` JSON summary and the locations from the line-level report, which
  differ by one line for this file (and for `container.rs`). The row shows both counts and a note under the table
  records the difference.
- Current-tree verification: a clean unit-only `cargo llvm-cov ... --lib --text` marks lines 20 and 25 of
  `handler.rs` as uncovered (2 lines) and 20 lines in total, while the JSON summary counts 3 and 22; the row now
  reads "3 (summary) / 2 (text)".
- Resolution reference: docs(issues): [#1349] reconcile the ledger, the uncovered-lines table and the approval times
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338936564>

### F3 - The approval time `13:3x` reads like a placeholder

- PR number: 2616
- Source review ID: 5602395027
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338895038>
- Concern: a non-numeric time in the progress log of `statistics-event-mod-tests.md` (and, by the same pattern, in
  the other two S6 plans).
- Solution: the three plan logs now say "between 13:23 and 13:50 UTC" with "the exact time was not noted", the
  convention the plans use elsewhere.
- Current-tree verification: `grep -rn "13:3x\|14:5x"` over the issue folder returns no match.
- Resolution reference: docs(issues): [#1349] reconcile the ledger, the uncovered-lines table and the approval times
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338936890>

### F4 - The reworded services module comment is grammatically awkward

- PR number: 2616
- Source review ID: 5602395027
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338895082>
- Concern: "specific for the HTTP tracker but it does not depend on the Axum web server" reads badly.
- Solution: the paragraph now reads "These modules contain logic that is specific to the HTTP tracker. They do not
  depend on the Axum web server and could be reused for other web servers." The plan's results record the rewording.
- Current-tree verification: `packages/http-core/src/services/mod.rs` shows the new text; the change is a comment
  only and the package has no `axum` dependency.
- Resolution reference: docs(http-core): [#1349] reword the services module comment
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338937052>

### F5 - A bencode fragment was added to the global project dictionary

- PR number: 2616
- Source review ID: 5602395027
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338895139>
- Concern: `project-words.txt` gained a word that comes from verbatim bencoded output in one evidence file, which
  lowers the dictionary's signal.
- Solution: scope the exception to the evidence file with an inline `cspell:ignore` comment, a form the repository
  already uses, and revert the dictionary.
- Current-tree verification: `git diff torrust/develop -- project-words.txt` is empty and the pre-commit
  spell-check passes with the inline comment in `manual-verification-evidence.md`.
- Resolution reference: docs(issues): [#1349] scope the bencode spelling exceptions to the manual evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338937334>

### F6 - A second bencode fragment was added to the global project dictionary

- PR number: 2616
- Source review ID: 5602395027
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338895191>
- Concern: the same concern as F5 for the second word.
- Solution: the second word is in the same inline `cspell:ignore` comment; the dictionary is reverted.
- Current-tree verification: the same checks as F5.
- Resolution reference: docs(issues): [#1349] scope the bencode spelling exceptions to the manual evidence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2616#discussion_r4338937657>

## Processing Log

- 2026-10-21 15:50 UTC - Copilot submitted review 5602395027 with six inline threads. The poll script reported six
  unresolved threads; the audit was created from GraphQL review-thread data.
- 2026-10-21 15:55 UTC - Verified each claim against the current tree, fixed the six findings in three signed
  commits, pushed, replied on each thread with the fixing commit subject and resolved all six. GraphQL data shows 0
  unresolved threads. A re-review was requested from Copilot.

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
