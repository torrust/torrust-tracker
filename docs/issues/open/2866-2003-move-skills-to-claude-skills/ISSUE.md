---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p1
epic: 2003
github-issue: 2866
spec-path: docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md
branch: "2866-move-skills-to-claude-skills-spec"
related-pr: null
last-updated-utc: "2026-11-15 20:55"
semantic-links:
  skill-links:
    - create-issue
    - add-new-skill
    - cleanup-completed-issues
  related-artifacts:
    - .github/skills
    - AGENTS.md
    - docs/agents/README.md
    - docs/skills/semantic-skill-link-convention.md
    - contrib/dev-tools/checks/frontmatter-validator
    - contrib/dev-tools/checks/agent-review-report-contract
    - lychee.toml
    - docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #2866 - Move the agent skills to `.claude/skills/` so Claude Code discovers them

Parent EPIC: #2003 - Overhaul: Automation Tools and AI Agent Guardrails (order 21).

## Goal

Move the repository's 53 agent skills from `.github/skills/<area>/<name>/` to a flat
`.claude/skills/<name>/`, so Claude Code discovers and loads them without a brief having to name
each one. Keep every live reference, tool and validator working, and keep old paths in historical
records resolvable.

## Background

Claude Code is the repository's only AI harness. It discovers project skills only at
`.claude/skills/<skill-name>/SKILL.md`. Today the skills live under `.github/skills/dev/<area>/`
(plus `.github/skills/add-new-skill/` and `.github/skills/usage/`), a layout chosen for GitHub
Copilot (ADR `20260420200013`). Claude Code never sees them, so an agent follows a skill only when
its brief names the file, and steps get skipped.

The bug in plain terms:

1. A session starts at the repository root. Claude Code looks for project skills in
   `.claude/skills/` and finds none, so no repository skill is listed to the model.
2. A contributor types `/create-issue`, or asks for work a skill covers. The command is unknown,
   and the model does not know the skill exists unless `AGENTS.md`, a brief, or a search points it
   at the file.
3. The model then works from memory or a partial read, and steps the skill requires are skipped.

This breaks the expectation `AGENTS.md` sets, that skills activate when their workflows are needed.
It is observable today: the reproduction in `manual-verification-evidence.md` (V0) shows no project
skill in a fresh clone of `develop`.

