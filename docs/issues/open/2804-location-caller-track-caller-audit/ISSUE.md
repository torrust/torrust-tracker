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
last-updated-utc: "2026-11-11 17:03"
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

Reported while adding tests to `packages/primitives` under EPIC #1347
(`docs/issues/open/1347-overhaul-packages-testing/EPIC.md`); the reporting session recorded it as its
finding F3. That finding is not in the repository yet, so this spec quotes it: `"x".parse::<PeerRole>()`
reports a fixed line in `peer.rs`, and `#[track_caller]` on `from_str` does not fix calls through
`str::parse`. Reproduced on 2026-11-10 (see
[`manual-verification-evidence.md`](manual-verification-evidence.md), section D1; outcome: Reproduced):

- `impl FromStr for PeerRole` builds `ParsePeerRoleError::InvalidPeerRole` with
  `location: Location::caller()` inside `from_str`, which is not `#[track_caller]`.
  `"x".parse::<PeerRole>()` therefore reports `packages/primitives/src/peer.rs:73:27` for every
  caller (stable `rustc 1.99.1`).
- Adding `#[track_caller]` to `from_str` is only a partial fix. A direct `PeerRole::from_str("x")`
  then reports the caller's line, but `str::parse` is not `#[track_caller]`, so
  `"x".parse::<PeerRole>()` reports `core/src/str/mod.rs:2783:9` inside the standard library (stable `rustc 1.99.1`; the std path and line depend on
  the toolchain, and the workspace sets only `rust-version = "1.88"`).

Without `#[track_caller]`, `Location::caller()` is just "the line where it is written" (the same
information as `file!()`/`line!()`). It is not wrong as a _detection site_, but it is wrong when the
error is meant to say _who called_, and it is noise when the text reaches users.

### Preliminary findings from the spec-drafting session (to be verified by T1)

A first pass over `torrust/develop` (2026-11-10) found 68 lines matching
`Location::caller()|track_caller` in 30 files (the count includes 6 in
`packages/axum-http-server/tests/server/asserts.rs`; of the other 62 lines, 21 are in `#[cfg(test)]` modules, one of them a comment, and 41 are
production: 38 `Location::caller()` lines plus the 3 attributes; T1 must confirm the split). `#[track_caller]` appears only 3 times:
`packages/configuration/src/lib.rs:348` and `packages/tracker-core/src/databases/error.rs:112` and
`:139`. All three are `From` impls that call `(Arc::new(err) as DynError).into()`, the crate's `DynError`
conversion. In `torrust-located-error` 3.0.0 both the `Located` and the `DynError` `Into` impls are
`#[track_caller]`, so this is the correct pattern.

Candidate wrong sites in non-test code (none is `#[track_caller]`):

| Area                          | Examples                                                                                                                                                                                  | Why the location is not the caller's                                                                                                                                                                                 |
| ----------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| trait impls                   | `PeerRole::from_str`, `NameValuePair::from_str`, announce `FromStr` impls (`primitives/src/peer.rs`, `http-protocol/src/v1/query.rs`, `requests/announce.rs`)                             | Reached through `str::parse`, which is not `#[track_caller]`; trait dispatch hides the real caller                                                                                                                   |
| trait impls                   | `peer::Id::try_from(Vec<u8>)`                                                                                                                                                             | `TryFrom::try_from` called via `.try_into()` or generics                                                                                                                                                             |
| private helpers               | `extract_info_hash`, `extract_peer_id`, `extract_port`, `extract_numwant`, `extract_number_of_bytes_from_param` (`http-protocol/.../announce.rs`), scrape equivalent                      | Location is the helper line, never the request                                                                                                                                                                       |
| `const fn` and small helpers  | `ClientIpSources::try_client_ip_from_*` (`peer_ip_resolver.rs`)                                                                                                                           | Fixed line                                                                                                                                                                                                           |
| async fns                     | database drivers (`sqlite`/`mysql`/`postgres` `whitelist_store.rs`, `auth_key_store.rs`: `InsertFailed`, `DeleteFailed`), `authorize`, `verify_auth_key`, `make_rust_tls`, `authenticate` | Hypothesis for T1 to confirm with a small experiment: `#[track_caller]` may not yield a meaningful caller for async call paths, because the body runs inside a polled future; the location is then inside the future |
| HTTP extractors               | `parse_key`, `custom_error`, `extract_announce_from`, `extract_scrape_from` (`axum-http-server`)                                                                                          | Fixed line in the extractor                                                                                                                                                                                          |
| sync fns that return an error | `verify_key_expiration` (`tracker-core/.../key/mod.rs`)                                                                                                                                   | Fixed line                                                                                                                                                                                                           |

