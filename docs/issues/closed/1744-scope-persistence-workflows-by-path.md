---
doc-type: issue
issue-type: task
status: done
priority: p2
github-issue: 1744
spec-path: docs/issues/closed/1744-scope-persistence-workflows-by-path.md
branch: 1744-scope-persistence-workflows-by-path
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - docs/issues/closed/1742-ci-change-aware-workflows-epic.md
    - .github/workflows/db-compatibility.yaml
    - .github/workflows/db-benchmarking.yaml
---

# Scope Persistence Workflows by Path

## Goal

Run persistence-specific CI workflows only when a pull request changes files that can affect
database compatibility or persistence benchmarking.

## Problem

The following workflows currently run broadly on most pull requests:

- [`.github/workflows/db-compatibility.yaml`](../../../.github/workflows/db-compatibility.yaml)
- [`.github/workflows/db-benchmarking.yaml`](../../../.github/workflows/db-benchmarking.yaml)

Both workflows are persistence-specific. They validate database compatibility and benchmark the
`bittorrent-tracker-core` persistence layer, but they currently run even when a pull request only
changes unrelated areas such as documentation, HTTP client code, or other non-persistence
packages.

That wastes CI time and runner capacity without increasing confidence.

Issue [#1726](https://github.com/torrust/torrust-tracker/issues/1726) may reduce the runtime cost
of these workflows later, but it does not change the fact that they should not run for unrelated
pull requests.

## Scope Decision

This issue should intentionally scope the persistence workflows to changes in `tracker-core`,
because the workflows are validating the persistence implementation directly.

The database compatibility jobs in
[`.github/workflows/db-compatibility.yaml`](../../../.github/workflows/db-compatibility.yaml)
run `cargo test -p bittorrent-tracker-core ... run_mysql_driver_tests` and
`run_postgres_driver_tests`. Those tests construct the database drivers and call the persistence
methods directly against real database instances.

Because of that, the intent of these workflows is narrower than general workspace regression
coverage: they are primarily checking schema, migration, query, and persistence-driver behavior in
`tracker-core`.

The preferred trigger scope for this issue is therefore:

- `packages/tracker-core/**`
- the workflow files themselves when they are modified

General compile or cross-package integration regressions remain the responsibility of the broader
testing workflows.

This issue should also avoid depending on the outcome of `#1726`. Even if `sccache` proves useful,
running persistence workflows for unrelated changes would still be wasteful.

## Proposed Changes

### Task 1: Define the persistence-relevant path set

- [ ] Define the narrow path set for persistence workflows, centered on `packages/tracker-core/**`.
- [ ] Decide whether workflow file changes should also trigger the workflows.
- [ ] Document explicitly that this is an intentional optimization tradeoff, not full dependency
      closure analysis.

### Task 2: Restrict the database compatibility workflow

- [ ] Update [`.github/workflows/db-compatibility.yaml`](../../../.github/workflows/db-compatibility.yaml)
      so it only runs for persistence-relevant changes.
- [ ] Validate behavior for both MySQL and PostgreSQL jobs.
- [ ] Confirm that required-check behavior remains mergeable for unrelated pull requests.

### Task 3: Restrict the persistence benchmarking workflow

- [ ] Update [`.github/workflows/db-benchmarking.yaml`](../../../.github/workflows/db-benchmarking.yaml)
      so it only runs for persistence-relevant changes.
- [ ] Ensure the path policy stays aligned with the compatibility workflow.
- [ ] Confirm that unrelated pull requests no longer trigger the benchmarking workflow.

### Task 4: Add guardrails for future dependency drift

- [ ] Add comments near the trigger rules explaining that the scope is intentionally limited to
      tracker-core persistence changes.
- [ ] Consider whether workflow file changes should bypass the path filter.
- [ ] Verify at least one negative case and one positive case with representative pull requests.

## Acceptance Criteria

- [ ] `db-compatibility` does not run for unrelated pull requests.
- [ ] `db-benchmarking` does not run for unrelated pull requests.
- [ ] Both workflows run when `packages/tracker-core/**` changes.
- [ ] The trigger rules are documented and maintainable.
- [ ] Required-check behavior does not leave unrelated pull requests blocked.

## References

- Related workflow: [`.github/workflows/db-compatibility.yaml`](../../../.github/workflows/db-compatibility.yaml)
- Related workflow: [`.github/workflows/db-benchmarking.yaml`](../../../.github/workflows/db-benchmarking.yaml)
- Related EPIC: [docs/issues/1742-ci-change-aware-workflows-epic.md](./1742-ci-change-aware-workflows-epic.md)
- Related issue: [#1726](https://github.com/torrust/torrust-tracker/issues/1726) (complementary
  build-time research, not a blocker for this change)
