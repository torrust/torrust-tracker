---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 2993
spec-path: docs/issues/open/2993-secrets-and-client-error-text/EPIC.md
epic-owner: null
last-updated-utc: "2026-11-22 13:24"
semantic-links:
  skill-links:
    - create-issue
    - handle-secrets
    - handle-errors-in-code
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - .claude/skills/handle-secrets/SKILL.md
    - .claude/skills/handle-errors-in-code/SKILL.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - docs/adrs/20261118045443_errors_do_not_record_source_locations.md
    - docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md
    - docs/issues/closed/2916-2912-si-4-tracker-core/ISSUE.md
    - docs/issues/open/384-strip-location-from-errors-sent-to-clients/ISSUE.md
---

<!-- skill-link: create-issue -->

# EPIC #2993 - Keep secrets out of errors, logs and client replies, and make client error text safe

Related to EPIC #2912 (Remove source locations from errors), but not a sub-issue of it. See "Relation to EPIC #2912".

## Goal

1. No secret (a peer authentication key, a REST API admin token, a database credential) is held by an error value, printed by an error message, or written to a log line or other output.
2. Text sent to a client of the HTTP tracker, the UDP tracker or the REST API is friendly and relevant, and gives an attacker nothing about the implementation (database driver text, nested internal error names, server timing values).

## Why This Is Needed

The review of the SI-4 pull request of EPIC #2912 (#2916) found that `authentication::key::Error::UnableToReadKey` keeps the peer key and prints it, against rule 3 of the errors ADR. The maintainer decided on 2026-11-22 that this is three separate problems:

1. Removing source locations from errors. This is EPIC #2912 and stays its only scope.
2. Not leaking secrets in logs, output and error text. This EPIC, sub-issue SI-A.
3. Client-facing errors that are friendly and relevant without exposing information an attacker can use. This EPIC, sub-issue SI-B.

The two problems differ in kind. SI-A is a bug class with concrete, listed cases. SI-B is a design question (what a client may be told) that needs an ADR before code.

## Inventory (verified against the tree at `develop` commit `bcca9764c`, 2026-11-22)

SI-4 of EPIC #2912 (#2916, pull request #2991) merged into `develop` on 2026-11-22 as `4fa49ec9c`, after this inventory was made. It removed the source locations from the tracker-core errors and nothing else that matters here: a re-run on a build that includes it (evidence file, section V9) shows the same replies without the location suffix, the key still echoed, and the MySQL duplicate-key reply still holding the key. The `tracker-core` line numbers below were refreshed against the tree that includes SI-4; the quoted reply texts in "Verified by running experiments" are from the run before it and still show locations.

Paths are relative to the repository root. Every item was read in the code, and the leaks were reproduced on a running tracker (see "Verified by running experiments"). The one thing not checked is how a source location prints in a release build; the debug build prints the workspace-relative path (`packages/tracker-core/src/authentication/service.rs:98:27`), and that is EPIC #2912's concern.

### A. Secrets held by errors or written to output

