---
schema-version: 1
doc-type: issue
issue-type: task
status: in-progress
priority: p2
epic: null
github-issue: 2691
spec-path: docs/issues/open/2691-reduce-agents-md-token-footprint/ISSUE.md
branch: "2691-reduce-agents-md-token-footprint"
related-pr: 2704
last-updated-utc: "2026-11-03 15:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - AGENTS.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/research.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/section-disposition.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/rule-inventory.md
---

<!-- skill-link: create-issue -->

# Issue #2691 - Research AGENTS.md Best Practices and Reduce Its Token Footprint

## Goal

Research published good practices for `AGENTS.md` (and equivalent agent instruction files), then
apply them to the repository's root `AGENTS.md` so it carries only the always-needed guidance and
consumes materially fewer tokens in every agent session.

## Background

`AGENTS.md` is loaded into the context of every agent session. At the time of drafting it is about
36 KB, 523 lines and 3,700 words, and Claude Code reports that it consumes too many tokens. Every
session pays that cost, including sessions where most of the content is irrelevant (for example a
documentation-only task that still loads the full package catalog, container instructions and
release branch strategy).

Observations from a first read of the file:

- It mixes always-needed rules (linting gate, GPG signing, branch naming) with reference material
  that is only needed for some tasks (the full package catalog and its three auxiliary tables,
  configuration file table, container volume mappings, documentation quick-navigation table).
- Several sections duplicate or restate content that already lives in skills, `docs/`, or other
  files (build/test commands, pre-commit step list, Git workflow detail, PR title rules). This also
  conflicts with the file's own Engineering Policy 6 (documentation single source of truth).
- The package catalog duplicates `cargo metadata --no-deps` and `docs/packages.md`, and is a
  maintenance burden because it drifts when packages are added or extracted.
- There is no `CLAUDE.md` in the repository, and none is needed: Claude Code accepts `AGENTS.md`
  as an automatic replacement for `CLAUDE.md` (confirmed by the maintainer). `AGENTS.md` therefore
  stays the single Git-tracked source of truth, consistent with ADR
  `20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`, and no
  provider-specific adapter is planned.

The repository's own rules constrain the solution: repository knowledge must stay reproducible from
Git-tracked documentation, and reducing `AGENTS.md` must not silently drop a rule that agents need
(for example the absolute GPG signing rule or the collaboration principles).

## Scope

### In Scope

- Research good practices for writing agent instruction files: the `AGENTS.md` specification and
  its guidance, Anthropic's Claude Code memory/`CLAUDE.md` guidance, GitHub Copilot custom
  instructions guidance, and any credible empirical studies or practitioner write-ups on
  instruction-file length, token cost and adherence. Record sources, dates and the practices adopted
  in an issue-local `research.md`.
- Measure the current token footprint and record a baseline (method and tool recorded so the
  measurement is reproducible, for example a tokenizer count and the Claude Code context report).
- Classify every section of `AGENTS.md` as: keep inline, move to a skill, move to `docs/`, replace
  with a link, replace with a command that produces the data, or delete. Record the rationale per
  section in an issue-local `section-disposition.md`.
- Apply the dispositions to `AGENTS.md`, moving content to its canonical home so that no rule is
  lost and nothing is duplicated.
- Keep `AGENTS.md` as the only root instruction file; do not add a `CLAUDE.md` adapter.
- Define a lightweight, repeatable guard against regrowth (for example a documented size budget,
  and, only if justified, an automated check).
- Update documentation, cross-references and `project-words.txt` affected by moved content.

### Out of Scope

- Rewriting the content of skills or `docs/` beyond receiving moved content and fixing links.
- Changing repository policies themselves (the goal is to relocate and condense, not to alter
  rules). Any proposed policy change is recorded as a follow-up issue.
- Changing `.github/agents/*.agent.md` files, unless a section moved out of `AGENTS.md` must be
  referenced from them.
- Provider-specific tuning that would make a single vendor's tooling mandatory.

## Architectural Decisions

