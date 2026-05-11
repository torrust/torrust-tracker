# Add a Docs-Only CI Fast Path

## Goal

Avoid running heavyweight test, compatibility, and E2E workflows for documentation-only pull
requests while still validating documentation quality in CI.

## Problem

Documentation changes currently trigger the same expensive workflows as code changes, including
the `Testing` workflow in [`.github/workflows/testing.yaml`](../../../.github/workflows/testing.yaml).
That workflow runs full-workspace linters, tests, and Docker-based E2E jobs, which is slow and
unnecessary when a pull request only changes documentation.

This is particularly costly in this repository because AI-assisted work produces frequent updates
to issue specs, ADRs, agent instructions, and other Markdown documents.

## Constraints

### 1. Documentation still needs CI coverage

We should not skip CI entirely for docs-only changes. At minimum, documentation-only pull requests
should run:

- Markdown linting
- Spell checking (`cspell`)

These checks should stay lightweight. Because [#1726](https://github.com/torrust/torrust-tracker/issues/1726)
is still researching whether Rust compilation can be sped up enough in CI, this issue should avoid
designs that introduce unnecessary workspace compilation just to validate documentation.

### 2. "Docs-only" must cover all documentation surfaces

This repository stores documentation in multiple places, not only in `docs/`. The trigger rules
should review at least the following categories:

- `docs/**`
- top-level Markdown such as `README.md`, `SECURITY.md`, and `AGENTS.md`
- package `README.md` files
- console `README.md` files
- `.github/skills/**/SKILL.md`
- `.github/agents/*.md`

The issue implementation should define the final path set explicitly.

### 3. Required checks must not block merge

If the repository marks heavyweight workflows as required checks, skipping them entirely with
`paths-ignore` may leave pull requests stuck. For this issue, the preferred approach is to update
branch protection so heavyweight workflows are no longer required for documentation-only pull
requests.

Keeping workflows running only to satisfy required-check mechanics defeats much of the value of a
docs-only fast path. Since pull requests are reviewed manually before merge, this issue should
prioritize faster workflow execution over preserving the current required-check set unchanged.

## Proposed Changes

### Task 1: Define the docs-only path policy

- [ ] List every documentation path category that should count as "docs-only".
- [ ] List the non-doc paths that should always force full CI, even if Markdown files also
      changed.
- [ ] Document the policy in the workflow comments so the rationale remains obvious.

### Task 2: Add a dedicated lightweight docs workflow

- [ ] Create a workflow dedicated to documentation validation.
- [ ] Run only the documentation-relevant checks, at minimum markdownlint and `cspell`.
- [ ] Keep the workflow lightweight. Using the internal `linter` binary is acceptable if its
      installation and execution cost stays low enough for documentation-only pull requests.
- [ ] Ensure the workflow is fast enough to serve as the main required signal for docs-only pull
      requests.

### Task 3: Exclude docs-only changes from heavyweight workflows

- [ ] Update the heavyweight PR workflows so docs-only changes do not run the full CI matrix.
- [ ] Update branch protection rules so skipped heavyweight workflows do not block
      documentation-only pull requests.
- [ ] Verify behavior for `pull_request` and, if needed, `push` events.
- [ ] Confirm that docs-only pull requests remain mergeable.

### Task 4: Validate mixed-change behavior

- [ ] Verify that a pull request touching both docs and Rust code still runs the full CI set.
- [ ] Verify that a pull request touching docs plus workflow files still runs the appropriate CI.
- [ ] Document at least one representative example for each case.

## Acceptance Criteria

- [ ] Documentation-only pull requests do not run heavyweight test and E2E workflows.
- [ ] Documentation-only pull requests still run markdownlint and `cspell` in CI.
- [ ] The docs-only workflow remains lightweight enough for documentation-only pull requests,
      including when implemented via the internal `linter` binary.
- [ ] Pull requests that touch code continue to run the full relevant CI workflows.
- [ ] Branch protection rules are adjusted so docs-only pull requests are not blocked by skipped
      heavyweight workflows.
- [ ] Workflow comments document the path policy clearly.

## References

- Related workflow: [`.github/workflows/testing.yaml`](../../../.github/workflows/testing.yaml)
- Related workflow: [`.github/workflows/os-compatibility.yaml`](../../../.github/workflows/os-compatibility.yaml)
- Related workflow: [`.github/workflows/db-compatibility.yaml`](../../../.github/workflows/db-compatibility.yaml)
- Related workflow: [`.github/workflows/db-benchmarking.yaml`](../../../.github/workflows/db-benchmarking.yaml)
- Related EPIC: [docs/issues/1742-ci-change-aware-workflows-epic.md](./1742-ci-change-aware-workflows-epic.md)
- Related issue: [#1726](https://github.com/torrust/torrust-tracker/issues/1726) (research on
  reducing the cost of workflows that still need to run)
- Related local spec: [docs/issues/1740-fix-container-workflow-caching.md](./1740-fix-container-workflow-caching.md)
