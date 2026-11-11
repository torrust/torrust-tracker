---
schema-version: 1
doc-type: issue
issue-type: task
status: done
priority: p2
epic: null
github-issue: 2808
spec-path: docs/issues/closed/2808-workspace-package-release-process/ISSUE.md
branch: "2808-workspace-package-release-process"
related-pr: 2825
last-updated-utc: "2026-11-11 13:51"
semantic-links:
  skill-links:
    - create-issue
    - release-new-version
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/git-workflow/release-new-version/SKILL.md
    - docs/release_process.md
    - docs/adrs/20260629000000_adopt_independent_package_versioning.md
    - .github/workflows/deployment-packages.yaml
---

<!-- skill-link: create-issue -->

# Issue #2808 - Define the workspace-package release process, its skill, and how packages point to it

## Goal

Make releasing a workspace package, or a standalone Torrust crate extracted from the workspace, a
documented and repeatable process: one source of truth in this repository, one skill that drives
it, and package-level docs that only point to it.

## Background

Releasing `torrust-server-lib` 0.6.0 (issue torrust/torrust-server-lib#5, PR torrust/torrust-server-lib#6; release docs issue
torrust/torrust-server-lib#7, PR torrust/torrust-server-lib#8) exposed gaps in how this repository documents package releases:

- The only release skill, [release-new-version](../../../../.github/skills/dev/git-workflow/release-new-version/SKILL.md),
  covers the tracker binary. There is **no skill for a workspace package**.
- The source of truth is `docs/release_process.md`, section "Publishing a Workspace Package"
  (ADR `20260629000000_adopt_independent_package_versioning.md`). It has defects found while
  reading it for this issue:
  - The "Automated Workflow" pushes `develop` to `releases/pkg/<crate>/v<semver>` and tags
    afterwards, but never says the published commit must be the one that carries the version
    bump and changelog entry, nor that the tag must point to the published commit. Step 5
    also pushes `torrust/main` to a branch named like the tag and then runs `git tag`, which
    reads as creating the same name twice (a branch and a tag).
  - The "Manual Fallback" says nothing about who runs it, from which commit, or about tagging.
  - It does not say that **no GitHub release** is created for a package. The ADR does ("GitHub
    Releases are used only for tracker application releases"); the doc only lists a GitHub
    release in the tracker flow. The maintainer confirmed this on 2026-11-10.
  - It has no guidance for a package that lives in **its own repository**.
- `release-new-version` step 6 lists crates the deployment workflow must publish
  (`torrust-located-error`, `torrust-tracker-primitives`, ...), which contradicts the ADR: a
  tracker release publishes only `torrust-tracker`. Its steps 2 and 9 also say to edit the version
  in all `Cargo.toml` files, which contradicts independent versioning. This is drift to fix or
  remove, not to copy.
- `torrust-server-lib`'s `docs/release_process.md` already points to this repository's
  `#publishing-a-workspace-package` anchor, but it ends with a three-step checklist that repeats
  the version-bump PR, publish and signed-tag steps.
- `torrust-server-lib` 0.6.0 was published manually from merged `main` (`cargo publish --dry-run`,
  then `cargo publish`), and the signed tag `v0.6.0` was created afterwards on the published commit
  (2dd04f4) by the maintainer, since agents never push to upstream. That repo has no `develop`
  branch and no deployment workflow, and tags `v<semver>` while this repository tags
  `pkg/<crate>/v<semver>`. Its `docs/release_process.md` now documents the divergence.
- Releases of `torrust-server-lib` <= 0.5.0 have no tags. Known candidate: 0.5.0 -> 2df974f.

Decisions already made by the maintainer (2026-11-10):

- This repository's release documentation stays the single source of truth.
- `docs/release_process.md` is split into a short index plus one document per process:
  `docs/release-process/tracker.md` (tracker binary) and `docs/release-process/workspace-packages.md`
  (workspace packages and standalone-repository crates). The index keeps a
  `## Publishing a Workspace Package` heading that redirects to the packages document, so existing
  links and anchors keep working.
- No GitHub release is created for workspace packages.
- `torrust-server-lib` 0.6.0 was published manually from merged `main` and tagged `v0.6.0` on the
  published commit.
- A crate in its own repository tags releases `v<semver>` (signed, on the published commit). The
  `pkg/<crate>/` prefix exists only to tell workspace crates apart, so a single-crate repository
  does not need it. The release doc lists this as a standalone-repository difference.
- Missing `torrust-server-lib` tags (<= 0.5.0) are backfilled, but only for versions whose published
  commit is verified against crates.io. The maintainer pushes the tags.
- Package READMEs point to `docs/release-process/workspace-packages.md` on `develop` (not a permalink, and
  not an anchor inside the tracker document). A short "Releasing" section is required only in standalone repositories and in
  workspace READMEs that already describe releasing; other READMEs are not changed.
- The specification goes through a spec-only PR before implementation.
- Implementation starts only after #2576 (update all package READMEs and crates.io metadata) is
  closed. That issue rewrites every package README against a new
  `docs/templates/PACKAGE-README.md`; doing the README pointers here in parallel would edit the same
  files, so the whole implementation waits and builds on its result.
- The packages document is named `docs/release-process/workspace-packages.md`, not `packages.md`,
  so it is not confused with the existing package catalogue `docs/packages.md`.

## Scope

### In Scope

- (s) **Split the release document.** Move the tracker binary steps (current steps 1-10 and
  container image tags) to `docs/release-process/tracker.md` and the package section (including
  the real-world example) to `docs/release-process/workspace-packages.md`, unchanged in content. Reduce
  `docs/release_process.md` to an index: the versioning policy, which process to use, links to
  both documents, and the redirecting `## Publishing a Workspace Package` heading. Update live
  references (`AGENTS.md`, `docs/AGENTS.md`, `docs/index.md`, the ADR's related artifacts, both
  deployment workflows, `release-new-version`, and open issue specs such as 2583 and 2183). Closed
  issue specs and PR review records keep their historical paths.
- (a) **The process.** Define precisely, in `docs/release-process/workspace-packages.md`, how a package is released:
  preconditions (own explicit `version`, changelog entry, tests, dry run), who does what (agent:
  PR, dry run, and publish only if cargo is authenticated and the maintainer approved; maintainer:
  signed tag and any upstream push), publish only from the merged commit, tag that exact commit,
  no GitHub release, order of steps, and the two variants: a workspace package (the
  `releases/pkg/**` workflow) and a standalone-repository crate (manual publish).
  Fix the step 5 tag/branch ambiguity.
- (b) **Skills.** Add `release-workspace-package` under `.github/skills/dev/git-workflow/`. It
  links to the doc for the steps and holds no copy of them. Decide whether it covers standalone
  crates as a variant (preferred: one skill, one variant section) and cross-link it from
  `release-new-version`. Remove the contradicting crate list from `release-new-version` step 6, and
  make its steps 2 and 9 bump only the `torrust-tracker` version, as the ADR requires.
- (c) **Package documentation.** Apply the decided pointer (a link to
  `docs/release-process/workspace-packages.md` on `develop`) to the workspace READMEs that already describe releasing, and specify the short
  "Releasing" section text for standalone repositories. A package doc carries only a pointer plus
  its differences, never a copy of the steps.
- (d) **Tag convention for standalone repositories**: document `v<semver>` as the
  standalone-repository difference, and prepare the follow-up `torrust-server-lib` issue text. It
  covers the backfill of verified tags for releases <= 0.5.0 (the maintainer pushes the tags) and
  replacing the duplicated checklist in its `docs/release_process.md` with a pointer to
  `docs/release-process/workspace-packages.md` plus its differences.

### Out of Scope

- Changing `deployment-packages.yaml` behavior or the tracker binary release steps (they only move
  to `docs/release-process/tracker.md`).
- Creating any tag, branch, or release upstream (maintainer only).
- Releasing any crate as part of this issue.
- Editing `torrust-server-lib` documents directly; they are changed through its own issues and PRs.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md`
- ADRs to create: `None known`. The workspace tag convention in the ADR does not change; the
  standalone `v<semver>` tag is a documented difference, not a new architectural decision. If the
  standalone-repository policy grows beyond tagging during implementation, stop and create an ADR.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable. Defects in the existing docs are fixed as part of the documentation work (a), not
as a behavior bug.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`. #2576 was closed before implementation started.

| ID  | Status  | Task                                                                                                                  | Notes / Expected Output                                                                                                                                    |
| --- | ------- | --------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE    | Record maintainer decisions for the split, (c) pointer format, and (d) tag convention and backfill                    | Recorded in Background on 2026-11-10; no ADR needed                                                                                                        |
| T2  | DONE    | Split `docs/release_process.md` into the index, `tracker.md` and `workspace-packages.md` (content moved, not changed) | Old anchors still resolve; live references updated; `git diff --color-moved` marks the moved lines                                                         |
| T3  | DONE    | Rewrite the package process in `docs/release-process/workspace-packages.md` (process, roles, no GitHub release)       | Steps ordered; tag/branch ambiguity fixed; workspace and standalone variants                                                                               |
| T4  | DONE    | Add the `release-workspace-package` skill and cross-link it                                                           | Skill links to `workspace-packages.md`; `release-new-version` links to `tracker.md`, loses the step 6 drift, and steps 2 and 9 bump only `torrust-tracker` |
| T5  | DONE    | Add pointer docs to package READMEs as decided in (c)                                                                 | Pointer only; no copied steps                                                                                                                              |
| T6  | DONE    | Prepare the follow-up `torrust-server-lib` issue text (tag backfill, docs cleanup)                                    | Text ready for the maintainer; nothing created without approval                                                                                            |

## Commit Points

| Task | Coherent change set                                                     | Commit policy                                        |
| ---- | ----------------------------------------------------------------------- | ---------------------------------------------------- |
| T2   | Pure move of release doc content plus reference updates (no rewording)  | Commit after focused validation and required review. |
| T3   | Package process rewrite in `docs/release-process/workspace-packages.md` | Commit after focused validation and required review. |
| T4   | New skill, cross-links, `release-new-version` steps 2, 6 and 9 fix      | Commit after focused validation and required review. |
| T5   | Package README pointers                                                 | Commit after focused validation and required review. |

Keep T2 a pure move so reviewers can check it with `git diff --color-moved` before reading the rewrite in T3. Rename detection (`git diff -M`) cannot pair it, because the index keeps the old path.

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/workspace-package-release-process/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-10 16:07 UTC - Claude Code (Claude Sonnet 5.5) - Drafted spec from the maintainer's draft after reading the release process doc, the ADR, `release-new-version` and `deployment-packages.yaml`.
- 2026-11-10 16:10 UTC - Claude Code (Claude Opus 5.5) - Recorded maintainer decisions: standalone tag `v<semver>`, verified backfill of server-lib <= 0.5.0, README pointer to `develop` with anchor, spec-only PR first. T1 done; ADR task dropped.
- 2026-11-10 16:15 UTC - Claude Code (Claude Opus 5.5) - Maintainer approved splitting `docs/release_process.md` into an index, `docs/release-process/tracker.md` and `docs/release-process/packages.md`. Added scope (s) and task T2 (pure move); README pointer now targets `packages.md`; tasks renumbered.
- 2026-11-10 16:17 UTC - Claude Code (Claude Opus 5.5) - Maintainer approved the spec; created issue #2808 as `josecelano-bot`; moved the spec to `docs/issues/open/2808-workspace-package-release-process/`.
- 2026-11-10 16:24 UTC - Claude Code (Claude Opus 5.5) - Maintainer decided the whole implementation waits until #2576 is closed (its README rewrite overlaps the README pointers here) and renamed the planned packages document to `docs/release-process/workspace-packages.md` to avoid confusion with `docs/packages.md`.
- 2026-11-10 16:49 UTC - Claude Code (Claude Opus 5.5) - Addressed the PR 2809 review of da2ce7 (F1-F5): T2 evidence is `git diff --color-moved` (the index keeps the old path, so `git diff -M` cannot pair a rename); the server-lib issue reference is qualified; the 16:15 entry, which the previous commit had rewritten, is restored to its original wording; the log times, which had been written one hour or more ahead of UTC, are corrected to UTC from the lint runs, the issue creation and the commit times; T2-T6 are `BLOCKED` by #2576.
- 2026-11-10 16:58 UTC - Claude Code (Claude Opus 5.5) - Addressed the Copilot review of PR 2809 (audit F6, F7): T4 also makes `release-new-version` steps 2 and 9 bump only `torrust-tracker`; T6 and M2 now cover the checklist that `torrust-server-lib`'s release doc still duplicates, through the follow-up issue text instead of an edit in that repository.
- 2026-11-10 17:23 UTC - Claude Code (Claude Opus 5.5) - Correction to the 16:49 entry (audit F8): the log times were not uniformly "one hour or more ahead of UTC". Before the F4 fix the first entry read 12:00 for 16:07 (4 h 07 min behind); the next three read 16:30, 16:45 and 17:00 for 16:10, 16:15 and 16:17 (20, 30 and 43 min ahead); only the last read 17:30 for 16:24 (66 min ahead).
- 2026-11-10 18:04 UTC - Claude Code (Claude Sonnet 5.5) - Addressed the Copilot review of 18:02 UTC on PR 2809 (audit F20): the torrust-server-lib PR references in Background are fully qualified. `related-pr` stays `null` (it names the implementation PR, as in the sibling spec-only PRs).
- 2026-11-10 19:18 UTC - Claude Code (Claude Sonnet 5.5) - Correction to the 18:04 entry (audit F27): it said `related-pr` "names the implementation PR". The tree does not define the field that way: its schema calls it "the nullable positive related pull request number", the specs introduced by the spec-only PRs for #2798 and #2801 keep it `null`, and the specs of #2393 to #2396 set it to their spec-only PR #2397. This spec keeps it `null` on the first precedent.
- 2026-11-11 11:17 UTC - Claude Code (Claude Sonnet 5.5) - #2576 closed; implemented T2-T6 in the implementation PR: pure-move split (T2), process rewrite (T3), `release-workspace-package` skill and `release-new-version` fixes (T4), no README describes releasing so none changed and the standalone "Releasing" text is in `workspace-packages.md` (T5), `torrust-server-lib-followup-issue.md` (T6); manual verification recorded in `manual-verification-evidence.md`. Open specs #2583 (T2 overlaps T3 here) and #2183 now name the new documents.
- 2026-11-11 11:57 UTC - Claude Code (Claude Sonnet 5.5) - Implementation completion review, after the review of PR 2825: the one material discovery (audit F1) was that a published crate's version bump must raise the `version` requirement in every dependent manifest in the same PR; it is now precondition 2 of `workspace-packages.md`, so no separate retrospective is needed. The PR description uses `Closes #2808`: the issue closes on merge, and the spec move to `docs/issues/closed/` and the torrust-server-lib issue creation are done by the maintainer's close-out afterwards.
- 2026-11-11 12:34 UTC - Claude Code (Claude Sonnet 5.5) - Correction to the 11:57 entry (audit F17): the discovery in audit F1 is material (a retained step that a review had to remove), so the completion review creates `implementation-retrospective.md` instead of the "no separate retrospective" the 11:57 entry stated.
- 2026-11-11 13:51 UTC - Claude Code (Claude Sonnet 5.5) - Archived after PR 2825 merged (issue #2808 closed on GitHub): spec moved to `docs/issues/closed/`, `status: done`, `related-pr: 2825`; da2ce7 approved the final head. The torrust-server-lib issue (tag backfill and checklist cleanup) is still to be created by the maintainer from `torrust-server-lib-followup-issue.md`.

## Acceptance Criteria

- [x] AC0: `docs/release_process.md` is an index linking `docs/release-process/tracker.md` and
      `docs/release-process/workspace-packages.md`; the tracker steps are moved without changes; the
      `#publishing-a-workspace-package` anchor still resolves; live references point to the new
      documents.
- [x] AC1: `docs/release-process/workspace-packages.md` defines the package release process end to end (preconditions,
      roles, order, publish from the merged commit, tag on the published commit, no GitHub
      release) for both a workspace package and a standalone-repository crate.
- [x] AC2: A `release-workspace-package` skill exists, links to `workspace-packages.md`, and duplicates none of its steps.
- [x] AC3: `release-new-version` links to `tracker.md`, no longer contradicts the ADR, and
      cross-links the new skill.
- [x] AC4: Package docs carry a pointer to `workspace-packages.md` (plus differences) and no copied steps.
- [x] AC5: The standalone-repository `v<semver>` tag is documented as a difference, and the
      `torrust-server-lib` follow-up issue text (tag backfill and docs cleanup) is ready for the
      maintainer.
- [x] AC6: A fresh agent can release a workspace package or a standalone crate using only the
      skill and the doc.
- [x] `linter all` exits with code `0`
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- Link check of the new skill, doc, and README pointers (covered by `linter markdown`)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                  | Human-oriented command/steps                                                                                                                          | Expected Result                                                                                                                                   | Status | Evidence                                     |
| --- | ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Walk the workspace-package path (dry run) | Follow only the skill and doc for a real package up to `cargo publish -p <crate> --dry-run`; do not publish or push                                   | Every step is findable and unambiguous; the dry run succeeds                                                                                      | DONE   | `manual-verification-evidence.md` section V1 |
| M2  | Walk the standalone-crate path (dry run)  | Follow only the skill and doc for `torrust-server-lib` up to `cargo publish --dry-run`; do not publish or push                                        | The standalone variant in `workspace-packages.md` is complete on its own; the duplication left in server-lib docs is covered by the T6 issue text | DONE   | `manual-verification-evidence.md` section V2 |
| M3  | Search for duplicated instructions        | `grep` package READMEs and the skills for the doc's step commands                                                                                     | Only pointers found; no copied steps                                                                                                              | DONE   | `manual-verification-evidence.md` section V3 |
| M4  | Check old links after the split           | Open `docs/release_process.md#publishing-a-workspace-package` and the links in `AGENTS.md`, `docs/index.md` and both workflows on the rendered branch | Each lands on the index heading or the new document; no dead link                                                                                 | DONE   | `manual-verification-evidence.md` section V4 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain for each command result when it can affect behavior.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- Nothing is published, tagged, or pushed upstream during verification.

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC0   | DONE                   | T2 commit / M4 |
| AC1   | DONE                   | T3 commit |
| AC2   | DONE                   | T4 commit |
| AC3   | DONE                   | T4 commit |
| AC4   | DONE                   | M3: no README describes releasing; pointer text specified in the document |
| AC5   | DONE                   | T3 commit / `torrust-server-lib-followup-issue.md` |
| AC6   | DONE                   | M1 and M2 evidence |

## Risks and Trade-offs

- A link to `develop` can drift from what a package release actually needs; a permalink goes stale.
  Mitigation: link to `docs/release-process/workspace-packages.md` on `develop` (decided), so readers always
  see the current process.
- Splitting the document breaks links from outside this repository (for example
  `torrust-server-lib` docs). Mitigation: the index keeps the `## Publishing a Workspace Package`
  heading as a redirect, and T6 notes any external link worth updating.
- Backfilled tags on old commits can point at the wrong commit. Mitigation: the maintainer verifies
  each commit against crates.io publish metadata before pushing.
- One skill for two variants can become long. Mitigation: keep it a thin router to the doc.

## Implementation Completion Review

- Retrospective: `implementation-retrospective.md`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`.
- If none is needed, add a progress-log entry explaining why.

## References

- Blocked by: #2576 (package READMEs and crates.io metadata)
- Related issues: torrust/torrust-server-lib#5, torrust/torrust-server-lib#7
- Related PRs: torrust/torrust-server-lib#6, torrust/torrust-server-lib#8
- Related ADRs: `docs/adrs/20260629000000_adopt_independent_package_versioning.md`
- Process docs: `docs/release_process.md` (index after T2), `docs/release-process/tracker.md`,
  `docs/release-process/workspace-packages.md`
