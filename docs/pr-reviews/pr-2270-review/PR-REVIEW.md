---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - issue #2230
---

<!-- skill-link: process-pr-review -->

# PR #2270 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2270>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `RESOLVED`, `UNRESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2270-f1 | Copilot | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2270-f2 | Copilot | Major | link-integrity | ORIGINAL | FIXED | NON_RESOLVABLE |
| F3 | review-finding:pr-2270-f3 | Copilot | Major | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2270-f4 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2270-f5 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2270-f6 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2270-f7 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2270-f8 | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2270-f9 | Human | Major | metadata | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F10 | review-finding:pr-2270-f10 | Human | Major | documentation | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F11 | review-finding:pr-2270-f11 | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F12 | review-finding:pr-2270-f12 | Human | Minor | metadata | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F13 | review-finding:pr-2270-f13 | Human | Minor | testing | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F14 | review-finding:pr-2270-f14 | Human | Minor | correctness | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F15 | review-finding:pr-2270-f15 | Human | Minor | documentation | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F16 | review-finding:pr-2270-f16 | Human | Minor | metadata | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F17 | review-finding:pr-2270-f17 | Human | Nit | documentation | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F18 | review-finding:pr-2270-f18 | Human | Suggestion | link-integrity | ORIGINAL | FOLLOW_UP | UNRESOLVED |
| F19 | review-finding:pr-2270-f19 | Human | Nit | metadata | ORIGINAL | FOLLOW_UP | UNRESOLVED |

## Finding Details

### F1 - Correct the `create-issue` workflow link

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522283
- Concern: The first `create-issue` link to `fix-bug` resolved outside the `dev/` skill directory.
- Solution: Replaced the broken relative link with the correct repository-relative skill path.
- Current-tree verification: The current file names `.github/skills/dev/debugging/fix-bug/SKILL.md`, which exists.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; the thread was resolved before the required reply was posted.

### F2 - Correct the duplicate `create-issue` workflow link

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#pullrequestreview-5250020892
- Concern: The second `create-issue` link repeated the same broken relative target.
- Solution: Replaced the duplicate broken link with the correct repository-relative skill path.
- Current-tree verification: Both `create-issue` references name the existing `fix-bug` skill path.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; this was a review-body-only finding.

### F3 - Correct the `write-unit-test` workflow link

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522217
- Concern: The `fix-bug` link to `write-unit-test` resolved outside the expected skill directory.
- Solution: Replaced the broken relative link with the correct repository-relative skill path.
- Current-tree verification: The current path names `.github/skills/dev/testing/write-unit-test/SKILL.md`, which exists.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; the thread was resolved before the required reply was posted.

### F4 - Use a lifecycle-stable worked-example reference

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522384
- Concern: The long-lived skill referenced issue #2226 through its movable `open/` path.
- Solution: Replaced lifecycle paths with the stable issue #2226 reference.
- Current-tree verification: No issue #2226 `open/` path remains in the skill.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; the thread was resolved before the required reply was posted.

### F5 - Avoid applying `fix-bug` metadata to every issue template use

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522318
- Concern: The generic issue template unconditionally added `fix-bug` to `skill-links`.
- Solution: Removed the unconditional skill link while retaining bug-conditional sections and the related artifact.
- Current-tree verification: The template's `skill-links` list contains only `create-issue`.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; the thread was resolved before the required reply was posted.

### F6 - Correct the sample artifact path

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522492
- Concern: The sample's `spec-path` and checkpoint named a nonexistent draft file.
- Solution: Pointed metadata and the checkpoint at the actual issue-local validation artifact.
- Current-tree verification: `spec-path` matches the tracked sample file path.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; the thread was resolved before the required reply was posted.

### F7 - Use a stable cross-issue relation in issue #2230

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522439
- Concern: Issue #2230 linked issue #2226 through a movable lifecycle path.
- Solution: Replaced the path with the stable issue #2226 relation.
- Current-tree verification: No issue #2226 `open/` path remains in issue #2230 artifacts.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; the thread was resolved before the required reply was posted.

### F8 - Add reciprocal `fix-bug` markers

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522350
- Concern: Long-lived artifacts listed by `fix-bug` lacked reciprocal skill markers.
- Solution: Added reciprocal `fix-bug` frontmatter markers to the owning long-lived artifacts.
- Current-tree verification: The markers are present, but F9, F16, and F19 require frontmatter corrections before all are machine-readable.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: N/A; the thread was resolved before the required reply was posted.

