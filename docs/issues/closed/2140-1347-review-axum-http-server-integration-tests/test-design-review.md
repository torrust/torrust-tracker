# Integration-Test Design Review

## Review Boundary

All 18 Rust files under `packages/axum-http-server/tests/` were reviewed. The test target contains
55 executable real-listener tests: 54 HTTP/lifecycle contracts and one lifecycle smoke test. The
remaining files are module wiring or test support. The package boundary is appropriate: tests
exercise an actual listener, Axum routing/extractors, HTTP client requests, bencoded responses,
and selected persisted state and statistics.

## Phase 1: Problems Identified

| Area                                  | Evidence                                                                                                                            | Effect                                                                                                                                         | Decision                                                                                                                                                                                                                           |
| ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Combined private-and-whitelisted mode | `configured_as_private_and_whitelisted.rs` contains two TODO-only modules.                                                          | The interaction and precedence of authentication, whitelist authorization, and scrape data hiding are not proven at the package HTTP boundary. | Resolved with six real-listener contracts using one file-local `PrivateListedTracker` scenario fixture.                                                                                                                            |
| Misleading IPv6 scenario              | `a_loopback_ipv6_client_uses_the_external_ip_when_ip_is_absent` configures an IPv6 external IP but binds its client to `127.0.0.1`. | The name inaccurately claims IPv6 client coverage.                                                                                             | Resolved: the tracker and client now use IPv6, with an IPv6 loopback availability guard.                                                                                                                                           |
| Repeated test bootstrap               | Most tests repeat logging setup, config extraction, `Arc` wrapping, environment creation, and teardown.                             | Boilerplate is substantial.                                                                                                                    | Do not introduce a generic factory: local configuration choice is the causal state and must remain visible. Consider only narrowly scoped scenario fixtures when combined-mode tests make multiple setup steps obscure that state. |
| Support and legacy clutter            | `common/http.rs` has no call sites; `server/requests/mod.rs` and `server/responses/mod.rs` are migration notices only.              | Test tree contains unused or non-contract support.                                                                                             | Remove only as a separately reviewed cleanup after confirming no intended near-term use.                                                                                                                                           |
| Minor stale annotations               | `should_fail_when_the_request_is_empty` has `#[allow(dead_code)]`; `assert_empty_announce_response` is unused.                      | Unnecessary suppression/unused helper masks test-code maintenance signals.                                                                     | Remove in a small cleanup increment if compilation remains clean.                                                                                                                                                                  |
| Naming and assertion scope            | `for_all_config_modes` is a shared-contract grouping, not a literal matrix; whitelist tests additionally assert log content.        | Names can mislead; log text is more brittle than the HTTP/bencoded contract.                                                                   | Do not rename the tree or weaken existing observability assertions in this issue without a distinct approved rationale. Do not add new log-text contracts.                                                                         |

## File-by-File Inventory

| Source                                     | Current contracts or role                                                                                                                   |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------- |
| `integration.rs`                           | Test crate wiring and stopped-clock alias; no contract.                                                                                     |
| `common/mod.rs`                            | Module wiring.                                                                                                                              |
| `common/fixtures.rs`                       | Shared malformed info hashes and random unlisted hash fixture.                                                                              |
| `common/http.rs`                           | Unused query helper; no call sites.                                                                                                         |
| `server/mod.rs`                            | Module wiring.                                                                                                                              |
| `server/asserts.rs`                        | Shared HTTP-200 and bencoded announce/scrape/error response assertions.                                                                     |
| `server/requests/mod.rs`                   | Legacy migration notice only.                                                                                                               |
| `server/responses/mod.rs`                  | Legacy migration notice only.                                                                                                               |
| `server/v1/mod.rs`                         | Module wiring.                                                                                                                              |
| `server/v1/contract/mod.rs`                | Environment start/stop smoke test.                                                                                                          |
| `for_all_config_modes/mod.rs`              | Health endpoint returns HTTP 200, JSON media type, and `Report::Ok`.                                                                        |
| `receiving_an_announce_request.rs`         | Public/default announce parsing, protocol errors, response encodings, peer selection/state, stats, external IP, and reverse-proxy behavior. |
| `receiving_an_scrape_request.rs`           | Public scrape parsing, response counts, multiple hashes, and stats.                                                                         |
| `and_running_on_reverse_proxy.rs`          | Missing/invalid X-Forwarded-For announce errors.                                                                                            |
| `configured_as_private.rs`                 | Private announce authentication and private scrape visibility.                                                                              |
| `configured_as_whitelisted.rs`             | Listed announce authorization and listed scrape visibility.                                                                                 |
| `configured_as_private_and_whitelisted.rs` | Six private-and-listed announce/scrape interaction contracts, using one focused scenario fixture.                                           |
| `using_ipv6_v6only.rs`                     | IPv6-only listener health reachability.                                                                                                     |

## Phase 2: Proposed Refactorings and Test Increments

Ordered from high-impact/low-effort to lower-impact/higher-effort. Each item remains a proposal
until maintainer approval, and only one approved item will be implemented at a time.

1. **Completed: add a six-scenario combined-mode contract matrix** in
   `configured_as_private_and_whitelisted.rs`. For announce: unauthenticated/unlisted returns the
   authentication failure (authentication precedes authorization); authenticated/unlisted returns
   the whitelist failure; authenticated/listed succeeds. For scrape with existing peer data:
   unauthenticated/listed returns zeroed data; authenticated/unlisted returns zeroed data;
   authenticated/listed returns real data. Use a small scenario fixture only if it names the causal
   state—authentication plus whitelist membership—without hiding the HTTP Act or bencoded Assert.
2. **Completed: correct the IPv6 external-IP test** to bind an IPv6 loopback client and explicitly skip when
   IPv6 cannot be bound, matching the suite's portability approach. This is a correctness and
   naming repair, not a coverage increase.
3. **Deferred: remove dead test support and stale suppression.** The dormant `common/http.rs`
   helper remains by maintainer decision because it can document a future local query-construction
   pattern. Retain the legacy migration notes and make no cleanup change in this issue.
4. **Consider a narrow local fixture for repeated combined-mode setup only** if item 1 proves its
   repeated persistence/key/whitelist setup obscures causal state. Do not create a cross-file
   factory and do not migrate the existing suite preemptively.
5. **Defer broad directory renaming and logging-contract changes.** The benefit is low relative to
   churn and can be reconsidered in a dedicated test-organization or observability issue.

## Test-Method Assessment

| Test level            | Assessment                                                                                                                                                                |
| --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Unit tests            | Strong handler, extractor, route, and lifecycle seams already exist. Keep protocol-to-domain mapping and framework rejection details there.                               |
| Package integration   | Correct real-listener boundary is well represented; combined private-and-whitelisted behavior is the meaningful missing configuration interaction.                        |
| Examples              | No public example API needs this package-specific contract. No example is proposed.                                                                                       |
| E2E                   | Root composition/container E2E is outside this package boundary; no evidence requires it.                                                                                 |
| Mutation testing      | Narrow sample is practical and caught the tested compact-response mutation. No target or CI gate is proposed.                                                             |
| Property/fuzz testing | Parser input variation is chiefly owned by `http-protocol` and extractors, which already carry focused tests. No package-listener property/fuzz case has been identified. |
