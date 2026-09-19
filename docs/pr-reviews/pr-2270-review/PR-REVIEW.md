---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - "issue #2230"
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
| F9 | review-finding:pr-2270-f9 | Human | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2270-f10 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2270-f11 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2270-f12 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F13 | review-finding:pr-2270-f13 | Human | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2270-f14 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2270-f15 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2270-f16 | Human | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F17 | review-finding:pr-2270-f17 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2270-f18 | Human | Suggestion | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2270-f19 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2270-f20 | Human | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2270-f21 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F22 | review-finding:pr-2270-f22 | Human | Minor | metadata | RE_RAISE_OF:F12 | FIXED | RESOLVED |

## Finding Details

### F1 - Correct the `create-issue` workflow link

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522283
- Concern: The first `create-issue` link to `fix-bug` resolved outside the `dev/` skill directory.
- Solution: Replaced the broken relative link with the correct repository-relative skill path.
- Current-tree verification: The current file names `.github/skills/dev/debugging/fix-bug/SKILL.md`, which exists.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053753360

### F2 - Correct the duplicate `create-issue` workflow link

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#pullrequestreview-5250020892
- Concern: The second `create-issue` link repeated the same broken relative target.
- Solution: Replaced the duplicate broken link with the correct repository-relative skill path.
- Current-tree verification: Both `create-issue` references name the existing `fix-bug` skill path.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#issuecomment-5743396140

### F3 - Correct the `write-unit-test` workflow link

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522217
- Concern: The `fix-bug` link to `write-unit-test` resolved outside the expected skill directory.
- Solution: Replaced the broken relative link with the correct repository-relative skill path.
- Current-tree verification: The current path names `.github/skills/dev/testing/write-unit-test/SKILL.md`, which exists.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053753481

### F4 - Use a lifecycle-stable worked-example reference

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522384
- Concern: The long-lived skill referenced issue #2226 through its movable `open/` path.
- Solution: Replaced lifecycle paths with the stable issue #2226 reference.
- Current-tree verification: No issue #2226 `open/` path remains in the skill.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053753573

### F5 - Avoid applying `fix-bug` metadata to every issue template use

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522318
- Concern: The generic issue template unconditionally added `fix-bug` to `skill-links`.
- Solution: Removed the unconditional skill link while retaining bug-conditional sections and the related artifact.
- Current-tree verification: The template's `skill-links` list contains only `create-issue`.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053753732

### F6 - Correct the sample artifact path

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522492
- Concern: The sample's `spec-path` and checkpoint named a nonexistent draft file.
- Solution: Pointed metadata and the checkpoint at the actual issue-local validation artifact.
- Current-tree verification: `spec-path` matches the tracked sample file path.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053753838

### F7 - Use a stable cross-issue relation in issue #2230

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522439
- Concern: Issue #2230 linked issue #2226 through a movable lifecycle path.
- Solution: Replaced the path with the stable issue #2226 relation.
- Current-tree verification: No issue #2226 `open/` path remains in issue #2230 artifacts.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053753934

### F8 - Add reciprocal `fix-bug` markers

- PR number: 2270
- Source review ID: 5250020892
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4048522350
- Concern: Long-lived artifacts listed by `fix-bug` lacked reciprocal skill markers.
- Solution: Added reciprocal `fix-bug` frontmatter markers to the owning long-lived artifacts.
- Current-tree verification: PyYAML parses the reciprocal markers in `write-unit-test`, Implementer, and `create-issue`; each resolves to `fix-bug`.
- Resolution reference: `docs(skills): address bug workflow review`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754034

### F9 - Restore valid `write-unit-test` frontmatter

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605094
- Concern: Misaligned `semantic-links` indentation makes the skill frontmatter invalid YAML.
- Solution: Aligned the semantic-link children and parsed every Agent Skill frontmatter with PyYAML.
- Current-tree verification: All Agent Skill frontmatter blocks parse and expose required `name` and `description` values.
- Resolution reference: `fix(skills): restore unit-test skill frontmatter`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754132

### F10 - Restore the required review audit and replies

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605095
- Concern: Earlier Copilot findings were resolved without replies or the canonical PR audit.
- Solution: Created this canonical audit before fixes and progressively updated it; replies and final thread verification remain pending.
- Current-tree verification: This audit contains F1-F22 with source review IDs, URLs, categories, dispositions, verification, and stable resolution subjects.
- Resolution reference: `docs(pr-reviews): start PR 2270 audit`; `docs(pr-reviews): update PR 2270 audit`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754210

### F11 - Preserve append-only independent review history

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605098
- Concern: The earlier Task Reviewer entry was edited in place and does not cover the current head.
- Solution: Restored the original 15:45 entry exactly and appended failed and passing re-reviews for later heads.
- Current-tree verification: The original evidence text matches its pre-follow-up version; the 11:50 and 11:57 entries remain append-only.
- Resolution reference: `docs(issues): restore original review evidence`; `docs(issues): record final bug workflow evidence`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754308

### F12 - Preserve issue numbers in YAML values

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605102
- Concern: Unquoted `issue #2226` YAML values parse as `issue` because `#` starts a comment.
- Solution: Updated the canonical convention to require quoting and quoted both new `issue #2226` values.
- Current-tree verification: PyYAML preserves `issue #2226` in both touched frontmatter blocks.
- Resolution reference: `fix(docs): preserve issue references in YAML`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754466

