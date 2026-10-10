---
doc-type: coverage-evidence
issue: 1348
package: torrust-tracker-udp-core
measured-commit: 6eba68a1f
measured-utc: 2026-10-02T17:06:00Z
---

# UDP Core Coverage Evidence

This document records the clean package-source baseline and complete module inventory for Issue #1348. Reports were collected after spec-only PR #2407 merged, before any Issue #1348 test or production change.

## Measurement Scope

`cargo-llvm-cov 0.6.16` on stable Rust 1.98.1 generated two clean reports, filtered to `packages/udp-core/src/`. Raw JSON reports remain ignored local artifacts under `.tmp/1348-coverage/`.

```text
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-core --all-features --json --output-path .tmp/1348-coverage/aggregate.json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-core --all-features --lib --json --output-path .tmp/1348-coverage/unit.json
```

Both commands completed successfully and ran 43 package unit tests. The package has no `tests/` directory or package integration target, so no integration-only baseline exists. The aggregate command selected the same `src/lib.rs` unit-test binary as `--lib`; identical totals do not make aggregate coverage evidence of an independent test level.

## Test-Level Baseline

### Aggregate/Global Coverage

Aggregate coverage includes the selected default package test target and is broad navigation evidence only. It is not proof that a source seam has adequate unit coverage.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 975 / 1,345 (72.49%) | 1,488 / 1,894 (78.56%) | 129 / 181 (71.27%) |

### Unit-Only Coverage

Unit-only (`--lib`) coverage is the primary Issue #1348 metric.

| Lines | Regions | Functions |
| ---: | ---: | ---: |
| 975 / 1,345 (72.49%) | 1,488 / 1,894 (78.56%) | 129 / 181 (71.27%) |

### Integration-Only Coverage

Not measured. `torrust-tracker-udp-core` has no package integration test target at baseline. If a file plan selects one, this document records a separate clean `--test <name>` report; higher-level `udp-server` or root coverage is not substituted for unit coverage.

## Module Inventory And T1 Hypotheses

Every source file has one T1 hypothesis. It determines processing order but is not a terminal decision: every file still receives its own file test plan, and only that plan can conclude no test change. The property column means that the file plan must assess the named property; it does not require a property-testing dependency.

| Source module | Unit-only lines | Property candidate | Selected behavior or ownership boundary | T1 hypothesis |
| --- | ---: | --- | --- | --- |
| `connection_cookie.rs` | 130 / 135 (96.30%) | Yes: make/check round trip and inclusive validity bounds | Stateless cookie issuance and validation are a security boundary. Existing examples cover normal, expired, future, and realistic wrong-address cases; assess whether a property covers meaningful additional state. | Property candidate |
| `crypto/keys.rs` | 35 / 41 (85.37%) | Yes: encryption/decryption round trip | Current test-only cipher selection is covered. Assess the keeper contract and whether a cipher round-trip belongs here or at the cookie boundary. | Test candidate |
| `crypto/ephemeral_instance_keys.rs` | 5 / 10 (50.00%) | No | Random seed and cipher initialization are process-global; current uncovered paths are production initialization. | Probably no change |
| `services/announce.rs` | 0 / 101 (0.00%) | No | Cookie policy, whitelist authorization, tracker-core error adaptation, socket-derived peer identity, and event publication are package-owned. Tracker-core announce rules remain collaborator-owned. | Test candidate |
| `peer_builder.rs` | 0 / 14 (0.00%) | No | Maps wire announce fields to a domain peer, preserving the socket IP rather than a request-controlled value. | Test candidate |
| `services/scrape.rs` | 0 / 77 (0.00%) | No | Cookie policy, wire info-hash conversion, tracker-core error adaptation, and event publication are package-owned; scrape aggregation remains collaborator-owned. | Test candidate |
| `services/connect.rs` | 147 / 157 (93.63%) | No | Cookie issuance and connect-event publication have direct tests. Review optional-public-URL and absent-sender branches. | Probably no change |
| `services/banning.rs` | 60 / 66 (90.91%) | Yes: arbitrary error-count threshold | Exact per-IP counting and strict `count > limit` ban boundary are package-owned and ADR-governed. Existing examples cover the basic threshold and reset. | Property candidate |
| `event.rs` | 131 / 137 (95.62%) | No | Connection-context labels and public-URL projection are directly tested. Error-event coverage is deferred to the general error-events EPIC. | Probably no change |
| `container.rs` | 0 / 97 (0.00%) | No | Service composition propagates instance ID, public URL, external IP, event sender, and ban threshold. Do not use the production container as a test bootstrap. | Test candidate |
| `statistics/event/handler.rs` | 199 / 200 (99.50%) | No | Event-to-counter routing is already directly protected; inspect its one remaining uncovered branch. | Probably no change |
| `statistics/repository.rs` | 21 / 26 (80.77%) | No | Repository initialization, collection snapshot, and counter update are package-owned; metrics failures remain collaborator behavior. | Test candidate |
| `statistics/metrics.rs` | 56 / 65 (86.15%) | No | IPv4/IPv6 request-count projections are package-owned; numeric conversion review remains owned by #2244. | Test candidate |
| `statistics/services.rs` | 32 / 32 (100.00%) | No | Full statistics aggregation is already directly covered. | Probably no change |
| `statistics/event/listener.rs` | 124 / 145 (85.52%) | No | Event dispatch has direct tests. Spawned listener cancellation, receive-loop termination, and cleanup are lifecycle concerns owned by #1488. | Lifecycle (#1488) |
| `statistics/mod.rs` | 8 / 8 (100.00%) | No | Metric declaration composition is fully covered through specialized modules. | Probably no change |
| `lib.rs` | 5 / 10 (50.00%) | No | `ConnectionIdValidationPolicy` and static initialization are package wiring. Assess direct enum policy tests; process-global initialization needs no coverage-only test. | Probably no change |
| `services/mod.rs` | 22 / 24 (91.67%) | No | Test fixtures and module wiring support service plans rather than an independent runtime contract. | Test support only |
| `crypto/mod.rs` | No executable entries | No | Module declaration wiring has no independent observable contract. | Wiring only |
| `statistics/event/mod.rs` | No executable entries | No | Module declaration wiring has no independent observable contract. | Wiring only |

