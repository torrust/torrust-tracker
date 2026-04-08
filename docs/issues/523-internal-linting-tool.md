# Issue #523 Implementation Plan (Internal Linting Tool)

## Goal

Replace the MegaLinter idea with Torrust internal linting tooling and integrate it into CI for this repository.

## Scope

- Target issue: https://github.com/torrust/torrust-tracker/issues/523
- CI workflow to modify: .github/workflows/testing.yaml
- External reference workflow: https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/.github/workflows/linting.yml

## Tasks

### 0) Create a local branch following GitHub branch naming conventions

- Approved branch name: `523-internal-linting-tool`
- Commands:
  - `git fetch --all --prune`
  - `git checkout develop`
  - `git pull --ff-only`
  - `git checkout -b 523-internal-linting-tool`
- Checkpoint:
  - `git branch --show-current` should output `523-internal-linting-tool`.

### 1) Install and run the linting tool locally; verify it passes in this repo

- Identify/install internal linting package/tool used by Torrust (likely `torrust-linting` or equivalent wrapper).
- Ensure local runtime dependencies are present (if any).
- Note: linter config files (step 2) must exist in the repo root before a full suite run; it is fine to do a first exploratory run first to discover which linters are active.
- Run the internal linting command against this repository.
- Capture the exact command and output summary for reproducibility.
- Checkpoint:
  - Linting command exits with code `0`.

### 2) Add and adapt linter configuration files

Some linters require a config file in the repo root. Use the deployer configs as reference and adapt values to this repository.

| File                 | Linter           | Reference                                                                                             |
| -------------------- | ---------------- | ----------------------------------------------------------------------------------------------------- |
| `.markdownlint.json` | markdownlint     | https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/.markdownlint.json |
| `.taplo.toml`        | taplo (TOML fmt) | https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/.taplo.toml        |
| `.yamllint-ci.yml`   | yamllint         | https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/.yamllint-ci.yml   |

Key adaptations to make per file:

- `.markdownlint.json`: review line-length rules and Markdown conventions used in this repo's docs.
- `.taplo.toml`: update `exclude` list to match this repo's generated/runtime folders (e.g. `target/**`, `storage/**`) instead of the deployer-specific ones (`build/**`, `data/**`, `envs/**`).
- `.yamllint-ci.yml`: update `ignore` block to reflect this repo's generated/runtime directories instead of cloud-init and deployer folders.

Commit message: `ci(lint): add linter config files (.markdownlint.json, .taplo.toml, .yamllint-ci.yml)`

Checkpoint:

- Config files are present in the repo root.
- Running each individual linter against the repo with the config produces expected/controlled output.

### 3) If local linting fails, fix all lint errors; commit fixes independently per linter

- If the linting suite reports failures:
  - Group findings by linter (for example: formatting, clippy, docs, spelling, yaml, etc.).
  - Fix only one linter category at a time.
  - Create one commit per linter category.
- Commit style proposal:
  - `fix(lint/<linter-name>): resolve <brief issue summary>`
- Constraints:
  - Do not mix workflow/tooling changes with source lint fixes in the same commit.
  - Keep each commit minimal and reviewable.
- Checkpoint:
  - Re-run linting suite; all checks pass before moving to workflow integration.

### 4) Review existing workflow example using internal linting

- Read and analyze:
  - https://raw.githubusercontent.com/torrust/torrust-tracker-deployer/refs/heads/main/.github/workflows/linting.yml
- Extract and adapt:
  - Trigger strategy.
  - Tool setup/install method.
  - Cache strategy.
  - Invocation command and CI fail behavior.
- Checkpoint:
  - Document a short mapping from deployer workflow pattern to this repo’s `testing.yaml` job structure.

### 5) Modify `.github/workflows/testing.yaml` to use the internal linting tool

- Update the current `check`/lint-related section to run the internal linting command.
- Replace existing lint/check execution path with the internal linting tool in this migration (no parallel transition mode).
- Ensure matrix/toolchain compatibility is explicit (nightly/stable behavior decided and documented).
- Validate workflow syntax before commit.
- Checkpoint:
  - Workflow is valid and executes linting through internal tool.

### 6) Commit workflow changes

- Commit only workflow-related changes in a dedicated commit.
- Commit message proposal:
  - `ci(lint): switch testing workflow to internal linting tool`
- Checkpoint:
  - `git show --name-only --stat HEAD` includes only expected workflow files (and any required supporting CI files if intentionally added).

### 7) Push to remote `josecelano` and open PR into `develop`

- Verify remote exists:
  - `git remote -v`
- Push branch:
  - `git push -u josecelano 523-internal-linting-tool`
- Open PR targeting `torrust/torrust-tracker:develop` with head `josecelano:523-internal-linting-tool`.
- PR content should include:
  - Why internal linting over MegaLinter.
  - Summary of lint-fix commits by linter.
  - Summary of workflow change.
  - Evidence (local run + CI status).
- Checkpoint:
  - PR is open, linked to issue #523, and ready for review.

## Execution Notes

- Keep PR review-friendly by separating commits by concern:
  1. Linter config files (step 2)
  2. Per-linter source fixes (step 3, only if needed)
  3. CI workflow migration (step 6)
- Use Conventional Commits for all commits in this implementation.
- If lint checks differ between local and CI, align tool versions and execution flags before merging.
- Avoid broad refactors unrelated to lint failures.

## Decisions Confirmed

1. Branch name: `523-internal-linting-tool`.
2. CI strategy: replace existing lint/check path with internal linting.
3. Commit convention: yes, use Conventional Commits.
4. PR target: base `torrust/torrust-tracker:develop`, head `josecelano:523-internal-linting-tool`.

## Risks and Mitigations

- Risk: Internal linting wrapper may not be version-pinned and may produce unstable CI behavior.
  - Mitigation: Pin tool version in workflow installation step.
- Risk: Internal linting may overlap with existing checks, increasing CI time.
  - Mitigation: Remove redundant jobs only after verifying coverage parity.
- Risk: Tool may require secrets or environment assumptions not available in CI.
  - Mitigation: Run dry-run in GitHub Actions on branch before requesting review.
