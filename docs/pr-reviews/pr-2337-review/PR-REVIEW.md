---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2337 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2337>.

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
| F1 | `review-finding:pr-2337-f1` | Copilot | Blocker | metadata | ORIGINAL | FIXED | RESOLVED |
| FM-001 | `review-finding:pr-2337-fm-001` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Unquoted `issue #<n>` references parse as the YAML string `issue`

- PR number: 2337
- Source review ID: 5309543110
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2337#discussion_r4097902158>
- Concern: `#` preceded by whitespace starts a YAML comment, so the spec's four unquoted
  `issue #<n>` entries in `related-artifacts` deserialize as `issue`, and the planned validator
  would reject the specification itself.
- Solution: quoted the four references and added quoting to the D10 migration checklist. The
  review also exposed the same latent defect in the #2266 accepted fixtures and the
  strict-reference unit test in `profile.rs`. Those tests parse `issue #…` as the path `issue`,
  so they never exercise the issue-reference form. T1 now owns fixing them, plus a regression
  diagnostic for an unquoted `issue #<n>` entry.
- Current-tree verification: a PyYAML parse of the spec frontmatter now yields
  `['issue #2264', 'issue #2266', 'issue #2280', 'issue #2003']`; before the fix it yielded
  `['issue', 'issue', 'issue', 'issue']`. A `git grep` over v1 draft/open specs finds no other
  unquoted `issue #` entry.
- Resolution reference: `docs(issues): [#2281] quote issue references in the validator spec frontmatter`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2337#discussion_r4097974729>

### FM-001 - NDJSON records for non-document events are unspecified

- PR number: 2337
- Source review ID: 5309543110
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2337#discussion_r4097902239>
- Concern: D3 routes help, usage errors, and runtime failures to NDJSON, but D9 defined only the
  document-diagnostic fields. The record shape, required and nullable fields, and categories for
  the other events were therefore left to the implementation.
- Solution: D9 now defines the complete stderr catalog (`diagnostic`, `usage_error`,
  `runtime_error`, `help`), including:
  - field order and always-present fields, with `null` for non-applicable values;
  - `kind` values aligned with the `clippy-allow-reasons` precedent;
  - deterministic record ordering and single-record usage/help runs;
  - JSON examples.

  D3 drops `--version`, which makes it an ordinary usage error. AC2 and M6 now require tests and
  manual checks for every record kind.
- Current-tree verification: inspected D3, D9, AC2, and M6 in
  `docs/issues/open/2281-2264-frontmatter-validator-command/ISSUE.md`; `linter markdown` and
  `linter cspell` exit `0`.
- Resolution reference: `docs(issues): [#2281] define the validator NDJSON record catalog`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2337#discussion_r4097974986>

## Processing Log

- 2026-09-24 20:00 UTC - Copilot review 5309543110 submitted with inline findings F1 and FM-001.
  Its overview also names a minor `FM-002`, but no inline comment or retrievable body exists for
  it (the review has two comments), so no row was recorded.
- 2026-09-24 20:03 UTC - Audit started; F1 and FM-001 normalized with reviewer IDs before the
  fixes.
- 2026-09-24 20:05 UTC - `docs(issues): [#2281] quote issue references in the validator spec frontmatter`
  authored (F1).
- 2026-09-24 20:07 UTC - `docs(issues): [#2281] define the validator NDJSON record catalog`
  authored (FM-001).
- 2026-09-24 20:10 UTC - Replies posted on F1 and FM-001.
- 2026-09-24 20:12 UTC - `validate-audit-record.py --base torrust/develop` exited `0` with one
  row, because its row pattern matches only `F<n>` IDs and skips the reviewer-provided `FM-001`.
  FM-001 was verified by hand: its source comment belongs to review 5309543110, its `[Major]`
  bracket matches, reply 4097974986 is on the same thread, and its resolution subject is on the
  branch.
- 2026-09-24 20:18 UTC - F1 and FM-001 resolved after `reply-status` confirmed both replies; a
  refreshed GraphQL fetch reports zero unresolved threads.

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
