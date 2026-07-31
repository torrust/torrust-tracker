---
doc-type: issue
issue-type: bug
status: draft
priority: p2
github-issue: null
spec-path: docs/issues/drafts/fix-https-tracker-health-check-protocol.md
branch: "{issue-number}-fix-https-tracker-health-check-protocol"
related-pr: null
last-updated-utc: 2026-07-31 14:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/axum-http-server/src/server.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - Fix HTTPS tracker health-check protocol

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

`packages/axum-http-server/src/server.rs` currently builds every HTTP-tracker
health-check URL as `http://{binding}/health_check`. For an HTTPS registration,
this probes plain HTTP on the TLS port and fails. The issue was pre-existing and
outside #2041's registry-metadata scope.

## Scope

### In Scope

- Derive the HTTP tracker health-check URL scheme from `ServiceBinding`.
- Preserve HTTP tracker health-check behavior for ordinary HTTP listeners.
- Add regression coverage for HTTPS listener health checks.
- Establish a test strategy for a TLS certificate trusted by the health-check
  client, then verify the aggregate health API reports `Ok` for an operational
  HTTPS tracker.

### Out of Scope

- Changing production TLS certificate loading or certificate validation policy.
- Changing the health API response schema.
- Changing runtime registry metadata or service identity behavior introduced by
  #2041.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                      | Notes / Expected Output                                                                        |
| --- | ------ | ----------------------------------------- | ---------------------------------------------------------------------------------------------- |
| T1  | TODO   | Add failing HTTPS health-check regression | Prove an HTTPS registration is not probed as plain HTTP.                                       |
| T2  | TODO   | Derive check URL from service binding     | Use the binding's protocol and address without altering HTTP paths.                            |
| T3  | TODO   | Define trusted-TLS test strategy          | Use test-only client trust or a trusted test certificate; do not weaken production validation. |
| T4  | TODO   | Validate health-report behavior           | Aggregate report marks healthy local HTTP and HTTPS services `Ok`.                             |
| T5  | TODO   | Document verification evidence            | Record automated and manual results.                                                           |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-07-31 14:00 UTC - agent - Drafted from the manual TLS verification observation in #2041. Awaiting user review before GitHub issue creation.

## Acceptance Criteria

- [ ] AC1: An HTTPS HTTP-tracker registration is health-checked through an
      `https://` URL, not an `http://` URL.
- [ ] AC2: An operational HTTPS listener using a certificate trusted by the
      health-check client yields a successful entry in the aggregate health
      report.
- [ ] AC3: Existing HTTP tracker health checks continue to pass.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.
- [ ] Manual verification scenarios are executed and documented (status + evidence).
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior.
- [ ] Documentation is updated when behavior/workflow changes.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-axum-http-server`
- `cargo test -p torrust-tracker-axum-health-check-api-server --test integration`
- `linter all`
- Relevant pre-commit and pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                               | Command/Steps                                                                 | Expected Result                                     | Status | Evidence |
| --- | -------------------------------------- | ----------------------------------------------------------------------------- | --------------------------------------------------- | ------ | -------- |
| M1  | Health-report HTTPS listener           | Start local TLS tracker with a certificate trusted by the health-check client | Health report has `Ok` for the HTTPS tracker entry. | TODO   |          |
| M2  | Preserve HTTP listener health checking | Start ordinary local HTTP tracker                                             | HTTP tracker entry remains `Ok`.                    | TODO   |          |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |

## Risks and Trade-offs

- The direct service binding URL is the canonical source of transport. Avoid
  reintroducing protocol inference from addresses or configuration fields.
- A self-signed certificate works for a direct `curl --insecure` probe, but
  default `reqwest` validation rejects it. The implementation must not weaken
  production certificate validation to make the test pass.

## References

- Related issue: #2041
- Affected implementation: `packages/axum-http-server/src/server.rs`
- Local TLS workflow: `.github/skills/dev/environment-setup/run-tracker-locally/SKILL.md`
