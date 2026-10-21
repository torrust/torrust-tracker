# PR #2613 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2613>.

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
| F1 | review-finding:pr-2613-f1 | Copilot | Nit (inferred) | formatting | ORIGINAL | NO_ACTION | SUPERSEDED |
| F2 | review-finding:pr-2613-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Long checklist lines in the #2606 spec should be hard-wrapped

- PR number: 2613
- Source review ID: 5601186671
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2613#discussion_r4337963540>
- Concern: Copilot asked to hard-wrap the long checklist items at line 201 of the #2606 `ISSUE.md`.
  The review gave the severity as Medium in its prose, not as a bracket.
- Solution: Declined, with no change to the file. The checklist items in `docs/templates/ISSUE.md`
  are unwrapped, so the spec matches the template. The reply names this row as the superseding
  finding.
- Current-tree verification: `grep -n MD013 .markdownlint.json` shows `"MD013": false` on line 3, so
  long lines do not fail markdownlint. `grep -c '^- \[ \]' docs/templates/ISSUE.md` counts 20
  unwrapped checklist items.
- Resolution reference: <https://github.com/torrust/torrust-tracker/pull/2613#discussion_r4338258502>
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2613#discussion_r4338258502>

### F2 - The pronoun "it" in the agent review sentence has no clear actor

- PR number: 2613
- Source review ID: 5601186671
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2613#discussion_r4337963629>
- Concern: The sentence in the #2606 spec used the pronoun "it" without a clear antecedent, so it was
  unclear whether the spec or the reviewer records the agent review result. The review gave the
  severity as Medium in its prose, not as a bracket.
- Solution: Reworded the sentence to name the reviewer as the actor: "When an independent reviewer
  receives this folder-style specification, the reviewer records their result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`." Committed in
  `2040fabd1`.
- Current-tree verification: `grep -n "reviewer records" docs/issues/open/2606-2278-validate-audit-record-merged-fix-subjects/ISSUE.md`
  shows the reworded sentence at line 303, and the subject of `2040fabd1` matches the Resolution
  reference.
- Resolution reference: docs(issues): name the reviewer as the actor in the agent review sentence
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2613#discussion_r4338255427>

## Processing Log

- 2026-10-21 14:18 UTC - Copilot review 5601186671 was submitted on 5b8c344b0 with inline threads
  4337963540 (line 201) and 4337963629 (line 304).
- 2026-10-21 14:40 UTC - Committed 2040fabd1 to fix F2.
- 2026-10-21 14:46 UTC - Pushed 2040fabd1 to josecelano/2606-add-implementation-completion-review-spec.
  The pre-push hooks passed.
- 2026-10-21 14:47 UTC - Replied to and resolved the F2 thread (reply 4338255427) and the F1 thread
  (reply 4338258502).

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
