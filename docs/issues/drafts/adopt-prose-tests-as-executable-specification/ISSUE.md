---
schema-version: 1
doc-type: issue
issue-type: task
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/adopt-prose-tests-as-executable-specification/ISSUE.md
branch: "{issue-number}-adopt-prose-tests-as-executable-specification"
related-pr: null
last-updated-utc: "2026-10-03 10:14"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - write-unit-test
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - .github/skills/dev/testing/write-unit-test/SKILL.md
    - docs/testing.md
    - docs/skills/semantic-skill-link-convention.md
    - packages/tracker-core/src/scrape_handler.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Adopt Prose-Style Tests as the Executable Specification

## Goal

Record in an ADR that prose-style tests are the project's executable specification of behavior, so domain contracts live in tests and Rustdoc rather than in extra specification documents, and define how specs, ADRs, Rustdoc, and tests link to each other.

## Background

During PR #2423 (issue #2406) the maintainer separated a domain decision (what scrape reports for `downloaded`) from a technical one (how persisted counts are loaded). The domain contract was not given a new kind of document. Instead it lives in the `ScrapeHandler` module Rustdoc, which names the prose-style tests that specify it, and the ADR keeps only the technical decision.

The maintainer's position:

- The project already has three specification layers: issue specs, Rustdoc, and tests. Adding a fourth (for example a behavior-spec document type) would duplicate them.
- Tests written in Behavior-Driven Development style, with `it_should_...` names and visible Arrange-Act-Assert, are the programmatic specification. Reading the test run output should read like a specification.
- [`cargo-pretty-test`](https://github.com/josecelano/cargo-pretty-test) (by the maintainer) renders `cargo test` output as a readable tree, similar to other languages' test runners.
- Issue specs, ADRs, and Rustdoc should link to the tests that enforce them, and tests should link back.

This convention is repository-wide, so it needs its own ADR rather than living implicitly in one PR.

## Scope

### In Scope

- Write a root ADR that records:
  - tests named as requirements are the executable specification of behavior;
  - domain/business behavior is documented in Rustdoc that names its specifying tests, while ADRs record technical decisions only;
  - no separate behavior-specification document type is introduced;
  - how issue specs, ADRs, and Rustdoc reference tests, and how tests reference issues and ADRs with the existing semantic markers (`// issue: #N`, `// adr: <path>`);
  - whether to recommend `cargo-pretty-test` (or an equivalent) for reading test output as a specification.
- Update the `write-unit-test` and `create-adr` skills and `docs/testing.md` to point to the ADR.

### Out of Scope

- Renaming or rewriting existing tests to the convention.
- Adding new semantic marker kinds unless the ADR shows the current ones cannot express a test link.
- Making a test-output tool a required CI dependency.

## Architectural Decisions

- Related ADRs: `docs/adrs/20261002173716_load_persisted_scrape_downloads_with_a_batched_uncached_lookup.md` (first ADR written under this split).
- ADRs to create: "Use prose-style tests as the executable specification".

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Not applicable.

## Regression Test Strategy

Not applicable.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Inventory current practice | How existing ADRs, specs, and Rustdoc reference tests; examples of prose-style test names and AAA |
| T2 | TODO | Evaluate test-output tooling | `cargo-pretty-test` and alternatives (for example `cargo nextest` output); recommendation and portability notes |
| T3 | TODO | Write the ADR | Root ADR plus index entry |
| T4 | TODO | Update skills and testing docs | `write-unit-test`, `create-adr`, `docs/testing.md` link to the ADR |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T3 | ADR and index entry | Commit after maintainer review |
| T4 | Skill and documentation updates | Commit after linters pass |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/adopt-prose-tests-as-executable-specification/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-03 10:14 UTC - Copilot - Drafted from the maintainer's direction during PR #2423 review.

## Acceptance Criteria

- [ ] AC1: A root ADR states that prose-style tests are the executable specification and that ADRs record technical decisions only.
- [ ] AC2: The ADR defines how issue specs, ADRs, and Rustdoc reference tests and how tests reference issues and ADRs.
- [ ] AC3: The ADR records a decision on test-output tooling such as `cargo-pretty-test`.
- [ ] AC4: The `write-unit-test` and `create-adr` skills and `docs/testing.md` link to the ADR.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Read a package's tests as a specification | Run `cargo test -p torrust-tracker-core` with the recommended output tool, if any, and read the output | The output reads as a list of behaviors | TODO | `manual-verification-evidence.md` section V1 |

### Disposable Verification Scripts

None planned.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | ADR |
| AC2 | TODO | ADR |
| AC3 | TODO | ADR |
| AC4 | TODO | Skill and documentation diffs |

## Risks and Trade-offs

- Rustdoc that names tests can drift when tests are renamed; the ADR should say how to keep them aligned (for example, review the linked Rustdoc when renaming a specifying test).
- A recommended output tool is an optional adapter, not a source of truth (AGENTS.md engineering policy 7).

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from the repository
  template at `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this issue
  specification's directory.
- If no retrospective is needed, add a concise progress-log entry explaining
  why the work had no material discovery.

## References

- Related issues: #2406
- Related PRs: #2423
