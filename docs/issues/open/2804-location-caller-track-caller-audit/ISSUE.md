---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: null
github-issue: 2804
spec-path: docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md
branch: "2804-location-caller-track-caller-audit-spec"
related-pr: null
last-updated-utc: "2026-11-10 12:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - .github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md
---

<!-- skill-link: create-issue -->

# Issue #2804 - Audit and fix errors that record a wrong `Location::caller()`

## Goal

Find every error in the workspace that records `Location::caller()` in a function that is not
`#[track_caller]` (so the location is a fixed library line, not the caller's), decide which of
those locations reach production output, and fix them with one consistent, tested approach.

## Background

Reported while adding tests to `packages/primitives` (EPIC #1347, finding F3). Confirmed with
throwaway tests on 2026-11-10:

- `impl FromStr for PeerRole` builds `ParsePeerRoleError::InvalidPeerRole { location:
Location::caller(), .. }` inside `from_str`, which is not `#[track_caller]`.
  `"x".parse::<PeerRole>()` therefore reports `packages/primitives/src/peer.rs:73:27` for every
  caller.
- Adding `#[track_caller]` to `from_str` is only a partial fix. A direct `PeerRole::from_str("x")`
  then reports the caller's line, but `str::parse` is not `#[track_caller]`, so
  `"x".parse::<PeerRole>()` reports `core/src/str/mod.rs:2783:9` inside the standard library.

Without `#[track_caller]`, `Location::caller()` is just "the line where it is written" (the same
information as `file!()`/`line!()`). It is not wrong as a _detection site_, but it is wrong when the
error is meant to say _who called_, and it is noise when the text reaches users.

### Preliminary findings from the spec-drafting session (to be verified by T1)

A first pass over `torrust/develop` (2026-11-10) found 68 lines matching
`Location::caller()|track_caller` in 30 files (the count includes 6 in
`packages/axum-http-server/tests/server/asserts.rs`; roughly half of the remaining production-looking
sites are in `#[cfg(test)]` code and must be re-checked). `#[track_caller]` appears only 3 times:
`packages/configuration/src/lib.rs:348` and `packages/tracker-core/src/databases/error.rs:112` and
`:139`. All three are `From` impls that call `Located(..).into()`, which is the correct pattern
(`Into::into` in `torrust-located-error` is `#[track_caller]`).

Candidate wrong sites in non-test code (none is `#[track_caller]`):

| Area                          | Examples                                                                                                                                                                                  | Why the location is not the caller's                                                                             |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| trait impls                   | `PeerRole::from_str`, `NameValuePair::from_str`, announce `FromStr` impls (`primitives/src/peer.rs`, `http-protocol/src/v1/query.rs`, `requests/announce.rs`)                             | Reached through `str::parse`, which is not `#[track_caller]`; trait dispatch hides the real caller               |
| trait impls                   | `peer::Id::try_from(Vec<u8>)`                                                                                                                                                             | `TryFrom::try_from` called via `.try_into()` or generics                                                         |
| private helpers               | `extract_info_hash`, `extract_peer_id`, `extract_port`, `extract_numwant`, `extract_number_of_bytes_from_param` (`http-protocol/.../announce.rs`), scrape equivalent                      | Location is the helper line, never the request                                                                   |
| `const fn` and small helpers  | `ClientIpSources::try_client_ip_from_*` (`peer_ip_resolver.rs`)                                                                                                                           | Fixed line                                                                                                       |
| async fns                     | database drivers (`sqlite`/`mysql`/`postgres` `whitelist_store.rs`, `auth_key_store.rs`: `InsertFailed`, `DeleteFailed`), `authorize`, `verify_auth_key`, `make_rust_tls`, `authenticate` | `#[track_caller]` is not usable on async fns in stable Rust (to verify in T1); the location is inside the future |
| HTTP extractors               | `parse_key`, `custom_error`, `extract_announce_from`, `extract_scrape_from` (`axum-http-server`)                                                                                          | Fixed line in the extractor                                                                                      |
| sync fns that return an error | `verify_key_expiration` (`tracker-core/.../key/mod.rs`)                                                                                                                                   | Fixed line                                                                                                       |

### Which locations reach production output (preliminary)

`Display` of these errors includes `{location}` (for example
`#[error("invalid param {raw_param} in {location}")]`). The text then reaches:

- **HTTP tracker responses**: `responses::error::Error::from(ParseAnnounceQueryError|ParseScrapeQueryError|
auth::Error|PeerIpResolutionError)` builds `failure_reason: format!("Bad request. Cannot parse query
params: {err}")` and similar (`http-protocol/src/v1/requests/{announce,scrape}.rs`,
  `responses/error.rs`). The bencoded response sent to the BitTorrent client contains a library
  source path and line.
- **UDP tracker responses**: `ErrorResponse { message: error.to_string() }`
  (`udp-server/src/handlers/error.rs`), for `AuthRequired`, `InternalServer` and wrapped
  tracker-core errors (whitelist, key expired).
- **Logs**: `tracing` output of the same errors; `LocatedError::into()` also logs at `debug`.
- **Library consumers**: `torrust-tracker-primitives`, `-http-protocol`, `-tracker-core` and others are
  published crates, so the location can be a path inside a `~/.cargo/registry` checkout or a CI
  build path.

Not yet checked: REST API (`axum-rest-api-server`) responses, CLI output, metrics labels, and event
payloads (`udp-server/src/event.rs`).

A location that identifies a library line is not user-actionable, may leak build paths, and
contradicts the `handle-errors-in-code` principles (context and actionability).

### Located error package

`torrust-located-error` is a standalone crate (extracted in #1894) still in the dependency tree.
`Located<E>` into `LocatedError` captures the location in `Into::into`, which is `#[track_caller]`,
so it reports the line of the `.into()` call. This is correct when `.into()` is written at a
meaningful call site and wrong when `.into()` is written inside a non-`#[track_caller]` helper, a
closure, an async fn or a trait method: the location is then that helper's line. `LocatedError`'s
`Display` prints `{source}, {location}`, so the same exposure applies. T1 must document these
interactions with a small experiment, not from memory.

## Scope

### In Scope

- Inventory every `Location::caller()` and `#[track_caller]` in all workspace packages and in
  standalone crates that are in the dependency tree (at least `torrust-located-error`), classified
  as: correct, wrong (fixed library line), or intentionally library-internal.
- Trace which wrong locations reach production output (HTTP, UDP, REST API, CLI, logs, metrics,
  events) versus developer-only debugging.
- Check repository guidance (`handle-errors-in-code` skill, ADRs, lint config) and whether a clippy
  lint, a custom test or a `grep`-based check can enforce the rule.
- Compare fix options, pick one, and record it (ADR if it changes the error-design convention).
- Fix the confirmed wrong sites per package, each with a regression test.
- Update the `handle-errors-in-code` skill with the final rule.

### Out of Scope

- Changing the `torrust-located-error` crate itself (separate repository). Propose upstream changes
  in an issue there if needed.
- Adding new error variants or changing error messages unrelated to `location`.
- The `docs/issues/drafts/1347-add-tests-primitives/` work of EPIC #1347 (it only records the finding).

## Issue Type Decision

This is an **audit first, then refactor, with bug-fix rigor for the user-visible part**:

- It is not a single defect with one failing behavior, so a one-PR bug fix would hide the real
  question: which locations are meaningful at all.
- The inventory and the decision (T1-T4) are research and produce a recommendation, not code.
- Slices that change output users see (HTTP/UDP responses) are substantively bug fixes: misleading
  output. They follow `.github/skills/dev/debugging/fix-bug/SKILL.md` per slice (red test, fix,
  green test, like-for-like recheck with a real request). The mechanical slices that only change
  an internal `Debug`-visible field are refactors with the same regression tests.

## Architectural Decisions

- Related ADRs: `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
  (`#[non_exhaustive]` policy for public error enums, relevant to removing or reshaping fields).
- ADRs to create: probably one, "How errors record source locations", once the option is chosen
  in T4. It would replace the "consider located-error" bullet in the skill with a rule.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Applies to each fix slice that changes client-visible output (see Issue Type Decision). Per slice,
following `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- analysis: which call path builds the error and who sees the text;
- real-artifact reproduction: run the tracker and send a request that triggers the error (for
  example an announce with a bad `numwant`, or a UDP request that triggers `InternalServer`); record
  the response containing the library path in `manual-verification-evidence.md`;
- regression-test boundary: see below;
- red test, production fix, green test, repeat the same request after the last production change.

For T1 to T5 (research, no code) this section is `Not applicable`.

## Regression Test Strategy

- **Smallest boundary**: a unit test at the error's constructor or `FromStr`/`TryFrom` seam that
  asserts the rendered `Display` (and, where a location is kept, `Debug`) does not contain the
  library path (for example `src/`) and does contain the useful context (the offending input).
  This is the causal seam; it needs no server.
- **Trait-dispatch case**: the test must call through `str::parse::<T>()` (not `T::from_str`)
  because that is the call shape where `#[track_caller]` fails.
- **Real boundary for output**: one collaboration test per protocol (HTTP, UDP) asserting the
  response body of a malformed request has no library path. Use the existing test servers in
  `packages/axum-http-server/tests` and `packages/udp-server`.
- **Enforcement** (if T3 finds one feasible): a lint or a repository test that fails when
  `Location::caller()` appears in a function that is neither `#[track_caller]` nor in an allow-list.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                          | Notes / Expected Output                                                                                                                                                                                                                                                                                        |
| --- | ------ | ------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Inventory and classify every site                             | Issue-local `inventory.md` with one row per `Location::caller()`/`#[track_caller]`/`Located(..).into()`: package, file:line, enclosing fn (async, closure, trait impl), test or production, class (correct/wrong/internal). Includes the small experiments on `parse`, async fn, closures and `Located::into`. |
| T2  | TODO   | Trace production exposure                                     | For each wrong site, the path to output (HTTP failure reason, UDP error message, REST API, CLI, logs, metrics, events), with the exact line that formats it. Mark "user-visible" or "developer-only".                                                                                                          |
| T3  | TODO   | Check guidance and enforceability                             | What the skill, ADRs, `Cargo.toml` lints and clippy config say. Is there a clippy lint (candidates: none known for this; verify) or a test that can enforce the rule? Record the answer even if "no".                                                                                                          |
| T4  | TODO   | Analyse fix options, recommend one                            | See options below; semver impact per published crate; recommended option with reasoning. Maintainer approves before any fix.                                                                                                                                                                                   |
| T5  | TODO   | Write the ADR and the slicing plan                            | ADR if the convention changes. Create sub-issues or fix PRs per package from the slicing below.                                                                                                                                                                                                                |
| T6  | TODO   | Fix slice: `primitives`                                       | `PeerRole`, `peer::Id`. Regression tests through `parse`/`try_into`. Reporting finding F3 closes here.                                                                                                                                                                                                         |
| T7  | TODO   | Fix slice: `http-protocol`, `http-core`, `axum-http-server`   | Query/announce/scrape parse errors, auth, peer IP. Client-visible bencoded responses. Real-artifact reproduction.                                                                                                                                                                                              |
| T8  | TODO   | Fix slice: `tracker-core`, database drivers                   | `InsertFailed`/`DeleteFailed`, key expiry, whitelist, `authorize`.                                                                                                                                                                                                                                             |
| T9  | TODO   | Fix slice: `udp-server`, `axum-server`, `configuration`, rest | UDP error messages, TLS, others found by T1.                                                                                                                                                                                                                                                                   |
| T10 | TODO   | Enforcement and docs                                          | Lint or test from T3, update the `handle-errors-in-code` skill, close the issue.                                                                                                                                                                                                                               |

### Fix options to compare in T4 (research plan)

1. **`#[track_caller]` along the whole call chain.** Cheap and keeps the field. Fails through trait
   methods called via `str::parse`/`.try_into()`, through async fns and through closures. Only
   works where every caller is under our control and non-async. Good for `From` impls that call
   `Located(..).into()` (already done in 3 places).
2. **Capture the location at the call site** with a constructor that is `#[track_caller]` or a
   macro. Works for sync helper chains (`extract_*`), not for trait methods reached through std.
3. **Replace the location with context** (the offending input, the parameter name, the operation).
   Always correct, user-actionable, matches the skill's "what/where/when/why". Loses the developer
   hint (which line), which is already available from the message and `tracing` spans.
4. **Remove the field.** Simplest; a breaking change for public `#[non_exhaustive]` enums (the
   variants gain/lose fields; `#[non_exhaustive]` protects against new variants, not against changed
   fields of existing ones), so it needs a major bump of every affected published crate.

Initial recommendation (to be confirmed by T4): option 3 for errors that reach clients (HTTP/UDP),
option 1/2 where the chain is purely synchronous and the location only appears in `Debug`/logs,
and never `Location::caller()` in trait impls or async fns. Keeping `Display` free of source paths
is the main rule.

## Commit Points

| Task  | Coherent change set                                                                  | Commit policy                                        |
| ----- | ------------------------------------------------------------------------------------ | ---------------------------------------------------- |
| T1-T3 | `inventory.md` and findings notes                                                    | Commit after focused validation and required review. |
| T4-T5 | Options analysis, ADR, slicing                                                       | Commit after focused validation and required review. |
| T6-T9 | One package slice per PR: failing test, then fix (separate commits when independent) | Commit after focused validation and required review. |
| T10   | Enforcement and skill update                                                         | Commit after focused validation and required review. |

Use a Conventional Commit message with the narrow affected scope, and sign every commit with GPG.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Independent reviewer reports recorded in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-10 12:00 UTC - Claude Code (Sonnet 5.5) - Drafted spec from the F3 finding of the primitives test session and a first-pass inventory on `torrust/develop`.

## Acceptance Criteria

- [ ] AC1: `inventory.md` classifies every `Location::caller()` and `#[track_caller]` in the workspace and the dependency-tree standalone crates.
- [ ] AC2: Every wrong site is marked user-visible or developer-only with the output path that proves it.
- [ ] AC3: The chosen approach is recorded (ADR when the convention changes) and the maintainer approved it before fixes.
- [ ] AC4: No client-visible HTTP or UDP response and no `Display` of a public error contains a library source path, proven by regression tests that go through `str::parse`/`try_into` where relevant.
- [ ] AC5: Remaining `Location::caller()` uses are either in a `#[track_caller]` chain that reaches a meaningful call site or on an allow-list with a reason.
- [ ] AC6: Semver impact is documented per published crate and version bumps are planned.
- [ ] AC7: The `handle-errors-in-code` skill states the rule, and an enforcement check exists or its infeasibility is recorded.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test --workspace` for the packages touched by each slice
- Pre-push checks (when applicable)

### Manual Verification Scenarios

| ID  | Scenario                                  | Human-oriented command/steps                                                                                         | Expected Result                                                           | Status | Evidence                                     |
| --- | ----------------------------------------- | -------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------- | ------ | -------------------------------------------- |
| M1  | HTTP announce with an invalid `numwant`   | Run the tracker, `curl` an announce with `numwant=abc`, capture the bencoded failure reason before and after the fix | After: reason names the parameter and value, contains no `packages/` path | TODO   | `manual-verification-evidence.md` section V1 |
| M2  | UDP request that yields an error response | Run the tracker, send a malformed or unauthorized UDP request with the tracker client, capture the error message     | After: message has no library path                                        | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | `"x".parse::<PeerRole>()`                 | Run the regression test through `str::parse` against the old and the fixed code                                      | Old: `peer.rs` path or std path. After: no path, input echoed             | TODO   | `manual-verification-evidence.md` section V3 |

Notes: use the nightly/stable toolchain label for every recorded command. Create
`manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md` when
executing the scenarios.

### Disposable Verification Scripts

None planned. T1's experiments on `parse`, async fns and closures are small Rust tests kept in the
issue folder only if they explain a decision.

### Acceptance Verification

| AC ID | Status | Evidence           |
| ----- | ------ | ------------------ |
| AC1   | TODO   | {inventory link}   |
| AC2   | TODO   | {inventory link}   |
| AC3   | TODO   | {ADR / PR link}    |
| AC4   | TODO   | {test/log/PR link} |
| AC5   | TODO   | {inventory link}   |
| AC6   | TODO   | {ADR / PR link}    |
| AC7   | TODO   | {PR link}          |

## Risks and Trade-offs

- **Breaking changes in published crates.** Removing or reshaping `location` fields changes public
  enums, including `#[non_exhaustive]` ones. Mitigation: do the changes together before the next
  major release of the affected crates; list them in T4.
- **Losing the developer hint.** A location helps maintainers. Mitigation: keep it in `Debug` or
  `tracing` fields only where the call chain makes it correct, and put the input in `Display`.
- **Scope creep across ~30 files.** Mitigation: inventory first, fix per package, one PR each.
- **Test churn.** Many tests build errors with `Location::caller()` (test helpers). Mitigation:
  change test helpers in the same slice as the production type.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Reporting session: EPIC #1347 (primitives tests), finding F3.
- Related issues: #2804, #1894 (`torrust-located-error` extraction).
- Related ADRs: `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
- Skill: `.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md`
- Rust reference: `#[track_caller]` (<https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute>)
