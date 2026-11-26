---
semantic-links:
  skill-links:
    - create-adr
    - handle-errors-in-code
  related-artifacts:
    - .claude/skills/create-adr/SKILL.md
    - .claude/skills/handle-errors-in-code/SKILL.md
    - docs/adrs/20261118045443_errors_do_not_record_source_locations.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
---

<!-- skill-link: create-adr -->

# Tell clients what they can act on and keep failure detail on the server

## Scope

Repository-level. The decision sets what four servers send a client when a request fails: the
HTTP tracker (`http-protocol`, `http-core`, `axum-http-server`), the UDP tracker (`udp-protocol`,
`udp-core`, `udp-server`), the REST API (`rest-api-protocol`, `rest-api-runtime-adapter`,
`axum-rest-api-server`) and the health-check API (`axum-health-check-api-server` and the probes in
`axum-http-server`, `axum-rest-api-server`, `udp-server` and `tracker-client`). It also amends rule
4 of a root ADR. The placement ADR (`20260830124000_place_adrs_by_decision_scope.md`) requires
root placement for protocol behaviour across packages.

## Description

Rule 4 of the errors ADR (`20261118045443_errors_do_not_record_source_locations.md`) says that
client text is the `Display` of the workspace error and that the text of a source error from a
crate the workspace does not control is not formatted into it. It does not say how much a
workspace `Display` may reveal, and the code breaks the second half of the rule today. The EPIC
inventory (EPIC #2993, section B) and the recheck in the SI-B specification (issue #3000) found, on a running tracker:

- **Database driver text.** `databases::error::Error` keeps the `sqlx` error as its source and
  prints it, and every protocol forwards that `Display`. An HTTP announce can answer
  `Tracker announce error: Database error: The Sqlite3 query was malformed: error returned from
  database: ...`, and before #2999 the REST API answered a duplicate key with a 500 that held
  `UNIQUE constraint failed: keys.key` (since #2999 it holds `The <driver> database already holds
  this authentication key`, still a 500 for the client's own mistake). A database server message can name the database user and
  the client address (`Access denied for user 'db_user'@'172.17.0.1'`, EPIC item A11). The same
  conversion labels every error raised by the server as `InvalidQuery`, so an authentication
  failure reads `The MySQL query was malformed`.
- **The Rust error tree.** Each layer adds its own prefix, so a UDP client reads
  `tracker announce error: Tracker core whitelist error: The torrent: <info-hash>, is not
  whitelisted`, and the HTTP tracker sends `Tracker whitelist error: ...` with the same inner text.
- **Rust type names.** A short UDP packet answers `error parsing request: SendableRequestParseError {
  message: "Couldn't parse action", opt_connection_id: None, opt_transaction_id: None }`, the
  `Debug` of a struct. The REST API wraps every 500 in `Unhandled rejection: Err { reason: "..." }`,
  the `Debug` of `ActionStatus`.
- **Server timing values.** A wrong UDP connection id answers `cookie value is expired: <f64>,
  expected > 1795353220.6572893`: the decoded cookie and the server clock.
- **Library text.** The REST API returns axum and serde text for a malformed JSON body or query
  (`Failed to parse the request body as JSON: key must be a string at line 1 column 2`), and the
  health-check API reports a failed probe with the `reqwest` error, the UDP client error (its
  `Display`, or its `Debug` when the client cannot be built) or the Tokio join error.
- **No detail on the server.** The REST API logs a 500 with its status and request id but not the
  reason, so an operator cannot tell why a request failed.

The maintainer's principle (2026-11-22): help the legitimate user as much as possible without
revealing a secret. The project is public, so the policy does not rely on hiding how the tracker
works; it removes what a client cannot act on and what changes with the implementation. Secrets
in errors and logs were removed by SI-A (#2999); this ADR is about everything else a client reads.

## Agreement

### 1. Three classes of failure

Every failure a client can receive belongs to one class, and the class decides what the text may
say:

| Class | Examples | The text says | The text may hold |
| --- | --- | --- | --- |
| **Bad request**: the client's own input is wrong | a missing or malformed parameter, a short UDP packet, a negative `left`, a malformed JSON body, an authentication key of the wrong length, a key that already exists | what is wrong and what is expected | the parameter or field name, the expected form, and the client's own value within the limits of rule 5 |
| **Refused**: the request is well formed and the tracker's policy refuses it | a torrent that is not whitelisted, a missing, unknown or expired key, a missing or wrong admin token, a disabled capability, an expired connection id | what was refused and what the client can do about it | the client's own non-secret identifiers (the info-hash it sent) |
| **Internal failure**: the request is valid and the tracker could not complete it | a database that is down or rejects a statement, a probe job that panicked, a state the code did not expect | which operation failed, that it is the tracker's fault, and a reference the operator can find (rule 7) | the operation name and the request id; nothing from the error's source |

A **known cause keeps a specific message.** When the code can tell the cause, and the cause is the
client's, the failure is a bad request or a refusal, not an internal failure, even when a lower
layer reports it as an error: a duplicate authentication key is the client's mistake and says so.

### 2. Internal failures send a fixed text with the operation and the request id

An internal failure sends one fixed sentence per operation, of the form:

```text
Internal tracker error: the <operation> could not be completed, try again later (request id: <id>)
```

The REST API v1 already names the operation in its reason (`failed to add key: ...`), so there the
reason is `failed to <operation>: internal tracker error (request id: <id>)` (rule 10).

It never holds the text of the error or of its source, the database driver name, a database user,
host or code, a file path or a library message. The words "try again later" are there because a
database outage is the usual cause and is transient; the text does not distinguish a database
from any other internal failure, since the client can do nothing different about either.

Options weighed:

1. A fixed generic text (`internal error`). Rejected alone: the user cannot report anything the
   operator can find.
2. The failed operation without driver text (`could not read the database`). Rejected: naming the
   component adds nothing the client can act on, and ties the text to the implementation.
3. **The operation plus a request id (chosen).** The HTTP tracker and the REST API already set
   `x-request-id` on every reply and attempt to record it in the request span (rule 7 says what is
   missing); the UDP server already makes a
   `request_id` for every request and logs it. Putting the id in the text lets a BitTorrent user,
   who never sees response headers, quote it to the operator.

### 3. One sentence, not the error tree

Client text is one sentence, or a short prefix naming the request and one reason
(`Invalid announce request: missing param info_hash`). It does not mirror the Rust nesting
(`tracker announce error: Tracker core whitelist error: ...`). The protocol layer builds the text
from the error's variant at the point where it builds the reply (`error_mapping.rs` for the HTTP
tracker, the `Error` to `ErrorResponse` conversion of the UDP server, the REST response functions,
the health-check probes), not from the `Display` chain of the outer error.

### 4. The texts are not a contract

No error text of the four servers is a contract that clients may match. Clients must rely on the
signal each protocol defines: the presence of `failure reason` (HTTP tracker), action 3 (UDP
tracker), the status code (REST API and health-check API) and the `status` field of a health-check
report. Any text may change in any release. The tracker keeps no changelog, so the pull request
that changes a text lists the old and the new one in its description (the SI-B specification does
for this change), and the notes of the GitHub release summarize them. Tests pin the texts to keep this policy, not to promise them.

### 5. Echoing the client's input

Text may repeat a value the client sent when it helps the client find the mistake, with these
limits:

- Never a secret, even a malformed one (the `handle-secrets` skill and the SI-A amendment of the
  secrecy ADR): the text names the parameter and the expected form instead.
- At most 64 characters of the value, counted on the value as the client sent it, before escaping;
  a longer value is cut and ends with `...`. 64 is longer than
  any well-formed tracker value (a hex info-hash is 40), so a value that needs cutting is already
  wrong.
- Control characters are escaped after the cut (Rust `char::escape_debug`, which keeps printable
  Unicode as sent and escapes control characters, `'`, `"` and `\`), so the text stays one line in
  a log, a terminal or a client's status bar.

### 6. The UDP connection-id error

Every connection cookie error (expired, from the future, not a normal number) answers the same
text, which tells the client what BEP 15 expects it to do:

```text
Invalid or expired connection id: send a new connect request
```

The decoded cookie value and the server's time bounds stay in the server log and the error event.

### 7. The detail goes to a `tracing` event at the failure

Every internal failure emits one `tracing` event at `ERROR` at the place where the reply is built,
with the full `Display` of the error (its source included), the operation and the request id. For
the HTTP tracker, the REST API and the health-check API the event runs inside the request span, and
the span must declare the field: `RedactedMakeSpan` (`packages/axum-server/src/request_log.rs`)
declares only `method`, `uri` and `version` today, so the `span.record("request_id", ...)` calls of
the three servers record nothing, and the implementation adds `request_id = tracing::field::Empty`
to it. The UDP server declares and records its own `request_id` in the handler span and adds it to
the reply. An operator who
gets a request id from a user finds the event by it. Non-secret driver text belongs in this event;
a secret never does (SI-A).

Bad requests and refusals keep their current logging. Their log levels are issue #1166.

### 8. Event payloads

An event payload stays on the server, so it follows the rules of a log, not of a reply: it may
carry the error's detail (the UDP `ErrorKind` strings, including `Database(String)`), and never a
secret. The events ADR (`20260727000000_events_are_objective_facts.md`) applies: the payload states
what happened, whatever the client was told. So the UDP error event keeps its texts; only the reply
changes. The general error-event contract is the draft `docs/issues/drafts/generalize-error-events/`.

### 9. The health-check API

The health-check API is reached by orchestrators and operators. Its default bind address is
`127.0.0.1:1313`, but the no-persistence container configuration and the e2e container
configuration bind it to `0.0.0.0:1313`, and an operator can bind it publicly, so the policy treats
it as reachable by anyone. Its statuses stay: `200`
with the report `health check failed` when a probe fails, and `503` with `shutting down` while the
application stops. Each failed probe reports a fixed reason of its own, never library text:

| Probe outcome | Result text |
| --- | --- |
| HTTP service answers a non-success status | `the service answered <status>` (for example `the service answered 500 Internal Server Error`) |
| HTTP service cannot be reached | `the service did not answer` |
| UDP service does not answer within the timeout | `the service did not answer within 2 seconds` |
| UDP service answers something other than a connect response | `the service did not answer with a connect response` |
| UDP reply cannot be received or parsed | `the service answered with an invalid response` |
| UDP probe cannot open its socket | `the probe could not open a UDP socket` |
| The probe job panics or is cancelled | `the health check job failed` |

The report keeps the service bindings and URLs it shows today; they identify the probed service and
are already the operator's configuration. The library text goes to a `tracing` event (rule 7).

The result text of a failed probe carries no request id, because the report keeps its shape. The
reference for an internal failure (the panicked or cancelled job) is the `x-request-id` header that
the health-check API sets on every reply, as the HTTP tracker and the REST API do. The `tracing`
event of rule 7 is emitted inside the request span, which declares the `request_id` field (rule 7),
so it carries the same id, and it records the service binding
that the report shows for the failed service. An operator relates the report to the event by that
id or by that binding.

### 10. REST API v1 and v2

API v1 is additive-only, so this ADR changes v1 **text** and keeps v1 **statuses and body shapes**:
the plain-text `Unhandled rejection: Err { reason: "<reason>" }` frame of a v1 500 stays, and only
the reason changes. The frame is the v1 body, and replacing it is the structured error body of API
v2 (EPIC #144).

**The duplicate authentication key keeps its 500 in v1.** Rule 1 makes it a bad request, and its
text becomes `failed to add key: the tracker already holds this authentication key`, but a v1
status does not change: answering `409 Conflict` instead would break clients that handle the 500,
and the correct statuses are the purpose of API v2 (EPIC #144). So in v1 a bad request can still
arrive as a 500, a known limitation that the v2 error model removes.

The v2 error model must follow the three classes of rule 1 (bad request `400`, `409` or another
`4xx`, refused `401`, `403`, `404` or `409`, internal failure `5xx`; a duplicate key is a `409`)
and carry the request id in its structured body, so
v1 text and v2 bodies tell a client the same thing.

### 11. The database error labels

A conversion from `sqlx` labels an error by what happened, not by a guess. An error raised by the
database server (`sqlx::Error::Database`) becomes a variant whose message says the server returned
an error (for example `The MySQL database server returned an error: <source>`), not
`InvalidQuery`, and the same holds in `from_statement_that_binds_a_secret`. These messages are
server-side detail (rule 7): no client sees them after this ADR.

### Amendment of rule 4 of the errors ADR

Rule 4 of `20261118045443_errors_do_not_record_source_locations.md` becomes: client text is chosen
by the protocol layer from the error's class and variant, as this ADR sets out. It may use the
`Display` of a workspace error only when that `Display` holds nothing from a source error and
follows rules 3 and 5 of this ADR. The rest of rule 4 (the text of a crate the workspace does not
control is not formatted into client text) is unchanged and now also covers the database driver,
`serde`, `axum`, `reqwest` and the standard library's I/O errors.

### Current and proposed texts

The texts at the time of this decision, from `develop` `ebe9e42b9`. `<id>` is the request id,
`<ih>` an info-hash, `<n>` a number. The SI-B specification copies the approved rows before the
code changes (its acceptance criterion AC5).

HTTP tracker (`failure reason`, always HTTP 200):

| Case | Class | Current text | Proposed text |
| --- | --- | --- | --- |
| Missing key | Refused | `Tracker authentication error: Missing authentication key: this tracker is private, use /announce/<key>` | `Missing authentication key: this tracker is private, use /announce/<key>` |
| Key of the wrong length | Bad request | `Tracker authentication error: Invalid authentication key: expected 32 characters, got <n>` | `Invalid authentication key: expected 32 characters, got <n>` |
| Key with a wrong character | Bad request | `Tracker authentication error: Invalid authentication key: only the characters 0-9, a-z and A-Z are allowed` | `Invalid authentication key: only the characters 0-9, a-z and A-Z are allowed` |
| Key not registered / expired | Refused | `Tracker authentication error: Authentication key not registered` / `... expired` | `Authentication key not registered` / `Authentication key expired` |
| Key check fails (latent, nothing builds it) | Internal | `Tracker authentication error: Key could not be verified: <sqlx text>` | `Internal tracker error: the authentication could not be completed, try again later (request id: <id>)` |
| Torrent not whitelisted | Refused | `Tracker whitelist error: The torrent: <ih>, is not whitelisted` | `The torrent <ih> is not whitelisted on this tracker` |
| Database failure on announce | Internal | `Tracker announce error: Database error: <driver text>` | `Internal tracker error: the announce could not be completed, try again later (request id: <id>)` |
| Database failure on scrape | Internal | `Tracker scrape error: Database error: <driver text>` | `Internal tracker error: the scrape could not be completed, try again later (request id: <id>)` |
| Any other tracker-core error | Internal | `Tracker error: <Display>` | `Internal tracker error: the request could not be completed, try again later (request id: <id>)` |
| Announce query parse | Bad request | `Bad request. Cannot parse query params for announce request: <reason>` | `Invalid announce request: <reason>` |
| Scrape query parse | Bad request | `Bad request. Cannot parse query params for scrape request: <reason>` | `Invalid scrape request: <reason>` |
| Malformed query string | Bad request | `Bad request. Cannot parse query params: invalid param <raw param>` | `Invalid request: malformed query param <raw param>` (rule 5) |
| Per-parameter reasons (`missing param <name>`, `invalid param value <v> for <name>[: it must percent-decode to exactly 20 bytes]`, `malformed percent encoding or invalid UTF-8 for ip`, the scrape `missing query params ...`) | Bad request | as listed | unchanged, with `<v>` cut and escaped by rule 5 |
| `ip` parameter (`Client-supplied peer IPs are disabled`, `DNS names are not supported for the announce ip parameter`, `The announce ip parameter must be an IPv4 or IPv6 literal`) | Refused / Bad request | as listed | unchanged |
| `X-Forwarded-For` missing behind a reverse proxy | Bad request | `Error resolving peer IP: missing or invalid right-most IP in the X-Forwarded-For header, which is mandatory when the tracker runs behind a reverse proxy` | unchanged |
| Socket address missing | Internal | `Error resolving peer IP: cannot get the client IP: the server did not provide the socket address of the connection` | `Internal tracker error: the request could not be completed, try again later (request id: <id>)` |
| Idle connection, unknown path | - | 408 with an empty body; axum's default 404 | unchanged |

UDP tracker (error packet message; the event column is the `ErrorKind` payload):

| Case | Class | Current reply | Proposed reply | Event text |
| --- | --- | --- | --- | --- |
| Request parse | Bad request | `error parsing request: SendableRequestParseError { message: "<msg>", opt_connection_id: ..., opt_transaction_id: ... }` | `Invalid request: <msg>` (the fixed `udp-protocol` messages, for example `Couldn't parse action`) | unchanged (`SendableRequestParseError: message: ...`) |
| Packet too short (an I/O read error) | Bad request | the same struct with the `std::io` text (`failed to fill whole buffer`) | `Invalid request: the packet is too short` | unchanged |
| Connection cookie (expired, from the future, not normal), announce and scrape | Refused | `tracker announce error: Connection cookie error: cookie value is expired: <f64>, expected > <f64>` (and the two other forms, and `tracker scrape error: ...`) | `Invalid or expired connection id: send a new connect request` | unchanged (values stay in the event) |
| Torrent not whitelisted, announce and scrape | Refused | `tracker announce error: Tracker core whitelist error: The torrent: <ih>, is not whitelisted` | `The torrent <ih> is not whitelisted on this tracker` | unchanged |
| Negative byte count | Bad request | ``tracker announce error: Invalid announce request: `left` must not be negative, got <n>`` | ``Invalid announce request: `left` must not be negative, got <n>`` | unchanged |
| Database failure, announce and scrape | Internal | `tracker announce error: Tracker core announce error: Database error: <driver text>` | `Internal tracker error: the announce could not be completed, try again later (request id: <id>)` (and `scrape`) | unchanged (driver text stays in `Database(String)`) |
| `Internal` (nothing builds it in production) | Internal | `internal server error: <message>` | `Internal tracker error: the request could not be completed, try again later (request id: <id>)` | unchanged |
| `AuthRequired` (nothing builds it in production) | Refused | `domain tracker requires authentication but is not supported in current UDP implementation` | unchanged | unchanged |
| Banned IP | - | no reply | unchanged | unchanged |

REST API (v1 statuses and body shapes unchanged, rule 10):

| Case | Class | Status | Current text | Proposed text |
| --- | --- | --- | --- | --- |
| No token | Refused | 500 | `Unhandled rejection: Err { reason: "unauthorized" }` | `Unhandled rejection: Err { reason: "unauthorized: send the header Authorization: Bearer <token>" }` |
| Wrong token | Refused | 500 | `... reason: "token not valid" }` | `... reason: "unauthorized: the token is not valid" }` |
| Malformed `Authorization` header | Bad request | 500 | `... reason: "unknown token provided" }` | `... reason: "unauthorized: the Authorization header must be Bearer <token>" }` |
| Key add, generate, delete or reload fails in the database | Internal | 500 | `... reason: "failed to <op>: <driver text>" }` | `... reason: "failed to <op>: internal tracker error (request id: <id>)" }` |
| Duplicate key | Bad request | 500 (rule 10) | `... reason: "failed to add key: The <driver> database already holds this authentication key" }` | `... reason: "failed to add key: the tracker already holds this authentication key" }` |
| Whitelist add, remove or reload fails in the database | Internal | 500 | `... reason: "failed to <op>: <driver text>" }` | `... reason: "failed to <op>: internal tracker error (request id: <id>)" }` |
| Malformed key | Bad request | 400 | `Invalid URL: invalid auth key: <reason>` and `Invalid auth key id param: <reason>` (SI-A) | unchanged |
| Key duration not a number | Bad request | 400 | `Invalid URL: invalid auth key duration: expected a number of seconds` | unchanged |
| Key duration overflows | Bad request | 400 | `Invalid URL: invalid auth key duration: "<n>"` | `Invalid URL: invalid auth key duration: <n> seconds is too long` |
| Malformed info-hash | Bad request | 400 | `Invalid URL: invalid infohash param: string "<v>", expected a 40 character long string` | unchanged, `<v>` cut and escaped by rule 5 |
| Malformed JSON body | Bad request | 400 | `Failed to parse the request body as JSON: <serde text>` | `Invalid request body: expected a JSON object with the fields key (string or null) and seconds_valid (number or null)` |
| JSON body of the wrong type | Bad request | 422 | `Failed to deserialize the JSON body into the target type: <field>: <serde text>` | `Invalid request body: invalid value for the field <field>` |
| Missing JSON content type | Bad request | 415 | ``Expected request with `Content-Type: application/json` `` (axum) | `Invalid request: the body must be sent with Content-Type: application/json` |
| Malformed query parameter | Bad request | 400 | `Failed to deserialize query string: <param>: <serde text>` | `Invalid query parameter <param>: expected <form>` (for example `a non-negative integer`) |
| Capability disabled | Refused | 409 | `{"status":"err","reason":"<capability> capability is disabled by configuration"}` | unchanged |
| Unknown torrent | - | 200 | `"torrent not known"` | unchanged (a v1 status and shape) |
| Idle connection, unknown path | - | 408, 404 | empty body | unchanged |

Health-check API: the report messages `health check failed` (200) and `shutting down` (503) are
unchanged; the per-service results are in rule 9. The current results are `Err(<reqwest error or
status text>)` for an HTTP service, `Err(Timed Out)`, `Err(Did not Connect)`, `Err(<receive error
Display>)` or `Err(<client construction error Debug>)` for a UDP service, and `the health check job
failed: <join error>`.

### Alternatives Considered

- **Keep the `Display` chain and only strip source errors.** It would remove the driver text but
  keep the nesting and the type names, and every new layer would add a prefix to client text.
- **Allow more detail for the authenticated REST API.** The administrator is trusted and reads the
  driver text in a reply today. Rejected: the administrator can read the server log, which holds
  the detail with the request id (rule 7), and one policy for all servers is simpler to test and
  review. Its risk (hiding detail from the administrator) is answered by rule 7.
- **Make some texts a contract** (for example the not-whitelisted reply). Rejected: no client of
  the four servers is known to match a text, each protocol has its own machine-readable signal, and
  a contract would freeze wording this ADR is changing.

### Consequences

- Positive: no client reads driver text, a database user or host, a Rust type name, an error tree
  or the server clock, and a user who reports an internal failure can quote an id the operator can
  find in the log.
- Positive: client text stops changing when an error type moves or a dependency updates its
  messages.
- Positive: the REST API logs why a request failed.
- Negative: an administrator who read the database error in a REST reply now reads it in the log.
- Negative: the texts change for every client, and tests that pin them change with them
  (`failure_reasons_hold_no_source_path.rs`, the UDP `contract.rs`, the REST `asserts.rs`, the
  health-check `contract.rs`).
- The rule-4 amendment above is recorded in the errors ADR as well.

## Affected Code

- HTTP tracker: `packages/http-core/src/services/error_mapping.rs`,
  `packages/http-protocol/src/v1/responses/error.rs`, `packages/http-protocol/src/v1/requests/`.
- UDP tracker: `packages/udp-server/src/error.rs`, `packages/udp-server/src/handlers/error.rs`,
  `packages/udp-core/src/connection_cookie.rs`.
- REST API: `packages/rest-api-runtime-adapter/src/v1/adapters/`,
  `packages/axum-rest-api-server/src/v1/responses.rs`, `.../v1/middlewares/auth.rs`,
  `.../v1/context/*/responses.rs`.
- Health-check API: `packages/axum-health-check-api-server/src/handlers.rs`, the `check_fn` of
  `axum-http-server` and `axum-rest-api-server`, `packages/tracker-client/src/udp/client.rs`
  (`check`).
- Database labels: `packages/tracker-core/src/databases/error.rs`.

## Date

2026-11-25

## References

- Issue [#3000](https://github.com/torrust/torrust-tracker/issues/3000) (SI-B) and its
  specification, found by the issue number under `docs/issues/`.
- EPIC [#2993](https://github.com/torrust/torrust-tracker/issues/2993) and its inventory and
  `manual-verification-evidence.md`, found by the issue number under `docs/issues/`.
- Issue [#2999](https://github.com/torrust/torrust-tracker/issues/2999) (SI-A), which removed
  secrets from errors, logs and replies, and its amendment of
  `20260822094338_adopt_secrecy_for_sensitive_values.md`.
- `20261118045443_errors_do_not_record_source_locations.md` (rule 4, amended here).
- `20260727000000_events_are_objective_facts.md`.
- EPIC [#144](https://github.com/torrust/torrust-tracker/issues/144), API v2.
- Issue [#384](https://github.com/torrust/torrust-tracker/issues/384), the 2023 report about what
  error replies reveal; issue [#1166](https://github.com/torrust/torrust-tracker/issues/1166), log
  levels for client errors.
- [BEP 15](https://www.bittorrent.org/beps/bep_0015.html), UDP tracker protocol (connection ids).
