---
schema-version: 1
doc-type: issue
issue-type: bug
status: done
priority: p1
epic: 2912
github-issue: 384
spec-path: docs/issues/closed/384-2912-si-7-strip-location-from-errors-sent-to-clients/ISSUE.md
branch: "384-strip-location-from-errors-sent-to-clients"
related-pr: 3028
last-updated-utc: "2026-11-24 17:33"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md
    - docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md
---

<!-- skill-link: create-issue -->

# Issue #384 - Make Option to Strip Location from Errors sent to Clients

Parent EPIC: [Remove source locations from errors](../2912-remove-source-locations-from-errors/EPIC.md) (#2912). Sub-issue SI-7 (the last one) of the EPIC.

## Goal

Make sure that no error the tracker sends to a client holds a source location, and close the report
that asked for it. The report asked for an option to strip the location. This specification proposes
to meet it by removing source locations from errors altogether, through the parent EPIC, and to use
this issue as the EPIC's final acceptance on a running tracker.

## Background

### The report

Issue #384 was opened on 2023-08-26 by an external user and is still open, with the
labels `Security`, `Enhancement / Feature Request` and `- Admin -`. Its text: full error messages,
including the code location, are sent to the client; whenever a `failure reason` is sent, a
`Location` value is attached "which might reveal some of the internal functionality". It asks for an
option to strip non-essential information before the error is sent.

The answer at the time (`da2ce7`, 2023-08-26) was that the information is not very sensitive,
because the project is open source, and that it helps administrators and developers more than it
risks; the issue stayed open for a possible override. The reporter agreed and asked for a way to
turn it off.

### What changed

The audit of issue #2804 (`docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md`, `options.md`) measured the feature:

- None of the 38 production `Location::caller()` sites of the audit (37 since #2850) reports a caller. Each reports a fixed
  library line, and `#[track_caller]` cannot fix any of them.
- 23 `Display` formats (22 since #2850) print the location, and `LocatedError` prints its own. The text reaches HTTP
  tracker clients (`failure reason`), UDP tracker clients (error message), REST API clients, event
  payloads and logs.
- For a registry crate (`torrust-info-hash`) the printed location is a path inside the build
  machine's `~/.cargo/registry` checkout.

So the benefit the 2023 answer relied on is not there, and the maintainer listed three further
objections: an attacker can infer the running version from paths and line numbers, the user-facing
text is coupled to the implementation, and the messages are unstable for consumers.

### Proposed resolution (to be confirmed in the review of this specification)

Remove the feature instead of adding an option: with no location in any error there is nothing to
strip and no configuration to maintain. The EPIC's sub-issues SI-1 to SI-6 do the removal; this
issue verifies the result from the client side and closes the report.

`da2ce7` took part in the 2023 discussion. The maintainer understands that he now prefers removal as
well, and asks him to confirm that in the review of the pull request that adds this specification.

## Scope

### In Scope

- A baseline reproduction on the current `develop`, before the removal: the responses the report
  describes, recorded as evidence. It can be done before the other sub-issues land.
- After SI-2 to SI-6 are merged: the same requests against a tracker built from `develop`, on every
  client surface (HTTP tracker, UDP tracker, REST API), recorded as evidence.
- A search of the documentation for example responses that show a location, and their update.
- A comment on #384 for the reporter that links the ADR and the EPIC and states the outcome, then
  closing the issue.

### Out of Scope

- A configuration option to strip locations: superseded by the removal.
- The content of server-side logs. `tracing` events may name the file and line of the log call;
  that stays on the server.
- The removal itself (SI-2 to SI-6) and the rule (SI-1).
- Other information an error reply may carry (for example database error codes). If the
  verification finds a reply that is too detailed for a client, record it as a follow-up.

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
issue follows `.claude/skills/fix-bug/SKILL.md`:

- analysis: the call paths and audiences are in the audit inventory
  (`docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md`, T2 table); re-check them against the tree before the first change;
- real-artifact reproduction: run the tracker from the current `develop` and send the requests of the manual scenarios below; record each reply with its library path. This is the baseline;
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
- **Maintained tests**: the protocol-level regression tests are owned by SI-3 and SI-4. This issue adds none unless the verification finds a client-visible reply that no maintained test covers; in that case add the test in the owning package and record it here.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Baseline reproduction | Replies with a library path on `develop` at `305027e70`, before the removal, for HTTP, UDP and REST, in `manual-verification-evidence.md` (toolchain and commit named). |
| T2 | DONE | Wait for the removal | SI-2 to SI-6 merged; record their pull requests here. `BLOCKED` until then. Done: SI-2 #2955, SI-3 #2968, SI-4 #2991, SI-5 #3001, SI-6 #3018, all merged into `develop`. |
| T3 | DONE | Like-for-like recheck | The T1 requests repeated on a tracker built from `develop` at `c94bda536`; no reply holds a source path (`manual-verification-evidence.md`). |
| T4 | DONE | Coverage gap check | Each surface exercised in T3 is covered by a maintained test of SI-3 or SI-4; a missing one is added in its package and linked here. Result: every surface is covered, no test added (table in the progress log entry of 2026-11-24 13:32 UTC). |
| T5 | DONE | Documentation | `grep -rn '\.rs:' docs packages/*/README.md README.md` reviewed for example responses with a location; each one updated. Result: no example reply holds a location; nothing to update. |
| T6 | DONE | Answer and close #384 | Comment on #384 linking the ADR and the EPIC; the maintainer closes the issue or lets the last pull request close it. |

## Commit Points

Map implementation-plan tasks that change code, tests, configuration, or documentation to small,
coherent commit opportunities. A commit point completes one independently reviewable behavior,
refactor, or evidence increment; do not group unrelated changes merely to reduce commit count.

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | The baseline evidence | Commit after focused validation and required review. |
| T3-T4 | The recheck evidence and any added test | Commit after focused validation and required review. |
| T5 | Documentation updates | Commit after focused validation and required review. |

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

- [x] Folder-style spec written in `docs/issues/closed/384-2912-si-7-strip-location-from-errors-sent-to-clients/ISSUE.md`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#384 already existed)
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed (no production change; verification only)
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [x] Evidence-based implementation completion review recorded: issue-local retrospective created for material discoveries, or progress log states why none was needed
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Independent reviewer reports recorded (not applicable: no independent reviewer received this folder; the pull request reviews are in `docs/pr-reviews/pr-3028-review/PR-REVIEW.md`) in issue-local `agent-review-reports.md` when reviewers received this folder-style specification
- [x] Committer verified spec progress is up to date before commit
- [x] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

Append one line per meaningful update. When an AI agent did the work, name its model after the
agent, for example `GitHub Copilot (Claude Opus 5.5)`, so later reviews can relate outcomes to
the model that produced them.

- 2026-11-11 17:05 UTC - Claude Code (Fable 5.2) - Drafted from the audit of issue #2804 (`docs/issues/open/2804-location-caller-track-caller-audit/inventory.md`, `options.md`) as a sub-issue of the EPIC draft. Not yet approved; line numbers are those of `develop` at `4f0af379a` and must be re-checked before implementation.
- 2026-11-17 17:30 UTC - Claude Code (Claude Opus 5.5) - EPIC coordinator: the maintainer approved the specification on 2026-11-17; this issue keeps its title and is SI-7 of EPIC #2912 (comment posted on #384).
- 2026-11-24 13:32 UTC - Claude Code (Claude Sonnet 5.5) - SI-7: T1-T5 done on one run. Baseline `tracker-before` built from `305027e70`, recheck `tracker-after` from `c94bda536` (SI-1 to SI-6 merged), both run with the same 17-request script plus 7 extra error requests; the baseline replies held a `packages/...` path (H7 and R6 also a `~/.cargo/registry` path), the recheck replies hold none (grep of the forbidden substrings is empty). `cargo test --workspace --no-fail-fast source_path`: 45 passed. No leak found, no production change. Coverage (T4), surface to maintained test: HTTP announce and scrape parse errors, `axum-http-server/tests/server/v1/contract/failure_reasons_hold_no_source_path.rs` and `http-protocol` unit tests (H1, H5-H7, R6, R7); private-key errors (H2-H4, H8), the same contract file and `tracker-core` `authentication/service.rs` tests; UDP whitelist rejection (U1), `udp-server/tests/server/contract.rs` `should_reject_a_torrent_that_is_not_whitelisted_with_a_message_that_holds_no_source_path`; REST (K2, K3), `axum-rest-api-server/tests/server/v1/contract/context/auth_key.rs` (`should_fail_without_a_source_path_*`). Not tied to a source-path test: R1 to R5 never held a path (their text comes from axum extractors and the REST layer); recorded as a note, no leak. Docs: no example reply holds a location. `docs/packages.md` line 280 still lists the external `located-error` crate in its table of extracted crates; it is not a reply and is left to the EPIC close-out.
- 2026-11-24 14:37 UTC - Claude Code (Claude Sonnet 5.5) - Review round 1 (CodeRabbit, PR 3028): AC1 wording narrowed to what the evidence shows (a replies-that-never-held-a-path clause); the path check re-run with the full forbidden set of the Regression Test Strategy, including `packages\` and `src\`, still empty.
- 2026-11-24 17:33 UTC - Claude Code (Claude Sonnet 5.5) - PR 3028 merged. Outcome comment posted on #384 (links the ADR, the EPIC and the evidence; 13 leaking replies before, none now). Spec archived to `docs/issues/closed/` and SI-7 marked DONE in EPIC #2912; the archive pull request closes #384.

## Acceptance Criteria

- [x] AC1: On a tracker built from `develop` after SI-2 to SI-6, none of the replies of the manual scenarios (HTTP, UDP, REST) holds a source path, and every baseline reply that held one (13 of the recorded replies) holds none now. Replies that never held a path (for example R1 to R5, whose text comes from axum extractors and the REST layer) are recorded as no-leak notes, not as baseline matches.
- [x] AC2: Every client surface exercised by the scenarios is covered by a maintained test.
- [x] AC3: The documentation shows no example error reply with a source location.
- [x] AC4: #384 has a comment that states the outcome and links the ADR and the EPIC, and is closed.
- [x] `linter all` exits with code `0`
- [x] Relevant tests pass
- [x] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [x] Documentation is updated when behavior/workflow changes

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
| M1 | HTTP failure reason, as in the report | Start the tracker; `curl` an announce with a missing `info_hash`, with `numwant=x` and with a short `info_hash` | Baseline: the bencoded `failure reason` ends in a `.rs:` path. After the removal: no path | DONE | `manual-verification-evidence.md` section V1 |
| M2 | UDP error message | On a listed tracker, send a UDP announce for an info-hash that is not whitelisted | Baseline: the message holds a path. After: no path | DONE | `manual-verification-evidence.md` section V2 |
| M3 | REST API error | Add the same key or whitelist entry twice through the REST API | Baseline: the error text holds a path. After: no path | DONE | `manual-verification-evidence.md` section V3 |
| M4 | Private tracker key errors | Announce over HTTP with an expired key and with a malformed key | Baseline: a path. After: no path | DONE | `manual-verification-evidence.md` section V4 |

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
| AC1 | DONE | `manual-verification-evidence.md` V1-V4: 13 baseline replies held a path; after, `grep` for the forbidden substrings finds nothing |
| AC2 | DONE | Coverage table in the progress log; `evidence/tests-green.txt` (45 tests of the `source_path` filter pass) |
| AC3 | DONE | `grep -rn '\.rs:' docs packages/*/README.md README.md` outside `docs/issues` and `docs/pr-reviews` finds only discussion, schema and ADR prose, no reply |
| AC4 | DONE | Outcome comment on #384: <https://github.com/torrust/torrust-tracker/issues/384#issuecomment-7139357870>; the issue closes with the pull request that archives this spec |

## Risks and Trade-offs

- **The removal is not approved.** Then this specification returns to the original ask (an option)
  and must be rewritten. Mitigation: the review of this specification decides it before any code.
- **A surface is missed.** The audit read the code; it did not run every path. Mitigation: T3 runs
  the tracker, and T4 ties each surface to a maintained test.
- **The issue stays open while the EPIC runs.** Mitigation: T1 can be done at once, and the EPIC
  lists this issue last.

## Implementation Completion Review

After implementation, compare the result with this specification. Record
invalidated assumptions, material design changes, unexpected validation
findings, and reusable lessons.

- Retrospective: not needed; no material discovery. The recheck found no remaining leak and no spec assumption was invalidated.
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

- Parent EPIC: #2912, `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md`
- Audit: `docs/issues/closed/2804-location-caller-track-caller-audit/ISSUE.md`, `inventory.md`, `options.md`, `experiments.md` (issue #2804)
- Related issues: #2804 (audit), #2583 (first publish of the workspace crates)
- This issue on GitHub: <https://github.com/torrust/torrust-tracker/issues/384>
