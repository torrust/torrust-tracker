---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p1
epic: null
github-issue: null
spec-path: docs/issues/drafts/error-locations-si-3-http-tracker-request-errors/ISSUE.md
branch: "{issue-number}-error-locations-si-3-http-tracker-request-errors"
related-pr: null
last-updated-utc: "2026-11-11 17:05"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/drafts/remove-source-locations-from-errors/EPIC.md
    - docs/issues/open/2804-location-caller-track-caller-audit/inventory.md
    - packages/http-protocol/src/v1/requests/announce.rs
    - packages/http-protocol/src/v1/requests/scrape.rs
    - packages/http-protocol/src/v1/query.rs
    - packages/http-protocol/src/v1/auth.rs
    - packages/http-protocol/src/v1/services/peer_ip_resolver.rs
    - packages/axum-http-server/src/v1/extractors/announce_request.rs
    - packages/axum-http-server/src/v1/extractors/scrape_request.rs
    - packages/axum-http-server/src/v1/extractors/authentication_key.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Remove source locations from the HTTP tracker request errors

Parent EPIC: [Remove source locations from errors](../remove-source-locations-from-errors/EPIC.md) (draft, GitHub issue not
created yet, so `epic` is `null` and this folder carries no EPIC number). Sub-issue SI-3 of the EPIC.

## Goal

Remove the `location` fields, the `LocatedError` sources and their text from the errors of
`packages/http-protocol`, and from the code in `packages/axum-http-server` that builds them, so that
no HTTP tracker `failure reason` produced by request parsing holds a source path.

## Background

This is the largest client-visible exposure found by the audit. The bencoded `failure reason` of an
HTTP tracker response embeds the error text, and a BitTorrent client shows it to its user.

Variants that print `{location}` (11, `options.md` fact 1):

| File | Variants |
| --- | --- |
| `http-protocol/src/v1/query.rs:103` | `ParseQueryError::InvalidParam` |
| `http-protocol/src/v1/requests/announce.rs:182`, `:184`, `:190`, `:197` | `ParseAnnounceQueryError::{MissingParams, MissingParam, InvalidParam, NumberOfBytesOverflow}` |
| `http-protocol/src/v1/requests/scrape.rs:41`, `:43` | `ParseScrapeQueryError::{MissingParams, MissingParam}` |
| `http-protocol/src/v1/services/peer_ip_resolver.rs:117-120`, `:125` | `PeerIpResolutionError::{MissingRightMostXForwardedForIp, MissingClientIp}` (three-line attribute at `:117`) |
| `http-protocol/src/v1/auth.rs:13`, `:16` | `auth::Error::{InvalidKeyFormat, CannotExtractKeyParam}` |

Variants whose source is a `LocatedError`, which prints its own location (3):
`ParseAnnounceQueryError::InvalidInfoHashParam` and `InvalidPeerIdParam` (`announce.rs:543`, `:560`)
and `ParseScrapeQueryError::InvalidInfoHashParam` (`scrape.rs:84`), all built with
`Located(err).into()`. For the two info-hash variants the inner error,
`torrust_info_hash::ConversionError` (registry crate, 0.2.0), prints a second location of its own.

Construction sites (inventory rows): P3, P4, P5a, P5b, P6 and P7 in `http-protocol`; P8, P9 and
P10 in the `axum-http-server` extractors, which build `MissingParams`, `InvalidKeyFormat` and
`CannotExtractKeyParam`.

Text path to the client: `From<ParseQueryError>`, `From<ParseAnnounceQueryError>`,
`From<ParseScrapeQueryError>` (`requests/announce.rs:318-329`, `scrape.rs:59`) and, in
`responses/error.rs:54` and `:62`, `From<auth::Error>` and `From<PeerIpResolutionError>`.

Six of the 11 formats print no input (`MissingParams` twice, the two `PeerIpResolutionError`
variants and the two `auth::Error` variants; `options.md` fact 3 has the full list), so removing
`in {location}` needs new wording there.

