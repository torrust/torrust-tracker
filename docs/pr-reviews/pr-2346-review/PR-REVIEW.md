---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - docs/issues/open/2345-keep-request-kind-in-udp-error-response-event/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2346 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2346>.

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
| F1 | `review-finding:pr-2346-f1` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2346-f2` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2346-f3` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2346-f4` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2346-f5` | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2346-f6` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2346-f7` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | `review-finding:pr-2346-f8` | Human | Suggestion | testing | ORIGINAL | FIXED | RESOLVED |
| F9 | `review-finding:pr-2346-f9` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F10 | `review-finding:pr-2346-f10` | Copilot | Nit (inferred) | documentation | ORIGINAL | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - V1 says the contract sends eleven announces; it sends twelve

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360923>
- Concern: the V1 contract sends eleven announces answered with error responses, then a twelfth
  that is banned and gets no response, so "sends eleven announce requests" and "the tracker
  answers each with an error response" were inaccurate.
- Solution: V1 and the ISSUE.md reproduction summary now describe eleven error-answered announces
  followed by a twelfth banned before handling.
- Current-tree verification: `packages/udp-server/tests/server/contract.rs` loops `for x in 0..=10`
  and then sends the banned twelfth request; both documents match that wording.
- Resolution reference: `docs(issues): [#2345] correct the V1 announce count in the reproduction`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111453785>

### F2 - Use the fix-bug outcome labels for V1 and V2

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360928>
- Concern: the fix-bug v1.1 text added in this PR requires exactly one of Reproduced, Trigger
  only, or Infeasible, but V1 and V2 used free-form wording although the skill cites this evidence
  as its worked example.
- Solution: V1 is labelled **Trigger only** and V2 **Reproduced**, in the evidence statuses and
  the ISSUE.md reproduction summary.
- Current-tree verification: inspected both statuses in `manual-verification-evidence.md` and the
  two reproduction bullets in `ISSUE.md`.
- Resolution reference: `docs(issues): [#2345] label V1 and V2 with the fix-bug reproduction outcomes`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111453850>

### F3 - M1/M2 used a compound status outside the listed values

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360930>
- Concern: `TODO (pre-fix run DONE)` is not a listed status, and each row mixed a finished pre-fix
  run with a planned post-fix recheck.
- Solution: split into M1 and M2 (pre-fix, `DONE`) and M3 and M4 (post-fix rechecks, `TODO`), and
  pointed AC4 at M3 and M4.
