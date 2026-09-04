---
doc-type: issue
issue-type: task
status: open
priority: p2
epic: null
github-issue: 2138
spec-path: docs/issues/open/2138-document-testing-strategy/ISSUE.md
branch: "2138-document-testing-strategy"
related-pr: null
last-updated-utc: 2026-09-04 16:45
semantic-links:
  skill-links:
    - create-issue
    - write-markdown-docs
    - write-unit-test
  related-artifacts:
    - docs/index.md
    - AGENTS.md
    - tests/AGENTS.md
    - packages/AGENTS.md
    - packages/e2e-tools/README.md
    - tests/lifecycle/native_tracker.rs
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - .github/workflows/testing.yaml
    - .github/workflows/container.yaml
    - .github/workflows/db-compatibility.yaml
    - docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md
---

<!-- skill-link: create-issue -->

# Issue #2138 - Document the Testing Strategy and Test Layers

Related EPIC: [#1347 — Overhaul: Packages Testing](https://github.com/torrust/torrust-tracker/issues/1347)

## Goal

Add a concise, human-facing testing strategy guide that explains the test types
used by Torrust Tracker, why each layer exists, which evidence it provides, and
where to find representative examples and authoritative procedures.

The guide must improve discovery without duplicating the detailed instructions
already owned by scoped `AGENTS.md` files, skills, scripts, CI workflows, ADRs,
or package READMEs.

## Background

Testing knowledge is currently distributed across repository instructions,
package and root integration-test guidance, specialized skills, CI workflows,
ADRs, package READMEs, and issue-local test plans. These documents contain good
technical detail, but a contributor does not have one entry point for answering
these questions:

- Which test layer should cover a change?
- What does each layer prove, and what does it deliberately not prove?
- When should a test be unit, package integration, root application integration,
  executable-boundary integration, container E2E, database compatibility, or
  manual verification?
- Which commands and existing tests are the best starting examples?

The proposed guide will provide that map. It does not replace the source of
truth for any existing procedure.

### Current Test Taxonomy

The guide must present the layers in the taxonomy the maintainers use:

- **Unit tests** — colocated with the code they exercise, inside each package.
- **Integration tests**, in two forms:
  - **In-process** — tests that call functions below the `main` level, either
    package-level tests in `packages/*/tests/` or root-level tests in `tests/`
    that drive the full application through the application container
    (`app::start()`); see `tests/AGENTS.md`.
  - **Executable-boundary** — tests that spawn the compiled tracker binary as a
    child process to verify OS-level behavior such as signal handling; see
    `tests/lifecycle/native_tracker.rs`.
- **End-to-end (E2E) tests**, in two forms, both driven by the runners in
  `packages/e2e-tools`:
  - **Container** — the tracker runs in a Docker/Podman image and is exercised
    with the project's own clients (`e2e_tests_runner`).
  - **Container plus a real BitTorrent client** — the tracker image is exercised
    by qBittorrent (`qbittorrent_e2e_runner`), per database backend.

### Testing Strategy

The guide must state the maintainers' strategy explicitly:

1. **More unit tests are better.** Unit tests are the preferred layer and the
   primary target for coverage growth.
2. **Test as close to the code as possible.** Coverage is being increased at
   the package level; a test that can live in a package must not be promoted to
   the root or to E2E.
3. **Root-level integration tests are reserved** for behavior that requires
   multiple services orchestrated by the tracker application container
   (multiple listeners, aggregate metrics, job manager, shutdown coordination);
   `tests/AGENTS.md` is the authoritative guidance for that boundary.
4. **E2E tests are the outermost safety net**, not the default. They prove the
   packaged artifact and real-client interoperability, and they are the slowest
   and least precise layer.

### History

The project had no automated tests roughly three years ago. E2E tests were the
first layer added because they were the only kind that could be introduced
without restructuring the code. Since then, the codebase has been progressively
refactored into workspace packages precisely to make unit and package-level
integration tests possible. The guide should record this so contributors
understand why the E2E suite is proportionally large and why the direction of
travel is toward lower-level tests, not more E2E coverage.

This work supports [EPIC #1347](https://github.com/torrust/torrust-tracker/issues/1347),
which increases package-level test coverage across the workspace. The guide
provides the selection criteria and navigation contributors need to make those
package-level additions at the appropriate layer.

## Scope

### In Scope

- Create `docs/testing.md` as the documentation entry point for the testing
  strategy.
- Describe the repository's test layers, their purpose, appropriate use, and
  limits, following the taxonomy in [Current Test Taxonomy](#current-test-taxonomy):
  1. unit and documentation tests;
  2. in-process integration tests — package level (`packages/*/tests/`);
  3. in-process integration tests — root application level (`tests/`);
  4. executable-boundary integration tests (`tests/lifecycle/`);
  5. container E2E tests (`e2e_tests_runner`);
  6. container plus real-client E2E tests (`qbittorrent_e2e_runner`), including
     the database compatibility matrix;
  7. manual verification; and
  8. benchmark and profiling workflows, explicitly distinguished from tests.
- State the [Testing Strategy](#testing-strategy) (prefer unit tests, test
  close to the code, reserve root-level tests for orchestration, E2E as the
  outermost net) and the [History](#history) explaining the current E2E-heavy
  distribution.
- For each layer, link to a representative in-repository example and the
  authoritative detailed guide, command, workflow, or configuration.
- Explain the validation ownership boundary among developer-focused checks,
  pre-commit, pre-push, and CI.
- Add one navigational link from `docs/index.md`.
- Correct directly related stale documentation discovered while validating the
  new guide only when the correction is small, factual, and in scope; otherwise
  record a follow-up.

### Out of Scope

- Rewriting existing scoped guidance into `docs/testing.md`.
- Changing test behavior, Cargo test-target configuration, test fixtures, CI
  workflows, hook scripts, coverage thresholds, container images, or database
  matrices.
- Retrofitting every historical issue plan, package README, or archived document
  with a link to the new guide.
- Mandating a fixed number of tests or changing the existing risk-based test-gap
  policy.
- Creating a new generic test framework, test taxonomy library, or test-only
  crate.

## Architectural Decisions

- Related ADRs:
  - `docs/adrs/20260603000000_keep_unit_tests_inside_container_build.md`
  - `docs/adrs/20260826124959_use_explicit_identifiers_for_test_log_assertions.md`
- ADRs to create: `None known`. This task documents existing decisions and
  responsibilities. Escalate only if implementation reveals a new, durable
  architectural policy rather than a documentation-navigation decision.

## Design and Ownership Review

Not applicable. This is documentation work and creates no child-process,
asynchronous-I/O, network-readiness, resource-cleanup, or reusable-fixture
implementation.

The guide must instead maintain clear documentation ownership:

| Topic                    | Authoritative detailed source                             | Role of `docs/testing.md`                                             |
| ------------------------ | --------------------------------------------------------- | --------------------------------------------------------------------- |
| Repository quality gates | `AGENTS.md`; hook scripts; pre-commit and pre-push skills | Summarize the gate boundary and link outward.                         |
| Package tests            | `packages/AGENTS.md`; package-local docs/tests            | Explain selection and link to package guidance.                       |
| Root integration tests   | `tests/AGENTS.md`                                         | Explain when full application context is needed and link to examples. |
| Executable-boundary      | `tests/AGENTS.md`; `tests/lifecycle/`                     | Explain the child-process boundary and link to the native runner.     |
| E2E runners              | `packages/e2e-tools/README.md`; compose files             | Explain what each runner proves and link to usage.                    |
| Test design conventions  | `write-unit-test` skill                                   | Link to naming, AAA, determinism, and fixture-design requirements.    |
| CI and E2E coverage      | CI workflows and container-build ADR                      | Describe guarantees and limits, then link outward.                    |
| Manual verification      | Issue specs and specialized testing skills                | Explain its complementary evidence role and link to procedures.       |

## Proposed Documentation Shape

`docs/testing.md` should contain these sections:

1. **Purpose and strategy** — prefer the lowest-cost test layer that can prove
   the required observable behavior; add higher layers only for boundaries the
   lower layer cannot execute. State the maintainers' strategy: more unit tests,
   closer to the code, root-level only for multi-service orchestration, E2E as
   the outermost net.
2. **Why the suite looks the way it does** — a short history paragraph: no
   tests three years ago, E2E first because it needed no refactoring, package
   extraction since then to enable lower-level tests.
3. **Test layers** — a table with columns for layer, when to use it, what it
   proves, what it does not prove, representative example, and authoritative
   procedure. Group rows as unit / integration (in-process package, in-process
   root, executable-boundary) / E2E (container, container plus qBittorrent).
4. **Validation ownership** — distinguish focused local checks, pre-commit,
   pre-push, CI, and manual verification. State that CI is merge authority.
5. **Writing maintainable tests** — concise links to Test Desiderata, AAA,
   deterministic clocks, test helpers, explicit log identifiers, and lifecycle
   fixture constraints without copying their detailed guidance.
   Before drafting this section, check whether [PR #2137](https://github.com/torrust/torrust-tracker/pull/2137)
   has merged. If it has, link the new
   `docs/testing/refactoring-patterns/README.md` catalog as a source for
   maintainability, readability, and expressiveness improvements. If it has
   not merged, do not block this issue or describe the catalog as available on
   `develop`; retain the PR as the related forthcoming source instead.
6. **Tests versus benchmarks and profiling** — explain that performance tools
   measure behavior and regressions but are not correctness gates.
7. **Further reading** — links to the existing detailed documentation.

The document must use relative links and preserve the current canonical source
of truth for commands and procedures. Do not duplicate command blocks that are
already maintained by hook skills or CI workflow files.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                             | Notes / Expected Output                                                                                              |
| --- | ------ | -------------------------------- | -------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Verify the testing inventory     | Confirmed layers, ownership, examples, and workflows, including the merged refactoring-pattern catalog.               |
| T2  | DONE   | Draft the testing strategy guide | Added `docs/testing.md` with concise strategy, layer, ownership, and reference navigation.                           |
| T3  | DONE   | Add documentation navigation     | Linked `docs/testing.md` from `docs/index.md`.                                                                        |
| T4  | DONE   | Validate source links and claims | All local links resolve; `linter markdown` and `linter cspell` pass.                                                 |
| T5  | DONE   | Review completion evidence       | Acceptance criteria and manual evidence reviewed; no material discovery requires a retrospective.                    |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/document-testing-strategy/ISSUE.md`
- [x] Draft reviewed and approved by user/maintainer
- [x] GitHub issue [#2138](https://github.com/torrust/torrust-tracker/issues/2138) created and issue number added to this spec
- [x] Draft moved to `docs/issues/open/2138-document-testing-strategy/ISSUE.md`
- [x] Implementation completed
- [x] Automatic verification completed (`linter all` and relevant documentation checks)
- [x] Manual link/claim review executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-09-03 16:00 UTC - GitHub Copilot - Created a temporary draft after a
  repository inventory found distributed testing guidance without a human-facing
  testing strategy index.
- 2026-09-04 07:45 UTC - GitHub Copilot - Moved draft to `docs/issues/drafts/`
  and incorporated maintainer input: explicit taxonomy (unit / in-process and
  executable-boundary integration / container and qBittorrent E2E), testing
  strategy (prefer unit, test close to code, root-level for orchestration only),
  and project history (E2E-first origin). -
  `docs/issues/drafts/document-testing-strategy/ISSUE.md`
- 2026-09-04 08:20 UTC - GitHub Copilot - Classified #1347 as a related EPIC,
  not the parent: this cross-cutting documentation task supports package-level
  coverage work but does not directly add coverage for one package. -
  `docs/issues/drafts/document-testing-strategy/ISSUE.md`
- 2026-09-04 09:00 UTC - GitHub Copilot - Added an implementation-time check
  for [PR #2137](https://github.com/torrust/torrust-tracker/pull/2137). Link
  its refactoring-pattern catalog only if it has merged; its availability does
  not block this issue. - `docs/issues/drafts/document-testing-strategy/ISSUE.md`
- 2026-09-04 09:15 UTC - GitHub Copilot - Created GitHub issue
  [#2138](https://github.com/torrust/torrust-tracker/issues/2138) after
  maintainer approval; moved the specification to the open-issues directory. -
  `docs/issues/open/2138-document-testing-strategy/ISSUE.md`
- 2026-09-04 09:25 UTC - GitHub Copilot - Placed the spec-only commit on
  `2138-document-testing-strategy-spec`; the base branch name remains reserved
  for implementation. -
  `docs/issues/open/2138-document-testing-strategy/ISSUE.md`
- 2026-09-04 16:30 UTC - GitHub Copilot - Verified the testing inventory and
  the merged PR #2137 refactoring-pattern catalog; added `docs/testing.md` and
  linked it from `docs/index.md`. Local relative-link validation, `linter
  markdown`, and `linter cspell` pass. - `docs/testing.md`; `docs/index.md`
- 2026-09-04 16:35 UTC - GitHub Copilot - Rechecked every acceptance criterion
  against the guide, index, linked sources, and validation evidence. No
  assumptions were invalidated and no material design change or reusable lesson
  beyond the merged refactoring-pattern catalog was discovered; a separate
  retrospective is unnecessary. - `docs/testing.md`; `docs/index.md`
- 2026-09-04 16:45 UTC - Task Reviewer - Independently reviewed the completed
  guide against all acceptance criteria. The initial review found and the
  implementation replaced a moving issue-specification link with the stable
  issue-workflow documentation; the follow-up review passed with no blockers. -
  `docs/testing.md`; `docs/issues/README.md`

## Acceptance Criteria

- [x] AC1: `docs/testing.md` describes every current major testing and
      verification layer: unit/docs, package integration, root integration,
      executable-boundary, container/qBittorrent E2E, database compatibility,
      manual verification, and benchmarks/profiling distinction.
- [x] AC2: The guide states the testing strategy (prefer unit tests, test close
      to the code, root-level tests only for multi-service orchestration, E2E as
      the outermost net) and the history explaining the E2E-heavy origin.
- [x] AC3: For every described layer, the guide explains when to use it, the
      behavior it can prove, and a meaningful limitation or non-guarantee.
- [x] AC4: Each layer links to at least one current representative example and
      to its detailed authoritative procedure, workflow, configuration, or
      scoped guidance where one exists.
- [x] AC5: The guide distinguishes focused developer checks, pre-commit,
      pre-push, CI merge authority, and manual verification without duplicating
      commands maintained elsewhere.
- [x] AC6: The guide accurately states that benchmarks and profiling complement
      but do not replace correctness testing.
- [x] AC7: `docs/index.md` links to the new guide.
- [x] AC8: The guide does not introduce conflicting commands, test
      requirements, or duplicate policy sources of truth.
- [x] AC9: `linter all` exits with code `0`.
- [x] AC10: Relevant documentation checks pass.
- [x] AC11: A reviewer can verify all test-layer claims and links against the
      current repository.

## Verification Plan

Define verification before implementation starts and execute it before closing
the issue.

### Automatic Checks

- `linter markdown`
- `linter cspell`
- `linter all`
- Markdown link/path validation available in the repository, if any

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                        | Command/Steps                                                                                                              | Expected Result                                                                           | Status | Evidence                 |
| --- | ------------------------------- | -------------------------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------- | ------ | ------------------------ |
| M1  | Verify test-layer inventory     | Compare every row in `docs/testing.md` with the linked `AGENTS.md`, skill, workflow, ADR, and representative test/example. | Every claimed layer has a valid source and accurate scope.                                | DONE   | Inventory and local link-target check, 2026-09-04 |
| M2  | Verify documentation navigation | Open `docs/index.md`, follow the testing-guide link, and inspect the guide's references.                                   | The guide is discoverable and links resolve to current repository artifacts.              | DONE   | `docs/index.md` link and local link-target check, 2026-09-04 |
| M3  | Verify non-duplication          | Compare commands/procedures in the guide with authoritative hook skills and workflow files.                                | The guide links to detailed procedures rather than creating a conflicting command source. | DONE   | Manual source-of-truth review, 2026-09-04 |
| M4  | Verify strategy and history     | Read the strategy and history sections against `tests/AGENTS.md` and maintainer input recorded in this spec.               | The guide states the preferred-layer ordering and the E2E-first origin accurately.        | DONE   | Manual review against issue specification and `tests/AGENTS.md`, 2026-09-04 |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                             |
| ----- | ---------------------- | ------------------------------------ |
| AC1   | DONE                   | Test Layers table; manual inventory review |
| AC2   | DONE                   | Strategy and Why the Suite Looks This Way sections |
| AC3   | DONE                   | Test Layers table; M1 review |
| AC4   | DONE                   | Representative links and local link-target check |
| AC5   | DONE                   | Validation Ownership section; M3 review |
| AC6   | DONE                   | Tests, Benchmarks, and Profiling section |
| AC7   | DONE                   | `docs/index.md`; M2 review |
| AC8   | DONE                   | Manual source-of-truth review; linked detailed procedures |
| AC9   | DONE                   | `linter all`, 2026-09-04 |
| AC10  | DONE                   | `linter markdown` and `linter cspell`, 2026-09-04 |
| AC11  | DONE                   | M1–M4 manual review; PR review pending |

## Risks and Trade-offs

| Risk                                                                          | Mitigation                                                                                                                         |
| ----------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| The guide duplicates volatile commands and becomes stale.                     | Keep commands in their existing owning skills, scripts, and workflows; link to them rather than copying command blocks.            |
| The guide overstates what one layer proves.                                   | Require a limitation/non-guarantee for every layer and cite the container-build ADR for environment-boundary claims.               |
| The guide becomes a broad testing tutorial rather than repository navigation. | Keep it concise and link to project-specific detailed sources.                                                                     |
| Existing documentation has stale paths or descriptions.                       | Correct only small factual issues found while validating a guide link; record broader repairs as separately scoped follow-up work. |
| Contributors read the E2E-heavy suite as the model to follow.                 | State the strategy and history explicitly so the guide steers new tests toward unit and package-level layers.                      |

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: Not needed. The implementation followed the approved
  navigation-only design. The merged refactoring-pattern catalog was included as
  planned; no material discovery or deviation warrants a separate record.
- If needed, create `implementation-retrospective.md` from
  `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in the future issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.

## References

- [Related EPIC #1347 — Overhaul: Packages Testing](https://github.com/torrust/torrust-tracker/issues/1347)
- [Forthcoming refactoring-pattern catalog — PR #2137](https://github.com/torrust/torrust-tracker/pull/2137)
- [Root repository instructions](../../../../AGENTS.md)
- [Package instructions](../../../../packages/AGENTS.md)
- [Root integration-test instructions](../../../../tests/AGENTS.md)
- [Executable-boundary lifecycle test](../../../../tests/lifecycle/native_tracker.rs)
- [E2E tools package](../../../../packages/e2e-tools/README.md)
- [Test-writing skill](../../../../.github/skills/dev/testing/write-unit-test/SKILL.md)
- [Pre-commit validation skill](../../../../.github/skills/dev/git-workflow/run-pre-commit-checks/SKILL.md)
- [Pre-push validation skill](../../../../.github/skills/dev/git-workflow/run-pre-push-checks/SKILL.md)
- [Testing CI workflow](../../../../.github/workflows/testing.yaml)
- [Container CI workflow](../../../../.github/workflows/container.yaml)
- [Database compatibility CI workflow](../../../../.github/workflows/db-compatibility.yaml)
- [Container build testing ADR](../../../adrs/20260603000000_keep_unit_tests_inside_container_build.md)
- [Test log assertion ADR](../../../adrs/20260826124959_use_explicit_identifiers_for_test_log_assertions.md)
