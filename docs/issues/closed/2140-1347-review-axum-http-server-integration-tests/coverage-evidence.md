# Coverage Evidence

## Measurement

Command run on 2026-09-04:

`cargo llvm-cov -p torrust-tracker-axum-http-server --all-features --json`

The command ran all package unit tests and the `integration` target: 34 unit tests and 55
integration tests passed. The figures below include package production source covered by both test
levels. Therefore, they navigate potential gaps but do not measure integration tests in isolation
or prove an observable HTTP contract is complete.

## Package-Source Baseline

| Production source                         |     Covered lines |   Coverage |
| ----------------------------------------- | ----------------: | ---------: |
| `src/lib.rs`                              |             5 / 5 |    100.00% |
| `src/server.rs`                           |         321 / 365 |     87.95% |
| `src/testing/environment.rs`              |         110 / 111 |     99.10% |
| `src/v1/extractors/announce_request.rs`   |           60 / 60 |    100.00% |
| `src/v1/extractors/authentication_key.rs` |           70 / 84 |     83.33% |
| `src/v1/extractors/client_ip_sources.rs`  |           13 / 14 |     92.86% |
| `src/v1/extractors/scrape_request.rs`     |           67 / 67 |    100.00% |
| `src/v1/handlers/announce.rs`             |         409 / 414 |     98.79% |
| `src/v1/handlers/health_check.rs`         |             4 / 4 |    100.00% |
| `src/v1/handlers/scrape.rs`               |         339 / 341 |     99.41% |
| `src/v1/routes.rs`                        |         128 / 137 |     93.43% |
| **Aggregate**                             | **1,526 / 1,602** | **95.26%** |

## Current Measurement

The same command was rerun on 2026-09-06 after adding the six combined private-and-listed
listener contracts and correcting the IPv6 loopback-client scenario. All 34 unit tests and 61
integration tests passed. Package-source coverage remains **1,526 / 1,602 lines (95.26%)**.

The unchanged percentage is expected: the additions prove configuration interaction and correct an
existing scenario's network boundary; they do not target previously uncovered production lines.
This confirms that the selected tests were behavior-driven rather than percentage-driven.

## Interpretation and Candidate Gaps

- The lowest coverage is in lifecycle/bootstrap (`server.rs`) and authentication-key extraction.
  The former includes startup and registration failure paths deliberately covered at the unit
  boundary; the latter already has focused extractor and invalid-key response unit coverage. No
  additional real-listener test is selected solely to raise either percentage.
- Announce and scrape handlers are already near-completely exercised by unit and integration
  tests. This does not cover their interaction when **both** private authentication and whitelist
  authorization are active: the corresponding integration-test module has no executable tests.
- `AnnounceService::handle_announce` authenticates before authorizing; `ScrapeService::handle_scrape`
  first hides data for unauthenticated private requests, then delegates authenticated requests to
  whitelist-aware scrape handling. The observable combined-mode ordering and visibility contracts
  are not evidenced at the HTTP listener boundary.
- The enabled query-IP policy is explicitly deferred until the v3.0.0 schema is runtime-active.
  It remains outside this issue rather than becoming a coverage-driven test addition.

## Bounded Mutation-Testing Assessment

`cargo-mutants 27.0.0` is installed and can run in this workspace. An initial filter aimed at the
error-variant name produced no generated mutants; this was a filter-discovery result, not a test
result. A subsequent bounded sample used one generated branch mutation:

`cargo mutants --no-config --in-place --baseline run --package torrust-tracker-axum-http-server --file packages/axum-http-server/src/v1/handlers/announce.rs --re 'replace == with != in build_response' --test-package torrust-tracker-axum-http-server --timeout 180 --output .tmp/2140-cargo-mutants --colors never --no-times`

The baseline without mutation passed and the one mutant was caught. The mutation inverts compact-response
selection in `build_response`; existing normal and compact response assertions are sufficiently
strong to detect it. No behavior-relevant surviving mutant was found in this one-mutant sample.

The tool is practical for narrow, targeted checks, but this result is not a mutation score and is
not evidence for a CI gate. A full package mutation run would create a separate, potentially large
backlog and is deferred unless separately approved.
