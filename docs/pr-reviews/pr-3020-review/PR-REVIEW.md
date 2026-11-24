---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .claude/skills/process-pr-review/SKILL.md
    - "issue #3019"
---

<!-- skill-link: process-pr-review -->

# PR #3020 Review Audit

Source: pull-request reviews and inline review threads for
<https://github.com/torrust/torrust-tracker/pull/3020>.

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
- When the current identity cannot resolve a thread (`viewerCanResolve` is `false`), keep the
  disposition established from the current tree and record `Thread state=OPEN`. The reviewer or PR
  author resolves it; permission says nothing about whether the concern was fixed.

## Findings

Review 6036567949 (CodeRabbit, 2026-11-24 09:19 UTC) posted two inline comments. Its body only
repeats them, so it adds no row.
Review 6037525308 (CodeRabbit, 2026-11-24 10:26 UTC, on the audit commit) posted one inline
comment, F3; its body only repeats it.
Review 6037619842 (CodeRabbit, 2026-11-24 10:33 UTC) is a reply in the F3 thread that found a
defect in the F3 fix: F4. Review 6037565542 (10:29 UTC) is a reply in the F2 thread that accepts
the plan change and asks for the audit validator to be run; it requests no change, so it adds no
row and was answered in the thread.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-3020-f1 | Unknown | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-3020-f2 | Unknown | Minor | testing | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-3020-f3 | Unknown | Minor | metadata | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-3020-f4 | Unknown | Minor (inferred) | metadata | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - The startup command must select the GitHub Actions cache backend

- PR number: 3020
- Source review ID: 6036567949
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4677842120>
- Concern: the spec did not say that `sccache --start-server` runs with
  `SCCACHE_GHA_ENABLED=true`. The server picks its backend when it starts, so exporting the
  variable to `$GITHUB_ENV` afterwards is too late, and the server would use the local disk cache.
- Solution: the Design and Ownership Review now has a "Backend selection" item: the startup
  command runs with `SCCACHE_GHA_ENABLED=true` and `SCCACHE_IDLE_TIMEOUT=0` in the step's `env:`,
  and both, with `RUSTC_WRAPPER=sccache`, are written to `$GITHUB_ENV` only after the startup
  succeeded. The same commit replaced an unverified statement in the item before it (that the
  post step "only runs for an action used directly by the job") with the reason actually relied
  on.
- Current-tree verification: `grep -n "Backend selection" docs/issues/open/3019-ci-sccache-startup-fallback/ISSUE.md`
  prints the new item.
- Resolution reference: docs(issues): [#3019] start the sccache server with the GitHub Actions cache backend
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4678537103>

### F2 - Hosted verification must cover the jobs the PR checks do not run

- PR number: 3020
- Source review ID: 6036567949
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4677842131>
- Concern: V5 relied on the PR checks, but `coverage.yaml` only runs for pushes to `develop`, and
  `docker-e2e` skips pull requests that target `develop`.
- Solution: the Regression Test Strategy now plans those runs on `josecelano-bot` fork branches,
  where pushes run the push-triggered workflows: `docker-e2e` runs for a feature-branch push, and
  a temporary fork-only commit adds the verification branch to the `push` trigger of
  `coverage.yaml`. New task T6b, scenario M6 and evidence section V6 track it, and AC2 and AC3
  now cite V6. The forced-timeout red and green runs moved to fork branches too, so no temporary
  commit enters the PR.
- Current-tree verification: `grep -c "V6" docs/issues/open/3019-ci-sccache-startup-fallback/ISSUE.md`
  printed `6`; `manual-verification-evidence.md` has a `V6` section.
- Resolution reference: docs(issues): [#3019] verify on fork branches, including the jobs PR checks skip
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4678537650>

### F3 - Cite the issue, not the spec path, in the audit frontmatter

- PR number: 3020
- Source review ID: 6037525308
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4678605284>
- Concern: the audit's `related-artifacts` listed
  `docs/issues/open/3019-ci-sccache-startup-fallback/ISSUE.md`, which goes stale when the spec
  moves to `docs/issues/closed/` after the issue closes.
- Solution: the entry is now `issue #3019`, the form other audit records use (for example
  `issue #2264`).
- Current-tree verification: `sed -n 1,9p docs/pr-reviews/pr-3020-review/PR-REVIEW.md` shows
  `- issue #3019` under `related-artifacts`.
- Resolution reference: docs(pr-reviews): [#3019] cite issue #3019 instead of the spec path in the PR 3020 audit
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4678654975>

### F4 - The unquoted `issue #3019` entry parses as `issue`

- PR number: 3020
- Source review ID: 6037619842
- Reviewer finding ID: N/A
- Source URL: <https://github.com/torrust/torrust-tracker/pull/3020#pullrequestreview-6037619842>
- Concern: in comment
  <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4678683115>, a reply inside
  the F3 thread (so this row cites its review, not a thread of its own), the reviewer found that
  the F3 fix wrote `- issue #3019` unquoted; YAML reads the space and `#3019` after `issue` as a comment, so the
  value is only `issue`.
- Solution: quoted the value as `"issue #3019"`, the form most records use (188 quoted against
  49 unquoted entries in `docs/` and `.claude/` on 2026-11-24). Swept the spec and evidence
  frontmatter of this PR: they have no other `#` value.
- Current-tree verification: `yaml.safe_load` of the frontmatter printed
  `['.claude/skills/process-pr-review/SKILL.md', 'issue #3019']`.
- Resolution reference: docs(pr-reviews): [#3019] quote the issue #3019 entry in the PR 3020 audit frontmatter
- Follow-up PR URL: N/A
- Reply URL: <https://github.com/torrust/torrust-tracker/pull/3020#discussion_r4678732133>

## Processing Log

- 2026-11-24 10:18 UTC - Started audit for CodeRabbit review 6036567949 (F1, F2). Both fixes
  were committed before this entry.
- 2026-11-24 10:20 UTC - Pushed both fixes; replied on both threads (reply `created_at`
  10:20:29Z and 10:20:31Z). CodeRabbit had already resolved the F2 thread; resolved the F1
  thread.
- 2026-11-24 10:31 UTC - Pushed the F3 fix and replied (reply `created_at` 10:31:02Z).
  CodeRabbit had already resolved the thread.
- 2026-11-24 10:37 UTC - Pushed the F4 fix and replied in the F3 thread (reply `created_at`
  10:37:50Z); answered the validator request in the F2 thread (10:37:52Z). All three threads
  were already resolved. Process correction: the F1 thread was resolved at 10:20 UTC before
  `validate-audit-record.py` ran; the skill runs it first. It passed when it ran, before the
  first audit commit.

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
