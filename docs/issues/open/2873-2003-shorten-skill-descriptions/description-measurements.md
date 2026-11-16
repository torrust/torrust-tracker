---
last-updated-utc: "2026-11-16 17:50"
semantic-links:
  related-artifacts:
    - docs/issues/open/2873-2003-shorten-skill-descriptions/ISSUE.md
    - docs/issues/open/2873-2003-shorten-skill-descriptions/manual-verification-evidence.md
    - .claude/skills/README.md
---

# Skill Description Measurements

Issue #2873 shortens the `description` of each project skill. This file keeps, for every skill,
the description before and after, its length and the tokens its `/context` row costs, so a lost
trigger phrase can be restored and the saving checked.

## How the Numbers Were Taken

- **Characters:** the YAML-parsed `description` (Python `yaml.safe_load` of the frontmatter, then
  `len`), the text a YAML parser reads.
- **Tokens:** the skill's row in the Skills table of `claude -p "/context"`, run in a fresh
  checkout of the commit (no parent `CLAUDE.md`), with Claude Code 2.1.318 and its default model
  `claude-sonnet-5-5`, 1M-token window. `/context` rounds each row to the nearest 10 tokens.
- **Before:** `develop` `9e463cb0c`, the merge of #2866's PR #2886, on 2026-11-16 at 17:35 UTC.
  The Skills category was 9.9k tokens (1.0%, its cap), with 53 Project rows.
- **After:** filled in by task T6.

`write-markdown-docs` is 123 characters before because YAML cuts its 578-character description
at a comment (see `manual-verification-evidence.md`, V0).

## Per Skill

| Area | Skill | Before (chars) | Before (tokens) | After (chars) | After (tokens) |
| ---- | ----- | -------------: | --------------: | ------------: | -------------: |
| Skill authoring | `add-new-skill` | 326 | 110 | 171 |  |
| Benchmarking | `run-benchmarks` | 724 | 250 | 187 |  |
| CI | `implement-workflow` | 151 | 60 | 155 |  |
| CI | `watch-pull-request-checks` | 352 | 130 | 155 |  |
| Debugging | `fix-bug` | 337 | 120 | 170 |  |
| Environment setup | `run-tracker-locally` | 334 | 120 | 161 |  |
| Git workflow | `commit-changes` | 399 | 140 | 123 |  |
| Git workflow | `create-feature-branch` | 359 | 130 | 127 |  |
| Git workflow | `merge-pull-request` | 222 | 80 | 147 |  |
| Git workflow | `open-pull-request` | 306 | 110 | 183 |  |
| Git workflow | `push-changes` | 329 | 120 | 178 |  |
| Git workflow | `release-new-version` | 377 | 130 | 157 |  |
| Git workflow | `release-workspace-package` | 358 | 130 | 146 |  |
| Git workflow | `run-linters` | 397 | 140 | 176 |  |
| Git workflow | `run-pre-commit-checks` | 356 | 130 | 128 |  |
| Git workflow | `run-pre-push-checks` | 355 | 130 | 138 |  |
| GitHub | `link-subissue-to-parent-issue` | 483 | 170 | 158 |  |
| Logging | `structured-runtime-logging` | 204 | 80 | 178 |  |
| Maintenance | `add-rust-dependency` | 352 | 130 | 169 |  |
| Maintenance | `add-workspace-member` | 265 | 100 | 155 |  |
| Maintenance | `catalog-security-vulnerabilities` | 457 | 160 | 170 |  |
| Maintenance | `check-runner-health` | 478 | 170 | 163 |  |
| Maintenance | `install-linter` | 346 | 120 | 128 |  |
| Maintenance | `run-manual-docker-security-scan` | 289 | 110 | 144 |  |
| Maintenance | `setup-dev-environment` | 400 | 140 | 180 |  |
| Maintenance | `triage-github-security-findings` | 370 | 140 | 173 |  |
| Maintenance | `update-dependencies` | 439 | 150 | 168 |  |
| Maintenance | `update-github-workflow-actions` | 238 | 90 | 164 |  |
| Planning | `cleanup-completed-issues` | 711 | 250 |  |  |
| Planning | `create-adr` | 452 | 160 |  |  |
| Planning | `create-issue` | 455 | 160 |  |  |
| Planning | `create-markdown-template` | 417 | 150 |  |  |
| Planning | `create-refactor-plan` | 479 | 170 |  |  |
| Planning | `reopen-issue` | 555 | 190 |  |  |
| Planning | `write-markdown-docs` | 123 | 50 | 578 |  |
| Pull request reviews | `fetch-review-threads` | 425 | 150 |  |  |
| Pull request reviews | `process-copilot-suggestions` | 127 | 50 |  |  |
| Pull request reviews | `process-pr-review` | 392 | 140 |  |  |
| Pull request reviews | `process-pr-review-feedback` | 116 | 50 |  |  |
| Pull request reviews | `resolve-review-threads` | 380 | 140 |  |  |
| Pull request reviews | `review-pr` | 224 | 80 |  |  |
| Pull request reviews | `wait-for-pr-review` | 418 | 150 |  |  |
| Rust code quality | `fix-clippy-warnings` | 348 | 120 |  |  |
| Rust code quality | `handle-errors-in-code` | 613 | 210 |  |  |
| Rust code quality | `handle-secrets` | 541 | 190 |  |  |
| Task reviews | `review-task` | 216 | 80 |  |  |
| Testing | `manual-http-download-completion-e2e` | 396 | 150 |  |  |
| Testing | `manual-torrent-cleanup-e2e` | 309 | 110 |  |  |
| Testing | `manual-udp-download-completion-e2e` | 364 | 140 |  |  |
| Testing | `public-trackers-for-testing` | 328 | 120 |  |  |
| Testing | `write-unit-test` | 383 | 130 |  |  |
| Using the tracker | `use-rest-api` | 333 | 120 |  |  |
| Using the tracker | `use-tracker-client` | 381 | 130 |  |  |
| **Total** | 53 skills | **19489** | **6980** |  |  |

