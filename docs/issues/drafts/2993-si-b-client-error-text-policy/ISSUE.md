---
schema-version: 1
doc-type: issue
issue-type: enhancement
status: draft
priority: p2
epic: 2993
github-issue: null
spec-path: docs/issues/drafts/2993-si-b-client-error-text-policy/ISSUE.md
branch: "{issue-number}-2993-si-b-client-error-text-policy"
related-pr: null
last-updated-utc: "2026-11-22 13:50"
semantic-links:
  skill-links:
    - create-issue
    - create-adr
    - handle-errors-in-code
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/create-adr/SKILL.md
    - .claude/skills/handle-errors-in-code/SKILL.md
    - docs/issues/open/2993-secrets-and-client-error-text/EPIC.md
    - docs/adrs/20261118045443_errors_do_not_record_source_locations.md
    - docs/issues/open/144-api-v2-correct-response-codes/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - SI-B Define and apply the client error text policy

Parent EPIC: [Keep secrets out of errors, logs and client replies, and make client error text safe](../../open/2993-secrets-and-client-error-text/EPIC.md) (#2993). Sub-issue SI-B.

## Goal

An accepted ADR says what an error reply may tell a client of the HTTP tracker, the UDP tracker and the REST API, and the three protocols follow it: the text helps a legitimate user fix the problem, and carries no database driver text, no internal error nesting, no server timing value and no Rust type name.

## Background

Section B of the EPIC inventory lists what each protocol sends today, reproduced on a running tracker (commands and output in the EPIC's [`manual-verification-evidence.md`](../../open/2993-secrets-and-client-error-text/manual-verification-evidence.md)):

- HTTP tracker: `Tracker announce error: Database error: <sqlx text>`; nested prefixes that mirror the Rust error tree; query parse texts that echo the client's own values.
- UDP tracker: the full `Display` chain of the server error. A wrong connection id returns `cookie value is expired: <value>, expected > 1795353220.6572893` (the server clock and the decoded cookie); a short packet returns `error parsing request: SendableRequestParseError { message: "Couldn't parse action", ... }` (a Rust type name).
- REST API: every key, whitelist and reload failure, and a missing or wrong token, is a 500 with `Unhandled rejection: Err { reason: "..." }`; database failures put `e.to_string()` of the driver error in it (`UNIQUE constraint failed: keys.key`); a malformed JSON body returns serde's own text.
- A database server message can name the database user and the client address (`Access denied for user 'db_user'@'172.17.0.1'`). Today it reaches the operator only; it would reach a client wherever driver text is forwarded (EPIC item A11).
- The UDP error event payload (`packages/udp-server/src/event.rs:114-153`) holds the same strings.

Rule 4 of the errors ADR says client text is the workspace error's own `Display` and that a source error from a crate the workspace does not control is not formatted into client text. It does not say how much a workspace `Display` may reveal, and the database errors break it today, because their `Display` includes the `sqlx` source.

The maintainer's decisions (2026-11-22): the text for database failures is decided in this issue's ADR, and the principle is to help the legitimate user as much as possible without revealing a secret. The project is public, so the policy does not rely on hiding how the tracker works; it removes what a client cannot act on and what changes with the implementation.

## Scope

### In Scope

1. **An ADR** (root, `docs/adrs/`, because it is a cross-package protocol decision) that decides:
   - the classes of error a client can receive (the client's own bad input; an authorization decision such as not whitelisted or key not registered; a failure inside the tracker) and what each class may say;
   - the text for a failure inside the tracker, including database failures. Options to weigh: a fixed generic text; the name of the failed operation without driver text; either of them with a request id the client can quote, since the HTTP tracker and the REST API already return `x-request-id`;
   - whether a known cause keeps a specific message (a duplicate key is the client's mistake, not an internal failure);
   - whether the reply mirrors the Rust error nesting (`tracker announce error: Tracker core whitelist error: ...`) or is one sentence;
   - whether client text may echo the client's own input, and with what limits (length, escaping);
   - what the UDP connection-id error says without the server clock;
   - whether any reply text is a contract that clients may match, or all of it is free to change;
   - where the detail goes: a `tracing` event on the server at the site of the failure, with the request id where one exists. Today the REST API logs a 500 with no reason, so the operator cannot tell why a request failed;
   - what an event payload may carry (the UDP `ErrorKind` strings), in line with the events ADR (`docs/adrs/20260727000000_events_are_objective_facts.md`).
2. **The implementation of the ADR**, after the maintainer accepts it: the HTTP tracker (`packages/http-core/src/services/error_mapping.rs`, `packages/http-protocol`), the UDP tracker (`packages/udp-server/src/error.rs`, `packages/udp-server/src/handlers/error.rs`, `packages/udp-core/src/connection_cookie.rs`), the REST API (`packages/rest-api-runtime-adapter/src/v1/adapters/`, `packages/axum-rest-api-server/src/v1/responses.rs`, the JSON rejection) and the UDP error event. Each old and new text is listed in this specification for approval before the code changes, as the SI-3 and SI-4 specifications of EPIC #2912 do.
3. **The wrong label**: `From<(SqlxError, Driver)>` maps every `sqlx::Error::Database` to `InvalidQuery`, so an authentication failure reads `The MySQL query was malformed`. Fix the mapping or the wording as the ADR decides.
4. **The skill**: update `handle-errors-in-code` with the policy.

### Out of Scope

- Secrets in errors and logs (SI-A).
- Source locations (EPIC #2912).
- REST status codes and the response shape of API v1: v1 is additive-only, and correct codes and a structured error body are API v2 (EPIC #144). This issue changes v1 text only. The ADR should say how the v2 error model relates to this policy so the two do not diverge.
- The general error-event contract (draft `docs/issues/drafts/generalize-error-events/`). This issue changes only what the existing UDP error event carries, if the ADR decides it must change.
- Server-side log content: non-secret driver text in a server log is correct and stays. A driver message that holds a secret (EPIC item A12, the MySQL duplicate-key message) is removed from the error, the reply and the log by SI-A.
- Log levels for client errors (#1166).

## Architectural Decisions

- Related ADRs: `docs/adrs/20261118045443_errors_do_not_record_source_locations.md` (rule 4), `docs/adrs/20260727000000_events_are_objective_facts.md`, `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`.
- ADRs to create: "Client error text policy" (title to settle), in `docs/adrs/`. It may amend rule 4 of the errors ADR; say so in both.

## Design and Ownership Review

Not applicable: the work is a decision record and changes to error text and error mapping. It adds no child process, asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

The driver text in client replies and the wrong `InvalidQuery` label are bugs against rule 4 of the errors ADR, so the implementation phase follows `.claude/skills/fix-bug/SKILL.md`: baseline reproduction on a running tracker (the requests of the manual scenarios below), red tests, the change, green tests, and the same reproduction after the last production change, recorded in `manual-verification-evidence.md`. The ADR phase is not a bug fix.

## Regression Test Strategy

- **Unit**: for each error family, a test at the mapping seam (`error_mapping.rs`, the REST adapters, the UDP `From` conversions) that builds the error with a source carrying a unique marker string and asserts the client text does not contain it.
- **Real boundary, REST API**: a test with the existing REST test server asserting that the reply for a forced database failure (a duplicate key insert) holds no driver text, and that the server log holds the detail.
- **Real boundary, HTTP and UDP trackers**: a duplicate insert cannot reach them. An announce updates the in-memory repository and touches the database only in `load_downloads_metric_if_needed` (`packages/tracker-core/src/announce_handler.rs:196-205`), which reads the persisted download count when the torrent is not in memory; scrape reads it in `persisted_downloads.load_many`. So the test needs `core.tracker_policy.persistent_torrent_completed_stat = true`, an info-hash that is not in memory, and a read that fails (a database that was dropped or closed after start, or a test double of the persisted-downloads store that returns a database error). It asserts that the announce and scrape replies of each tracker hold no driver text.
- **UDP**: tests that the connection-id error holds no clock value and the parse error holds no type name.
- A helper that forbids known driver markers (`sqlx`, `error returned from database`, `constraint failed`, `Access denied`) in client text, used by the protocol tests.

## Implementation Plan

Test-development loop (required by `docs/templates/ISSUE.md`, "Commit Points", and the `create-issue` skill): the red tests are not written as one batch. Use the `write-unit-test` skill and work one behavior area at a time, in the order of the commit points below (T4 to T7). For each area: write the smallest behavior-focused test, record its red output, make the production change, record the green output, then perform and record an explicit test-design review before starting the next area. The review uses the prose-first Arrange-Act-Assert comparison (write temporary prose for each section, refactor until the code expresses it, remove redundant prose) and confirms that the test exposes the one causal initial-state difference, that its fixture owns only incidental mechanics, and that the production Act and the independently specified expected result stay visible. Record each review in the task's evidence. Commit each reviewed increment before the next one. After the last increment, stop for maintainer review before final verification, the final commit and the pull request.

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Draft the ADR | The decisions listed in the scope, with the options and the reasons; a table of every current client text and its proposed text. Its own pull request. |
| T2 | TODO | Maintainer accepts the ADR | No code before this. Update this specification with the approved texts. |
| T3 | TODO | Baseline reproduction and test plan | Evidence recorded; the list of tests per behavior area (T4 to T7) written. The tests are written red-then-green inside each area, with a recorded test-design review after each passing increment. |
| T4 | TODO | Database errors and the server-side event | Non-secret driver text stays in the source and the log (SI-A has already removed driver messages that hold a secret); the `tracing` event exists; the `InvalidQuery` label is right. |
| T5 | TODO | HTTP tracker text | As the ADR decides. |
| T6 | TODO | UDP tracker text and the error event | As the ADR decides. |
| T7 | TODO | REST API text | As the ADR decides, within v1's additive-only rule. |
| T8 | TODO | Skill and documentation | `handle-errors-in-code`; any documented example reply. |
| T8b | TODO | Maintainer checkpoint | Stop for maintainer review after the last test increment, before final verification, the final commit and the pull request. |
| T9 | TODO | Final recheck | The baseline reproduction repeated after the last production change. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T1 | The ADR and the index entry | Commit after focused validation and required review. |
| T4 | Database error mapping and the tracing event | Commit after the red and green runs and the recorded test-design review. |
| T5 | HTTP tracker text | Commit after the red and green runs and the recorded test-design review. |
| T6 | UDP tracker text and event | Commit after the red and green runs and the recorded test-design review. |
| T7 | REST API text | Commit after the red and green runs and the recorded test-design review. |
| T8 | Skill and documentation | Commit after focused validation and required review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] ADR accepted
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-22 13:50 UTC - Claude Code (Claude Fable 5.2) - Drafted from the EPIC inventory after the maintainer asked that the sub-issue specifications be part of the EPIC pull request. Not approved; no GitHub issue created.

## Acceptance Criteria

- [ ] AC1: An accepted ADR defines the client error text policy and answers each question listed in the scope.
- [ ] AC2: No reply of the HTTP tracker, the UDP tracker or the REST API holds database driver text, shown by a test through each protocol server and on a running tracker.
- [ ] AC3: A database failure leaves a `tracing` event on the server with the detail, and the operator can relate it to the client's reply.
- [ ] AC4: The UDP connection-id error holds no server clock value, and the UDP parse error holds no Rust type name.
- [ ] AC5: Every changed client text is listed, old and new, in this specification and was approved before the change.
- [ ] AC6: The `handle-errors-in-code` skill states the policy.
- [ ] AC7: Each test increment has a recorded test-design review, and the maintainer checkpoint after the last increment is recorded in the progress log.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior

## Verification Plan

### Automatic Checks

- `linter all`
- The unit and protocol tests named in the Regression Test Strategy
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | REST database failure | Add the same key twice through the REST API; read the reply and the server log | The reply follows the ADR and holds no driver text; the log holds the detail | TODO | `manual-verification-evidence.md` section V1 |
| M2 | HTTP and UDP whitelist rejection | Announce a non-whitelisted info-hash over HTTP and over UDP | One clear sentence each, no nesting beyond what the ADR allows | TODO | `manual-verification-evidence.md` section V2 |
| M3 | UDP connection id and short packet | Announce with a wrong connection id; send a 3-byte packet | No clock value; no type name | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Database down | On a MySQL or PostgreSQL tracker with `persistent_torrent_completed_stat = true`, stop the database and send an HTTP and a UDP announce for an info-hash that is not in memory, a scrape, and a REST key request | The client text follows the ADR and names neither the user nor the host; the log holds them | TODO | `manual-verification-evidence.md` section V4 |
| M5 | REST bad token and bad JSON | Call the API with a wrong token and with a malformed JSON body | Text as the ADR decides, within v1's rules | TODO | `manual-verification-evidence.md` section V5 |

Notes: manual verification is mandatory even when automated tests pass. Record the toolchain for every command, and create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Disposable Verification Scripts

A short UDP probe (connect, announce, raw packet) is needed for M2 and M3, because no maintained tool sends a wrong connection id or a truncated packet. Keep it in this issue's folder; the implementer removes it at close-out unless it is promoted to a Rust test. If it is written in Python, record why Rust was not suitable.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {test/log/PR link} |
| AC2 | TODO | {test/log/PR link} |
| AC3 | TODO | {test/log/PR link} |
| AC4 | TODO | {test/log/PR link} |
| AC5 | TODO | {test/log/PR link} |
| AC6 | TODO | {test/log/PR link} |
| AC7 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **Hiding detail from the REST administrator.** The administrator is trusted and today reads the driver text in the reply. Mitigation: the detail goes to the server log with the request id; the ADR may allow more for the authenticated REST API than for the public trackers.
- **Clients that match on text.** Mitigation: list old and new texts; the ADR says whether any text is a contract.
- **Divergence from API v2.** Mitigation: the ADR names how the v2 error model (EPIC #144) relates to the policy.
- **Over-generic messages that help nobody.** Mitigation: the maintainer's principle; a known client mistake keeps a specific message.
- **Conflict with SI-A and SI-4.** All three touch the tracker-core errors. Mitigation: run them one after the other: SI-4, then SI-A, then this issue's code.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Related issues: #2993 (EPIC), #2912, #384 (the 2023 report about what error replies reveal), #144 (API v2), #1166 (log levels)
- Related ADRs: `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`, `docs/adrs/20260727000000_events_are_objective_facts.md`
- Related drafts: `docs/issues/drafts/generalize-error-events/EPIC.md`