## Prioritized Gaps

1. `services/announce.rs`, `peer_builder.rs`, and `services/scrape.rs` have no package-local unit coverage despite owning request-validation and adaptation decisions on the announce/scrape critical path.
2. `container.rs` has no unit coverage for configuration propagation into package services.
3. `connection_cookie.rs`, `crypto/keys.rs`, and `services/banning.rs` need their natural property candidates assessed before deciding whether example-based tests are sufficient.
4. `statistics/repository.rs` and `statistics/metrics.rs` have focused package-owned gaps; they follow tracker-answering behavior because metrics defects do not change tracker responses.

## Per-File Results

Unit-only (`--lib`) file totals after each completed file plan, measured with the same clean command
as the baseline. Totals include in-file test modules, so denominators grow as tests are added.

| Source module | Baseline lines | Latest lines | Latest regions | Latest functions | Plan |
| --- | ---: | ---: | ---: | ---: | --- |
| `services/announce.rs` | 0 / 101 (0.00%) | 390 / 396 (98.48%) | 434 / 444 (97.75%) | 48 / 50 (96.00%) | [services-announce-tests.md](test-refactor-plans/services-announce-tests.md) |
| `peer_builder.rs` | 0 / 14 (0.00%) | 79 / 79 (100.00%) | 115 / 115 (100.00%) | 8 / 8 (100.00%) | [peer-builder-tests.md](test-refactor-plans/peer-builder-tests.md) |
| `services/scrape.rs` | 0 / 77 (0.00%) | 264 / 273 (96.70%) | 303 / 316 (95.89%) | 37 / 40 (92.50%) | [services-scrape-tests.md](test-refactor-plans/services-scrape-tests.md) |
| `connection_cookie.rs` | 130 / 135 (96.30%) | 171 / 171 (100.00%) | 372 / 376 (98.94%) | 22 / 22 (100.00%) | [connection-cookie-tests.md](test-refactor-plans/connection-cookie-tests.md) |
| `crypto/cookie_cipher.rs` | 49 / 49 (100.00%) after #2458 | 51 / 51 (100.00%) | 88 / 88 (100.00%) | 12 / 12 (100.00%) | [crypto-cookie-cipher-tests.md](test-refactor-plans/crypto-cookie-cipher-tests.md) |
| `services/connect.rs` | 147 / 157 (93.63%) | 108 / 114 (94.74%) | 113 / 120 (94.17%) | 13 / 15 (86.67%) | [services-connect-tests.md](test-refactor-plans/services-connect-tests.md) |

Package unit-only after `services/announce.rs` (stable Rust 1.99.0, rebased onto `develop` after
PR #2423): 1,324 / 1,591 lines (83.22%), 1,886 / 2,181 regions (86.47%), 171 / 211 functions
(81.04%). After `peer_builder.rs` (rebased onto `develop` again; no `udp-core` change): 1,392 /
1,656 lines (84.06%), 1,979 / 2,271 regions (87.14%), 178 / 218 functions (81.65%). After
`services/scrape.rs` (rebased onto `develop` again; no `udp-core` change): 1,656 / 1,852 lines
(89.42%), 2,282 / 2,509 regions (90.95%), 215 / 244 functions (88.11%). After reopening
`services/announce.rs` for R2h: 1,708 / 1,901 lines (89.85%), 2,340 / 2,564 regions (91.26%),
222 / 250 functions (88.80%). After `connection_cookie.rs` (rebased onto `develop` again; no
`udp-core` change): 1,749 / 1,937 lines (90.29%), 2,429 / 2,654 regions (91.52%), 229 / 257
functions (89.11%). After rebasing onto Issue #2458 (2026-10-09; stable Rust 1.99.0): 2,026 /
2,097 lines (96.61%), 2,706 / 2,794 regions (96.85%), 248 / 266 functions (93.23%). This jump
comes mostly from #2458, not from #1348: it deleted `crypto/keys.rs` and
`crypto/ephemeral_instance_keys.rs`, added a fully covered `crypto/cookie_cipher.rs` (49 / 49),
and added `container.rs` tests (0 / 97 to 183 / 183) and the static-initialization removal that
leaves `lib.rs` at 5 / 5. After `crypto/cookie_cipher.rs`: 2,028 / 2,099 lines (96.62%), 2,707 /
2,795 regions (96.85%), 250 / 268 functions (93.28%). After `services/connect.rs`: 1,929 / 2,000
lines (96.45%), 2,566 / 2,654 regions (96.68%), 242 / 260 functions (93.08%); the small drop comes
from removing covered test code, not from losing production coverage. The final
package comparison is recorded at T7.

## Next Evidence

- `test-refactor-plans/README.md` records shared guardrails and the plan index.
- A plan is created only when its source file becomes the single `IN_PROGRESS` ledger entry.
- `manual-verification-evidence.md`, `mutation-evidence.md`, and, when warranted, `implementation-retrospective.md` are created at their respective stages.
