# Set Up Basic AI Agent Configuration

## Goal

Set up the foundational configuration files in this repository to enable effective collaboration with AI coding agents. This includes adding an `AGENTS.md` file to guide agents on project conventions, adding agent skills for repeatable specialized tasks, and defining custom agents for project-specific workflows.

## References

- **AGENTS.md specification**: https://agents.md/
- **Agent Skills specification**: https://agentskills.io/specification
- **GitHub Copilot — About agent skills**: https://docs.github.com/en/copilot/concepts/agents/about-agent-skills
- **GitHub Copilot — About custom agents**: https://docs.github.com/en/copilot/concepts/agents/copilot-cli/about-custom-agents

## Background

### AGENTS.md

`AGENTS.md` is an open, plain-Markdown format stewarded by the [Agentic AI Foundation](https://aaif.io/) under the Linux Foundation. It acts as a "README for agents": a single, predictable file where coding agents look first for project-specific context (build steps, test commands, conventions, security considerations) that would otherwise clutter the human-focused `README.md`.

It is supported by a wide ecosystem of tools including GitHub Copilot (VS Code), Cursor, Windsurf, OpenAI Codex, Claude Code, Jules (Google), Warp, and many others. In monorepos, nested `AGENTS.md` files can be placed inside each package; the closest file to the file being edited takes precedence.

### Agent Skills

Agent Skills (https://agentskills.io/specification) are directories of instructions, scripts, and resources that an agent can load to perform specialized, repeatable tasks. Each skill lives in a folder named after the skill and contains at minimum a `SKILL.md` file with YAML frontmatter (`name`, `description`, optional `license`, `compatibility`, `metadata`, `allowed-tools`) followed by Markdown instructions.

GitHub Copilot supports:

- **Project skills** stored in the repository at `.github/skills/`, `.claude/skills/`, or `.agents/skills/`
- **Personal skills** stored in the home directory at `~/.copilot/skills/`, `~/.claude/skills/`, or `~/.agents/skills/`

### Custom Agents

Custom agents are specialized versions of GitHub Copilot that can be tailored to project-specific workflows. They are defined as Markdown files with YAML frontmatter (agent profiles) stored at:

- **Repository level**: `.github/agents/CUSTOM-AGENT-NAME.md`
- **Organization/enterprise level**: `/agents/CUSTOM-AGENT-NAME.md` inside a `.github-private` repository

An agent profile includes a `name`, `description`, optional `tools`, and optional `mcp-servers` configurations. The Markdown body of the file acts as the agent's prompt (it is not a YAML frontmatter key). The main Copilot agent can run custom agents as subagents in isolated context windows, including in parallel.

## Tasks

### Task 0: Create a local branch

- Approved branch name: `<issue-number>-ai-agent-configuration`
- Commands:
  - `git fetch --all --prune`
  - `git checkout develop`
  - `git pull --ff-only`
  - `git checkout -b <issue-number>-ai-agent-configuration`
- Checkpoint: `git branch --show-current` should output `<issue-number>-ai-agent-configuration`.

---

### Task 1: Add `AGENTS.md` at the repository root

Provide AI coding agents with a clear, predictable source of project context so they can work
effectively without requiring repeated manual instructions.

**Inspiration / reference AGENTS.md files from other Torrust projects**:

- https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/AGENTS.md
- https://raw.githubusercontent.com/torrust/torrust-linting/refs/heads/main/AGENTS.md

Create `AGENTS.md` in the repository root, adapting the above files to the tracker. At minimum
the file must cover:

- [x] Repository link and project overview (language, license, MSRV, web framework, protocols, databases)
- [x] Tech stack (languages, frameworks, databases, containerization, linting tools)
- [x] Key directories (`src/`, `src/bin/`, `packages/`, `console/`, `contrib/`, `tests/`, `docs/`, `share/`, `storage/`, `.github/workflows/`)
- [x] Package catalog (all workspace packages with their layer and description)
- [x] Package naming conventions (`axum-*`, `*-server`, `*-core`, `*-protocol`)
- [x] Key configuration files (`.markdownlint.json`, `.yamllint-ci.yml`, `.taplo.toml`, `cspell.json`, `rustfmt.toml`, etc.)
- [x] Build & test commands (`cargo build`, `cargo test --doc`, `cargo test --all-targets`, E2E runner, benchmarks)
- [x] Lint commands (`linter all` and individual linters; how to install the `linter` binary)
- [x] Dependencies check (`cargo machete`)
- [x] Code style (rustfmt rules, clippy policy, import grouping, per-format rules)
- [x] Collaboration principles (no flattery, push back on weak ideas, flag blockers early)
- [x] Essential rules (linting gate, GPG commit signing, no `storage/`/`target/` commits, `cargo machete`)
- [x] Git workflow (branch naming, Conventional Commits, branch strategy: `develop` → `staging/main` → `main`)
- [x] Development principles (observability, testability, modularity, extensibility; Beck's four rules)
- [x] Container / Docker (key commands, ports, volume mount paths)
- [x] Auto-invoke skills placeholder (to be filled in when `.github/skills/` is populated)
- [x] Documentation quick-navigation table
- [x] Add a brief entry to `docs/index.md` pointing contributors to `AGENTS.md`, `.github/skills/`, and `.github/agents/`

Commit message: `docs(agents): add root AGENTS.md`

Checkpoint:

- `linter all` exits with code `0`.
- At least one AI agent (GitHub Copilot, Cursor, etc.) can be confirmed to pick up the file.

**References**:

- https://agents.md/
- https://github.com/openai/codex/blob/-/AGENTS.md (real-world example)
- https://github.com/apache/airflow/blob/-/AGENTS.md (real-world monorepo example)

---

### Task 2: Add Agent Skills

Define reusable, project-specific skills that agents can load to perform specialized tasks on
this repository consistently.

- [x] Create `.github/skills/` directory
- [x] Review and confirm the candidate skills listed below (add, remove, or adjust before starting implementation)
- [x] For each skill, create a directory with:
  - `SKILL.md` — YAML frontmatter (`name`, `description`, optional `license`, `compatibility`) + step-by-step instructions
  - `scripts/` (optional) — executable scripts the agent can run
  - `references/` (optional) — additional reference documentation
- [x] Validate skill files against the Agent Skills spec (name rules: lowercase, hyphens, no consecutive hyphens, max 64 chars; description: max 1024 chars)

**Candidate initial skills** (ported / adapted from `torrust-tracker-deployer`):

The skills below are modelled on the skills already proven in
[torrust-tracker-deployer](https://github.com/torrust/torrust-tracker-deployer)
(`.github/skills/`). Deployer-specific skills (Ansible, Tera templates, LXD, SDK,
deployer CLI architecture) are excluded because they have no equivalent in the tracker.

Directory layout to mirror the deployer structure:

```text
.github/skills/
  add-new-skill/
  dev/
    git-workflow/
    maintenance/
    planning/
    rust-code-quality/
    testing/
```

**`add-new-skill`** ✅ — meta-skill: guide for creating new Agent Skills for this repository.

**`dev/git-workflow/`**:

- `commit-changes` ✅ — commit following Conventional Commits; pre-commit verification checklist.
- `create-feature-branch` ✅ — branch naming convention and lifecycle.
- `open-pull-request` ✅ — open a PR via GitHub CLI or GitHub MCP tool; pre-flight checks.
- `release-new-version` ✅ — version bump, signed release commit, signed tag, CI verification.
- `review-pr` ✅ — review a PR against Torrust quality standards and checklist.
- `run-linters` ✅ — run the full linting suite (`linter all`); fix individual linter failures.
- `run-pre-commit-checks` ✅ — mandatory quality gates before every commit.

**`dev/maintenance/`**:

- `install-linter` ✅ — install the `linter` binary and its external tool dependencies.
- `setup-dev-environment` ✅ — full onboarding guide: system deps, Rust toolchain, storage dirs, linter, git hooks, smoke test.
- `update-dependencies` ✅ — run `cargo update`, create branch, commit, push, open PR.

**`dev/planning/`**:

- `create-adr` ✅ — create an Architectural Decision Record in `docs/adrs/`.
- `create-issue` ✅ — draft and open a GitHub issue following project conventions.
- `write-markdown-docs` ✅ — GFM pitfalls (auto-links, ordered list numbering, etc.).
- `cleanup-completed-issues` ✅ — remove issue doc files and update roadmap after PR merge.

**`dev/rust-code-quality/`**:

- `handle-errors-in-code` ✅ — `thiserror`-based structured errors; what/where/when/why context.
- `handle-secrets` ✅ — wrapper types for tokens/passwords; never use plain `String` for secrets.

**`dev/testing/`**:

- `write-unit-test` ✅ — `it_should_*` naming, AAA pattern, `MockClock`, `TempDir`, `rstest`.

Commit message: `docs(agents): add initial agent skills under .github/skills/`

Checkpoint:

- `linter all` exits with code `0`.
- At least one skill can be successfully activated by GitHub Copilot.

**References**:

- https://agentskills.io/specification
- https://docs.github.com/en/copilot/concepts/agents/about-agent-skills
- https://docs.github.com/en/copilot/how-tos/copilot-cli/customize-copilot/add-skills
- https://github.com/anthropics/skills (community skills examples)
- https://github.com/github/awesome-copilot (community collection)

---

### Task 3: Add Custom Agents

Define custom GitHub Copilot agents tailored to Torrust project workflows so that specialized
tasks can be delegated to focused agents with the right prompt context.

- [x] Create `.github/agents/` directory
- [x] Identify workflows that benefit from a dedicated agent
- [x] For each agent, create `.github/agents/<agent-name>.md` with:
  - YAML frontmatter: `name` (optional), `description`, optional `tools`
  - Prompt body: role definition, scope, constraints, and step-by-step instructions
- [ ] Test each custom agent by assigning it to a task or issue in GitHub Copilot CLI

**Candidate initial agents**:

- `committer` ✅ — commit specialist: reads branch/diff, runs pre-commit checks
  (`./scripts/pre-commit.sh`), proposes a GPG-signed Conventional Commit message, and creates
  the commit only after scope and checks are clear. Reference:
  [`torrust-tracker-demo/.github/agents/commiter.agent.md`](https://raw.githubusercontent.com/torrust/torrust-tracker-demo/refs/heads/main/.github/agents/commiter.agent.md)
- `implementer` ✅ — software implementer that applies Test-Driven Development and seeks the
  simplest solution. Follows a structured process: analyse → decompose into small steps →
  implement with TDD → call the Complexity Auditor after each step → call the Committer when
  ready. Guided by Beck's Four Rules of Simple Design.
- `complexity-auditor` ✅ — code quality auditor that checks cyclomatic and cognitive complexity
  of changes after each implementation step. Reports PASS/WARN/FAIL per function using thresholds
  and Clippy's `cognitive_complexity` lint. Called by the Implementer; can also be invoked
  directly.

**Future agents** (not yet implemented):

- `issue-planner` — given a GitHub issue, produces a detailed implementation plan document
  (like those in `docs/issues/`) including branch name, task breakdown, checkpoints, and commit
  message suggestions.
- `code-reviewer` — reviews PRs against Torrust coding conventions, clippy rules, and security
  considerations.
- `docs-writer` — creates or updates documentation files following the existing docs structure.

Commit message: `docs(agents): add initial custom agents under .github/agents/`

Checkpoint:

- `linter all` exits with code `0`.
- At least one custom agent can be assigned to a task in GitHub Copilot CLI.

**References**:

- https://docs.github.com/en/copilot/concepts/agents/copilot-cli/about-custom-agents
- https://docs.github.com/en/copilot/how-tos/copilot-cli/customize-copilot/create-custom-agents-for-cli
- https://docs.github.com/en/copilot/reference/customization-cheat-sheet

---

### Task 4 (optional / follow-up): Add nested `AGENTS.md` files in packages

Once the root file is stable, evaluate whether any workspace packages have sufficiently different
conventions or setup to warrant their own `AGENTS.md`. This can be tracked as a separate follow-up
issue.

- [x] Evaluate workspace packages for package-specific conventions
- [x] Add `packages/AGENTS.md` — guidance scoped to all workspace packages
- [x] Add `src/AGENTS.md` — guidance scoped to the main binary/library source

> **Note**: Completed as part of Task 1. `packages/AGENTS.md` and `src/AGENTS.md` were added
> alongside the root `AGENTS.md`.

---

### Task 5: Add `copilot-setup-steps.yml` workflow

Create `.github/workflows/copilot-setup-steps.yml` so that the GitHub Copilot cloud agent gets a
fully prepared development environment before it starts working on any task. Without this file,
Copilot discovers and installs dependencies itself via trial-and-error, which is slow and
unreliable.

The workflow must contain a single `copilot-setup-steps` job (the exact job name is required by
Copilot). Steps run in GitHub Actions before Copilot starts; the file is also automatically
executed as a normal CI workflow whenever it changes, providing built-in validation.

**Reference example** (from `torrust-tracker-deployer`):
https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/.github/workflows/copilot-setup-steps.yml

Minimum steps to include:

- [x] Trigger on `workflow_dispatch`, `push` and `pull_request` (scoped to the workflow file path)
- [x] `copilot-setup-steps` job on `ubuntu-latest`, `timeout-minutes: 30`, `permissions: contents: read`
- [x] `actions/checkout@v6` — check out the repository (verify this is still the latest stable
      version on the GitHub Marketplace before merging)
- [x] `dtolnay/rust-toolchain@stable` — install the stable Rust toolchain (pin MSRV if needed)
- [x] `Swatinem/rust-cache@v2` — cache `target/` and `~/.cargo` between runs
- [x] `cargo build` warm-up — build the workspace (or key packages) so incremental compilation is
      ready when Copilot starts editing
- [x] Install the `linter` binary —
      `cargo install --locked --git https://github.com/torrust/torrust-linting --bin linter`
- [x] Install `cargo-machete` — `cargo install cargo-machete`; ensures Copilot can run unused
      dependency checks (`cargo machete`) as required by the essential rules
- [x] Smoke-check: run `linter all` to confirm the environment is healthy before Copilot begins
- [x] Install Git pre-commit hooks — `./scripts/install-git-hooks.sh`

Commit message: `ci(copilot): add copilot-setup-steps workflow`

Checkpoint:

- The workflow runs successfully via the repository's **Actions** tab (manual dispatch or push to
  the file).
- `linter all` exits with code `0` inside the workflow.

**References**:

- https://docs.github.com/en/copilot/how-tos/use-copilot-agents/cloud-agent/customize-the-agent-environment
- https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/.github/workflows/copilot-setup-steps.yml

---

### Task 6: Create an ADR for the AI agent framework approach

> **Note**: This task documents the decision that underlies the whole issue. It can be done
> before Tasks 1–5 if preferred — recording the decision first and then implementing it is
> the conventional ADR practice.

Document the decision to build a custom, GitHub-Copilot-aligned agent framework (AGENTS.md +
Agent Skills + Custom Agents) rather than adopting one of the existing pre-defined agent
frameworks that were evaluated.

**Frameworks evaluated and not adopted**:

- [obra/superpowers](https://github.com/obra/superpowers)
- [gsd-build/get-shit-done](https://github.com/gsd-build/get-shit-done)

**Reasons for not adopting them**:

1. Complexity mismatch — they introduce abstractions that are heavier than what tracker
   development needs.
2. Precision requirements — the tracker involves low-level programming where agent work must be
   reviewed carefully; generic productivity frameworks are not designed around that constraint.
3. GitHub-first ecosystem — the tracker is hosted on GitHub and makes intensive use of GitHub
   resources (Actions, Copilot, MCP tools, etc.). Staying aligned with GitHub Copilot avoids
   unnecessary integration friction.
4. Tooling churn — the AI agent landscape is evolving rapidly; depending on a third-party
   framework risks forced refactoring when that framework is deprecated or pivots. A first-party
   approach is more stable.
5. Tailored fit — a custom solution can be shaped precisely to Torrust conventions, commit style,
   linting gates, and package structure from day one.
6. Proven in practice — the same approach has already been validated during the development of
   `torrust-tracker-deployer`.
7. Agent-agnostic by design — keeping the framework expressed as plain Markdown files
   (AGENTS.md, SKILL.md, agent profiles) decouples it from any single agent product, making
   migration or multi-agent use straightforward.
8. Incremental adoption — individual skills, custom agents, or patterns from those frameworks can
   still be cherry-picked and integrated progressively if specific value is identified.

- [x] Create `docs/adrs/<YYYYMMDDHHMMSS>_ai-agent-framework-approach.md` using the `create-adr` skill
- [x] Record the decision, the alternatives considered, and the reasoning above

Commit message: `docs(adrs): add ADR for AI agent framework approach`

Checkpoint:

- `linter all` exits with code `0`.

**References**:

- `docs/adrs/README.md` — ADR naming convention for this repository
- https://adr.github.io/

---

## Acceptance Criteria

- [ ] `AGENTS.md` exists at the repo root and contains accurate, up-to-date project guidance.
- [ ] At least one skill is available under `.github/skills/` and can be successfully activated by GitHub Copilot.
- [ ] At least one custom agent is available under `.github/agents/` and can be assigned to a task.
- [ ] `copilot-setup-steps.yml` exists, the workflow runs successfully in the **Actions** tab, and `linter all` exits with code `0` inside it.
- [ ] An ADR exists in `docs/adrs/` documenting the decision to use a custom GitHub-Copilot-aligned agent framework.
- [ ] All files pass spelling checks (`cspell`) and markdown linting.
- [ ] A brief entry in `docs/index.md` points contributors to `AGENTS.md`, `.github/skills/`, and `.github/agents/`.
