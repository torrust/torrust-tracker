---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p1
epic: 2003
github-issue: 2904
spec-path: docs/issues/open/2904-2003-migrate-custom-agents-to-claude-code/ISSUE.md
branch: "2904-migrate-custom-agents-to-claude-code-spec"
related-pr: null
last-updated-utc: "2026-11-17 13:11"
semantic-links:
  skill-links:
    - create-issue
    - process-pr-review
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/process-pr-review/SKILL.md
    - .claude/skills/wait-for-pr-review/SKILL.md
    - .claude/skills/review-task/SKILL.md
    - .claude/skills/review-pr/SKILL.md
    - .claude/skills/commit-changes/SKILL.md
    - .claude/skills/fix-clippy-warnings/SKILL.md
    - .github/agents
    - .github/agents/README.md
    - AGENTS.md
    - docs/agents/README.md
    - docs/agents/orchestration.md
    - docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md
    - docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md
    - contrib/dev-tools/checks/agent-review-report-contract/src/main.rs
    - docs/templates/ISSUE.md
    - docs/issues/open/2003-overhaul-guardrails-and-automation/EPIC.md
    - docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2904 - Migrate the custom agents to Claude Code and add an explicit orchestrator

**Parent EPIC:** #2003 - Overhaul: Automation Tools and AI Agent Guardrails

## Goal

Replace the ten GitHub Copilot custom agent profiles in `.github/agents/` with a smaller set of
Claude Code custom agents in `.claude/agents/`, led by an explicit orchestrator agent, so the roles
can be launched again and a role's model can be fixed in its definition.

The first agent delivered is the one that addresses pull-request reviews, with its model pinned, so
the number of review rounds per pull request can be measured.

## Background

### Why migrate

The maintainer moved from GitHub Copilot to Claude Code. His reasons (2026-11-15):

- the Claude Code tools are more mature, and the work costs less under its subscription;
- Claude Code is the more widely used of the two, and the other maintainer uses it too;
- he is the only active contributor today, and new contributors are more likely to use Claude Code;
- keeping two AI harnesses compatible is too much work for one person.

Claude Code reads custom agents only from `.claude/agents/` (project) and `~/.claude/agents/`
(user). It does not read `.github/agents/*.agent.md`. So since the move, none of the ten profiles
has been available, and the handoffs drawn in `docs/agents/orchestration.md` have not run.

### What the ten profiles did, and what covers it today

| Profile | What it did | Covered today by | Still happening? |
| --- | --- | --- | --- |
| `copilot-suggestions-handler` | Entry point that processes Copilot review findings through `process-pr-review` | `process-pr-review` skill (all of it) | Yes, by the main session on whatever model it runs |
| `task-reviewer` | Independent check of the acceptance criteria before a pull request; writes `agent-review-reports.md` | `review-task` skill (checklist); the persistence rules are only in the profile | No |
| `pr-reviewer` | Local review of an open pull request; `POST_MERGE_FINDINGS` for a merged one; persists a report | `review-pr` skill (checklist) | Rarely; reviews arrive on GitHub |
| `researcher` | Read-only evidence from other trackers, BEPs and issue trackers | No skill | Ad hoc, without the "facts, not decisions" rule |
| `complexity-auditor` | Audit after every implementation step: cyclomatic count by hand, nesting, function length | Clippy `cognitive_complexity = "deny"` in `Cargo.toml` covers one of its four checks | No |
| `committer` | Review-and-verify commit workflow | `commit-changes` skill (most of it) | Sessions commit themselves; three rules have no other home |
| `implementer` | Test-driven implementation, and it also started the auditor, the task reviewer and the committer | `create-issue`, `fix-bug`, `write-unit-test`, `AGENTS.md` | The main session implements; the handoffs do not happen |
| `planner` | Write and classify specs, decompose, delegate | `create-issue` skill | Yes, through the skill |
| `github-operator` | GitHub operations kept out of the main context | `create-issue`, `link-subissue-to-parent-issue`, `fetch-review-threads`, `resolve-review-threads` | Yes, through the skills |
| `clippy-fixer` | Fix Clippy warnings, one commit per lint | `fix-clippy-warnings` skill | Yes, through the skill |

Evidence that the independent task review stopped: of the issue folders archived in October 2026,
25 of 48 contain an `agent-review-reports.md`; of those archived from 1 November 2026 to
15 November 2026, 2 of 32 do. The issue template still has a checkpoint for that report.

Method, on `develop` at the commit "Merge torrust/torrust-tracker#2900": a folder counts as
archived on the UTC date of the oldest commit that added or renamed a file into
`docs/issues/closed/<folder>/` (`git log --diff-filter=AR --name-only`, committer date); it has a
report when `docs/issues/closed/<folder>/agent-review-reports.md` exists at that revision of
`develop`, not at the commit that archived the folder: one October folder
(`2413-2410-si-22-1-document-event-flows-and-draft-adr`) received its report in a later commit the
same day, and checking at the archive commit gives 24 of 48. The
counts include EPIC folders: two in October, one of them with a report, and one in November,
without.

The implementation retrospective was not lost with the profiles: the `create-issue` skill and the
issue template both require the completion review.

The three `committer` rules with no other home are: check that the issue specification's progress
is current before committing; check that the branch's issue number has an open specification; and
commit `project-words.txt` changes first when one change is split into several commits. The
`run-pre-commit-checks` skill points to "the committer agent spec" for the second.

### Why the review role comes first

Review loops have been the main cost of recent work. Pull request #2851 took 14 review rounds,
recorded in `docs/pr-reviews/pr-2851-review/PR-REVIEW.md`. The attribution footers of the commits
that record each round name three models: rounds 1 to 3 Claude Fable 5.2, rounds 4 to 9 Claude
Sonnet (round 4 names Sonnet 5.5, rounds 5 to 9 name Sonnet 5), and rounds 10 to 14 Claude Opus
5.5. The processing log of that record holds 10 entries that correct earlier entries of the same
record. After round 9, the maintainer decided (2026-11-14) that the role that addresses reviews
runs on Opus 5.5 at minimum, and that the result is measured. So five rounds of that pull request
already ran on the model that D13 pins; the baseline for the measurement is the whole pull
request, 14 rounds on mixed models, not a count on one model. A Claude Code agent definition can pin the model, so the role no longer depends on
which model the session happens to use.

