---
semantic-links:
  skill-links:
    - write-unit-test
  related-artifacts:
    - docs/testing/README.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
---

# Test Refactoring-Pattern Catalog

Use this catalog when generated or existing test code is correct but does not clearly communicate
the behavior it protects. Entries describe reviewed, repository-native patterns; they complement
the mandatory conventions in the [unit-test skill](../../../.github/skills/dev/testing/write-unit-test/SKILL.md).

## Entries

| Pattern                                                                                                | Use it when                                                                                        | Representative source                                   |
| ------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| [Scenario fixture with independent expected outputs](scenario-fixture-independent-expected-outputs.md) | One domain input must be verified through multiple independently decoded response representations. | `packages/axum-http-server/src/v1/handlers/announce.rs` |

## Entry Requirements

Each entry must include:

1. The readability or maintainability problem that triggered the refactor.
2. The selected pattern and its essential constraints.
3. Appropriate and inappropriate uses.
4. A repository source example and its originating issue, when applicable.
5. How the pattern preserves deterministic execution and one behavior-focused contract.
