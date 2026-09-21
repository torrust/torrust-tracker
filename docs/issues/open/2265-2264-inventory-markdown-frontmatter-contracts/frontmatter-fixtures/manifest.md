# Frontmatter Fixture Manifest

These fixtures define v1 parser and validator expectations for issue #2266. A fixture is evaluated
as a complete Markdown file against the structural layer only: `accepted/` fixtures must be treated
as if they lived in `docs/issues/open/`, and their `spec-path` values are not resolved against the
repository. Whole-tree validation must exclude this `frontmatter-fixtures/` directory; otherwise
the rejected fixtures would fail the gate and the accepted fixtures would fail location checks.

| Fixture | Expected result | Diagnostic category when rejected |
| --- | --- | --- |
| `accepted/issue.md` | Accept as strict issue profile. | None. |
| `accepted/epic.md` | Accept as strict EPIC profile. | None. |
| `rejected/issue-wrong-scalar.md` | Reject. | `wrong-scalar-type`. |
| `rejected/issue-unknown-field.md` | Reject. | `unknown-field`. |
| `rejected/issue-invalid-status.md` | Reject. | `invalid-allowed-value`. |
| `rejected/issue-invalid-reference.md` | Reject. | `invalid-reference-syntax`. |

The implementation additionally tests malformed YAML, unclosed delimiters, and repository-aware
location/path cases; they are not duplicated here because a YAML parser error cannot be represented
as an accepted Markdown fixture.
