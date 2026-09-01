---
doc-type: issue
issue-type: bug
status: done
priority: p2
epic: null
github-issue: 2089
spec-path: docs/issues/closed/2089-fix-https-tracker-health-check-protocol/ISSUE.md
branch: "2089-fix-https-tracker-health-check-protocol"
related-pr: 2093
last-updated-utc: 2026-09-01 10:27
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/closed/2089-fix-https-tracker-health-check-protocol/health-check-test-design.md
    - docs/issues/closed/2089-fix-https-tracker-health-check-protocol/tls-manual-test.md
    - packages/axum-http-server/src/server.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
---

<!-- skill-link: create-issue -->

# Issue #2089 - Fix HTTPS tracker health-check protocol

## Goal

Make the HTTP tracker health-check job probe a registered listener with the
same transport protocol as its `ServiceBinding`, so HTTPS listeners report
their real health status.

## Background

During manual verification for #2041, a TLS-enabled HTTP tracker successfully
bound as `https://0.0.0.0:60057/` and directly returned `{"status":"Ok"}` from
its `/health_check` endpoint. The aggregate health API correctly exposed that
HTTPS `service_binding`, its final socket address, and
`service_type="http_tracker"`, but reported an error for the service.

`packages/axum-http-server/src/server.rs` previously built every HTTP-tracker
health-check URL as `http://{binding}/health_check`. For an HTTPS registration,
this probes plain HTTP on the TLS port and fails. The issue was pre-existing and
outside #2041's registry-metadata scope.

## Scope

### In Scope

- Derive the HTTP tracker health-check URL scheme from `ServiceBinding`.
- Preserve ordinary HTTP health-check behaviour.
- Add focused URL-construction coverage for HTTP and HTTPS bindings.
- Add aggregate HTTPS health-report coverage using a known test certificate and
  a named, non-capturing trusted-test health-check callback.
- Keep certificate validation enabled. The test callback trusts only its known
  test certificate.

### Out of Scope

- Changing production TLS certificate loading or certificate validation policy.
- Adding configurable production trust anchors for health checks.
- Changing `torrust-server-lib` to store stateful closure callbacks.
- Changing the health API response schema.
- Changing runtime registry metadata or service identity behavior introduced by
  #2041.

## Architectural Decisions

The accepted test design is documented in
[`health-check-test-design.md`](health-check-test-design.md). It records the
rejected stateful-closure and production-configuration alternatives.

- Related ADRs: `docs/adrs/20260728115400_define_registar_as_runtime_service_registry.md`
- ADRs to create: None known. Create an ADR during implementation only if the
  work reveals a material architectural decision beyond the established
  `ServiceBinding` and TLS-validation conventions.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                            | Notes / Expected Output                                                                       |
