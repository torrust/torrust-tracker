---
report-id: 2026-09-04_rest-api-token-timing
date-received: 2026-09-02
date-disclosed: 2026-09-04
status: hardened
severity: hardening
weakness: CWE-208 (observable timing discrepancy)
component: packages/axum-rest-api-server/src/v1/middlewares/auth.rs
fix-commit: 90d7a637
fix-pr: 2144
issue-spec: docs/issues/closed/2143-rest-api-constant-time-token-comparison/ISSUE.md
reported-by: Abdurazzoqov Javohir (GitHub abdurazzoqovjavohir700-dev)
review-cadence: on-recheck-condition
requires-recheck-when: the token comparison stops using subtle::ConstantTimeEq, a new secret-comparison site is added without it, or a practical remote timing recovery is demonstrated on a supported platform
---

# REST API access-token timing comparison

## Finding

The REST API authentication middleware compared a caller-supplied token with each configured
token using plain `==` and `Iterator::any`, either of which may exit early. The reporter
identified this by source review as a CWE-208 hardening gap: in principle, response timing
could reveal matching token prefixes. The reporter did not measure a timing difference and
also noted that the API did not rate-limit requests.

## Maintainer assessment

- Code path confirmed in `develop` and every released version through `v3.0.0-rc.1`.
- **Reproduction attempted — negative.** A release-build micro-benchmark (2 million iterations,
  32- and 128-byte tokens, x86-64/glibc) showed no position-dependent timing in plain `==`:
  no monotonic timing trend among mismatch offsets. glibc's `memcmp` compares inputs of this
  size with wide SIMD loads, leaving no observable prefix signal before network jitter.
- **Severity assigned: hardening (low).** A practical remote attack was not demonstrated and
  was not reproducible locally. The prior code nevertheless lacked a constant-time comparison
  contract, so this is tracked as a preventative hardening improvement rather than a confirmed
  vulnerability or CVE.
- The reporter's proposed `subtle` crate was independently vetted as untrusted input: it was
  already resolved in `Cargo.lock` at the same version/checksum via `sqlx`, has zero
  dependencies, is maintained by dalek-cryptography, and had no advisories. A std-only
  alternative offered weaker guarantees with no dependency-footprint benefit.

## Action taken

`authenticate` now uses `subtle::ConstantTimeEq::ct_eq` for each configured token, combines
all results with bitwise OR, and converts to `bool` only after evaluating every configured
token. Therefore, for a fixed supplied-token length, neither the matching prefix length nor
the position of a matching configured token changes the comparison work. Unit and integration
tests preserve authentication behaviour.

Token-length leakage, query-string token authentication, and REST API rate limiting are not
part of this change. Rate limiting remains a possible independent defense-in-depth feature.

This report also exposed missing maintainer workflow steps. The confidential-remediation
process now requires independent reproduction, maintainer-set severity, and vetting of
reporter-suggested dependencies before remediation.

## Recheck conditions

Reopen this report when the token comparison stops using `subtle::ConstantTimeEq`, a new
secret-comparison site is added without it, or a practical remote timing recovery is
demonstrated on a supported platform.

## Credit

Reported by **Abdurazzoqov Javohir** ([@abdurazzoqovjavohir700-dev](https://github.com/abdurazzoqovjavohir700-dev)),
who approved public credit.
