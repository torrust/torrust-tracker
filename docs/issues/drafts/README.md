---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/README.md
    - .github/skills/dev/planning/create-issue/SKILL.md
---

# Issue Drafts

This folder contains draft issue specification files that are not yet linked to a created GitHub issue.

## Purpose

Draft specs capture problem framing, scope, and implementation intent before opening a tracked issue.

Use an unnumbered descriptive filename for a standalone draft. When the draft is an explicitly
established subissue of a known EPIC, prefix its filename or folder with the parent EPIC's GitHub
issue number, for example `1669-extract-torrust-tracker-client-to-standalone-repo.md`. Set the
frontmatter field `epic: 1669` and identify the parent in the document body. Do not infer a parent
EPIC from related work; leave `epic: null` and use an unnumbered name until the relationship is
established.

The prefix is the parent EPIC number, not the future subissue number. After the GitHub subissue is
created, move the spec to `docs/issues/open/` and rename it to begin with its own assigned issue
number; use the open-spec naming convention for the complete subissue form.

Use drafts when:

- The work is still being refined.
- The issue title/scope is not final.
- Supporting references and acceptance criteria are still being assembled.

## References

- Issues index: [../README.md](../README.md)
- Workflow source of truth: [`.github/skills/dev/planning/create-issue/SKILL.md`](../../../.github/skills/dev/planning/create-issue/SKILL.md)
