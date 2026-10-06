---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2440 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2440>.

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
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`, `documentation`,
  `maintainability`, `security`, `other`
- An outdated thread whose concern was fixed is `FIXED`/`RESOLVED`, even when GitHub marks the
  original thread outdated after the push. For in-PR feedback, use `NO_ACTION`/`SUPERSEDED` only
  for a duplicate, superseded, or no-change concern. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work.

## Findings

Severities follow each inline comment's `[Severity]` bracket. Copilot's overview badge rates F2
`Medium` and its comment says `[Major]`; human review 5417497236 (da2ce7) assessed F1 and F2 as
Minor and blocking, and F3 as Nit and non-blocking. That review endorsed F1-F3 rather than
opening duplicate threads, so its body adds no separate rows; its extra F1 point (the PR body
claim) is handled under F1.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2440-f1` | Copilot | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2440-f2` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2440-f3` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2440-f4` | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2440-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2440-f6` | Human | Minor | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F7 | `review-finding:pr-2440-f7` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Spec claimed every conversion preserved behaviour

- PR number: 2440
- Source review ID: 5417339974
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4186062148>
- Concern: The spec said every entry used a fix that preserves behaviour for all reachable values. A099's out-of-range `Duration` is constructible and now errors instead of truncating, and A123 now panics instead of wrapping. Review 5417497236 added that the PR body's "only observable behaviour change" sentence was wrong too.
- Solution: The Review Outcomes section now separates A129 (behaviour-preserving, framework outcome 1) from A099 and A123 (explicit failure for out-of-range values; for A099, the public function's error contract gains a case under framework outcome 2), and records why neither failure is reachable in production. The PR body's sentence is rewritten to match.
- Current-tree verification: re-read `docs/issues/open/2246-2243-review-domain-numeric-conversions/ISSUE.md` Review Outcomes and the live PR body.
- Resolution reference: `docs(issues): [#2246] distinguish preserved and changed conversion behaviour`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4187246096>

### F2 - Checked peer-count boundary was untested

- PR number: 2440
- Source review ID: 5417339974
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4186062045>
- Concern: The benchmarking tests build only small swarms, so they stayed green under the old wrapping casts; AC3's claim for A123 was therefore false.
- Solution: Extracted `peer_count_as_u32` and added two unit tests: `u32::MAX` converts, and on 64-bit targets `u32::MAX + 1` panics (the conversion cannot fail on 32-bit targets). With the helper mutated to `count as u32`, the panic test failed; the checked conversion was restored by hand. The spec's A123 validation cell and AC3 evidence cite the tests.
- Current-tree verification: `cargo test -p torrust-tracker-torrent-repository-benchmarking` passed (17 unit tests); Clippy with `-D warnings` clean.
- Resolution reference: `test(torrent-repository-benchmarking): [#2246] cover the checked peer-count boundary`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4187246350>

### F3 - Evidence stamp lacked minute precision

- PR number: 2440
- Source review ID: 5417339974
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4186062222>
- Concern: The evidence `last-updated-utc` was date-only, while the template uses `YYYY-MM-DD HH:MM`.
- Solution: Stamped the time the evidence was written; the F4 commit later moved it to the rerun time.
- Current-tree verification: the evidence frontmatter reads `last-updated-utc: "2026-10-05 17:57"`.
- Resolution reference: `docs(issues): [#2246] stamp evidence update time to the minute`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4187246610>

### F4 - M1 config and raw output were not preserved

- PR number: 2440
- Source review ID: 5417497236
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4186184739>
- Concern: The evidence named a `.tmp/` config without embedding it, and condensed the output into a hand-built table instead of quoting it.
- Solution: Embedded `tracker.toml` verbatim and reran M1 with exactly the documented commands, quoting the four seeding responses and the six step-3 lines. The counts match the first run. Added why seeding announces return one peer (the client defaults `peers_wanted` to 1).
- Current-tree verification: `diff` of the embedded `toml` block against `.tmp/m1-2246/tracker.toml` was empty; the quoted output matches the summary table.
- Resolution reference: `docs(issues): [#2246] embed M1 config and verbatim output in evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4187246866>

### F5 - A123 inventory row omitted "reverse"

- PR number: 2440
- Source review ID: 5417497236
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4186184751>
- Concern: "mirroring the production `Coordinator::seeders_and_leechers`" suggests production performs the same narrowing, but it converts `u32 → usize`.
- Solution: The row now says "mirroring the reverse (`u32 → usize`) conversion"; the PR body's table uses the same wording.
- Current-tree verification: `grep -c "mirroring the production"` on the inventory returns 0.
- Resolution reference: `docs(issues): [#2246] name the reverse conversion in the A123 inventory row`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4187247158>

### F6 - PR body cited an audit record that did not exist yet

- PR number: 2440
- Source review ID: 5418840707
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2440#pullrequestreview-5418840707>
- Concern: At the reviewed head, the PR body said review findings are tracked in `docs/pr-reviews/pr-2440-review/PR-REVIEW.md`, but that file did not exist yet. Raised in the review body because the PR body has no diff line.
- Solution: The PR body was updated before the audit was committed, and the audit commit is the next commit after the reviewed head. No body change is needed now that the file exists.
- Current-tree verification: `validate-audit-record.py --pr-number 2440` exits 0 against the committed record.
- Resolution reference: `docs(pr-reviews): [#2246] audit round-1 review findings on #2440`
- Follow-up PR URL: N/A
- Reply URL: N/A

### F7 - Spec lacked a Progress Log entry for the review round

- PR number: 2440
- Source review ID: 5418840707
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4187308488>
- Concern: The round-1 fixes changed Review Outcomes, the A123 validation, and AC3, but the spec's Progress Log had no entry for them and `last-updated-utc` still read 15:04.
- Solution: Added a Progress Log entry for the round-1 review fixes and moved the stamp with it.
- Current-tree verification: the spec frontmatter reads `last-updated-utc: "2026-10-05 18:39"`, matching the new Progress Log entry.
- Resolution reference: `docs(issues): [#2246] log the PR #2440 review-round update in the spec`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2440#discussion_r4187491287>

## Processing Log

- 2026-10-05 16:01 UTC - Copilot review 5417339974 submitted findings F1-F3.
- 2026-10-05 16:14 UTC - Human review 5417497236 (da2ce7, round 1) requested changes: endorsed F1-F3 and added F4 and F5, numbered after Copilot's.
- 2026-10-05 18:13 UTC - Fixed F1-F5 in separate commits, pushed after the pre-push suite passed, updated the PR body (F1, F5), replied to each thread, and started this audit.
- 2026-10-05 18:41 UTC - Human review 5418840707 (da2ce7, round 2, 18:20 UTC, at `docs(issues): [#2246] name the reverse conversion in the A123 inventory row`) confirmed F1-F5 fixed and added F6 (review body) and F7 (inline). F6 was already fixed by the audit commit, pushed after that head. Fixed F7, pushed, and replied on its thread.

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
