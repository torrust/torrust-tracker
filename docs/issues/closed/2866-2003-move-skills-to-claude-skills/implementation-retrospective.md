---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2866-2003-move-skills-to-claude-skills/ISSUE.md
    - docs/issues/closed/2866-2003-move-skills-to-claude-skills/manual-verification-evidence.md
    - docs/pr-reviews/pr-2886-review/PR-REVIEW.md
---

# Implementation Retrospective — Move the Agent Skills to `.claude/skills/`

## Purpose

Record evidence-based process improvements discovered while implementing issue #2866. This is a
blameless review of the implementation approach; it does not replace acceptance-criteria
verification.

## Outcome

PR #2886 moved the 53 skills from `.github/skills/dev/<area>/<name>/` to a flat
`.claude/skills/<name>/`, left a redirect stub at each old `SKILL.md` path, added the index
`.claude/skills/README.md`, and added `frontmatter-validator --skill-layout`, which runs on every
commit and in the Docs Lint workflow. In a fresh clone of the branch, Claude Code listed the 53
project skills and ran `/create-issue` as a command (evidence V1 and V2); before the move it
listed none (V0). The design is the one the spec approved; no task changed scope.

## What Went Well

1. Reproducing the defect in a fresh clone before writing code (M0) gave a like-for-like check
   (M1, M2) that proved the fix, not just the file layout.
2. Proving the placement rule red on the tree before the move (T2) and mutation-testing each rule
   increment caught real gaps: a fixture that returned text for removed files, and an index rule
   that counted the intro link as a second listing of `add-new-skill`.
3. Keeping the old paths alive as stubs meant about 520 historical records needed no rewrite, and
   later merges of `develop` that brought new records with old paths broke nothing.
4. A recorded rewrite script plus a classification rule (live, historical, out of scope) made the
   sweep repeatable; the T9 sweep and each later merge reran it in seconds.

## What Changed During Implementation

- **Five legacy specs had to migrate in the move commit.** The commit gate rejects an edited open
  or draft spec still in the pre-v1 shape, and the path rewrite touched five of them. The spec did
  not foresee this; the migration went into T3.
- **`linter lychee` does not check `.claude/`.** `torrust-linting` 0.2.0 passes only `**/*.md` and
  `.github/**/*.md`, and the first glob skips hidden folders, so links inside the skills lost
  link-check coverage. A one-off check found no broken link; the gap is a follow-up for
  `torrust-linting`.
- **The PR diff was too big for Copilot.** With a stub left at each old path, GitHub cannot pair the
  old and new `SKILL.md`, so the PR showed every skill as new: 20,414 changed lines, over Copilot's
  20,000-line limit. Copilot did not review; da2ce7 reviewed alone.
- **`develop` moved three times during review.** Each merge brought specs that the move had
  edited and that were then archived or renamed on `develop`; one needed a hand resolution, and
  the last merge push dismissed the standing approval. The merge message of the hand resolution carried a closing
  keyword (`closed #2850`), recorded as F3 and left as is by the maintainer's decision.
- **Progress-log times were first estimated.** The T2 to T7 entries were written with guessed
  times and corrected to the commit times in the T8 commit.

## Root Cause

- The spec's reference inventory counted files and classified them, but did not ask which edited
  files the commit gate validates more strictly (legacy specs) or which tools cover the new
  folder (lychee's globs). Both were properties of tooling outside the moved files.
- A move that keeps a file at the old path is not a rename in the PR-level diff. The spec relied
  on per-commit rename detection and did not consider how reviewers would see the PR as a whole.
- A long review window on a branch that edits many shared documents invites conflicts with every
  archive PR on `develop`; the merge message was written by hand under that pressure.

## Improvements for Future Work

1. When a spec moves or rewrites many files, check which validators apply stricter rules to
   edited files, and which linters' globs cover the destination, before planning the commits.
2. When a change keeps a file at the old path, estimate the PR-level diff size and say in the spec
   whether an automated reviewer can review it, so the maintainer decides before the PR opens.
3. Run the closing-keyword scan on merge commit messages too before pushing a hand-resolved merge.
4. Take progress-log times from `git log` or `date -u` when writing the entry, never ahead of time.

## Avoiding Overcorrection

No new rule is needed for splitting large moves into several PRs: this PR's size came from the
stubs, which were a deliberate decision, and a human reviewer recomputed the whole change. No
new check is needed for progress-log times; reading the clock is already the rule.

## Evidence

- Issue specification: `docs/issues/closed/2866-2003-move-skills-to-claude-skills/ISSUE.md`
- Manual verification: `manual-verification-evidence.md` in this folder (V0 to V5, T2, T3, T6, T9)
- Implementation PR #2886 and its audit `docs/pr-reviews/pr-2886-review/PR-REVIEW.md`
- ADR `docs/adrs/20261116082711_store_agent_skills_flat_in_claude_skills.md`
- Follow-up issue #2873 (skill descriptions)