- Related ADRs:
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- ADRs to create: None known. The maintainer decided no ADR is needed for now. Re-evaluate after
  implementation whether the `AGENTS.md` role and size budget
  warrant one (placement would be root `docs/adrs/`, as the decision is repository-wide).

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                                  | Notes / Expected Output                                                                                 |
| --- | ------ | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Measure the baseline token footprint of `AGENTS.md`                                   | Method, tool and numbers recorded in `research.md` (baseline section)                                   |
| T2  | DONE   | Research best practices for agent instruction files                                   | `research.md` with sources, dates, and a short list of adopted practices with the evidence behind each  |
| T3  | DONE   | Record how the major tools consume `AGENTS.md` (Claude Code accepts it as is)         | Short section in `research.md`; confirms no adapter file is needed                                      |
| T4  | DONE   | Classify each `AGENTS.md` section and record the disposition                          | `section-disposition.md` covering 100% of sections, including the destination for every moved rule      |
| T4a | DONE   | Build the rule inventory from the baseline `AGENTS.md`                                | `rule-inventory.md`: one row per rule, seeded from the source file, not from the planned moves          |
| T5  | DONE   | Maintainer review of research and dispositions before any restructuring               | Explicit maintainer approval recorded in the progress log                                               |
| T6  | TODO   | Move content to canonical homes (skills/docs) and replace it with links where needed  | No rule lost, no duplicated procedural guidance; links validated                                        |
| T7  | DONE   | Condense `AGENTS.md`                                                                    | Reduced file; footprint re-measured with the T1 method                                                  |
| T8  | TODO   | Add a regrowth guard (documented budget; automated check only if justified)           | Documented in the owning skill or doc; any tool follows the shell-vs-Rust policy                        |
| T9  | TODO   | Behavioral check that agents still follow the key rules after the reduction           | See manual scenarios M1 to M3                                                                           |
| T10 | TODO   | Re-evaluate after implementation whether an ADR is warranted                          | Outcome recorded in the progress log and in the implementation completion review                       |

## Commit Points

