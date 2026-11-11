---
semantic-links:
  skill-links:
    - process-pr-review
  related-artifacts:
    - .github/skills/dev/pr-reviews/process-pr-review/SKILL.md
---

<!-- skill-link: process-pr-review -->

# PR #2825 Review Audit

Source: pull-request reviews and inline review threads for https://github.com/torrust/torrust-tracker/pull/2825.

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

One round: da2ce7 review 5858161814 (changes requested, findings F1 to F11) and Copilot review 5858165805 (five comments, run on the same head), F12 to F16. Copilot's review body is an overview and a summary of the change, with no assertion beyond its five inline comments. F12 repeats F3.

| Finding ID | Review finding reference | Author class | Severity | Category | Relationship | Disposition | Thread state |
| ---------- | ------------------------ | ------------ | -------- | -------- | ------------ | ----------- | ------------ |
| F1 | review-finding:pr-2825-f1 | Human | Major | correctness | ORIGINAL | FIXED | RESOLVED |
| F2 | review-finding:pr-2825-f2 | Human | Minor | correctness | ORIGINAL | FIXED | RESOLVED |
| F3 | review-finding:pr-2825-f3 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F4 | review-finding:pr-2825-f4 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F5 | review-finding:pr-2825-f5 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F6 | review-finding:pr-2825-f6 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F7 | review-finding:pr-2825-f7 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F8 | review-finding:pr-2825-f8 | Human | Nit | link-integrity | ORIGINAL | FIXED | RESOLVED |
| F9 | review-finding:pr-2825-f9 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F10 | review-finding:pr-2825-f10 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F11 | review-finding:pr-2825-f11 | Human | Nit | maintainability | ORIGINAL | FIXED | RESOLVED |
| F12 | review-finding:pr-2825-f12 | Copilot | Minor (inferred) | documentation | RE_RAISE_OF:F3 | NO_ACTION | SUPERSEDED |
| F13 | review-finding:pr-2825-f13 | Copilot | Minor (inferred) | maintainability | ORIGINAL | FIXED | RESOLVED |
| F14 | review-finding:pr-2825-f14 | Copilot | Minor (inferred) | documentation | ORIGINAL | FIXED | RESOLVED |
| F15 | review-finding:pr-2825-f15 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |
| F16 | review-finding:pr-2825-f16 | Copilot | Nit (inferred) | formatting | ORIGINAL | FIXED | RESOLVED |

## Finding Details

