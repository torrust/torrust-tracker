---
semantic-links:
  skill-links:
    - write-markdown-docs
  related-artifacts:
    - docs/issues/closed/2089-fix-https-tracker-health-check-protocol/ISSUE.md
    - packages/axum-http-server/src/server.rs
    - packages/axum-health-check-api-server/tests/server/contract.rs
---

# HTTPS health-check test design

## Decision

Test the aggregate HTTPS health report with a named, non-capturing function
that builds a `reqwest::Client` trusting the known test certificate, then uses
an explicit HTTP-server helper that accepts that client. Register this named
function only in the HTTPS integration test.

Production continues to register `check_fn`, which uses normal `reqwest`
certificate validation and the system trust store.

## Problem

The bug fix makes `check_fn` derive `/health_check` from the protocol in the
registered `ServiceBinding`. An HTTPS binding must therefore be probed through
an HTTPS connection. The automated HTTPS listener uses a controlled self-signed
certificate so the test is deterministic. Default `reqwest` validation
correctly rejects that certificate.

The desired aggregate test must prove both that the probe uses HTTPS and that a
client explicitly trusting the test certificate receives `200 OK`.

## Initial proposal: stateful registry callback

The initial proposal was to modify `torrust-server-lib` so a registration could
store an `Arc<dyn Fn(&ServiceBinding) -> ServiceHealthCheckJob + Send + Sync>`.
The HTTPS test would build a certificate-trusting `reqwest::Client` once and
capture it in that closure.

This is technically valid and may be useful in a future independently scoped
library issue: stateful callbacks can carry client pools, timeouts, credentials,
or other immutable dependencies. It is not required for this bug.

### Why this proposal was rejected for #2089

- It expands a standalone public library API and requires release and tracker
  dependency-upgrade work for a focused one-line protocol defect.
- It changes the registry's callback model from explicit function pointers to
  trait objects, complicating public API documentation, cloning, and `Debug`.
- A captured client conceals the dependency at registration time. Although this
  is safe when designed well, a named function is more explicit for this test.
- It would make the scope substantially larger without increasing confidence in
  the URL-scheme fix.

## Considered production configuration alternative

Another proposal was to add an argument to `check_fn` that configures a custom
client for self-signed certificates, potentially as a production capability.

This was also rejected for #2089. The information does not belong in
`ServiceBinding`, whose responsibility is only a protocol and local socket
address. A production private-PKI feature would require an explicit trust-policy
configuration, validation, documentation, and security review. It must never
implicitly trust the tracker's own server certificate or disable certificate
validation. That is a separate feature, not a prerequisite for this bug fix.

## Accepted alternative

Add an explicit helper in `axum-http-server` that accepts a client:

```text
check_fn(service_binding)
  -> builds the ordinary default client
  -> check_fn_with_client(service_binding, client)
```

The HTTPS integration test defines a named callback with the existing registry
function-pointer signature:

```text
trusted_test_check_fn(service_binding)
  -> builds a client with the known test certificate as an additional root
  -> check_fn_with_client(service_binding, client)
```

The test callback contains no captured state, uses no global mutable state, and
is explicit at the test registration site. It is test-only. Certificate
validation remains enabled: only the exact known test certificate is added as a
trust anchor. The implementation must not use
`danger_accept_invalid_certs(true)`.

## Consequences

- No change or release is needed in `torrust-server-lib`.
- The test exercises the actual registry-to-health-API call path.
- Production behavior remains limited to normal system trust-store validation.
- A client is constructed for each test probe. This is acceptable for test code;
  a future production custom-client feature should instead build and reuse a
  configured client deliberately.
