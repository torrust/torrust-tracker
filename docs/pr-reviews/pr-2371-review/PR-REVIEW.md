---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2371 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2371>.

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

Audit IDs `F1`-`F3` are the reviewer's IDs from Copilot review 5352718785 (round 1, at the head
`docs(issues): add draft spec for mining PR review audit records`). No earlier finding exists, so
no ID collides. Each inline comment carries a `[Major]` bracket; the review overview's
`High`/`Medium` labels are outside the severity vocabulary and are not used. The review body only
lists the three inline findings and adds no other request.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2371-f1` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2371-f2` | Copilot | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2371-f3` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The planned `Prevention` audit field contradicts EPIC #2278's fixed roster decisions

- PR number: 2371
- Source review ID: 5352718785
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2371#discussion_r4133552577>
- Concern: EPIC #2278 fixes "no new per-finding fields", and its order 8 owns pinning the complete
  validator roster. The spec's T3 added a `Prevention` field and depended only on order 7.
- Solution: `docs(issues): move PR review preventability to the analysis follow-up` drops the
  audit field. The analysis follow-up classifies preventability in its own `docs/analysis/`
  artifact keyed by `review-finding:` references, and the export carries that reference. With no
  roster change, the export depends only on order 7 and reads fields through order 8's roster pin
  if that has merged. The author-field alternative stays documented as needing a recorded #2278
  decision and order 8. The Coordination section also records #2278's statement that frontmatter
  validation of audit records belongs to #2264. Tasks were renumbered T1-T4.
- Current-tree verification: `EPIC.md` for #2278 lists "no new per-finding fields" under
  "Decisions the matrix fixes". `grep -n "Prevention"` on the spec matches only the Goal prose and
  the alternative paragraph in "Input for the Analysis Follow-up"; no task, acceptance criterion,
  or manual scenario adds an audit field, and AC5 requires the roster to stay unchanged.
- Resolution reference: `docs(issues): move PR review preventability to the analysis follow-up`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2371#discussion_r4134468314>

### F2 - One JSON object per finding breaks the CLI output contract

- PR number: 2371
- Source review ID: 5352718785
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2371#discussion_r4133552515>
- Concern: The CLI output contract ADR requires exactly one JSON object on successful stdout and
  empty stdout on failure. A stream of one object per finding violates it and can leave partial
  stdout when a later record is malformed.
- Solution: `docs(issues): plan one JSON result object for the PR review export` makes the export
  validate every marked record first, then write one result object with a `findings` array. A
  malformed marked record leaves stdout empty and produces NDJSON diagnostics on stderr with exit
  code `1`. The In Scope bullet, the export task, AC3 (formerly AC4), and scenarios M2 and M4 were
  aligned.
- Current-tree verification: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
  Agreement section 1 states the one-object and empty-on-failure rules.
  `grep -niE "json lines|jsonl|one line per"` on the spec returns nothing.
- Resolution reference: `docs(issues): plan one JSON result object for the PR review export`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2371#discussion_r4134468732>

### F3 - `frontmatter-validator --all` always fails on unrelated legacy specs

- PR number: 2371
- Source review ID: 5352718785
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2371#discussion_r4133552652>
- Concern: The validator README says `--all` exits `1` while legacy draft and open specs remain,
  so the planned check could not verify that historical PR review records still pass.
- Solution: `docs(issues): scope the PR review frontmatter check to docs/pr-reviews` changes the
  automatic check to `frontmatter-validator docs/pr-reviews` and notes why `--all` is unsuitable.
- Current-tree verification: `cargo run -q -p frontmatter-validator --bin frontmatter-validator --
  docs/pr-reviews` exits `0` with no diagnostics.
- Resolution reference: `docs(issues): scope the PR review frontmatter check to docs/pr-reviews`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2371#discussion_r4134469036>

## Processing Log

- 2026-09-29 14:07 UTC - Committed the F3 fix.
- 2026-09-29 14:08 UTC - Committed the F2 fix.
- 2026-09-29 14:10 UTC - Committed the F1 fix.
- 2026-09-29 14:12 UTC - Replied on the F1, F2, and F3 threads; `reply-status` reported 3 of 3
  threads replied, and all three were resolved.
- 2026-09-29 14:46 UTC - Started this audit; a GraphQL refresh shows 3 of 3 threads resolved.

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
