---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/closed/2314-preserve-udp-scrape-response-order/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2329 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2329>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Status Values

- Relationship: `ORIGINAL`, `RE_RAISE_OF:<FindingId>`
- Disposition: `FIXED`, `NO_ACTION`, `SUPERSEDED`, `FOLLOW_UP`
- Thread state: `OPEN`, `RESOLVED`, `NON_RESOLVABLE`, `SUPERSEDED`
- Severity: `Blocker`, `Major`, `Minor`, `Nit`, `Suggestion`
- Author class: `Copilot`, `Human`, `Unknown`
- Category: `link-integrity`, `formatting`, `metadata`, `testing`, `correctness`,
  `documentation`, `maintainability`, `security`, `other`

## Findings

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2329-f1` | Copilot | Blocker | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Retrospective frontmatter has an invalid `related-artifacts` list indentation

- PR number: 2329
- Source review ID: 5301097842
- Reviewer finding ID: DOC-1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2329#discussion_r4090956059>
- Concern: the newly archived issue link was indented six spaces, unlike the other four-space list
  entries, so the retrospective frontmatter was not valid YAML.
- Solution: aligned the archived issue link with the other `related-artifacts` sequence entries.
- Current-tree verification: `ruby -e 'require "yaml"; YAML.load_file("docs/pr-reviews/pr-2320-review/review-retrospective.md")'`
  was unavailable locally; the equivalent installed-PyYAML frontmatter parse exits `0` and the list
  contains eight `related-artifacts` entries. `markdownlint
  docs/pr-reviews/pr-2320-review/review-retrospective.md` also exits `0`.
- Resolution reference: `fix(docs): align archived issue link in retrospective frontmatter`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2329#discussion_r4091505171>

## Processing Log

- 2026-09-24 07:20 UTC - Copilot review 5301097842 submitted (F1).
- 2026-09-24 07:20 UTC - Audit started and F1 normalized before the fix.
- 2026-09-24 07:20 UTC - `fix(docs): align archived issue link in retrospective frontmatter`
  authored (F1).
- 2026-09-24 08:29 UTC - Reply posted on F1.
- 2026-09-24 08:30 UTC - F1 resolved after audit validation.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
