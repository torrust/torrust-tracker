---
semantic-links:
  skill-links:
    - create-issue
    - cleanup-completed-issues
  related-artifacts:
    - docs/issues/README.md
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/cleanup-completed-issues/SKILL.md
---

# Open Issues

This folder contains folder-style issue specifications for GitHub issues that are currently open.
New primary specs use the allowed uppercase `ISSUE.md` (or `EPIC.md` for an EPIC), allowing
issue-local evidence, plans, and retrospectives to remain in the same directory. Legacy standalone
files are migrated in the planned archive batches tracked by issue #2159.

## Purpose

Open specs are the active implementation backlog for work that has already been formalized in
this repository.

Notes:

- Not every open GitHub issue has a spec file in this repository.
- New specs are added progressively when work starts on those issues.

## References

- Issues index: [../README.md](../README.md)
- Create and update specs: [`.claude/skills/create-issue/SKILL.md`](../../../.claude/skills/create-issue/SKILL.md)
- Move completed specs to closed: [`.claude/skills/cleanup-completed-issues/SKILL.md`](../../../.claude/skills/cleanup-completed-issues/SKILL.md)
