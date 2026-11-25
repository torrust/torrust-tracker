# Agent Skills

Each skill is a folder `.claude/skills/<name>/` with a `SKILL.md`, where Claude Code discovers it.
Claude Code reads only this flat layout: a skill one folder deeper is never listed, so the grouping
by area lives in this index instead of in folders.

Before issue #2866 the skills lived under `.github/skills/dev/<area>/<name>/` (and
`.github/skills/add-new-skill/` and `.github/skills/usage/<name>/`). Those `SKILL.md` paths now
hold redirect stubs, inventoried in [`.github/skills/README.md`](../../.github/skills/README.md).

When you add, rename, or remove a skill, update this index in the same change; the
`frontmatter-validator --skill-layout` check fails when the index and the skill folders disagree.
See [`add-new-skill`](add-new-skill/SKILL.md).

## Skill authoring

| Skill                                     | Purpose                                                                    |
| ----------------------------------------- | -------------------------------------------------------------------------- |
| [`add-new-skill`](add-new-skill/SKILL.md) | Guide for creating effective Agent Skills for the torrust-tracker project. |

## Benchmarking

| Skill                                       | Purpose                                                                                                                                       |
| ------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------- |
| [`run-benchmarks`](run-benchmarks/SKILL.md) | Choose, run, and record Torrust Tracker benchmarks, including before-and-after performance evidence for changes to performance-critical code. |

## CI

| Skill                                                             | Purpose                                                                                              |
| ----------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| [`implement-workflow`](implement-workflow/SKILL.md)               | Add or materially change GitHub Actions workflows, CI jobs, workflow scripts, and their permissions. |
| [`watch-pull-request-checks`](watch-pull-request-checks/SKILL.md) | Wait for the CI checks of a pull request to finish with the GitHub CLI, then read which ones failed. |

## Debugging

| Skill                         | Purpose                                                                  |
| ----------------------------- | ------------------------------------------------------------------------ |
| [`fix-bug`](fix-bug/SKILL.md) | Canonical workflow for investigating and fixing bugs in torrust-tracker. |

## Environment setup

| Skill                                                 | Purpose                                                      |
| ----------------------------------------------------- | ------------------------------------------------------------ |
| [`run-tracker-locally`](run-tracker-locally/SKILL.md) | Run the Torrust Tracker locally for development and testing. |

## Git workflow

| Skill                                                             | Purpose                                                                                                                                         |
| ----------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------- |
| [`commit-changes`](commit-changes/SKILL.md)                       | Guide for committing changes in the torrust-tracker project.                                                                                    |
| [`create-feature-branch`](create-feature-branch/SKILL.md)         | Guide for creating feature branches following the torrust-tracker branching conventions.                                                        |
| [`merge-pull-request`](merge-pull-request/SKILL.md)               | Safely construct, inspect, validate, sign, and optionally push a maintainer GitHub pull-request merge using the repository-local vendored tool. |
| [`open-pull-request`](open-pull-request/SKILL.md)                 | Open a pull request from a feature branch using GitHub CLI (preferred) or GitHub MCP tools.                                                     |
| [`push-changes`](push-changes/SKILL.md)                           | Guide for pushing commits in the torrust-tracker project.                                                                                       |
| [`release-new-version`](release-new-version/SKILL.md)             | Guide for releasing a new version of the Torrust Tracker using the standard staging branch, tag, and crate publication workflow.                |
| [`release-workspace-package`](release-workspace-package/SKILL.md) | Guide for releasing a single workspace package, or a crate that lives in its own standalone Torrust repository, to crates.io.                   |
| [`run-linters`](run-linters/SKILL.md)                             | Run code quality checks and linters for the torrust-tracker project.                                                                            |
| [`run-pre-commit-checks`](run-pre-commit-checks/SKILL.md)         | Run all mandatory pre-commit verification steps for the torrust-tracker project.                                                                |
| [`run-pre-push-checks`](run-pre-push-checks/SKILL.md)             | Run all mandatory pre-push verification steps for the torrust-tracker project.                                                                  |

## GitHub

| Skill                                                                     | Purpose                                                                                                     |
| ------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- |
| [`link-subissue-to-parent-issue`](link-subissue-to-parent-issue/SKILL.md) | Guide for linking an existing GitHub issue as a sub-issue of a parent issue in the torrust-tracker project. |

## Logging

| Skill                                                               | Purpose                                                                                                                        |
| ------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------ |
| [`structured-runtime-logging`](structured-runtime-logging/SKILL.md) | Use when adding or changing logs for runtime service identity, service startup, listener bindings, or tracing instrumentation. |

## Maintenance

| Skill                                                                           | Purpose                                                                                                                                            |
| ------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`add-rust-dependency`](add-rust-dependency/SKILL.md)                           | Add a Rust crate dependency safely: latest stable version, minimal features, and the quality gates.                                                |
| [`add-workspace-member`](add-workspace-member/SKILL.md)                         | Add or remove an explicit Cargo workspace member in Torrust Tracker.                                                                               |
| [`catalog-security-vulnerabilities`](catalog-security-vulnerabilities/SKILL.md) | Guide for cataloging public container, dependency, and CVE warnings that do not affect the project, with escalation for affecting vulnerabilities. |
| [`check-runner-health`](check-runner-health/SKILL.md)                           | Check the self-hosted CI runner (load, memory, disk, Docker cleanup) and record a dated report.                                                    |
| [`install-linter`](install-linter/SKILL.md)                                     | Install the torrust-linting `linter` binary and its external tool dependencies.                                                                    |
| [`run-manual-docker-security-scan`](run-manual-docker-security-scan/SKILL.md)   | Guide for running a manual Docker security scan for the tracker runtime image and documenting results.                                             |
| [`setup-dev-environment`](setup-dev-environment/SKILL.md)                       | Set up a local development environment for torrust-tracker from scratch.                                                                           |
| [`triage-github-security-findings`](triage-github-security-findings/SKILL.md)   | Triage GitHub Security tab findings, keep the finding inventory, and create issue specs by root cause.                                             |
| [`update-dependencies`](update-dependencies/SKILL.md)                           | Guide for updating project dependencies in the torrust-tracker project.                                                                            |
| [`update-github-workflow-actions`](update-github-workflow-actions/SKILL.md)     | Update GitHub Actions workflow dependencies safely in Torrust Tracker, including synchronizing the Torrust organization allowlist.                 |

