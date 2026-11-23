---
schema-version: 1
doc-type: issue
issue-type: bug
status: planned
priority: p1
epic: 2993
github-issue: 2999
spec-path: docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md
branch: "2999-si-a-keep-secrets-out-of-errors-and-logs"
related-pr: null
last-updated-utc: "2026-11-23 20:42"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
    - handle-secrets
    - handle-errors-in-code
  related-artifacts:
    - .claude/skills/fix-bug/SKILL.md
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/handle-secrets/SKILL.md
    - docs/issues/open/2993-secrets-and-client-error-text/EPIC.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - docs/adrs/20261118045443_errors_do_not_record_source_locations.md
---

<!-- skill-link: create-issue -->

# Issue #2999 - SI-A Keep secrets out of errors, logs and output

Parent EPIC: [Keep secrets out of errors, logs and client replies, and make client error text safe](../2993-secrets-and-client-error-text/EPIC.md) (#2993). Sub-issue SI-A.

## Goal

No error value keeps a peer authentication key or a REST API admin token, no error message prints one, and no log line written for a real request or a configuration failure holds one. A legitimate user still gets a message that tells them what to fix.

## Background

The inventory of the EPIC (section "Inventory", items A1 to A12) lists every case found in the tree and reproduced on a running tracker. This issue owns the cases that are secrets: A1 to A8, A10 and A12. A9 is a documentation fix that belongs here too, and so is A12 (a database server message that holds the key). A11 (database server messages that name the user and host) is owned by SI-B. The commands and output of the reproductions are in the EPIC's [`manual-verification-evidence.md`](../2993-secrets-and-client-error-text/manual-verification-evidence.md). The inventory was rechecked against `develop` `6bc417424` on 2026-11-23 (nothing fixed; corrections and missed cases folded into this specification and the EPIC rows).

Start condition (maintainer, 2026-11-23): this issue starts after SI-5 of EPIC #2912 (#3001) merges, because both change `packages/configuration/src/lib.rs`. #3001 merged on 2026-11-23. Implementation session: Claude Fable 5.2 at high effort; no other implementation session of this EPIC runs at the same time.

The rule already exists: rule 3 of the errors ADR (`docs/adrs/20261118045443_errors_do_not_record_source_locations.md`) says a secret is never the offending input in a message and the error does not keep it, and the `handle-secrets` skill forbids secrets in logs and error text. The code predates both for the peer key, which is not a `SecretString` and prints in clear wherever it is formatted. The skill and the secrecy ADR cover configuration secrets only; they do not name the peer key or secrets carried in URLs.

The maintainer's principle (2026-11-22): help the legitimate user as much as possible without revealing the secret. Security does not rely on hiding how the tracker works, so a message may state facts about the client's own input (its length, a character outside the allowed set) and the state of the credential (not registered, expired), never the secret.

## Scope

### In Scope

1. **Errors that keep or print a key (A1, A2).**
   - `authentication::key::Error::UnableToReadKey` (`packages/tracker-core/src/authentication/key/mod.rs:176`) loses its `key` field. Its message becomes `Authentication key not registered` (wording confirmed in review).
   - `PeerKeyError::InvalidKey` (`packages/tracker-core/src/error.rs:124`) and the REST `AuthKeyError::InvalidKey { key, reason }` (`packages/rest-api-protocol/src/v1/context/auth_key/resources/auth_key.rs:31`, also built in `packages/rest-api-runtime-adapter/src/v1/adapters/auth_key.rs:54-57` and copied at `:79-82`) stop keeping the uploaded key. They keep the parse error, which already says what is wrong (length or characters), and the REST reply states the length that was received.
   - The REST responses `invalid_auth_key_response` (`packages/axum-rest-api-server/src/v1/context/auth_key/responses.rs:53-55`) and `invalid_auth_key_param_response` (`packages/axum-rest-api-server/src/v1/responses.rs:60-62`, `Invalid auth key id param "<key>"`) stop echoing the key.
   - The axum path rejection of `POST /api/v1/key/<seconds>` (`Path<u64>`, `packages/axum-rest-api-server/src/v1/context/auth_key/handlers.rs:65`) answers 400 ``Invalid URL: Cannot parse `<value>` to a `u64` ``, so a key posted by mistake to that route is echoed back. The reply stops echoing the rejected value; the 400 status stays.
2. **Messages for the peer key** (the proposal in the EPIC, to confirm in review): `Missing authentication key: this tracker is private, use /announce/<key>`; `Invalid authentication key: expected 32 characters, got <n>`; `Invalid authentication key: only the characters 0-9, a-z and A-Z are allowed`; `Authentication key not registered`; `Authentication key expired`. The malformed-key texts are produced in `packages/http-protocol/src/v1/auth.rs` and `packages/tracker-core/src/authentication/key/peer_key.rs`; the length case needs the length passed to the error, not the key. The HTTP extractor drops the `ParseKeyError` today (`packages/axum-http-server/src/v1/extractors/authentication_key.rs:111`, `:118`), so the length and the character cases cannot be told apart there until it keeps the kind. A fifth text, `Cannot extract authentication key param from URL path: expected /announce/<key> or /scrape/<key>` (`packages/http-protocol/src/v1/auth.rs:17`), holds no key and stays unless the review changes it.
3. **Request logging (A3, A4).** The HTTP tracker, the REST API and the health-check API log `%uri` in the request event and the `DefaultMakeSpan` span records it again. Replace both with a redacted URI: the path segment after `/announce/`, `/scrape/` and `/api/v1/key/`, and the value of the `token` query parameter, are replaced by a fixed marker (`***`, the marker the configuration redaction uses; decided 2026-11-23. Issue #3004 masks the tracker key in `tracker_client` output with its own function and `****`; the difference is accepted and this issue does not reuse or change it). The REST route is `{prefix}/key/{seconds_valid_or_key}` (`packages/axum-rest-api-server/src/v1/context/auth_key/routes.rs:24`), so redacting the segment after `/api/v1/key/` also hides the seconds value of `POST /api/v1/key/<seconds>`. The rule redacts every segment after `/announce/`, `/scrape/` and `/api/v1/key/` whatever its shape, because a malformed key (a valid key with a stray character, a wrong length) is still the user's secret; losing the seconds value in the log is accepted. One tested function owns the rule, shared by the three servers; its home is decided at implementation (a small module in an existing shared package, not a new package unless the review asks for it). The span must not record the raw URI either: use a custom `MakeSpan`.
4. **The generated key (A5).** Remove the `tracing::debug!` lines that print the generated key (`packages/tracker-core/src/authentication/key/mod.rs:111`, `:118`).
5. **The type (A7).** `Key` gets a redacting `Debug` (`Key([REDACTED])`). Its `Display` stays only where the key is deliberately handed to its owner or persisted (the generate and add responses, the database store), and each remaining use is listed in the issue's evidence. `PeerKey`'s manual `Display` is checked the same way. Types that hold a key and derive `Debug` (`InMemoryKeyRepository`, `AuthenticationService`, `AddKeyRequest`, `AddKeyForm`, the REST `AuthKey` resource, `AuthKeyError`; no code formats them today) are covered by the `Key` change where the field is a `Key`; a field typed `String` gets a redacting `Debug` or a recorded reason. Tests assert the redacted text without printing a test secret (`handle-secrets` skill).
6. **The REST client (A6).** The client strips the URL from every `reqwest` error before it wraps it (`without_url`): `TransportError` (`packages/rest-api-client/src/v1/client.rs:31`), `DeserializationError` (`:44`) and the body reads mapped to `TransportError` (`:205`, `:215`). The URL holds the token on GET requests and the peer key on a key delete (`delete_auth_key`, `:277`). The client sends the token in the `Authorization: Bearer` header on its POST, DELETE and token-aware GET requests (`:395-396`, `:476-497`), and on those GET requests it also adds it to the query string (`:369-370`), which is removed. Two exceptions stay as they are and are documented: `get_request` (`:514-517`) sends no token at all, and a GET whose caller supplies its own `Authorization` header keeps that header (`:486-489`). The free GET helper's send error (`:552`) is part of the `without_url` audit too. Check the tests that rely on the query token. The plain `String` token fields (A8) become `SecretString` where that is practical; if a field cannot change without a breaking change, record why.
7. **Configuration errors (A10).** A configuration error does not print the value of a secret option. For the `figment` error of a secret field (the database password, an access token), the loader reports the option path and the expected type with a fixed message and no value. How it is detected (the key path of the `figment` error against a list of secret paths, or reading the secret fields as strings and rejecting other types with a fixed message) is decided at implementation and recorded.
8. **Database errors that hold a secret (A12).** A database server message can contain the value that was written: adding a peer key that already exists makes MySQL answer `Duplicate entry '<peer key>' for key 'keys.key'` (reproduced; see the EPIC evidence file, section V8). `databases::error::Error` keeps the `sqlx` source and prints it, so the key is in the retained error and in the REST reply (and, before SI-4 of EPIC #2912, in a DEBUG log line). For the key store operations the error must not keep or print the driver message: map a unique-constraint violation to a variant of its own with a fixed message (for example "the key already exists"), using the driver's error kind (`sqlx::error::ErrorKind::UniqueViolation`), not its text; for any other driver error raised by a statement that binds a secret, keep the error kind and code and drop the message. Record at implementation which statements bind a secret (the `keys` table writes). What non-secret driver detail a client may see is SI-B.
9. **Documentation (A9 and the gap).** Update the `handle-secrets` skill and the secrecy ADR (or add an ADR if the review prefers) with the rule for the peer key and for secrets carried in URLs; fix `docs/containers.md` so the examples use the `Authorization` header and the v3 network database fields instead of `?token=` (lines 562, 650, 719) and a `mysql://` `path` (line 532, with its DSN percent-encoding advice at 535-537). Also fix the other places that teach `?token=` (decided 2026-11-23): the skills `use-rest-api` (`SKILL.md:43`), `manual-http-download-completion-e2e` (`:34`, `:66`, `:290`) and `manual-udp-download-completion-e2e` (`:28`, `:60`); the rustdoc of `src/lib.rs` (`:277`, `:387`), `packages/axum-rest-api-server/src/lib.rs` (`:10`, `:54`, `:59`, `:90`, `:155`), `v1/middlewares/auth.rs:20`, `v1/context/{auth_key,stats,torrent,whitelist}/mod.rs` and `v1/context/torrent/handlers.rs:54`, `:59`; and `contrib/dev-tools/experiments/dual-stack-sockets/README.md:61`. Where a document explains the query form on purpose (the server still accepts it), it says that the header is the safe form.

### Out of Scope

- The wording of client text for non-secret errors, and database text for clients (SI-B).
- Source locations in messages (EPIC #2912). A message this issue rewrites must not gain one.
- The user and host in database server messages (SI-B).
- Changing authentication behaviour, key lifetimes or REST status codes.
- Moving the peer key out of the URL: BEP 3 clients put it there, and the tracker protocol fixes it.
- The CI tools under `src/console/ci/` that log configuration, environment values or database URLs (EPIC, "Verified by running experiments"): a follow-up issue of their own (maintainer, 2026-11-23).
- The tracker key in `tracker_client` output (#3004).

## Architectural Decisions

- Related ADRs: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`, `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`.
- ADRs to create: none known. The peer-key rule may be recorded by amending the secrecy ADR; decide in review.

## Design and Ownership Review

Not applicable: the work changes error types, a log formatting function, a type's `Debug` and documentation. It adds no child process, asynchronous I/O, network readiness, resource cleanup or reusable test fixture.

## Bug-Fix Process

This is substantively a bug (a secret printed to a client and to logs), so it follows `.claude/skills/fix-bug/SKILL.md`:

- analysis: the call paths and audiences are in the EPIC inventory; re-check them against the tree before the first change (SI-4, #2916, merged on 2026-11-22 and removed the source locations from these errors);
- real-artifact reproduction: run a tracker and send the requests of the manual scenarios below; record each reply and the log lines. The EPIC's session reproduced them, and the commands, configuration and output are in the EPIC's `manual-verification-evidence.md`; repeat that baseline on the commit this issue starts from;
- regression-test boundary: see Regression Test Strategy;
- red regression-test evidence, production fix, green test evidence, and the same reproduction run again after the last production change, all recorded in `manual-verification-evidence.md`.

## Regression Test Strategy

- **Errors**: a unit test per changed variant that builds the error from a unique test key and asserts that neither `Display` nor `Debug` contains it, and that the message still names what failed. Write it first and record its red output.
- **Log redaction**: a unit test of the shared redaction function over the URI shapes (`/announce/<key>?...`, `/scrape/<key>`, `/api/v1/key/<key>?token=<t>`, a bare `?token=`, a URI with no secret, a percent-encoded key). A test through each server captures the tracing output with a unique test key and token and asserts neither appears in it.
- **Generated key**: a test that captures the debug output of key generation.
- **Configuration**: a test that loads a configuration with a numeric database password and a numeric access token and asserts the error contains the option path and not the value.
- **Database errors**: a test that adds the same key twice and asserts that the retained error (`Display`, `Debug` and its source chain), the REST reply and the captured log hold no key. It must run against MySQL, the driver that prints the value, so it belongs with the database-backed driver tests; the SQLite and PostgreSQL runs assert the same thing and guard against a future driver message change.
- **REST client**: a test that forces a transport error on a GET with a token and on a key delete, and asserts the text holds neither the URL query, the token nor the key.
- **REST path rejection**: a test that posts a 32-character key to `POST /api/v1/key/<seconds>` and asserts a 400 reply that does not hold the key.
- **Redaction of malformed segments**: the redaction function test includes a key with a stray character and a key of the wrong length after `/announce/` and `/api/v1/key/`.
- **Existing tests that pin today's texts** change with the messages: `packages/tracker-core/src/authentication/service.rs:220`, `packages/axum-http-server/src/v1/handlers/announce.rs:504`, `packages/axum-http-server/tests/server/v1/contract/failure_reasons_hold_no_source_path.rs:218`, and the rustdoc of `extractors/authentication_key.rs:35`. The shared driver test `it_should_fail_without_a_source_path_trying_to_add_the_same_key_twice` (`packages/tracker-core/src/databases/driver/mod.rs`) is the natural place for the duplicate-key assertion.
- **Real boundary**: the existing protocol tests (`packages/axum-http-server/tests`, the REST API server tests) get assertions that a failure reason and a REST error hold no key.

## Implementation Plan

Test-development loop (required by `docs/templates/ISSUE.md`, "Commit Points", and the `create-issue` skill): the red tests are not written as one batch. Use the `write-unit-test` skill and work one behavior area at a time, in the order of the commit points below. For each area: write the smallest behavior-focused test, record its red output, make the production change, record the green output, then perform and record an explicit test-design review before starting the next area. The review uses the prose-first Arrange-Act-Assert comparison (write temporary prose for each section, refactor until the code expresses it, remove redundant prose) and confirms that the test exposes the one causal initial-state difference, that its fixture owns only incidental mechanics, and that the production Act and the independently specified expected result stay visible. Record each review in the task's evidence. Commit each reviewed increment before the next one. After the last increment, stop for maintainer review before final verification, the final commit and the pull request.

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Baseline reproduction | The EPIC cases A1 to A6, A10 and A12 reproduced on the starting commit (after #3001 merged); evidence recorded. |
| T2 | TODO | Test plan | The list of tests per behavior area (T3 to T8), written before the first one. The tests themselves are written red-then-green inside each area, with a recorded test-design review after each passing increment. |
| T3 | TODO | Key errors and messages | A1, A2 and the new peer-key messages; the REST responses stop echoing the key. |
| T4 | TODO | `Key` type | Redacting `Debug`; remaining `Display` uses listed and justified. |
| T5 | TODO | Request logging | The shared redaction function; custom `MakeSpan` and `on_request` in the three servers; the generated-key log lines removed. |
| T6 | TODO | REST client | `without_url`, header token, `SecretString` where practical. |
| T7 | TODO | Configuration errors | A10: no secret value in a configuration error. |
| T7b | TODO | Database errors | A12: the key store errors keep and print no driver message that holds a key; MySQL duplicate-key regression test. |
| T8 | TODO | Documentation | `handle-secrets` skill, the ADR change, `docs/containers.md`. |
| T8b | TODO | Maintainer checkpoint | Stop for maintainer review after the last test increment, before final verification, the final commit and the pull request. |
| T9 | TODO | Final recheck | The baseline reproduction repeated after the last production change. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T3 | Key errors, messages and REST responses, with their tests | Commit after the red and green runs and the recorded test-design review. |
| T4 | `Key` and `PeerKey` formatting | Commit after the red and green runs and the recorded test-design review. |
| T5 | Redaction function and the three servers | Commit after the red and green runs and the recorded test-design review. |
| T6 | REST client | Commit after the red and green runs and the recorded test-design review. |
| T7 | Configuration errors, with their tests | Commit after the red and green runs and the recorded test-design review. |
| T7b | Database errors that hold a key, with the MySQL regression test | Commit after the red and green runs and the recorded test-design review. |
| T8 | Skill, ADR and documentation | Commit after the red and green runs and the recorded test-design review. |

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Spec-only PR merged into `develop` before implementation (#2994; refreshed by #3014, which must merge before implementation too)
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
- 2026-11-23 11:06 UTC - Claude Code (Claude Sonnet 5.5) - The maintainer approved the specification; created #2999 and moved it to `docs/issues/open/`. Merged with the EPIC by pull request #2994.
- 2026-11-23 18:08 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator session - Rechecked against `develop` `6bc417424` before the start: nothing fixed. Added the missed sites (the third `InvalidKey` site, the other `reqwest` wrap sites and the peer key in the delete URL, the fifth key text, the other `?token=` teachers), corrected the REST client description and file paths, and recorded the maintainer's answers of 2026-11-23 (start after #3001, marker `***`, CI tools out of scope, one session with Claude Fable 5.2 at high effort).
- 2026-11-23 19:29 UTC - Claude Code (Claude Opus 5.5), EPIC coordinator session - Review of PR 3014: added the axum path rejection of `POST /api/v1/key/<seconds>`, made the URI redaction cover every segment whatever its shape, and named the REST client's GET exceptions and the free GET helper's send error.

## Acceptance Criteria

- [ ] AC1: No error value keeps a peer key or an admin token, and no `Display` or `Debug` of an error prints one.
- [ ] AC2: A failure reason for a bad, unknown, expired or missing key tells the user which of the four cases it is, and holds no key.
- [ ] AC3: No log line written for a request to the HTTP tracker, the REST API or the health-check API holds a peer key or an admin token, shown on a running tracker with `trace_filter = "debug"`.
- [ ] AC4: Generating a key does not write it to the log.
- [ ] AC5: `Key` prints as `Key([REDACTED])` in `Debug`, and every remaining `Display` use is deliberate and listed.
- [ ] AC6: A transport error of the REST client holds neither the token, the peer key nor the URL query.
- [ ] AC7: A configuration error for a secret option names the option and the expected type and does not print the value.
- [ ] AC8: The `handle-secrets` skill and an ADR state how the peer key and URL-borne secrets are handled, and `docs/containers.md`, the skills and the rustdoc no longer teach `?token=`, and `docs/containers.md` no longer teaches a `mysql://` `path`.
- [ ] AC9: Adding a peer key that already exists leaves no key in the retained error, the REST reply or the log, on MySQL, PostgreSQL and SQLite.
- [ ] AC10: Each test increment has a recorded test-design review, and the maintainer checkpoint after the last increment is recorded in the progress log.
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
| M1 | Wrong peer key on a private tracker | Announce with an unregistered key, a 14-character key, a key with a `-`, an expired key, and no key | Five failure reasons, one per case, none holding the key | TODO | `manual-verification-evidence.md` section V1 |
| M2 | REST key errors | `POST /api/v1/keys` with a short key; `DELETE /api/v1/key/<bad>` | 400 texts that state the length or the rule and not the key | TODO | `manual-verification-evidence.md` section V2 |
| M3 | Request logs | With `trace_filter = "debug"`, send an announce with a key, a REST call with `?token=`, and a key delete; read the log | No line holds the key or the token; the `uri=` fields show the marker | TODO | `manual-verification-evidence.md` section V3 |
| M4 | Generated key | `POST /api/v1/key/3600` with `trace_filter = "debug"` | The log has no `Generated key` line with the key | TODO | `manual-verification-evidence.md` section V4 |
| M5 | Configuration error | Start the tracker with `password = 987654321` and with a numeric access token | The error names the option and the expected type, not the number | TODO | `manual-verification-evidence.md` section V5 |
| M6 | REST client | Point the client at a closed port with a token; do a GET and a key delete | The error text holds no token and no key | TODO | `manual-verification-evidence.md` section V6 |
| M7 | Duplicate key on MySQL | On a tracker with a MySQL database and `trace_filter = "debug"`, add the same key twice through the REST API; read the reply and the log | Neither holds the key | TODO | `manual-verification-evidence.md` section V7 |

Notes: manual verification is mandatory even when automated tests pass. Record the toolchain for every command, and create `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.

### Disposable Verification Scripts

None planned. The UDP and HTTP probes of the EPIC session were ad-hoc commands; promote any durable check to a Rust test.

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
| AC8 | TODO | {test/log/PR link} |
| AC9 | TODO | {test/log/PR link} |
| AC10 | TODO | {test/log/PR link} |

## Risks and Trade-offs

- **Redacting the URI loses debugging value.** An operator cannot see which key a client used. Mitigation: if correlation is needed, log a short stable fingerprint (a truncated hash) instead of the key; decide in review.
- **Overlap with EPIC #2912.** SI-4 (#2916) has merged. SI-5 (#2917, pull request #3001) rewrites the configuration errors that item 7 also touches. Decided 2026-11-23: this issue starts after #3001 merges; it merged the same day.
- **A breaking change in a public error enum.** `UnableToReadKey` loses a field. Mitigation: tracker-core is not published yet, and the enum is `#[non_exhaustive]`.
- **Moving the REST client token to the header** breaks nothing on the server, which accepts both, but changes what a proxy log sees. Mitigation: record it in the release notes.
- **Detecting secret options in a `figment` error** can miss a new secret option. Mitigation: derive the list from the `SecretString` fields, or test that each secret field is covered.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this directory; otherwise record in the progress log why none was needed.

## References

- Related issues: #2993 (EPIC), #2912, #2916 (SI-4, where the finding arose), #2917 (SI-5, startup errors, pull request #3001), #2079 (adopt `secrecy`), #3004 (tracker key in `tracker_client` output)
- Related ADRs: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`, `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`
