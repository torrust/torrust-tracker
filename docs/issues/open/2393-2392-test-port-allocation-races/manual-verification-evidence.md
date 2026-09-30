---
doc-type: manual-verification-evidence
issue-spec: docs/issues/open/2393-2392-test-port-allocation-races/ISSUE.md
last-updated-utc: 2026-09-30 14:58
---

<!-- markdownlint-disable MD003 -->

# Manual Verification Evidence

## Purpose

Record the initial real-artifact reproduction and the like-for-like recheck required by the bug-fix
workflow. This draft contains observed results only; it does not claim a fix.

## Environment and Prerequisites

- Date and time (UTC): 2026-09-30 13:15 to 13:29
- Artifact under test: `develop` at `4d64d3c34` (merge of PR #2390). Its only differences from
  `9d840f55d` (merge of PR #2388) are documentation files.
- Operating system: Linux workstation with 32 logical processors.
- Test runner: `cargo-nextest 0.9.88 (b44b7adf7 2025-01-15)`.
- Command configuration: default nextest concurrency, `--no-fail-fast` for repeated runs. Logs are
  retained locally under `.tmp/flaky/` and are not versioned.

## Verification Processes

### V1 - Original parallel-suite reproduction

- Goal: observe the intermittent failure against a real test artifact before any code change.
- Current hypothesis: tests either release a port selected by a port-`0` bind before the tested
  service binds it, or use a fixed port inside the system ephemeral range. Parallel tests can then
  claim the same address.
- Status: `REPRODUCED`

#### Steps Performed

1. Built the `axum-http-server` test binaries:

   ```sh
   cargo nextest run -p torrust-tracker-axum-http-server --no-run
   ```

2. Ran the two duplicate-registration tests 200 times:

   ```sh
   cargo nextest run -p torrust-tracker-axum-http-server --no-fail-fast \
     -E 'test(/duplicate_binding_error_when_registration_fails|registration_fails$/)'
   ```

3. Ran the full `axum-http-server` package suite 100 times:

   ```sh
   cargo nextest run -p torrust-tracker-axum-http-server --no-fail-fast
   ```

4. Built the default-member tests and ran the full suite once, then 50 more times:

   ```sh
   cargo nextest run --tests --all-features --no-run
   cargo nextest run --tests --all-features --no-fail-fast
   ```

#### Observed Result

```text
Two duplicate-registration tests only: 200 passed, 0 failed
axum-http-server package suite: 100 passed, 0 failed
full-suite run 1: 1147 passed, 1 failed
  torrust-tracker::cli-configuration
  base_source_precedence::it_should_select_the_cli_configuration_when_both_child_environment_base_sources_are_set
  tracker startup failed because the configured health-check API listener address was already in use
full-suite runs 2-51: 49 passed, 1 failed
  torrust-tracker-axum-http-server
  server::tests::it_should_release_the_listener_and_preserve_the_duplicate_binding_error_when_registration_fails
  HTTP listener should be released after registration failure: Address already in use (os error 98)
```

The CLI test configures the health-check API to `127.0.0.1:43152`; the Linux ephemeral range on
this host includes that port. The duplicate-registration fixture chooses a port by binding
`127.0.0.1:0`, reading its address, then dropping the listener before the tested `HttpServer`
binds it. Both sequences allow a concurrent test to claim the address.

#### Conclusion

Reproduced. The two narrow scopes did not fail, while the full parallel suite independently
reproduced both port-allocation failure modes. The exact failure observed during #2298's Docker run
is the second failure above, at the same `axum-http-server` assertion.

### V2 - Duplicate-registration repair

- Status: `TODO`

### V3 - CLI configuration repair

- Status: `TODO`

### V4 - Full recheck

- Status: `TODO`

## Failures and Follow-up

- No production listener failed. The affected behavior is test address ownership and test-only
  configuration.
- The source inventory found other port-selection fixtures in `axum-rest-api-server`,
  `axum-health-check-api-server`, `udp-server`, and bootstrap tests. Their disposition belongs to
  T5; do not assume they are unsafe merely because they bind port `0`.
