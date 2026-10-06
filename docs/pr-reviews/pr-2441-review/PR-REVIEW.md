---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #1669"
---

<!-- skill-link: process-pr-review -->

# PR #2441 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2441>.

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
| F1 | `review-finding:pr-2441-f1` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2441-f2` | Human | Suggestion | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2441-f3` | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2441-f4` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2441-f5` | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | `review-finding:pr-2441-f6` | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Outdated TslConfig and tsl.rs names in the axum-server note

- PR number: 2441
- Source review ID: 5418930104
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4187384973>
- Concern: The EPIC's note on `torrust-tracker-axum-server` said `tsl.rs` imports `TslConfig`, but the current file is `tls.rs` and it imports `v3_0_0::tls::TlsConfig`; the historical #1860 references should keep the original name.
- Solution: The note's first sentence now names `tls.rs` and `v3_0_0::tls::TlsConfig`; the #1860 quick-list and details-table entries keep `TslConfig`, matching that issue's title. Severity is inferred from Copilot's "Low severity" overview badge and "non-blocking" wording.
- Current-tree verification: `packages/axum-server/src/tls.rs:7` imports `torrust_tracker_configuration::v3_0_0::tls::TlsConfig` and line 6 imports `torrust_located_error::{DynError, LocatedError}`; the note in `docs/issues/open/1669-overhaul-packages/EPIC.md` inspected; pre-commit passed.
- Resolution reference: `docs(issues): [#1669] use current TlsConfig name in axum-server note`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4187506400>

### F2 - Newly listed subissues have no Details row

- PR number: 2441
- Source review ID: 5419233306
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4187643908>
- Concern: The subissues newly marked done in the quick list (#1864, SI-23 to SI-27, SI-34, SI-35), and SI-29 already at the base, had no row in the Details table, which is the only part of the EPIC with a Local Spec column.
- Solution: Added a `DONE` row for each, linking its closed `ISSUE.md`, with the related decision in Notes where one exists.
- Current-tree verification: the nine new rows at the end of the Details table in `docs/issues/open/1669-overhaul-packages/EPIC.md` inspected; `linter lychee` and `linter markdown` passed.
- Resolution reference: `docs(issues): [#1669] add Details rows for subissues missing from the EPIC table`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4192952037>

### F3 - Excluded dev-tools declare publish = false

- PR number: 2441
- Source review ID: 5419233306
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4187643926>
- Concern: The inventory said the excluded dev-tools declare `publish = []`, but every manifest declares `publish = false`.
- Solution: The sentence now quotes `publish = false`. The `[]` came from `cargo metadata`, which normalizes `false` to an empty registry list.
- Current-tree verification: `grep -n '^publish' contrib/dev-tools/*/*/Cargo.toml` shows `publish = false` in all six manifests.
- Resolution reference: `docs(issues): [#1669] quote publish = false for excluded dev-tools`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4192952316>

### F4 - REST API paragraph splits the publication observation

- PR number: 2441
- Source review ID: 5419233306
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4187643929>
- Concern: The inserted REST API paragraph separated the publication count from the `torrust-axum-` and #1659 sentences, so they read as a statement about the REST API restructure.
- Solution: Moved those sentences back to the end of the Observation paragraph; the REST API restructure is its own paragraph after it.
- Current-tree verification: the Observation paragraph and the following paragraph in the Package Inventory section inspected.
- Resolution reference: `docs(issues): [#1669] keep the publication observation together`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4192952516>

### F5 - Baseline entry reads as if no spec exists

- PR number: 2441
- Source review ID: 5419233306
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4187643941>
- Concern: The quick-list entry said the baseline was "not yet recorded in a GitHub issue or spec", but a draft spec exists and the Details row links it.
- Solution: The entry now says the draft spec does not yet record the completed artifacts and has no GitHub issue.
- Current-tree verification: the baseline quick-list entry and Details row inspected; `docs/issues/drafts/1669-01-establish-baseline-analysis/ISSUE.md` exists with `github-issue: null`.
- Resolution reference: `docs(issues): [#1669] clarify that the baseline draft spec exists`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4192952702>

### F6 - SI-27 spec last-updated-utc not bumped

- PR number: 2441
- Source review ID: 5419233306
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4187643946>
- Concern: The SI-27 spec was edited for the DEC-17 reference repair, but its `last-updated-utc` still read 2026-06-20.
- Solution: Set `last-updated-utc` to 2026-10-06, keeping the field's date-only format.
- Current-tree verification: frontmatter of `docs/issues/closed/1908-1669-si-27-move-driver-enum-to-primitives/ISSUE.md` inspected; the staged frontmatter check passed.
- Resolution reference: `docs(issues): [#1669] bump SI-27 spec last-updated-utc after DEC-17 repair`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2441#discussion_r4192952920>

## Processing Log

- 2026-10-05 18:28 UTC - Copilot review 5418930104 posted one inline finding, F1, and recommended approval.
- 2026-10-05 18:44 UTC - Fixed F1, pushed after the pre-push suite passed, and replied on its thread before recording it here.
- 2026-10-05 18:58 UTC - Human review 5419233306 (da2ce7, round 1) approved with five optional inline findings, F2 to F6.
- 2026-10-06 08:06 UTC - Rebased onto the latest `develop`, fixed F2 to F6 in one commit each, pushed once after the pre-push suite passed, and replied on each thread before recording them here.

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