`torrust-tracker-http-protocol` and `torrust-tracker-axum-http-server` are unpublished (0.1.0, first
publish planned in #2583), so the change adds no release step.

## Scope

### In Scope

- Remove `location` from the 11 variants and their formats; reword the messages that lose their
  only context so that each names what failed.
- Replace the three `LocatedError` sources with the plain source error (`#[source]`), and stop
  formatting a source the workspace does not control (`torrust_info_hash::ConversionError`) into the
  client text. The message keeps `param_name` and `param_value`.
- Remove every `Location::caller()` and `Located(..).into()` in `packages/http-protocol/src` and in
  the three `axum-http-server` extractors.
- Remove the `torrust-located-error` dependency from `packages/http-protocol/Cargo.toml`.
- Update the tests that build or match these variants (`http-protocol` `responses/error.rs` tests,
  `axum-http-server` tests) and the contract tests that assert failure reasons.

### Out of Scope

- `authentication::key::Error` and `WhitelistError` reasons, which are owned by `tracker-core` (SI-4).
- The `location` parameter of `assert_bencoded_error` in `axum-http-server/tests/server/asserts.rs`:
  it is a test helper that reports the test's own line and is allowed by the ADR.
- A fix inside `torrust-info-hash` (its repository; listed in the EPIC's external follow-ups).
- Changing which requests are accepted or rejected.

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
- real-artifact reproduction: run the tracker with the default development configuration and send a malformed announce (for example `numwant=x`, a missing `info_hash`, a 19-byte `peer_id`, a short `info_hash`) and a scrape without `info_hash`; record each bencoded response;
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
- **Real boundary**: one test per failure family through the HTTP test server in `packages/axum-http-server/tests`, asserting that the `failure reason` holds no source path. Include a short `info_hash` and a short `peer_id`, the cases that went through `LocatedError`.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Reproduce and record | Responses of the malformed requests with a library path, in `manual-verification-evidence.md`. |
| T2 | TODO | Red tests | Unit tests at the `FromStr`/`TryFrom<Query>` seams and HTTP-server tests for the failure reasons; red output recorded. |
| T3 | TODO | Remove `location` from the 11 variants | Fields, formats and captures removed in `http-protocol` and the `axum-http-server` extractors; messages reworded where needed. |
| T4 | TODO | Replace the three `LocatedError` sources | Plain `#[source]` errors; client text does not print the inner source; `torrust-located-error` removed from `http-protocol/Cargo.toml`. |
| T5 | TODO | Green and recheck | Green output, then the T1 requests repeated against the running tracker. |
| T6 | TODO | Review the reworded messages | A table of old and new `failure reason` texts in the evidence file, for the maintainer to approve the wording. |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The red regression tests | Commit after focused validation and required review. |
| T3 | The `location` removal with its tests green | Commit after focused validation and required review. |
| T4 | The `LocatedError` replacement and dependency removal | Commit after focused validation and required review. |
| T1, T5, T6 | Evidence | Commit after focused validation and required review. |

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

- [x] Folder-style spec drafted in `docs/issues/drafts/error-locations-si-3-http-tracker-request-errors/ISSUE.md`
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

- [ ] AC1: No error type in `packages/http-protocol` has a `location` field or a `LocatedError` source.
- [ ] AC2: `packages/http-protocol/src` and the `axum-http-server` extractors have no `Location::caller()` and no `Located(..).into()` outside test code.
- [ ] AC3: No `failure reason` produced by request parsing, key extraction or peer-IP resolution holds a source path, proven by HTTP-server tests including the short `info_hash` and short `peer_id` cases.
- [ ] AC4: Each reworded message still names what failed and the offending input where there is one.
- [ ] AC5: `packages/http-protocol/Cargo.toml` no longer depends on `torrust-located-error`.
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
| M1 | Malformed announce | Start the tracker; `curl` an announce with `numwant=x`, then one without `info_hash`, then one with a 19-byte `peer_id` | Each bencoded `failure reason` names the parameter and holds no `.rs:` path | TODO | `manual-verification-evidence.md` section V1 |
| M2 | Short info-hash | `curl` an announce and a scrape with a 10-byte `info_hash` | The reason names `info_hash` and holds no path, including no `.cargo/registry` path | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Key and proxy errors | On a private tracker, announce with a malformed key in the URL; on a reverse-proxy tracker, announce without `X-Forwarded-For` | The reasons hold no path | TODO | `manual-verification-evidence.md` section V3 |

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

### Disposable Verification Scripts

None planned. If one is needed, record here why it is better than a maintained Rust test, its
issue-local path, what it verifies and who removes it, before creating it.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **A client parses the old text.** The `failure reason` is free text in the protocol and the old
  text changed with every refactor. Mitigation: list old and new texts in the evidence (T6).
- **Messages become vague.** Mitigation: AC4 and the wording review.
- **Scope overlap with SI-4.** `http-core` maps `tracker-core` errors into failure reasons.
  Mitigation: this sub-issue owns only errors defined in `http-protocol`.

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
