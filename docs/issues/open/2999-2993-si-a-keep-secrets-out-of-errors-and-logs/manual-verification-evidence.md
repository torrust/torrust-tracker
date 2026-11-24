---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md
last-updated-utc: "2026-11-24 16:41"
---

# Manual Verification Evidence

<!-- cspell:ignore nnounce -->

## Purpose

Record the baseline reproduction of the secret leaks owned by SI-A (#2999), the test plan, the
red and green runs of each test increment with its test-design review, and the final recheck on
the finished build. Every output below comes from a command actually run.

## Preserving Verification Artifacts

- [`evidence/baseline-output.txt`](evidence/baseline-output.txt): the complete output of the EPIC
  script [`reproduce.sh`](../2993-secrets-and-client-error-text/evidence/reproduce.sh) on the
  starting commit. One edit: the scratch work directory is shown as `<work-dir>`. The script
  replaces the generated key of V4 by `<the generated key>` itself.

All keys, tokens and passwords are the test values of the EPIC script (`TestPeerKey...`,
`TestAdminToken0001`, `TestDbPassword01`). The database containers were removed by the script.

## Environment and Prerequisites

- Date and time (UTC) of the baseline: 2026-11-24 16:25 to 16:26.
- Artifact under test (baseline): `target/debug/torrust-tracker` built with
  `cargo build --bin torrust-tracker` from `develop` at `b81e4512e` (the starting commit, after
  #3001 merged), `rustc 1.100.0 (0ef1bc386 2026-11-09)`.
- Operating system / environment: Linux 7.0.0-44-generic; Docker 28.3.3; images `mysql:8.0` and
  `postgres:16`; `curl`.
- Command: `bash docs/issues/open/2993-secrets-and-client-error-text/evidence/reproduce.sh
  <abs path>/target/debug/torrust-tracker <scratch work dir>`.

## T1 - Baseline Reproduction

Outcome: **Reproduced**, for every SI-A case the script covers. Nothing was fixed on `develop`
since the EPIC recheck. The replies and log lines below are copied from
`evidence/baseline-output.txt`.

- A1, peer key not registered (HTTP announce):
  `Tracker authentication error: Failed to read key: TestPeerKey000000000000000000001`.
- A1, malformed key (14 characters): the reply does not echo it but does not say what is wrong:
  `Invalid format for authentication key param: expected 32 alphanumeric characters`.
- A2, REST add with a 14-character key:
  `Invalid URL: invalid auth key: string "ShortTestKey01", Invalid key length. Key must be have 32 chars [400]`.
- A2, REST delete with a malformed key: `Invalid auth key id param "BadTestKey" [400]`.
- A3/A4, request log `uri=` fields holding a key or the query token:

  ```text
  uri=/announce/ShortTestKey01?info_hash=...&peer_id=-qB00000000000000001&port=17548&left=0
  uri=/announce/TestPeerKey000000000000000000001?info_hash=...
  uri=/api/v1/key/BadTestKey
  uri=/api/v1/stats?token=WrongTestToken0001
  ```

  The span (`request{method=GET uri=/api/v1/stats?token=WrongTestToken0001 version=HTTP/1.1}`)
  repeats the URI on every line written inside the request.
- A5, generated key: 1 `Generated key` line, holding the key.
- A10, configuration errors:
  ``invalid type: found signed int `987654321`, expected a string for key "default.core.database.password"``
  and the same for `424242` under `default.http_api.access_tokens.admin`.
- A12, duplicate key on MySQL:
  `Duplicate entry 'TestPeerKey000000000000000000003' for key 'keys.key'` in the REST reply (500).
  The log held 0 lines with the key (SI-4 removed the DEBUG line). SQLite and PostgreSQL replies
  hold no key.
- A6, A7, A8 are confirmed by reading the code, as in the EPIC evidence: `ClientError` wraps the
  `reqwest::Error` with its URL (`packages/rest-api-client/src/v1/client.rs`), `Key` derives
  `Debug`, and `ConnectionInfo::api_token` is a plain `Option<String>`.

Not covered by this run: V6 (UDP) did not run because the script resolves `udp_probe.py` relative
to the work directory after `cd`; V6 is SI-B's (no secret), so it was not repeated. The expired-key
reply was not requested (it needs a key past its lifetime); the HTTP test suite covers it.

## T2 - Test Plan

Written before the first test. Each area is done red, then green, then a recorded test-design
review (prose-first Arrange-Act-Assert), then a commit.

T3, key errors and messages:

- `ParseKeyError` names the received length (`expected 32 characters, got 14`) and the allowed
  characters, and never the key.
- `authentication::key::Error::UnableToReadKey` prints `Authentication key not registered`;
  neither `Display` nor `Debug` holds the key (the variant has no field).
- `PeerKeyError::InvalidKey` keeps the parse error and no key; `Display` and `Debug` hold no key.
- The HTTP extractor turns a 14-character key, a key with a `-` and a missing key into three
  different failure reasons, none holding the key.
- REST: `POST /api/v1/keys` with a short key answers 400 with the length and without the key;
  `DELETE /api/v1/key/<bad>` answers 400 without the key; `POST /api/v1/key/<32-char key>`
  answers 400 without the key.
- The existing tests that pin today's texts change with the messages.

T4, the `Key` type:

- `format!("{:?}", key)` is `Key([REDACTED])` and holds no key text; the same for `PeerKey`.
- `AddKeyRequest`, `AddKeyForm` and the REST `AuthKey` resource (fields typed `String`) get a
  redacting `Debug`.

T5, request logging:

- A unit test of the shared redaction function over the URI shapes: `/announce/<key>?...`,
  `/scrape/<key>`, `/api/v1/key/<key>?token=<t>`, a bare `?token=`, a URI with no secret, a
  percent-encoded key, a key with a stray character and a key of the wrong length.
- A test of the custom `MakeSpan`: the span records the redacted URI.
- A test through each server (HTTP tracker, REST API, health check) that captures the tracing
  output with a unique key and token and asserts neither appears.
- A test that captures the debug output of key generation and finds no key.

T6, REST client:

- A transport error on a GET with a token and on a key delete holds neither the token, the key
  nor the URL query (`Display`, `Debug` and the source chain).
- A GET with a token sends it in the `Authorization` header and not in the query.

T7, configuration errors:

- Loading a configuration with a numeric database password and a numeric access token fails with
  an error that names the option path and the expected type and does not hold the number.

T7b, database errors:

- Adding the same key twice: the retained error (`Display`, `Debug`, source chain) holds no key
  on SQLite, MySQL and PostgreSQL (the shared driver test), and is the "key already exists"
  variant.
- The REST reply for that case holds no key.

## T3 - Key Errors and Messages

Toolchain: stable `rustc 1.100.0`, `cargo test`. Commit subject:
`fix(auth): [#2999] keep the peer key out of errors and replies and name what is wrong with it`.

### Red runs

- tracker-core (run before any production change of the area; nine tests failed, the full output
  is in [`evidence/t3-red-tracker-core.txt`](evidence/t3-red-tracker-core.txt)). The causal lines:

  ```text
  left: "Failed to read key: YZSl4lMZupRuOpSRC3krIKR5BPB14nrJ"
  right: "Authentication key not registered"
  left: "Invalid key length. Key must be have 32 chars"
  right: "expected 32 characters, got 14"
  the error Display holds the test secret        (UnableToReadKey, and PeerKeyError::InvalidKey)
  test result: FAILED. 55 passed; 9 failed
  ```

- HTTP extractor and REST contract tests: the production change was made in the same step as the
  test edit for these two areas (the REST adapter stopped compiling as soon as `PeerKeyError`
  lost its `key` field, so no runtime red run exists). Their tests were proven with
  mutate-then-restore instead:
  - Mutation 1: the extractor reports a wrong-length key as "only the characters ... allowed".
    Failed: `it_should_name_the_received_length_and_not_the_key_when_the_path_key_has_the_wrong_length`
    and `it_should_encode_an_invalid_key_format_failure_response_for_an_invalid_path_key`.
    (A first attempt did not compile and was discarded.)
  - Mutation 2: `generate_auth_key_handler` answers the `Path<u64>`-style reply that echoes the
    segment. Failed: `deprecated_generate_key_endpoint::should_fail_generating_a_new_auth_key_when_the_key_duration_is_invalid`.
  - Both production files were restored from a backup copy afterwards.

### Green run

`cargo test --workspace --no-fail-fast`: no failure (tracker-core 286 passed, axum-rest-api-server
169 + 60 passed, axum-http-server 53 + 77 passed).

### Test-design review (prose-first AAA)

- Arrange: the one causal difference is the shape of the key (14 characters, a dash, 32
  characters with a two-byte character, an unregistered but well-formed key). The test body keeps
  each literal visible, because the literal selects the behavior and is the secret the assertion
  looks for.
- Act: the production call stays visible (`Key::new`, `authenticate`, `add_peer_key`,
  `parse_key`, the router or `ApiHttpClient` request).
- Assert: the expected text is written out independently in each test (no production-derived
  expectation). Absence of the secret goes through one helper, `assert_holds_no_secret`, which
  checks `Display`, `Debug` and the source chain and never prints the secret.
- Smell check: one case needed a decision. The REST helper `assert_invalid_auth_key_*_param` now
  takes the expected reason from the table of the test, so the table, not the helper, owns the
  expected text. A fixture key (`invalid`) was a substring of the message ("invalid auth key"); it
  was replaced with `NotAValidKey`, so the absence check is meaningful.
- The 32-character key with a non-ASCII character guards that the length counts characters, not
  bytes (`chars().count()`).

## T4 - The `Key` Type

Toolchain: stable `rustc 1.100.0`, `cargo test`. Commit subject:
`fix(tracker-core): [#2999] redact the peer key in Debug and in the PeerKey text`.

### Red run

Ten tests failed before the production change ([`evidence/t4-red.txt`](evidence/t4-red.txt)):

```text
left: "Key(\"TestPeerKeyForDebugOutput0000001\")"
right: "Key([REDACTED])"
the Debug output holds the test secret      (PeerKey, InMemoryKeyRepository, AddKeyRequest, AddKeyForm, AuthKey)
test result: FAILED. 60 passed; 8 failed (tracker-core); 3 passed; 2 failed (rest-api-protocol)
```

### Green run

`cargo test --workspace --no-fail-fast`: no failure after the change.

### Decisions

- `Key` has a manual `Debug` (`Key([REDACTED])`). Types that derive `Debug` over a `Key` field
  (`PeerKey`, `InMemoryKeyRepository`, `AuthenticationService`) are redacted by composition; a
  test checks `PeerKey` and `InMemoryKeyRepository` (the tokio `RwLock` prints its contents).
- Fields typed `String` got a manual redacting `Debug`: `AddKeyRequest::opt_key`,
  `AddKeyForm::opt_key` and `AuthKey::key`. The other fields still print.
- `PeerKey`'s manual `Display` had no production use (found by searching for `PeerKey` text
  formatting); it now prints ``key: `[REDACTED]` `` and the expiry. Its four tests pinned the key
  text and changed with it.
- Remaining uses of `Key::to_string()`/`Display` and `Serialize`, each deliberate:
  1. `rest-api-runtime-adapter/src/v1/adapters/auth_key.rs`, `peer_key_to_auth_key`: the REST reply
     that hands a generated or added key to its owner.
  2. `tracker-core/src/databases/driver/{sqlite,mysql,postgres}/auth_key_store.rs`: the value bound
     to the `keys` table (persistence).
  3. `rest-api-client/src/v1/client.rs`: `format!("key/{key}")`, the key delete URL the caller
     asked for; the URL is stripped from client errors by T6.
  4. `tracker-client/src/http/client/mod.rs` builds the announce URL with the key; its output is
     masked by #3004 (out of scope).
  5. `Serialize` on `Key` and `PeerKey`: the database load path and REST bodies above.

### Test-design review (prose-first AAA)

- Arrange: the unique 32-character test key is visible in each test because it is both the input
  and the thing the assertion looks for. Nothing else is set up (the repository test inserts one
  key).
- Act: only the formatting call (`format!("{x:?}")`, `to_string()`); production code is the
  `Debug`/`Display` impl itself.
- Assert: `Key` is asserted against the literal `Key([REDACTED])`; composed types assert the
  absence of the secret and, where it costs nothing, that the other fields still print (so a
  blanket "print nothing" does not pass). The absence message never prints the secret.
- Smell check: `AuthenticationService` has no own test (the redaction comes from the repository it
  holds, which has one); recorded here as deliberate.

## T5 - Request Logging and the Generated Key

Toolchain: stable `rustc 1.100.0`, `cargo test`. Commit subject:
`fix(servers): [#2999] keep peer keys and admin tokens out of the request log`.

### Design

- One tested function owns the rule: `torrust_tracker_axum_server::request_log::redact_uri`
  (`packages/axum-server/src/request_log.rs`). The three servers already depend on the package, so
  no new package is needed. It redacts everything after a path segment `announce`, `scrape` or
  `key` (whatever its shape) and the value of a `token` query parameter (any case), with `***`.
  A custom `RedactedMakeSpan` records the redacted URI in the span; it replaces `DefaultMakeSpan`
  in the HTTP tracker, the REST API and the health-check API, and their `on_request` events log
  `redact_uri(request.uri())`. The `latency_ms` field name is unchanged (decision 5).
- A URI in absolute form loses its scheme and authority in the log (only `path?query` is shown).
- The segment names are compared as written; a percent-encoded name (`/%61nnounce/<key>`) does
  not match a route, so it reaches no handler and carries no key.
- `tower-http` is pinned to `0.7` in `axum-server` to share the servers' version; `Cargo.lock`
  gained `http` and `tower-http 0.7.1` edges for the package, no new crate.
- `tracker-core::authentication::key::generate_key` no longer logs: both `Generated key` debug
  lines are gone.

### Red runs and mutations

- Through each server (one test binary per crate that captures every level,
  [`evidence/t5-red.txt`](evidence/t5-red.txt)), before the servers used the redaction:

  ```text
  axum-http-server          the log holds a test peer key of 32 characters
  axum-rest-api-server      the log holds the test admin token
  axum-health-check-api-server  the log holds the test admin token
  ```

- Generated key (A5), before removing the lines:
  `the log holds the generated key` for the expiring and the permanent case
  ([`evidence/t5-mutations.txt`](evidence/t5-mutations.txt)).
- The redaction function and the span were written together with their tests, so their guard was
  shown by mutate-then-restore ([`evidence/t5-mutations.txt`](evidence/t5-mutations.txt)): with
  the path not redacted 10 of 19 cases fail; with the query not redacted 5 of 19 fail; with the
  span recording the raw URI, or the event recording it, the HTTP tracker test fails with the
  same leak message. Mutated files were restored from a copy and the test run green again. (Two
  first attempts did not compile, because the unused-import lint rejected them; the successful
  ones carry an `allow(unused_imports)`.)

### Green run

`cargo test --workspace --no-fail-fast`: no failure. The three `request_logging` binaries pass.

### Test-design review (prose-first AAA)

- Arrange: the test keys and token are visible local values: they are the thing the assertion
  searches for, and each says its role (`well_formed_key`, `malformed_key`,
  `key_of_the_wrong_length`, `a_key_posted_where_the_seconds_go`). The server env is the
  crate's own `Started` environment.
- Act: the requests are the visible calls (`client.get`, `reqwest` GET/DELETE/POST); the production
  logging is exercised by the running server, not called by the test.
- Assert: the leak assertions come first, so a red run shows the leak and not a missing marker;
  the marker assertion (`uri=/announce/***?`) guards against a fix that drops the URI entirely.
  The redaction table states each full expected text independently of the function.
- Smell check: three test binaries repeat the capture pattern deliberately (each crate owns its
  server's startup); the shared pieces (`setup_capturing_every_level`, `captured_logs`,
  `capture_logs_while`) live in `torrust-tracker-test-helpers`.

## T6 - The REST Client

Toolchain: stable `rustc 1.100.0`, `cargo test`. Commit subject:
`fix(rest-api-client): [#2999] keep the token and the key out of client errors`.

### Changes

- `ClientError` wraps every `reqwest` error through `without_url`: `From<reqwest::Error>` (the
  transport errors, including the client build and the body reads), a new `deserialization`
  constructor, and the free `get` helper's send error.
- The token is sent in the `Authorization: Bearer` header only. The query injection of `get_result`
  is removed. The two documented exceptions stay: `get_request` sends no token at all, and a GET
  whose caller supplies its own `Authorization` header keeps it. The server still accepts
  `?token=`; a caller that builds the query itself (the contract tests of the query form) is
  unchanged.
- A8: `ConnectionInfo::api_token` is now `Option<SecretString>` (`secrecy` 0.10, already a
  workspace dependency). `ConnectionInfo::authenticated(origin, &str)` keeps its signature. A
  breaking change for a caller that read the field or cloned `ConnectionInfo` (a `SecretString` is
  not `Clone`, so the `Clone` derive is gone): the in-tree callers (the contract-test
  environment, `tests/`) adapted; the CI tools under `src/console/ci` build `ConnectionInfo`
  through `authenticated` and needed no change. The crate is `0.x`; mention it in the release
  notes (spec risk list).

### Red run

Six tests failed before the change ([`evidence/t6-red.txt`](evidence/t6-red.txt)), for example:

```text
the error holds the test token                (GET transport error; deserialization error)
the error holds the test key                  (key delete, plain and typed client)
the request line holds the token: GET /api/v1/stats?token=TestAdminTokenOfTheClient0001 HTTP/1.1
```

The free-helper test was tightened afterwards (the first assertion matched the peer socket address
in the connect error, which stays and is not part of the URL path or query); its guard was shown by
mutation ([`evidence/t6-mutations.txt`](evidence/t6-mutations.txt)): the free helper keeping the URL
fails it; the deserialization error keeping the URL fails the deserialization test; `From` keeping
the URL fails the free-helper test and both key-delete tests.

### Green run

`cargo test --workspace --no-fail-fast`: no failure (13 client tests).

### Test-design review (prose-first AAA)

- Arrange: the closed address (a listener that was dropped) or a one-shot local server that
  returns a chosen body and hands back the request head; the token and key constants are visible.
- Act: the real client call (`get_tracker_statistics`, `delete_auth_key`, `ApiClient::...`, the
  free `get`).
- Assert: everything a reader can see of the error (`Display`, `Debug`, each source) is searched
  for the secret by one helper, so a future `reqwest` change that moves the text between layers is
  still caught; the request head is checked for the header and for the absence of the token in
  the request line.
- Smell check: a helper with two uses (`everything_shown_by`) stays named for what it returns; the
  request-head test checks the header case-insensitively because HTTP header names are.

## T7 - Configuration Errors (A10)

Toolchain: stable `rustc 1.100.0`, `cargo test`. Commit subject:
`fix(configuration): [#2999] do not print the value of a secret option in a configuration error`.

### Decision: how a secret option is detected

By the key path of the `figment` error against a list (`SECRET_OPTIONS` in
`packages/configuration/src/secret_options.rs`), not by reading the secret fields as strings: the
secret fields keep their `SecretString` type and TOML syntax, and the list covers every source
(file, inline TOML, `TORRUST_TRACKER_CONFIG_OVERRIDE_*` variables) because `figment` reports the
same path for all of them. Entries:

- `core.database.password`, and everything below it;
- `http_api.access_tokens`, and everything below it (a map of labels to tokens);
- `core.database` for an error about the value itself only (a table written as a string such as
  `mysql://user:<password>@host/db` echoes the whole string); other errors inside the database
  table (a bad port) keep printing their value, because that value is not a secret.

Each error of a chained `figment` error is redacted; the option path, the expected type and the
source stay. The value becomes the fixed text `a value of another type (not shown: the option is
a secret)`. A new secret option must be added to the list (the `handle-secrets` skill says so, T8).
Limit: the detection does not know a secret field that is added without an entry; the test of
the module loads an invalid value for each current entry.

### Red run

Five of seven tests failed on the starting loader ([`evidence/t7-red.txt`](evidence/t7-red.txt));
for each: `the error text holds the secret value`. The non-secret control test passed.

### Green run

`cargo test -p torrust-tracker-configuration`: 168 passed. The environment-override case reports
``invalid type: found a value of another type (not shown: the option is a secret), expected a
string for key "CORE.DATABASE.PASSWORD" in `TORRUST_TRACKER_CONFIG_OVERRIDE_` environment
variable(s)``.

### Mutations (one per clause, [`evidence/t7-mutations.txt`](evidence/t7-mutations.txt))

- Access tokens redacted only when the map itself is wrong: the numeric and the boolean token cases
  fail.
- No rule for a database written as a string: the DSN case fails.
- The database rule covering everything below it: the non-secret control test fails (a bad port
  would lose its value). (A first attempt, which made the second variant unused, did not compile
  and was discarded.)

### Test-design review (prose-first AAA)

- Arrange: each case is a complete TOML text, so the reader sees the one causal difference (a
  password or token written as a number or boolean, or a table written as a string) with the secret
  value next to the option path the assertion expects.
- Act: `Configuration::load`, the production loader, inside a `figment::Jail` with a cleared
  environment.
- Assert: the secret value is absent from both `Display` and `Debug`; the option path and the word
  `expected` are present, so a fix that drops the whole message does not pass; a control test keeps
  non-secret values printed.

## T7b - Database Errors That Hold a Key (A12)

Toolchain: stable `rustc 1.100.0`, `cargo test`; MySQL and PostgreSQL through the driver tests'
testcontainers (`--features db-compatibility-tests`, Docker 28.3.3). Commit subject:
`fix(tracker-core): [#2999] keep the database server message out of key store errors`.

### Statements that bind a secret

The `keys` table: `get_key_from_keys` (`SELECT ... WHERE key = ?`), `add_key_to_keys`
(`INSERT`) and `remove_key_from_keys` (`DELETE`), in the SQLite, MySQL and PostgreSQL drivers.
`load_keys` binds nothing and is unchanged. The whitelist and torrent statements bind info hashes,
which are not secrets.

### Change

`Error::from_statement_that_binds_a_secret` (`databases/error.rs`) converts the `sqlx` error of
those statements. A server error of kind `UniqueViolation` (the driver's kind, not its text)
becomes the new `Error::KeyAlreadyExists` (`The <driver> database already holds this authentication
key`). Any other server error becomes `InvalidQuery` whose source keeps only the kind and the code.
Connection, pool and I/O errors convert as before. What non-secret driver detail a client may see
is SI-B's. The existing SQLite test that pinned `InvalidQuery` for a duplicate now expects
`KeyAlreadyExists`.

### Red run

- Unit and SQLite ([`evidence/t7b-red.txt`](evidence/t7b-red.txt)): three failed, among them
  `the error Display holds the test secret` and
  `InvalidQuery { source: Database(ServerError { message: "Duplicate entry '...' for key ..." ...`.
- MySQL (real server): with the new conversion disabled by mutation, the shared driver test fails
  with `the error Display holds the test secret` at `driver/mod.rs:317`.

### Green run

`cargo test --workspace --no-fail-fast`: no failure. With Docker:
`TORRUST_TRACKER_CORE_RUN_MYSQL_DRIVER_TEST=true TORRUST_TRACKER_CORE_RUN_POSTGRES_DRIVER_TEST=true
cargo test -p torrust-tracker-core --features db-compatibility-tests --lib -- run_mysql_driver_tests
run_postgres_driver_tests`: `run_mysql_driver_tests ... ok`, `run_postgres_driver_tests ... ok`
(the shared test checks `Display`, `Debug` and the source chain and the `KeyAlreadyExists` variant).
A REST contract test adds the same key twice and checks the 500 reply holds no key and says the
database already holds it.

### Test-design review (prose-first AAA)

- Arrange: a fake server error whose message holds the secret, with a flag for the unique-violation
  kind: the one causal difference between the two unit tests. The driver test adds one key twice.
- Act: the production conversion (`from_statement_that_binds_a_secret`) and the real
  `add_key_to_keys`.
- Assert: the secret is absent from `Display`, `Debug` and the chain; the variant and the kept kind
  and code are asserted, so a fix that drops all detail does not pass.
- Smell check: the fake is a test-local implementation of `sqlx`'s `DatabaseError` trait, needed
  because `sqlx` has no public constructor.

## T8 - Documentation

Commit subject: `docs(secrets): [#2999] teach the Authorization header and the peer-key rule`.

- The secrecy ADR (`docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`) has an
  amendment (decided in the issue spec: amend, no new ADR) for the peer key, secrets in URLs and
  library errors that print a rejected value. The `handle-secrets` skill (version 1.3) has a
  matching section and two checklist items.
- `docs/containers.md`: the three `?token=` examples use the `Authorization` header; the MySQL
  example uses the v3 network fields (`host`, `port`, `user`, `password`, `database`) instead of a
  `mysql://` `path`, and the DSN percent-encoding advice is replaced by a note that the password
  needs no encoding.
- Skills `use-rest-api`, `manual-http-download-completion-e2e`, `manual-udp-download-completion-e2e`;
  the rustdoc of `src/lib.rs`, `axum-rest-api-server` (`lib.rs`, `v1/middlewares/auth.rs`,
  `v1/context/{auth_key,stats,torrent,whitelist}`, `torrent/handlers.rs`); and
  `contrib/dev-tools/experiments/dual-stack-sockets/README.md` use the header. Where the query form
  is explained on purpose (`axum-rest-api-server/src/lib.rs`, `v1/middlewares/auth.rs`,
  `src/lib.rs`), it says the server still accepts it and the header is the safe form.
- Check after the edit: `grep -rn "token=" --include=*.md --include=*.rs .` outside `docs/issues/`,
  `docs/pr-reviews/`, `target/` and test code finds only the sentences that explain the query form
  on purpose, and tests that exercise it (`auth.rs`, `routes.rs`, `client.rs`).
- Pre-commit gate (markdown, links, spelling, doc tests): passed.
