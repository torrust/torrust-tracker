---
doc-type: coverage-evidence
issue: 1348
package: torrust-tracker-axum-http-server
measured-commit: 312ad07e
measured-utc: 2026-09-02
---

# Coverage Evidence

This document records the reproducible coverage baseline and the latest detailed package-source
report for Issue #1348. Aggregate percentages are only a navigation aid: use the per-file results
and uncovered-function locations to decide which critical behavior needs more coverage.

## Measurement Method

The report was generated at commit `312ad07e` with:

```text
cargo llvm-cov -p torrust-tracker-axum-http-server --all-features --json
```

The tables sum the JSON `summary` objects for files below `packages/axum-http-server/src/`. This
includes test code, so it is not a production-only coverage measure. The full machine-readable
JSON is intentionally not versioned because it is a generated 66 MB artifact; rerun the command
above to inspect line- and region-level detail for the measured revision.

## Package Coverage Comparison

| Measurement                           |                  Lines |                Regions |          Functions |
| ------------------------------------- | ---------------------: | ---------------------: | -----------------: |
| Baseline (before Issue #1348 changes) | 1,153 / 1,229 (93.82%) | 1,616 / 1,763 (91.66%) | 137 / 153 (89.54%) |
| Latest (commit `312ad07e`)            | 1,387 / 1,460 (95.00%) | 1,915 / 2,057 (93.10%) | 163 / 179 (91.06%) |

## Latest Detailed File Report

Files are ordered by ascending line coverage to highlight likely follow-up candidates.

| Source file                           |              Lines |            Regions |         Functions | Coverage interpretation                                                                                          |
| ------------------------------------- | -----------------: | -----------------: | ----------------: | ---------------------------------------------------------------------------------------------------------------- |
| `v1/extractors/authentication_key.rs` |   36 / 46 (78.26%) |   53 / 62 (85.48%) |    7 / 8 (87.50%) | Lowest line coverage; review path-extraction and error-mapping branches.                                         |
| `server.rs`                           | 262 / 306 (85.62%) | 428 / 524 (81.68%) |  23 / 35 (65.71%) | Lowest function and region coverage; prioritize lifecycle, TLS, health-check, and startup-failure paths by risk. |
| `v1/extractors/client_ip_sources.rs`  |   13 / 14 (92.86%) |   20 / 21 (95.24%) |   2 / 2 (100.00%) | Small residual branch gap.                                                                                       |
| `v1/routes.rs`                        | 128 / 137 (93.43%) | 209 / 223 (93.72%) |  14 / 17 (82.35%) | Request-layer branches remain partially uncovered.                                                               |
| `v1/handlers/announce.rs`             | 410 / 415 (98.80%) | 470 / 477 (98.53%) | 48 / 48 (100.00%) | Response adapter and handler error paths have strong package-level coverage.                                     |
| `v1/handlers/scrape.rs`               | 292 / 295 (98.98%) | 413 / 422 (97.87%) | 31 / 31 (100.00%) | Response adapter and handler error paths have strong package-level coverage.                                     |
| `testing/environment.rs`              | 110 / 111 (99.10%) | 128 / 134 (95.52%) | 17 / 17 (100.00%) | Remaining regions are test-environment alternatives.                                                             |
| `lib.rs`                              |    5 / 5 (100.00%) |    6 / 6 (100.00%) |   1 / 1 (100.00%) | Fully covered.                                                                                                   |
| `v1/extractors/announce_request.rs`   |  60 / 60 (100.00%) |  86 / 86 (100.00%) |   8 / 8 (100.00%) | Fully covered.                                                                                                   |
| `v1/extractors/scrape_request.rs`     |  67 / 67 (100.00%) |  98 / 98 (100.00%) | 10 / 10 (100.00%) | Fully covered.                                                                                                   |
| `v1/handlers/health_check.rs`         |    4 / 4 (100.00%) |    4 / 4 (100.00%) |   2 / 2 (100.00%) | Fully covered.                                                                                                   |

## Uncovered Function Areas

The coverage tool identifies the following source areas with at least one uncovered function.
These locations are a review queue, not a requirement to test every implementation detail.

| Area                                  | Uncovered function locations                                                             | Follow-up focus                                                                                 |
| ------------------------------------- | ---------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- |
| `server.rs`                           | 116, 117, 131, 140, 142, 155, 247, 252, 274, 281, 286, 323, 325, 329, 345, 354, 359, 369 | Server launch variants, registration/startup cleanup, stop flow, and health-check client paths. |
| `testing/environment.rs`              | 30, 45, 79, 89, 121, 133, 154, 164, 180, 209                                             | Test environment construction, lifecycle alternatives, and initialization.                      |
| `v1/extractors/authentication_key.rs` | 65, 77, 78, 90, 111                                                                      | Key parsing and extraction-rejection mapping.                                                   |
| `v1/routes.rs`                        | 142, 160                                                                                 | Request-layer composition and middleware branches.                                              |
| `v1/extractors/announce_request.rs`   | 52, 53                                                                                   | Axum extractor trait entry point; parser behavior is fully covered.                             |
| `v1/extractors/scrape_request.rs`     | 52, 53                                                                                   | Axum extractor trait entry point; parser behavior is fully covered.                             |
| `v1/extractors/client_ip_sources.rs`  | 58, 59                                                                                   | Axum extractor trait entry point.                                                               |
| `v1/handlers/announce.rs`             | 25, 29, 38, 43, 53, 59                                                                   | Public Axum handler entry points and delegation; focused adapter and error behavior is covered. |
| `v1/handlers/scrape.rs`               | 25, 29, 40, 45, 51, 57                                                                   | Public Axum handler entry points and delegation; focused adapter and error behavior is covered. |
| `v1/handlers/health_check.rs`         | 5                                                                                        | Handler entry point.                                                                            |

## Coverage Decision

Issue #1348 closes the highest-value direct gaps: announce and scrape response adaptation,
request-ID middleware behavior, and registration-failure listener release. Future work should
consider `server.rs` and authentication-key extraction first, but only where a stable,
behavior-focused package test provides regression value.