### What Claude Code supports

Every fact below was checked on 2026-11-15 against the raw text of the official Claude Code
documentation, and each row links the section that states it. The section anchors were derived
from the heading names and not opened in a browser; if one fails, search the page for the heading
in the link text. A row that starts with "Not documented" names the pages that were searched.
Nothing below was run yet; the manual scenarios verify the parts this issue relies on. The
installed version on the day was Claude Code 2.1.318.

| Fact | Where the documentation says it |
| --- | --- |
| **Agent files** |  |
| Project agents live in `.claude/agents/`, user agents in `~/.claude/agents/`; both are scanned recursively. | [Choose the subagent scope](https://code.claude.com/docs/en/sub-agents#choose-the-subagent-scope) |
| Only `name` and `description` are required in the frontmatter. | [Frontmatter reference](https://code.claude.com/docs/en/sub-agents#frontmatter-reference) |
| An unknown frontmatter key is ignored without an error. | [Frontmatter reference](https://code.claude.com/docs/en/sub-agents#frontmatter-reference) |
| `tools` is an allow list, and without it the agent inherits every tool; `disallowedTools` is a deny list. | [Restrict which tools a subagent can use](https://code.claude.com/docs/en/sub-agents#restrict-which-tools-a-subagent-can-use), [Frontmatter reference](https://code.claude.com/docs/en/sub-agents#frontmatter-reference) |
| `effort` is a frontmatter field; it overrides the session's effort level. | [Frontmatter reference](https://code.claude.com/docs/en/sub-agents#frontmatter-reference) |
| **Model** |  |
| `model` takes `sonnet`, `opus`, `haiku`, `fable`, a full model ID, or `inherit`. | [Choose a model](https://code.claude.com/docs/en/sub-agents#choose-a-model) |
| The model is the first of: the model the caller passes for one invocation, the frontmatter `model`, `CLAUDE_CODE_SUBAGENT_MODEL`, the main conversation's model. So a caller can override a pinned model. | [Choose a model](https://code.claude.com/docs/en/sub-agents#choose-a-model) |
| `CLAUDE_CODE_SUBAGENT_MODEL` together with `CLAUDE_CODE_SUBAGENT_MODEL_FORCE=1` makes Claude Code ignore the frontmatter `model`. | [Run every subagent on one model](https://code.claude.com/docs/en/sub-agents#run-every-subagent-on-one-model) |
| When an organization's model allow list blocks the pinned model, the agent runs on a substitute model, not an error. | [When an allowlist blocks a subagent model](https://code.claude.com/docs/en/sub-agents#when-an-allowlist-blocks-a-subagent-model), [Restrict model selection](https://code.claude.com/docs/en/model-config#restrict-model-selection) |
| **Starting an agent** |  |
| Claude delegates to an agent from its `description`, or the user names it with `@agent-<name>`. | [Understand automatic delegation](https://code.claude.com/docs/en/sub-agents#understand-automatic-delegation), [Invoke subagents explicitly](https://code.claude.com/docs/en/sub-agents#invoke-subagents-explicitly) |
| `claude --agent <name>`, or the `agent` settings key (scope "Any file"), runs the whole session as that agent; the flag wins over the setting. | [Invoke subagents explicitly](https://code.claude.com/docs/en/sub-agents#invoke-subagents-explicitly), [`agent` setting](https://code.claude.com/docs/en/settings-reference#agent) |
| In that mode the agent's body replaces the default Claude Code system prompt entirely. | [Invoke subagents explicitly](https://code.claude.com/docs/en/sub-agents#invoke-subagents-explicitly) |
| `tools: Agent(a, b)` is an allow list of the agents that may be started; it applies only to an agent running as the main thread. | [Restrict which subagents can be spawned](https://code.claude.com/docs/en/sub-agents#restrict-which-subagents-can-be-spawned) |
| An agent can start its own agents, up to three layers below the main conversation. | [Let subagents spawn their own subagents](https://code.claude.com/docs/en/sub-agents#let-subagents-spawn-their-own-subagents) |
| The CLI, the desktop app and the IDE extensions read the same settings files. | [Settings](https://code.claude.com/docs/en/settings) (page introduction) |
| Not documented: starting a desktop-app session as a named agent. The desktop page was searched for `--agent`, "custom agent", "named agent" and "subagent"; the web-session page was not checked. | [Desktop](https://code.claude.com/docs/en/desktop) |
| **Talking to an agent** |  |
| `AskUserQuestion` is one of the tools removed from every started agent, so it cannot ask the user. | [Available tools](https://code.claude.com/docs/en/sub-agents#available-tools) |
| The caller can continue a finished agent by its ID or name, and the agent keeps its full conversation history. | [Resume subagents](https://code.claude.com/docs/en/sub-agents#resume-subagents) |
| In the terminal the user can open an agent's transcript and type into it to continue it. | [Finished subagents in the panel and task list](https://code.claude.com/docs/en/sub-agents#finished-subagents-in-the-panel-and-task-list), [Resume subagents](https://code.claude.com/docs/en/sub-agents#resume-subagents) |
| In the desktop app the user can click a background entry to see its output in the subagent pane. Sending it a message there was not found. | [Watch background tasks](https://code.claude.com/docs/en/desktop#watch-background-tasks) |
| **Agent teams** |  |
| Experimental and disabled by default; enabled with `CLAUDE_CODE_EXPERIMENTAL_AGENT_TEAMS=1`. | [Agent teams](https://code.claude.com/docs/en/agent-teams) (page introduction), [Enable agent teams](https://code.claude.com/docs/en/agent-teams#enable-agent-teams) |
| The user can message any teammate directly. | [Talk to teammates directly](https://code.claude.com/docs/en/agent-teams#talk-to-teammates-directly) |
| A session has exactly one team. | [Limitations](https://code.claude.com/docs/en/agent-teams#limitations) |
| A teammate can use an agent definition, but its `skills` field is not applied. | [Use subagent definitions for teammates](https://code.claude.com/docs/en/agent-teams#use-subagent-definitions-for-teammates) |
| Agent teams are listed among the features available in the CLI and not in the desktop app. | [What's not available in Desktop](https://code.claude.com/docs/en/desktop#whats-not-available-in-desktop) |
| **Enforcement** |  |
| An agent's frontmatter can declare hooks; a `Stop` hook there becomes `SubagentStop`. | [Hooks in subagent frontmatter](https://code.claude.com/docs/en/sub-agents#hooks-in-subagent-frontmatter), [Hooks in skills and agents](https://code.claude.com/docs/en/hooks#hooks-in-skills-and-agents) |
| Exit code 2 from a `SubagentStop` hook prevents the agent from stopping. | [Exit code 2 behavior per event](https://code.claude.com/docs/en/hooks#exit-code-2-behavior-per-event) |
| A dynamic workflow takes no user input in the middle of a run. | [Behavior and limits](https://code.claude.com/docs/en/workflows#behavior-and-limits) |
| Not documented: using a named `.claude/agents/` definition in a workflow's `agent()` call. The workflows page was searched for `.claude/agents`, `subagent_type` and "agent type". | [Workflows](https://code.claude.com/docs/en/workflows) |
| **Skills** |  |
| Project skills are `.claude/skills/<skill-name>/SKILL.md`. The page describes no category folders between `skills/` and the skill. | [Choose where skills load](https://code.claude.com/docs/en/skills#choose-where-skills-load) |
| A `<skill-name>` entry can be a symlink to a directory elsewhere on disk. | [Choose where skills load](https://code.claude.com/docs/en/skills#choose-where-skills-load) |
| Not documented: a setting or variable that adds another skills folder, or reading `.agents/skills/`. The skills page was searched; the settings and environment-variable pages were not. `--add-dir` loads the skills of another directory's `.claude/skills/`. | [Load skills from a directory outside the project](https://code.claude.com/docs/en/skills#load-skills-from-a-directory-outside-the-project) |
| The `skills` field of an agent injects the full content of each listed skill at start; a skill that is missing is skipped with a warning in the debug log. | [Preload skills into subagents](https://code.claude.com/docs/en/sub-agents#preload-skills-into-subagents) |
| **`AGENTS.md`** |  |
| Reading `AGENTS.md` directly needs Claude Code 2.1.277 or later. | [AGENTS.md](https://code.claude.com/docs/en/memory#agents-md) |
| By default it is read only when there is no `CLAUDE.md`, `.claude/CLAUDE.md` or `CLAUDE.local.md` in the working directory or any directory above it. The rule is not limited to the repository, so a `CLAUDE.md` in a parent folder counts. | [When Claude Code reads AGENTS.md](https://code.claude.com/docs/en/memory#when-claude-code-reads-agents-md) |
| A user-level `~/.claude/CLAUDE.md` does not count. | [When Claude Code reads AGENTS.md](https://code.claude.com/docs/en/memory#when-claude-code-reads-agents-md) |
| To load both, a user chooses "Project instructions" = `claude-md-and-agents-md` in `/config`, or sets `pluginConfigs` in user or managed settings; project and local entries are ignored. | [Choose which instruction files load](https://code.claude.com/docs/en/memory#choose-which-instruction-files-load), [`pluginConfigs` setting](https://code.claude.com/docs/en/settings-reference#pluginconfigs) |
| A `CLAUDE.md` with an `@AGENTS.md` import next to it shares one file between tools. | [Share one file with other coding tools](https://code.claude.com/docs/en/memory#share-one-file-with-other-coding-tools) |
| A started agent loads every level of the instruction files the main conversation loads, including an `AGENTS.md` loaded as project instructions. The built-in Explore and Plan agents skip them. | [What loads at startup](https://code.claude.com/docs/en/sub-agents#what-loads-at-startup), [Built-in subagents](https://code.claude.com/docs/en/sub-agents#built-in-subagents) |

What this means for this issue:

- An agent definition can pin a model, but the pin can be overridden by the caller, by two
  environment variables and by an organization's allow list. The evidence must therefore read the
  model from the agent's transcript.
- An agent started by the orchestrator cannot ask the contributor a question. It returns the
  question, and the orchestrator can continue the same agent with the answer.
- Claude Code did not discover our skills while they lived in `.github/skills/`, so an agent could
  not preload them with `skills`. Since issue #2866 they are in `.claude/skills/<name>/` (D7), and
  an agent can.
- With a `CLAUDE.md` above the checkout, neither the session nor the agents it starts get the
  repository's `AGENTS.md`, unless the repository has its own `CLAUDE.md` that imports it (D11)
  or the contributor opts in on their side.

## Decisions

Decided by the maintainer on 2026-11-15:

- **D1 - One harness.** Claude Code is the repository's AI harness. Compatibility with GitHub
  Copilot custom agents is not kept: `.github/agents/` is removed, not maintained beside
  `.claude/agents/`.
- **D2 - Standards first.** Where an open standard exists that Claude Code and other agents
  follow, the repository uses the standard, not a Claude-only equivalent. So the root instructions
  stay in `AGENTS.md`, which is the single source of truth (see D11 for the pointer file that
  Claude Code needs). There is no shared standard for
  custom agent definitions, so `.claude/agents/` is the one harness-specific place. This decision
  is recorded in an ADR that belongs to the AI harness, not to the tracker (see
  [Architectural Decisions](#architectural-decisions)).
- **D3 - Explicit orchestrator.** A new `orchestrator` agent starts the implementer and the other
  agents. The implementer no longer starts the reviewers or any other agent; it only implements.
- **D4 - Review role first.** The first agent ported is the one that addresses pull-request
  reviews, with its model pinned to Opus 5.5 at minimum. It moves to Fable only if Opus 5.5 proves
  not enough (maintainer, 2026-11-14).

- **D5 - Every interaction between agents leaves a trace.** When one agent hands work to another,
  or reports back, the exchange is recorded in a Markdown file in the issue specification's
  folder, so a person can later see who asked what, who answered what, and with which result. The
  reviewer's report on the implementer's work is the first case. Agents may also pass messages in
  whatever way the harness offers, but the file is the record. In this issue the file is only a
  trace. Controlling the workflow from it, and later from a state machine, is future work
  (maintainer, 2026-11-15). D18 refines the format: JSON reply files, not Markdown.
- **D6 - The Copilot pull-request reviewer is not a constraint.** Moving or rewriting the agents
  and skills may change how well GitHub's Copilot reviewer reviews pull requests. That effect is
  accepted and is not measured or protected here. Review quality rests on two other layers: the
  independent review inside the orchestrated workflow while implementing (`task-reviewer`), and
  the review of the open pull request by the other maintainer, who uses a separate Claude account
  and configuration (maintainer, 2026-11-15).
- **D7 - The skills must be discoverable by Claude Code, in a flat folder with an index.** Leaving
  them where Claude Code does not find them is not an option. The 53 skills move from
  `.github/skills/dev/<area>/<name>/` to `.claude/skills/<name>/`, the only layout the
  documentation describes. Their names are already unique, so no skill is renamed. The grouping by
  area, which the folders gave, moves to an index file that lists the skills by area. Symlinks and
  plugins were considered and rejected: a second place to keep in sync, or an install step for
  every contributor. The move was a separate sub-issue of EPIC #2003, so
  that this issue does not grow too big (maintainer, 2026-11-15). Done: issue #2866, merged on
  2026-11-16. The index is `.claude/skills/README.md`, and the old `SKILL.md` paths hold redirect
  stubs.
- **D8 - The skills move first.** This issue started after the skills sub-issue (D7) was merged.
  The agents are therefore written against `.claude/skills/` from the start and can preload their
  skills with the `skills` field. The move rewrote two files in `.github/agents/`: `README.md`
  and `copilot-suggestions-handler.agent.md`, which now name the new skills folder. It did not
  rewrite the other profiles: five of them (`committer`, `implementer`, `github-operator`,
  `pr-reviewer`, `task-reviewer`) name skill paths that now hold redirect stubs, until this issue
  removes the profiles (maintainer, 2026-11-15).
- **D9 - The orchestrator is the session the contributor talks to.** It is not an agent started
  by another session. An agent that another agent starts cannot ask the contributor a question,
  and the orchestrator must: the approval of the specification, design questions, the hand-over.
  As a started agent it would need the main session to relay every exchange, and that session
  would be a second orchestrator with no rules. The role must not depend on one tool, neither the
  Claude Code desktop app nor Claude Code itself:

  - The body of `.claude/agents/orchestrator.md` is plain Markdown and complete on its own, so a
    session in any tool can take the role by reading it.
  - How a session gets the role is a convenience, not a requirement:
    `claude --agent orchestrator` in the terminal, or an instruction to read the file and act as
    the orchestrator anywhere else.
  - The frontmatter (model, tools, `skills`) is the only Claude Code part of the file.

  Two costs are accepted. When the implementer has a design question, it returns it; the
  orchestrator asks the contributor and continues the same implementer with the answer, which
  keeps its context. And a session started with `--agent` loses the default Claude Code system
  prompt, which is one more reason for the body to be complete. Agent teams, which would let the
  contributor talk to the implementer directly, are not used: they are experimental and not
  available in every Claude Code surface (maintainer, 2026-11-17).
- **D10 - The harness ADR goes in `docs/adrs/` now.** The ADR for D1 and D2 is written beside the
  other AI-harness ADRs, which are already there. The harness-workspace draft
  (`docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`) moves the harness
  crates and today plans no ADR move. So this issue adds an item to that draft's scope: decide
  whether the AI-harness ADRs move to the workspace, with this ADR named among them (maintainer,
  2026-11-17).
- **D11 - One source of truth, and pointers instead of copies.** `AGENTS.md` holds the root
  instructions because it is the standard and Claude Code supports it. Where a harness-specific
  file is needed for something the standard file cannot do, that file only points to the source of
  truth and repeats none of its content. First case: Claude Code does not read `AGENTS.md` when
  any `CLAUDE.md` exists in a directory above the checkout, so the repository adds a `CLAUDE.md`
  whose only content is the import `@AGENTS.md`. With it, every session and every agent it starts
  gets `AGENTS.md`, whatever the contributor has in parent folders. The same rule applies to any
  later harness-specific file (maintainer, 2026-11-17).
- **D12 - The complexity audit becomes one review item, then a linter.** `complexity-auditor` is
  not ported. The `review-task` checklist gains one item on function length and nesting, checked
  once before the pull request. A follow-up issue evaluates Clippy's `too_many_lines` and
  `excessive_nesting` lints as the deterministic replacement, in line with the EPIC's first design
  principle. The lints are not enabled here, because they may flag existing code (maintainer,
  2026-11-17).
- **D13 - Models.** `pr-review-handler`, `task-reviewer` and `pr-reviewer` are pinned to the full
  model ID of Opus 5.5. A full ID, not the alias `opus`, so the review-rounds measurement is
  against one known model and does not change when a new model ships. The reviewers are pinned
  because a reviewer weaker than the author finds little, and the pin keeps the review independent
  of how the session was started. `orchestrator`, `implementer` and `researcher` use `inherit`, so
  the contributor chooses the model per session. Known limit: the frontmatter cannot express a
  minimum, so in a session on a stronger model the pinned reviewers are weaker than the
  implementer. Changing a pin is a one-line pull request (maintainer, 2026-11-17).
- **D14 - The implementer commits; commit rules live in the skill, the plan and the hook.** There
  is no `committer` agent for now: a gate in front of every commit costs one agent run and two
  trace entries per commit, which is too much. Its work is placed as follows:

  - Conventions: the implementer preloads `commit-changes` with the `skills` field. The three
    rules that only the committer held move into that skill.
  - Rules about the shape of commits that need judgement (for example refactor first, then
    change): they are decided before implementing, in the specification's Commit Points, and the
    task reviewer checks the commit series against that plan once, before the pull request.
  - Rules a script can check (size, number, message format): they belong in the Git hook, added
    by their own issues when the rules exist.

  A `committer` agent can be added later if these three places prove not enough; nothing here
  blocks it (maintainer, 2026-11-17).
- **D15 - The Copilot setup workflow stays.** `.github/workflows/copilot-setup-steps.yml` is not
  removed, here or in a follow-up. Copilot is still requested as a reviewer of pull requests, and
  it is not known whether that review uses the workflow. It goes away when the Copilot review is
  replaced by the project's own reviewer, which is a later decision (maintainer, 2026-11-17).
- **D16 - One issue, in small pull requests that merge independently.** The priority is the
  review: each pull request is small enough to review easily, and nothing is merged that depends
  on an agent, a template or a rule that is not merged yet. So the order goes from what depends on
  nothing to what depends on everything:

  1. Foundations that the agents refer to (the pointer `CLAUDE.md`, the reply schema and its
     validator).
  2. One pull request per agent that starts no other agent (`pr-review-handler` first, then
     `task-reviewer`, `implementer`, `pr-reviewer`, `researcher`). Each is additive.
  3. The `orchestrator`, with the rewritten `docs/agents/orchestration.md`, after every agent it
     starts is merged.
  4. The removal of `.github/agents/` and the last references, with the ADR, at the end.

  An old profile is removed in the pull request of its replacement only when no live document
  still depends on it; otherwise it waits for step 4. After each merge `develop` is consistent:
  every check passes and no live document names something that does not exist (maintainer,
  2026-11-17).
- **D17 - A stop hook enforces the task-review reply.** `task-reviewer` declares a stop hook in
  its frontmatter. When the agent tries to finish, the hook runs the reply validator (D18) on the
  reply file the reviewer was asked to write; if the file is missing or invalid, it exits with
  code 2, which keeps the agent working, and tells it what is wrong. Reason: the review report is
  the one that stopped being written (25 of 48 archived folders in October, 2 of 32 in the first
  half of November; see Background for the method). Requirements: the hook cannot loop forever,
  and it lets the agent finish when the reviewer was given no folder-style specification, because
  then there is no place for a reply file. The bound is 3 refusals per run of the agent, the same
  number as the round limit of D18: the fourth attempt to finish is allowed, and the hook reports
  that the limit was reached. The orchestrator then finds no valid reply file, stops, and asks the
  contributor. This is the only
  hook in this issue; it is a Claude Code mechanism, so the rule itself stays written in the
  `review-task` skill (maintainer, 2026-11-17).
- **D18 - The trace is one JSON reply file per reply, validated against a schema.** Only replies
  are recorded for now, not requests. Stricter formats are preferred for artifacts, so that a
  schema can validate them and, later, the communication between agents:

  - Location: the folder `agent-replies/` in the issue specification's folder.
  - Name: `<agent>-<subject>-r<round>.json`. The agent is an agent name; the subject is a task ID
    of the Implementation Plan in lower case (`t2`) or `pr<number>`; the round starts at 1.
    Example: `task-reviewer-t2-r1.json`. No other name is accepted.
  - Rounds: each call of an agent on a subject writes a new file; earlier files are never
    edited. The limit is 3 rounds per agent and subject. If round 3 does not end in a pass, the
    orchestrator stops and asks the contributor.
  - Schema: one schema for every reply, `docs/schemas/agent-reply-v1.schema.json`, with the
    fields `schema-version`, `agent`, `model`, `subject`, `round`, `created-utc`, `verdict`,
    `summary`, `findings` (each with `severity`, `location`, `description`), `questions` and
    `evidence`. `verdict` is a closed list (for example `REVIEW_PASSED`, `REVIEW_FAILED`, `DONE`,
    `BLOCKED`, `QUESTION`), so the orchestrator decides the next step from one field.
  - Validation: one validator, used by the stop hook (D17) and by the repository checks on every
    reply file in a pull request. It checks the file name, the schema, and that the name agrees
    with the `agent`, `subject` and `round` fields.
  - Every agent the orchestrator starts writes a reply file. `pr-review-handler` is the
    exception for now: its audit record `PR-REVIEW.md` is its trace, and converting it belongs
    to the audit tooling of EPIC #2278.
  - The reply file replaces `agent-review-reports.md` for specifications created after T9
    changes the issue template; closed specifications keep theirs. This specification was
    created from the earlier template, so the independent reviews of its own work are recorded
    in `agent-review-reports.md` until the issue closes. The reply files written in this folder
    are the evidence of scenarios M5 and M7 only. The `agent-review-report-contract` check, which
    requires exact sentences in the Copilot profiles, is replaced by the validator.
  - Reply files are committed with the work they describe (maintainer, 2026-11-17).

Proposed in this specification and approved with it:

- **P1 - Agent roster.** Six agents:

  | Agent | From | Notes |
  | --- | --- | --- |
  | `orchestrator` | New | Owns the delivery path of one issue; starts the others |
  | `implementer` | `implementer` | Without the handoffs |
  | `task-reviewer` | `task-reviewer` | Gains one complexity item from `complexity-auditor` |
  | `pr-review-handler` | `copilot-suggestions-handler` | For every reviewer, not only Copilot; model pinned |
  | `pr-reviewer` | `pr-reviewer` | |
  | `researcher` | `researcher` | |

  Not ported: `planner` (its leftovers go to `create-issue`), `committer` (its three rules go to
  `commit-changes`), `github-operator`, `clippy-fixer` (its one-commit-per-lint rule goes to
  `fix-clippy-warnings`) and `complexity-auditor` (one checklist item goes to `review-task`; the
  audit after every step is dropped). See D12 and D14.
- **P2 - Thin agents.** An agent file holds the role's identity, tools, model, inputs and report
  format, and lists the skill that owns the workflow in its `skills` field, which preloads it.
  The workflow text is not copied into the agent, following the single-source-of-truth policy in `AGENTS.md`.

## Scope

### In Scope

- Create `.claude/agents/` with the agreed agents, one small pull request at a time (D16).
- Pin the model of `pr-review-handler` in its definition.
- Define the `orchestrator` agent and rewrite `docs/agents/orchestration.md` for it.
- Move the rules that only a dropped profile held into the skill that owns the workflow.
- Remove `.github/agents/`.
- Update every tracked reference to the profiles: `AGENTS.md`, `docs/agents/README.md`, the
  `agent-review-report-contract` check (replaced by the reply validator), the
  `frontmatter-validator` rule for `.agent.md` files,
  the `run-pre-commit-checks` pointer to the committer, the issue template checkpoints, and the
  governance ADR's inventory and review record.
- Add a root `CLAUDE.md` that only imports `AGENTS.md` (D11).
- Record decisions D1 and D2 in an ADR in `docs/adrs/`, and add to the harness-workspace draft's
  scope the item about moving the AI-harness ADRs (D10).
- Define the JSON reply files (D18): the schema, the validator with its tests, and the rule in
  every agent that writes one.
- Add the stop hook of `task-reviewer` (D17).
- Record the evidence that each agent works, and the review rounds per pull request for the first
  pull requests that `pr-review-handler` handles.

### Out of Scope

- The maintainer's personal roles and session conventions (coordinator sessions, session names,
  the contributor account). They are not repository content.
- A redesign of the review process. `process-pr-review` and `wait-for-pr-review` keep their
  contract; they gain only the sentence that says which agent runs a round.
- The audit tooling of EPIC #2278 (scripted thread resolution, generated or validated audit rows).
- Creating the separate AI-harness workspace
  (`docs/issues/drafts/2003-separate-ai-harness-cargo-workspace/ISSUE.md`).
- Moving the skills to `.claude/skills/` (D7, done in issue #2866), shortening their descriptions
  (issue #2873), and removing the redirect stubs left under `.github/skills/`.
- A state machine, or any other use of the reply files to control the workflow beyond the
  orchestrator reading the `verdict` field. Recording the requests is also left for later (D18).
- Dynamic workflows, or any machine enforcement of the handoffs other than the one stop hook of
  D17. The orchestrator is an agent that follows instructions; `docs/agents/orchestration.md`
  already says an arrow is intent, not enforcement.

## Architectural Decisions

- Related ADRs:
  - `docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`: this issue
    changes what it decided (Copilot alignment, profiles in `.github/agents/`).
  - `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`:
    its adapter rules still apply; its inventory row for custom profiles and its review records
    are updated, because a provider migration is one of its review triggers.
  - `docs/adrs/20261116082711_store_agent_skills_flat_in_claude_skills.md`: where the skills that
    the agents preload live (D7).
- ADRs to create: "Use Claude Code as the AI harness and prefer open agent standards" (D1, D2 and D11).
  It supersedes the Copilot-alignment part of ADR `20260420200013`. It belongs to the AI harness.
  The harness workspace does not exist yet, so it goes in `docs/adrs/` for now (D10).

## Design and Ownership Review

Not applicable. The work adds Markdown agent definitions, a JSON schema with a small validator,
one stop hook and documentation; it has no asynchronous I/O, network readiness or test fixtures.

The ownership that matters here is between agents:

| Agent | Talks to the contributor | Starts other agents | Writes |
| --- | --- | --- | --- |
| `orchestrator` | Yes; it is the session | Yes | The issue specification's progress, including the contributor's answers to returned questions |
| `implementer` | No; returns questions to the orchestrator | No | Code, tests, docs, commits, its reply file |
| `task-reviewer` | No | No | Verified checkboxes, its reply file |
| `pr-review-handler` | No; returns undecided findings | No | Fixes, commits, the push, replies, thread resolutions, `PR-REVIEW.md` |
| `pr-reviewer` | No | No | Its reply file |
| `researcher` | No | No | Its reply file, when it was given an issue folder |

The reply files are defined in D18. Only the agent named in a reply file writes it.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Open Questions

All settled with the maintainer between 2026-11-15 and 2026-11-17. Each answer is a decision above.

- **Q1 - Settled by D9.** The orchestrator is the session the contributor talks to.
- **Q2 - Settled by D10.** The ADR for D1 and D2 goes in `docs/adrs/` now.
- **Q3 - Settled by D11.** The repository adds a `CLAUDE.md` that only imports `AGENTS.md`.
- **Q4 - Settled by D7 and D8.** The skills moved to a flat `.claude/skills/` with an index in
  issue #2866. Its index records that Claude Code never lists a skill one folder deeper. The
  context cost of the 53 descriptions is handled by issue #2873.
- **Q5 - Settled by D12.** One item in `review-task` now; a follow-up issue for the Clippy lints.
- **Q6 - Settled by D13.** The handler and both reviewers are pinned to Opus 5.5; the rest inherit.
- **Q7 - Settled by D14.** The implementer commits; no `committer` agent for now.
- **Q8 - Settled by D15.** The Copilot setup workflow stays.
- **Q9 - Settled by D16.** One issue, in small pull requests that merge independently.
- **Q10 - Settled by D17.** `task-reviewer` gets a stop hook that requires its report.
- **Q11 - Settled by D18.** One JSON reply file per reply, validated against a schema.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

Each task is one pull request, in this order (D16). A task starts only after the tasks it depends
on are merged.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | TODO   | Specification pull request | This specification in `docs/issues/open/`, and a new row in the EPIC's Subissues table |
| T2  | TODO   | Pointer `CLAUDE.md` | A root `CLAUDE.md` whose only content is the import of `AGENTS.md` (D11); scenario M4 recorded |
| T3  | TODO   | Reply schema and validator | `docs/schemas/agent-reply-v1.schema.json`; the validator with tests, developed in the progressive loop of the `write-unit-test` skill with a recorded design review per increment, and run by the repository checks; `docs/schemas/README.md` updated (D18) |
| T4  | TODO   | `pr-review-handler` | `.claude/agents/pr-review-handler.md` with the pinned model; `wait-for-pr-review` and `process-pr-review` say which agent runs a round; the inventory `removed-profile-rules.md` created in this folder with the rules of `copilot-suggestions-handler` (AC6); scenarios M1 to M3 recorded |
| T5  | TODO   | `task-reviewer` | The agent file with the pinned model and the stop hook (D17), with tests of its limit of 3 refusals; the complexity item in `review-task` (D12); the reply rule in `review-task`; scenario M7 recorded |
| T6  | TODO   | `implementer` | The agent file; the three committer rules in `commit-changes` (D14); the rules that only `planner` held moved into `create-issue` |
| T7  | TODO   | `pr-reviewer` | The agent file with the pinned model; the reply rule in `review-pr` |
| T8  | TODO   | `researcher` | The agent file |
| T9  | TODO   | `orchestrator` | The agent file; `docs/agents/orchestration.md` rewritten for the orchestrator-led flow, with the 3-round limit; the issue template checkpoints; scenario M5 recorded |
| T10 | TODO   | End of `.github/agents/` | `.github/agents/` and the `agent-review-report-contract` check removed; the one-commit-per-lint rule in `fix-clippy-warnings`; the inventory `removed-profile-rules.md` completed for all ten profiles (AC6); `AGENTS.md`, `docs/agents/README.md`, `docs/templates/AGENT-REVIEW-REPORTS.md`, the `frontmatter-validator` rule and the governance ADR updated; the new ADR, and the ADR-move item added to the scope of the harness-workspace draft (D10) |
| T11 | TODO   | Measurement | Review rounds recorded for the first three pull requests handled by `pr-review-handler` (scenario M6); the pull requests of this issue count |
| T12 | TODO   | Close-out pull request | Completion review, specification moved to `docs/issues/closed/`, EPIC row `DONE` |

T4 to T8 are additive: each adds one agent and removes nothing that a live document still names.
The Copilot profiles stay until T10, so `develop` is consistent after every merge.

The inventory of AC6 is the file `removed-profile-rules.md` in this folder. It lists every rule
that only a Copilot profile holds, with its destination or the reason it
is dropped. The destination is a named skill. It is the agent file only for a rule about the role
itself (its identity, tools, inputs or report format), which P2 keeps in the agent; workflow text
never goes there. T4 creates it, each of T5 to T9 adds the rows of the profiles its agent replaces, and
T10 completes it before removing the profiles.

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1   | The specification; the EPIC row | One commit each, after `linter all` |
| T2   | The pointer file; the evidence | One commit each |
| T3   | The schema; the validator, one commit per reviewed test increment; its wiring into the checks; the documentation | One commit per item, after focused validation and the test-design review |
| T4 to T8 | The agent file; each skill change (in T6 the `create-issue` change is its own commit); the inventory rows; the evidence | One commit per item, after focused validation |
| T9   | The agent file; the orchestration guide; the template checkpoints; the inventory rows; the evidence | One commit per item, after focused validation |
| T10  | The completed inventory; the removal of `.github/agents/` with the old check and the validator rule; each moved rule; each documentation update; the ADR | One commit per item, after focused validation |
| T11  | The measurement evidence | One commit per recorded pull request |
| T12  | The completion review; the move to `closed/`; the EPIC row | One commit each |

Tests are added in T3 (the reply validator) and in T5 (the limit of 3 refusals of the stop hook). For
this test-producing work, use the `write-unit-test` skill and complete an explicit design review
after each passing test increment, before maintainer review and commit. Confirm that the test
exposes the one causal initial-state difference; its fixture owns only incidental mechanics; and
the production Act plus independently specified expected result remain visible. The review must
use the mandatory prose-first Arrange-Act-Assert comparison: write temporary prose for each
section, refactor until the code expresses it, remove redundant prose, and retain only irreducible
context. Record this review in task evidence or a file-local test plan. Assess helper boundaries
by meaningful named actions and abstraction-level alignment, not caller count: a single-use helper
is valid when it keeps the test readable and hides only incidental mechanics. Commit each reviewed
test-design increment before starting the next planned file or behavior area. Keep final
verification and completion evidence separate when it improves reviewability.

Record a justified no-change decision in the task's evidence without creating an empty commit. Use
a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2003-migrate-custom-agents-to-claude-code/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification (this issue keeps that file for its own reviews; see D18)
- [ ] Spec progress verified as up to date before each commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-15 08:40 UTC - Claude Code (Claude Fable 5.2) - Inventoried the ten profiles against the skills and the Claude Code documentation; drafted this specification with the maintainer's decisions D1 to D8, and eleven open questions; added what the Claude Code documentation says is possible, each fact with its link.
- 2026-11-17 12:09 UTC - Claude Code (Claude Fable 5.2) - Updated the draft after the skills move (#2866) merged; settled the open questions Q1 to Q11 with the maintainer as decisions D9 to D18; reordered the Implementation Plan into small pull requests that merge independently (D16).
- 2026-11-17 12:14 UTC - Claude Code (Claude Fable 5.2) - The maintainer approved the draft; created issue #2904, moved the specification to `docs/issues/open/`, and added row 23 to the EPIC's Subissues table. Linking #2904 as a GitHub sub-issue of #2003 is pending: the contributor account's request was refused with `404`.
- 2026-11-17 12:55 UTC - Claude Code (Claude Fable 5.2) - Addressed the review `5938631990` of `da2ce7` on pull request #2905 (findings F1 to F7), one commit per finding: the baseline of pull request #2851 is 14 rounds on three models, not 9 on one; the archived-report counts are recomputed with their method; the plan gains the AC6 inventory, the planner rules and the test-development loop.
- 2026-11-17 13:11 UTC - Claude Code (Claude Fable 5.2) - Addressed Copilot's review `5939023583` on pull request #2905 (findings F8 and F9): the stop hook's limit is 3 refusals, and D18 states which report file this issue uses for its own reviews.

## Acceptance Criteria

- [ ] AC1: `.claude/agents/` contains the agreed agents, and each loads in Claude Code with no
      ignored frontmatter key.
- [ ] AC2: `pr-review-handler` declares its model in its frontmatter, and a recorded run shows it
      ran on that model.
- [ ] AC3: One real review round on a real pull request was processed by `pr-review-handler` from
      fetch to resolved threads, with `validate-audit-record.py` exiting `0`.
- [ ] AC4: The `orchestrator` started the `implementer` and the `task-reviewer` on a real task, and
      each wrote a valid reply file. The implementer started no agent.
- [ ] AC5: `.github/agents/` no longer exists, and no tracked live document, skill, check or
      template refers to a removed profile. Closed specifications and merged audit records are
      history and stay unchanged.
- [ ] AC6: Every rule that only a removed profile held is either in a named skill, or in the agent
      file when it is about the role itself (identity, tools, inputs, report format), or listed as
      dropped with its reason, in the inventory `removed-profile-rules.md` in this specification's
      folder.
- [ ] AC7: `docs/agents/orchestration.md` describes the orchestrator-led flow and matches the
      agent files.
- [ ] AC8: The ADR for D1 and D2 exists at the agreed location, and the governance ADR's inventory
      and review record describe the new adapters.
- [ ] AC9: The review rounds of the first three pull requests handled by `pr-review-handler` are
      recorded next to the baseline of pull request #2851: 14 rounds, with its split by model.
- [ ] AC10: Every agent started in scenarios M5 and M7 has its reply file in `agent-replies/`, each
      file passes the validator, and a reader can follow the sequence from the files alone.
- [ ] AC11: The stop hook of `task-reviewer` refuses to let it finish without a valid reply file,
      and lets it finish on the fourth attempt, after 3 refusals, reporting that the limit was
      reached.
- [ ] AC12: The root `CLAUDE.md` contains only the import of `AGENTS.md`.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- The reply validator's tests, and a run of the validator on every reply file
- `cargo test -p agent-review-report-contract` until the check is removed in T10
- `cargo test -p frontmatter-validator` in the task that changes its `.agent.md` rule
- The pre-commit and pre-push hooks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

Every scenario records the Claude Code version it ran on.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1  | The agent files load | Start Claude Code with `--debug` in the repository; list the agents; search the debug output for ignored frontmatter | Every agent merged so far is listed; no key is reported as ignored | TODO | `manual-verification-evidence.md` section V1 |
| M2  | The pinned model applies | Start `pr-review-handler` from a session running on a different model; record the model named in the agent's transcript file, not the one the agent says it is | The agent ran on the model in its frontmatter, with no fallback | TODO | `manual-verification-evidence.md` section V2 |
| M3  | A real review round | On an open pull request with a submitted review, let the session that owns it delegate the round to `pr-review-handler` | Fixes committed and pushed, every thread replied to and then resolved, the audit record updated, `validate-audit-record.py` exits `0`, a short report returned | TODO | `manual-verification-evidence.md` section V3 |
| M4  | `AGENTS.md` reaches the agents | In a checkout with a `CLAUDE.md` in a parent folder, and in one without, ask the session and one started agent which repository rules they were given | Both the session and the started agent received `AGENTS.md` in both cases; `CLAUDE.md` contains only the import | TODO | `manual-verification-evidence.md` section V4 |
| M5  | The orchestrator leads | Start a session as the orchestrator on a small real task, once with `claude --agent orchestrator` and once by telling a normal session to read the file and take the role | It starts the implementer, then the task reviewer; each writes a valid reply file in `agent-replies/`; the implementer starts no agent; questions come back to the contributor through the orchestrator | TODO | `manual-verification-evidence.md` section V5 |
| M6  | Review rounds per pull request | For the first three pull requests handled by `pr-review-handler`, count the review rounds with fixes | The counts are recorded with the model of each round, next to the baseline of pull request #2851 (14 rounds, split by model) | TODO | `manual-verification-evidence.md` section V6 |
| M7  | The stop hook holds the reviewer | Start `task-reviewer` on a task, first with an instruction that makes it try to finish without a reply file, then normally | The hook refuses the first attempt and names what is missing; the agent then writes a valid reply file and finishes; a forced case shows 3 refusals and then the fourth attempt allowed, with the limit reported | TODO | `manual-verification-evidence.md` section V7 |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`
  when executing these scenarios, with the actual commands, output and outcomes.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned.

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
| AC11  | TODO                   |          |
| AC12  | TODO                   |          |

## Risks and Trade-offs

- **The orchestrator relays questions.** An implementer started by the orchestrator cannot ask the
  contributor anything. Each design question costs one stop and one restart of the implementer.
  Mitigation: the specification is approved before implementation starts, and the implementer is
  started per task, not per issue.
- **A reply can say more than what happened.** Each reply is written by a model. The schema
  checks its form, not its truth. Mitigation: replies are short, the `evidence` field names
  commands and paths that a reader can check, and the rules of "Statements in Your Own Records"
  in `process-pr-review` apply.
- **JSON is harder to read in a pull request.** Long findings read worse than in Markdown.
  Accepted for the validation it allows; the `summary` field keeps the first read short.
- **Many small pull requests.** Twelve pull requests, eleven after the specification's, each with
  its own review rounds.
  Accepted (D16): small pull requests are easier to review and never depend on unmerged work.
- **Three samples say little.** Three pull requests cannot prove that the pinned model lowers the
  round count; pull requests differ in size and reviewers. The measurement records the counts and
  claims no more than they show.
- **An agent is still instructions.** Nothing forces the orchestrator to start the task reviewer.
  The reviewer's reply file is the evidence that it did, and the stop hook (D17) guarantees the
  file once the reviewer runs.
- **A harness-specific folder.** `.claude/agents/` only works in Claude Code. The governance ADR
  allows it as an adapter: the workflows stay in skills and documents that any agent can read.
- **A role taken by instruction is weaker.** Where a session cannot be started as a named agent,
  the orchestrator role depends on the session being told to take it and keeping to it. M5 runs
  both ways and records the difference.
- **Text contract check.** The `agent-review-report-contract` check requires exact sentences in
  four Copilot profiles. The profiles and the check stay untouched until T10 removes both
  together, so the check passes after every merge.

## Implementation Completion Review

After implementation, compare the result with this specification. Record invalidated assumptions,
material design changes, unexpected validation findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository template at
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining why the work had no
  material discovery.
- When an independent reviewer receives this folder-style specification, it records its result in
  `agent-review-reports.md` using `docs/templates/AGENT-REVIEW-REPORTS.md`. This issue keeps that
  file for the reviews of its own work, as D18 states; the JSON reply files apply to
  specifications created after T9.

This step is not optional: the close-out pull request carries either the retrospective or the
progress-log entry.

## References

- Related issues: #2003, #2155, #2160, #2278
- Related PRs: #2851 (the baseline: 14 review rounds on three models, see Background)
- Related ADRs: `docs/adrs/20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md`,
  `docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`
- Agent orchestration guide: `docs/agents/orchestration.md`
- Claude Code documentation: `https://code.claude.com/docs/en/sub-agents.md`
