---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/testing/refactoring-patterns/README.md
    - tests/AGENTS.md
---

# Testing

This directory contains durable testing guidance shared by all workspace packages and the main
application. It is not an issue-spec archive: patterns recorded here remain useful after their
originating issue is closed.

## References

| Resource                                                                     | Purpose                                                                                                         |
| ---------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------- |
| [Test refactoring-pattern catalog](refactoring-patterns/README.md)           | Reviewed examples for improving generated or existing test code without changing the behavior under test.       |
| [Unit-test skill](../../.github/skills/dev/testing/write-unit-test/SKILL.md) | Required workflow and baseline conventions for writing package-local unit tests.                                |
| [Application integration-test guidance](../../tests/AGENTS.md)               | Boundary selection, process isolation, lifecycle, and scenario guidance for main-application integration tests. |

## Adding Catalog Entries

Add one lowercase-kebab-case Markdown file per reviewed refactor under
`docs/testing/refactoring-patterns/`. Each entry must state the problem, the selected pattern,
when to use it, when not to use it, and a representative repository example. Keep the entry about
test design rather than an issue's delivery history.

Link the entry from the catalog README. When the entry changes the test-writing workflow, update
the `write-unit-test` skill in the same change.
