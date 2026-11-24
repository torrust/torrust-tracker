---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2999-2993-si-a-keep-secrets-out-of-errors-and-logs/ISSUE.md
last-updated-utc: "2026-11-24 16:41"
---

# Manual Verification Evidence

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
