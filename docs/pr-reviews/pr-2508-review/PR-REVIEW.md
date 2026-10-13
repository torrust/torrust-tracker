---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
    - docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md
    - docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md
---

<!-- skill-link: process-pr-review -->

# PR #2508 Review Audit

Source: pull-request reviews and inline review threads for <https://github.com/torrust/torrust-tracker/pull/2508>.

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

Review 5507407394 (round 1, `copilot-pull-request-reviewer`, state `COMMENTED`, submitted
2026-10-13 17:23 UTC on head `fda1851a4`,
<https://github.com/torrust/torrust-tracker/pull/2508#pullrequestreview-5507407394>) left two
inline comments without severity brackets. Its body labels the first "Medium severity" and the
second "Low severity"; the rows record them as `Major (inferred)` and `Minor (inferred)`. The rest
of the body summarizes the pull request and is not a further finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2508-f1` | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2508-f2` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The immediately effective ADR conflicts with the unchanged mandatory-rebase skills

- PR number: 2508
- Source review ID: 5507407394
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4261831838>
- Concern: The ADR's transition note makes the recommendation effective as soon as it merges, but
  the `open-pull-request` skill still required a rebase before opening or updating every pull
  request and the `merge-pull-request` skill still required one immediately before merging.
  `AGENTS.md` rule 8 requires linked skills to stay synchronized. Either update those skills in this
  pull request or defer the immediate-transition language to the implementation pull request.
- Solution: Kept the immediate effect, which the maintainer chose when deciding to accept the ADR in
  the spec-only pull request, and updated the instructions instead. The `open-pull-request` skill
  now makes rebasing recommended (when opening a pull request, when `develop` is known to have
  changed, and whenever it is cheap) and required only on conflicts, needed `develop` changes, or a
  stale merge base reported by the merge tool. The `merge-pull-request` skill no longer rebases in
  advance and refreshes the head, by rebase or "Update branch", only when the tool reports a stale
  merge base. `AGENTS.md` replaces "only rebase onto `develop` to resolve conflicts" with the
  ADR rule and link. The ADR transition note states that this pull request updates those
  instructions, and the #2507 specification marks T3 and T4 in progress.
- Current-tree verification:
  `git show HEAD:<file> | grep -ciE 'always rebase|immediately before starting the interactive merge, rebase|should sit on top of the latest|only rebase onto .develop. to resolve'`
  returns `0` for both skills and `AGENTS.md`; `linter all` passes in the pre-commit gate of the fix
  commit.
- Resolution reference: `docs(skills): [#2507] recommend pull-request rebases instead of requiring them`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4261912749>

### F2 - The ADR frontmatter cites a moving issue-spec path

- PR number: 2508
- Source review ID: 5507407394
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4261831917>
- Concern: The ADR's `semantic-links.related-artifacts` listed
  `docs/issues/open/2507-stop-requiring-pull-request-rebases/ISSUE.md`, which moves to
  `docs/issues/closed/` when #2507 closes. The semantic-link convention requires ADRs to cite issue
  specifications by a quoted stable issue reference.
- Solution: Replaced the path with `"issue #2507"`, as
  `docs/skills/semantic-skill-link-convention.md` ("Cross-Referencing ADRs") prescribes. The ADR body
  keeps its Markdown links to the specification and its evidence; the `cleanup-completed-issues`
  workflow audits and repairs those when the specification is archived.
- Current-tree verification:
  `git show HEAD:docs/adrs/20261013161419_stop_requiring_pull_request_rebases.md | sed -n 7,12p`
  lists `"issue #2507"` and no `docs/issues/open/2507` path; the staged frontmatter validator passes
  in the pre-commit gate of the fix commit.
- Resolution reference: `docs(adrs): [#2507] cite the implementing issue by number in the ADR frontmatter`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4261913215>

## Processing Log

- 2026-10-13 17:23 UTC - Review 5507407394 (`copilot-pull-request-reviewer`, `COMMENTED`, on head
  `fda1851a4`) left inline comments 4261831838 and 4261831917; the thread capture shows both
  threads `resolved=false`.
- 2026-10-13 17:28 UTC - Committed the F2 fix
  `docs(adrs): [#2507] cite the implementing issue by number in the ADR frontmatter`.
- 2026-10-13 17:30 UTC - Committed the F1 fix
  `docs(skills): [#2507] recommend pull-request rebases instead of requiring them`, and pushed both
  fixes.
- 2026-10-13 17:31 UTC - Re-verified both fixes against the pushed head and posted the F1 and F2
  replies on their threads.
- 2026-10-13 17:33 UTC - Recorded F1 and F2 `FIXED`/`RESOLVED` and ran `validate-audit-record.py`.
- 2026-10-13 17:34 UTC - Committed and pushed
  `docs(pr-reviews): [#2507] add the PR #2508 review audit` although its pre-commit gate had failed
  on cspell (unknown word in the F1 solution): the command that parsed the gate's JSON status exited
  `0` and did not stop the commit. Fixed forward by rewording the F1 solution in a separate commit
  instead of rewriting the pushed commit.

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
