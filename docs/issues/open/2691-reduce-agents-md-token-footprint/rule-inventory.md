---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/2691-reduce-agents-md-token-footprint/ISSUE.md
    - docs/issues/open/2691-reduce-agents-md-token-footprint/section-disposition.md
    - AGENTS.md
---

<!-- skill-link: create-issue -->

# Rule Inventory: Root AGENTS.md Baseline

Issue #2691 completeness checklist. It lists every rule and fact in the **baseline** root
`AGENTS.md`, built from the original file before any restructuring, not from the list of planned
moves. A check seeded from the moves could only confirm those moves; this inventory is seeded
from the source.

- **Baseline:** `AGENTS.md` at commit `c86b1c24e` (`Merge torrust/torrust-tracker#2692: docs(issues):
  [#2691] add specification to reduce the AGENTS.md token footprint`), a commit on `develop`. Show it
  with `git show c86b1c24e:AGENTS.md`. The file is 523 lines and its MD5 is
  `f3516c788b19491443907ca3c30cd25e`.
- **Locator:** a distinctive phrase from the baseline text that identifies the rule, so a reviewer
  can find it in the baseline and in the final location. A moved rule may be reworded, so the
  reviewer confirms the meaning, not only the phrase.
- **Final location:** filled in as each row is processed in T6 and T7. Rows start as
  `PENDING`. A row is `ASSERTED` only after the final location was opened and confirmed to contain
  the rule (or, for `DELETE`, to have a recorded reason).
- **Final assertion (M4):** at the end, every row must be `ASSERTED`. A row without a final
  location, or a `DELETE` without a reason, fails the issue's acceptance criterion AC3.

Planned handling comes from [`section-disposition.md`](section-disposition.md) and the maintainer
decisions D1 to D5 recorded there.

