# Torrust Tracker — AI Assistant Instructions

**Repository**: [torrust/torrust-tracker](https://github.com/torrust/torrust-tracker)

## 📋 Project Overview

**Torrust Tracker** is a high-quality, production-grade BitTorrent tracker written in Rust. It
matchmakes peers and collects statistics, supporting the UDP, HTTP, and TLS socket types with
native IPv4/IPv6 support, private/whitelisted mode, and a management REST API.

- **Language**: Rust (edition 2024, MSRV 1.88)
  - **MSRV policy**: Once `bittorrent-*` crates are extracted as standalone
    libraries (#1669), the tracker application should track a recent stable Rust
    version while those libraries should each carry the minimum MSRV needed for
    external consumer compatibility.
- **License**: AGPL-3.0-only
- **Version**: 3.0.0-develop
- **Web framework**: [Axum](https://github.com/tokio-rs/axum)
- **Async runtime**: Tokio
- **Protocols**: BitTorrent UDP (BEP 15), HTTP (BEP 3/23), REST management API
- **Databases**: SQLite3, MySQL, PostgreSQL
- **Workspace type**: Cargo workspace (multi-crate monorepo)

## 🏗️ Tech Stack

- **Languages**: Rust, YAML, TOML, Markdown, Shell scripts
- **Web framework**: Axum (HTTP server + REST API)
- **Async runtime**: Tokio (multi-thread)
- **Testing**: testcontainers (E2E)
- **Databases**: SQLite3, MySQL, PostgreSQL
- **Containerization**: Docker / Podman (`Containerfile`)
- **CI**: GitHub Actions
- **Linting tools**: markdownlint, yamllint, taplo, cspell, shellcheck, clippy, rustfmt (unified
  under the `linter` binary from [torrust/torrust-linting](https://github.com/torrust/torrust-linting))

## 📁 Key Directories

- `src/` — Main binary and library entry points (`main.rs`, `lib.rs`, `app.rs`, `container.rs`)
- `src/bin/` — Additional binary targets (`e2e_tests_runner`, `http_health_check`, `profiling`)
- `src/bootstrap/` — Application bootstrap logic
- `src/console/` — Console entry points
- `packages/` — Cargo workspace packages (all domain logic lives here; see package catalog below)
- `console/` — Console tools (e.g., `tracker-client`)
- `contrib/` — Developer tooling
- `contrib/dev-tools/` — Developer tools: git hooks (`pre-commit.sh`, `pre-push.sh`, `install-git-hooks.sh`),
  container scripts, and init scripts
- `tests/` — Integration tests (`integration.rs`, `servers/`)
- `docs/` — Project documentation, ADRs, issue specs, and benchmarking guides
- `docs/adrs/` — Architectural Decision Records
- `docs/issues/` — Issue specs / implementation plans
- `share/default/` — Default configuration files and fixtures
- `storage/` — Runtime data (git-ignored); databases, logs, config
- `.tmp/` — Workspace-local temp dir (git-ignored); AI agent hook logs (`TORRUST_GIT_HOOKS_LOG_DIR=.tmp`)
  and benchmark script cargo isolation dirs (`contrib/dev-tools/workflow-benchmarks/`)
- `.github/workflows/` — CI/CD workflows (testing, coverage, container, deployment)
- `.github/skills/` — Agent Skills for specialized workflows and task-specific guidance
- `.github/agents/` — Custom Copilot agents and their repository-specific definitions

## 📦 Package Catalog

All packages live under `packages/`. The workspace version is `3.0.0-develop`.

| Package                           | Crate Name                                        | Prefix / Layer | Description                                   |
| --------------------------------- | ------------------------------------------------- | -------------- | --------------------------------------------- |
| `axum-health-check-api-server`    | `torrust-tracker-axum-health-check-api-server`    | `axum-*`       | Health monitoring endpoint                    |
| `axum-http-server`                | `torrust-tracker-axum-http-server`                | `axum-*`       | BitTorrent HTTP tracker server (BEP 3/23)     |
| `axum-rest-api-server`            | `torrust-tracker-axum-rest-api-server`            | `axum-*`       | Management REST API server                    |
| `axum-server`                     | `torrust-tracker-axum-server`                     | `axum-*`       | Base Axum HTTP server infrastructure          |
| `configuration`                   | `torrust-tracker-configuration`                   | domain         | Config file parsing, environment variables    |
| `events`                          | `torrust-tracker-events`                          | domain         | Domain event definitions                      |
| `http-protocol`                   | `torrust-tracker-http-protocol`                   | `*-protocol`   | HTTP tracker protocol (BEP 3/23) parsing      |
| `http-core`                       | `torrust-tracker-http-core`                       | `*-core`       | HTTP-specific tracker domain logic            |
| `primitives`                      | `torrust-tracker-primitives`                      | domain         | Core domain types (InfoHash, PeerId, ...)     |
| `rest-api-client`                 | `torrust-tracker-rest-api-client`                 | client tools   | REST API client library                       |
| `rest-api-core`                   | `torrust-tracker-rest-api-core`                   | client tools   | REST API core logic                           |
| `swarm-coordination-registry`     | `torrust-tracker-swarm-coordination-registry`     | domain         | Torrent/peer coordination registry            |
| `test-helpers`                    | `torrust-tracker-test-helpers`                    | utilities      | Mock servers, test data generation            |
| `torrent-repository-benchmarking` | `torrust-tracker-torrent-repository-benchmarking` | benchmarking   | Torrent storage benchmarks                    |
| `tracker-client`                  | `torrust-tracker-client`                          | client tools   | CLI tracker interaction/testing client        |
| `tracker-core`                    | `torrust-tracker-core`                            | `*-core`       | Central tracker peer-management logic         |
| `udp-protocol`                    | `torrust-tracker-udp-protocol`                    | `*-protocol`   | UDP tracker protocol (BEP 15) framing/parsing |
| `udp-core`                        | `torrust-tracker-udp-core`                        | `*-core`       | UDP-specific tracker domain logic             |
| `udp-server`                      | `torrust-tracker-udp-server`                      | server         | UDP tracker server implementation             |

**Extracted packages** — previously part of this workspace, now in their own standalone repositories:

| Package          | Crate Name               | Standalone Repository                                                               | Description                                                      |
| ---------------- | ------------------------ | ----------------------------------------------------------------------------------- | ---------------------------------------------------------------- |
| `clock`          | `torrust-clock`          | [torrust/torrust-clock](https://github.com/torrust/torrust-clock)                   | Deterministic clock abstraction                                  |
| `located-error`  | `torrust-located-error`  | [torrust/torrust-located-error](https://github.com/torrust/torrust-located-error)   | Diagnostic errors with source locations                          |
| `metrics`        | `torrust-metrics`        | [torrust/torrust-metrics](https://github.com/torrust/torrust-metrics)               | Prometheus-compatible metrics: counters, gauges, labels, samples |
| `net-primitives` | `torrust-net-primitives` | [torrust/torrust-net-primitives](https://github.com/torrust/torrust-net-primitives) | Generic networking primitive types (ServiceBinding, Protocol)    |
| `server-lib`     | `torrust-server-lib`     | [torrust/torrust-server-lib](https://github.com/torrust/torrust-server-lib)         | Shared server library utilities                                  |

**Console tools** (under `console/`):

| Tool             | Description                          |
| ---------------- | ------------------------------------ |
| `tracker-client` | Client for interacting with trackers |

**Community contributions** (under `contrib/`):

| Crate | Description                                                                                            |
| ----- | ------------------------------------------------------------------------------------------------------ |
| —     | None (bencode migrated to [torrust/torrust-bittorrent](https://github.com/torrust/torrust-bittorrent)) |

## 🏷️ Package Naming Conventions

| Prefix       | Responsibility                         | Dependencies             |
| ------------ | -------------------------------------- | ------------------------ |
| `axum-*`     | HTTP server components using Axum      | Axum framework           |
| `*-server`   | Server implementations                 | Corresponding `*-core`   |
| `*-core`     | Domain logic and business rules        | Protocol implementations |
| `*-protocol` | BitTorrent protocol implementations    | BEP specifications       |
| `udp-*`      | UDP protocol-specific implementations  | Tracker core             |
| `http-*`     | HTTP protocol-specific implementations | Tracker core             |

## 📄 Key Configuration Files

The `linter` binary has **no configuration file of its own**. It is a thin wrapper that
delegates to each tool, which reads its own config file from the project root. The
config files are already present in the repository — no manual setup is needed.

Files listed in `.gitignore` are **not** automatically excluded from linting. Each linter
has its own ignore mechanism (e.g. `.markdownlintignore` for markdownlint,
`.cspell.gitignore` for cspell). Add `.gitignore` paths that must be excluded from a
linter to the appropriate ignore file.

| File                                      | Used by                                                                                                                             |
| ----------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| `.markdownlint.json`                      | markdownlint                                                                                                                        |
| `.yamllint-ci.yml`                        | yamllint                                                                                                                            |
| `.taplo.toml`                             | taplo (TOML formatting)                                                                                                             |
| `cspell.json`                             | cspell (spell checker) configuration                                                                                                |
| `project-words.txt`                       | cspell project-specific dictionary                                                                                                  |
| `rustfmt.toml`                            | rustfmt (`group_imports = "StdExternalCrate"`, `max_width = 130`)                                                                   |
| `.cargo/config.toml`                      | Cargo aliases (`cov`, `cov-lcov`, `cov-html`, `time`) and global `rustflags` (`-D warnings`, `-D unused`, `-D rust-2018-idioms`, …) |
| `Cargo.toml`                              | Cargo workspace root                                                                                                                |
| `compose.qbittorrent-e2e.sqlite3.yaml`    | qBittorrent E2E Compose stack for SQLite backend                                                                                    |
| `compose.qbittorrent-e2e.mysql.yaml`      | qBittorrent E2E Compose stack for MySQL backend                                                                                     |
| `compose.qbittorrent-e2e.postgresql.yaml` | qBittorrent E2E Compose stack for PostgreSQL backend                                                                                |
| `Containerfile`                           | Container image definition                                                                                                          |
| `codecov.yaml`                            | Code coverage configuration                                                                                                         |

## 🧪 Build, Test, and Lint

Use this section as a quick policy-level summary. For detailed command workflows and troubleshooting,
prefer the corresponding skills.

Common commands:

```sh
cargo build
cargo test --doc --workspace
cargo test --tests --benches --examples --workspace --all-targets --all-features
cargo test --test integration
cargo +nightly doc --no-deps --bins --examples --workspace --all-features
cargo bench --package torrent-repository-benchmarking
```

Mandatory quality gate before every commit:

```sh
./contrib/dev-tools/git/hooks/pre-commit.sh
```

Pre-commit defaults to concise text output and runs the fast local profile:

1. `cargo machete`
2. `linter all`
3. `cargo test --doc --workspace`

Use `--format=text --verbosity=verbose` for full streaming output, or `--format=json` for a
single structured JSON payload.

Both hooks write per-step logs to `TORRUST_GIT_HOOKS_LOG_DIR` (default: `/tmp`).
In restricted AI-agent sandboxes, set `TORRUST_GIT_HOOKS_LOG_DIR=.tmp` to keep temporary logs
inside the workspace for both hooks (`.tmp/` is git-ignored).
When using `.tmp`, periodically clean old logs (for example, remove stale `pre-commit-*.log` and
`pre-push-*.log` files) because OS-managed `/tmp` cleanup does not apply.

Gate ownership:

- Pre-commit: fast local feedback
- Pre-push: nightly toolchain checks + full stable test suite (no pre-commit duplicates; no E2E)
- CI: merge authority (includes E2E matrix)

Linter entry point:

```sh
linter all
```

Primary skill references:

- `run-linters`: `.github/skills/dev/git-workflow/run-linters/SKILL.md`
- `run-pre-commit-checks`: `.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md`
- `run-pre-push-checks`: `.github/skills/dev/git-workflow/run-pre-push-checks/SKILL.md`
- `setup-dev-environment`: `.github/skills/dev/maintenance/setup-dev-environment/SKILL.md`

Supporting docs:

- [docs/benchmarking.md](docs/benchmarking.md)
- [docs/profiling.md](docs/profiling.md)
- [docs/containers.md](docs/containers.md)

## 🎨 Code Style

- **rustfmt**: Format with `cargo fmt` before committing. Config: `rustfmt.toml`
  (`group_imports = "StdExternalCrate"`, `imports_granularity = "Module"`, `max_width = 130`).
- **Compile flags**: `.cargo/config.toml` enables strict global `rustflags` (`-D warnings`,
  `-D unused`, `-D rust-2018-idioms`, `-D future-incompatible`, and others). All code must
  compile cleanly with these flags — no suppressions unless absolutely necessary.
- **clippy**: No warnings allowed (`cargo clippy -- -D warnings`).
- **Imports**: All imports at the top of the file, grouped (std → external crates → internal
  crate). Prefer short imported names over fully-qualified paths
  (e.g., `Arc<MyType>` not `std::sync::Arc<crate::my::MyType>`). Use full paths only to
  disambiguate naming conflicts.
- **TOML**: Must pass `taplo fmt --check **/*.toml`. Auto-fix with `taplo fmt **/*.toml`.
- **Markdown**: Must pass markdownlint.
- **YAML**: Must pass `yamllint -c .yamllint-ci.yml`.
- **Spell checking**: Add new technical terms to `project-words.txt` (one word per line,
  alphabetical order).

## 🤝 Collaboration Principles

These rules apply repository-wide to every assistant, including custom agents.

When acting as an assistant in this repository:

- Do not flatter the user or agree with weak ideas by default.
- Push back when a request, diff, or proposed commit looks wrong.
- Flag unclear but important points before they become problems.
- Ask a clarifying question instead of making a random choice when the decision matters.
- Call out likely misses: naming inconsistencies, accidental generated files,
  staged-versus-unstaged mismatches, missing docs updates, or suspicious commit scope.

When raising a likely mistake or blocker, say so clearly and early instead of burying it after
routine status updates.

## 🧭 Engineering Policies

These policies are repository-wide and apply to all agents and workflows.

<!-- skill-link: add-rust-dependency -->

1. **Dependency freshness**: prefer the latest stable Rust crate version when adding or upgrading
   dependencies unless there is a compatibility reason not to. If not using the latest stable
   version, document why.
2. **Container base image freshness**: prefer current supported base images in `Containerfile`
   and compose artifacts. If an older base image is retained, document the reason.
3. **Shell vs Rust threshold**: use shell scripts for simple orchestration only. When logic
   becomes non-trivial, stateful, safety-critical, or worth testing independently, prefer Rust.
4. **Testing coverage and maintainability**: aim for high maintainable automated coverage. If
   behaviour is left untested, document why and prefer improving design/testability when practical.
5. **Rust documentation quality**: document public APIs and important internal module/type
   invariants. Prefer high-signal Rust docs over boilerplate comments.
6. **Documentation single source of truth**: avoid duplicating procedural guidance across docs.
   Keep folder READMEs lightweight (purpose and navigation), and treat `.github/skills/` plus
   canonical docs (for example `docs/index.md`) as the authoritative workflow sources.
   When duplications are found, remove or replace them with links to the canonical source.

Implementation workflow references:

- Dependency updates: `.github/skills/dev/maintenance/update-dependencies/SKILL.md`
- Adding a new Rust dependency: `.github/skills/dev/maintenance/add-rust-dependency/SKILL.md`
- Unit testing conventions: `.github/skills/dev/testing/write-unit-test/SKILL.md`

## 🔧 Essential Rules

1. **Linting gate**: `linter all` must exit `0` before every commit. No exceptions.
2. **GPG commit signing**: All commits **must** be signed with GPG (`git commit -S`).
3. **Never commit `storage/` or `target/`**: These directories contain runtime data and build
   artifacts. They are git-ignored; never force-add them.
4. **Unused dependencies**: Run `cargo machete` before committing. Remove any unused
   dependencies immediately.
5. **Rust imports**: All imports at the top of the file, grouped (std → external crates →
   internal crate). Prefer short imported names over fully-qualified paths.
6. **Continuous self-review**: Review your own work against project quality standards. Apply
   self-review at three levels:
   - **Mandatory** — before opening a pull request
   - **Strongly recommended** — before each commit
   - **Recommended** — after completing each small, independent, deployable change
7. **Security**: Do not report security vulnerabilities through public GitHub issues. Send an
   email to `info@nautilus-cyberneering.de` instead. See [SECURITY.md](SECURITY.md).
8. **Skill-link synchronization**: When modifying any artifact containing a `skill-link:` marker,
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

**Commit messages** follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
feat(<scope>): add X
fix(<scope>): resolve Y
chore(<scope>): update Z
docs(<scope>): document W
refactor(<scope>): restructure V
ci(<scope>): adjust pipeline U
test(<scope>): add tests for T
```

Scope should reflect the affected package or area (e.g., `tracker-core`, `udp-protocol`, `ci`, `docs`).

**Branch strategy**:

- Feature branches are cut from `develop`
- Direct pushes to `develop` and `main` are not allowed; changes must be merged via PR
- PRs targeting `develop` or `main` must come from a fork branch (`<fork-owner>:<branch>`), not a branch in `torrust/torrust-tracker`
- Remote names are contributor-local (`josecelano`, `origin`, `upstream`, `torrust`, etc.); do not assume fixed remote names
- PRs target `develop`
- `develop` → `staging/main` → `main` (release pipeline)
- PRs must pass all CI status checks before merge

See [docs/release_process.md](docs/release_process.md) for the full release workflow.

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

```sh
# Run the latest image
docker run -it torrust/tracker:latest
# or with Podman
podman run -it docker.io/torrust/tracker:latest

# Build and run via Docker Compose
docker compose up -d               # Start all services (detached)
docker compose logs -f tracker     # Follow tracker logs
docker compose down                # Stop and remove containers
```

**Volume mappings** (local `storage/` → container paths):

```text
./storage/tracker/lib  →  /var/lib/torrust/tracker
./storage/tracker/log  →  /var/log/torrust/tracker
./storage/tracker/etc  →  /etc/torrust/tracker
```

**Ports**: UDP tracker: `6969`, HTTP tracker: `7070`, REST API: `1212`

See [docs/containers.md](docs/containers.md) for detailed container documentation.

## 🎯 Auto-Invoke Skills

Agent Skills live under [`.github/skills/`](.github/skills/). Each skill is a `SKILL.md` file
with YAML frontmatter and Markdown instructions covering a repeatable workflow.

> Skills supplement (not replace) the rules in this file. Rules apply always; skills activate
> when their workflows are needed.

**For VS Code**: Enable `chat.useAgentSkills` in settings to activate skill discovery.

**Learn more**: See [Agent Skills Specification (agentskills.io)](https://agentskills.io/specification).

## 📚 Documentation

- [Documentation Index](docs/index.md)
- [Package Architecture](docs/packages.md)
- [Benchmarking](docs/benchmarking.md)
- [Profiling](docs/profiling.md)
- [Containers](docs/containers.md)
- [Release Process](docs/release_process.md)
- [ADRs](docs/adrs/README.md)
- [Issues / Implementation Plans](docs/issues/)
- [API docs (docs.rs)](https://docs.rs/torrust-tracker/)
- [Report a security vulnerability](SECURITY.md)

### Quick Navigation

| Task                                 | Start Here                                           |
| ------------------------------------ | ---------------------------------------------------- |
| Understand the architecture          | [`docs/packages.md`](docs/packages.md)               |
| Run the tracker in a container       | [`docs/containers.md`](docs/containers.md)           |
| Read all docs                        | [`docs/index.md`](docs/index.md)                     |
| Understand an architectural decision | [`docs/adrs/README.md`](docs/adrs/README.md)         |
| Read or write an issue spec          | [`docs/issues/`](docs/issues/)                       |
| Run benchmarks                       | [`docs/benchmarking.md`](docs/benchmarking.md)       |
| Run profiling                        | [`docs/profiling.md`](docs/profiling.md)             |
| Understand the release process       | [`docs/release_process.md`](docs/release_process.md) |
| Report a security vulnerability      | [`SECURITY.md`](SECURITY.md)                         |
| Agent skills reference               | [`.github/skills/`](.github/skills/)                 |
| Custom agents reference              | [`.github/agents/`](.github/agents/)                 |