### F1 - Step 5 moves only the root requirement, after publishing

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430486
- Concern: The old step 5 updated only the root `Cargo.toml` dependency entry after publishing, while 14 manifests declare a versioned path requirement on `torrust-tracker-primitives`; an incompatible bump does not build until every dependent is raised in the version-bump PR.
- Solution: Moved the requirement update into precondition 2 (every manifest that depends on the crate, required for an incompatible bump or when a dependent needs the new version, optional otherwise) and removed step 5.
- Current-tree verification: `grep -n "every manifest" docs/release-process/workspace-packages.md` finds the sub-point; `grep -c "follow-up PR" docs/release-process/workspace-packages.md` prints 0.
- Resolution reference: docs(release): [#2808] require dependents' version requirements in the version-bump PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537599757

### F2 - Steps 2 and 9 drop the e2e-tools requirement on the tracker

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430498
- Concern: `packages/e2e-tools/Cargo.toml` requires `torrust-tracker` at `3.0.0-develop`; the removed sentence covered it, and at step 9 a pre-release only matches a requirement with the same pre-release.
- Solution: Both comments now name the root `Cargo.toml` and the `torrust-tracker` requirement in `packages/e2e-tools/Cargo.toml`, and the note says every requirement on `torrust-tracker` is bumped.
- Current-tree verification: `grep -c e2e-tools .github/skills/dev/git-workflow/release-new-version/SKILL.md` prints 2.
- Resolution reference: docs(skills): [#2808] name the e2e-tools requirement, add skill links, align the docs index wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537600114

### F3 - T2 of the first-publish spec describes defects this PR fixed

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430509
- Concern: The T2 row of the first-publish spec still described the `torrust/main` push, the old step 6 and the removed skill content.
- Solution: Rewrote the row to record what #2808 fixed and keep what remains.
- Current-tree verification: `sed -n 273p` of the spec shows the new row, which begins "Not blocked. The release-process spec #2808 already fixed".
- Resolution reference: docs(issues): [#2808] answer the review of PR 2825 in the specs and the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537600482

### F4 - The client-extraction spec still cites the removed file list

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430513
- Concern: Spec #2594 (lines 91 and 150) says `release-new-version` lists `console/tracker-client/Cargo.toml` and plans to update that list.
- Solution: Line 91 now says the skill no longer lists the file (removed by #2808); the task item that updated the list is dropped.
- Current-tree verification: `grep -n release-new-version docs/issues/open/2594-*/ISSUE.md` finds only the rewritten line.
- Resolution reference: docs(issues): [#2808] answer the review of PR 2825 in the specs and the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537600851

### F5 - V4 credits link checking to markdownlint

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430520
- Concern: Local file and fragment links are checked by `linter lychee`, not markdownlint.
- Solution: V4 now names `linter lychee` and its exit code 0, run at the fix head.
- Current-tree verification: `linter lychee` exited 0 (the shell `$?` printed after redirecting its output to a file).
- Resolution reference: docs(issues): [#2808] answer the review of PR 2825 in the specs and the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537601465

### F6 - Implementation PR linked with Related to; completion review pending

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430529
- Concern: The PR implements every task but its description said `Related to #2808`, and the completion review was not recorded.
- Solution: Changed the PR description to `Closes #2808` and appended a progress-log entry that records the completion review and what the maintainer's close-out still does.
- Current-tree verification: `gh-bot pr view 2825` body starts with `Closes #2808`; the spec's progress log has the completion-review entry.
- Resolution reference: docs(issues): [#2808] answer the review of PR 2825 in the specs and the evidence
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537601782

### F7 - docs index row still describes the old pipeline

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430536
- Concern: The `release_process.md` row of `docs/index.md` still read "Branch strategy, versioning, and the staging → main release pipeline".
- Solution: Use the wording of `docs/AGENTS.md`: "Release index: versioning policy and which release process to use".
- Current-tree verification: `grep -n "staging → main" docs/index.md` prints nothing.
- Resolution reference: docs(skills): [#2808] name the e2e-tools requirement, add skill links, align the docs index wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537602132

### F8 - "see the ADR above" points at nothing

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430543
- Concern: The ADR appears only in the frontmatter of the rendered document.
- Solution: Linked `../adrs/20260629000000_adopt_independent_package_versioning.md` in the text.
- Current-tree verification: `grep -n "ADR\](" docs/release-process/workspace-packages.md` finds the link; `linter lychee` exited 0.
- Resolution reference: docs(release): [#2808] require dependents' version requirements in the version-bump PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537602542

### F9 - The example's commands contradict the steps

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430550
- Concern: The example pushed `develop` HEAD and ran `git push --tags`, against steps 2 and 4.
- Solution: Use the step forms (push `<sha>:refs/heads/...`, tag `<sha>`, push one tag ref) and remove the "abbreviated" note.
- Current-tree verification: `grep -c "git push --tags\|develop:releases" docs/release-process/workspace-packages.md` prints 0.
- Resolution reference: docs(release): [#2808] require dependents' version requirements in the version-bump PR
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537602935

### F10 - "the Tracker document" is ambiguous

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430558
- Concern: The text meant a section of `workspace-packages.md`, not `tracker.md`.
- Solution: Name and link the "Pointing to This Process from Package Documentation" section.
- Current-tree verification: `sed -n 38,45p` of the follow-up issue text shows the named section and link.
- Resolution reference: docs(skills): [#2808] name the e2e-tools requirement, add skill links, align the docs index wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537603327

### F11 - The new skill has no Skill Links section

- PR number: 2825
- Source review ID: 5858161814
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537430569
- Concern: `add-new-skill` asks for a short "Skill Links" section.
- Solution: Added it, listing `docs/release-process/workspace-packages.md` and `.github/workflows/deployment-packages.yaml`.
- Current-tree verification: `grep -n "^## Skill Links" .github/skills/dev/git-workflow/release-workspace-package/SKILL.md` finds it.
- Resolution reference: docs(skills): [#2808] name the e2e-tools requirement, add skill links, align the docs index wording
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537603721

### F12 - The T2 row of the first-publish spec is stale

- PR number: 2825
- Source review ID: 5858165805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537433565
- Concern: The same stale T2 row as F3.
- Solution: Superseded by F3: fixed by the same rewrite of the row.
- Current-tree verification: `sed -n 273p` of the spec shows the rewritten row.
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537604110
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537604110

### F13 - tracker.md title hard-codes a version

- PR number: 2825
- Source review ID: 5858165805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537433612
- Concern: The title carried `(v2.2.2)`, which goes stale.
- Solution: Removed the version from the title of `tracker.md` and of the index.
- Current-tree verification: `head -14 docs/release-process/tracker.md` shows the title without a version.
- Resolution reference: docs(release): [#2808] fix the title, grammar, code tag and step 9 heading of tracker.md
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537604406

### F14 - Step 9 title says Create Comment

- PR number: 2825
- Source review ID: 5858165805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537433637
- Concern: The commands create a commit.
- Solution: Renamed to "Create Commit that bumps next development version".
- Current-tree verification: `grep -n "Create Comment" docs/release-process/tracker.md` prints nothing.
- Resolution reference: docs(release): [#2808] fix the title, grammar, code tag and step 9 heading of tracker.md
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537604778

### F15 - Grammar: "that the your git"

- PR number: 2825
- Source review ID: 5858165805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537433680
- Concern: Typo in the note before the remote output.
- Solution: Now "that your git".
- Current-tree verification: `grep -n "the your" docs/release-process/tracker.md` prints nothing.
- Resolution reference: docs(release): [#2808] fix the title, grammar, code tag and step 9 heading of tracker.md
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537605191

### F16 - Code block tagged `s`

- PR number: 2825
- Source review ID: 5858165805
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537433710
- Concern: `s` is not a highlighting identifier for command output.
- Solution: Tagged the block `text`.
- Current-tree verification: `grep -n '```s$' docs/release-process/tracker.md` prints nothing.
- Resolution reference: docs(release): [#2808] fix the title, grammar, code tag and step 9 heading of tracker.md
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537605564

## Processing Log

- 2026-11-11 11:41 UTC - Started audit: da2ce7 review 5858161814 (submitted 11:41:44Z) and Copilot review 5858165805 (11:42:06Z), both on the head of the PR when opened; 16 inline threads, all unresolved.
- 2026-11-11 11:57 UTC - Fixes committed in four signed commits (the last at 11:57:42Z by `git log --format=%cI`): `docs(release): [#2808] fix the title, grammar, code tag and step 9 heading of tracker.md`, `docs(release): [#2808] require dependents' version requirements in the version-bump PR`, `docs(skills): [#2808] name the e2e-tools requirement, add skill links, align the docs index wording`, `docs(issues): [#2808] answer the review of PR 2825 in the specs and the evidence`. The pre-commit gate ran on each.
- 2026-11-11 12:01 UTC - Replied on all 16 threads (reply `created_at` from 12:00:29Z to 12:01:08Z); F12 uses the `Superseded by F3` form.
- 2026-11-11 12:02 UTC - Resolved the 16 threads through GraphQL; each mutation returned `isResolved: true` (16 results). The PR description was changed to `Closes #2808` for F6 before the replies.

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
