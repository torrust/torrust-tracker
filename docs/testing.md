---
semantic-links:
  skill-links:
    - write-unit-test
    - run-pre-commit-checks
    - run-pre-push-checks
  related-artifacts:
    - docs/testing/README.md
    - tests/AGENTS.md
    - packages/AGENTS.md
    - packages/e2e-tools/README.md
    - .github/workflows/testing.yaml
    - .github/workflows/container.yaml
    - .github/workflows/db-compatibility.yaml
---

# Testing Strategy

This guide helps contributors select the lowest-cost test layer that can prove
an observable behavior. It complements, rather than replaces, the detailed
procedures and conventions linked below.

## Strategy

1. **More unit tests are better.** They are the primary target for coverage
   growth because they are fast, deterministic, and low-maintenance.
2. **Test as close to the code as possible.** Put behavior in a package-level
   test when it can be proved there; do not promote it to the root application
   or E2E layer without a boundary that requires it.
3. **Use root-level integration tests only for orchestration.** These tests are
   for behavior involving multiple services assembled by the tracker application
   container, rather than an individual package.
4. **Use E2E as the outermost safety net.** It validates the packaged artifact
   and real-client interoperability, but is slower and less precise than lower
   layers.

When behavior remains untested, record the reason as required by the
[unit-test skill](../.github/skills/dev/testing/write-unit-test/SKILL.md).

## Why the Suite Looks This Way

The project had no automated tests roughly three years ago. E2E tests were
introduced first because they could be added without refactoring the existing
application. The subsequent extraction and refactoring of workspace packages
made unit and package-level integration tests practical.