## Description Texts

### Skill authoring

#### `add-new-skill`

- Before (326): Guide for creating effective Agent Skills for the torrust-tracker project. Use when you need to create a new skill (or update an existing skill) that extends AI agent capabilities with specialized knowledge, workflows, or tool integrations. Triggers on "create skill", "add new skill", "how to add skill", or "skill creation".
- After (171): Create or update an Agent Skill in .claude/skills: its folder, frontmatter, a short description and its index row. Use when adding a new skill or changing an existing one.

### Benchmarking

#### `run-benchmarks`

- Before (724): Choose, run, and record Torrust Tracker benchmarks, including before-and-after performance evidence for changes to performance-critical code. Covers the benchmark levels (Criterion microbenchmarks per package, the persistence benchmark, the aquatic UDP load test, comparative and workflow benchmarks), how to pick one, how to measure a baseline before a change, and how to check that a benchmark measures anything. Use when a change touches the UDP or HTTP request path or other hot code, when an issue needs performance evidence, or when asked to benchmark. Triggers on "benchmark", "run benchmarks", "performance regression", "measure performance", "before and after performance", "load test", "aquatic", or "cargo bench".
- After (187): Choose, run and record benchmarks (Criterion, persistence, aquatic UDP load test) with before-and-after numbers. Use when a change touches hot code or an issue needs performance evidence.

### CI

#### `implement-workflow`

- Before (151): Use when: adding or materially changing GitHub Actions workflows, CI jobs, workflow scripts, artifacts, matrices, permissions, or provider integration.
- After (155): Add or change GitHub Actions workflows, CI jobs, workflow scripts and their permissions. Use when editing .github/workflows or changing what a CI job does.

#### `watch-pull-request-checks`

