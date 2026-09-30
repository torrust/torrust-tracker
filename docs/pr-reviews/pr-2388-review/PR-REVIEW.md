---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/drafts/2003-mine-ai-agent-memories/ISSUE.md
---

<!-- skill-link: process-pr-review -->

# PR #2388 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2388>.

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

Audit IDs `F1`-`F4` are assigned in source order from Copilot review 5365459253 (round 1, at the
head `docs(issues): add draft spec for mining AI agent memories`). The inline comments carry no
`[Severity]` bracket, so every severity is inferred from the prose; the review overview's
`Medium severity` badge is outside the severity vocabulary and is not used. The review body only
lists the four inline findings and adds no other request. F4 has the same text as F3 on a
different line and asks for the same change, so it is recorded as a re-raise.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2388-f1` | Copilot | Minor (inferred) | metadata | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | `review-finding:pr-2388-f2` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2388-f3` | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2388-f4` | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |

## Finding Details

### F1 - The `branch:` value does not follow the spec-only branch naming convention

- PR number: 2388
- Source review ID: 5365459253
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4143981571>
- Concern: `branch: "chore/mine-ai-agent-memories-spec"` does not match the
  `{issue-number}-{short-description}-spec` convention. The reviewer suggested
  `2003-mine-ai-agent-memories-spec` or `branch: null` until the issue number exists.
- Solution: No change. The value names the branch that actually carries the draft. The
  `{issue-number}-...-spec` form needs this issue's own number, which does not exist until the
  issue is created after the draft merges. A `2003-` prefix would present the parent EPIC's
  number as the issue number, the ambiguity #2375 is fixing. `null` violates the schema.
  `chore/<short-description>` is the AGENTS.md form for work without a tracked issue, and the
  earlier draft `mine-pr-review-audit-records` (#2371) uses the same pattern. The create-issue
  workflow replaces the value when the issue is created.
- Current-tree verification: `docs/schemas/frontmatter-v1.schema.json` defines Issue `branch` as
  `{"type": "string", "minLength": 1}`. `AGENTS.md` line 332 lists
  `chore/<short-description>`. `docs/issues/drafts/mine-pr-review-audit-records/ISSUE.md` has
  `branch: "chore/pr-review-data-mining-spec"`. `frontmatter-validator` passes the draft folder.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4144108876>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4144108876>

### F2 - The publication risk still reads as an unmet precondition

- PR number: 2388
- Source review ID: 5365459253
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4143981684>
- Concern: The Publishing memories risk said the maintainer "must still approve" publishing the
  snapshots, while the Workflow Checkpoints and Progress Log already record that approval.
- Solution: `docs(issues): state that snapshot publication is approved` rewords the risk to say
  the maintainer approved publishing both snapshots on 2026-09-30, after the privacy review.
- Current-tree verification: `grep -c "must still"` on the spec returns `0`; line 293 now reads
  "The maintainer approved publishing both snapshots on 2026-09-30".
- Resolution reference: `docs(issues): state that snapshot publication is approved`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4144109099>

### F3 - The privacy statement denies local paths while the provenance names a local folder

- PR number: 2388
- Source review ID: 5365459253
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4143981759>
- Concern: The local snapshot's Privacy Review said no local filesystem paths were found, but the
  Provenance names the workspace folder `torrust-tracker-agent-02`.
- Solution: `docs(issues): narrow the local snapshot privacy statement to absolute paths` says
  no absolute local filesystem paths were found, names the folder as the only local path detail,
  and explains why it is kept: repository memory is scoped to that folder, and the name
  identifies no user or machine.
- Current-tree verification: `grep -n "absolute local filesystem paths"` on the local snapshot
  matches the Privacy Review, and the next sentences name `torrust-tracker-agent-02` and the
  reason for keeping it.
- Resolution reference: `docs(issues): narrow the local snapshot privacy statement to absolute paths`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4144109319>

### F4 - The privacy statement denies local paths (re-raise on the Privacy Review line)

- PR number: 2388
- Source review ID: 5365459253
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4143981832>
- Concern: The same text as F3, anchored on the last line of the Privacy Review paragraph instead
  of the Provenance line.
- Solution: No separate change. The F3 fix covers it.
- Current-tree verification: Same as F3.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4144109500>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2388#discussion_r4144109500>

## Processing Log

- 2026-09-30 11:26 UTC - Committed the F2 fix.
- 2026-09-30 11:28 UTC - Committed the F3 fix.
- 2026-09-30 11:35 UTC - Replied on the F1, F2, F3, and F4 threads; `reply-status` reported 4 of
  4 threads replied, and all four were resolved.
- 2026-09-30 11:35 UTC - Started this audit.

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