### Which locations reach production output (preliminary)

`Display` of these errors includes `{location}` (for example
`#[error("invalid param {raw_param} in {location}")]`). The text then reaches:

- **HTTP tracker responses**: the `failure_reason` of `responses::error::Error` embeds the error text.
  The impls are `From<ParseQueryError>` ("Bad request. Cannot parse query params: {err}",
  `requests/announce.rs:318-323`), `From<ParseAnnounceQueryError>` ("... for announce request: {err}",
  `announce.rs:329`), `From<ParseScrapeQueryError>` ("... for scrape request: {err}", `scrape.rs:59`),
  and in `responses/error.rs` `From<auth::Error>` ("Tracker authentication error: {err}") and
  `From<PeerIpResolutionError>` ("Error resolving peer IP: {err}"). The bencoded response sent to the
  BitTorrent client contains a library source path and line.
- **UDP tracker responses**: `ErrorResponse { message: error.to_string() }`
  (`udp-server/src/handlers/error.rs:57-59`). T2 must confirm these two paths and look for others:
  - A whitelist error. An announce for a non-whitelisted info-hash on a listed tracker fails in
    `AnnounceService::handle_announce` (`udp-core/src/services/announce.rs:109`) with the tracker-core
    `WhitelistError::TorrentNotWhitelisted` built in `whitelist/authorization.rs:61`, wrapped by
    `UdpAnnounceError::TrackerCoreWhitelistError`. A UDP scrape does not take this path: the UDP scrape
    service never calls `authorize` and the tracker-core scrape handler maps a failed `authorize` to
    zeroed statistics, so `UdpScrapeError::TrackerCoreWhitelistError` is not built by production code.
  - A database error. With `core.tracker_policy.persistent_torrent_completed_stat` enabled, an
    announce or scrape for a torrent absent from memory loads its stats from the database. A driver
    failure becomes a `databases::error::Error` through the `#[track_caller]`
    `From<(SqlxError, Driver)>`, whose `LocatedError` source prints `{source}, {location}`, and
    travels as `AnnounceError::Database` or `ScrapeError::Database` inside
    `UdpAnnounceError::TrackerCoreAnnounceError` or `UdpScrapeError::TrackerCoreScrapeError`.
  - `udp-server` `Error::Internal` and `Error::AuthRequired` are built only in `#[cfg(test)]` code,
    `event::ErrorKind::InternalServer` has no location, a malformed request becomes `InvalidRequest`
    (no location) and the UDP path has no key authentication, so key expiry cannot occur there.
- **Logs**: `tracing` output of the same errors; the `Located` conversion also logs at `debug` (the `DynError` conversion does not).
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
- The primitives test work of EPIC #1347 (it only reported the finding).

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
  example an announce with a bad `numwant`, or a UDP announce for a non-whitelisted info-hash on a tracker configured as listed); record
  the response containing the library path in `manual-verification-evidence.md`;
- regression-test boundary: see below;
- red test, production fix, green test, repeat the same request after the last production change.

For T1 to T5 (research, no code) this section is `Not applicable`.

## Regression Test Strategy

