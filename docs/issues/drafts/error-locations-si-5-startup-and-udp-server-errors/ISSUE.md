---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p2
epic: null
github-issue: null
spec-path: docs/issues/drafts/error-locations-si-5-startup-and-udp-server-errors/ISSUE.md
branch: "{issue-number}-error-locations-si-5-startup-and-udp-server-errors"
related-pr: null
last-updated-utc: "2026-11-11 17:39"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/drafts/remove-source-locations-from-errors/EPIC.md
    - docs/issues/open/2804-location-caller-track-caller-audit/inventory.md
    - packages/configuration/src/lib.rs
    - packages/axum-server/src/tls.rs
    - packages/udp-server/src/error.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Remove source locations from the configuration, TLS and UDP server errors

Parent EPIC: [Remove source locations from errors](../remove-source-locations-from-errors/EPIC.md) (draft, GitHub issue not
created yet, so `epic` is `null` and this folder carries no EPIC number). Sub-issue SI-5 of the EPIC.

## Goal

Remove the remaining `location` fields and `LocatedError` sources from
`packages/configuration`, `packages/axum-server` and `packages/udp-server`, so that startup errors
shown to an operator and the UDP server's own errors hold no source path.

## Background

These are the uses left after SI-2 to SI-4 (`docs/issues/open/2804-location-caller-track-caller-audit/options.md`, facts 1 and 2).

| Package | Use | Audience |
| --- | --- | --- |
| `configuration` | Four public variants of `Error` whose source is a `LocatedError` and whose format prints it (`lib.rs:312-337`: `UnableToProcessExplicitConfigFile`, `UnableToLoadFromEnvironmentVariable`, `UnableToLoadFromConfigFile`, `ConfigError`); the `#[track_caller]` `From<figment::Error>` (`lib.rs:348`); `attach_explicit_config_path` (`lib.rs:258`) | Operator, at startup |
| `axum-server` | `Error::BadTlsConfig` with a `LocatedError` source (`tls.rs:25`, built at `:53`); `Error::MissingTlsConfig` with a `location` field its format does not print (`tls.rs:19`, built at `:43`, inventory row P16) | Operator, at startup |
| `udp-server` | `Error::Internal` and `Error::AuthRequired` print `{location}` (`error.rs:36`, `:43`). Both are built only in `#[cfg(test)]` code | None in production today |

For `configuration` the location is correct as a chain (it is the `?` line in the loader), and it is
still a library path in a message for an operator who needs the configuration key and the file, not
the tracker's source line.

