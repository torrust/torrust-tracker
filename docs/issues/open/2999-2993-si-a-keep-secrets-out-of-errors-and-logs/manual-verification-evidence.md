---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md
last-updated-utc: "2026-11-25 11:34"
---

# Manual Verification Evidence

<!-- cspell:ignore nnounce oken zzoken -->

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
- Names are compared after percent-decoding (review round 1 of the pull request: the server
  decodes `%74oken` to `token`, so an encoded name must not keep its value; see section R1). The
  logged text keeps the original spelling of the name.
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

Five of the six tests the module had at that point failed on the starting loader
([`evidence/t7-red.txt`](evidence/t7-red.txt), `1 passed; 5 failed`); for each: `the error text
holds the secret value`. The non-secret control test passed. The seventh test (the
environment-override case) was added after that red run, with the fix already in place, so it
was not seen red then; its guard was shown later by mutation (section R2, finding F3 of the
second review). An earlier version of this paragraph said "five of seven", which was wrong.

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

## T9 - Final Recheck (M1 to M7)

- Date and time (UTC): 2026-11-24 17:54 to 17:56.
- Artifact: `target/debug/torrust-tracker` built with `cargo build --bin torrust-tracker` after the
  last production change, from the branch head with the subject `docs(secrets): [#2999] teach the
  Authorization header and the peer-key rule` (the commits before it hold all production changes;
  this one only changes documentation). `rustc 1.100.0`, Docker 28.3.3, `mysql:8.0`, `postgres:16`.