- **Smallest boundary**: a unit test at the error's constructor or `FromStr`/`TryFrom` seam that
  asserts the rendered `Display` (and, where a location is kept, `Debug`) does not contain a source
  path and does contain the useful context (the offending input).
  This is the causal seam; it needs no server.
- **Trait-dispatch case**: the test must call through `str::parse::<T>()` (not `T::from_str`)
  because that is the call shape where `#[track_caller]` fails.
- **Portable path check**: a helper forbids these substrings: `.rs:` (any `file:line`), `packages/`,
  `packages\`, `src/`, `src\`, `.cargo/registry` and `rustlib/src`, so Windows paths, workspace paths,
  registry checkouts and CI checkout paths are all caught without depending on one layout.
- **Real boundary for output**: one collaboration test per protocol (HTTP, UDP) asserting the
  response body of a malformed request has no library path. Use the existing test servers in
  `packages/axum-http-server/tests` and `packages/udp-server`.
- **Enforcement** (if T3 finds one feasible): a lint or a repository test that fails when
  `Location::caller()` appears in a function that is neither `#[track_caller]` nor in an allow-list.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                      | Notes / Expected Output                                                                                                                                                                                                                                                                                                                                             |
| --- | ------ | ----------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Inventory and classify every site         | Issue-local `inventory.md` with one row per `Location::caller()`/`#[track_caller]`/`Located(..).into()`: package, file:line, enclosing fn (async, closure, trait impl), test or production, class (correct/wrong/internal). Includes the small experiments on `parse`, async fn, closures and `Located::into`.                                                      |
| T2  | DONE   | Trace production exposure                 | For each wrong site, the path to output (HTTP failure reason, UDP error message, REST API, CLI, logs, metrics, events), with the exact line that formats it. Mark "user-visible" or "developer-only".                                                                                                                                                               |
| T3  | DONE   | Check guidance and enforceability         | What the skill, ADRs, `Cargo.toml` lints and clippy config say. Is there a clippy lint (candidates: none known for this; verify) or a test that can enforce the rule? Record the answer even if "no".                                                                                                                                                               |
| T4  | DONE   | Analyse fix options, recommend one        | Recommendation in `options.md`: stop printing `{location}` in `Display` (option 3); option 1 fixes no production site (all 38 are reached through trait impls, closures, `TryFrom` or `async` code). Open for the maintainer: keep or remove the field, and the `LocatedError` decision. Awaiting approval before any fix.                                          |
| T5  | DONE   | Write the ADR and the slicing plan        | Slicing done as an EPIC draft with six sub-issue drafts and the specification for #384 (see below). The ADR moved to sub-issue SI-1 of that EPIC, because the maintainer proposed to remove the feature and that decision is still under review.                                                                                                                    |

### Tasks moved to the EPIC draft

After T4 the maintainer proposed to remove source locations from errors altogether, instead of the
options compared in `options.md`. The work is then larger than this issue and cuts across the
workspace, so the fix tasks that this plan listed as T6 to T9 are not done here. They are planned in
the EPIC draft `docs/issues/drafts/remove-source-locations-from-errors/EPIC.md`:

| Former task | Now in |
| --- | --- |
| T6 Red regression tests per slice, T7 Fix slices, T8 Green and like-for-like recheck per slice | Sub-issue drafts SI-2 (`primitives`), SI-3 (HTTP tracker request errors), SI-4 (`tracker-core`), SI-5 (configuration, TLS and UDP server errors), each with its own red test, fix and recheck |
| T9 Enforcement and docs | SI-1 (ADR and `handle-errors-in-code` skill) and SI-6 (drop `torrust-located-error`, repository check, upstream issue drafts) |
| Client-side verification (M1, M2) | `docs/issues/open/384-strip-location-from-errors-sent-to-clients/ISSUE.md` (SI-7) |

The EPIC and its sub-issues are drafts: no GitHub issue exists for them yet. This issue can be
closed when the maintainer creates the EPIC issue; until then it stays open as the record of the
audit.

### Fix options to compare in T4 (research plan)