- Before (352): Wait for the CI checks of a pull request to finish with the GitHub CLI, then read which ones failed. Use after pushing to a pull request when you need to know whether CI passed, instead of asking the contributor or polling by hand. Triggers on "watch CI", "wait for checks", "wait for CI", "did CI pass", "gh pr checks", or "watch pull request checks".
- After (155): Wait for a pull request's CI checks to finish with gh pr checks, then read which failed. Use after pushing to a PR when you need to know whether CI passed.

### Debugging

#### `fix-bug`

- Before (337): Canonical workflow for investigating and fixing bugs in torrust-tracker. Use when work is substantively a bug, even if issue metadata or labels are missing or wrong. Requires source analysis, real-artifact reproduction or infeasibility evidence, regression-test selection, red/green validation, code fix, and like-for-like final recheck.
- After (170): Investigate and fix a bug: analysis, reproduction, a regression test seen red, the fix, then a like-for-like recheck. Use when the work is a bug, whatever its labels say.

### Environment setup

#### `run-tracker-locally`

- Before (334): Run the Torrust Tracker locally for development and testing. Use this skill to start the tracker with default configuration, understand configuration loading, and interact with tracker services (UDP and HTTP). Triggers on "run tracker", "start tracker locally", "develop tracker", "test tracker locally", or "run tracker for testing".
- After (161): Run the tracker locally with the default configuration and reach its UDP, HTTP and API services. Use when starting the tracker for development or manual testing.

### Git workflow

#### `commit-changes`

- Before (399): Guide for committing changes in the torrust-tracker project. Covers conventional commit format, pre-commit verification checklist, GPG signing, and commit quality guidelines. Use when committing code, running pre-commit checks, or following project commit standards. Triggers on "commit", "commit changes", "how to commit", "pre-commit", "commit message", "commit format", or "conventional commits".
- After (123): Commit changes as signed Conventional Commits after the pre-commit checks. Use when committing or writing a commit message.

#### `create-feature-branch`

- Before (359): Guide for creating feature branches following the torrust-tracker branching conventions. Covers branch naming format, lifecycle, and common patterns. Use when creating branches for issues, starting work on tasks, or setting up development branches. Triggers on "create branch", "new branch", "checkout branch", "branch for issue", or "start working on issue".
- After (127): Create a feature branch named by the project's branch conventions. Use when starting work on an issue or creating a new branch.

#### `merge-pull-request`

- Before (222): Safely construct, inspect, validate, sign, and optionally push a maintainer GitHub pull-request merge using the repository-local vendored tool. Use when asked to merge a pull request or perform a maintainer merge workflow.
- After (147): Build, inspect, sign and optionally push a maintainer merge of a pull request with the vendored merge tool. Use when asked to merge a pull request.

#### `open-pull-request`

- Before (306): Open a pull request from a feature branch using GitHub CLI (preferred) or GitHub MCP tools. Covers pre-flight checks, correct base/head configuration for fork workflows, title/body conventions, and post-creation validation. Use when asked to "open PR", "create pull request", or "submit branch for review".
- After (183): Open a pull request from a feature branch with the GitHub CLI: fork base and head, title and body conventions, checks after. Use when asked to open a PR or submit a branch for review.

#### `push-changes`

- Before (329): Guide for pushing commits in the torrust-tracker project. Covers the push workflow, pre-push hook setup, and the SSH idle-timeout problem that can interrupt pushes when the pre-push hook runs long. Triggers on "push changes", "git push", "how to push", "push branch", "SSH timeout on push", or "Connection closed by remote host".
- After (178): Push commits through the pre-push hook, and avoid the SSH idle timeout when the hook runs long. Use when pushing a branch or a push fails with "Connection closed by remote host".

#### `release-new-version`

