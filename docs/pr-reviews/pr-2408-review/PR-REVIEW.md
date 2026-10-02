---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2406"
---

<!-- skill-link: process-pr-review -->

# PR #2408 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2408>.

## Ownership

The PR author owns this tracked audit record. Reviewers, including repository review agents,
deliver findings through GitHub and have no repository-artifact obligation.

- Post-merge workflow approval: N/A

## Findings

Copilot review 5391455102 (round 1) left four inline comments. The review body provides only an
overview, so it creates no additional finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| PR2406-003 | `review-finding:pr-2408-pr2406-003` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| PR2406-004 | `review-finding:pr-2408-pr2406-004` | Copilot | Major | documentation | ORIGINAL | FIXED | RESOLVED |
| PR2406-001 | `review-finding:pr-2408-pr2406-001` | Copilot | Major | metadata | ORIGINAL | FIXED | RESOLVED |
| PR2406-002 | `review-finding:pr-2408-pr2406-002` | Copilot | Major | testing | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### PR2406-003 - Option B overstates its cache-residency benefit

- PR number: 2408
- Source review ID: 5391455102
- Reviewer finding ID: PR2406-003
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4165456972>
- Concern: Option B claimed one database read per torrent per process lifetime even though peerless-torrent cleanup can evict the cached swarm.
- Solution: The option now limits the benefit to cache residency and states that a later scrape can read again after cleanup.
- Current-tree verification: `linter markdown` and `linter cspell` passed after the documentation change; the full pre-commit and pre-push gates also passed.
- Resolution reference: `docs(issues): strengthen #2406 reproduction evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4166398053>

### PR2406-004 - Workflow checkpoint contradicts the spec-only PR

- PR number: 2408
- Source review ID: 5391455102
- Reviewer finding ID: PR2406-004
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4165457013>
- Concern: The checkpoint incorrectly said the specification would ship with the implementation fix.
- Solution: The checkpoint now states that this spec-only PR must merge before implementation starts in a separate follow-up branch.
- Current-tree verification: `linter markdown` and `linter cspell` passed after the documentation change; the full pre-commit and pre-push gates also passed.
- Resolution reference: `docs(issues): strengthen #2406 reproduction evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4166401466>

### PR2406-001 - Evidence does not identify the tested source state

- PR number: 2408
- Source review ID: 5391455102
- Reviewer finding ID: PR2406-001
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4165457054>
- Concern: The original phrase "current workspace" did not provide an auditable source state for the tested binary.
- Solution: The evidence now identifies the debug binary by full commit OID and Conventional Commit subject.
- Current-tree verification: `linter markdown` and `linter cspell` passed after the documentation change; the full pre-commit and pre-push gates also passed.
- Resolution reference: `docs(issues): strengthen #2406 reproduction evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4166403264>

### PR2406-002 - Evidence does not prove the value reached SQLite

- PR number: 2408
- Source review ID: 5391455102
- Reviewer finding ID: PR2406-002
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4165457087>
- Concern: The pre-restart scrape alone could not distinguish a persistence write failure from scrape ignoring a persisted value.
- Solution: The evidence now records the post-shutdown SQLite query that returned the tested info hash with `completed = 1` before restart.
- Current-tree verification: `linter markdown` and `linter cspell` passed after the documentation change; the full pre-commit and pre-push gates also passed.
- Resolution reference: `docs(issues): strengthen #2406 reproduction evidence`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2408#discussion_r4166405722>

## Processing Log

- 2026-10-02 11:56 UTC - Copilot review 5391455102 submitted four inline findings.
- 2026-10-02 13:55 UTC - Re-derived all four fixes against the current branch, replied to each review thread, and started this audit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