| --- | ------ | ------------------------------- | --------------------------------------------------------------------------------------------- |
| T1  | DONE   | Add HTTPS URL regression        | Prove an HTTPS binding produces an `https://` probe URL, not `http://`.                       |
| T2  | DONE   | Derive URL from service binding | Use the binding's canonical URL without altering HTTP paths.                                  |
| T3  | DONE   | Add named trusted-test callback | The callback builds a `reqwest` client that trusts only the static loopback test certificate. |
| T4  | DONE   | Add aggregate HTTPS regression  | The aggregate report marks the operational HTTPS service `Ok`.                                |
| T5  | DONE   | Validate health-report behavior | HTTP-server package tests and health-check API integration tests pass.                        |
| T6  | DONE   | Document verification evidence  | Automated evidence and both manual scenarios are recorded.                                    |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] Implementation completed
- [x] Automatic verification completed (`linter all`, relevant tests, and pre-commit checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-31 14:00 UTC - agent - Drafted from the manual TLS verification observation in #2041. Awaiting user review before GitHub issue creation.
- 2026-08-24 00:00 UTC - user - Approved the draft specification.
- 2026-08-24 00:00 UTC - agent - Created GitHub issue #2089 and moved the approved specification to `docs/issues/open/`.
- 2026-08-24 00:00 UTC - agent and user - Rejected a stateful registry callback change for this focused issue; documented the named non-capturing test-callback alternative.
- 2026-08-24 17:42 UTC - agent - Added protocol-aware URL construction, a focused HTTP/HTTPS URL regression, and a TLS aggregate regression that trusts only the static loopback test certificate.
- 2026-08-24 17:42 UTC - agent - Verified `cargo test -p torrust-tracker-axum-http-server` (22 unit and 55 integration tests) and `cargo test -p torrust-tracker-axum-health-check-api-server --test integration` (8 tests).
- 2026-08-24 18:42 UTC - agent - Ran `linter all`; markdown, YAML, TOML, spell-check, Clippy, rustfmt, and ShellCheck all passed.
- 2026-08-24 - agent - Documented the static TLS fixture creation and manual verification process in [`tls-manual-test.md`](tls-manual-test.md).
- 2026-08-24 - agent - Generated a one-day local CA and loopback TLS leaf under `.tmp/` for M1. The platform trust store has no user-writable anchor location: `trust anchor` returned `p11-kit: no configured writable location to store anchors`. M1 is blocked pending a trusted local development certificate or administrator-installed trust anchor.
- 2026-08-24 - agent - Completed M2 with the default development configuration. `curl --fail --silent --show-error http://127.0.0.1:1313/health_check` returned `status: Ok`; HTTP tracker entries for `http://0.0.0.0:7070/` and `http://0.0.0.0:7171/` both returned `200 OK`.
- 2026-08-24 - user and agent - Unblocked M1 by installing the temporary CA in the platform trust store. The direct trusted HTTPS probe returned `{"status":"Ok"}`. The aggregate `http://127.0.0.1:1313/health_check` report returned `status: Ok` with `https://127.0.0.1:7443/`, an HTTPS `/health_check` probe URL, and `200 OK`.
- 2026-08-24 - agent - The pre-push all-features suite exposed ambiguous Rustls crypto providers in the HTTPS integration test. The test now explicitly installs the `ring` provider before TLS configuration; `cargo +stable test -p torrust-tracker-axum-health-check-api-server --test integration --all-features` passed (8 tests).
- 2026-08-25 - agent - Opened ready-for-review PR #2093 targeting `develop`.

## Acceptance Criteria

- [x] AC1: An HTTPS HTTP-tracker registration is health-checked through an `https://` URL, not an `http://` URL.
- [x] AC2: An operational HTTPS listener using the named trusted-test callback yields a successful entry in the aggregate health report.
- [x] AC3: Existing HTTP tracker health checks continue to pass.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass.
- [x] Manual verification scenarios are executed and documented (status + evidence).
- [x] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [x] Documentation is updated when behavior/workflow changes.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-axum-http-server`
- `cargo test -p torrust-tracker-axum-health-check-api-server --test integration`
- `linter all`
- Relevant pre-commit and pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                               | Command/Steps                                                                 | Expected Result                                     | Status | Evidence                                                                                               |
| --- | -------------------------------------- | ----------------------------------------------------------------------------- | --------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------ |
| M1  | Health-report HTTPS listener           | Start local TLS tracker with a certificate trusted by the health-check client | Health report has `Ok` for the HTTPS tracker entry. | DONE   | Direct TLS probe and aggregate report both returned `Ok`; aggregate HTTPS tracker result was `200 OK`. |
| M2  | Preserve HTTP listener health checking | Start ordinary local HTTP tracker                                             | HTTP tracker entry remains `Ok`.                    | DONE   | `http://127.0.0.1:1313/health_check` returned `Ok` with `200 OK` for both configured HTTP trackers.    |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                 |
| ----- | ---------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `server::tests::it_should_build_a_health_check_url_using_the_service_binding_protocol` passed in the HTTP-server package test suite.     |
| AC2   | DONE                   | `http::it_should_return_good_health_for_https_service_with_a_trusted_test_certificate` passed in the health-check API integration suite. |
| AC3   | DONE                   | Existing HTTP health-check aggregate test passed in the health-check API integration suite.                                              |

## Risks and Trade-offs

- `ServiceBinding` is the canonical source of transport; do not infer protocol
  from addresses or configuration fields.
- Default `reqwest` validation rejects the test's self-signed certificate. The
  named test callback adds exactly that certificate as a root rather than
  disabling validation.
- The named callback constructs a client per test probe. This is deliberate
  test-only simplicity; production continues using its default client.

## References

- Related issue: #2041
- Design record: [`health-check-test-design.md`](health-check-test-design.md)
- TLS fixture and manual verification: [`tls-manual-test.md`](tls-manual-test.md)
- Affected implementation: `packages/axum-http-server/src/server.rs`
- Local TLS workflow: `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`
