---
schema-version: 1
doc-type: issue
issue-type: bug
status: in-progress
priority: p3
epic: null
github-issue: 2726
spec-path: docs/issues/open/2726-rest-api-https-health-check/ISSUE.md
branch: "2726-rest-api-https-health-check"
related-pr: null
last-updated-utc: "2026-11-05 13:07"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/axum-rest-api-server/src/server.rs
    - packages/axum-http-server/src/server.rs
    - packages/axum-health-check-api-server/src/handlers.rs
    - docs/issues/closed/2089-fix-https-tracker-health-check-protocol/ISSUE.md
    - docs/issues/open/2637-1347-add-tests-axum-rest-api-server/ISSUE.md
---

<!-- skill-link: create-issue -->

# Issue #2726 - The REST API health check probes `http://` even when the API serves TLS

## Goal

Make the health check that the REST API registers probe the API over the protocol the server actually serves, so a
REST API configured with TLS is reported healthy when it is healthy.

## Background

Found while planning the `server.rs` slice (S5, ledger row 18) of #2637 (the `torrust-tracker-axum-rest-api-server`
test coverage, EPIC #1347). Nothing in #2637 is changed by this draft.

`check_fn` in `packages/axum-rest-api-server/src/server.rs` builds the health check URL by hand:

```rust
let url = format!("http://{}/api/health_check", service_binding.bind_address());
```

The scheme is hard-coded. `ApiServer::start` and `ApiServer::start_with_cancellation` both register `check_fn` for a
service binding whose protocol is `Protocol::HTTPS` when the launcher has a TLS configuration (the binding is built from
`launcher.tls.is_some()`), so with TLS enabled the registered binding says `https://...` while the registered check
probes `http://...` on the same TLS listener, and fails.

The HTTP tracker had the same defect and fixed it in #2089 (commit subject `fix(axum-http-server): check HTTPS trackers
over TLS`): its `check_fn` builds the URL from the service binding (`service_binding.url().join("health_check")`) and a
`check_fn_with_client` variant lets tests trust a known test certificate. The REST API was out of scope there. This is
not known to be tracked elsewhere (searched `docs/issues/` and the drafts).

### Evidence (reproduced with real artifacts)

A throwaway in-crate test (kept out of the repository; the text is in the hand-off) started the token-aware server
with a TLS launcher built from the existing test certificate of `packages/axum-health-check-api-server/tests/fixtures`,
read the registered service from the `Registar` and ran its health check, as the health check API does:

```text
BINDING<<https://127.0.0.1:41907/>>
INFO<<checking api health check at: http://127.0.0.1:41907/api/health_check>>
RESULT<<Err("error sending request for url (http://127.0.0.1:41907/api/health_check)")>>
HTTPS GET https://127.0.0.1:41907/api/health_check  ->  Ok(200)   (client trusting the test certificate)
```

The same listener answers `200` over HTTPS, so the service is healthy and only the probe is wrong.

### Real-binary reproduction (2026-11-04)

The tracker was run with only the REST API and the health check API, the REST API serving TLS with the test certificate
(configuration schema 3.0.0, `[http_api.tls_config]`). `curl -sk https://127.0.0.1:41212/api/health_check` answered `200`,
while the health check API report was `status: "Error"` with HTTP `200`:

```text
"service_binding":"https://127.0.0.1:41212/", "info":"checking api health check at: http://127.0.0.1:41212/api/health_check",
"result":{"Err":"error sending request for url (http://127.0.0.1:41212/api/health_check)"}
```

Control with TLS disabled: the entry probes `http://127.0.0.1:41212/api/health_check`, the result is `{"Ok":"200 OK"}` and
the report is `status: "Ok"`.

### Impact (what was and was not verified)

- The health check API (`packages/axum-health-check-api-server`) turns any failed check into a report whose status is
  `Error` ("health check failed") with the REST API entry's `result` set to the error above. **Verified by reading the
  code**: `CheckReport::fail` is `result.is_err()` and the handler answers `responses::error`.
- That report is still served with HTTP `200` (`Json(Report::error(..))` has no status change), and the container
  `HEALTHCHECK` (`http_health_check`) only checks for a success status, so **the container is not marked unhealthy**.
  Verified by reading `Containerfile` and `src/bin/http_health_check.rs`. The harm is a false `Error` report for anyone
  reading the body of `/health_check` (monitoring, dashboards) on a tracker with a TLS REST API.
