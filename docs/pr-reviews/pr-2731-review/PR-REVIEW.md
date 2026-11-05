---
pr-number: 2731
pr-url: https://github.com/torrust/torrust-tracker/pull/2731
last-updated-utc: "2026-11-05 14:01"
---

# PR #2731 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2731.

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

Round 1: Copilot review 5785642686 on the head `820d16fbc` (three inline findings). All three are valid and fixed.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2731-f1 | Copilot | Minor (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2731-f2 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2731-f3 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Align the check limit wording with what is counted and pass a run limit

- PR number: 2731
- Source review ID: 5785642686
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2731#discussion_r4480994038
- Concern: The text says "pushes that only fix failing checks", but the count is the head commits whose checks failed (failed attempts), and `gh run list` returns only the 20 latest runs by default, so the count can be wrong on a pull request with many runs.
- Solution: Valid. The Round Limit section describes the count as failed fixing attempts and the command passes `--limit 1000`. The same wording is fixed in `watch-pull-request-checks`.
- Current-tree verification: `grep` finds "failed fixing attempts" and `--limit 1000` in `.github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md`; `grep -rn "pushes that fixed\|that only fix failing\|fixed failing checks" AGENTS.md .github/skills` printed nothing.
- Resolution reference: docs(agents): count failed check commits with a full run list
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2731#discussion_r4481339629

### F2 - Split the AGENTS.md review loop paragraph

- PR number: 2731
- Source review ID: 5785642686
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2731#discussion_r4480994152
- Concern: The paragraph mixes several concepts in long sentences, and "pushes that fixed failing checks" is ambiguous because a push is known to have fixed checks only afterwards.
- Solution: Valid. The paragraph is split into two paragraphs of short sentences, the three ask-the-contributor cases are listed in one sentence, and the wording is "head commits have failed checks".
- Current-tree verification: Read the "After opening a PR" paragraphs in `AGENTS.md` after the fix; the grep in F1 found no remaining instance of the old phrase.
- Resolution reference: docs(agents): split the review loop paragraph in AGENTS.md
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2731#discussion_r4481341532

### F3 - Split the open-pull-request review loop description

- PR number: 2731
- Source review ID: 5785642686
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2731#discussion_r4480994206
- Concern: The description is one long sentence, and its parenthetical "review rounds and failed-check pushes" is easy to skim past and implies only successful check fixes are counted.
- Solution: Valid. The section is split into three paragraphs and names the limits as more than 5 rounds of review fixes, or more than 5 head commits with failed checks.
- Current-tree verification: Read the "After Opening: Run the Review Loop" section of `.github/skills/dev/git-workflow/open-pull-request/SKILL.md` after the fix.
- Resolution reference: docs(agents): split the review loop description in open-pull-request
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2731#discussion_r4481342053

## Processing Log

- 2026-11-05 14:01 UTC - Started audit; round 1 (Copilot review 5785642686, submitted 13:26 UTC on `820d16fbc`) processed: F1, F2 and F3 fixed in the three commits cited above (committed 13:28 to 13:29 UTC) and pushed; replied and resolved on all three threads at 13:59 UTC (the replies were delayed by an org policy that blocked the bot token); zero unresolved threads after refreshing GraphQL. Copilot is re-requested after the audit is pushed.

## Completion Rules

- Re-derive the reply claim against the current tree before replying or resolving a thread.
- Reply on every resolvable thread before resolving it.
- For an outdated thread whose concern was fixed, record `Disposition=FIXED` and
  `Thread state=RESOLVED`, even if GitHub marks the thread outdated after the push. For a
  duplicate, superseded, or no-change in-PR thread, reply exactly
  `Superseded by <FindingId>: <reason>.`, record `Disposition=NO_ACTION` and
  `Thread state=SUPERSEDED`, then resolve it. A post-merge `NO_ACTION` requires maintainer
  approval to decline the follow-up work. When the current identity cannot resolve a thread
  (`viewerCanResolve` is `false`), still reply, keep the disposition established from the current
  tree, and record `Thread state=OPEN`; the reviewer or PR author resolves it.
- A consolidated PR conversation response may cover multiple review rounds only when it names
  every review ID and every finding ID with its disposition and resolution reference. Record its
  durable URL in each related row.
- Cite a fix by its unique Conventional Commit subject or durable reply URL, never by a branch SHA
  that can change after a rebase.
- Refresh review threads using GraphQL and confirm that no unresolved actionable thread remains.