| ID  | Baseline section | Rule or fact | Locator | Planned handling | Final location | Status |
| --- | ---------------- | ------------ | ------- | ---------------- | -------------- | ------ |
| R001 | Project Overview | Repository is torrust/torrust-tracker | `torrust/torrust-tracker` | KEEP |  | PENDING |
| R002 | Project Overview | Production-grade BitTorrent tracker; UDP, HTTP, TLS; IPv4/IPv6; private/whitelisted mode; REST API | `production-grade BitTorrent tracker` | KEEP (trim) |  | PENDING |
| R003 | Project Overview | Rust edition 2024, MSRV 1.88 | `edition 2024, MSRV 1.88` | KEEP |  | PENDING |
| R004 | Project Overview | MSRV policy once bittorrent-* crates are extracted (#1669) | `MSRV policy` | KEEP (trim) or MOVE |  | PENDING |
| R005 | Project Overview | License AGPL-3.0-only; version 3.0.0-develop | `AGPL-3.0-only` | KEEP |  | PENDING |
| R006 | Project Overview | Axum web framework, Tokio runtime, BEP 15/3/23, SQLite/MySQL/PostgreSQL, Cargo workspace | `Cargo workspace (multi-crate monorepo)` | DELETE (derivable) |  | PENDING |
| R007 | Tech Stack | Languages, Axum, Tokio, testcontainers, databases, Containerfile, GitHub Actions | `testcontainers (E2E)` | DELETE (derivable) |  | PENDING |
| R008 | Tech Stack | Linters unified under the `linter` binary from torrust/torrust-linting (markdownlint, lychee, yamllint, taplo, cspell, shellcheck, clippy, rustfmt) | `unified` | MOVE to `run-linters` skill or keep one line |  | PENDING |
| R009 | Key Directories | src/ holds main.rs, lib.rs, app.rs, container.rs; src/bin/ has http_health_check; E2E runners and profiling are in packages/e2e-tools | `src/bin/` | DELETE (derivable) |  | PENDING |
| R010 | Key Directories | src/bootstrap/ and src/console/ purpose | `src/bootstrap/` | DELETE (derivable) |  | PENDING |
| R011 | Key Directories | packages/ holds all domain logic; console/ holds console tools | `all domain logic lives here` | KEEP (trim) + link |  | PENDING |
| R012 | Key Directories | contrib/dev-tools/ holds git hooks, container scripts, init scripts and developer-tool crates | `contrib/dev-tools/` | KEEP (trim) |  | PENDING |
| R013 | Key Directories | tests/ integration tests; docs/ with adrs/ and issues/; share/default/ default config | `share/default/` | KEEP (trim) |  | PENDING |
| R014 | Key Directories | storage/ is runtime data and git-ignored | ``storage/` — Runtime data`` | KEEP |  | PENDING |
| R015 | Key Directories | .tmp/ is a git-ignored workspace temp dir for hook logs (TORRUST_GIT_HOOKS_LOG_DIR=.tmp) and benchmark isolation dirs | `.tmp/` | KEEP |  | PENDING |
| R016 | Key Directories | .github/workflows, .github/skills and .github/agents locations | `` `.github/skills/` `` | KEEP |  | PENDING |
| R017 | Package Catalog | List of workspace packages with crate names, layers, publish status | `Package Catalog` | DONE | `docs/packages.md#package-catalog` | ASSERTED |
| R018 | Package Catalog | Packages are versioned independently (ADR 20260629000000); `publish = false` rows are unpublished | `versioned independently` | DONE | `docs/packages.md#package-catalog` | ASSERTED |
| R019 | Package Catalog | Extracted packages (clock, located-error, metrics, net-primitives, server-lib) and their repositories | `Extracted packages` | DONE | `docs/packages.md#extracted-packages` | ASSERTED |
| R020 | Package Catalog | Console tool `torrust-tracker-client` (tracker_client, tracker_checker) | `Console tools` | DONE | `docs/packages.md#console-tools` | ASSERTED |
| R021 | Package Catalog | Workspace developer tools under contrib/dev-tools (eight crates) | `Workspace developer tools` | DONE | `docs/packages.md#workspace-developer-tools` | ASSERTED |
| R022 | Package Catalog | Skill-owned tool wait-for-pr-review and community contributions (none) | `Skill-owned tools` | DONE | `docs/packages.md` | ASSERTED |
| R023 | Package Catalog | `cargo metadata --no-deps --format-version 1` is the authoritative package list | `cargo metadata --no-deps --format-version 1` | DONE | `AGENTS.md` (pointer) | ASSERTED |
| R024 | Package Naming Conventions | Prefix table axum-*, *-server, *-core, *-protocol, rest-api-*, udp-*, http-* with responsibilities | `Package Naming Conventions` | DONE | `docs/packages.md#package-conventions`; pointer in `AGENTS.md` | ASSERTED |
| R025 | Key Configuration Files | `linter` has no configuration file of its own; it delegates to each tool, which reads its own config from the project root | `no configuration file of its own` | KEEP (trim) |  | PENDING |
| R026 | Key Configuration Files | Files in .gitignore are not automatically excluded from linting; each linter has its own ignore (.markdownlintignore, .cspell.gitignore) | `not** automatically excluded` | KEEP (trim) |  | PENDING |
| R027 | Key Configuration Files | Which config file each tool reads (.markdownlint.json, .yamllint-ci.yml, .taplo.toml, cspell.json, lychee.toml, project-words.txt) | `` `.yamllint-ci.yml` `` | MOVE to `run-linters` skill |  | PENDING |
| R028 | Key Configuration Files | rustfmt.toml settings (group_imports = StdExternalCrate, max_width = 130) | `max_width = 130` | MOVE to `run-linters` skill or DELETE (in rustfmt.toml) |  | PENDING |
| R029 | Key Configuration Files | .cargo/config.toml defines aliases (cov, cov-lcov, cov-html, time) and global rustflags (-D warnings, -D unused, -D rust-2018-idioms) | `Cargo aliases` | MOVE or DELETE (in the file) |  | PENDING |
| R030 | Key Configuration Files | Cargo.toml workspace root; compose.qbittorrent-e2e.*.yaml stacks; Containerfile; codecov.yaml | `compose.qbittorrent-e2e.sqlite3.yaml` | DELETE (derivable) |  | PENDING |
| R031 | Build, Test, and Lint | Quick policy-level summary; prefer the corresponding skills for command workflows | `policy-level summary` | KEEP (trim) |  | PENDING |
| R032 | Build, Test, and Lint | Common cargo commands: build, doc tests, all-targets tests, integration test, nightly doc builds, torrent-repository-benchmarking bench | `cargo test --doc --workspace` | MOVE to `docs/testing.md` or `tests/AGENTS.md` |  | PENDING |
| R033 | Build, Test, and Lint | Mandatory quality gate before every commit: ./contrib/dev-tools/git/hooks/pre-commit.sh | `Mandatory quality gate before every commit` | KEEP |  | PENDING |
| R034 | Build, Test, and Lint | Pre-commit runs nine fast local steps (dictionary, clippy allows, frontmatter, machete, deny, nightly fmt, linter all, hadolint, doc tests) | `` Run `cargo machete --with-metadata` `` | DELETE (script prints it) or LINK to `run-pre-commit-checks` |  | PENDING |
| R035 | Build, Test, and Lint | --format=text --verbosity=verbose for streaming output; --format=json for structured output | `--format=json` | MOVE to `run-pre-commit-checks` skill |  | PENDING |
| R036 | Build, Test, and Lint | Hooks write per-step logs to TORRUST_GIT_HOOKS_LOG_DIR (default /tmp); set it to .tmp in restricted AI-agent sandboxes; clean old logs periodically | `TORRUST_GIT_HOOKS_LOG_DIR=.tmp` | KEEP (trim) |  | PENDING |
| R037 | Build, Test, and Lint | Gate ownership: pre-commit fast feedback; pre-push nightly checks + packaging check + full stable tests (no E2E); CI is merge authority and includes E2E | `Gate ownership` | MOVE to `run-pre-push-checks` skill or KEEP (trim) |  | PENDING |
| R038 | Build, Test, and Lint | Linter entry point is `linter all` | `Linter entry point` | KEEP |  | PENDING |
| R039 | Build, Test, and Lint | Skill references: run-linters, run-pre-commit-checks, run-pre-push-checks, setup-dev-environment | `Primary skill references` | KEEP (trim) |  | PENDING |
| R040 | Build, Test, and Lint | Supporting docs: benchmarking, profiling, containers | `Supporting docs` | LINK (docs index) |  | PENDING |
| R041 | Code Style | Format with cargo fmt before committing; rustfmt config in rustfmt.toml | `` Format with `cargo fmt` `` | DELETE (enforced by linter) |  | PENDING |
| R042 | Code Style | Strict global rustflags (-D warnings, -D unused, -D rust-2018-idioms, -D future-incompatible); no suppressions unless absolutely necessary | `no suppressions unless absolutely necessary` | KEEP |  | PENDING |
| R043 | Code Style | clippy: no warnings allowed | `No warnings allowed` | DELETE (enforced by linter) |  | PENDING |
| R044 | Code Style | Imports at the top of the file, grouped std, external, internal; prefer short imported names; full paths only to disambiguate | `Prefer short imported names` | KEEP |  | PENDING |
| R045 | Code Style | TOML must pass taplo fmt --check; auto-fix with taplo fmt | `taplo fmt --check` | DELETE (enforced by linter) |  | PENDING |
| R046 | Code Style | Markdown must pass markdownlint and lychee local checks | `lychee` | DELETE (enforced by linter) |  | PENDING |
| R047 | Code Style | YAML must pass yamllint -c .yamllint-ci.yml | `yamllint -c .yamllint-ci.yml` | DELETE (enforced by linter) |  | PENDING |
| R048 | Code Style | Add new technical terms to project-words.txt, one per line, alphabetical | `project-words.txt` | KEEP (trim) |  | PENDING |
| R049 | Collaboration Principles | Do not flatter the user or agree with weak ideas by default | `Do not flatter the user` | KEEP |  | PENDING |
| R050 | Collaboration Principles | Push back when a request, diff or proposed commit looks wrong | `Push back when` | KEEP |  | PENDING |
| R051 | Collaboration Principles | Flag unclear but important points early | `Flag unclear but important points` | KEEP |  | PENDING |
| R052 | Collaboration Principles | Ask a clarifying question instead of making a random choice when the decision matters | `Ask a clarifying question` | KEEP |  | PENDING |
| R053 | Collaboration Principles | Undocumented process gate: stop after read-only investigation, identify the gap, get explicit maintainer approval before branches, edits, commits, pushes, GitHub artifacts, review replies or resolved threads | `explicit approval of the proposed process` | KEEP |  | PENDING |
| R054 | Collaboration Principles | Call out likely misses (naming, generated files, staged versus unstaged, missing docs, commit scope) | `Call out likely misses` | KEEP |  | PENDING |
| R055 | Collaboration Principles | Raise likely mistakes or blockers clearly and early | `clearly and early` | KEEP |  | PENDING |
| R056 | Engineering Policies | 1. Dependency freshness: prefer the latest stable crate; document exceptions | `Dependency freshness` | KEEP (trim) per D5 |  | PENDING |
| R057 | Engineering Policies | 2. Container base image freshness; document retained older images | `Container base image freshness` | KEEP (trim) per D5 |  | PENDING |
| R058 | Engineering Policies | 3. Shell vs Rust threshold: shell only for simple orchestration | `Shell vs Rust threshold` | KEEP (trim) per D5 |  | PENDING |
| R059 | Engineering Policies | 4. Testing coverage and maintainability: document untested behaviour | `Testing coverage and maintainability` | KEEP (trim) per D5 |  | PENDING |
| R060 | Engineering Policies | 5. Rust documentation quality for public APIs and invariants | `Rust documentation quality` | KEEP (trim) per D5 |  | PENDING |
| R061 | Engineering Policies | 6. Documentation single source of truth; folder READMEs stay lightweight; skills and docs/index.md are authoritative | `Documentation single source of truth` | KEEP (trim) per D5 |  | PENDING |
| R062 | Engineering Policies | 7. AI-agent implementation independence; provider-specific adapters are optional; link to the portability ADR | `AI-agent implementation independence` | KEEP (trim) per D5 |  | PENDING |
| R063 | Engineering Policies | 8. Repository-owned skill authority; warn on conflicts with external skills | `Repository-owned skill authority` | KEEP (trim) per D5 |  | PENDING |
| R064 | Engineering Policies | Implementation workflow references: update-dependencies, add-rust-dependency, write-unit-test, implement-workflow skills | `Implementation workflow references` | KEEP (trim) or MOVE to skills |  | PENDING |
| R065 | Essential Rules | `linter all` must exit 0 before every commit, no exceptions | ``must exit `0` before every commit`` | KEEP |  | PENDING |
| R066 | Essential Rules | All commits must be GPG-signed with git commit -S | `GPG commit signing` | KEEP |  | PENDING |
| R067 | Essential Rules | GPG timeout: stop the failed attempt, notify the user, ask whether to retry manually or rerun while the user enters the passphrase; never retry automatically, handle the passphrase in chat, use --no-gpg-sign, or skip signing; absolute rule | `This rule is absolute` | KEEP |  | PENDING |
| R068 | Essential Rules | Never commit or force-add storage/ or target/ | `` Never commit `storage/` or `target/` `` | KEEP |  | PENDING |
| R069 | Essential Rules | Run cargo machete before committing; remove unused dependencies immediately | `cargo machete` | KEEP (trim) |  | PENDING |
| R070 | Essential Rules | Rust imports at the top of the file, grouped; short names (duplicate of the Code Style import rule) | `**Rust imports**` | MERGE with Code Style rule |  | PENDING |
| R071 | Essential Rules | Continuous self-review at three levels: mandatory before opening a PR, strongly recommended before each commit, recommended after each small deployable change | `Continuous self-review` | KEEP |  | PENDING |
| R072 | Essential Rules | Report security vulnerabilities by email to info@nautilus-cyberneering.de, not public issues; see SECURITY.md | `info@nautilus-cyberneering.de` | KEEP |  | PENDING |
| R073 | Essential Rules | Skill-link synchronization: when changing an artifact with a skill-link marker, update the linked skill and run its validation script | `Skill-link synchronization` | KEEP |  | PENDING |
| R074 | Git Workflow | Branch naming: <issue-number>-<short-description>, feat/, fix/, chore/ | `<issue-number>-<short-description>` | KEEP |  | PENDING |
| R075 | Git Workflow | Spec-only branch uses a -spec suffix so the base name stays free for implementation | `-spec` | KEEP or MOVE to `create-feature-branch` skill |  | PENDING |
| R076 | Git Workflow | Conventional Commits types feat, fix, chore, docs, refactor, ci, test; scope names the package or area | `Conventional Commits` | KEEP (trim) |  | PENDING |
| R077 | Git Workflow | Feature branches are cut from develop | `` cut from `develop` `` | KEEP |  | PENDING |
| R078 | Git Workflow | Direct pushes to develop and main are not allowed; changes merge via PR | ``Direct pushes to `develop` and `main` are not allowed`` | KEEP |  | PENDING |
| R079 | Git Workflow | PRs into develop or main must come from a fork branch, not a branch in torrust/torrust-tracker | `must come from a fork branch` | KEEP |  | PENDING |
| R080 | Git Workflow | Remote names are contributor-local; do not assume fixed remote names | `Remote names are contributor-local` | KEEP (trim) |  | PENDING |
| R081 | Git Workflow | PRs target develop; develop to staging/main to main is the release pipeline; PRs must pass all CI checks | `staging/main` | KEEP (trim) or MOVE to release docs |  | PENDING |
| R082 | Git Workflow | Keep intermediate commits: do not squash, fixup or rewrite history to hide superseded steps; never propose squashing as cleanup | `Keep intermediate commits` | KEEP (no other home) |  | PENDING |
| R083 | Git Workflow | Rebase onto develop is recommended when cheap and required on conflicts (ADR 20261013161419) | `stop_requiring_pull_request_rebases` | MOVE to `open-pull-request` skill (verify) or KEEP one line |  | PENDING |
| R084 | Git Workflow | Other history rewrites are allowed but not ideal; prefer a new commit; keep every step; confirm the tree is unchanged; push with --force-with-lease; replace commit ids with subjects; note the rewrite in the PR body; never rewrite develop, main or merged history | `--force-with-lease` | MOVE to `open-pull-request` skill (verify) |  | PENDING |
| R085 | Git Workflow | Minor process, skill or docs improvements may ride along in the PR that produced them, each in its own commit; substantial changes get their own issue and PR | `may ride along` | KEEP (trim) or MOVE |  | PENDING |
| R086 | Git Workflow | PR title is a single Conventional Commits line, without the PR number; include [#issue] only for the issue the PR implements or closes; the merge tool copies the title into the merge subject | `copies the PR title verbatim` | MOVE to `open-pull-request` skill (verify) |  | PENDING |
| R087 | Git Workflow | Release process is documented in docs/release_process.md | `release_process.md` | LINK |  | PENDING |
| R088 | Development Principles | Core principles: observability, testability, modularity, extensibility | `Observability` | KEEP per D4 |  | PENDING |
| R089 | Development Principles | Code quality standards: clean, maintainable, readable, testable | `Clean` | KEEP per D4 |  | PENDING |
| R090 | Development Principles | Beck's four rules of simple design in priority order | `Beck` | KEEP per D4 |  | PENDING |
| R091 | Container / Docker | Commands to run the latest image with Docker or Podman and to use Docker Compose | `docker compose up -d` | DONE | `docs/containers.md` (run: lines 21 and 27; compose up and down: lines 509 and 528) | ASSERTED |
| R092 | Container / Docker | Volume mappings from storage/ to container paths | `/var/lib/torrust/tracker` | DONE | `docs/containers.md#volumes` | ASSERTED |
| R093 | Container / Docker | Ports: UDP 6969, HTTP 7070, REST API 1212 | `` UDP tracker: `6969` `` | DONE | `docs/containers.md` (default HTTP_PORT, API_PORT and UDP_PORT; publish examples) | ASSERTED |
| R094 | Auto-Invoke Skills | Skills live under .github/skills as SKILL.md files; skills supplement the rules and activate when their workflows are needed | `Agent Skills live under` | KEEP (trim) |  | PENDING |
| R095 | Auto-Invoke Skills | For VS Code enable chat.useAgentSkills; link to the Agent Skills specification | `chat.useAgentSkills` | DELETE or MOVE to `setup-dev-environment` |  | PENDING |
| R096 | Documentation | Links to documentation index, package architecture, benchmarking, profiling, containers, release process, ADRs, issues, docs.rs | `Documentation Index` | LINK (docs index) |  | PENDING |
| R097 | Documentation | Quick-navigation table mapping tasks to starting documents | `Quick Navigation` | LINK (docs index) |  | PENDING |
| R098 | Documentation | Report security vulnerabilities through SECURITY.md | `Report a security vulnerability` | KEEP |  | PENDING |
| R099 | Documentation | Custom agents reference in .github/agents/README.md | `Custom agents reference` | KEEP (trim) |  | PENDING |

Total rows: 99.