- Before (377): Guide for releasing a new version of the Torrust Tracker using the standard staging branch, tag, and crate publication workflow. Covers version bump, release commit, staging branch promotion, PR to main, release branch/tag creation, crate publication, and merge-back to develop. Use when asked to "release", "cut a version", "publish a new version", or "create release vX.Y.Z".
- After (157): Release a new tracker version: version bump, staging branch, PR to main, tag, crate publication, merge-back. Use when asked to release or cut version vX.Y.Z.

#### `release-workspace-package`

- Before (358): Guide for releasing a single workspace package, or a crate that lives in its own standalone Torrust repository, to crates.io. Drives the process documented in docs/release-process/workspace-packages.md. Use when asked to "release a package", "publish a crate", "publish torrust-<crate> to crates.io", "tag a package release", or "release torrust-server-lib".
- After (146): Release one workspace package, or a crate in its own Torrust repository, to crates.io. Use when asked to publish a crate or tag a package release.

#### `run-linters`

- Before (397): Run code quality checks and linters for the torrust-tracker project. Includes Rust clippy, rustfmt, Markdown, local link checking, YAML, TOML, spell checking, and shellcheck. Use when asked to lint code, check formatting, fix code quality issues, or prepare for commit. Triggers on "lint", "run linters", "check code quality", "fix formatting", "run clippy", "run rustfmt", or "pre-commit checks".
- After (176): Run the linters (clippy, rustfmt, Markdown, links, YAML, TOML, cspell, shellcheck) with the linter binary. Use when linting, fixing formatting, or checking one tool's findings.

#### `run-pre-commit-checks`

- Before (356): Run all mandatory pre-commit verification steps for the torrust-tracker project. Covers the pre-commit script (automated checks), manual review steps, and individual linter commands for debugging. Use before any commit or PR to ensure all quality gates pass. Triggers on "pre-commit checks", "run all checks", "verify before commit", or "check everything".
- After (128): Run every check of the pre-commit hook, or one step alone to debug it. Use before committing, or when the pre-commit hook fails.

#### `run-pre-push-checks`

- Before (355): Run all mandatory pre-push verification steps for the torrust-tracker project. Covers the pre-push script (automated checks), output modes, and log-directory configuration. Use before pushing or when running the nightly toolchain checks and the full stable test suite. Triggers on "pre-push checks", "run pre-push", "verify before push", or "push checks".
- After (138): Run the pre-push checks: the nightly toolchain checks and the full stable test suite. Use before pushing, or when the pre-push hook fails.

### GitHub

#### `link-subissue-to-parent-issue`

- Before (483): Guide for linking an existing GitHub issue as a sub-issue of a parent issue in the torrust-tracker project. Covers the GitHub REST API flow, the required internal issue ID for the child issue, verification, and common failure modes. Use when setting a parent issue for a sub-issue, attaching a child issue to an epic, or linking an existing issue under another issue. Triggers on "set parent issue", "link subissue", "add sub-issue", "attach child issue", or "make issue a subissue".
- After (158): Link an existing GitHub issue as a sub-issue of a parent issue or EPIC through the REST API. Use when setting a parent issue or adding a sub-issue to an EPIC.

### Logging

#### `structured-runtime-logging`

- Before (204): Use when adding or changing logs for runtime service identity, service startup, listener bindings, or tracing instrumentation. Prefer explicit structured tracing fields over Rust Debug-formatted metadata.
- After (178): Write runtime logs with structured tracing fields, not Debug-formatted values. Use when adding or changing logs for service startup, listener bindings or tracing instrumentation.

### Maintenance

#### `add-rust-dependency`

- Before (352): Guide for safely adding a new Rust crate dependency in torrust-tracker, starting from the latest stable crates.io version, minimizing features, documenting version rationale, and validating with cargo machete and repository quality gates. Use when introducing a new dependency, selecting a crate version, or justifying why an older version is required.
- After (169): Add a Rust crate dependency: latest stable version, minimal features, a recorded version reason, cargo machete. Use when adding a dependency or choosing a crate version.

#### `add-workspace-member`

