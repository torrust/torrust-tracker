---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - "issue #2386"
---

<!-- skill-link: process-pr-review -->

# PR #2405 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/2405>.

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

Copilot review 5389769540 (round 1) left three inline comments with reviewer finding IDs F1 to F3
and `[Minor]` severity brackets, recorded as given; its overview badges rate all three Low. The
audit keeps the reviewer IDs, which do not collide. The review body holds only the overview.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2405-f1` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2405-f2` | Copilot | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2405-f3` | Copilot | Minor | correctness | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The guide dropped the decision's 15-minute threshold for adding capacity

- PR number: 2405
- Source review ID: 5389769540
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2405#discussion_r4164107499>
- Concern: The #2386 Decision adds a second server only when waits over 15 minutes remain common
  after cancellation, but the edited "Add Runner Capacity" text allowed adding capacity for any
  observed wait.
- Solution: The section now says to add capacity only if, after superseded pull-request runs are
  cancelled (#2402), the queue-time recheck still shows waits over 15 minutes as common, and it
  points to the #2386 Decision for the threshold and recheck timing.
- Current-tree verification: at the PR head after the 2026-10-02 11:04 UTC changes,
  `grep -c 'still shows waits over 15 minutes' docs/self-hosted-runner.md` prints `1`.
- Resolution reference: `docs(self-hosted-runner): keep the 15-minute threshold for adding capacity`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2405#discussion_r4165133991>

### F2 - The EPIC row changed without refreshing its last-updated time

- PR number: 2405
- Source review ID: 5389769540
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2405#discussion_r4164107447>
- Concern: EPIC #1840 row 17 changed on 2026-10-02 while `last-updated-utc` still read
  2026-10-01 14:52, contrary to the required EPIC metadata.
- Solution: `last-updated-utc` is now `2026-10-02 10:40`, the time of the edit.
- Current-tree verification: at the PR head after the 2026-10-02 11:04 UTC changes, line 9 of
  `docs/issues/open/1840-improve-pr-workflow-performance-epic/EPIC.md` reads
  `last-updated-utc: "2026-10-02 10:40"`.
- Resolution reference: `docs(issues): [#1840] refresh the EPIC last-updated time`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2405#discussion_r4165134150>

### F3 - The closing-keyword scan missed cross-repository references

- PR number: 2405
- Source review ID: 5389769540
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2405#discussion_r4164107402>
- Concern: The scan allowed at most 20 characters between the keyword and `#`, so it missed
  `Closes torrust/torrust-tracker#2386`, which GitHub accepts.
- Solution: The scan now matches a keyword, an optional colon, whitespace, and then one of the
  reference forms GitHub accepts: `#N`, `owner/repo#N`, or an issue URL. Matching the reference
  forms also stops false positives such as `fixed here (PR #2403`.
- Current-tree verification: at the PR head after the 2026-10-02 11:04 UTC changes, the command
  copied from the skill, run against ten test lines, matches `Closes #2386`, `closes: #2386`, a
  quoted `Closes #2386`, `Closes torrust/torrust-tracker#2386`, a `Fixes` issue URL, and
  `resolved #12`, and does not match `Related to #2386`, `fixed here (PR #2403`, `Closes out
  #2386`, or a quoted commit subject.
- Resolution reference: `docs(skills): match every GitHub closing-reference form in the PR body scan`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2405#discussion_r4165134303>

## Processing Log

- 2026-10-02 08:35 UTC - Copilot review 5389769540 submitted with three inline findings.
- 2026-10-02 11:00 UTC - Committed the F3 fix.
- 2026-10-02 11:03 UTC - Committed the F1 fix.
- 2026-10-02 11:04 UTC - Committed the F2 fix.
- 2026-10-02 11:05 UTC - Re-derived each fix against the PR head and replied on all three
  threads; started this audit.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the original thread outdated after the push. For a
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
