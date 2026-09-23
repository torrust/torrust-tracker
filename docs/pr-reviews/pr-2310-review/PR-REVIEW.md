---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - "issue #2308"
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2310 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2310>.

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

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2310-f1` | Copilot | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2310-f2` | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F3 | `review-finding:pr-2310-f3` | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |
| F4 | `review-finding:pr-2310-f4` | Copilot | Major (inferred) | documentation | ORIGINAL | FIXED | NON_RESOLVABLE |

## Finding Details

### F1 - Use a UTC-minute EPIC timestamp

- PR number: 2310
- Source review ID: 5288263915
- Reviewer finding ID: F1
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2310#discussion_r4080180020>
- Concern: The updated EPIC frontmatter used a date-only `last-updated-utc` value instead of its required quoted UTC-minute value.
- Solution: Restored the `2026-09-23 07:24` UTC-minute value in the EPIC frontmatter.
- Current-tree verification: `rg -n '^last-updated-utc: "2026-09-23 07:24"$' docs/issues/open/2278-2003-strengthen-pr-review-author-self-audit/EPIC.md` returns the frontmatter value; the full pre-commit gate passed.
- Resolution reference: `docs(issues): restore EPIC update timestamp`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2310#discussion_r4080457732>

### F2 - Keep follow-up pull requests in their separate field

- PR number: 2310
- Source review ID: 5288263915
- Reviewer finding ID: F2
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2310#pullrequestreview-5288263915>
- Concern: The proposed contract put a `FOLLOW_UP` pull-request URL in `Resolution reference`, contrary to the existing separate Follow-up PR URL field.
- Solution: The specification now requires a durable reply URL in `Resolution reference` for `FOLLOW_UP` and records any pull request only in Follow-up PR URL.
- Current-tree verification: The F76 scope and AC4 text in the #2308 specification require the separate Follow-up PR URL field; the full pre-commit gate passed.
- Resolution reference: `docs(issues): clarify audit contract boundaries`
- Follow-up PR URL: N/A
- Reply URL: N/A

### F3 - Preserve the post-merge no-action path

- PR number: 2310
- Source review ID: 5288263915
- Reviewer finding ID: F3
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2310#pullrequestreview-5288263915>
- Concern: The proposed F60 wording could erase the existing post-merge `NO_ACTION` disposition when a maintainer declines approved follow-up work.
- Solution: The specification distinguishes in-PR no-action handling from the preserved maintainer-approved post-merge `NO_ACTION` path.
- Current-tree verification: The F60 scope and AC1 text in the #2308 specification preserve the post-merge `NO_ACTION` path; the full pre-commit gate passed.
- Resolution reference: `docs(issues): clarify audit contract boundaries`
- Follow-up PR URL: N/A
- Reply URL: N/A

### F4 - Preserve all consolidated-response conditions

- PR number: 2310
- Source review ID: 5288263915
- Reviewer finding ID: F4
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2310#pullrequestreview-5288263915>
- Concern: The proposed F79 wording retained only the multiple-review-round condition and omitted the identifiers, dispositions, resolution references, and durable URL required by Step 8.
- Solution: The specification now requires each of those conditions whenever a consolidated response covers multiple review rounds.
- Current-tree verification: The F79 scope and AC5 text in the #2308 specification name each required condition; the full pre-commit gate passed.
- Resolution reference: `docs(issues): clarify audit contract boundaries`
- Follow-up PR URL: N/A
- Reply URL: N/A

## Processing Log

- 2026-09-23 08:27 UTC - Started audit; normalized Copilot review 5288263915 findings F1-F4.
- 2026-09-23 08:27 UTC - Fixed F1 in `docs(issues): restore EPIC update timestamp`; full pre-commit gate passed.
- 2026-09-23 08:27 UTC - Fixed F2-F4 in `docs(issues): clarify audit contract boundaries`; full pre-commit gate passed.
- 2026-09-23 08:27 UTC - Replied to F1 with the committed resolution and validation evidence.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated or superseded thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
