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
| [Scenario fixtures for causal initial state](scenario-fixtures-for-causal-initial-state.md)            | Several setup operations establish the one state that makes the Act behave differently.            | `packages/axum-http-server/src/server.rs`               |
| [Prose-first Arrange-Act-Assert verification](prose-first-arrange-act-assert-verification.md)          | A correct test is hard to read because its code does not yet express its behavioral intent.        | `packages/udp-server/src/handlers/mod.rs`               |
| [Named helpers for abstraction-level alignment](named-helpers-for-abstraction-level-alignment.md)       | A coherent setup action is obscured by low-level mechanics or rejected only because it has one caller. | `packages/udp-server/tests/server/contract.rs`        |

## Entry Requirements

Each entry must include:

1. The readability or maintainability problem that triggered the refactor.
2. The selected pattern and its essential constraints.
3. Appropriate and inappropriate uses.
4. A repository source example and its originating issue, when applicable.
5. How the pattern preserves deterministic execution and one behavior-focused contract.
