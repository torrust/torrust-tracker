# Agents Instructions — `docs/issues/open/`

## File Naming Conventions

Spec files in this folder follow one of these naming patterns:

### Standalone issue (not part of an EPIC)

```text
{ISSUE_NUMBER}-{short-description}.md
```

Example:

```text
1875-review-lto-fat-in-dev-profile.md
```

### EPIC spec

```text
{EPIC_ISSUE_NUMBER}-{short-description}.md
```

Example:

```text
1978-configuration-overhaul-epic.md
```

### Subissue (part of an EPIC)

```text
{SUB_ISSUE_NUMBER}-{EPIC_ISSUE_NUMBER}-{short-description}.md
```

Where:

- `{SUB_ISSUE_NUMBER}` — GitHub issue number of the subissue itself
- `{EPIC_ISSUE_NUMBER}` — GitHub issue number of the parent EPIC

Example:

```text
1979-1978-copy-configuration-schema-v2-to-v3-baseline.md
```

### Subissue with explicit implementation order

An optional `si-{N}` segment can be added between the EPIC number and the description when
the implementation order within the EPIC is significant and worth surfacing in the filename:

```text
{SUB_ISSUE_NUMBER}-{EPIC_ISSUE_NUMBER}-si-{ORDER}-{short-description}.md
```

Where:

- `si-{N}` — "subissue N" in the EPIC's implementation order

Example:

```text
1969-1938-si-8-eliminate-unwraps-from-rest-api-client.md
```

## Key Rule

**The most important part is the issue number prefix.** It makes it easy to locate any spec
from a GitHub issue number and vice versa. Always start the filename with the GitHub issue
number.

## Summary Table

| Pattern             | Example                                                    |
| ------------------- | ---------------------------------------------------------- |
| Standalone          | `1875-review-lto-fat-in-dev-profile.md`                    |
| EPIC                | `1978-configuration-overhaul-epic.md`                      |
| Subissue            | `1979-1978-copy-configuration-schema-v2-to-v3-baseline.md` |
| Subissue with order | `1969-1938-si-8-eliminate-unwraps-from-rest-api-client.md` |