- Command: `bash <abs path>/docs/issues/open/2993-secrets-and-client-error-text/evidence/reproduce.sh
  <abs path>/target/debug/torrust-tracker <scratch work dir>` (the baseline's command, unchanged).
  Complete output: [`evidence/final-output.txt`](evidence/final-output.txt), compared with
  [`evidence/baseline-output.txt`](evidence/baseline-output.txt).

| ID | Result | Observed (final run) |
|---|---|---|
| M1 | DONE | Unregistered key: `Authentication key not registered`. No key: `Missing authentication key: this tracker is private, use /announce/<key>`. 14 characters: `Invalid authentication key: expected 32 characters, got 14`. None holds a key. The characters case (`...only the characters 0-9, a-z and A-Z are allowed`) and the expired case (`Authentication key expired`) were not requested by the script; the contract and unit tests assert them. |
| M2 | DONE | `POST /api/v1/keys` short key: `Invalid URL: invalid auth key: expected 32 characters, got 14 [400]`. `DELETE /api/v1/key/BadTestKey`: `Invalid auth key id param: expected 32 characters, got 10 [400]`. |
| M3 | DONE | With `trace_filter = "debug"`: 0 `uri=` fields hold a test key or token; the 500 line shows `uri=/api/v1/stats?token=***`; 0 log lines hold the admin token. The health-check API is covered by its test binary (the script does not start it with traffic). |
| M4 | DONE | `Generated key` lines: 0; lines holding the generated key: 0. |
| M5 | DONE | ``invalid type: found a value of another type (not shown: the option is a secret), expected a string for key "default.core.database.password"`` and the same for `default.http_api.access_tokens.admin`. |
| M6 | DONE (by test) | The REST client is a library with no command line; the closed-port GET with a token and key delete are the `rest-api-client` tests (`it_should_not_hold_the_token_or_the_url_query_in_the_transport_error_of_a_get_request`, `it_should_not_hold_the_key_in_the_transport_error_of_a_key_delete`), which run the real client against a refused connection. |
| M7 | DONE | MySQL duplicate key: `failed to add key: The MySQL database already holds this authentication key [500]`; 0 log lines hold the key. PostgreSQL gives the same text; SQLite likewise. |

Unchanged by design, owned by SI-B: the database server message for wrong credentials still names
the user and the client address (A11), and UDP and whitelist reply texts.

### Automatic checks

`./contrib/dev-tools/git/hooks/pre-commit.sh` passed for every commit (cargo machete, `linter all`,
nightly formatting, doc tests). `cargo test --workspace --no-fail-fast` passed after the last code
change; the MySQL and PostgreSQL driver tests passed with
`--features db-compatibility-tests`.

### Completion review

No `implementation-retrospective.md` is needed: no increment needed a redesign. Two notes for
reviewers: (1) the HTTP-extractor and REST-reply areas were written together with their tests and
proven by mutation, not by a run against the old code (the adapter stopped compiling as soon as
`PeerKeyError` lost its field); (2) the generated-key log lines were removed, not replaced by a
key-less line.

## R1 - Review Round 1 (Greptile)

Greptile reviewed the head with the subject `docs(issues): [#2999] record the final recheck and the
acceptance evidence` and reported two findings, both accepted:

1. **P1, encoded token names reach the log**: `redact_query` compared the raw name, but the server
   decodes it, so `/api/v1/stats?%74oken=<token>` authenticates and was logged unredacted. The
   redaction now percent-decodes query parameter names and path segment names before comparing
   (the logged text keeps the original spelling). Red run: four new cases failed
   (`%74oken`, `%74%6F%6B%65%6E`, `to%6ben`, `/%61nnounce/<key>`; 21 passed, 4 failed); green: 25
   passed. A name that is not UTF-8 after decoding is not treated as `token` (that case is in the
   table). An earlier version of this entry also said a malformed-escape case was in the table;
   it was not, and the decoder accepted a sign after the `%` (section R2, finding F1 of the second
   review, fixes both).
2. **P2, import style**: the code this branch added used `std::fmt::...`, `std::borrow::Cow` and an
   `Arc` import inside a function. Imports moved to the file headers with short names in
   `handler.rs`, `peer_key.rs`, `add_key_form.rs`, `databases/error.rs` and `test-helpers/logging.rs`
   (behaviour unchanged; the workspace builds and the tests pass).

The checks run again after this round: `cargo test --workspace --no-fail-fast` and the pre-commit gate.

### Recheck after the review fix (2026-11-24 18:10 UTC)

Production code changed after the T9 run, so the recheck was repeated on a debug build of the
branch head after the review-fix commit (subject `fix(axum-server): [#2999] decode query and path
names before redacting the request URI`): `reproduce.sh` with the same command as T9, output in
[`evidence/final-output-after-review-round-1.txt`](evidence/final-output-after-review-round-1.txt).
Every M1 to M7 line is the same as in T9 (no key or token in any reply or log line, `uri=...?token=***`,
0 `Generated key` lines, the configuration errors name the option, the duplicate key on MySQL and
PostgreSQL answers `already holds this authentication key`). By hand, on a private tracker with the
script's configuration, `GET /api/v1/stats?%74oken=TestAdminToken0001` returned the statistics JSON
(the server decodes the name, so the request authenticated) and the log held
`uri=/api/v1/stats?%74oken=***` and 0 lines with the test token.

## R2 - Review Round 2 (da2ce7)

da2ce7 approved the head with the subject `docs(pr-reviews): [#2999] add the PR 3042 review audit`
(review 6054012280, 2026-11-25 08:18 UTC) with seven findings, none blocking. All were accepted.
The audit record assigns them the audit-local ids F3 to F9, because F1 and F2 are taken by the
first round. Red and mutation output: [`evidence/review-round-2.txt`](evidence/review-round-2.txt).

- Reviewer F1 (Minor), `percent_decoded` decodes `%+1`: `u8::from_str_radix` accepts a sign, so
  `%+4` was decoded to the byte 0x04, while the comment called it malformed and the evidence
  claimed a table case that did not exist. Nothing leaked (such an escape yields only 0x00 to
  0x0F). The decoder now requires two ASCII hexadecimal digits. Red run of a new test of the
  decoder: `left: "\u{4}oken"`, `right: "%+4oken"` (30 passed, 1 failed); green: 32 passed. The
  redaction table gained the row `%7=1&%+4=2`, which stays as written.
- Reviewer F2 (Minor), `use-rest-api` skill: the "Query Parameter Method" section showed the
  header command. It now says in one paragraph that the server still accepts `?token=` and why the
  header is the safe form, with no query example.
- Reviewer F3 (Minor), the configuration red run: corrected in section T7 above ("five of the six
  tests"). The environment-override test now has a recorded mutation: with the
  `core.database.password` entry removed from `SECRET_OPTIONS`, it and the numeric-password case
  fail with `the error text holds the secret value` (5 passed, 2 failed); the file was restored
  from a copy and the 7 tests pass.
- Reviewer F4 (Nit), dictionary: `nnounce` and `oken` are removed from `project-words.txt` and
  ignored inline (`cspell:ignore`) in `request_log.rs`, this file and the audit. `unredacted` stays.
- Reviewer F5 (Nit): the blank line at the end of `evidence/t6-mutations.txt` and
  `evidence/t7-mutations.txt` is removed; `git diff --check` against `develop` prints nothing.
- Reviewer F6 (Nit): `last-updated-utc` of this file and of `ISSUE.md` are current, `related-pr`
  is 3042, and the progress log has entries for the pull request and both review rounds.
- Reviewer F7 (Nit), the length test: `it_should_count_characters_not_bytes_when_naming_the_received_length`
  now uses a 31-character, 32-byte key and asserts `expected 32 characters, got 31`; the former
  32-character case keeps its assertion under the name
  `it_should_reject_a_32_character_key_with_a_non_ascii_character_for_its_characters`. Mutation
  (the length counts bytes): both fail (`left: "only the characters ..."`, `right: "expected 32
  characters, got 31"`); restored, 50 passed.

### Recheck after the second review fix (2026-11-25 08:26 UTC)

Production code changed again (`percent_decoded`), so the recheck was repeated on a debug build of
the tree that holds the fix commit with the subject `fix(axum-server): [#2999] decode only a
percent followed by two hex digits when redacting`: `reproduce.sh` with the same command as T9,
output in [`evidence/final-output-after-review-round-2.txt`](evidence/final-output-after-review-round-2.txt).
A `diff` of this output with the round 1 output, leaving out the lines that hold timestamps,
`valid_until`, request ids and the UDP cookie value, prints nothing: M1 to M7 are unchanged. By
hand on the private tracker of the script: `GET /api/v1/stats?%74oken=TestAdminToken0001` returned
the statistics and was logged as `uri=/api/v1/stats?%74oken=***`;
`GET /api/v1/stats?%+4oken=TestAdminToken0001` answered `unauthorized` (the server does not read
that name as `token`) and was logged as written, value included. That is the intended rule: only
a parameter the server reads as `token` is redacted, so a value sent under any other name is
logged like any other parameter.

## R3 - Review Round 3 (CodeRabbit)

CodeRabbit reviewed the head with the subject `fix(axum-server): [#2999] decode only a percent
followed by two hex digits when redacting` (review 6054287756, 2026-11-25 08:40 UTC) and posted
three findings, recorded in the audit as F10 to F12. Red output:
[`evidence/review-round-3.txt`](evidence/review-round-3.txt).

- F10 (Major), TOML syntax errors: confirmed in part. A `figment` error for a TOML syntax error
  has no option path and quotes the whole source line, so for a broken password line the error
  object held the password and `redact_secret_values` did not change it. It did not reach a user:
  both loaders check the mandatory options first, and that check replaces any parse error by
  `Missing mandatory configuration option. Option path: metadata.schema_version` (observed on the
  binary before the fix, with `password = "TestDbPasswordSyntax01" extra`: given through
  `TORRUST_TRACKER_CONFIG_TOML`, the last line of the startup error ended with that message; given
  as a file, the first 400 characters of that line were read and end inside the same message. The
  occurrences of the password were counted only after the fix: 0, see the recheck below). The
  redaction now keeps the location of a syntax error (`TOML parse error at line N, column M`) and
  drops the quoted line, so the text stays safe if the loaders start reporting syntax errors. Red:
  the new unit test of `redact_secret_values` failed with `the error text holds the secret value`
  (8 passed, 1 failed); the new test through the loader passed before the fix, for the reason
  above, and guards that path. Green: 170 passed in the package. The sentence in
  `docs/containers.md` ("A configuration error names the option and never prints the password")
  stays: both paths are covered and tested.
- F11 (Minor), "the safe form": the documents said the header is the safe form. They now say what
  it does (it keeps the token out of logged URLs) and that it does not protect the token in
  transit without HTTPS: `docs/containers.md` (and a warning on its plain-HTTP Azure example, which
  has no TLS to switch to), `src/lib.rs`, `axum-rest-api-server` (`lib.rs`,
  `v1/middlewares/auth.rs`), the skills `handle-secrets` and `use-rest-api`, and the ADR amendment.
  `grep -rn "safe form"` over the Markdown and Rust files outside `docs/issues/` and
  `docs/pr-reviews/` prints nothing.
- F12 (Major), a key after an empty path segment: confirmed. `/announce//<key>` was logged whole,
  because only the next segment was looked at. Anything that is not empty after the prefix is now
  redacted; `/announce`, `/announce/` and `/announce//` are unchanged. Red: three new table rows
  failed (33 passed, 3 failed); green: 36 passed.

### Recheck after the third review fix (2026-11-25 08:46 UTC)

Production code changed again (`request_log.rs`, `secret_options.rs`), so the recheck was repeated
on a debug build of the tree that holds the fix commit with the subject `fix(secrets): [#2999]
redact a key after an empty path segment and the source line of a TOML syntax error`:
`reproduce.sh` with the same command as T9, output in
[`evidence/final-output-after-review-round-3.txt`](evidence/final-output-after-review-round-3.txt).
A `diff` with the round 2 output, leaving out the lines that hold timestamps, `valid_until`,
request ids and the UDP cookie value, prints nothing. By hand on the private tracker of the script:
`GET /announce//TestPeerKey000000000000000000009?port=1` (HTTP tracker) and
`DELETE /api/v1/key//TestPeerKey000000000000000000009` (REST API, token in the header) both
answered 404, the log held 0 lines with the key, and its `uri=` fields were
`uri=/announce/***?port=1` and `uri=/api/v1/key/***`. Starting the tracker with the broken password
line printed 0 occurrences of the password.

## R4 - Review Round 4 (da2ce7)

da2ce7 requested changes on the head before the round 3 fix (review 6054388295, 2026-11-25 08:49
UTC) with three findings, recorded in the audit as F13 to F15. Two are the findings of round 3
(the key after an empty path segment; the TOML syntax error), already fixed by the round 3 commit,
so they changed nothing; their threads were answered as superseded. The third, two progress-log
entries out of stamp order in `ISSUE.md`, was fixed by moving one entry. No production code
changed in this round. For the reviewer's unterminated-string example, the tracker binary built
after the round 3 fix was started with a configuration whose last line is
`password = "TestDbPasswordUnterminated01` (no closing quote): the output ended with `Missing
mandatory configuration option. Option path: metadata.schema_version` and held 0 occurrences of
the password.

## R5 - Review Round 5 (Greptile, da2ce7)

Greptile reviewed the head with the round 3 fix (review 6054462408, 2026-11-25 08:55 UTC) with two
findings, and da2ce7 commented on the same head (review 6054537873, 09:00 UTC) with one. They are
F16 to F18 in the audit.

- F16 (Greptile), slash-heavy requests: confirmed. The round 3 change looked at the rest of the
  path for every segment, so a path of only slashes cost a quadratic number of comparisons in the
  logging callback, which runs for every request, also a 404. Measured in a test build with a
  temporary timing print (removed afterwards): a slash-only path of 60000 bytes took 657.9 ms
  before the fix and 1.0 ms after it ([`evidence/review-round-5.txt`](evidence/review-round-5.txt)).
  `redact_path` now finds the last segment that is not empty once and looks for a prefix before
  it. The behaviour is unchanged: the 36 table and decoder cases pass, and two tests were added (a
  key after 60000 slashes is redacted; a path of 60000 slashes is returned as it is). They assert
  the result, not the time, so they do not fail on the old code; the measurement above is the
  evidence for the cost.
- F17 (Greptile), shell history: the `use-rest-api` skill said a URL is written to "shell-history
  logs" and that the header keeps the token out of "those logs", but a header typed on a command
  line is saved in the shell history too. The sentence now names proxy, browser and server logs
  only, and a paragraph says neither form keeps a token out of the shell history and shows reading
  it from a file inside the command. `grep -rn "shell-history"` over the Markdown and Rust files
  outside `docs/issues/` and `docs/pr-reviews/` prints nothing.
- F18 (da2ce7), audit rows: at the head he read, the audit had no rows for the three round 3
  threads, although their replies named F10 to F12, and none for the three round 4 threads. The
  rows F10 to F12 were committed after the replies (`docs(pr-reviews): [#2999] record the third
  review round in the PR 3042 audit`) and F13 to F15 in `docs(pr-reviews): [#2999] record the
  fourth review round in the PR 3042 audit`. Each round's replies were posted after its fix was
  pushed and before its audit commit was pushed, so a review that reads the head in between sees
  replies without rows.

### Recheck after the fifth review fix (2026-11-25 09:06 UTC)

Production code changed again (`redact_path`), so the recheck was repeated on a debug build of the
tree that holds the fix commit with the subject `fix(axum-server): [#2999] find the end of the
secret path once when redacting`: `reproduce.sh` with the same command as T9, output in
[`evidence/final-output-after-review-round-5.txt`](evidence/final-output-after-review-round-5.txt).
A `diff` with the round 3 output, leaving out the lines that hold timestamps, `valid_until`,
request ids and the UDP cookie value, prints nothing. By hand on the private tracker of the
script: `GET /announce//TestPeerKey000000000000000000009?port=1` answered 404 and
`GET /announce/TestPeerKey000000000000000000009?port=1` answered 200 (a failure reason); the log
held 0 lines with the key and its only `uri=/announce` field was `uri=/announce/***?port=1`. A
`GET` of a path of 30000 slashes answered 404 in 0.0014 s (`curl`'s `time_total`).

## R6 - Review Round 6 (CodeRabbit, da2ce7)

The fifth round reached the limit of rounds a session runs on its own; the maintainer asked for a
sixth ("Round 6 now", in the session, 2026-11-25). CodeRabbit reviewed the head with the round 5
fix (review 6054779352, 09:17 UTC) with two findings, and da2ce7 approved the head with the round 5
audit commit (review 6054897595, 09:25 UTC) with two nits. They are F21 to F24 in the audit.

- F21 (CodeRabbit, Minor), log-safety claim: the documents said the header keeps the token out of
  proxy and server logs. A proxy or server set to log request headers still records it. They now
  say the header keeps the token out of the URL and name that limit: `docs/containers.md`, the
  skills `use-rest-api` and `handle-secrets`, `src/lib.rs` and `axum-rest-api-server`
  (`lib.rs`, `v1/middlewares/auth.rs`).
- F22 (CodeRabbit, Minor), rows F16 to F18: at the head it read, this file said they were in the
  audit, whose table ended at F15. They were added by the round 5 audit commit, pushed after that
  head; nothing else changed.
- F23 (da2ce7, Nit), the comment on the 60000-slash test: it read as a guard on the cost, but the
  test checks the result only. The comment now says so and points at the measurement in section R5.
- F24 (da2ce7, Nit), two audit details: the F13 detail quoted the scan that round 5 replaced, and
  the F19 detail said its `grep` prints a single line where it prints two. Both details carry a
  note now.

No production code changed in this round (documentation, a test comment and the audit record), so
the real-tracker recheck of section R5 stands. `cargo test -p torrust-tracker-axum-server --lib
request_log` passes (38 passed) and the pre-commit gate passes.