- Before (265): Add or remove an explicit Cargo workspace member in Torrust Tracker. Use when editing the root workspace members list, adding a developer-tool crate, registering a new standalone workspace package, or deciding whether a package is tested inside the container image.
- After (155): Add or remove a Cargo workspace member in the root members list. Use when adding a tool or package crate, or deciding whether the container image tests it.

#### `catalog-security-vulnerabilities`

- Before (457): Guide for cataloging public container, dependency, and CVE warnings that do not affect the project, with escalation for affecting vulnerabilities. Use for Docker DX, Trivy, cargo audit, RustSec, dependency advisories, CVE analysis, non-affecting CVEs, or container CVEs. GitHub Code Scanning and Code Quality findings, including Trivy alerts, are inventoried and clustered by triage-github-security-findings; this skill records the CVE analysis behind them.
- After (170): Record the CVE analysis of container and dependency warnings (Trivy, cargo audit, RustSec), and escalate any that affect the project. Use when checking a CVE or advisory.

#### `check-runner-health`

- Before (478): Check the self-hosted CI runner (torrust-runner-01) for CPU load, memory, disk space, and whether the automatic Docker cleanup works, then write a dated report under docs/self-hosted-runner-reports/ and open a pull request. Use for periodic runner health checks, disk-full or queued-job suspicions, or when asked to report on the self-hosted runner. Triggers on "runner health", "runner status report", "check self-hosted runner", "runner disk space", or "runner cleanup check".
- After (163): Check the self-hosted CI runner (load, memory, disk, Docker cleanup) and record a dated report in a PR. Use for periodic runner checks, a full disk or queued jobs.

#### `install-linter`

- Before (346): Install the torrust-linting `linter` binary and its external tool dependencies. Use when setting up a new development environment, after a fresh clone, or when the `linter` binary is missing. Triggers on "install linter", "setup linter", "linter not found", "install torrust-linting", "missing linter binary", or "set up development environment".
- After (128): Install the torrust-linting linter binary and the tools it calls. Use after a fresh clone or when the linter command is missing.

#### `run-manual-docker-security-scan`

- Before (289): Guide for running a manual Docker security scan for the tracker runtime image and documenting results. Covers build, Trivy scan, CVE triage, per-CVE catalog updates, and scan history updates. Use when asked to run a manual container scan, triage Docker CVEs, or refresh security scan docs.
- After (144): Build the tracker container image, scan it with Trivy, triage its CVEs and record the results. Use when asked for a manual Docker security scan.

#### `setup-dev-environment`

- Before (400): Set up a local development environment for torrust-tracker from scratch. Covers system dependencies, Rust toolchain, storage directories, linter binary, git hooks, and smoke tests. Use when onboarding to the project, setting up a new machine, or after a fresh clone. Triggers on "setup dev environment", "fresh clone", "onboarding", "install dependencies", "set up environment", or "getting started".
- After (180): Set up a development environment from scratch: system packages, Rust toolchain, storage folders, linter, git hooks, smoke tests. Use when onboarding or on a fresh machine or clone.

#### `triage-github-security-findings`

- Before (370): Review and cluster public GitHub Security tab findings from Code Scanning and Code Quality, classify affecting and non-affecting results, maintain the finding inventory, and create focused issue specs by root cause. Use when reviewing security alerts, CodeQL alerts, GitHub code scanning, code quality findings, workflow permission findings, or recurring scanner triage.
- After (173): Triage GitHub Security tab findings (code scanning, CodeQL, code quality), keep the finding inventory and file issue specs by root cause. Use when reviewing security alerts.

#### `update-dependencies`

- Before (439): Guide for updating project dependencies in the torrust-tracker project. Covers the manual cargo update workflow including branch creation, running checks, committing, and pushing. Distinguishes trivial updates (Cargo.lock only) from breaking-change updates (code rework needed). Use when updating dependencies, running cargo update, or bumping deps. Triggers on "update dependencies", "cargo update", "update deps", or "bump dependencies".
- After (168): Update Cargo dependencies with cargo update, telling Cargo.lock-only updates from breaking ones, then check, commit and push. Use when updating or bumping dependencies.