## Planning

| Skill                                                           | Purpose                                                                                                    |
| --------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------- |
| [`cleanup-completed-issues`](cleanup-completed-issues/SKILL.md) | Guide for archiving closed issue specification files from docs/issues/open/ to docs/issues/closed/.        |
| [`create-adr`](create-adr/SKILL.md)                             | Guide for creating Architectural Decision Records (ADRs) in the torrust-tracker project.                   |
| [`create-issue`](create-issue/SKILL.md)                         | Guide for creating GitHub issues in the torrust-tracker project.                                           |
| [`create-markdown-template`](create-markdown-template/SKILL.md) | Create or maintain reusable Markdown document templates in the torrust-tracker repository.                 |
| [`create-refactor-plan`](create-refactor-plan/SKILL.md)         | Guide for creating refactor plans in the torrust-tracker project.                                          |
| [`reopen-issue`](reopen-issue/SKILL.md)                         | Guide for reopening a closed GitHub issue or EPIC whose specification was archived in docs/issues/closed/. |
| [`write-markdown-docs`](write-markdown-docs/SKILL.md)           | Guide for writing Markdown documentation in this project.                                                  |

## Pull request reviews

| Skill                                                                 | Purpose                                                                                                                             |
| --------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- |
| [`fetch-review-threads`](fetch-review-threads/SKILL.md)               | Fetch every GitHub pull request review thread, including resolved and outdated ones, for the torrust-tracker project.               |
| [`process-copilot-suggestions`](process-copilot-suggestions/SKILL.md) | Deprecated compatibility redirect for Copilot pull-request review suggestions.                                                      |
| [`process-pr-review`](process-pr-review/SKILL.md)                     | Process every pull-request review finding the same way, whoever the reviewer is: a person or a bot.              |
| [`process-pr-review-feedback`](process-pr-review-feedback/SKILL.md)   | Deprecated compatibility redirect for pull-request review feedback.                                                                 |
| [`resolve-review-threads`](resolve-review-threads/SKILL.md)           | Resolve addressed GitHub pull request review threads for the torrust-tracker project.                                               |
| [`review-pr`](review-pr/SKILL.md)                                     | Review an existing pull request for the torrust-tracker project.                                                                    |
| [`wait-for-pr-review`](wait-for-pr-review/SKILL.md)                   | Wait for pull-request reviewers without spending tokens, using a deterministic Rust poller, then run the review loop on each event. |

## Rust code quality

| Skill                                                     | Purpose                                                               |
| --------------------------------------------------------- | --------------------------------------------------------------------- |
| [`fix-clippy-warnings`](fix-clippy-warnings/SKILL.md)     | Guide for fixing Rust Clippy warnings in the torrust-tracker project. |
| [`handle-errors-in-code`](handle-errors-in-code/SKILL.md) | Guide for error handling in this Rust project.                        |
| [`handle-secrets`](handle-secrets/SKILL.md)               | Guide for handling sensitive data (secrets) in this Rust project.     |

## Task reviews

| Skill                                 | Purpose                                                |
| ------------------------------------- | ------------------------------------------------------ |
| [`review-task`](review-task/SKILL.md) | Review a completed implementation task before push/PR. |

## Testing

| Skill                                                                                 | Purpose                                                                                                              |
| ------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| [`manual-http-download-completion-e2e`](manual-http-download-completion-e2e/SKILL.md) | Manually verify the started-to-completed peer lifecycle through the HTTP tracker with `curl`.                        |
| [`manual-torrent-cleanup-e2e`](manual-torrent-cleanup-e2e/SKILL.md)                   | Manually verify that the tracker removes inactive peers while retaining a peerless torrent when configured to do so. |
| [`manual-udp-download-completion-e2e`](manual-udp-download-completion-e2e/SKILL.md)   | Manually verify the started-to-completed peer lifecycle through the UDP tracker with `tracker_client`.               |
| [`public-trackers-for-testing`](public-trackers-for-testing/SKILL.md)                 | Public tracker targets for manual testing and debugging of tracker clients.                                          |
| [`write-unit-test`](write-unit-test/SKILL.md)                                         | Write unit tests to the project conventions: behavior-driven names, Arrange-Act-Assert, `MockClock` and `rstest`.    |

## Using the tracker

| Skill                                               | Purpose                                                                                                           |
| --------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------- |
| [`use-rest-api`](use-rest-api/SKILL.md)             | Use the Torrust Tracker REST API.                                                                                 |
| [`use-tracker-client`](use-tracker-client/SKILL.md) | Use the Torrust Tracker Client CLI to make BitTorrent announce and scrape requests against UDP and HTTP trackers. |
