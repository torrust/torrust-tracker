# Agents Instructions — `docs/issues/open/`

## Spec Naming Conventions

Use a standalone Markdown file when a specification has no issue-local supporting artifacts.
Use a folder when it needs issue-local artifacts; the primary file inside the folder is `ISSUE.md`
for issues or `EPIC.md` for EPICs. The GitHub issue number must start every filename or folder
name.

### Standalone specification

#### Issue

```text
{ISSUE_NUMBER}-{short-description}.md
```

Example:

```text
1843-migrate-git-hooks-scripts-from-bash-to-rust.md
```

#### EPIC

```text
{EPIC_ISSUE_NUMBER}-{short-description}.md
```

Example:

```text
1978-configuration-overhaul-epic.md
```

### Folder-based specification

#### Issue (not part of an EPIC)

```text
{ISSUE_NUMBER}-{short-description}/ISSUE.md
```

Example:

```text
2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md
```

#### EPIC spec

```text
{EPIC_ISSUE_NUMBER}-{short-description}/EPIC.md
```

Example:

```text
1669-overhaul-packages/EPIC.md
```

#### Subissue (part of an EPIC)

```text
{SUB_ISSUE_NUMBER}-{EPIC_ISSUE_NUMBER}-{short-description}/ISSUE.md
```

Where:

- `{SUB_ISSUE_NUMBER}` — GitHub issue number of the subissue itself
- `{EPIC_ISSUE_NUMBER}` — GitHub issue number of the parent EPIC

Example:

```text
docs/issues/closed/1859-1669-move-tracker-policy-and-private-mode-to-primitives/ISSUE.md
```

#### Subissue with explicit implementation order

An optional `si-{N}` segment can be added between the EPIC number and the description when
the implementation order within the EPIC is significant and worth surfacing in the filename:

```text
{SUB_ISSUE_NUMBER}-{EPIC_ISSUE_NUMBER}-si-{ORDER}-{short-description}/ISSUE.md
```

Where:

- `si-{N}` — "subissue N" in the EPIC's implementation order

Example:

```text
docs/issues/closed/1965-1669-si-34-consolidate-duplicate-http-types/ISSUE.md
```

## Key Rule

**The most important part is the issue number prefix.** It makes it easy to locate any spec
from a GitHub issue number and vice versa. Always start the filename or folder name with the
GitHub issue number.

## Summary Table

| Pattern             | Example                                                                                    |
| ------------------- | ------------------------------------------------------------------------------------------ |
| Standalone issue    | `1843-migrate-git-hooks-scripts-from-bash-to-rust.md`                                      |
| Standalone EPIC     | `1978-configuration-overhaul-epic.md`                                                      |
| Folder-based issue  | `2022-vendor-and-document-maintainer-merge-workflow/ISSUE.md`                              |
| EPIC                | `1669-overhaul-packages/EPIC.md`                                                           |
| Subissue            | `docs/issues/closed/1859-1669-move-tracker-policy-and-private-mode-to-primitives/ISSUE.md` |
| Subissue with order | `docs/issues/closed/1965-1669-si-34-consolidate-duplicate-http-types/ISSUE.md`             |