| Task | Coherent change set                                          | Commit policy                                        |
| ---- | ------------------------------------------------------------ | ---------------------------------------------------- |
| T1   | Baseline measurement evidence                                | Commit after focused validation and required review. |
| T2   | `research.md` with sources and adopted practices             | Commit after focused validation and required review. |
| T3   | Tool-consumption findings in `research.md`                   | Commit after focused validation and required review. |
| T4   | `section-disposition.md`                                     | Commit after focused validation and required review. |
| T4a  | `rule-inventory.md`                                          | Commit after focused validation and required review. |
| T6   | One commit per destination (skill or doc) receiving content | Commit after focused validation and required review. |
| T7   | Condensed `AGENTS.md`                                          | Commit after focused validation and required review. |
| T8   | Regrowth guard                                               | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. Use
a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2691-reduce-agents-md-token-footprint/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-30 13:45 UTC - Claude Code (Claude Sonnet 5.5) - Drafted the specification from the maintainer's request - this file
- 2026-10-30 13:55 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the scope and decided no ADR is needed for now; removed the ADR task and added a post-implementation re-evaluation (T10) - this file
- 2026-10-30 14:05 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer confirmed Claude Code accepts `AGENTS.md` in place of `CLAUDE.md`; removed the adapter work from the scope, T3, T7 and AC5 - this file
- 2026-10-30 14:15 UTC - Claude Code (Claude Sonnet 5.5) - Maintainer approved the draft; created GitHub issue #2691, chose a spec-only PR, and moved the spec to `docs/issues/open/` - https://github.com/torrust/torrust-tracker/issues/2691
- 2026-10-30 15:00 UTC - Claude Code (Claude Sonnet 5.5) - Completed T1 to T3: baseline measured (about 7.5k tokens by proxy tokenizers), research and adopted practices recorded, confirmed `AGENTS.md` needs no adapter; target reduction still open - `research.md`
- 2026-10-30 15:30 UTC - Claude Code (Claude Sonnet 5.5) - Recorded the maintainer's `/context all` baseline: `AGENTS.md` is 12.4k Claude tokens, about 1.65 times the proxy tokenizer count, and about 22% of the session context - `research.md`
- 2026-10-30 16:00 UTC - Claude Code (Claude Sonnet 5.5) - Completed T4: proposed a disposition for all 17 sections, estimated about 4.1k Claude tokens (about 67% reduction, about 60% if trimmed conservatively), listed prerequisites for T6 and five open questions; waiting for maintainer review (T5) - `section-disposition.md`
- 2026-10-30 16:30 UTC - Claude Code (Claude Sonnet 5.5) - T5: maintainer reviewed the dispositions and decided D1 to D5 (target about 5k Claude tokens, skill/nested-`AGENTS.md` placement rule, catalog in `docs/packages.md`, keep Development Principles, keep all eight policies inline); estimate now about 4.5k Claude tokens - `section-disposition.md`
- 2026-10-30 17:00 UTC - Claude Code (Claude Sonnet 5.5) - T6 step 1: moved the Package Catalog to `docs/packages.md` (23 packages and crate names verified one to one, tool tables carried over) and reversed the back-links in `packages/AGENTS.md`; `AGENTS.md` keeps a pointer and the `cargo metadata` command - `section-disposition.md`
- 2026-11-03 09:30 UTC - Claude Code (Claude Sonnet 5.5) - Added T4a after an outside comment on the issue noted that a checklist seeded from the planned moves can only confirm those moves: built `rule-inventory.md` with 99 rows from the baseline file at the commit `Merge torrust/torrust-tracker#2692: docs(issues): [#2691] add specification to reduce the AGENTS.md token footprint`; all 99 locator phrases verified present in the baseline; 9 rows already asserted (catalog) - `rule-inventory.md`
- 2026-11-03 10:15 UTC - Claude Code (Claude Sonnet 5.5) - T6 step 2: replaced the Package Naming Conventions and Container / Docker sections with links (`docs/packages.md#package-conventions`, `docs/containers.md`, destination content verified first) and asserted the 4 matching inventory rows - `rule-inventory.md`
- 2026-11-03 11:00 UTC - Claude Code (Claude Sonnet 5.5) - T6 step 3: replaced the Key Configuration Files and Build, Test, and Lint content with short pointers, after confirming that the `run-linters`, `run-pre-commit-checks`, `run-pre-push-checks` skills and `docs/benchmarking.md` already hold the removed content; corrected the `run-linters` ignore note; found and dropped three stale baseline statements (`cargo test --test integration`, `.cspell.gitignore`, four of five cargo aliases); asserted 16 inventory rows - `section-disposition.md`, `rule-inventory.md`
- 2026-11-03 13:00 UTC - Claude Code (Claude Sonnet 5.5) - T6 step 4: moved the Git Workflow detail to the `open-pull-request` skill. The skill pointed back to `AGENTS.md` for the history-rewrite rules and held only a stub of the PR-title rules, so I added both to the skill first (new section Commit History and Pull-Request Scope, extended Title section) and fixed the circular reference; `AGENTS.md` keeps branch naming, the Conventional Commits form, the branch strategy and the keep-intermediate-commits rule; asserted 14 inventory rows; `AGENTS.md` is now about 4.1k proxy tokens - `rule-inventory.md`
- 2026-11-03 15:00 UTC - Claude Code (Claude Sonnet 5.5) - T7: the maintainer approved the remaining trims including shortened policies (decision D6). Condensed Project Overview, Key Directories, Code Style, Engineering Policies, Essential Rules, Auto-Invoke Skills and Documentation, and removed Tech Stack; moved the VS Code skills setting to the `setup-dev-environment` skill first; removed one duplicated rule and renumbered Essential Rules; found one more stale baseline statement (`tests/` contents). `AGENTS.md` is now 2,994 proxy tokens (about 4.9k Claude tokens, estimated), 229 lines, 13,027 bytes. All 99 inventory rows are asserted; 45 `AGENTS.md` rows were checked mechanically and 4 reworded rows by reading - `section-disposition.md`, `rule-inventory.md`