1. **`#[track_caller]` along the whole call chain.** Cheap and keeps the field. Fails through trait
   methods called via `str::parse`/`.try_into()`, through async fns and through closures. Only
   works where every caller is under our control and non-async. Good for `From` impls that call
   `.into()` on a `Located` or `DynError` (already done in 3 places).
2. **Capture the location at the call site** with a constructor that is `#[track_caller]` or a
   macro. Works for sync helper chains (`extract_*`), not for trait methods reached through std.
3. **Replace the location with context** (the offending input, the parameter name, the operation).
   Always correct, user-actionable, matches the skill's "what/where/when/why". Loses the developer
   hint (which line), which is already available from the message and `tracing` spans.
4. **Remove the field.** Simplest; a breaking change for public `#[non_exhaustive]` enums (the
   variants gain/lose fields; `#[non_exhaustive]` protects against new variants, not against changed
   fields of existing ones), so it needs a breaking release of every affected published crate (a major bump for
   `torrust-tracker-primitives` 3.1.0 and `torrust-tracker-configuration` 3.0.0, a minor bump for the
   0.1.0 crates); AC6 settles the step per crate.

Initial recommendation (to be confirmed by T4): option 3 for errors that reach clients (HTTP/UDP),
option 1/2 where the chain is purely synchronous and the location only appears in `Debug`/logs,
and never `Location::caller()` in trait impls or async fns. Keeping `Display` free of source paths
is the main rule.

## Commit Points

| Task  | Coherent change set                      | Commit policy                                        |
| ----- | ---------------------------------------- | ---------------------------------------------------- |
| T1-T3 | `inventory.md` and findings notes        | Commit after focused validation and required review. |
| T4-T5 | Options analysis, ADR, slicing           | Commit after focused validation and required review. |
| T6    | Red regression tests of one slice        | Commit after focused validation and required review. |
| T7    | Production fix of that slice             | Commit after focused validation and required review. |
| T8    | Green and recheck evidence of that slice | Commit after focused validation and required review. |
| T9    | Enforcement and skill update             | Commit after focused validation and required review. |

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

