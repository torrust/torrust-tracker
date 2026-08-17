---
semantic-links:
  skill-links:
    - cleanup-completed-issues
  related-artifacts:
    - docs/issues/README.md
    - .github/skills/dev/planning/cleanup-completed-issues/SKILL.md
---

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

## Archive Maintenance

Archiving a spec also requires repairing live documentation references to its former
`docs/issues/open/` path and updating frontmatter in every affected current document. This keeps
EPIC tables, issue dependencies, ADR links, and issue-local evidence discoverable after the move.
The authoritative procedure is the cleanup workflow skill below.

## References

- Issues index: [../README.md](../README.md)
- Cleanup workflow source of truth: [`.github/skills/dev/planning/cleanup-completed-issues/SKILL.md`](../../../.github/skills/dev/planning/cleanup-completed-issues/SKILL.md)