## Acceptance Criteria

- [ ] AC1: `research.md` documents the sources consulted and the practices adopted, each tied to
      evidence, and states which practices were deliberately not adopted and why.
- [ ] AC2: A baseline and a final token measurement exist, taken with the same documented method,
      and the final footprint of `AGENTS.md` is about 5k Claude Code tokens or fewer in `/context`
      (target agreed with the maintainer in T5, decision D1; baseline 12.4k).
- [ ] AC3: Every section of the original `AGENTS.md` has a recorded disposition, and every rule that
      was moved is reachable from `AGENTS.md` by link or skill reference.
- [ ] AC7: Every row of `rule-inventory.md`, built from the baseline file before restructuring,
      has status `ASSERTED`: its final location was opened and confirmed, or a `DELETE` has a
      recorded reason.
- [ ] AC4: No procedural guidance is duplicated between `AGENTS.md` and a skill or `docs/` page
      (Engineering Policy 6).
- [ ] AC5: `AGENTS.md` remains the sole source of truth and no provider-specific adapter file is
      introduced (Engineering Policy 7).
- [ ] AC6: A regrowth guard is documented in the owning skill or doc.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `./contrib/dev-tools/git/hooks/pre-commit.sh`
- Link and frontmatter validation of every moved or edited document
- Pre-push checks when applicable

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                              | Human-oriented command/steps                                                                                                  | Expected Result                                                                  | Status | Evidence                                     |
| --- | ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | Token footprint before and after      | Measure `AGENTS.md` with the T1 method on the original commit and on the final commit                       | Final footprint is lower by the agreed target                                    | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | Key rules survive in a fresh session  | Start a fresh Claude Code session and ask it to describe the commit rules, branch naming and linting gate                     | Answers match the original rules, including GPG signing and `linter all`         | TODO   | `manual-verification-evidence.md` section V2 |
| M4  | Rule inventory is fully asserted      | For every row of `rule-inventory.md`, open the final location, confirm it holds the rule, and set the row to `ASSERTED`; list any `DELETE` with its reason | No row is `PENDING`; no rule is unreachable | TODO   | `manual-verification-evidence.md` section V4 |
| M3  | Moved content is discoverable         | In a fresh session, ask for the package list and the container ports, and observe where the agent looks                       | The agent finds the canonical source through links or skills                     | TODO   | `manual-verification-evidence.md` section V3 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Record the toolchain or runtime that produced each result when it can affect behavior, and record
  which agent and model ran M2 and M3, since adherence varies by model.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  when executing these scenarios.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. If a token-counting script is needed, record its issue-local path, why a maintained
Rust test is not suitable, and its removal or retention owner here before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence           |
| ----- | ---------------------- | ------------------ |
| AC1   | TODO                   | {test/log/PR link} |
| AC2   | TODO                   | {test/log/PR link} |
| AC3   | TODO                   | {test/log/PR link} |
| AC4   | TODO                   | {test/log/PR link} |
| AC5   | TODO                   | {test/log/PR link} |
| AC6   | TODO                   | {test/log/PR link} |
| AC7   | TODO                   | {test/log/PR link} |

## Risks and Trade-offs

- **Rules silently lost during the move.** Mitigation: the per-section disposition table (AC3) and
  maintainer review before restructuring (T5).
- **Agents stop following a rule that is now one link away.** Always-needed, high-consequence rules
  (GPG signing, linting gate, security reporting) stay inline; M2 checks adherence.
- **Over-trimming makes agents spend more tokens searching for what they need.** The research must
  weigh always-loaded cost against on-demand lookup cost, not just minimize the file size.
- **Single-vendor optimization.** Practices from one provider's guidance may not apply to others;
  the research records the provenance of each adopted practice.
- **Regrowth.** Without a guard the file grows again; T8 addresses this.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`.

## References

- Related issues: none established. EPIC #2003 (AI agent guardrails) may be the parent; the
  maintainer should confirm before the draft is renamed with an EPIC prefix.
- Related ADRs:
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- AGENTS.md specification: <https://agents.md>
