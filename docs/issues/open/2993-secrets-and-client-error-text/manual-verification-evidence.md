---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2993-secrets-and-client-error-text/EPIC.md
last-updated-utc: "2026-11-22 15:05"
---

# Manual Verification Evidence

<!-- cspell:ignore characterse -->

## Purpose

Record the reproductions behind the inventory of EPIC #2993: what a running tracker sends to
clients and writes to its log today. It is the baseline the sub-issues SI-A and SI-B repeat before
they change anything. Nothing here is a fix; every result shows current behavior.

## Preserving Verification Artifacts

- [`evidence/reproduce.sh`](evidence/reproduce.sh): the script that ran every request below. It
  writes the tracker configuration files it uses, so they are in the script.
  After the run it was edited to satisfy ShellCheck (`cd ... || exit 1`, loop variables renamed to
  `_`, one `shellcheck disable` comment) and the private-mode UDP line it printed empty was dropped
  from the output; the edited script was not run again.
- [`evidence/udp_probe.py`](evidence/udp_probe.py): the UDP requests of V6.
- [`evidence/output.txt`](evidence/output.txt): the complete output of the run. Two edits were made
  to it: the scratch work directory is shown as `<work-dir>`, and the long run of zeros in the UDP
  cookie value is shortened to `-0.0…0<digits>`. The generated key of V4 was replaced by
  `<the generated key>` by the script itself.

All keys, tokens and passwords are test values made up for this run (`TestPeerKey...`,
`TestAdminToken0001`, `TestDbPassword01`). The databases were local containers that were removed
after the run.

The script is disposable. Python is used for the UDP probe because it needs only a few lines of
socket code and no build; the sub-issues promote the durable checks to Rust tests, as their
specifications say. The EPIC owner removes both scripts when the EPIC closes, unless a sub-issue
still uses them.

## Environment and Prerequisites

- Date and time (UTC): 2026-11-22 14:43 to 14:45.
- Artifact under test: `target/debug/torrust-tracker`, built with `cargo build --bin torrust-tracker`
  (`rustc 1.100.0 (0ef1bc386 2026-11-09)`, the stable toolchain of the checkout) from a tree whose
  code equals `develop` at `98fa27df2` (the branch adds documentation only;
  `git diff --stat 98fa27df2 HEAD -- packages src Cargo.toml Cargo.lock console contrib` is empty).
  This is before the SI-4 pull request of EPIC #2912 (#2991), so replies still hold source locations.
- Operating system / environment: Linux 7.0.0-43-generic; Docker 28.3.3; images `mysql:8.0` and
  `postgres:16`; `curl`; `python3 -I`.
- Prerequisites and setup performed: none beyond the build. The script starts and stops the
  trackers and the two database containers.
- Command: `bash evidence/reproduce.sh <path to target/debug/torrust-tracker> <empty work dir>`.

An earlier run on the same day used an older debug build (file date 2026-11-19, commit not
recorded) and gave the same texts. Only the run above is evidence.

## Analysis

Three configurations are used: a private tracker on SQLite with the HTTP tracker and the REST API
(V1 to V5); a public tracker with `listed = true`, because UDP trackers are not started in private
mode (V6); and trackers on MySQL and PostgreSQL (V8). V7 needs no running tracker: the
configuration fails to load. `trace_filter = "debug"` is set everywhere so that the DEBUG lines are
visible.

## Verification Processes

Each process lists the request and the observed result, copied from `evidence/output.txt`.

### V1 - Peer key errors of the HTTP tracker (inventory A1)

- `GET /announce/TestPeerKey000000000000000000001?...` (well formed, not registered):

  ```text
  d14:failure reason141:Tracker authentication error: Failed to read key: TestPeerKey000000000000000000001, packages/tracker-core/src/authentication/service.rs:98:27e
  ```

- `GET /announce?...` (no key):

  ```text
  d14:failure reason108:Tracker authentication error: Missing authentication key, packages/http-core/src/services/announce.rs:308:27e
  ```

- `GET /announce/ShortTestKey01?...` (14 characters):

  ```text
  d14:failure reason110:Tracker authentication error: Invalid format for authentication key param: expected 32 alphanumeric characterse
  ```

Conclusion: only the "not registered" case echoes the key. The malformed case already omits it and
does not say what is wrong with it (the length).

### V2 - REST API key errors (inventory A2)