- [x] Folder-style spec drafted in `docs/issues/open/2804-location-caller-track-caller-audit/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation (#2805)
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

- 2026-11-10 12:00 UTC - Claude Code (Sonnet 5.5) - Drafted spec from the F3 finding of the primitives test session and a first-pass inventory on `torrust/develop`.
- 2026-11-10 13:45 UTC - Claude Code (Sonnet 5.5) - Corrected the UDP exposure analysis after review F1.
- 2026-11-10 13:50 UTC - Claude Code (Sonnet 5.5) - Addressed review round 1 (F1-F9 and Copilot): recorded the reproduction, split the red/fix/green tasks, restored template passages, corrected facts and wording. Evidence: `manual-verification-evidence.md`.
- 2026-11-10 14:25 UTC - Claude Code (Sonnet 5.5) - Addressed review round 2 (F10-F16): UDP paths, timestamps, evidence numbering, audit record `docs/pr-reviews/pr-2805-review/PR-REVIEW.md`.
- 2026-11-10 17:49 UTC - Claude Code (Sonnet 5.5) - T1 to T3 done: `inventory.md` (classification, experiments, exposure trace, guidance and enforceability check).
- 2026-11-10 21:51 UTC - Claude Code (Sonnet 5.5) - Reworked the T2 trace, the closure and class columns, the standalone-crate scan and the experiment record after review of PR #2814 (`experiments.md`).
- 2026-11-11 10:44 UTC - Claude Code (Sonnet 5.5) - T4 done: option comparison, semver estimate per crate and slicing proposal in `options.md`; recommendation awaits maintainer approval.
- 2026-11-11 11:11 UTC - Claude Code (Sonnet 5.5) - Reworked T4 after review of PR #2821: count of 23 printed locations, option 1 reaches no production site, semver against the first-publish plan, slices S1 to S7 (`options.md`).
- 2026-11-11 11:16 UTC - Claude Code (Sonnet 5.5) - Added the Copilot findings of PR #2821 to T4: additive constructors are not breaking, the `configuration` `LocatedError` sources, the peer-id wrapping site, and direct tests for S5 and S6 (`options.md`).
- 2026-11-11 11:55 UTC - Claude Code (Sonnet 5.5) - Corrected the S3 and S5 slices of T4 after the second review of PR #2821: `MissingTlsConfig` needs no change, version steps per crate, the info-hash paths need source suppression or an upstream fix (`options.md`).
- 2026-11-11 17:03 UTC - Claude Code (Fable 5.2) - T5 done as a slicing plan: the maintainer proposed to remove the feature, so the fix tasks moved to an EPIC draft with six sub-issue drafts and the specification for #384. The ADR is sub-issue SI-1 of that EPIC. AC1, AC2 and AC6 recorded as done with their evidence.

## Acceptance Criteria

- [x] AC1: `inventory.md` classifies every `Location::caller()` and `#[track_caller]` in the workspace and the dependency-tree standalone crates.
- [x] AC2: Every wrong site is marked user-visible or developer-only with the output path that proves it.
- [ ] AC3 (moved to the EPIC draft, SI-1): The chosen approach is recorded (ADR when the convention changes) and the maintainer approved it before fixes.
- [ ] AC4 (moved to the EPIC draft, SI-2 to SI-5 and #384): No client-visible HTTP or UDP response and no `Display` of a public error contains a library source path, proven by regression tests that go through `str::parse`/`try_into` where relevant.
- [ ] AC5 (moved to the EPIC draft, SI-6): Remaining `Location::caller()` uses are either in a `#[track_caller]` chain that reaches a meaningful call site or on an allow-list with a reason.
- [x] AC6: Semver impact is documented per published crate and the breaking release per crate is planned.
- [ ] AC7 (moved to the EPIC draft, SI-1 and SI-6): The `handle-errors-in-code` skill states the rule, and an enforcement check exists or its infeasibility is recorded.
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
| M2  | UDP request that yields an error response | Run the tracker, send a UDP announce for a non-whitelisted info-hash to a listed tracker, capture the error message  | After: message has no library path                                        | TODO   | `manual-verification-evidence.md` section V2 |
| M3  | `"x".parse::<PeerRole>()`                 | Run the regression test through `str::parse` against the old and the fixed code                                      | Old: `peer.rs` path or std path. After: no path, input echoed             | TODO   | `manual-verification-evidence.md` section V3 |

Notes: use the nightly/stable toolchain label for every recorded command. Add
the scenarios' sections to the existing `manual-verification-evidence.md` (created from
`docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`; it already holds the drafting-time section D1).

### Disposable Verification Scripts

None planned. T1's experiments on `parse`, async fns and closures are small Rust tests kept in the
issue folder only if they explain a decision.

### Acceptance Verification

| AC ID | Status | Evidence                              |
| ----- | ------ | ------------------------------------- |
| AC1   | DONE   | `inventory.md` (PR #2814)             |
| AC2   | DONE   | `inventory.md`, T2 table (PR #2814)   |
| AC3   | TODO   | Moved: ADR of SI-1 in the EPIC draft  |
| AC4   | TODO   | Moved: SI-2 to SI-5 and #384          |
| AC5   | TODO   | Moved: SI-6                           |
| AC6   | DONE   | `options.md`, semver table (PR #2821) |
| AC7   | TODO   | Moved: SI-1 and SI-6                  |

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

- Reporting work: EPIC #1347 (`docs/issues/open/1347-overhaul-packages-testing/EPIC.md`), primitives test session, finding F3.
- Related issues: #2804, #1894 (`torrust-located-error` extraction).
- Related ADRs: `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`
- Skill: `.github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md`
- Rust reference: `#[track_caller]` (<https://doc.rust-lang.org/reference/attributes/codegen.html#the-track_caller-attribute>)
