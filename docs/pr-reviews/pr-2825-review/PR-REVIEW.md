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

Round 1: da2ce7 review 5858161814 (changes requested, findings F1 to F11) and Copilot review 5858165805 (five comments, run on the same head), F12 to F16. Copilot's review body is an overview and a summary of the change, with no assertion beyond its five inline comments. F12 repeats F3.

Round 2: da2ce7 review 5858720859 (approved, with findings F17 to F24; his `F11` collided with round 1's `F11` and is `F25`, a re-raise of F11 because the workflow marker was only partly fixed), submitted 12:30:01Z.

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
| F17 | review-finding:pr-2825-f17 | Human | Minor | documentation | ORIGINAL | FIXED | RESOLVED |
| F18 | review-finding:pr-2825-f18 | Human | Nit | correctness | ORIGINAL | FIXED | RESOLVED |
| F19 | review-finding:pr-2825-f19 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F20 | review-finding:pr-2825-f20 | Human | Nit | metadata | ORIGINAL | FIXED | RESOLVED |
| F21 | review-finding:pr-2825-f21 | Human | Nit | maintainability | ORIGINAL | NO_ACTION | SUPERSEDED |
| F22 | review-finding:pr-2825-f22 | Human | Nit | formatting | ORIGINAL | FIXED | RESOLVED |
| F23 | review-finding:pr-2825-f23 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F24 | review-finding:pr-2825-f24 | Human | Nit | documentation | ORIGINAL | FIXED | RESOLVED |
| F25 | review-finding:pr-2825-f25 | Human | Nit | metadata | RE_RAISE_OF:F11 | FIXED | RESOLVED |

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
- Concern: `packages/e2e-tools/Cargo.toml` requires `torrust-tracker` at `3.0.0-develop`; the removed sentence covered it, and at step 9 a pre-release matches only a requirement that names a pre-release of the same major.minor.patch (wording corrected in round 2, finding F18).
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
- Resolution reference: docs(release): [#2808] fix the title, grammar, code tag and step 9 heading of tracker.md; docs(issues): [#2808] answer the review of PR 2825 in the specs and the evidence
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

### F17 - The completion review calls the discovery material but creates no retrospective

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876251
- Concern: The 11:57 log entry said the dependents' requirement was the one material discovery and no retrospective was needed; the checkpoint stayed unticked.
- Solution: Created `implementation-retrospective.md`, ticked the checkpoint, pointed the Retrospective line at it, and appended a correction to the log.
- Current-tree verification: `ls docs/issues/open/2808-*/implementation-retrospective.md` lists it; `grep -n "\[x\] Evidence-based" docs/issues/open/2808-*/ISSUE.md` finds the ticked line.
- Resolution reference: docs(issues): [#2808] add the implementation retrospective and tick the completion review
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537945691

### F18 - "same pre-release" is narrower than Cargo's rule

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876260
- Concern: A pre-release matches only a comparator naming a pre-release of the same major.minor.patch; the skill and the audit's F2 Concern said "the same pre-release".
- Solution: Skill wording now: "matches only a requirement that names a pre-release of `3.0.1`"; the F2 Concern in this audit has the same wording.
- Current-tree verification: `grep -n "names a pre-release" .github/skills/dev/git-workflow/release-new-version/SKILL.md` finds it.
- Resolution reference: docs(skills): [#2808] state Cargo's pre-release matching rule precisely in release-new-version
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537946460

### F19 - Week 3 example does not mention the raised requirement

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876270
- Concern: The `0.2.0` to `0.3.0` bump is incompatible, and precondition 2 also raises the tracker's requirement.
- Solution: Added the clause to the week 3 example.
- Current-tree verification: `grep -n "precondition 2" docs/release-process/workspace-packages.md` finds the example clause.
- Resolution reference: docs(release): [#2808] tie the week 3 example to the dependents' requirement precondition
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537946879

### F20 - Re-scoped specs keep their old stamp and have no log entry

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876277
- Concern: Specs #2583 and #2594 changed without a `last-updated-utc` change or a progress-log entry.
- Solution: Set both stamps from the commit time and appended a one-line log entry naming #2808 and PR 2825.
- Current-tree verification: `git show` of the commit shows both stamps at 2026-11-11 12:34 and the added entries.
- Resolution reference: docs(issues): [#2808] stamp and log the re-scoped specs of #2583 and #2594
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537947255

### F21 - Fix commits carry several findings each

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876286
- Concern: Commits 9c791e522 and a925c2434 each carry several independent fixes, and the first has no body; the review asks for one commit per finding next round.
- Solution: No history rewrite was asked for. This round's fixes use one commit per finding.
- Current-tree verification: `git log` over the round 2 fix commits lists one commit per finding (retrospective, skill-link, example, index, spec stamps, pre-release wording).
- Resolution reference: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537947635
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537947635

### F22 - The new index description cell is one character short

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876297
- Concern: The closing pipe of the `release_process.md` row sat one column left of the adjacent rows.
- Solution: Added one space.
- Current-tree verification: `awk '/release_process.md\]\(release_process.md\)|profiling.md\]\(profiling/{print length($0)}' docs/index.md` prints the same length for the release row and the adjacent `profiling.md` row.
- Resolution reference: docs(index): [#2808] align the release index row of the docs table
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537947971

### F23 - The first audit log entry is stamped before an event it names, and records no validator run

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876305
- Concern: The first entry (11:41) names the Copilot review submitted at 11:42:06Z, and the log never states when the validator ran; replies and resolutions came before the record was committed.
- Solution: Appended a correction entry to the Processing Log (the log is append-only) with the real stamps and the validator order.
- Current-tree verification: The Processing Log of this record ends with the correction entry.
- Resolution reference: docs(pr-reviews): [#2808] record round 2 of the PR 2825 review audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537948341

### F24 - F13's Resolution reference cites only one of two commits

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: N/A
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876313
- Concern: The index title change of F13 was made in a different commit than the `tracker.md` one.
- Solution: F13's Resolution reference now lists both commit subjects separated by `;`.
- Current-tree verification: The F13 detail entry of this record shows both subjects; the validator exits 0.
- Resolution reference: docs(pr-reviews): [#2808] record round 2 of the PR 2825 review audit
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537948707

### F25 - The workflow lacks the skill-link marker (partly fixed F11)

- PR number: 2825
- Source review ID: 5858720859
- Reviewer finding ID: F11
- Source URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537876256
- Concern: The new skill lists `deployment-packages.yaml`, but the workflow carries no `skill-link: release-workspace-package` marker.
- Solution: Added the marker to the workflow header.
- Current-tree verification: `grep -n skill-link .github/workflows/deployment-packages.yaml` finds both markers.
- Resolution reference: ci(deployment-packages): [#2808] add the release-workspace-package skill-link marker
- Follow-up PR URL: N/A
- Reply URL: https://github.com/torrust/torrust-tracker/pull/2825#discussion_r4537946030

## Processing Log

- 2026-11-11 11:41 UTC - Started audit: da2ce7 review 5858161814 (submitted 11:41:44Z) and Copilot review 5858165805 (11:42:06Z), both on the head of the PR when opened; 16 inline threads, all unresolved.
- 2026-11-11 11:57 UTC - Fixes committed in four signed commits (the last at 11:57:42Z by `git log --format=%cI`): `docs(release): [#2808] fix the title, grammar, code tag and step 9 heading of tracker.md`, `docs(release): [#2808] require dependents' version requirements in the version-bump PR`, `docs(skills): [#2808] name the e2e-tools requirement, add skill links, align the docs index wording`, `docs(issues): [#2808] answer the review of PR 2825 in the specs and the evidence`. The pre-commit gate ran on each.
- 2026-11-11 12:01 UTC - Replied on all 16 threads (reply `created_at` from 12:00:29Z to 12:01:08Z); F12 uses the `Superseded by F3` form.
- 2026-11-11 12:02 UTC - Resolved the 16 threads through GraphQL; each mutation returned `isResolved: true` (16 results). The PR description was changed to `Closes #2808` for F6 before the replies.
- 2026-11-11 12:32 UTC - Round 2: da2ce7 review 5858720859 (approved, submitted 12:30:01Z) with nine comments, all unresolved. Fix commits at 12:32:14Z, 12:32:44Z, 12:33:14Z, 12:33:44Z, 12:34:13Z and 12:35:13Z (`git log --format=%cI`), one per finding, pushed after the last.
- 2026-11-11 12:37 UTC - Replied on the nine round 2 threads (reply `created_at` 12:37:24Z to 12:37:43Z); F21 uses the `Superseded by F21` form.
- 2026-11-11 12:38 UTC - Resolved the nine threads through GraphQL (nine results, each `isResolved: true`).
- 2026-11-11 12:38 UTC - Correction to the first entry (audit F23): it was stamped 11:41 but names the Copilot review submitted at 11:42:06Z; the first fix commit is at 11:54:47Z. Also, `validate-audit-record.py` first ran against this record after the round 1 replies and resolutions (it needs the reply URLs) and before the first audit commit (12:02:56Z), with `failures: 0`; for round 2 it ran before the audit commit and reported only the two resolution references (F23, F24) that name that commit's own subject, which cannot exist before the commit.
- Correction (audit F18, F24): the F2 Concern wording and the F13 Resolution reference were edited in place in round 2; the earlier text said "a pre-release only matches a requirement with the same pre-release" and cited only the `tracker.md` commit.

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
