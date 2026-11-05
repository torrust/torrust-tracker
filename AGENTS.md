# Torrust Tracker — AI Assistant Instructions

**Repository**: [torrust/torrust-tracker](https://github.com/torrust/torrust-tracker)

## 📋 Project Overview

**Torrust Tracker** is a production-grade BitTorrent tracker written in Rust (edition 2024, MSRV 1.88;
AGPL-3.0-only; version 3.0.0-develop). It matchmakes peers and collects statistics over UDP, HTTP and
TLS, with IPv4/IPv6, private/whitelisted mode and a management REST API.

**MSRV policy**: once the `bittorrent-*` crates are extracted as standalone libraries (#1669), the
tracker application tracks a recent stable Rust, while each library carries the minimum MSRV its
external consumers need.

## 📁 Key Directories

- `packages/` — Cargo workspace packages; all domain logic lives here (see the [package catalog](docs/packages.md#package-catalog))
- `console/` — Console tools
- `contrib/dev-tools/` — Git hooks, container and init scripts, and the developer-tool crates
- `tests/` — Main-application integration tests (see `tests/AGENTS.md`)
- `docs/` — Documentation index: `docs/index.md`; ADRs in `docs/adrs/`; issue specs in `docs/issues/`
- `share/default/` — Default configuration files and fixtures
- `storage/` — Runtime data (git-ignored)
- `.tmp/` — Workspace-local temp dir (git-ignored): hook logs and benchmark isolation dirs
- `.github/workflows/`, `.github/skills/`, `.github/agents/` — CI workflows, Agent Skills, custom agents

## 📦 Package Catalog

Packages, crate names, layers, publish status and tools are listed in the
[package catalog](docs/packages.md#package-catalog). The authoritative, always-current list is
`cargo metadata --no-deps --format-version 1`. Package naming conventions (prefixes and layers) are
in [package conventions](docs/packages.md#package-conventions).

## 📄 Key Configuration Files

The `linter` binary has no configuration file of its own, and `.gitignore` does not exclude files
from linting; the `run-linters` skill lists each tool's config and ignore mechanism. Cargo aliases
and global `rustflags` are in `.cargo/config.toml`; formatting is in `rustfmt.toml`.

## 🧪 Build, Test, and Lint

Mandatory quality gate before every commit:

```sh
./contrib/dev-tools/git/hooks/pre-commit.sh
```

Linter entry point: `linter all` (the `linter` binary from [torrust/torrust-linting](https://github.com/torrust/torrust-linting)). CI is the merge authority and also runs the E2E matrix.

Both git hooks write per-step logs to `TORRUST_GIT_HOOKS_LOG_DIR` (default: `/tmp`). In restricted
AI-agent sandboxes set `TORRUST_GIT_HOOKS_LOG_DIR=.tmp` so the logs stay inside the workspace
(`.tmp/` is git-ignored).

Skills for these workflows (each is `.github/skills/dev/<area>/<name>/SKILL.md`): `run-linters`,
`run-pre-commit-checks`, `run-pre-push-checks`, `setup-dev-environment`.

## 🎨 Code Style

- **Compile flags**: `.cargo/config.toml` enables strict global `rustflags` (`-D warnings`,
  `-D unused`, `-D rust-2018-idioms`, `-D future-incompatible`, and others). All code must
  compile cleanly with these flags and pass clippy without warnings — no suppressions unless
  absolutely necessary.
- **Imports**: All imports at the top of the file, grouped (std → external crates → internal
  crate). Prefer short imported names over fully-qualified paths
  (e.g., `Arc<MyType>` not `std::sync::Arc<crate::my::MyType>`). Use full paths only to
  disambiguate naming conflicts.
- **Formatting and linting** (rustfmt, taplo, yamllint, markdownlint, lychee, cspell) are enforced by
  `linter all`. Add new technical terms to `project-words.txt` (one word per line, alphabetical).

## 🤝 Collaboration Principles

These rules apply repository-wide to every assistant, including custom agents.

When acting as an assistant in this repository:

- Do not flatter the user or agree with weak ideas by default.
- Push back when a request, diff, or proposed commit looks wrong.
- Flag unclear but important points before they become problems.
- Ask a clarifying question instead of making a random choice when the decision matters.
- Before starting a task or process that has no documented repository workflow and no verified
  repository precedent, stop after read-only investigation, identify the gap, and ask the
  maintainer for explicit approval of the proposed process. Do not create a branch, edit files,
  commit, push, open or modify GitHub artifacts, reply to reviews, or resolve threads before that
  approval. A request for an outcome is not approval to invent an undocumented process unless the
  maintainer explicitly acknowledges the gap and authorizes the proposed workflow. When the new
  process proves reusable, document it in the owning skill and update affected adapters in the same
  work or an explicitly approved follow-up.
- Call out likely misses: naming inconsistencies, accidental generated files,
  staged-versus-unstaged mismatches, missing docs updates, or suspicious commit scope.

When raising a likely mistake or blocker, say so clearly and early instead of burying it after
routine status updates.

## 🧭 Engineering Policies

These policies are repository-wide and apply to all agents and workflows.

<!-- skill-link: add-rust-dependency -->

1. **Dependency freshness**: prefer the latest stable crate version; if not using it, document why.
2. **Container base image freshness**: prefer current supported base images in `Containerfile`
   and compose artifacts; document the reason for an older one.
3. **Shell vs Rust threshold**: shell for simple orchestration only; prefer Rust when logic is
   non-trivial, stateful, safety-critical, or worth testing independently.
4. **Testing coverage and maintainability**: aim for high maintainable automated coverage; document
   why behaviour is left untested and prefer improving testability.
5. **Rust documentation quality**: document public APIs and important module/type invariants; prefer
   high-signal docs over boilerplate.
6. **Documentation single source of truth**: do not duplicate procedural guidance; keep folder
   READMEs lightweight; `.github/skills/` and canonical docs (for example `docs/index.md`) are the
   authoritative workflow sources; replace duplicates with links.
7. **AI-agent implementation independence**: keep repository knowledge, decisions, workflows, and
   validation reproducible from Git-tracked documentation, scripts, tests, and documented standard
   interfaces. Provider-specific profiles, state, indexes, tools, and cloud setup are optional
   adapters, not sources of truth; document an adapter's purpose, portability limitation, and
   practical alternative before making it required. See
   [`docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md`](docs/adrs/20260821172000_establish_ai_agent_context_capability_and_portability_governance.md).
8. **Repository-owned skill authority**: for repository workflows, follow this repository's skills
   over third-party, provider, runtime, or IDE skills (optional adapters). When their guidance
   materially conflicts, warn the user, name the conflict and the governing repository skill, and
   follow the repository guidance unless a higher-priority instruction prevents it.

Implementation workflow skills (same location): `update-dependencies`, `add-rust-dependency`,
`write-unit-test`, `implement-workflow`.

## 🔧 Essential Rules

1. **Linting gate**: `linter all` must exit `0` before every commit. No exceptions.
2. **GPG commit signing**: All commits **must** be signed with GPG (`git commit -S`). **GPG timeout handling**: If the GPG passphrase prompt times out during a commit, the agent
   **must stop the failed attempt**, notify the user, and ask whether they prefer to retry the
   commit manually or have the agent rerun the same signed command while they enter the
   passphrase directly in the terminal prompt. Never retry automatically, request or handle the
   passphrase in chat, bypass GPG signing with `--no-gpg-sign`, or skip the signing step under
   any circumstances. This rule is absolute.
3. **Never commit `storage/` or `target/`**: These directories contain runtime data and build
   artifacts. They are git-ignored; never force-add them.
4. **Unused dependencies**: Run `cargo machete` before committing. Remove any unused
   dependencies immediately.
5. **Continuous self-review**: Review your own work against project quality standards. Apply
   self-review at three levels:
   - **Mandatory** — before opening a pull request
   - **Strongly recommended** — before each commit
   - **Recommended** — after completing each small, independent, deployable change
6. **Security**: Do not report security vulnerabilities through public GitHub issues. Send an
   email to `info@nautilus-cyberneering.de` instead. See [SECURITY.md](SECURITY.md).
7. **Skill-link synchronization**: When modifying any artifact containing a `skill-link:` marker,
   also review and update the linked skill instructions in `.github/skills/` so behavior,
   commands, and references remain aligned. If the linked skill has a validation script, run it
   before finishing.

## 🌿 Git Workflow

**Branch naming**:

```text
<issue-number>-<short-description>   # e.g. 1697-ai-agent-configuration (preferred)
feat/<short-description>             # for features without a tracked issue
fix/<short-description>              # for bug fixes
chore/<short-description>            # for maintenance tasks
```

A spec-only branch adds a `-spec` suffix, so the base name stays free for implementation (see the
`create-feature-branch` skill).

**Commit messages** follow [Conventional Commits](https://www.conventionalcommits.org/):
`type(scope): summary`, with the types `feat`, `fix`, `chore`, `docs`, `refactor`, `ci` and `test`.
Scope names the affected package or area (e.g., `tracker-core`, `udp-protocol`, `ci`, `docs`).

**Branch strategy**:

- Feature branches are cut from `develop`; PRs target `develop`
- Direct pushes to `develop` and `main` are not allowed; changes must be merged via PR
- PRs targeting `develop` or `main` must come from a fork branch (`<fork-owner>:<branch>`), not a branch in `torrust/torrust-tracker`
- PRs must pass all CI status checks before merge

**Commit history**: keep intermediate commits. Do not squash, fixup, or rewrite a branch's history to
hide steps that were later superseded; the sequence of a mistake and its correction is deliberately
preserved as training material. Never propose squashing as a cleanup. Minor process, skill, or
documentation improvements may ride along in the PR that produced them, each in its own commit; do
not propose splitting them out by default.

PR titles, bodies, issue-linking keywords, rebasing and history-rewrite rules are in the
`open-pull-request` skill (`.github/skills/dev/git-workflow/open-pull-request/SKILL.md`); merging
is in the `merge-pull-request` skill.

**After opening a PR**, an agent does not stop: it runs the review loop in the `wait-for-pr-review`
skill (poller wait, `process-pr-review`, push, re-request) and asks the contributor only when more
than 5 rounds of review fixes have been pushed, or when the skill says to.

See [docs/release_process.md](docs/release_process.md) for the release workflow.

## 🧭 Development Principles

For detailed information see [`docs/`](docs/).

**Core Principles:**

- **Observability**: If it happens, we can see it — even after it happens (deep traceability)
- **Testability**: Every component must be testable in isolation and as part of the whole
- **Modularity**: Clear package boundaries; servers contain only network I/O logic
- **Extensibility**: Core logic is framework-agnostic for easy protocol additions

**Code Quality Standards** — both production and test code must be:

- **Clean**: Well-structured with clear naming and minimal complexity
- **Maintainable**: Easy to modify and extend without breaking existing functionality
- **Readable**: Clear intent that can be understood by other developers
- **Testable**: Designed to support comprehensive testing at all levels

**Beck's Four Rules of Simple Design** (in priority order):

1. **Passes the tests**: The code must work as intended — testing is a first-class activity
2. **Reveals intention**: Code should be easy to understand, expressing purpose clearly
3. **No duplication**: Apply DRY — eliminating duplication drives out good designs
4. **Fewest elements**: Remove anything that doesn't serve the prior three rules

Reference: [Beck Design Rules](https://martinfowler.com/bliki/BeckDesignRules.html)

## 🐳 Container / Docker

Running the image, volume mappings, ports and Docker Compose usage are in
[docs/containers.md](docs/containers.md).

## 🎯 Auto-Invoke Skills

Agent Skills live under [`.github/skills/`](.github/skills/); custom agents are described in
[`.github/agents/README.md`](.github/agents/README.md). Skills supplement (not replace) the rules in
this file: rules apply always, skills activate when their workflows are needed.

## 📚 Documentation

The documentation index is [`docs/index.md`](docs/index.md) (architecture, ADRs, issue specs,
benchmarking, profiling, containers, release process). Do not report security vulnerabilities through
public issues; see [SECURITY.md](SECURITY.md).