- `POST /api/v1/keys` with `{"key":"ShortTestKey01","seconds_valid":100}`:

  ```text
  Invalid URL: invalid auth key: string "ShortTestKey01", Invalid key length. Key must be have 32 chars, packages/tracker-core/src/authentication/handler.rs:108:46 [400]
  ```

- `DELETE /api/v1/key/BadTestKey`:

  ```text
  Invalid auth key id param "BadTestKey" [400]
  ```

Conclusion: both replies echo the supplied key.

### V3 - Request logs (inventory A3, A4)

The `uri=` fields of the server log after V1, V2 and a `GET /api/v1/stats?token=WrongTestToken0001`
(which answered `Unhandled rejection: Err { reason: "token not valid" } [500]`):

```text
uri=/announce/ShortTestKey01?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=17548&left=0
uri=/announce/TestPeerKey000000000000000000001?info_hash=%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00%00&peer_id=-qB00000000000000001&port=17548&left=0
uri=/api/v1/key/BadTestKey
uri=/api/v1/stats?token=WrongTestToken0001
```

The other REST requests sent the admin token in the `Authorization: Bearer` header. The log held
0 lines with that token.

Conclusion: a key in the path and a token in the query string are logged; a token in the header is
not. The log line for the 500 reply holds the status and the request id and no reason:

```text
2026-11-22T14:43:51.138915Z ERROR request{method=GET uri=/api/v1/stats?token=WrongTestToken0001 version=HTTP/1.1}: API: response latency_ms=0 status_code=500 Internal Server Error server_socket_addr=127.0.0.1:41212 service_binding=http://127.0.0.1:41212/ request_id=79d28710-5453-4595-b6a5-7aaf83e34e8c
```

### V4 - Generated key in the log (inventory A5)

`POST /api/v1/key/3600` answered 200. The log then held one `Generated key` line, and one line with
the generated key:

```text
2026-11-22T14:43:51.142622Z DEBUG request{method=POST uri=/api/v1/key/3600 version=HTTP/1.1}: torrust_tracker_core::authentication::key: Generated key: <the generated key>, valid for: 3600s seconds
```

Conclusion: at DEBUG level the new key is written to the log in clear.

### V5 - REST API database and rejection text (inventory B)

- The same key added twice (SQLite); the second reply:

  ```text
  Unhandled rejection: Err { reason: "failed to add key: The Sqlite3 query was malformed: error returned from database: (code: 2067) UNIQUE constraint failed: keys.key, packages/tracker-core/src/databases/driver/sqlite/auth_key_store.rs:74:22, packages/tracker-core/src/authentication/handler.rs:103:50" } [500]
  ```

- A malformed JSON body (`{bad`):

  ```text
  Failed to parse the request body as JSON: key must be a string at line 1 column 2 [400]
  ```

- `GET /api/v1/key/abc/extra`: 404 with an empty body.

Conclusion: the reply for a database failure is the driver text; the JSON rejection is serde's text.

### V6 - UDP and HTTP replies on a public listed tracker (inventory B)

```text
--- UDP announce, not whitelisted
tracker announce error: Tracker core whitelist error: The torrent: 0000000000000000000000000000000000000000, is not whitelisted, packages/tracker-core/src/whitelist/authorization.rs:61:23
--- UDP announce, wrong connection id
tracker announce error: Connection cookie error: cookie value is expired: -0.0…010544892055975403, expected > 1795358512.2066972
--- UDP 3-byte packet
error parsing request: SendableRequestParseError { message: "Couldn't parse action", opt_connection_id: None, opt_transaction_id: None }
--- HTTP announce, not whitelisted
d14:failure reason158:Tracker whitelist error: The torrent: 0000000000000000000000000000000000000000, is not whitelisted, packages/tracker-core/src/whitelist/authorization.rs:61:23e
```

Conclusion: the UDP replies nest internal error names; the connection-id reply holds a value
derived from the server clock (`expected > 1795358512.2066972`); the parse reply holds a Rust type
name.

Separately, the private configuration with a `[[udp_trackers]]` entry added logged:

```text
2026-11-22T14:44:17.829867Z  WARN complete_startup: torrust_tracker_lib::app: Could not start UDP trackers while in private mode. UDP is not safe for private trackers!
```

This run was a single command outside the script:
`timeout 8 target/debug/torrust-tracker --config-toml-path private-udp.toml`, where
`private-udp.toml` is the script's `private.toml` plus
`[[udp_trackers]]` / `bind_address = "127.0.0.1:46969"`.

### V7 - Configuration errors echo the value (inventory A10)

`password = 987654321` under `[core.database]`, then `admin = 424242` under
`[http_api.access_tokens]`:

```text
Failed processing the configuration: invalid type: found signed int `987654321`, expected a string for key "default.core.database.password" in TOML source string, packages/configuration/src/v3_0_0/mod.rs:409:28, packages/configuration/src/lib.rs:258:56
Failed processing the configuration: invalid type: found signed int `424242`, expected a string for key "default.http_api.access_tokens.admin" in TOML source string, packages/configuration/src/v3_0_0/mod.rs:409:28, packages/configuration/src/lib.rs:258:56
```

Conclusion: a secret that TOML reads as an integer is printed on stderr and in the log.

### V8 - Database server messages (inventory A11, A12)

Wrong credentials (user `db_user`, a wrong password); the end of the startup error:

```text
The MySQL query was malformed: error returned from database: 1045 (28000): Access denied for user 'db_user'@'172.17.0.1' (using password: YES), packages/tracker-core/src/databases/driver/mysql/schema_migrator.rs:86:41
Failed to apply PostgreSQL schema migrations: while executing migrations: error returned from database: password authentication failed for user "db_user", packages/tracker-core/src/databases/driver/postgres/schema_migrator.rs:12:9
```

The same key added twice, on MySQL:

```text
Unhandled rejection: Err { reason: "failed to add key: The MySQL query was malformed: error returned from database: 1062 (23000): Duplicate entry 'TestPeerKey000000000000000000003' for key 'keys.key', packages/tracker-core/src/databases/driver/mysql/auth_key_store.rs:72:22, packages/tracker-core/src/authentication/handler.rs:103:50" } [500]
```

The MySQL tracker's log then held 1 line with the key (cut at 330 characters by the script):

```text
2026-11-22T14:44:04.850783Z DEBUG request{method=POST uri=/api/v1/keys version=HTTP/1.1}: torrust_located_error: The MySQL query was malformed: error returned from database: 1062 (23000): Duplicate entry 'TestPeerKey000000000000000000003' for key 'keys.key', packages/tracker-core/src/databases/driver/mysql/auth_key_store.rs:72:2
```

On PostgreSQL:

```text
Unhandled rejection: Err { reason: "failed to add key: The PostgreSQL query was malformed: error returned from database: duplicate key value violates unique constraint \"keys_key_key\", packages/tracker-core/src/databases/driver/postgres/auth_key_store.rs:72:22, packages/tracker-core/src/authentication/handler.rs:103:50" } [500]
```

and its log held 0 lines with the key.

Conclusion: the server message names the database user and the client address (A11). On MySQL the
duplicate-key message holds the peer key, in the reply and in a DEBUG log line; on PostgreSQL and
SQLite (V5) it does not (A12). An authentication failure is labelled "query was malformed".

### V9 - After SI-4 of EPIC #2912 merged

SI-4 (#2916) merged into `develop` as `4fa49ec9c` while this pull request was in review. The key
cases were sent again at 2026-11-22 15:00 UTC to a debug build of the tree that includes it
(`develop` at `6f62cc107` plus this branch's documentation), with the same configuration files and
by hand, not with the script:

- `GET /announce/TestPeerKey000000000000000000001?...`:

  ```text
  d14:failure reason82:Tracker authentication error: Failed to read key: TestPeerKey000000000000000000001e
  ```

- `POST /api/v1/keys` with `{"key":"ShortTestKey01","seconds_valid":100}`:

  ```text
  Invalid URL: invalid auth key: string "ShortTestKey01", Invalid key length. Key must be have 32 chars
  ```

- The same key added twice on MySQL; the second reply:

  ```text
  Unhandled rejection: Err { reason: "failed to add key: The MySQL query was malformed: error returned from database: 1062 (23000): Duplicate entry 'TestPeerKey000000000000000000003' for key 'keys.key'" } [500]
  ```

  The MySQL tracker's log held 0 lines with the key.

Conclusion: SI-4 removed the source locations and, with `LocatedError`, the DEBUG line that held
the key. The key is still echoed in the HTTP failure reason, in the REST reply for an uploaded key
and in the MySQL duplicate-key reply. The other processes were not repeated on this build.

## Not Reproduced

- A6 (the REST client prints the URL with the token in a transport error), A7 and A8 (plain
  `String` types) were confirmed by reading the code, not by running it. The EPIC inventory gives
  the lines.
- The expired-key reply was not requested: it needs a key past its lifetime.

## Failures and Follow-up

No request failed to run. The findings are the inventory of the EPIC; the sub-issues own the fixes.