| ID | Location | What leaks | Seen by |
| --- | --- | --- | --- |
| A1 | `packages/tracker-core/src/authentication/key/mod.rs:176` (`UnableToReadKey { key: Box<Key> }`, built in `authentication/service.rs:96`) | the full 32-character peer key, kept and printed (`Failed to read key: {key}`) | the HTTP client that sent the key |
| A2 | `packages/tracker-core/src/error.rs:124` (`PeerKeyError::InvalidKey { key: String }`, built in `authentication/handler.rs:103`, `:118`) and `packages/rest-api-protocol/src/v1/context/auth_key/resources/auth_key.rs:31` (`InvalidKey { key, reason }`, Display `invalid key: "{key}"`) | the key an administrator uploaded, echoed back | the REST API administrator; REST handlers `auth_key/handlers.rs:38-46` and `invalid_auth_key_response` (`responses.rs:53-55`) |
| A3 | `packages/axum-http-server/src/v1/routes.rs:82-104` (`TraceLayer`, `DefaultMakeSpan` at INFO plus `on_request` with `%uri`) | the peer key in `/announce/<key>` and `/scrape/<key>`, plus the whole query string, in the request event and in the span of every event inside it | server log reader |
| A4 | `packages/axum-rest-api-server/src/routes.rs:76-98` (same pattern; the auth middleware is inside the trace layer, so wrong tokens are logged too) | the admin token in `?token=<token>` and the key in `/api/v1/key/<key>` | server log reader |
| A5 | `packages/tracker-core/src/authentication/key/mod.rs:111`, `:118` (`tracing::debug!("Generated key: {}...")`) | a newly generated peer key, in clear, at DEBUG | server log reader |
| A6 | `packages/rest-api-client/src/v1/client.rs:30` (`TransportError(reqwest::Error)`) and `:369-370` (token added to the query string); `without_url` is never called | reqwest prints the URL, so a transport error on a GET contains `?token=<token>` | users of the client library and console tools |
| A7 | `packages/tracker-core/src/authentication/key/peer_key.rs:127-128`, `:46-47`, `:71-75` (`Key` and `PeerKey` derive `Debug`, `Display`, `Serialize`) | the peer key prints in clear wherever the value is formatted | any log or error that formats a `Key` |
| A8 | `packages/axum-rest-api-server/src/v1/middlewares/auth.rs:62-65` (`QueryParams { token: Option<String> }` derives `Debug`); `packages/rest-api-client/src/connection_info.rs:7-9` (`api_token: Option<String>`) | plain `String` tokens; no log of them was found | none today; a latent risk |
| A9 | `docs/containers.md` (lines about 492-562) | the documentation teaches `?token=` URLs and a `mysql://user:password@host` URL in config and environment | readers; the request log then records the token. The MySQL example (`path = "mysql://db_user:...@mysql:3306/torrust_tracker"`) is also stale: the v3 schema refuses `path` for a network database (`packages/configuration/src/v3_0_0/database.rs:203-204`) |
| A10 | `packages/configuration/src/v3_0_0/mod.rs:409` (`figment.extract()`), wrapped by `Error::ConfigError` (`packages/configuration/src/lib.rs:335`) | a configuration error prints the offending value, so a numeric database password or access token is printed (details below) | the operator: stderr and the server log |
| A11 | `packages/tracker-core/src/databases/error.rs:92-116` (`From<(SqlxError, Driver)>` keeps the `sqlx` text) | a database server message names the database user and the client address (details below). Not a secret; owned by SI-B | the operator today; a client wherever driver text is forwarded |
| A12 | `packages/tracker-core/src/databases/error.rs:92-116` (the `sqlx` source is kept and printed), reached from `packages/tracker-core/src/databases/driver/mysql/auth_key_store.rs` | a database server message can contain a secret: adding a peer key that already exists makes MySQL answer `Duplicate entry '<peer key>' for key 'keys.key'`, and the error keeps and prints it. PostgreSQL (`duplicate key value violates unique constraint "keys_key_key"`) and SQLite (`UNIQUE constraint failed: keys.key`) do not print the value | the REST administrator (500 reply). Before SI-4 of EPIC #2912 also the server log at DEBUG, through the `torrust_located_error` event; that line is gone since SI-4 |

Checked and found safe, so that the spec does not redo it:

- Config tokens (`AccessTokens`) and the v3 database password are `SecretString`; `Debug` and the startup configuration log (`src/bootstrap/app.rs:66`, `to_redacted_json`) redact them. `Serialize` redacts them too (`serialize_with`).
- No `databases::error::Error` variant has a URL, DSN or password field; the connection URL goes only to `from_str`.
- No event payload, metric label or health-check output carries a key, token or credential.
- The `Authorization: Bearer` header is not logged (`include_headers` is off).
- The health-check API server has the same `TraceLayer` and logs `%uri` (`packages/axum-health-check-api-server/src/server.rs:151-167`), but its routes (`/`, `/health_check`) carry no secret. SI-A gives it the same redaction function so that the three servers behave alike.
- No `#[instrument]` takes a key, token or database URL without skipping it.

Verified by running experiments (2026-11-22, a debug build of the tracker; MySQL 8.0 and PostgreSQL 16 in local containers). The commands, configuration files and output are in [`manual-verification-evidence.md`](manual-verification-evidence.md):