- Not verified: other consumers of the report body, and whether any deployment enables TLS on the REST API today.

### Observation, a separate question (not part of this bug)

`check_fn` returns `Ok(<status text>)` for any HTTP response, including a `500`, and returns `Err` only when the request
cannot be sent. Its doc comment says "Or if there request returns an error code". The HTTP tracker's `check_fn` behaves the
same way, so this looks like an ecosystem-wide design, not a REST API defect. Decide whether to align the doc comment or
the behavior in a separate issue.

## Scope

### In Scope

- Build the REST API health check URL from the service binding so the scheme follows the served protocol
  (`https` when TLS is enabled), as the HTTP tracker does.
- A way for tests to trust a known test certificate (the `check_fn_with_client` shape of #2089), without changing
  production certificate validation.
- A regression test that fails before the change and passes after it, and manual verification against a real TLS
  REST API.

### Out of Scope

- Changing how the health check API aggregates or reports results, or the report schema.
- Production trust anchors for health checks (the same out-of-scope decision as #2089).
- The `Ok(<any status>)` semantics (the separate observation above).
- The health check API server's own registration (not checked).

## Architectural Decisions

No new decision expected; the HTTP tracker's decision in #2089 applies. If the fix needs a shared helper for the URL
(the HTTP tracker and the REST API now repeat it), decide whether it belongs in `torrust-server-lib` (an extracted
repository) or stays in each server; do not change the extracted crates in this issue without the maintainer.

## Design and Ownership Review

The check belongs to the server that registers it: `axum-rest-api-server` owns its own health endpoint path
(`/api/health_check`) and `server.rs` owns the registration, so the fix is local to that file. The registry and the
report belong to `torrust-server-lib` and `axum-health-check-api-server` and do not change.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

1. Analysis: hard-coded `http://` in `check_fn` against a binding that says `https`.
2. Reproduction with real artifacts: the TLS server, the registered check and the HTTPS answer above; repeat it by hand
   with the real binary and a TLS REST API configuration (manual scenario below).
3. Regression-test boundary: see the next section.
4. Red evidence: the regression test failing before the change.
5. Fix: derive the URL from the service binding.
6. Green evidence and a like-for-like recheck of the reproduction.

## Regression Test Strategy

Smallest deterministic boundary, two layers, both in `packages/axum-rest-api-server/src/server.rs`:

- A unit test of the URL builder (a pure function of a `ServiceBinding`): an HTTP binding gives
  `http://127.0.0.1:<port>/api/health_check`, an HTTPS binding gives `https://...`. Fails now because the scheme is
  fixed.
- One test that starts the token-aware server with a TLS launcher, takes the registered service binding from the `Registar`
  and runs `check_fn_with_client` on it, expecting `Ok("200 OK")` with a client that trusts the test certificate. (The
  registered function pointer itself uses the default client, so it is covered by the URL test and the manual recheck.) It protects the integration of the binding,
  the TLS listener and the probe.

The TLS test needs a certificate. **Decision (maintainer, 2026-11-04):** copy the existing self-signed test certificate
and key (valid until August 2036) from `packages/axum-health-check-api-server/tests/fixtures/`
(`https-health-check-cert.pem`, `https-health-check-key.pem`) into this package's test fixtures. No new dependency. The S5
slice of #2637 uses the same answer for its TLS startup tests.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                   | Notes / Expected Output                                                                 |
| --- | ------ | -------------------------------------- | --------------------------------------------------------------------------------------- |
| T1  | DONE   | Reproduce and record the evidence      | Real binary with a TLS REST API; the health report shows the REST API entry failing.   |
| T2  | DONE   | Write the failing regression tests     | URL builder test and the TLS registered-binding test, red before the fix.               |
| T3  | DONE   | Build the URL from the service binding | Add a client-taking variant for tests, as in #2089; production validation unchanged.    |
| T4  | DONE   | Verify green and recheck               | Tests green; the manual scenario now shows the REST API entry healthy.                  |

## Commit Points

Signed Conventional Commits with the issue number. As implemented: `refactor(axum-rest-api-server): [#2726] ...` (the URL
seam and the fixtures, behavior unchanged), then `fix(axum-rest-api-server): [#2726] check HTTPS REST APIs over TLS`
(the regression tests, the fix and the evidence). The red run was measured on the seam commit plus the new tests, before
the fix; a red test cannot be committed alone because the pre-commit script runs the tests.

## Progress Tracking

### Workflow Checkpoints

- [x] Draft written from a finding of #2637 S5.
- [x] GitHub issue created and linked (#2726).
- [x] Implementation (PR pending).

### Progress Log

- 2026-11-04 16:10 UTC - GitHub Copilot - Draft written. Found while planning the `server.rs` slice of #2637; reproduced
  with a throwaway test (see Evidence); impact checked by reading the health check API and the container `HEALTHCHECK`.
- 2026-11-04 18:00 UTC - Claude Sonnet 5.5 - Reproduced with the scratch test and with the real binary (see Real-binary
  reproduction); the maintainer chose to copy the existing test certificate fixture.
- 2026-11-05 08:33 UTC - Claude Sonnet 5.5 - GitHub issue #2726 created; the draft moved to `docs/issues/open/`.
- 2026-11-05 12:23 UTC - Claude Sonnet 5.5 - Implemented: fixtures and the URL seam, then the two regression tests with the fix.
  Red and green output and the real-binary recheck are in `manual-verification-evidence.md`.
- 2026-11-05 12:25 UTC - Claude Sonnet 5.5 - Independent pre-PR review: no blocking findings; reworded AC1, T2 and the commit points to match
  what the tests and history show.
- 2026-11-05 12:29 UTC - Claude Sonnet 5.5 - The pre-push hook (workspace tests) failed the TLS test: with more than one Rustls provider
  enabled in the workspace, Rustls cannot pick one. The test now installs `ring` once, as the health check API tests do.
  This adds `rustls` (`default-features = false`, feature `ring`, version 0.23, the version already in `Cargo.lock`) as a
  dev-dependency of this package; no new crate enters the lockfile. The certificate decision (copy the fixture, no
  generator dependency) is unchanged.
- 2026-11-05 13:07 UTC - Claude Sonnet 5.5 - The `Container` workflow failed the TLS test: it read the fixtures from a path fixed at compile time
  (`/build/src/...`), which does not exist where the container test stage runs the tests. The test now embeds the fixtures with
  `include_bytes!`, as the health check API test does.

## Acceptance Criteria

- [x] AC1: With TLS enabled the REST API registers a health check that probes `https://` and, run with a client that
      trusts the test certificate, succeeds for a healthy server (automatic test); the registered function with a trusted
      certificate is checked manually (V5).
- [x] AC2: With TLS disabled the check still probes `http://` and succeeds.
- [x] AC3: A regression test failed before the change and passes after it; the output is recorded.
- [x] AC4: Production certificate validation is unchanged.
- [x] AC5: The health report of a real tracker with a TLS REST API lists the REST API entry as healthy when the host
      trusts the certificate (see `manual-verification-evidence.md`, V5).

## Verification Plan

### Automatic Checks

`cargo test -p torrust-tracker-axum-rest-api-server` (the TLS test is
`it_should_report_a_tls_rest_api_as_healthy_when_the_check_trusts_the_test_certificate`), `linter all`, `./contrib/dev-tools/git/hooks/pre-commit.sh` and
`./contrib/dev-tools/git/hooks/pre-push.sh`.

### Manual Verification Scenarios

1. Run the tracker (skill `run-tracker-locally`) with `[http_api]` TLS configured and a self-signed certificate.
2. `curl -sk https://<api>/api/health_check` answers `200`.
3. Fetch the health check API report (`/health_check` of the health check API) and read the REST API entry: on `develop`
   it is an error for `http://...`; after the fix it is healthy for `https://...`.
4. Repeat with TLS disabled and confirm no regression.

## Risks and Trade-offs

- It overlaps with #2637 S5 (`server.rs`): the fix touches `check_fn` and both add tests in the same test module.
- A certificate fixture or a new dev-dependency is a maintenance cost; decide it once for both issues.

## References

- Found by: #2637 (EPIC #1347 subissue), slice S5, ledger row 18 (`server.rs`)
- Precedent: #2089, `docs/issues/closed/2089-fix-https-tracker-health-check-protocol/ISSUE.md`
- Code: `check_fn` in `packages/axum-rest-api-server/src/server.rs`; `check_fn_with_client` and `health_check_url` in
  `packages/axum-http-server/src/server.rs`; `CheckReport::fail` in `packages/axum-health-check-api-server/src/resources.rs`
