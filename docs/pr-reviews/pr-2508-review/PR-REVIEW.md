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
2026-10-13 17:23 UTC on the head whose tip commit was
`docs(issues): [#2507] link the specification pull request`,
<https://github.com/torrust/torrust-tracker/pull/2508#pullrequestreview-5507407394>) left two
inline comments without severity brackets. Its body labels the first "Medium severity" and the
second "Low severity"; the rows record them as `Major (inferred)` and `Minor (inferred)`. The rest
of the body summarizes the pull request and is not a further finding.

Reviews 5507501437 and 5507501939 (josecelano, state `COMMENTED`) are the containers GitHub created
for the F1 and F2 thread replies; they carry no finding.

Review 5507795043 (round 2, `copilot-pull-request-reviewer`, state `COMMENTED`, submitted
2026-10-13 17:56 UTC on the head whose tip commit was
`docs(pr-reviews): [#2507] fix a spelling failure in the PR #2508 review audit`, before the
rebase, <https://github.com/torrust/torrust-tracker/pull/2508#pullrequestreview-5507795043>) ran a
"Lite" review after a runner timeout warning and left three inline comments without severity
labels; the rows record inferred severities. Its body lists the round-1 findings as resolved and is
not a further finding.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | `review-finding:pr-2508-f1` | Copilot | Major (inferred) | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | `review-finding:pr-2508-f2` | Copilot | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |
| F3 | `review-finding:pr-2508-f3` | Copilot | Minor (inferred) | testing | ORIGINAL | FIXED | RESOLVED |
| F4 | `review-finding:pr-2508-f4` | Copilot | Nit (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | `review-finding:pr-2508-f5` | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |

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

### F3 - A failed pre-commit gate did not block the audit commit

- PR number: 2508
- Source review ID: 5507795043
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4262164052>
- Concern: The Processing Log records that the pre-commit gate failed on cspell without blocking
  the commit, because the command that parsed its JSON status exited `0`. That weakens the
  repository's enforcement; the comment asks for a tracked follow-up to harden the gating script so
  every failed check produces a non-zero exit and blocks the commit, or at least fails CI.
- Solution: Verified that the gating script is not the weak point: it exits `1` on a failed check,
  so the installed hook blocks the commit, and the same linters run in CI. The slip came from the
  agent's manual invocation in a clone without the hook, which piped the JSON output into a parser
  whose exit status replaced the script's. Instead of a script follow-up, the `commit-changes` skill
  now says to make the commit depend on the script's own exit status and never to pipe its output
  into a command whose exit status replaces it, with an example; this branch's later commits use
  that form.
- Current-tree verification: with a misspelled Markdown file staged,
  `TORRUST_GIT_HOOKS_LOG_DIR=.tmp ./contrib/dev-tools/git/hooks/pre-commit.sh --format=json`
  exited `1` with JSON `status=fail`, `exit_code=1`, and failed step "Running all linters" (probe
  file unstaged and removed afterwards).
  `git show HEAD:.github/skills/dev/git-workflow/commit-changes/SKILL.md | grep -c 'Never pipe its output'`
  returns `1`.
- Resolution reference: `docs(skills): [#2507] gate manual commits on the pre-commit script's exit status`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4262204970>

### F4 - The commit-id citation rule used "will be merged" for review replies

- PR number: 2508
- Source review ID: 5507795043
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4262164124>
- Concern: The `open-pull-request` skill said "nothing that will be merged (issue specs, evidence
  files, PR body, review replies) may cite a PR-branch commit by id", although review replies are not
  merged into the repository. The comment asks to distinguish repository artifacts from pull-request
  conversation artifacts, or to state the rule in terms of durable references.
- Solution: Partly agreed: the maintainer merge tool copies the PR body into the merge commit, so the
  body is merged, but review replies are not. The rule now forbids PR-branch commit ids in any
  durable reference and names them: repository artifacts (issue specs, evidence files, review
  audits), the PR body, and review replies.
- Current-tree verification:
  `git show HEAD:.github/skills/dev/git-workflow/open-pull-request/SKILL.md | grep -c 'nothing that will be'`
  returns `0`; `linter all` passes in the fix commit's pre-commit gate.
- Resolution reference: `docs(skills): [#2507] scope the commit-id citation rule to durable references`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4262205278>

### F5 - The ADR index row used alignment padding

- PR number: 2508
- Source review ID: 5507795043
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4262164172>
- Concern: The new `docs/adrs/index.md` row padded its cells with alignment whitespace, unlike the
  surrounding rows, which creates noisy diffs and is brittle with table formatters.
- Solution: The row now uses single spaces around the cell separators, like the surrounding rows.
  The padding had been added only to match the header widths without re-padding the whole table.
- Current-tree verification: the `20261013161419` row of `git show HEAD:docs/adrs/index.md`
  contains no double space; `linter markdown` passes.
- Resolution reference: `docs(adrs): [#2507] drop the alignment padding from the ADR index row`
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/2508#discussion_r4262205669>

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
- 2026-10-13 17:55 UTC - Rebased the branch onto `develop` at the maintainer's request (no
  conflicts; all six branch commits re-signed). The head `fda1851a4` named in the 17:23 entry is the
  pre-rebase tip `docs(issues): [#2507] link the specification pull request`; the Findings
  introduction now names it by subject. Resolution references cite commit subjects, so they still
  match the rebased branch.
- 2026-10-13 17:56 UTC - Review 5507795043 (`copilot-pull-request-reviewer`, `COMMENTED`, on the
  pre-rebase head) left inline comments 4262164052, 4262164124, and 4262164172; the thread capture
  shows all three `resolved=false`, the first `outdated=true` after the rebase.
- 2026-10-13 17:58 UTC - Probed the pre-commit gate with a staged misspelled file: it exited `1`.
  Committed the F5 fix `docs(adrs): [#2507] drop the alignment padding from the ADR index row`.
- 2026-10-13 17:59 UTC - Committed the F4 fix
  `docs(skills): [#2507] scope the commit-id citation rule to durable references`.
- 2026-10-13 18:00 UTC - Committed the F3 fix
  `docs(skills): [#2507] gate manual commits on the pre-commit script's exit status`, pushed the
  three fixes, re-verified them against the pushed head, and posted the F3, F4, and F5 replies.
- 2026-10-13 18:02 UTC - Recorded F3, F4, and F5 `FIXED`/`RESOLVED` and ran
  `validate-audit-record.py`.

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