- A1, A3 and A4 were reproduced on a running tracker (A2 and A5 in the next item, A10 and A11 below). A6, A7 and A8 were confirmed by reading the code, and A9 is documentation. For A1, A3 and A4: the announce failure reason `Tracker authentication error: Failed to read key: <key>, <location>` carried the sent key, and the request log held `uri=/announce/<key>?...` and `uri=/api/v1/stats?token=<wrong token>`. With `Authorization: Bearer`, the admin token did not appear in the log, so the header form is the safe way to call the API today.
- More reproductions on the same build. A2: `POST /api/v1/keys` with a 14-character key returned 400 `Invalid URL: invalid auth key: string "<the key>", Invalid key length. Key must be have 32 chars, <location>`. The text is verbatim, including the grammar slip "must be have" (`packages/tracker-core/src/authentication/key/peer_key.rs:195`); SI-A replaces this message. A5: with `trace_filter = "debug"`, `POST /api/v1/key/3600` wrote `Generated key: <the 32-character key>, valid for: ...` to the log. A4: `DELETE /api/v1/key/<key>` wrote `uri=/api/v1/key/<key>`. A duplicate key returned 500 `Unhandled rejection: Err { reason: "failed to add key: The Sqlite3 query was malformed: error returned from database: (code: 2067) UNIQUE constraint failed: keys.key, <two locations>" }`. The ERROR log lines for a 500 reply hold the status and the request id but not the body, so the server log does not tell an operator why the request failed.
- A6 by reading: `reqwest` appends `for url (<url>)` to its `Display` (`reqwest-0.13.5/src/error.rs:300-302`), and the REST client has no header form: it always adds the token to the query string (`packages/rest-api-client/src/v1/client.rs:369-370`). Its users in the workspace are the REST server tests and the qBittorrent end-to-end tool (`src/console/ci/qbittorrent_e2e/tracker/client.rs`).
- UDP replies, on a public tracker with `listed = true`: a non-whitelisted announce returned `tracker announce error: Tracker core whitelist error: The torrent: <info-hash>, is not whitelisted, <location>`; an announce with a wrong connection id returned `tracker announce error: Connection cookie error: cookie value is expired: <decoded cookie as f64>, expected > 1795353220.6572893`, which gives the client the server clock and the decoded value of the cookie it sent; a 3-byte packet returned `error parsing request: SendableRequestParseError { message: "Couldn't parse action", opt_connection_id: None, opt_transaction_id: None }`, a Rust type name in client text.
- UDP trackers are not started in private mode (`Could not start UDP trackers while in private mode. UDP is not safe for private trackers!`), so the peer key never travels over UDP.
- A10, configuration errors echo secret values. `figment` prints the offending value (`Actual::Str` and the integer forms, `figment-0.10.19/src/error.rs:323-345`). Reproduced: `password = 987654321` makes the tracker print `invalid type: found signed int 987654321, expected a string for key "default.core.database.password"`, and `admin = 424242` under `[http_api.access_tokens]` prints the number the same way. A numeric password or token, which TOML reads as an integer, is therefore printed on stderr and in the log. Seen by the operator. Related: `Error::ConfigError` is built by `LocatedError`, so the line also holds a source location (SI-5 of EPIC #2912).
- A11, database server messages name the user and the client address. Reproduced with MySQL: `1045 (28000): Access denied for user 'db_user'@'172.17.0.1' (using password: YES)`; with PostgreSQL: `password authentication failed for user "db_user"`. sqlx passes the server message through unchanged (`sqlx-mysql-0.8.6/src/error.rs:44-49`). Today this reaches the operator (stderr and log). It reaches a client only when a runtime database failure is mapped into a client reply (section B), which is why SI-B must not forward driver text. The MySQL case is also labelled `The MySQL query was malformed` because `From<(SqlxError, Driver)>` maps every `Database` error to `InvalidQuery`, so the label is wrong as well.
- The v2 configuration is not reachable at runtime. The tracker loads `v3_0_0::Configuration` only (`src/bootstrap/config.rs:8`); a file with `schema_version = "2.0.0"` is refused with `UnsupportedVersion` (`v3_0_0/mod.rs:413`). Only the v2 `Logging` types are still imported (`packages/configuration/src/logging.rs:18`). So the v2 `Database.path` masking issue is dead code, and is out of scope for SI-A.
- `url::ParseError` is safe: its `Display` is a fixed string per variant and never includes the input (`url-2.5.9/src/parser.rs:63-70`), so a malformed database URL does not echo the password.
- `src/console/ci/e2e/runner.rs:80` logs the raw tracker configuration TOML. It is a CI tool, not the tracker binary; record it and decide.

### B. What each protocol sends a client

HTTP tracker (a bencoded `failure reason`, always HTTP 200; wrapper `packages/http-protocol/src/v1/responses/error.rs:21-25`):

| Error family | Client text (shape) | Source |
| --- | --- | --- |
| authentication | `Tracker authentication error: Failed to read key: <key>[, <location>]`, `Missing authentication key`, `Key has expired` | `packages/http-core/src/services/error_mapping.rs:4-21`, `packages/tracker-core/src/authentication/key/mod.rs:176-185` |
| whitelist | `Tracker whitelist error: The torrent: <info-hash>, is not whitelisted` | `packages/tracker-core/src/error.rs:102` |
| database (announce and scrape) | `Tracker announce error: Database error: <sqlx text>` | `packages/tracker-core/src/error.rs:77`, `:90`; `databases/error.rs` |
| query parse | `Bad request. Cannot parse query params: invalid param <raw param>` and per-parameter texts that echo the client's own values | `packages/http-protocol/src/v1/requests/announce.rs:216-245`, `:346-354`, `query.rs:102` |
| client IP | `Error resolving peer IP: missing or invalid right-most IP in the X-Forwarded-For header, which is mandatory when the tracker runs behind a reverse proxy` | `peer_ip_resolver.rs:112-121` |

UDP tracker (the error packet message is `error.to_string()` of `udp_server::error::Error`, `handlers/error.rs:57-60`):

| Error family | Client text (shape) | Source |
| --- | --- | --- |
| request parse | `error parsing request: {request_parse_error:?}` (Debug of a struct) | `packages/udp-server/src/error.rs:24` |
| connection cookie | `... Connection cookie error: cookie value is expired: <f64>, expected > <f64>` (server timing values) | `packages/udp-core/src/connection_cookie.rs:112-118` |
| whitelist | `tracker announce error: Tracker core whitelist error: The torrent: <info-hash>, is not whitelisted` | `packages/udp-core/src/services/announce.rs:190-204` |
| database | `tracker announce error: Tracker core announce error: Database error: Failed to connect to <driver> database: error communicating with database: <io error>` | `packages/tracker-core/src/databases/error.rs:92-116` |
| internal | `internal server error: {message}` | `packages/udp-server/src/error.rs:36` |

REST API (`packages/axum-rest-api-server/src/v1/responses.rs`):

| Error family | Client text (shape) | Status |
| --- | --- | --- |
| any key, whitelist or reload failure | `Unhandled rejection: Err { reason: "failed to add key: <driver error text>" }` (Debug formatting of `ActionStatus`) | 500 |
| missing or wrong admin token | `Unhandled rejection: Err { reason: "unauthorized" }`, `"token not valid"`, `"unknown token provided"` | 500 |
| bad info-hash or key parameter | `Invalid URL: invalid infohash param: string "<value>", expected a 40 character long string`; `Invalid URL: invalid auth key: string "<key>", <reason>` (echoes the supplied key) | 400 |
| feature disabled | `{"status":"err","reason":"private capability is disabled by configuration"}` | 409 |

Database error text. `databases::error::Error` (`packages/tracker-core/src/databases/error.rs`) wraps the `sqlx` error and its `Display` (for example `error returned from database: (code: 2067) UNIQUE constraint failed: keys.key`). `rest-api-runtime-adapter` forwards `e.to_string()` into `AuthKeyError::Database(String)` and `WhitelistError::Database(String)`, which the REST response prints verbatim (`adapters/auth_key.rs:49-84`, `adapters/whitelist.rs:32-46`). Server-side error logs also carry the driver text (`statistics/event/handler.rs:108`, `:129`; `packages/udp-server/src/handlers/error.rs:103-121`), which is appropriate there.

REST rejections raised by axum itself (verified on a running tracker): an unknown path such as `/api/v1/key/abc/extra` returns 404 with an empty body; a malformed JSON body returns 400 with `Failed to parse the request body as JSON: key must be a string at line 1 column 2`, which is serde's text and so a source error from a crate the workspace does not control (ADR rule 4).

Event payloads. `packages/udp-server/src/event.rs:114-153` `ErrorKind` holds `.to_string()` of internal errors (including `Database(String)`); no other event type carries an error string.

### C. Documentation on handling secrets (inventory)

| Kind | Artifact | What it covers | Gap |
| --- | --- | --- | --- |
| Skill | `.claude/skills/handle-secrets/SKILL.md` (listed in `.claude/skills/README.md:118`) | `SecretString`, `expose_secret` only at the last boundary, never in logs, `Display` or errors, constant-time compare, redaction tests | configuration secrets only; the peer key and secrets carried in URLs are not named |
| Skill | `.claude/skills/handle-errors-in-code/SKILL.md` | a secret is never echoed or kept in an error; client text is the error's own message | no rule for what client text may say about infrastructure failures |
| ADR | `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md` | use `secrecy` directly; redacted diagnostics; keep manual masking for legacy URLs until migrated | the peer key and request logging are not covered |
| ADR | `docs/adrs/20261118045443_errors_do_not_record_source_locations.md` | rule 3 (a secret is never the offending input) and rule 4 (client text is the workspace error's `Display`) | does not say how much a `Display` may reveal |
| ADR | `docs/adrs/20261007085634_inject_the_udp_connection_cookie_cipher.md` | the cipher key prints as `CookieCipher([REDACTED])` | none |
| ADR | `docs/adrs/20260519000000_define_global_cli_output_contract.md`, section 7 | JSON output must not expose secrets; `mask_secrets()` | CLI output only |
| Security doc | `docs/security/analysis/reports/2026-09-04_rest-api-token-timing.md` | constant-time token comparison; recheck when a new secret-comparison site appears | none |
| Security docs | `SECURITY.md`, `docs/security/README.md`, `vulnerability-remediation.md`, `public-scanner-findings.md` | reporting and triage process | not code guidance |
| User doc | `docs/containers.md` | configuration through environment variables | teaches `?token=` URLs and a clear database URL |
| History | `docs/issues/closed/2079-adopt-secrecy-for-sensitive-configuration/`, `docs/issues/closed/1490-1978-decompose-database-configuration/` | how config secrets were migrated | none |
| Incidental | skills `implement-workflow`, `merge-pull-request`, `triage-github-security-findings`, `check-runner-health`; `.github/agents/committer.agent.md`, `clippy-fixer.agent.md` | CI, runner and commit-hygiene credentials | not about code secrets |

There is no ADR on peer-key handling, request-log redaction or client-facing error text.

## Scope

### In Scope

SI-A, secrets (a bug class):

- Remove every cluster of A1 to A8, A10 and A12 (A12: a database error must not keep or print a driver message that holds a secret) (a configuration error must not echo a secret value, for example by building the figment error message without the offending value or by rejecting a non-string secret with a fixed message): no error keeps or prints a key or token; request logging of both servers redacts the key path segment and the `token` query parameter (the maintainer's decision, 2026-11-22); the generated key is not logged in clear; the REST client strips the URL from transport errors; `Key` gets redacting `Debug` and no clear `Display` where a clear value is not needed.
- A rule for the peer key in the `handle-secrets` skill (and an ADR or an update of the secrecy ADR, decided in SI-A), and a fix of the `docs/containers.md` examples (A9).
- Tests that assert redaction without printing the secret, and a regression test per leak.

SI-B, client error text (a design question):

- An ADR that defines what a client may be told, per protocol and error family, and what must stay on the server (database driver text, internal error nesting, timing values, echo of client input). The ADR decides the text for database failures; the maintainer chose to decide it there.
- The implementation of that ADR for the HTTP tracker, the UDP tracker, the REST API and the event payloads, with each old and new text listed for approval, as the SI-3 and SI-4 specs do.
- A way for a developer to find the detail the client no longer sees (a `tracing` event on the server, perhaps with a request id the client can quote); the ADR decides.

### Out of Scope

- Removing source locations (EPIC #2912, SI-2 to SI-7).
- Changing authentication behaviour, key lifetimes or the REST API status codes and response shapes (EPIC #144, API v2). SI-B changes text only unless the ADR decides otherwise and says why.
- The general error-event contract (draft `docs/issues/drafts/generalize-error-events/`).
- The configuration-file secrets already covered by #2079, except the legacy v2 `Database.path` if SI-A decides to handle it.
- Server-side log content other than secrets (driver text in server logs is correct and stays).

## Relation to EPIC #2912

Decided by the maintainer on 2026-11-22: this EPIC is standalone and linked. It is not a sub-issue of #2912, so #2912 keeps its single scope (source locations). Effects on #2912:

- SI-4 (#2916, PR #2991) merged on 2026-11-22. It did not fix A1 or A2; it links this EPIC for the standing review finding.
- SI-5 (#2917): one interaction. Its AC2 says the startup error for a bad configuration file "names the file and the invalid key or value". For a secret option (the database password, an access token) the value must not be printed (A10). SI-5 should name the option and the expected type only for those options, or leave the value to SI-A; the maintainer decides which issue owns it.
- SI-6 (#2918) is unchanged.
- SI-7 (#384) is unchanged: it checks that no client reply holds a source path. Its out-of-scope note about other information in a reply (for example database error codes) points to SI-B.
- Order: nothing here blocks #2912. SI-4 has merged, so SI-A can start on the tracker-core errors.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| Order | Issue | Local Spec | Status | Notes |
| --- | --- | --- | --- | --- |
| 1 | #[To be assigned] - SI-A Keep secrets out of errors, logs and output | `docs/issues/drafts/2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md` | TODO | SI-4 (#2916) has merged, so it can start. A bug class with the listed cases A1 to A9 |
| 2 | #[To be assigned] - SI-B Define and apply the client error text policy | `docs/issues/drafts/2993-si-b-client-error-text-policy/ISSUE.md` | TODO | Starts with an ADR; implementation after the maintainer accepts the ADR. May start in parallel with SI-A, but it touches the same error types, so run them one after the other |

Both sub-issue specifications are drafts until the maintainer approves them; their GitHub issues are created then.

## Delivery Strategy

One pull request per sub-issue, each spec-first. SI-A first, because it is a bug with known cases. SI-B writes the ADR first and waits for the maintainer's acceptance before code.

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Phase 1: secrets

- Outcome: SI-A merged.
- Exit criteria: no log line produced by a real request holds a peer key or an admin token (shown on a running tracker); no error value or message holds one; the skill and ADR text cover the peer key.

### Phase 2: client text

- Outcome: the SI-B ADR accepted and applied.
- Exit criteria: the replies of the three protocols for each error family match the ADR; no database driver text reaches a client.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic spec drafted in `docs/issues/drafts/` and moved to `docs/issues/open/`
- [x] Epic spec reviewed and approved by user/maintainer
- [x] GitHub epic issue created and issue number added to this spec
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-22 10:28 UTC - Claude Code (Claude Fable 5.2) - Drafted the EPIC from the inventory of the tree and the maintainer's decisions of 2026-11-22: an EPIC with two sub-issues, request-URI logging in scope (redact in both servers), standalone and linked to EPIC #2912, database text for clients decided in the SI-B ADR. Not approved; no GitHub issue created.
- 2026-11-22 13:24 UTC - Claude Code (Claude Fable 5.2) - The maintainer approved the specification. Created EPIC #2993 as `josecelano-bot` (labels `EPIC`, `Security`) and moved the specification to `docs/issues/open/`. GitHub sub-issue links wait for the maintainer (the bot account cannot add them).

## Maintainer Decisions

Recorded 2026-11-22 in the session that drafted this specification:

1. The work is an EPIC with two sub-issues (SI-A secrets, SI-B client text), because the first is a bug class and the second a design question that needs an ADR.
2. Request-URI logging is in scope: redact the peer key and the `token` query parameter in both the HTTP tracker and the REST API.
3. The EPIC is standalone, linked from EPIC #2912 and SI-4, not a sub-issue of it.
4. What a client receives for database failures is decided in the SI-B ADR, not fixed in this specification. The user and host in database server messages (A11) are an operator-side exposure and a client-text question, so SI-B owns them. A database server message that holds a secret (A12) is a secret leak, so SI-A owns it: SI-A makes the retained error, the reply and the log free of the secret, and SI-B then decides what non-secret detail a client may see.

5. Principle for client text about a secret: help the legitimate user as much as possible without revealing the secret. The project is public, so an attacker can read the source, and security must not rely on hiding how the tracker works. A message may therefore state any fact about the client's own input (its length, that it holds a character outside the allowed set) and the state of the credential (not registered, expired), but never the secret itself. SI-B applies the same principle to the other error families.

### Proposed messages for the peer key (to be confirmed in the SI-A specification)

The key can be wrong for four different reasons, and each gets its own message. None of them needs the key: the client already has what it sent.

| Case | Today | Proposed | What it tells an attacker |
| --- | --- | --- | --- |
| No key in the URL | `Missing authentication key` | `Missing authentication key: this tracker is private, use /announce/<key>` | nothing new: the tracker is private |
| Malformed, wrong length | `Invalid format for authentication key param: expected 32 alphanumeric characters` (already without the key) | `Invalid authentication key: expected 32 characters, got <n>` | the length of the client's own input, and the format, which is public in the source and the documentation |
| Malformed, wrong characters | the same text as above | `Invalid authentication key: only the characters 0-9, a-z and A-Z are allowed` (no position and no character) | the format, which is public |
| Well formed, not registered | `Failed to read key: <key>` | `Authentication key not registered` (or `Unknown authentication key`) | that this guess failed, which any reply reveals |
| Registered, expired | `Key has expired` | `Authentication key expired` (the expiry time may be added; SI-A decides) | that the key existed. It no longer works, and guessing a key by trial is not practical: 62^32 values |

The REST API messages for an uploaded key (A2) follow the same pattern: the length or the character rule, never the key.

## Acceptance Criteria

- [ ] All required subissues are created and linked.
- [ ] Implementation order is explicit and justified.
- [ ] Dependencies and blockers are documented and current.
- [ ] Epic status reflects actual state of linked subissues.
- [ ] Every completed subissue includes automated verification evidence.
- [ ] Every completed subissue includes manual verification evidence.
- [ ] Every completed subissue includes post-implementation acceptance criteria review.
- [ ] Every completed subissue includes an implementation completion review.
- [ ] Documentation and governance updates are included when required.
- [ ] No error value keeps a peer key or an admin token, and no message prints one (A1, A2, A6, A10, A12).
- [ ] No log line written for a real request on the HTTP tracker or the REST API holds a peer key or an admin token, shown on a running tracker (A3, A4, A5).
- [ ] The `handle-secrets` skill, and an ADR, state how the peer key is handled (A7), and `docs/containers.md` no longer teaches a pattern that logs a secret (A9).
- [ ] An accepted ADR defines the client text policy, and the replies of the HTTP tracker, the UDP tracker and the REST API follow it, shown on a running tracker.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| --- | --- | --- |
| AC1 | TODO | {issue/spec/PR links} |
| AC2 | TODO | {issue/spec/PR links} |
| AC3 | TODO | {issue/spec/PR links} |
| AC4 | TODO | {issue/spec/PR links} |
| AC5 | TODO | {issue/spec/PR links} |
| AC6 | TODO | {issue/spec/PR links} |
| AC7 | TODO | {issue/spec/PR links} |
| AC8 | TODO | {issue/spec/PR links} |
| AC9 | TODO | {issue/spec/PR links} |
| AC10 | TODO | {issue/spec/PR links} |
| AC11 | TODO | {issue/spec/PR links} |
| AC12 | TODO | {issue/spec/PR links} |
| AC13 | TODO | {issue/spec/PR links} |

## Risks and Trade-offs

- **Redacting the URI loses debugging value.** An operator can no longer see which key a client used. Mitigation: log a stable short fingerprint if the maintainer wants correlation; SI-A decides.
- **Hiding database detail from clients hides it from administrators.** The REST admin loses the driver text. Mitigation: the detail goes to a `tracing` event on the server, and the reply may carry a request id.
- **Changing client text breaks clients that match on it.** Today's text is unstable and unspecified, but a client may still match it. Mitigation: each sub-issue lists old and new texts for approval, and the ADR says whether any text is a contract.
- **Conflict with EPIC #2912.** Both touch tracker-core error types. SI-4 has merged; SI-5 and SI-6 touch other packages and the `LocatedError` dependency. Mitigation: re-check the inventory lines against the tree before the first change.
- **Scope growth.** Mitigation: the Out of Scope list; API v2 status codes stay in EPIC #144.

## References

- Related issues: #2912 (EPIC, source locations), #2916 (SI-4, where the finding arose), #384 (original client-exposure report, SI-7 of #2912), #2079 (adopt `secrecy`, closed), #144 (API v2)
- Related PRs: #2991 (SI-4)
- Related ADRs: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`, `docs/adrs/20261118045443_errors_do_not_record_source_locations.md`
- Checked for overlap and found none: #2387 (lifecycle logs) keeps request logs out of its scope and points to #1166, which is an open discussion about `tracing` levels (for example ERROR for a wrong UDP connection id), not about what a log line may contain; #2411 (spam and abuse resistance) does not cover log redaction; EPIC #144 defines the error body and status codes of API v2 only, so SI-B covers v1 text and the two trackers
- Related drafts: `docs/issues/drafts/generalize-error-events/EPIC.md`
- Skills: `.claude/skills/handle-secrets/SKILL.md`, `.claude/skills/handle-errors-in-code/SKILL.md`