#### `update-github-workflow-actions`

- Before (238): Update GitHub Actions workflow dependencies safely in Torrust Tracker, including synchronizing the Torrust organization allowlist. Use when updating workflow action versions, Dependabot GitHub Actions updates, or allowed-actions settings.
- After (164): Update the GitHub Actions that workflows use and keep the Torrust organization allowlist in sync. Use for action version bumps or Dependabot GitHub Actions updates.

### Planning

#### `cleanup-completed-issues`

- Before (711): Guide for archiving closed issue specification files from docs/issues/open/ to docs/issues/closed/. Covers verifying closure on GitHub, moving files, updating frontmatter, auditing and repairing affected documentation links, creating a branch, and opening a PR. Permanent deletion of closed specs is not automated — the user must explicitly request it. Also covers a read-only periodic audit that checks docs/issues/open/ holds only open issues. Use when cleaning up closed issue specs, archiving issue docs, auditing docs/issues/, or maintaining the docs/issues/ folder. Triggers on "cleanup issue", "archive issue", "move closed issue", "clean completed issues", "audit issue specs", or "maintain issue docs".
- After: not yet rewritten

#### `create-adr`

- Before (452): Guide for creating Architectural Decision Records (ADRs) in the torrust-tracker project. Covers decision-scope placement, timestamp-based names, free-form structure, collection-specific index registration, and commit workflow. Use when documenting architectural decisions, recording design choices, or adding decision records. Triggers on "create ADR", "add ADR", "new decision record", "architectural decision", "document decision", or "add decision".
- After: not yet rewritten

#### `create-issue`

- Before (455): Guide for creating GitHub issues in the torrust-tracker project. Covers the full workflow from specification drafting, user review, to GitHub issue creation with proper documentation and file naming. Supports task, bug, feature, and epic issue types. Use when creating issues, opening tickets, filing bugs, proposing tasks, or adding features. Triggers on "create issue", "open issue", "new issue", "file bug", "add task", "create epic", or "open ticket".
- After: not yet rewritten

#### `create-markdown-template`

- Before (417): Create or maintain reusable Markdown document templates in the torrust-tracker repository. Use when creating a Markdown template, extracting a reusable template from documentation, adding a document skeleton, or deciding whether template-like Markdown belongs in docs/templates. Triggers on "create markdown template", "add markdown template", "document template", "extract template", or "reusable Markdown template".
- After: not yet rewritten

#### `create-refactor-plan`

- Before (479): Guide for creating refactor plans in the torrust-tracker project. Covers identifying quality gaps, decomposing them into trackable items ordered by impact vs effort, writing the plan document, and committing it. Use when planning improvements to readability, testability, maintainability, modularity, or documentation quality. Triggers on "create refactor plan", "refactor plan", "plan refactor", "post-implementation improvements", "code quality plan", or "technical debt plan".
- After: not yet rewritten

#### `reopen-issue`

- Before (555): Guide for reopening a closed GitHub issue or EPIC whose specification was archived in docs/issues/closed/. Covers deciding whether to reopen or file new work, reopening on GitHub, linking a new subissue, moving the spec back to docs/issues/open/, migrating legacy frontmatter, resetting closure checkpoints, and repairing live references. Use when new work belongs to a closed EPIC or a closed issue regressed or was incomplete. Triggers on "reopen issue", "reopen EPIC", "add subissue to closed EPIC", "unarchive issue spec", or "move spec back to open".
- After: not yet rewritten

#### `write-markdown-docs`

