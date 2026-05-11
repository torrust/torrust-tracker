# Recently Closed Issues

This folder holds issue specification files for issues that have been closed but are kept
temporarily as a reference buffer for ongoing and upcoming work.

## Purpose

Closed spec files are moved here (rather than deleted immediately) because:

- The reasoning and design decisions captured in a spec often remain relevant to the next
  issue in a series.
- Reviewers and contributors benefit from being able to trace _why_ a decision was made
  across multiple related issues.
- It provides a grace period before permanent removal, reducing the risk of losing context
  that is still actively referenced.

## Lifecycle

1. **Issue closed / PR merged** → spec file moves from `docs/issues/open/` to `docs/issues/closed/`.
2. **Buffer period** → file lives here while adjacent issues are still in progress.
3. **Cleanup** → once the spec is no longer referenced by active work, it is deleted.

Use the `cleanup-completed-issues` skill to manage this lifecycle.
