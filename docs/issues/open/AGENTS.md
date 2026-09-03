# Agents Instructions — `docs/issues/open/`

## Spec Naming Conventions

All new specifications use a folder. The primary file inside the folder is
lowercase `issue.md` for issues or `epic.md` for EPICs. This keeps supporting
artifacts, including `implementation-retrospective.md`, in the issue directory.
The GitHub issue number must start every folder name. Existing standalone files
and uppercase primary files are legacy; migrate them when materially updating
the specification or adding an issue-local artifact.

### Legacy standalone specification

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

Legacy folders whose primary files are uppercase `ISSUE.md` or `EPIC.md` are
also migrated to lowercase `issue.md` or `epic.md` when they are materially
updated or need an issue-local artifact.

### Required folder-based specification

#### Issue (not part of an EPIC)

```text
{ISSUE_NUMBER}-{short-description}/issue.md
```

Example:

```text
2022-vendor-and-document-maintainer-merge-workflow/issue.md
```

#### EPIC spec

```text
{EPIC_ISSUE_NUMBER}-{short-description}/epic.md
```

Example:

```text
1669-overhaul-packages/epic.md
```

#### Subissue (part of an EPIC)

```text
{SUB_ISSUE_NUMBER}-{EPIC_ISSUE_NUMBER}-{short-description}/issue.md
```

Where:

- `{SUB_ISSUE_NUMBER}` — GitHub issue number of the subissue itself
- `{EPIC_ISSUE_NUMBER}` — GitHub issue number of the parent EPIC

Example:

```text
docs/issues/closed/1859-1669-move-tracker-policy-and-private-mode-to-primitives/issue.md
```

#### Subissue with explicit implementation order

An optional `si-{N}` segment can be added between the EPIC number and the description when
the implementation order within the EPIC is significant and worth surfacing in the filename:

```text
{SUB_ISSUE_NUMBER}-{EPIC_ISSUE_NUMBER}-si-{ORDER}-{short-description}/issue.md
```

Where:

- `si-{N}` — "subissue N" in the EPIC's implementation order

Example:

```text
docs/issues/closed/1965-1669-si-34-consolidate-duplicate-http-types/issue.md
```

## Key Rule

**The most important part is the issue number prefix.** It makes it easy to locate any spec
from a GitHub issue number and vice versa. Always start the filename or folder name with the
GitHub issue number.

## Summary Table

| Pattern                 | Example                                                                                    |
| ----------------------- | ------------------------------------------------------------------------------------------ |
| Legacy standalone issue | `1843-migrate-git-hooks-scripts-from-bash-to-rust.md`                                      |
| EPIC spec               | `1978-configuration-overhaul-epic/epic.md`                                                 |
| Folder-based issue      | `2022-vendor-and-document-maintainer-merge-workflow/issue.md`                              |
| Subissue                | `docs/issues/closed/1859-1669-move-tracker-policy-and-private-mode-to-primitives/issue.md` |
| Subissue with order     | `docs/issues/closed/1965-1669-si-34-consolidate-duplicate-http-types/issue.md`             |