- Current-tree verification: inspected the Manual Verification Scenarios table; every row carries
  one of `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.
- Resolution reference: `docs(issues): [#2345] split manual scenarios into pre-fix and post-fix rows`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111453924>

### F4 - `status: open` is not a v1 IssueStatus value

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360931>
- Concern: the spec declares `schema-version: 1`, whose status values are `draft`, `planned`,
  `in-progress`, `blocked`, `in-review`, and `done`.
- Solution: changed the frontmatter to `status: planned`. The same value in other open specs is a
  repository-wide gap left to #2281.
- Current-tree verification: `docs/schemas/frontmatter-v1.schema.json` lists `planned` as a
  status constant; the spec frontmatter declares it.
- Resolution reference: `docs(issues): [#2345] use the v1 planned status in the spec frontmatter`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111453978>

### F5 - The #2345 worked example is not linked back and "that issue" is ambiguous

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360933>
- Concern: the scope sentence's nearest antecedent became #2345 instead of #2226, and the skill's
  `related-artifacts` and Skill Links named only #2226, so changes to #2345's evidence would not
  prompt a re-check.
- Solution: the scope sentence names both issues, and `issue #2345` is listed in
  `related-artifacts` and Skill Links with a re-check note.
- Current-tree verification: inspected the frontmatter, Worked Example, and Skill Links sections of
  `.github/skills/dev/debugging/fix-bug/SKILL.md`.
- Resolution reference: `docs(skills): link the fix-bug internal-outcome worked example`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454029>

### F6 - The template's agent-review-reports lines were dropped

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360935>
- Concern: the spec omitted the `agent-review-reports.md` checkpoint and Implementation Completion
  Review bullet that `docs/templates/ISSUE.md` and sibling specs keep.
- Solution: restored both lines.
- Current-tree verification: the checkpoint list and Implementation Completion Review section now
  reference `agent-review-reports.md`; `docs/templates/AGENT-REVIEW-REPORTS.md` exists.
- Resolution reference: `docs(issues): [#2345] restore the agent review report checkpoint`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454073>

### F7 - Self-check item 5 cannot be ticked for planned post-fix rows

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360938>
- Concern: at review time every bug spec has planned post-fix rows, so "every verification row
  describes what was actually run" cannot be ticked literally.
- Solution: scoped the item to rows marked `DONE`.
- Current-tree verification: inspected the Pre-Review Self-Check list in
  `.github/skills/dev/debugging/fix-bug/SKILL.md`.
- Resolution reference: `docs(skills): scope the fix-bug self-check to completed verification rows`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454121>

### F8 - Bound the event loop with one deadline

- PR number: 2346
- Source review ID: 5325901139
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111360939>
- Concern: `EVENT_PUBLICATION_TIMEOUT` is a per-receive `Duration`, so a receive-until-
  `UdpResponseSent` loop restarts it on every iteration, while `write-unit-test` asks for one
  absolute deadline.
- Solution: the Design and Ownership Review requires one `tokio::time::Instant` deadline computed
  before the loop with `tokio::time::timeout_at`, and the Regression Test Strategy points to it.
- Current-tree verification: `packages/udp-server/src/server/processor.rs` declares
  `EVENT_PUBLICATION_TIMEOUT` as a `Duration` applied per call in `receive_event`; the spec now
  states the single-deadline rule.
- Resolution reference: `docs(issues): [#2345] bound the regression event loop with one deadline`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454201>

### F9 - Unquoted `last-updated-utc` in the evidence file

- PR number: 2346
- Source review ID: 5325777400
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111245648>
- Concern: the evidence file's `last-updated-utc` is unquoted while `ISSUE.md` quotes it, which
  Copilot says could be typed as a datetime.
- Solution: no change. The evidence template ships the field unquoted, the strict frontmatter
  profile covers only `issue` and `epic` documents, and the value has no seconds.
- Current-tree verification: `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` has
  `last-updated-utc: YYYY-MM-DD HH:MM`; PyYAML parses the evidence value as the string
  `'2026-09-26 10:52'`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454253>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454253>

### F10 - "req kin" reads like a typo

- PR number: 2346
- Source review ID: 5325777400
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111245670>
- Concern: the phrase "req kin" in the spec looks like a typo or abbreviation.
- Solution: no change. It is the verbatim subject of commit `27e2db4b`, quoted in backticks next to
  the commit id; correcting it would break the citation.
- Current-tree verification: `git log -1 --format=%s 27e2db4b` prints
  `refactor: [#1382] include req kin in UDP error response if it's known`.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454308>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2346#discussion_r4111454308>

## Processing Log

- 2026-09-26 11:32 UTC - Copilot review 5325777400 submitted with two unnumbered inline findings.
  Its overview body is a summary with no further actionable assertion.
- 2026-09-26 12:25 UTC - da2ce7 review 5325901139 approved with inline findings F1-F8. Its body
  summarizes them and argues no change for the two Copilot findings; it adds no separate
  actionable assertion, so no body row was recorded. The accompanying ACK conversation comment is
  a verdict only.
- 2026-09-26 12:43 UTC - Audit started. Kept da2ce7's F1-F8 IDs and assigned F9 and F10 to the
  Copilot findings to avoid colliding with them. All ten threads were unresolved and not outdated.
- 2026-09-26 12:44 UTC - `docs(issues): [#2345] correct the V1 announce count in the reproduction`
  authored (F1).
- 2026-09-26 12:45 UTC - `docs(issues): [#2345] label V1 and V2 with the fix-bug reproduction outcomes`
  authored (F2).
- 2026-09-26 12:55 UTC - `docs(issues): [#2345] split manual scenarios into pre-fix and post-fix rows`
  authored (F3).
- 2026-09-26 12:56 UTC - `docs(issues): [#2345] use the v1 planned status in the spec frontmatter`
  authored (F4).
- 2026-09-26 12:57 UTC - `docs(issues): [#2345] restore the agent review report checkpoint`
  authored (F6).
- 2026-09-26 13:00 UTC - `docs(issues): [#2345] bound the regression event loop with one deadline`
  authored (F8).
- 2026-09-26 13:01 UTC - `docs(skills): link the fix-bug internal-outcome worked example` authored
  (F5).
- 2026-09-26 13:02 UTC - `docs(skills): scope the fix-bug self-check to completed verification rows`
  authored (F7).
- 2026-09-26 13:04 UTC - Rebased onto `torrust/develop` (merge of torrust/torrust-tracker#2343) and
  pushed; pre-push checks passed.
- 2026-09-26 13:06 UTC - Replies posted on all ten threads.
- 2026-09-26 13:08 UTC - `validate-audit-record.py --base torrust/develop` exited `0` (10 rows,
  0 failures).
- 2026-09-26 13:10 UTC - `reply-status` confirmed a reply on every thread; all ten threads
  resolved. A refreshed GraphQL fetch reports zero unresolved threads.

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