### F9 - Restore valid `write-unit-test` frontmatter

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605094
- Concern: Misaligned `semantic-links` indentation makes the skill frontmatter invalid YAML.
- Solution: Pending correction and YAML parse validation.
- Current-tree verification: PyYAML parsing fails at `related-artifacts` in the current tree.
- Resolution reference: Pending.
- Reply URL: Pending.

### F10 - Restore the required review audit and replies

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605095
- Concern: Earlier Copilot findings were resolved without replies or the canonical PR audit.
- Solution: This audit reconstructs all findings; replies and final thread verification remain pending.
- Current-tree verification: The audit was absent before this change, and earlier inline threads contain no replies.
- Resolution reference: Pending audit commit and durable reply URLs.
- Reply URL: Pending.

### F11 - Preserve append-only independent review history

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605098
- Concern: The earlier Task Reviewer entry was edited in place and does not cover the current head.
- Solution: Pending restoration of the original entry and a fresh independent review appended after fixes.
- Current-tree verification: Git history shows the evidence line changed in `docs(skills): address bug workflow review`.
- Resolution reference: Pending.
- Reply URL: Pending.

### F12 - Preserve issue numbers in YAML values

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605102
- Concern: Unquoted `issue #2226` YAML values parse as `issue` because `#` starts a comment.
- Solution: Pending correction in the canonical convention and this PR's frontmatter values.
- Current-tree verification: YAML parsing currently drops `#2226` from both new values.
- Resolution reference: Pending.
- Reply URL: Pending.

### F13 - Require mutation-based red-test proof

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605103
- Concern: `fix-bug` permits skipping red proof more broadly than `write-unit-test` and omits mutate-then-restore.
- Solution: Pending alignment with the stricter existing regression-test rule.
- Current-tree verification: The current skill permits recording why a red check cannot be performed.
- Resolution reference: Pending.
- Reply URL: Pending.

### F14 - Preserve bug-workflow ordering in Implementer

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605105
- Concern: Implementer attaches reproduction and boundary selection to the red-test step and omits the final recheck.
- Solution: Pending explicit ordering and final like-for-like evidence requirement.
- Current-tree verification: `recheck` does not appear in the current agent instructions.
- Resolution reference: Pending.
- Reply URL: Pending.

### F15 - Align issue scope with the semantic bug trigger

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605106
- Concern: The In Scope bullet still limits Implementer to `issue-type: bug` despite broader accepted requirements.
- Solution: Pending scope correction and a progress-log entry for the pre-implementation refinement.
- Current-tree verification: The In Scope bullet and AC5 currently describe different triggers.
- Resolution reference: Pending.
- Reply URL: Pending.

### F16 - Restore parseable Implementer frontmatter

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605108
- Concern: An unquoted colon in the existing description prevents parsing the newly added reciprocal marker.
- Solution: Pending description quoting, standard indentation, and YAML parse validation.
- Current-tree verification: PyYAML rejects the current frontmatter at the description value.
- Resolution reference: Pending.
- Reply URL: Pending.

### F17 - Correct the PR description

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605109
- Concern: The PR body says the worked example uses the current `open/` folder, but the tree uses a stable issue reference.
- Solution: Pending GitHub PR body update.
- Current-tree verification: The skill names issue #2226 without an `open/` path.
- Resolution reference: Pending durable PR URL.
- Reply URL: Pending.

### F18 - Keep cross-skill links under lychee validation

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605114
- Concern: Inline-code skill paths are not validated by lychee.
- Solution: Pending restoration as Markdown links using targets verified from each containing skill folder.
- Current-tree verification: The current paths are inline code and therefore outside lychee link checks.
- Resolution reference: Pending.
- Reply URL: Pending.

### F19 - Consolidate `create-issue` semantic links

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605116
- Concern: `semantic-links` is split between top-level frontmatter and `metadata`.
- Solution: Pending consolidation into one top-level canonical block.
- Current-tree verification: The current frontmatter contains both locations.
- Resolution reference: Pending.
- Reply URL: Pending.

## Processing Log

- 2026-09-19 08:20 UTC - Fetched all GraphQL threads with repository scripts and fetched submitted reviews plus review-specific comments by review ID.
- 2026-09-19 08:25 UTC - Normalized Copilot review 5250020892 as F1-F8 and human review 5255047473 as F9-F19; no re-raises found.
- 2026-09-19 08:30 UTC - Recorded the pre-action current-tree decisions; F9-F19 remain open pending independent fixes.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite fixes by unique Conventional Commit subjects or durable reply URLs, never branch SHAs.
- Refresh review threads using GraphQL and confirm no unresolved actionable thread remains.
