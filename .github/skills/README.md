# Moved: Agent Skills

The agent skills moved to [`.claude/skills/`](../../.claude/skills/README.md) in issue #2866, one
flat folder per skill, because Claude Code discovers project skills only at
`.claude/skills/<name>/SKILL.md`. Add new skills there, not here.

This folder holds only redirect stubs, so that old links and paths in historical records still
lead to the skills. Each stub has no frontmatter, so no tool reads it as a skill, and one link to
the skill's current place.

## Stub Inventory

The table lists the 53 `SKILL.md` paths that existed before the move. The `frontmatter-validator
--skill-layout` check keeps it and the stubs in step: every row has a stub and every stub has a
row. Rows and stubs are never deleted:

- when a skill is renamed, its row's current name and its stub's link change to the new name;
- when a skill is removed, its row's current name becomes `removed`, and its stub says so and
  links to the skill's last version on `develop` or to its replacement.

| Old path | Original name | Current name |
| -------- | ------------- | ------------ |
| `.github/skills/add-new-skill/SKILL.md` | `add-new-skill` | `add-new-skill` |
| `.github/skills/dev/benchmarking/run-benchmarks/SKILL.md` | `run-benchmarks` | `run-benchmarks` |
| `.github/skills/dev/ci/implement-workflow/SKILL.md` | `implement-workflow` | `implement-workflow` |
| `.github/skills/dev/ci/watch-pull-request-checks/SKILL.md` | `watch-pull-request-checks` | `watch-pull-request-checks` |
| `.github/skills/dev/debugging/fix-bug/SKILL.md` | `fix-bug` | `fix-bug` |
| `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md` | `run-tracker-locally` | `run-tracker-locally` |
| `.github/skills/dev/git-workflow/commit-changes/SKILL.md` | `commit-changes` | `commit-changes` |
| `.github/skills/dev/git-workflow/create-feature-branch/SKILL.md` | `create-feature-branch` | `create-feature-branch` |
| `.github/skills/dev/git-workflow/merge-pull-request/SKILL.md` | `merge-pull-request` | `merge-pull-request` |
| `.github/skills/dev/git-workflow/open-pull-request/SKILL.md` | `open-pull-request` | `open-pull-request` |
| `.github/skills/dev/git-workflow/push-changes/SKILL.md` | `push-changes` | `push-changes` |
| `.github/skills/dev/git-workflow/release-new-version/SKILL.md` | `release-new-version` | `release-new-version` |
| `.github/skills/dev/git-workflow/release-workspace-package/SKILL.md` | `release-workspace-package` | `release-workspace-package` |
| `.github/skills/dev/git-workflow/run-linters/SKILL.md` | `run-linters` | `run-linters` |
| `.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md` | `run-pre-commit-checks` | `run-pre-commit-checks` |
| `.github/skills/dev/git-workflow/run-pre-push-checks/SKILL.md` | `run-pre-push-checks` | `run-pre-push-checks` |
| `.github/skills/dev/github/link-subissue-to-parent-issue/SKILL.md` | `link-subissue-to-parent-issue` | `link-subissue-to-parent-issue` |
| `.github/skills/dev/logging/structured-runtime-logging/SKILL.md` | `structured-runtime-logging` | `structured-runtime-logging` |
| `.github/skills/dev/maintenance/add-rust-dependency/SKILL.md` | `add-rust-dependency` | `add-rust-dependency` |
| `.github/skills/dev/maintenance/add-workspace-member/SKILL.md` | `add-workspace-member` | `add-workspace-member` |
| `.github/skills/dev/maintenance/catalog-security-vulnerabilities/SKILL.md` | `catalog-security-vulnerabilities` | `catalog-security-vulnerabilities` |
| `.github/skills/dev/maintenance/check-runner-health/SKILL.md` | `check-runner-health` | `check-runner-health` |
| `.github/skills/dev/maintenance/install-linter/SKILL.md` | `install-linter` | `install-linter` |
| `.github/skills/dev/maintenance/run-manual-docker-security-scan/SKILL.md` | `run-manual-docker-security-scan` | `run-manual-docker-security-scan` |
| `.github/skills/dev/maintenance/setup-dev-environment/SKILL.md` | `setup-dev-environment` | `setup-dev-environment` |
| `.github/skills/dev/maintenance/triage-github-security-findings/SKILL.md` | `triage-github-security-findings` | `triage-github-security-findings` |
| `.github/skills/dev/maintenance/update-dependencies/SKILL.md` | `update-dependencies` | `update-dependencies` |
| `.github/skills/dev/maintenance/update-github-workflow-actions/SKILL.md` | `update-github-workflow-actions` | `update-github-workflow-actions` |
| `.github/skills/dev/planning/cleanup-completed-issues/SKILL.md` | `cleanup-completed-issues` | `cleanup-completed-issues` |
| `.github/skills/dev/planning/create-adr/SKILL.md` | `create-adr` | `create-adr` |
| `.github/skills/dev/planning/create-issue/SKILL.md` | `create-issue` | `create-issue` |
| `.github/skills/dev/planning/create-markdown-template/SKILL.md` | `create-markdown-template` | `create-markdown-template` |
| `.github/skills/dev/planning/create-refactor-plan/SKILL.md` | `create-refactor-plan` | `create-refactor-plan` |
| `.github/skills/dev/planning/reopen-issue/SKILL.md` | `reopen-issue` | `reopen-issue` |
| `.github/skills/dev/planning/write-markdown-docs/SKILL.md` | `write-markdown-docs` | `write-markdown-docs` |
| `.github/skills/dev/pr-reviews/fetch-review-threads/SKILL.md` | `fetch-review-threads` | `fetch-review-threads` |
| `.github/skills/dev/pr-reviews/process-copilot-suggestions/SKILL.md` | `process-copilot-suggestions` | `process-copilot-suggestions` |
| `.github/skills/dev/pr-reviews/process-pr-review/SKILL.md` | `process-pr-review` | `process-pr-review` |
| `.github/skills/dev/pr-reviews/process-pr-review-feedback/SKILL.md` | `process-pr-review-feedback` | `process-pr-review-feedback` |
| `.github/skills/dev/pr-reviews/resolve-review-threads/SKILL.md` | `resolve-review-threads` | `resolve-review-threads` |
| `.github/skills/dev/pr-reviews/review-pr/SKILL.md` | `review-pr` | `review-pr` |
| `.github/skills/dev/pr-reviews/wait-for-pr-review/SKILL.md` | `wait-for-pr-review` | `wait-for-pr-review` |
| `.github/skills/dev/rust-code-quality/fix-clippy-warnings/SKILL.md` | `fix-clippy-warnings` | `fix-clippy-warnings` |
| `.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md` | `handle-errors-in-code` | `handle-errors-in-code` |
| `.github/skills/dev/rust-code-quality/handle-secrets/SKILL.md` | `handle-secrets` | `handle-secrets` |
| `.github/skills/dev/task-reviews/review-task/SKILL.md` | `review-task` | `review-task` |
| `.github/skills/dev/testing/manual-http-download-completion-e2e/SKILL.md` | `manual-http-download-completion-e2e` | `manual-http-download-completion-e2e` |
| `.github/skills/dev/testing/manual-torrent-cleanup-e2e/SKILL.md` | `manual-torrent-cleanup-e2e` | `manual-torrent-cleanup-e2e` |
| `.github/skills/dev/testing/manual-udp-download-completion-e2e/SKILL.md` | `manual-udp-download-completion-e2e` | `manual-udp-download-completion-e2e` |
| `.github/skills/dev/testing/public-trackers-for-testing/SKILL.md` | `public-trackers-for-testing` | `public-trackers-for-testing` |
| `.github/skills/dev/testing/write-unit-test/SKILL.md` | `write-unit-test` | `write-unit-test` |
| `.github/skills/usage/use-rest-api/SKILL.md` | `use-rest-api` | `use-rest-api` |
| `.github/skills/usage/use-tracker-client/SKILL.md` | `use-tracker-client` | `use-tracker-client` |