The custom-agents migration (a later EPIC #2003 sub-issue, order 22) waits for this issue: its
agent definitions name skills, and they must name them at their final place.

### Decisions already made

The maintainer decided on 2026-11-15:

- The skills must be discoverable by Claude Code. Layout: flat, `.claude/skills/<name>/SKILL.md`.
  The grouping by area that the folders give today moves to an index,
  `.claude/skills/README.md`, that lists the skills by area.
- Claude Code is the only harness; GitHub Copilot compatibility is not kept. Open standards are
  preferred where one exists: `AGENTS.md` stays, and there is no repository `CLAUDE.md`.
- Rejected: one symlink per skill (a second place to keep in sync, fragile on Windows) and one
  plugin per area (an install step for every contributor).
- `.github/agents/*.agent.md` are not refactored here. Stale skill paths inside them are accepted,
  because the custom-agents migration removes those files. The only exception is the smallest
  change that keeps a repository check green.
- Historical records are not rewritten. Instead, each old `SKILL.md` path keeps a short redirect
  stub that points to the new location (see [Redirect stubs](#redirect-stubs)).
- Descriptions are shortened in a follow-up sub-issue, not here (see
  [Context cost](#context-cost-of-the-skill-listing)).
- The implementation is one PR with staged commits.

### Pre-spec checks

Both checks ran on 2026-11-15 with Claude Code 2.1.318.

#### A folder level between `skills/` and the skill folder is not discovered

In a scratch git repository with `.claude/skills/flat-probe/SKILL.md` and
`.claude/skills/dev/nested-probe/SKILL.md`:

- Asked to list its `*-probe` skills, Claude Code listed only `flat-probe`.
- `/flat-probe` ran. `/nested-probe` and `/dev:nested-probe` were both unknown commands.

The Claude Code skills documentation ("Choose where skills load") describes
`.claude/skills/<skill-name>/SKILL.md` and nested `<subdir>/.claude/skills/` folders for
monorepos, and no category level. The flat layout is therefore required, not only preferred.

#### Context cost of the skill listing

Once discovered, each skill's name and description is loaded into every session. The
documentation says the listing budget is 1% of the model's context window, every name is always
kept, and the descriptions of the least-invoked skills are dropped first; each entry is capped at
1,536 characters.

`/context` in a scratch repository holding the 53 `SKILL.md` files, flat:

| Measurement                                          | Value                   |
| ---------------------------------------------------- | ----------------------- |
| Context window reported (Sonnet 5.5 and Haiku 5.5)   | 1M tokens               |
| Skills category, no project skills                   | 4.6k tokens             |
| Skills category, with the 53 project skills          | 9.9k tokens (1.0%, cap) |
| The 53 project skill entries alone (sum of rows)     | ~6,980 tokens           |
| Built-in skills whose descriptions were dropped      | 9                       |
| Description length: total / mean / max (characters)  | 19,489 / 368 / 724      |
| Descriptions of 300 characters or more               | 37 of 53                |

The listing already exceeds the budget on a 1M window. On a 200k window the budget is about 2k
tokens, and most descriptions would be dropped. Descriptions need shortening; that is the
follow-up sub-issue, so this move stays a reviewable rename.

### Reference inventory

On `torrust/develop` at `abd7a10ff`: 53 `SKILL.md` files and 71 files in total under
`.github/skills/` (scripts, references, and the `wait-for-pr-review` tool crate move too). All 53
names are unique. 663 files outside the folder contain `.github/skills`:

| Area                                         | Files | Kind                                    |
| -------------------------------------------- | ----- | --------------------------------------- |
| `docs/pr-reviews/` (records)                 | 320   | Historical; frontmatter and plain text  |
| `docs/issues/closed/`                        | 199   | Historical                              |
| `docs/issues/open/`                          | 55    | Live                                    |
| `docs/adrs/`                                 | 19    | Live navigation in decision records     |
| `docs/issues/drafts/`                        | 18    | Live                                    |
| `docs/templates/`                            | 13    | Live                                    |
| `contrib/`                                   | 6     | Tooling with the path built in          |
| `.github/` (agents, workflows `AGENTS.md`)   | 6     | Agents: out of scope; workflows: live   |
| Other `docs/`, `packages/`, root files       | 27    | Mixed; classified during implementation |

Tooling with the path built in: the `Cargo.toml` workspace member
(`.github/skills/dev/pr-reviews/wait-for-pr-review/tool`), `.dockerignore`, the
`frontmatter-validator` (`SKILLS_DIRECTORY`, tests, README), `agent-review-report-contract`
(about 40 literal paths), the `include_str!` of `fetch-review-threads/SKILL.md` in
`github-review-threads`, and `docs/packages.md`. 32 skill files hold 96 relative-link lines whose
depth changes (five levels deep today, three after). One moved script also finds the repository
root by depth: `run-tracker-locally/scripts/validate-skill-links.sh` walks six folders up, and its
`scripts/` folder will be four levels below the root. No other moved script, test, or manifest
resolves a path by depth (searched for `..` path walks, `dirname`, `__file__` and manifest paths).

`skill-links` frontmatter holds skill names, not paths, so it does not change; only the
validator's lookup folder does.

## Scope

### In Scope

- `git mv` of every file under `.github/skills/` to `.claude/skills/<name>/`, keeping each
  skill's `scripts/`, `references/` and `tool/` folders.
- Every live reference to the old paths: `AGENTS.md`, `docs/AGENTS.md`, `docs/agents/`, open and
  draft specs, ADR links and `related-artifacts`, templates, folder READMEs, package docs, the
  skills themselves, and `.github/workflows/AGENTS.md`.
- Tooling with the path built in (listed above).
- Redirect stubs at the 53 old `SKILL.md` paths, and a check that keeps them correct.
- The index `.claude/skills/README.md`, listing every skill by area, and a check that it lists
  exactly the skills on disk.
- Validator rules for the new layout (see [Validator rules](#validator-rules-after-the-move)).
- The `add-new-skill` skill and its reference: where a new skill goes, and the index row it needs.
- A new ADR that records the move, and a dated row in the governance ADR's capability inventory.
- Creating the follow-up sub-issue that shortens the descriptions.

### Out of Scope

- `.github/agents/*.agent.md`, apart from the smallest change a failing check requires.
- Rewriting historical records: `docs/issues/closed/`, the records in `docs/pr-reviews/` and
  `docs/copilot-pr-reviews/`, `docs/refactor-plans/closed/`, runner reports, and discussion
  rounds. Their plain-text paths and frontmatter stay as written; the stubs keep them resolvable.
- Shortening or retuning skill descriptions (follow-up sub-issue).
- Renaming any skill or changing what a skill says, beyond its paths and links.
- Claude Code settings, hooks, or permissions for skills (`allowed-tools`, `skillOverrides`).

## Design

### Classification rule for references

- A Markdown link is live navigation wherever the link checker checks it. Rewrite it to the new
  path.
- A plain-text path or a frontmatter `related-artifacts` entry in a historical record is history.
  Leave it; the redirect stub resolves it.
- Everything in a live document (open and draft specs, ADRs, templates, READMEs, skills, tooling)
  is rewritten.

The implementation records, in the spec's evidence, the exact commands that classified and
rewrote the references, so a reviewer can reproduce the sweep.

### Redirect stubs

Each old path `.github/skills/<area>/<name>/SKILL.md` (and `add-new-skill/SKILL.md`) keeps a stub:

```markdown
# Moved: `<name>`

This skill moved to [`.claude/skills/<name>/SKILL.md`](<relative path>) in issue #2866.
```

- A stub has no frontmatter, so no tool treats it as a skill.
- Only `SKILL.md` gets a stub. Old command lines in records, such as
  `python3 .github/skills/.../validate-audit-record.py`, stay as history and are not redirected.
- Stubs are added in a commit after the move. A stub at the old path in the same commit would
  turn the rename into an edit plus an addition and break `git log --follow`.

### Validator rules after the move

- Every `SKILL.md` that is not an inventoried stub is at `.claude/skills/<name>/SKILL.md`; one
  elsewhere, for example a new skill added under `.github/skills/`, is an error.
- `frontmatter-validator` resolves `skill-links` names against `.claude/skills/<name>/SKILL.md`
  only. A `SKILL.md` deeper than one folder under `.claude/skills/` is an error, because Claude
  Code would not discover it.
- A skill's `name` equals its folder name.
- `.github/skills/README.md` keeps the inventory of the 53 original `SKILL.md` paths, each with
  its skill name. The stub files and the inventory match exactly: every inventoried path has a
  stub, and no stub is missing from the inventory. Deleting only a stub therefore fails. A skill
  added after the move is not in the inventory and needs no stub.
- Inventory rows and stubs are never deleted. When an inventoried skill is later renamed, its stub
  links to the new name; when it is removed, its stub says so and links to the skill's last
  version on `develop` or to its replacement, and its inventory row records the removal. The
  validator accepts a stub marked removed without a link to an existing skill.
- Every stub that is not marked removed has no frontmatter and exactly one link, which resolves to
  an existing `.claude/skills/<name>/SKILL.md` with the stub's folder name, or with the new name
  its inventory row records. No other file exists under
  `.github/skills/` apart from the stubs and their README.
- `.claude/skills/README.md` lists every skill folder exactly once and names no missing skill.
- These repository-wide rules run on every commit and in CI, not only on staged files, so
  deleting or renaming a skill without its stub or index row fails. They live in the
  `frontmatter-validator` crate, which already snapshots the skill folders; the exact entry point
  is settled in T6 and recorded here.
- `agent-review-report-contract` and `github-review-threads` read the new paths.
- The link checker needs no new exclusion: live links are rewritten and old links reach a stub.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`
  (placed the skills under `.github/skills/`) and
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
  (capability inventory, with an event-driven review).
- ADRs to create: "Store agent skills flat in `.claude/skills/`", root collection
  (`docs/adrs/`), because it is repository-wide. It records the decisions above, the rejected
  options, and the nested-folder evidence, and states that it supersedes the skills location in
  ADR `20260420200013`. The governance ADR gets a dated inventory row for the move.

## Design and Ownership Review

Not applicable: no child processes, asynchronous I/O, readiness, or fixtures.

## Bug-Fix Process

The work fixes broken behavior: Claude Code does not discover the repository's skills, so agents
skip their steps. It follows the `fix-bug` skill
(`.github/skills/dev/debugging/fix-bug/SKILL.md`, at `.claude/skills/fix-bug/SKILL.md` after the
move):

- **Analysis and hypothesis.** Claude Code reads project skills only from
  `.claude/skills/<name>/SKILL.md`; the skills sit under `.github/skills/`, and one folder deeper
  is not read either ([Pre-spec checks](#pre-spec-checks)).
- **Real-artifact reproduction.** Done while drafting this spec (M0, outcome **Reproduced**): in a
  fresh clone of `develop` at `abd7a10ff`, `claude -p "/context"` listed no `Project` skill and
  `claude -p "/create-issue"` did not run as a command. The scratch probes in the pre-spec checks
  are design evidence, not this reproduction.
- **Regression-test boundary.** See [Regression Test Strategy](#regression-test-strategy).
- **Red evidence (T2).** The placement rule, run against the tree before the move, fails and names
  the 53 skills outside `.claude/skills/`.
- **Fix.** T3 to T8.
- **Green evidence and final recheck (T9).** The placement rule passes on the moved tree, and M1
  and M2 repeat M0's commands, like for like, on the branch.

All results go to issue-local `manual-verification-evidence.md`.

## Regression Test Strategy

Whether Claude Code discovers a skill depends on the Claude Code program and a model, so it cannot
be a maintained deterministic test; M0, M1 and M2 cover it by hand. The smallest deterministic
boundary that fails for the defect is the repository layout itself: a placement rule (T2) that every
`SKILL.md` that is not an inventoried stub is at `.claude/skills/<name>/SKILL.md`, flat. Its unit
tests cover a skill under `.github/skills/` and a skill one folder too deep under
`.claude/skills/`; it runs on every commit and in CI, so the defect cannot come back unnoticed.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                     | Notes / Expected Output                                                                                                                                                                                                              |
| --- | ------ | ---------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| T1  | TODO   | Create the description follow-up issue   | A new EPIC #2003 sub-issue to shorten and retune the 53 descriptions, with the measurements above. Created before the implementation PR opens, so the PR links it.                                                                     |
| T2  | TODO   | Placement rule, proven red               | The regression test: the placement rule of [Validator rules](#validator-rules-after-the-move) with its unit tests, not yet wired into the hook. Its run against the tree before the move is recorded red in `manual-verification-evidence.md`. |
| T3  | TODO   | Move the skills and fix every live path  | `git mv` of all 71 files; relative links inside moved skills; depth-dependent paths in moved scripts (`validate-skill-links.sh`); tooling paths; every live reference by the classification rule. The commit passes `linter all` and the tests on its own. Git shows 71 renames.                           |
| T4  | TODO   | Add the redirect stubs                   | 53 stubs plus `.github/skills/README.md`, which says the folder only holds redirects and inventories the 53 original paths.                                                                                                                                                       |
| T5  | TODO   | Add the index                            | `.claude/skills/README.md` lists each skill by area (today's area folders), with a one-line purpose, and notes the old location. `AGENTS.md` and `docs/agents/` point to it.                                                          |
| T6  | TODO   | Remaining validator rules                | The other rules in [Validator rules](#validator-rules-after-the-move), with unit tests and a negative case for each; the placement rule and these rules are wired into the pre-commit hook and CI, and the placement rule's green run is recorded. Tests follow the `write-unit-test` skill and the test-design review in [Commit Points](#commit-points).                                                                                  |
| T7  | TODO   | Update `add-new-skill`                   | New skills go to `.claude/skills/<name>/`, add an index row, and need no stub.                                                                                                                                                         |
| T8  | TODO   | ADRs                                     | New ADR; dated row in the governance ADR's inventory and review log.                                                                                                                                                                   |
| T9  | TODO   | Final sweep and verification             | Search again for `.github/skills` and `skills/dev/` after the last edit; every remaining hit is a stub, out-of-scope agent profile, or historical record. Run every validator, `linter all`, and the manual scenarios.                  |

## Commit Points

| Task | Coherent change set                                         | Commit policy                                                                                       |
| ---- | ----------------------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| T2   | Placement rule and its unit tests                           | One commit per reviewed test increment, as for T6; the red run is recorded in the evidence, not committed as a failing check. |
| T3   | The move plus every path fix the gate needs                 | One commit. Its message records the sweep commands. Moved files change only in their links and in depth-dependent paths such as the root walk of `validate-skill-links.sh`. |
| T4   | Stubs                                                       | One commit, after T3, so the renames stay detectable.                                               |
| T5   | Index and its links                                         | One commit.                                                                                         |
| T6   | Remaining validator rules and tests                         | One commit per reviewed test increment (one rule group each). After each increment passes, record the prose-first Arrange-Act-Assert review in the task evidence before starting the next. Stop for maintainer review after the last increment, before T9. |
| T7   | `add-new-skill`                                             | One commit.                                                                                         |
| T8   | ADRs                                                        | One commit.                                                                                         |
| T9   | Evidence, acceptance verification, and progress log         | One commit.                                                                                         |

T1 creates a GitHub issue and its spec PR; it is not part of this implementation PR's commits.

T2 and T6 produce tests. Their tests use the `write-unit-test` skill and add one behavior at
a time. After each increment passes focused validation, and before the next one starts, review its
design and record the review in the task evidence: write temporary prose for the Arrange, Act and
Assert sections, refactor until the code says it, and remove the redundant prose. The review
confirms that the test exposes the one causal initial-state difference, that its fixture owns only
incidental mechanics, and that the production Act and the independently specified expected result
stay visible. Helper boundaries are judged by meaningful named actions and abstraction level, not
by caller count. Commit each reviewed increment before the next, and stop for maintainer review
after the last increment, before the final verification (T9), the commit of the evidence, and the
pull request.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-move-skills-to-claude-skills/ISSUE.md` (now `docs/issues/open/2866-2003-move-skills-to-claude-skills/ISSUE.md`)
- [x] Spec reviewed and approved by the maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded in `implementation-retrospective.md`
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-15 12:50 UTC - Claude Code (Claude Opus 5.5) - Drafted the spec after the nested-folder and context-cost checks and the maintainer's design decisions - this spec, [Pre-spec checks](#pre-spec-checks)
- 2026-11-15 20:14 UTC - Claude Code (Claude Opus 5.5) - Maintainer approved the draft; created issue #2866, moved the spec to `docs/issues/open/`, and added the EPIC #2003 row at order 21. Linking #2866 as a GitHub sub-issue of #2003 returned HTTP 404 for the contributor account, so the maintainer links it - https://github.com/torrust/torrust-tracker/issues/2866
- 2026-11-15 20:48 UTC - Claude Code (Claude Opus 5.5) - Reproduced the defect in a fresh clone of `develop` at `abd7a10ff` (M0): no project skill listed, `/create-issue` not run as a command - `manual-verification-evidence.md` V0

## Acceptance Criteria

- [ ] AC1: No `SKILL.md` remains under `.github/skills/` except the 53 redirect stubs, and every
  skill is at `.claude/skills/<name>/SKILL.md`, flat.
- [ ] AC2: A new Claude Code session at the repository root lists all 53 project skills.
- [ ] AC3: Git records the move as renames (`git log --follow` on a moved skill reaches its
  history before the move).
- [ ] AC4: Every live reference names the new path; every remaining `.github/skills` hit is a
  stub, an out-of-scope agent profile, or a historical record.
- [ ] AC5: Every old `SKILL.md` path resolves to a stub that links to the moved skill, and the
  validator fails when a stub, the stub inventory, the index, or a skill folder disagree,
  including when only a stub is deleted.
- [ ] AC6: `.claude/skills/README.md` lists every skill by area.
- [ ] AC7: The moved scripts and the `wait-for-pr-review` tool run from their new paths.
- [ ] AC8: The new ADR exists and the governance ADR's inventory records the move.
- [ ] AC9: The description follow-up sub-issue exists and is linked from the EPIC.
- [ ] AC10: The placement rule fails on the tree before the move (recorded red, T2) and passes on
  the moved tree, with unit tests for a skill under `.github/skills/` and a skill one folder too
  deep under `.claude/skills/`.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo run --package frontmatter-validator --bin frontmatter-validator -- --staged` on each
  commit, and the new repository-wide skill rules
- `cargo test` for `frontmatter-validator`, `agent-review-report-contract`,
  `github-review-threads`, and `wait-for-pr-review`
- `python3 -m unittest discover -s .claude/skills/process-pr-review/scripts`
- `./contrib/dev-tools/git/hooks/pre-commit.sh` and the pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                          | Human-oriented command/steps                                                                                                       | Expected Result                                                               | Status | Evidence                                     |
| --- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M0  | Reproduce the defect before T3    | In a fresh clone of `develop` (done at `abd7a10ff`), run `claude -p "/context"` and `claude -p "/create-issue"` at the root         | No `Project` skill row; `/create-issue` is an unknown command                  | DONE   | `manual-verification-evidence.md` section V0 |
| M1  | Claude Code discovers the skills  | In a fresh clone of the branch, run M0's `claude -p "/context"` and count the `Project` rows                                       | 53 project skills; no stub listed                                              | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | A skill runs by name              | In the same clone, run M0's `claude -p "/create-issue"`                                                                          | Claude Code loads `.claude/skills/create-issue/SKILL.md`                      | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | Moved scripts and tool run        | Run `validate-audit-record.py` on a merged audit record, `bash .claude/skills/run-tracker-locally/scripts/validate-skill-links.sh`, and `cargo run -p wait-for-pr-review -- --help` from the new paths | All three succeed; the skill-links script prints `Skill links validation passed` | TODO   | `manual-verification-evidence.md` section V3 |
| M4  | An old link still leads somewhere | Open a closed spec that links an old `SKILL.md` path and follow the link on the branch                                             | The stub opens and its link reaches the moved skill                           | TODO   | `manual-verification-evidence.md` section V4 |
| M5  | The validator catches drift       | In scratch commits, delete one skill folder, then (separately) delete only one stub, and run the new rules each time               | Each run fails: the first names the orphan stub and index row, the second the missing inventoried stub | TODO   | `manual-verification-evidence.md` section V5 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |

## Risks and Trade-offs

- **A large first commit.** T3 must pass the commit gate on its own, so it carries the move and
  every path fix. Mitigation: the moved files change only in their links and depth-dependent paths, the sweep commands are
  recorded, and `git show -M --stat` shows the renames.
- **53 stubs with no planned end.** They keep about 520 historical records navigable without
  rewriting them. The T6 rules keep them from going stale.
- **Descriptions stay long until the follow-up.** Claude Code drops the least-used descriptions
  until then; the names stay listed.
- **Out-of-scope agent profiles keep old paths** until the custom-agents migration removes them;
  the stubs keep those paths resolvable.
- **Places outside the repository** (for example the maintainer's own notes and other
  repositories) may name the old paths. The implementation lists those it finds in the close-out
  report; it does not change them.

## Implementation Completion Review

- Retrospective: required. Create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md`, covering the sweep, the review rounds, and
  any reference the first sweep missed.
- When an independent reviewer receives this folder-style specification, it records its result
  in `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Parent EPIC: #2003 (`docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md`)
- Claude Code skills documentation: https://code.claude.com/docs/en/skills
- Related ADRs: `docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`,
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