The E2E suite is therefore proportionally large, but it is not the default model
for new coverage. The current direction is toward maintainable unit and
package-level tests, as supported by [EPIC #1347](https://github.com/torrust/torrust-tracker/issues/1347).

## Test Layers

| Layer                                         | Use it when                                                                                                      | It proves                                                                                              | It does not prove                                                                           | Representative example                                                                                             | Authoritative guidance                                                                                                                            |
| --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------- |
| Unit and documentation tests                  | A package type, function, or module has behavior that can run without I/O or a deployed service.                 | The focused behavior is correct in isolation.                                                          | Cross-package wiring, process behavior, or a packaged runtime.                              | [`Driver` tests](../packages/primitives/src/driver.rs)                                                             | [Unit-test skill](../.github/skills/dev/testing/write-unit-test/SKILL.md); [package testing guidance](../packages/AGENTS.md#testing-packages)     |
| Package-level in-process integration          | A package boundary needs real collaborators, such as a server and its handler, without the complete application. | The package's components work together through its public boundary.                                    | Application-wide service coordination or the compiled tracker executable.                   | [HTTP server contract tests](../packages/axum-http-server/tests/server/v1/contract/)                               | [Package testing guidance](../packages/AGENTS.md#testing-packages); [test refactoring patterns](testing/refactoring-patterns/README.md)           |
| Root application-level in-process integration | An observable behavior needs the complete application container and multiple coordinated services.               | Application startup, cross-service coordination, aggregate metrics, and job or shutdown orchestration. | OS process boundaries, signals sent to the tracker executable, or container-image behavior. | [Port-zero metrics suite](../tests/metrics/port_zero.rs)                                                           | [Root integration-test guidance](../tests/AGENTS.md)                                                                                              |
| Executable-boundary integration               | The behavior requires starting the compiled tracker as a child process, such as OS-signal handling.              | The native executable starts and reacts correctly at its process boundary.                             | Container-image behavior or interoperability with an external BitTorrent client.            | [Native tracker fixture](../tests/lifecycle/native_tracker.rs)                                                     | [Child-process configuration isolation](../tests/AGENTS.md#child-process-configuration-isolation)                                                 |
| Container E2E                                 | The tracker must be exercised as the built container artifact with project-controlled clients.                   | The image builds and tracker behavior works through its network boundary.                              | Interoperability with a production BitTorrent client or every database backend.             | [`e2e_tests_runner`](../packages/e2e-tools/README.md#binaries)                                                     | [E2E tools usage](../packages/e2e-tools/README.md); [container workflow](../.github/workflows/container.yaml)                                     |
| Container plus qBittorrent E2E                | Compatibility must be demonstrated against a real BitTorrent client and configured database backend.             | The containerized tracker interoperates with qBittorrent for the selected backend.                     | Isolated package behavior or exhaustive coverage of all failure paths.                      | [`qbittorrent_e2e_runner`](../packages/e2e-tools/README.md#binaries)                                               | [E2E tools usage](../packages/e2e-tools/README.md); [container workflow](../.github/workflows/container.yaml)                                     |
| Database compatibility                        | A persistence change affects MySQL or PostgreSQL driver behavior or supported-version compatibility.             | The selected tracker-core database-driver scenarios work against the workflow's version matrix.        | Complete tracker container behavior or SQLite behavior not covered by the scenario.         | [Database compatibility workflow](../.github/workflows/db-compatibility.yaml)                                      | [Database compatibility workflow](../.github/workflows/db-compatibility.yaml); [package testing guidance](../packages/AGENTS.md#testing-packages) |
| Manual verification                           | Automated evidence cannot fully demonstrate a user- or environment-facing outcome.                               | The recorded scenario worked in the reviewed environment.                                              | Repeatable coverage across inputs, platforms, and future changes.                           | [Manual HTTP completion E2E procedure](../.github/skills/dev/testing/manual-http-download-completion-e2e/SKILL.md) | [Issue-specification workflow](issues/README.md); the applicable [testing skill](../.github/skills/dev/testing/)                                  |

## Validation Ownership

Choose focused checks while developing, then use the repository gates for their
respective responsibilities:

| Owner                    | Responsibility                                                                                                                   | Detailed procedure                                                                                               |
| ------------------------ | -------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| Developer-focused checks | Give fast feedback for the changed package, test, documentation page, or workflow. They do not replace repository gates.         | [Package testing guidance](../packages/AGENTS.md#testing-packages)                                               |
| Pre-commit               | Performs the fast local gate: dependency checks, all linters, Containerfile linting, and documentation tests.                    | [Pre-commit checks](../.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md)                           |
| Pre-push                 | Performs nightly checks and the full stable test suite; it intentionally excludes E2E tests.                                     | [Pre-push checks](../.github/skills/dev/git-workflow/run-pre-push-checks/SKILL.md)                               |
| CI                       | Is the merge authority. It runs workflow-selected validation, including container and qBittorrent E2E coverage where applicable. | [Testing workflow](../.github/workflows/testing.yaml); [container workflow](../.github/workflows/container.yaml) |
| Manual verification      | Complements automated evidence with scenario status and recorded evidence in the relevant issue specification.                   | [Issue-specification workflow](issues/README.md)                                                                 |

## Writing Maintainable Tests

The [unit-test skill](../.github/skills/dev/testing/write-unit-test/SKILL.md)
is the source of truth for Test Desiderata, behavior-focused naming, visible
Arrange-Act-Assert structure, deterministic clocks, isolation, and lifecycle
fixture design.

When a correct test does not clearly communicate the behavior it protects,
consult the [test refactoring-pattern catalog](testing/refactoring-patterns/README.md).
The catalog contains reviewed repository-native patterns for improving test
maintainability, readability, and expressiveness without changing the behavior
under test. Use a pattern only where its stated constraints apply; keep the
production Act and observable assertions visible.

Use [test helpers](../packages/test-helpers/) for shared mock servers and test
data. For root integration tests, follow the port isolation, fixture shutdown,
and scenario constraints in [`tests/AGENTS.md`](../tests/AGENTS.md).

## Tests, Benchmarks, and Profiling

Tests establish correctness claims. [Benchmarks](benchmarking.md) measure
performance under defined workloads, while [profiling](profiling.md) identifies
where a workload spends time or memory. These tools can reveal regressions and
guide optimization, but they do not replace correctness tests or the repository
quality gates.

## Further Reading

- [Testing guidance and pattern catalog](testing/README.md)
- [Package architecture and testing guidance](../packages/AGENTS.md)
- [Main application integration-test guidance](../tests/AGENTS.md)
- [Container test rationale ADR](adrs/20260603000000_keep_unit_tests_inside_container_build.md)
- [Test log assertion ADR](adrs/20260826124959_use_explicit_identifiers_for_test_log_assertions.md)