Release steps: `torrust-tracker-configuration` was published at 3.0.0 and the publish plan schedules
4.0.0 (#2583), so changing the type of a public source is inside that release. `axum-server` and
`udp-server` are unpublished (0.1.0).

## Scope

### In Scope

- `configuration`: replace the four `LocatedError` sources with the plain source error; remove the
  `#[track_caller]` conversion's location capture and the `Located`/`DynError` use in
  `attach_explicit_config_path`; keep the path and the figment diagnostic in the message.
- `axum-server`: replace the `BadTlsConfig` source with the plain error; remove the `location` field
  of `MissingTlsConfig` and its capture.
- `udp-server`: remove `location` from `Internal` and `AuthRequired` and from the tests that build
  them (`handlers/error.rs`, `event.rs`).
- Remove the `torrust-located-error` dependency from `packages/configuration/Cargo.toml` and
  `packages/axum-server/Cargo.toml`.
- Record the breaking change of `configuration` for its 4.0.0 release.

### Out of Scope

- Removing the two test-only `udp-server` variants altogether: see the draft
  `docs/issues/drafts/follow-up-si-19-remove-unused-error-variants/`.
- `packages/tracker-client`, which imports `torrust_located_error::DynError` (SI-6).
- Changing configuration loading behavior or TLS setup.

## Architectural Decisions

- Related ADRs: the ADR created by sub-issue SI-1 of the parent EPIC (errors do not record source locations); `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
  (`#[non_exhaustive]` policy for public error enums).
- ADRs to create: None known

During implementation, stop and create an ADR when a decision affects project
architecture or design patterns, selects an approach among meaningful
alternatives, or has consequences future contributors need to understand. Do not
create ADRs for routine implementation details or style choices already governed
by project conventions.

## Design and Ownership Review

Not applicable: the work changes error types and their text. It adds no child process,
asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

Removing a location that a client can read is substantively a bug fix (misleading output), so this
issue follows `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- analysis: the call paths and audiences are in the audit inventory
  (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, T2 table); re-check them against the tree before the first change;
- real-artifact reproduction: start the tracker with a configuration file that has an invalid value and, separately, with TLS enabled and a certificate file that is not valid PEM; record the startup error text;
- regression-test boundary: see Regression Test Strategy;
- red regression-test evidence, production fix, green test evidence, and the same reproduction run
  again after the last production change, all recorded in `manual-verification-evidence.md`.

## Regression Test Strategy

- **Smallest boundary**: a unit test at the error's constructor or `FromStr`/`TryFrom` seam that
  asserts the rendered `Display` holds no source path and still names what failed. It needs no
  server. Write it first and record its red output.
- **Trait-dispatch case**: where the error is built in a `FromStr` or `TryFrom` impl, call through
  `str::parse::<T>()` or `try_into()`, the call shape real callers use.
- **Portable path check**: a helper forbids these substrings: `.rs:`, `packages/`, `packages\`,
  `src/`, `src\`, `.cargo/registry` and `rustlib/src`, so workspace paths, registry checkouts, CI
  checkout paths and Windows paths are all caught.
- **Seams for this sub-issue**: direct `to_string()` tests. For TLS use invalid PEM files through `make_rust_tls` (`BadTlsConfig`); a missing certificate gives `MissingTlsConfig`, whose text has no location today, so it is not a red test. For `udp-server`, format the two variants directly, because no UDP reply builds them.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce and record | Startup error text for a bad configuration file and for invalid PEM files, in `manual-verification-evidence.md`. |
| T2 | TODO | Red tests | `to_string()` tests for the four configuration variants, `BadTlsConfig` and the two UDP variants; red output recorded. |
| T3 | TODO | `configuration` | Plain sources, no location capture, dependency removed, 4.0.0 note recorded. |
| T4 | TODO | `axum-server` | Plain `BadTlsConfig` source, `MissingTlsConfig` without `location`, dependency removed. |
| T5 | TODO | `udp-server` | The two variants and their tests without `location`. |
| T6 | TODO | Green and recheck | Green output, then the T1 startups repeated. |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The red regression tests | Commit after focused validation and required review. |
| T3 | `configuration` | Commit after focused validation and required review. |
| T4 | `axum-server` | Commit after focused validation and required review. |
| T5 | `udp-server` | Commit after focused validation and required review. |
| T1, T6 | Evidence | Commit after focused validation and required review. |

Record a justified no-change decision in the task's evidence without creating an empty commit. For
test-producing work, use the `write-unit-test` skill and complete an explicit design review after
each passing test increment, before maintainer review and commit. Confirm that the test exposes the
one causal initial-state difference; its fixture owns only incidental mechanics; and the production
Act plus independently specified expected result remain visible. The review must use the mandatory
prose-first Arrange-Act-Assert comparison: write temporary prose for each section, refactor until
the code expresses it, remove redundant prose, and retain only irreducible context. Record this
review in task evidence or a file-local test plan. Assess helper boundaries by meaningful named
actions and abstraction-level alignment, not caller count: a single-use helper is valid when it
keeps the test readable and hides only incidental mechanics. Commit each reviewed test-design
increment before starting the next planned file or behavior area. Keep final verification and
completion evidence separate when it improves reviewability. Use a Conventional Commit message
with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/error-locations-si-5-startup-and-udp-server-errors/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-11 17:05 UTC - Claude Code (Fable 5.2) - Drafted from the audit of issue #2804 (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, `options.md`) as a sub-issue of the EPIC draft. Not yet approved; line numbers are those of `develop` at `4f0af379a` and must be re-checked before implementation.

## Acceptance Criteria

- [ ] AC1: `configuration::Error`, `axum_server::tls::Error` and the `udp-server` `Error` have no `location` field and no `LocatedError` source.
- [ ] AC2: The startup error for a bad configuration file names the file and the invalid key or value and holds no source path.
- [ ] AC3: The startup error for invalid PEM files holds no source path.
- [ ] AC4: `packages/configuration/Cargo.toml` and `packages/axum-server/Cargo.toml` no longer depend on `torrust-located-error`.
- [ ] AC5: The breaking change of `torrust-tracker-configuration` is recorded for its 4.0.0 release.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `linter all`
- Relevant tests for changed components
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Bad configuration file | Run the tracker with `TORRUST_TRACKER_CONFIG_TOML_PATH` pointing to a file with an invalid value | The error names the file and the problem and holds no `.rs:` path | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Invalid TLS files | Enable TLS for the HTTP tracker with a certificate file that is not valid PEM and start the tracker | The error says the TLS configuration is bad and holds no path | TODO | `manual-verification-evidence.md` section V2 |

Notes:

- Manual verification is mandatory even when automated tests pass. It is a
  real human-oriented use of the feature or reproduction of the bug fix, not a
  simulated result and not merely running automated tests.
- Every recorded validation command result must identify the toolchain or
  runtime that produced it when one can affect behavior. For example, record
  `cargo +nightly fmt --all -- --check` as `nightly Rust toolchain`, rather than
  the ambiguous `cargo fmt --all -- --check`.
- Create `manual-verification-evidence.md` from
  `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when executing these
  scenarios. Record actual prerequisites, actions, commands, program output,
  relevant tracker logs, and outcomes there.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

Temporary scripts may automate an issue-local verification scenario, but they
are neither maintained automatic tests nor manual-verification evidence. Before
creating one, record in the issue specification:

- why temporary automation is better for this concrete scenario than a
  maintained Rust automatic test;
- the script's issue-local path, what it verifies, and its intended removal or
  retention owner; and
- when using Python instead of Rust, why Rust is not suitable for that specific
  script.

Keep the script in the issue-specification folder so later reviewers can inspect
the verification performed. Promote durable product-behavior checks into Rust
automatic tests when practical, then remove the disposable script.

None planned for this issue.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **The figment diagnostic already carries enough context, or not.** Mitigation: T1 records the
  current text so the new text can be compared.
- **`configuration` 4.0.0 is published first.** Mitigation: EPIC ordering; otherwise this needs a
  later major.

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
- When an independent reviewer receives this folder-style specification, it
  records its result in `agent-review-reports.md` using
  `docs/templates/AGENT-REVIEW-REPORTS.md`. Do not create that artifact for a
  legacy standalone specification or when no folder-style specification was
  supplied.

## References

- Parent EPIC draft: `docs/issues/drafts/remove-source-locations-from-errors/EPIC.md`
- Audit: `docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md`, `inventory.md`, `options.md`, `experiments.md` (issue #2804)
- Related issues: #2804 (audit), #384 (external report), #2583 (first publish of the workspace crates)
- Related draft: `docs/issues/drafts/follow-up-si-19-remove-unused-error-variants/`
