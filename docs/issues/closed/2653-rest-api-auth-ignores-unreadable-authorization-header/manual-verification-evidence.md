---
doc-type: manual-verification-evidence
issue-spec: docs/issues/closed/2653-rest-api-auth-ignores-unreadable-authorization-header/ISSUE.md
last-updated-utc: "2026-10-26 17:08"
---

# Manual Verification Evidence

## Purpose

Record real, human-oriented verification of the behavior described in issue #2653. This is evidence from
commands performed against the real tracker binary. The pre-review reproduction (V1) is recorded here; the
final like-for-like recheck (V2) is added during implementation.

## Environment and Prerequisites

- Date and time (UTC): 2026-10-26, 12:41 to 12:42.
- Artifact under test: the `torrust-tracker` debug binary built with `cargo build --bin torrust-tracker` from the
  branch of the `docs(issues): add issue specification for #2653` commit. The branch has no source change
  under `packages/`; its merge base with `develop` is `722f2b5b265036ec9d6d02f67a00d538b33769e1`, and
  `git diff --stat 722f2b5b265036ec9d6d02f67a00d538b33769e1 develop --
  packages/axum-rest-api-server/src/v1/middlewares/auth.rs` printed nothing, so `auth.rs` is the same on `develop`.
- Operating system / environment: Linux 7.0.0-38-generic, `rustc 1.99.1`, `curl 8.18.0`.
- Prerequisites and setup performed: an isolated tracker with only the REST API enabled, using the configuration
  below and a SQLite database under `.tmp/`.

Tracker configuration (`.tmp/2653-tracker.toml`, verbatim):

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
inactive_peer_cleanup_interval = 120
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "./.tmp/2653-sqlite3.db"

[http_api]
bind_address = "127.0.0.1:12653"

[http_api.access_tokens]
admin = "EvidenceToken2653"
```

## Verification Processes

### V1 - Reproduce the unreadable `Authorization` header on the real tracker

- Goal: observe whether an `Authorization` header with a non-ASCII byte is ignored when a valid `token` query
  parameter is present, and compare it with other invalid headers.
- Initial state: a freshly started tracker with the configuration above.
- Status: `DONE`
- Outcome: **Reproduced**. The wrong outcome itself was observed: probe P1 answered `200`.

#### Steps Performed

The tracker was started at 12:41:32 UTC with `./target/debug/torrust-tracker --config-toml-path
.tmp/2653-tracker.toml` and its log reported `API: Started on: http://127.0.0.1:12653`. The probes ran one after
another, in the order below, each against `http://127.0.0.1:12653/api/v1/stats`. The header of P1 and P4 is
`Authorization: Bearer` followed by a space and the single byte `0xFF` (bash `$'Authorization: Bearer \xff'`).

| ID | Request | Start (UTC) |
| --- | --- | --- |
| P1 | Unreadable header (`0xFF`) and `?token=EvidenceToken2653` | 12:41:42.370 |
| P2 | `Authorization: Basic x` and `?token=EvidenceToken2653` | 12:41:42.393 |
| P3 | `Authorization: bearer EvidenceToken2653` and `?token=EvidenceToken2653` | 12:41:42.401 |
| P4 | Unreadable header (`0xFF`), no `token` query | 12:41:42.409 |
| P5 | `Authorization: Bearer EvidenceToken2653` only | 12:41:42.416 |
| P6 | `?token=EvidenceToken2653` only | 12:41:42.424 |
| P7 | No credentials | 12:41:42.433 |

The tracker was stopped with `kill` on its process id at about 12:41:55 UTC; `ss -tlnp | grep -c 12653` then
printed `0`.

#### Observed Result

```text
P1 HTTP 200   {"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,...
P2 HTTP 500   Unhandled rejection: Err { reason: "unknown token provided" }
P3 HTTP 500   Unhandled rejection: Err { reason: "unknown token provided" }
P4 HTTP 500   Unhandled rejection: Err { reason: "unauthorized" }
P5 HTTP 200   {"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,...
P6 HTTP 200   {"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,...
P7 HTTP 500   Unhandled rejection: Err { reason: "unauthorized" }
```

#### Conclusion

P1 shows the defect: an unreadable header together with a valid query token is accepted, while P2 and P3 show
that other invalid headers are rejected with `unknown token provided`. P4 shows the header is treated as absent:
without the query token it gets `unauthorized` (the "missing token" answer), not `unknown token provided`.
P5 to P7 are the controls and behave as documented. After the fix, P1 and P4 are expected to answer
`500` with `unknown token provided`. The same commands will be repeated as V2.

## Regression Test

### R1 - Unit test of the `auth` middleware