- Before (123): Guide for writing Markdown documentation in this project. Covers GitHub Flavored Markdown pitfalls, especially the critical
- Before, as written in the file (578), which YAML cuts at the space before `#NUMBER`: Guide for writing Markdown documentation in this project. Covers GitHub Flavored Markdown pitfalls, especially the critical #NUMBER pattern that auto-links to GitHub issues and PRs (NEVER use #1, #2, #3 as step/list numbers). Use ordered lists or plain numbers instead. Covers intentional vs accidental autolinks for issues, @mentions, and commit SHAs. Use when writing .md files, documentation, issue descriptions, PR descriptions, or README updates. Triggers on "markdown", "write docs", "documentation", "#number", "github markdown", "autolink", "markdown pitfall", or "GFM".
- After (578): Guide for writing Markdown documentation in this project. Covers GitHub Flavored Markdown pitfalls, especially the critical #NUMBER pattern that auto-links to GitHub issues and PRs (NEVER use #1, #2, #3 as step/list numbers). Use ordered lists or plain numbers instead. Covers intentional vs accidental autolinks for issues, @mentions, and commit SHAs. Use when writing .md files, documentation, issue descriptions, PR descriptions, or README updates. Triggers on "markdown", "write docs", "documentation", "#number", "github markdown", "autolink", "markdown pitfall", or "GFM".

### Pull request reviews

#### `fetch-review-threads`

- Before (425): Fetch every GitHub pull request review thread, including resolved and outdated ones, for the torrust-tracker project. Use when asked to find open PR review threads, list review comments, collect thread IDs before resolving suggestions, audit earlier resolutions, or inspect Copilot review feedback. Triggers on "fetch review threads", "list unresolved PR comments", "get review thread IDs", or "find open review suggestions".
- After: not yet rewritten

#### `process-copilot-suggestions`

- Before (127): Deprecated compatibility redirect for Copilot pull-request review suggestions. Use the unified process-pr-review skill instead.
- After: not yet rewritten

#### `process-pr-review`

- Before (392): Process every pull-request review finding, regardless of whether it was authored by Copilot, a person, or another bot. Use when asked to process PR review feedback, resolve review threads, audit review comments, address Copilot and maintainer findings together, triage review feedback submitted after merge, or handle a review left on a closed pull request that another pull request replaced.
- After: not yet rewritten

#### `process-pr-review-feedback`

- Before (116): Deprecated compatibility redirect for pull-request review feedback. Use the unified process-pr-review skill instead.
- After: not yet rewritten

#### `resolve-review-threads`

- Before (380): Resolve addressed GitHub pull request review threads for the torrust-tracker project. Use when asked to mark PR suggestions as resolved, resolve review comments, close addressed review threads, or clear Copilot review feedback after fixes are pushed. Triggers on "resolve PR threads", "mark suggestions as resolved", "resolve review comments", or "close addressed review threads".
- After: not yet rewritten

#### `review-pr`

- Before (224): Review an existing pull request for the torrust-tracker project. Covers checklist-based PR quality verification, code style standards, test requirements, documentation, and review feedback. Use only when a PR already exists.
- After: not yet rewritten

#### `wait-for-pr-review`

- Before (418): Wait for pull-request reviewers without spending tokens, using a deterministic Rust poller, then run the review loop on each event. Use after opening or updating a pull request when you must wait for Copilot or a maintainer review, instead of being told to wait. Triggers on "wait for review", "wait for da2ce7", "wait for Copilot review", "poll the PR", "block until reviewed", "review loop", or "wait-for-pr-review".
- After: not yet rewritten

### Rust code quality

#### `fix-clippy-warnings`

- Before (348): Guide for fixing Rust Clippy warnings in the torrust-tracker project. Covers proper application of clippy suggestions, when to add allowances, and how to document exceptions. Use when asked to fix clippy warnings, improve code quality, or resolve linter issues. Triggers on "fix clippy", "clippy warnings", "rust code quality", or "linting issues".
- After: not yet rewritten

#### `handle-errors-in-code`

