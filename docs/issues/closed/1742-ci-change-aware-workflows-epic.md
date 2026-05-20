---
doc-type: issue
issue-type: epic
status: done
priority: p2
github-issue: 1742
spec-path: docs/issues/closed/1742-ci-change-aware-workflows-epic.md
branch: null
related-pr: null
last-updated-utc: null
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - .github/workflows/
---

# EPIC: Make CI Change-Aware

## Goal

Reduce unnecessary CI time and runner usage by making heavyweight workflows run only when the
changed files can affect the behavior they validate.

The current CI setup runs several expensive workflows for almost every pull request, including
documentation-only changes. That slows down review and merge for low-risk changes and consumes
GitHub-hosted runner minutes without increasing confidence.

This EPIC groups two implementation subissues plus one related research track:

1. Existing issue [#1726](https://github.com/torrust/torrust-tracker/issues/1726), which researches
   whether `sccache` can reduce Rust build times for the workflows that still need to run.
2. A new docs-only CI fast path so documentation changes do not wait for full test and E2E
   matrices.
3. A new persistence-scoped CI strategy so database compatibility and benchmarking workflows only
   run for persistence-relevant changes.

The intent is to reduce waste without weakening the safety net for code changes.

## Why This Is Needed

The following workflows currently run broadly on `push` and `pull_request` events:

- [`.github/workflows/testing.yaml`](../../../.github/workflows/testing.yaml)
- [`.github/workflows/os-compatibility.yaml`](../../../.github/workflows/os-compatibility.yaml)
- [`.github/workflows/db-compatibility.yaml`](../../../.github/workflows/db-compatibility.yaml)
- [`.github/workflows/db-benchmarking.yaml`](../../../.github/workflows/db-benchmarking.yaml)

This has two visible effects:

- Small documentation-only pull requests wait behind workflows that cannot be affected by the
  change.
- Persistence-specific workflows run even when a pull request does not touch persistence-related
  code.

The repository already has adjacent CI optimization work in progress:

- [#1726](https://github.com/torrust/torrust-tracker/issues/1726) is an evidence-driven research
  issue about Rust compilation costs and whether `sccache` should be adopted at all.
- [#1740](../1740-fix-container-workflow-caching.md) addresses container build cache behavior.

That makes this a good time to define a coherent, change-aware CI strategy rather than continuing
with one-off workflow tweaks.

## Scope

This EPIC covers workflow triggering and workflow gating only.

In scope:

- Add a docs-only CI fast path with lightweight checks.
- Restrict persistence-specific workflows to persistence-relevant changes.
- Review required-check behavior so selective triggers do not leave pull requests blocked by
  missing or permanently pending checks.
- Document the path rules and rationale in the workflow files.

Out of scope:

- Rewriting the test matrix.
- Replacing the current cache strategy wholesale.
- Container cache optimization already tracked in [#1740](../1740-fix-container-workflow-caching.md).

## Related Research Track

### Research `sccache` impact on remaining heavy workflows

- Existing issue: [#1726](https://github.com/torrust/torrust-tracker/issues/1726)
- Local spec: [docs/issues/open/1726-reduce-build-times-sccache/ISSUE.md](../open/1726-reduce-build-times-sccache/ISSUE.md)
- Focus: determine, with benchmarks, whether `sccache` reduces compilation cost for workflows that
  still need to run.
- Relationship to this EPIC: complementary, but not a blocker. The docs-only fast path and
  persistence scoping issues can proceed independently of the `1726` research outcome.

## Implementation Subissues

### Subissue 1: Add a Docs-Only CI Fast Path

- Issue: [#1743](https://github.com/torrust/torrust-tracker/issues/1743)
- Local spec: [docs/issues/1743-docs-only-ci-fast-path.md](./1743-docs-only-ci-fast-path.md)
- Focus: skip heavyweight workflows for documentation-only changes while still running markdown
  and spelling checks.

### Subissue 2: Scope Persistence Workflows by Path

- Issue: [#1744](https://github.com/torrust/torrust-tracker/issues/1744)
- Local spec:
  [docs/issues/1744-scope-persistence-workflows-by-path.md](./1744-scope-persistence-workflows-by-path.md)
- Focus: run database compatibility and persistence benchmarking only when changes can affect
  persistence behavior.

## Risks and Constraints

### 1. Required checks must remain mergeable

If a workflow is skipped entirely via `paths` or `paths-ignore`, branch protection can treat a
required check as missing. The implementation must either:

- update required-check configuration to match the new workflow model, or
- keep the workflow running and use an early change-detection job that exits green when the
  workflow is not relevant.

### 2. `#1726` should not block change-aware trigger work

Issue `#1726` is about reducing the cost of relevant workflows after they start. This EPIC is
about avoiding irrelevant workflow runs in the first place.

That means:

- docs-only fast-path work should not wait for `sccache` research to finish,
- persistence workflow scoping should not wait for `sccache` research to finish, and
- any implementation here should avoid assuming that `sccache` will be adopted.

### 3. "Docs-only" must be defined explicitly

Documentation is not limited to `docs/` in this repository. Relevant documentation paths also
include files such as:

- `README.md`
- `SECURITY.md`
- `AGENTS.md`
- `.github/skills/**/SKILL.md`
- package and console `README.md` files

The subissue should define the exact path set and justify it.

### 4. Docs workflow must stay lightweight even if `#1726` is unresolved

The live `#1726` issue confirms that Rust compilation is a major part of CI cost and that the
benefit of `sccache` is still under research. A docs-only workflow should therefore avoid relying
on Rust compilation for its main checks when possible.

In practice, that means keeping the docs-only workflow lightweight and avoiding unnecessary
workspace compilation. Using the internal `linter` binary is acceptable if its installation and
execution cost stays low enough that the workflow remains fast for documentation-only pull
requests.

### 5. Persistence workflow scope is intentionally narrower than general regression coverage

The persistence-specific workflows are intended to validate schema, migration, query, and
persistence-driver behavior in `tracker-core`, not to provide full cross-package regression
coverage.

For that reason, the corresponding subissue intentionally prefers a narrow trigger centered on
`packages/tracker-core/**` plus workflow-file changes when relevant. Broader compile and
integration regressions remain the responsibility of the general testing workflows.

## Acceptance Criteria

- [ ] A documented change-aware CI strategy exists for docs-only and persistence-related changes.
- [ ] The EPIC links `#1726` as a related research track and links the two new implementation
      subissues.
- [ ] The final implementation keeps pull requests mergeable under the repository's required-check
      policy.
- [ ] Heavy workflows no longer run for documentation-only pull requests.
- [ ] Persistence-specific workflows no longer run for unrelated changes.

## References

- Related issue: [#1726](https://github.com/torrust/torrust-tracker/issues/1726)
- Related local spec: [docs/issues/1740-fix-container-workflow-caching.md](./1740-fix-container-workflow-caching.md)
- Related workflows:
  - [`.github/workflows/testing.yaml`](../../../.github/workflows/testing.yaml)
  - [`.github/workflows/os-compatibility.yaml`](../../../.github/workflows/os-compatibility.yaml)
  - [`.github/workflows/db-compatibility.yaml`](../../../.github/workflows/db-compatibility.yaml)
  - [`.github/workflows/db-benchmarking.yaml`](../../../.github/workflows/db-benchmarking.yaml)
