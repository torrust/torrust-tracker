# Previous Discussion: Unified Rust Repository Automation Runner

## Status

This document records an earlier exploratory discussion about a potential implementation for
repository automation and guardrails. It is historical design input for the EPIC, not an
approved architecture or implementation plan.

The proposal intentionally makes strong choices so they can be evaluated. The EPIC must compare
it with distributed and incremental alternatives, validate its assumptions against the current
repository, and obtain maintainer approval before adopting any part of it.

## Motivation Discussed

The project relies on multiple automation scripts, GitHub Actions steps, and independently
implemented validation logic. The discussion assumed that continued growth would make this
system increasingly difficult to maintain, extend, and reuse.

The proposed response was to consolidate repository actions and guardrail checks into one
extensible Rust automation framework. Instead of maintaining execution logic across shell
scripts and CI workflow steps, the framework would expose a consistent model usable locally and
in CI.

## Proposed Goals

- Replace scattered automation execution logic with a unified Rust CLI.
- Make actions and checks reusable locally and in CI where their environment permits it.
- Allow new actions and checks to be added without modifying the execution engine.
- Support different validation policies for different execution contexts.
- Share common project metadata across validations.
- Produce consistent output for humans and machines.

These were proposal goals, not conclusions supported by the EPIC inventory or options analysis.

## Proposed Architecture

```text
      +-----------------------+
      | Repository Tool Runner|
      |      (Rust CLI)       |
      +-----------------------+
          |
        Load Policy
          |
        Execution Planner
          |
       +--------------+--------------+
       |                             |
    Actions                        Checks
  update dependencies            formatting / Clippy
   archive issue specs             tests / E2E / bans
```

Each operation would be implemented as an independent **action** or **check**. The runner would
be responsible only for:

- loading configuration;
- resolving dependencies;
- scheduling execution;
- aggregating results; and
- reporting progress and outcomes.

## Operation Model

The earlier discussion used “guardrail” for every operation. This refinement separates three
concepts:

| Type       | Role                                                       | Side effects                   | Example                                            |
| ---------- | ---------------------------------------------------------- | ------------------------------ | -------------------------------------------------- |
| **Action** | Performs repository work                                   | Expected and declared          | Update dependencies, archive completed issue specs |
| **Check**  | Verifies an objective condition                            | Read-only by default           | Formatting, tests, layer-boundary bans             |
| **Policy** | Selects and orders actions/checks for an execution context | Depends on selected operations | Pre-commit, pre-push, CI, nightly, release         |

They may share execution context, scheduling, output, and cache infrastructure, but actions need
dry-run/apply, idempotency, and side-effect safeguards that do not belong to read-only checks.

Candidate checks discussed included:

- Rust formatting;
- Clippy;
- unit tests;
- integration tests;
- end-to-end tests;
- benchmarks;
- documentation checks;
- license validation;
- dependency auditing;
- container image validation;
- API compatibility;
- Torrust-specific project conventions.

Candidate actions include:

- update dependencies;
- archive or clean completed issue specifications;
- prepare branches or commit metadata; and
- install repository Git hooks.

The intended extension model was that adding an action or check would require implementing a new
Rust component without changing the core runner.

## Existing Composite Testing Guardrail

`.github/workflows/testing.yaml` is already a composite CI guardrail. Its current guarantees
include:

- Rust formatting on the nightly matrix entry;
- all configured linters on stable and nightly;
- workspace documentation tests;
- workspace tests, benches, and examples across all targets and features;
- Cargo dependency layer-boundary bans through `cargo deny check bans`;
- successful construction of the tracker container image;
- tracker E2E validation against the container image; and
- qBittorrent E2E validation with SQLite, MySQL, and PostgreSQL.

This existing database-backed E2E coverage replaces the earlier speculative “SQL migration
validation” extension. The inventory must describe the guarantee the tests actually provide and
must not claim migration-schema coverage beyond the observed tests.

The workflow includes setup and image-build actions, but its overall role is a merge/CI
guardrail. A future design may reuse its individual checks without assuming the workflow itself
should disappear.

## Policy Model

Policies would define **what runs**, not **how operations run**. Example policies included:

- `quick`;
- `ci`;
- `release`;
- `nightly`; and
- `benchmark`.

An illustrative mapping was:

| Policy    | Example operations                       |
| --------- | ---------------------------------------- |
| `quick`   | formatting, Clippy                       |
| `ci`      | formatting, Clippy, tests, documentation |
| `release` | all applicable validations               |

This model aimed to keep local feedback fast while scheduling expensive validations less
frequently.

## Dependency Resolution

The proposal assumed that some actions and checks naturally depend on others. Examples included:

- benchmarks require successful tests;
- end-to-end tests require container images; and
- release validation requires successful documentation generation.

The runner would resolve and schedule these dependencies automatically.

## Shared Execution Context

Every action and check would receive a shared execution context containing relevant project metadata,
for example:

- workspace path;
- Cargo metadata;
- Git information;
- environment variables;
- changed files; and
- CI metadata.

The intended benefit was avoiding duplicated repository-discovery logic across guardrails.

## Standardized Results

Every check would return a common result model. Proposed states were:

- passed;
- failed;
- warning; and
- skipped.

Actions would need a related but distinct result model that makes mutation explicit, such as:

- changed;
- unchanged;
- skipped; and
- failed.

Results could include:

- execution time;
- summary; and
- detailed diagnostics.

The common result model was intended to support consistent terminal presentation,
machine-readable event streams, reports, and CI integration. Any future design should align this
idea with the EPIC's JSONL/NDJSON, progress, exit-code, and diagnostics principles.

## Illustrative CLI

```bash
guard run --policy quick
guard run --policy ci
guard run --policy release
guard run fmt
guard run clippy tests
guard run --all
```

The command and binary names were placeholders.

## Proposed CI Integration

The discussion proposed replacing repeated workflow steps such as:

```yaml
- run: cargo fmt
- run: cargo clippy
- run: cargo nextest
- run: ./scripts/check_docs.sh
```

with one policy invocation:

```yaml
- run: cargo guard --policy ci
```

The intended outcome was for the same automation implementation to run locally and in CI.

## Potential Extensions

The proposed framework was expected to support future checks such as:

- API compatibility analysis;
- performance regression detection;
- project-specific architecture rules;
- documentation completeness checks;
- security and supply-chain analysis; and
- custom linting for the Torrust ecosystem.

## Claimed Benefits to Validate

The discussion identified these potential benefits:

- one source of truth for repository operation contracts and policies;
- strongly typed implementation in Rust;
- easier extension with new actions and checks;
- consistent local and CI behavior;
- faster feedback through configurable policies;
- less duplicated shell and workflow logic; and
- a foundation for future quality tooling.

These are hypotheses. The EPIC should validate them against implementation cost, coupling,
failure isolation, portability, startup and compilation overhead, ownership boundaries, and the
cost of centralizing unrelated checks.

## Questions for the EPIC

- Does one runner reduce total complexity, or merely move distributed complexity into a central
  framework?
- Which checks should be native Rust components, and which should remain external commands
  orchestrated through stable adapters?
- Can actions and checks be added without modifying the execution engine in practice, and is a
  plugin mechanism needed or justified?
- Which infrastructure can actions and checks safely share without hiding mutation or weakening
  read-only guarantees?
- How should local, pre-commit, pre-push, CI, nightly, release, and benchmark policies relate?
- How should the dependency graph represent generated artifacts, services, databases, and
  containers in addition to pass/fail prerequisites?
- How are cache keys, result reuse, cancellation, concurrency, timeouts, and retries represented?
- How does the runner stream JSONL/NDJSON progress while preserving actionable human output?
- What remains in GitHub Actions because it is infrastructure orchestration, and which workflows
  remain valuable composite guardrails even if their checks use shared tooling?
- Does compiling or installing the runner create a bootstrapping problem for lightweight checks?
- How can migration happen incrementally without maintaining two conflicting sources of truth?
- What evidence would justify selecting this proposal over improving the current distributed
  system?

## Relationship to the EPIC

The EPIC inventory should map this proposal to current hooks, workflows, skills, agents, and
analysis tools. The options analysis should then compare this model with at least:

1. an improved distributed model with shared contracts;
2. incremental consolidation of only duplicated execution infrastructure; and
3. a unified runner similar to this proposal.

No implementation issue should treat this document as an approved decision unless the EPIC
records that decision after maintainer review.