- Before (613): Guide for error handling in this Rust project. Covers the four principles (clarity, context, actionability, explicit enums over anyhow), the thiserror pattern for structured errors, including what/where/when/why context, writing actionable help text, and avoiding vague errors. Also covers the located-error package for errors with source location. Use when writing error types, handling Results, adding error variants, or reviewing error messages. Triggers on "error handling", "error type", "Result", "thiserror", "anyhow", "error enum", "error message", "handle error", "add error variant", or "located-error".
- After: not yet rewritten

#### `handle-secrets`

- Before (541): Guide for handling sensitive data (secrets) in this Rust project. NEVER use plain String for API tokens, passwords, or other credentials. Use the current stable secrecy crate's direct secret types to prevent accidental exposure through Debug output, logs, and error messages. Call .expose_secret() only when the actual value is needed. Use when working with credentials, API keys, tokens, passwords, or any sensitive configuration. Triggers on "secret", "API token", "password", "credential", "sensitive data", "secrecy", or "expose secret".
- After: not yet rewritten

### Task reviews

#### `review-task`

- Before (216): Review a completed implementation task before push/PR. Validates issue-spec acceptance criteria, scope, tests, docs, and lint readiness on a local branch. Use when asked to verify issue completion without an open PR.
- After: not yet rewritten

### Testing

#### `manual-http-download-completion-e2e`

- Before (396): Manual end-to-end verification of started -> completed peer lifecycle using the HTTP tracker announce/scrape endpoints with curl (or browser for stats). Use when contributors want a fast, transparent simulation of download completion without containerized clients. Triggers on "manual http e2e", "http announce completed test", "simulate completion with curl", or "verify completed counter http".
- After: not yet rewritten

#### `manual-torrent-cleanup-e2e`

- Before (309): Manually verify that the tracker removes inactive peers while retaining a peerless torrent when configured to do so. Use when testing torrent cleanup, inactive peer expiration, or shutdown behavior of the cleanup runner. Triggers on "torrent cleanup test", "inactive peer expiration", or "manual cleanup e2e".
- After: not yet rewritten

#### `manual-udp-download-completion-e2e`

- Before (364): Manual end-to-end verification of started -> completed peer lifecycle using tracker_client (unified) and tracker stats API. Use when contributors need to simulate a peer completing a download without running containerized qBittorrent E2E. Triggers on "manual e2e", "simulate peer completion", "udp started completed test", or "verify downloads increment manually".
- After: not yet rewritten

#### `public-trackers-for-testing`

- Before (328): Public tracker targets for manual testing and debugging of tracker clients. Use when validating announce/scrape behavior against live services, comparing local vs public behavior, or diagnosing network timeouts. Triggers on "public tracker", "test against demo tracker", "debug tracker timeout", or "which tracker should I use".
- After: not yet rewritten

#### `write-unit-test`

- Before (383): Guide for writing unit tests following project conventions including behavior-driven naming (it*should*\*), AAA pattern, MockClock for deterministic time testing, and parameterized tests with rstest. Use when adding tests for domain entities, value objects, utilities, or tracker logic. Triggers on "write unit test", "add test", "test coverage", "unit testing", or "add unit tests".
- After: not yet rewritten

### Using the tracker

#### `use-rest-api`

- Before (333): Use the Torrust Tracker REST API. Covers authentication, all endpoints (stats, metrics, torrents, auth keys, whitelist), and making announce/scrape requests to verify API behaviour. Triggers on "use API", "test API", "call REST API", "query API", "API endpoint", "curl tracker", "tracker client", "announce request", or "verify API".
- After: not yet rewritten

#### `use-tracker-client`

- Before (381): Use the Torrust Tracker Client CLI to make BitTorrent announce and scrape requests against UDP and HTTP trackers. Covers the unified `tracker_client` binary, all subcommands, options, and output formats. Triggers on "tracker client", "use tracker client", "announce request", "scrape request", "http announce", "udp announce", "tracker_client", "test tracker", or "verify tracker".
- After: not yet rewritten