### F13 - Require mutation-based red-test proof

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605103
- Concern: `fix-bug` permits skipping red proof more broadly than `write-unit-test` and omits mutate-then-restore.
- Solution: Required mutate-then-restore red proof whenever a maintained regression test is practical.
- Current-tree verification: `fix-bug` limits infeasibility to cases with no practical maintained test and names mutate-then-restore.
- Resolution reference: `fix(skills): require regression red proof`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053756399

### F14 - Preserve bug-workflow ordering in Implementer

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605105
- Concern: Implementer attaches reproduction and boundary selection to the red-test step and omits the final recheck.
- Solution: Required reproduction and boundary selection before red, and final like-for-like evidence before independent review.
- Current-tree verification: Implementer names both the pre-red order and the final artifact recheck; V3 verifies the current behavior.
- Resolution reference: `fix(agents): preserve bug workflow order`; `docs(issues): record final bug workflow evidence`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754687

### F15 - Align issue scope with the semantic bug trigger

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605106
- Concern: The In Scope bullet still limits Implementer to `issue-type: bug` despite broader accepted requirements.
- Solution: Aligned In Scope with AC5 and recorded the preserved pre-implementation maintainer clarification.
- Current-tree verification: Scope and AC5 both require substantive bug detection regardless of metadata or labels.
- Resolution reference: `docs(issues): align bug workflow scope`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754814

### F16 - Restore parseable Implementer frontmatter

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605108
- Concern: An unquoted colon in the existing description prevents parsing the newly added reciprocal marker.
- Solution: Quoted the colon-containing description and normalized semantic-link indentation.
- Current-tree verification: PyYAML parses Implementer and exposes `semantic-links.skill-links = [fix-bug]`.
- Resolution reference: `fix(agents): restore Implementer frontmatter`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053754926

### F17 - Correct the PR description

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605109
- Concern: The PR body says the worked example uses the current `open/` folder, but the tree uses a stable issue reference.
- Solution: Updated the PR body to describe the stable issue #2226 reference.
- Current-tree verification: The PR body and skill both describe issue #2226 without a lifecycle path.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2270
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053755038

### F18 - Keep cross-skill links under lychee validation

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605114
- Concern: Inline-code skill paths are not validated by lychee.
- Solution: Restored three cross-skill references as Markdown links with correct folder-relative targets.
- Current-tree verification: `linter all` passes, including lychee local Markdown links and fragments.
- Resolution reference: `fix(skills): validate cross-skill links`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053755195

### F19 - Consolidate `create-issue` semantic links

- PR number: 2270
- Source review ID: 5255047473
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4052605116
- Concern: `semantic-links` is split between top-level frontmatter and `metadata`.
- Solution: Consolidated skill links and related artifacts into one top-level semantic-links block.
- Current-tree verification: PyYAML exposes one top-level block and no nested `metadata.semantic-links`.
- Resolution reference: `fix(skills): consolidate issue skill links`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053755287

### F20 - Bring the audit forward to the current tree

- PR number: 2270
- Source review ID: 5255823584
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053264314
- Concern: The pre-action audit was not updated after the fixes, so current-tree claims and dispositions were stale.
- Solution: Re-derived F9-F19 against the current tree, recorded fix subjects, and added the latest review round.
- Current-tree verification: Rows and details now match the pushed fixes; only reply URLs and final thread states remain pending.
- Resolution reference: `docs(pr-reviews): update PR 2270 audit`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053755408

### F21 - Disclose review-follow-up scope in the PR body

- PR number: 2270
- Source review ID: 5255823584
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053264319
- Concern: The PR body omitted the semantic-link convention change, audit, and retrospective.
- Solution: Added a Changes bullet naming all three artifacts.
- Current-tree verification: The current PR body discloses the convention correction, canonical audit, and retrospective.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2270
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053755575

### F22 - Quote the audit's issue relation

- PR number: 2270
- Source review ID: 5255823584
- Source URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053264315
- Reviewer finding ID: F12
- Concern: The audit itself retained an unquoted `issue #2230` YAML value after the original F12 fix.
- Solution: Quoted the audit's issue relation in accordance with the corrected convention.
- Current-tree verification: PyYAML preserves the audit related artifact as `issue #2230`.
- Resolution reference: `docs(pr-reviews): update PR 2270 audit`
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2270#discussion_r4053755478

## Processing Log

- 2026-09-19 08:20 UTC - Fetched all GraphQL threads with repository scripts and fetched submitted reviews plus review-specific comments by review ID.
- 2026-09-19 08:25 UTC - Normalized Copilot review 5250020892 as F1-F8 and human review 5255047473 as F9-F19; no re-raises found.
- 2026-09-19 08:30 UTC - Recorded the pre-action current-tree decisions; F9-F19 remain open pending independent fixes.
- 2026-09-19 13:59 UTC - Re-derived F9-F19 against the current tree, normalized review 5255823584 as F20, F21, and F22 (`RE_RAISE_OF:F12`), and recorded stable fix subjects; replies and final thread states remain pending.
- 2026-09-19 16:15 UTC - Posted finding-specific replies for every inline thread, posted consolidated response https://github.com/torrust/torrust-tracker/pull/2270#issuecomment-5743396140 for reviews 5250020892, 5255047473, and 5255823584, resolved all current threads, and refreshed GraphQL state with zero unresolved threads.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite fixes by unique Conventional Commit subjects or durable reply URLs, never branch SHAs.
- Refresh review threads using GraphQL and confirm no unresolved actionable thread remains.
