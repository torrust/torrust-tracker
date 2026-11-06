---
doc-type: manual-verification-evidence
issue: 2637
status: completed
measured-utc: 2026-11-06T13:32:00Z
---

# Manual Verification Evidence

Issue #2637 manual scenarios M1-M5, run by hand with `curl` against the `torrust-tracker` debug binary built from
the S6 branch (production code identical to `develop` at the merge of #2744). Each scenario used an isolated
configuration (loopback API on ports 41212-41215, a per-scenario SQLite file, UDP and HTTP trackers on port 0) kept
in the git-ignored `.tmp/manual/`. The tracker was stopped with `SIGINT` after each scenario and every process
logged `Torrust tracker successfully shutdown.`. No script was used.

| ID | Result | Observed |
| --- | --- | --- |
| M1 | PASS | `/api/health_check` without a token: `200 {"status":"Ok"}`. `/api/v1/stats` with no token and with a wrong token: `500`, body `Unhandled rejection: Err { reason: "unauthorized" }` and `... "token not valid" }`. With `Authorization: Bearer MyAccessToken`: `200` and the stats JSON. |
| M2 | PASS | Valid header and invalid `?token=`: `200` (the header decides). Invalid header and valid `?token=`: `500 "token not valid"`. `Authorization: Basic MyAccessToken` and a bare `Authorization: MyAccessToken`: `500 "unknown token provided"`. |
| M3 | PASS | Default (public, unlisted): `POST /api/v1/key/60` -> `409 private capability is disabled by configuration`; `POST /api/v1/whitelist/<hash>` -> `409 listed capability is disabled by configuration`. `private = true`: the key `POST` -> `200` with a key, the whitelist `POST` -> `409`. `listed = true`: the whitelist `POST` -> `200 {"status":"ok"}`, the key `POST` -> `409`. |
| M4 | PASS | `/api/v1/stats?format=json` -> `200` JSON with the documented fields; `?format=prometheus` -> `200` Prometheus text (`# HELP torrents ...`). `/api/v1/metrics?format=json` -> `200 {"metrics":[...]}`; `?format=prometheus` -> `200` Prometheus text (`# HELP tracker_core_...`). |
| M5 | PASS | With `[http_api.tls_config]` and a self-signed certificate: `curl --insecure https://127.0.0.1:41215/api/health_check` -> `200`, the protected route with the token -> `200`; plain `http://` to the same port fails (`curl` code `000`). `SIGINT` -> the API logged the graceful shutdown, the process exited and logged `successfully shutdown`. |

## Observation (not a #2637 change)

An unauthenticated or wrongly authenticated request to a protected route answers `500` with a plain-text
`Unhandled rejection` body, not `401`/`403`. The package integration tests pin this legacy behavior, so it was
recorded in `lessons.md` as a candidate for a separate issue and not changed here.
