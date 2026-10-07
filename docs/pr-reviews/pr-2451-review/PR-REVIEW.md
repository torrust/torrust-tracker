---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2448-1488-si-17-migrate-standalone-udp-environment/ISSUE.md
    - docs/issues/open/2449-1488-si-23-migrate-rest-api-test-environment/ISSUE.md
    - docs/issues/open/2450-1488-si-24-migrate-health-check-api-test-environment/ISSUE.md
    - docs/issues/open/1488-overhaul-tracker-shutdown/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2451 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2451>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`; append `(inferred)` when derived
  from free prose.
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

Copilot review 5428029967 ("Lite" effort, 12:05 UTC; its full agentic review timed out) left two
inline comments without reviewer IDs or severity brackets; its badge markup rates both
`Low severity`, recorded as `Minor (inferred)`. They take the next free audit IDs, F5 and F6,
because da2ce7's review numbered its findings F1-F4. The review body is an overview only.

da2ce7 review 5428068587 (round 1, CHANGES_REQUESTED at `29e1ea072`, 12:08 UTC) left four inline
findings, F1-F4, kept as given. Its body repeats them and adds no other actionable assertion; the
PR-title note is explicitly not raised. F3 asks for the same change as Copilot's earlier F6, so it
is recorded as a re-raise.

da2ce7 review 5428606322 (round 2, APPROVED at `2dab46a2f`, 12:51 UTC) left one inline finding,
F7, kept as given. Its body confirms the round-1 fixes and this record, and notes the owed log
entry for the 12:41 push, recorded below; it adds no other actionable assertion.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F5 | `review-finding:pr-2451-f5` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2451-f6` | Copilot | Minor (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F1 | `review-finding:pr-2451-f1` | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2451-f2` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2451-f3` | Human | Nit | formatting | RE_RAISE_OF:F6 | NO_ACTION | SUPERSEDED |
| F4 | `review-finding:pr-2451-f4` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F7 | `review-finding:pr-2451-f7` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F5 - "registar" reads like a typo in SI-24 prose

- PR number: 2451
- Source review ID: 5428029967
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195003999>
- Concern: SI-24 fact 4 wrote "registar" as plain prose, where it reads as a misspelling of
  "registrar".
- Solution: `registar` is the code's field and parameter name (`Registar<RuntimeServiceMetadata>`),
  so it stays, formatted as code, with a one-time note that it is the service registry spelled as
  in the code. The Design and Ownership Review line also formats it as code.
- Current-tree verification: SI-24 lines 66 and 120 at the PR head show `` `registar` ``.
- Resolution reference: `docs(issues): [#1488] format the registar field as code in SI-24`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195266902>

### F6 - SI-24 D1 has an unclosed parenthesis

- PR number: 2451
- Source review ID: 5428029967
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195004063>
- Concern: the parenthetical "(join error, server error, or drain `TimedOut`." never closed.
- Solution: close it after `TimedOut`.
- Current-tree verification: SI-24 line 98 at the PR head reads "`TimedOut`). Approved".
- Resolution reference: `docs(issues): [#1488] close the parenthesis in SI-24 D1`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195267164>

### F1 - EPIC finding 8 still says only the legacy UDP stop path observes the OS signal

- PR number: 2451
- Source review ID: 5428068587
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195034908>
- Concern: the PR widened EPIC finding 2 to the REST API and health-check API test environments,
  but finding 8 still said only the legacy UDP stop path observes the OS signal; both test
  environments start through the legacy axum path, which selects on `global_shutdown_signal()`.
- Solution: widen finding 8 the same way: the legacy stop paths of the UDP, REST API, and
  health-check API test environments observe the signal until SI-17, SI-23, and SI-24 migrate
  them; SI-19 removes the legacy API.
- Current-tree verification: EPIC lines 91-94 at the PR head.
- Resolution reference: `docs(issues): [#1488] widen EPIC finding 8 to the REST API and health-check test environments`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195267426>

### F2 - SI-23 and SI-24 lack four `create-issue` items that SI-17 carries

- PR number: 2451
- Source review ID: 5428068587
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195034918>
- Concern: the two new specs had no toolchain rule, no step creating the evidence file from the
  template, no T0 commit point, and no retrospective-or-log-entry rule.
- Solution: copy SI-17's lines into both specs.
- Current-tree verification: at the PR head, each file matches all four lines (`grep -cE` returns
  4 for each).
- Resolution reference: `docs(issues): [#1488] add missing verification and completion-review items to SI-23 and SI-24`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195267686>

### F3 - SI-24 D1 has an unclosed parenthesis (re-raise)

- PR number: 2451
- Source review ID: 5428068587
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195034923>
- Concern: same as F6, raised three minutes after Copilot's comment.
- Solution: none separate; F6's fix closes the parenthesis.
- Current-tree verification: as F6.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195267991>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195267991>

### F4 - SI-24 M1's "no `Started` usage" collides with the environment's own `Started` alias

- PR number: 2451
- Source review ID: 5428068587
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195034929>
- Concern: the environment declares `pub type Started = Environment<Running>;`, which the spec
  keeps, so a literal M1 check would fail or force an out-of-scope removal.
- Solution: name the legacy channel types, `signals::Halted` and `signals::Started`.
- Current-tree verification: SI-24 line 226 at the PR head.
- Resolution reference: `docs(issues): [#1488] name the legacy signals::Started channel in SI-24 M1`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195268282>

### F7 - Current-tree verification cites a commit id the rebase removed

- PR number: 2451
- Source review ID: 5428606322
- Reviewer finding ID: F7
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4195469125>
- Concern: five current-tree verification lines named the round-1 tip `776208a7d`, which the
  rebase in the record's own push removed; branch ids change after a rebase.
- Solution: say "at the PR head" instead. The 12:31 Processing Log entry keeps the id as history.
- Current-tree verification: the id appears once at the PR head, in the 12:31 log entry.
- Resolution reference: `docs(pr-reviews): cite the PR head instead of a rebased-away id in the #2451 record`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2451#discussion_r4196577877>

## Processing Log

- 2026-10-06 12:31 UTC - Fetched Copilot review 5428029967 and da2ce7 review 5428068587 (six
  inline threads); committed five fixes separately (12:22-12:28); rebased onto `develop`
  (28 commits behind); `linter all` passed; force-pushed `776208a7d` (12:31:14).
- 2026-10-06 12:33 UTC - Reply guard found no newer review; replied on all six threads
  (12:33:04-12:33:12).
- 2026-10-06 12:34 UTC - Recorded this audit and ran the validator (0 failures); the threads are
  resolved after it is pushed.
- 2026-10-06 14:32 UTC - Late entry, owed per da2ce7 round 2: after the 12:34 entry, rebased onto
  `develop` again (11 commits behind), force-pushed the record (12:41:10), found no new review in
  the reply guard, and resolved all six threads.
- 2026-10-06 14:32 UTC - Fetched da2ce7 review 5428606322 (round 2, APPROVED; F7); committed the
  F7 fix (14:23:59); rebased onto `develop` (42 commits behind); force-pushed (14:31:08); replied
  on the F7 thread (14:32:21).

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