- Boundary: a `#[tokio::test]` in the existing test module of
  `packages/axum-rest-api-server/src/v1/middlewares/auth.rs`. It builds a minimal `Router` with one route,
  applies `auth` as a layer, and sends one request through `tower::ServiceExt::oneshot`. This is the smallest
  deterministic boundary: the decision is made inside the middleware and needs no socket. The existing contract
  tests of the package start a real server, and none sends a non-ASCII header.
- Test: `it_rejects_an_unreadable_authorization_header_even_with_a_valid_token_query`. The request carries a
  valid `token` query (`api-token-12345`) and the header value `Bearer \xff`; the expected result is `500` and the
  body `Unhandled rejection: Err { reason: "unknown token provided" }`, which is the body the route handler could
  not produce.
- Prose-first Arrange-Act-Assert review (the prose was removed once the code expressed it):
  - Arrange: the middleware guards a route and the tracker accepts `api-token-12345`; the request has a valid
    `?token` and an `Authorization` value that is not readable text. Both causal inputs, the byte `\xff` and
    the valid query, are literal in the test.
  - Act: send that request through the router.
  - Assert: the status is `500` and the body is the `unknown token provided` rejection, so the handler did not
    run. The expected values are written literally, not derived from production code.
- Dev-dependency: `tower` with the `util` feature, needed for `oneshot`; `cargo machete --with-metadata` found no
  unused dependency.

#### Red (before the fix)

Run with the stable Rust toolchain on the commit
`test(axum-rest-api-server): [#2653] reject an unreadable Authorization header`, which still has the original
`extract_bearer_token_from_header`:

```text
$ cargo test -p torrust-tracker-axum-rest-api-server --lib v1::middlewares::auth
running 4 tests
test v1::middlewares::auth::tests::it_authenticates_a_configured_token ... ok
test v1::middlewares::auth::tests::it_authenticates_each_configured_token ... ok
test v1::middlewares::auth::tests::it_rejects_tokens_that_differ_at_any_position_or_length ... ok
test v1::middlewares::auth::tests::it_rejects_an_unreadable_authorization_header_even_with_a_valid_token_query ... FAILED

thread '...it_rejects_an_unreadable_authorization_header_even_with_a_valid_token_query' panicked at packages/axum-rest-api-server/src/v1/middlewares/auth.rs:221:9:
assertion `left == right` failed
  left: 200
 right: 500

test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 11 filtered out; finished in 0.00s
```

#### Green (after the fix)

Run with the stable Rust toolchain on the commit
`fix(axum-rest-api-server): [#2653] reject an unreadable Authorization header`:

```text
$ cargo test -p torrust-tracker-axum-rest-api-server
test v1::middlewares::auth::tests::it_rejects_an_unreadable_authorization_header_even_with_a_valid_token_query ... ok
test result: ok. 15 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.08s
test result: ok. 58 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.31s
test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

`cargo +nightly fmt --all -- --check` (nightly Rust toolchain) and `cargo clippy -p
torrust-tracker-axum-rest-api-server --all-targets` (stable) passed on the same code.

### V2 - Like-for-like recheck on the real tracker after the fix

- Goal: repeat the seven probes of V1 against a tracker built after the last production change.
- Initial state: a freshly started tracker with the V1 configuration, recreated verbatim from the V1 section.
- Status: `DONE`
- Artifact under test: the `torrust-tracker` debug binary built with `cargo build --bin torrust-tracker` on the
  commit `fix(axum-rest-api-server): [#2653] reject an unreadable Authorization header`; `git status --short`
  printed nothing before the build.

#### Steps Performed

The tracker was started at 13:19:28 UTC with `./target/debug/torrust-tracker --config-toml-path
.tmp/2653-tracker.toml`, and `ss -tlnp | grep 12653` showed it listening. The same seven probes of V1 ran one after
another, from 13:19:42.495 (P1) to 13:19:42.561 (P7), with the same commands and the same header bytes. The
tracker was stopped with `kill` on its process id at about 13:19:47 UTC; `ss -tlnp | grep -c 12653` then printed
`0`.

#### Observed Result

```text
P1 HTTP 500   Unhandled rejection: Err { reason: "unknown token provided" }
P2 HTTP 500   Unhandled rejection: Err { reason: "unknown token provided" }
P3 HTTP 500   Unhandled rejection: Err { reason: "unknown token provided" }
P4 HTTP 500   Unhandled rejection: Err { reason: "unknown token provided" }
P5 HTTP 200   {"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,...
P6 HTTP 200   {"torrents":0,"seeders":0,"completed":0,"completed_in_session":0,"completed_persisted":0,...
P7 HTTP 500   Unhandled rejection: Err { reason: "unauthorized" }
```

#### Conclusion

The symptom is gone. P1 (unreadable header with a valid `?token`) changed from `200` to `500 unknown token
provided`, and P4 (unreadable header alone) changed from `unauthorized` to `unknown token provided`, so the header is
no longer treated as absent. P2, P3 and the controls P5 to P7 answer exactly as in V1.

## Failures and Follow-up

None.
