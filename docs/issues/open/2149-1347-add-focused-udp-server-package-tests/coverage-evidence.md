---
doc-type: coverage-evidence
issue: 2149
package: torrust-tracker-udp-server
measured-commit: 2054d494
measured-utc: 2026-09-07
---

# UDP Server Coverage Evidence

This document records the package-source coverage baseline before Issue #2149 adds or changes
tests.

## Aggregate/Global Coverage Measurement

```text
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-server --all-features --json
```

The raw JSON report was generated at commit `2054d494` and filtered by files below
`packages/udp-server/src/`. Its 62 MB generated output is deliberately retained only in ignored
local temporary storage, not committed. The table below sums its file `summary` objects. It
includes all selected package test binaries and test-support code, so it is broad navigation
evidence rather than a production-only coverage measure, proof of behavioral completeness, or
evidence that unit coverage is sufficient.

## Unit-First Test-Level Coverage Policy

Aggregate package reports can combine unit and integration test binaries, hiding which boundary
executed a source seam. Unit tests are the default for package-owned behavior because they are fast,
deterministic, and close to the responsibility under test. Add or retain a package integration test
only when a unit test cannot protect the behavior at an appropriate boundary or the real-loopback
contract is clearer and more maintainable.

Do not decline a feasible deterministic package unit test because integration, example, root, or
end-to-end coverage already executes the behavior. When aggregate coverage informs a selected-seam
decision, record separate reports before claiming coverage ownership:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-server --all-features --lib --json
cargo llvm-cov clean --workspace
cargo llvm-cov -p torrust-tracker-udp-server --all-features --test integration --json
```

Do not compare percentages across those reports as a single total: unit reports include unit test
and test-support code while integration reports compile only the exercised package production slice.
Use them to identify the test level that protects each selected behavior.

### Test-Level Reporting Tables

Update aggregate/global and unit-only tables independently. Aggregate/global totals show broad
package progress; unit-only totals show whether the primary package-local objective is improving.
Integration-only evidence identifies distinct real-boundary protection and must never substitute for
a unit-only result.

### Selected-Seam Test-Level Evidence

At commit `9eb74c23`, the separate reports for `packages/udp-server/src/handlers/mod.rs` show:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Unit-only (`--lib`) | 184 / 214 (85.98%) | 224 / 249 (89.96%) | 32 / 37 (86.49%) | The direct `handle_packet` test executes the selected sendable parse-error routing seam. No executable source-line entries are uncovered in this report. |
| Integration-only (`--test integration`) | 31 / 31 (100.00%) | 18 / 18 (100.00%) | 5 / 5 (100.00%) | The real-loopback suite executes a separate compiled production slice; its smaller denominator excludes unit-test and test-support code. |
| Combined package report | 205 / 214 (95.79%) | 239 / 249 (95.98%) | 35 / 37 (94.59%) | Navigation-only aggregate; it must not be used to attribute the selected dispatcher coverage to unit or integration tests. |

The unit test is the appropriate primary boundary for sendable parse-error routing: it makes the
raw packet, dispatcher Act, returned request kind, and response transaction ID directly readable
without socket lifecycle or client/server mechanics. Integration tests remain valuable for actual
loopback transport behavior, but are neither needed nor used as evidence for this internal dispatch
contract.

## Aggregate/Global Package Coverage

| Measurement                   |                  Lines |                Regions |          Functions |
| ----------------------------- | ---------------------: | ---------------------: | -----------------: |
| Baseline before issue changes | 4,814 / 4,965 (96.96%) | 6,326 / 6,604 (95.79%) | 485 / 499 (97.19%) |
| Latest container-plan checkpoint | 5,423 / 5,548 (97.75%) | 7,088 / 7,340 (96.57%) | 551 / 565 (97.52%) |

## Unit-Only Package Coverage

The #2149 baseline predates the separated measurement policy, so no unit-only baseline exists. Do
not derive one from the aggregate baseline. Record the final unit-only package measurement here and
compare future unit-only measurements only with an equivalent unit-only command.

| Measurement | Lines | Regions | Functions |
| --- | ---: | ---: | ---: |
| Baseline before issue changes | Not measured separately | Not measured separately | Not measured separately |
| Latest container-plan checkpoint | 5,317 / 5,548 (95.84%) | 6,961 / 7,340 (94.84%) | 535 / 565 (94.69%) |

### Container Composition Test-Level Evidence

At commit `3c56dc45`, clean separately collected reports show the following for
`packages/udp-server/src/container.rs`:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Aggregate/global | 59 / 59 (100.00%) | 72 / 72 (100.00%) | 5 / 5 (100.00%) | Broad progress only; includes all selected package test binaries and test-only code. |
| Unit-only (`--lib`) | 59 / 59 (100.00%) | 72 / 72 (100.00%) | 5 / 5 (100.00%) | The direct R2 test protects the package-owned enabled sender-to-event-bus publication path. |
| Integration-only (`--test integration`) | 19 / 19 (100.00%) | 29 / 29 (100.00%) | 2 / 2 (100.00%) | Separately confirms higher-level execution of the compiled production slice; it does not substitute for the direct unit contract. |

The reports have different denominators and are not combined. The remaining composition details
are internal allocation or handle-cloning mechanics, generic events-package behavior, or root
consumer policy; no additional coverage-only container test is selected.

## Current Increment Coverage

The following measurement was taken after the completed request-buffer plan at commit `796e2a9e`.
It is an interim comparison, not the final Issue #2149 measurement; later file-plan increments can
change package totals and source-file denominators.

| Source file | Baseline lines | Current lines | Change | Baseline regions | Current regions | Change | Baseline functions | Current functions | Change |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| `server/request_buffer.rs` | 24 / 45 (53.33%) | 144 / 156 (92.31%) | +38.98 pp | 36 / 74 (48.65%) | 196 / 223 (87.89%) | +39.24 pp | 3 / 4 (75.00%) | 22 / 23 (95.65%) | +20.65 pp |

The added test code increases the measured denominator because package-source coverage includes
`#[cfg(test)]` code. The meaningful result is that the capacity-available, oldest-first eviction,
and buffer-drop cleanup contracts now execute deterministically. The remaining uncovered areas are
the intentionally untested scheduler-dependent incoming-task race guard and implementation details
not selected by the approved plan.

## Baseline Detailed File Report

Files are ordered by ascending line coverage so the table highlights the review queue. The issue
will prioritize meaningful package-owned behavior, not every uncovered line or framework branch.

| Source file                                     |               Lines |                Regions |         Functions | Coverage interpretation                                                                                                                  |
| ----------------------------------------------- | ------------------: | ---------------------: | ----------------: | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `server/request_buffer.rs`                      |    24 / 45 (53.33%) |       36 / 74 (48.65%) |    3 / 4 (75.00%) | Lowest package-owned seam; review normal capacity eviction, completed-handle removal, and drop cleanup without defining shutdown policy. |
| `event.rs`                                      |    18 / 30 (60.00%) |       25 / 49 (51.02%) |   3 / 3 (100.00%) | Event labels and display/conversion branches need behavior-based classification coverage.                                                |
| `error.rs`                                      |    23 / 32 (71.88%) |       22 / 31 (70.97%) |    4 / 5 (80.00%) | Error-kind conversion has a narrow deterministic unit-test seam.                                                                         |
| `server/bound_socket.rs`                        |    33 / 42 (78.57%) |       66 / 88 (75.00%) |    6 / 7 (85.71%) | Port-zero allocation and endpoint metadata are package-owned; avoid assuming universal dual-stack behavior.                              |
| `statistics/event/handler/error.rs`             |   86 / 106 (81.13%) |     103 / 173 (59.54%) | 11 / 11 (100.00%) | Review remaining error-category branches after more critical transport seams; do not add tests solely for region percentage.             |
| `server/launcher.rs`                            |  112 / 135 (82.96%) |     115 / 144 (79.86%) |  11 / 13 (84.62%) | Admission and task-management branches are valuable only if deterministic and not owned by pending shutdown work.                        |
| `server/states.rs`                              |    45 / 54 (83.33%) |       45 / 57 (78.95%) |  10 / 14 (71.43%) | Existing lifecycle error propagation is strong; defer cancellation redesign to the shutdown subissues.                                   |
| `handlers/mod.rs`                               |  157 / 166 (94.58%) |     175 / 185 (94.59%) |  30 / 32 (93.75%) | Packet dispatch and parse/handler-error conversion are candidate focused seams.                                                          |
| `server/receiver.rs`                            |    21 / 22 (95.45%) |       29 / 31 (93.55%) |   3 / 3 (100.00%) | Existing integration coverage exercises receive behavior; select a narrow socket-adapter test only if it demonstrates a clear gap.       |
| `banning/event/handler.rs`                      |    28 / 29 (96.55%) |       31 / 33 (93.94%) |   4 / 4 (100.00%) | Broadly covered; no percentage-driven expansion proposed.                                                                                |
| `lib.rs`                                        |    28 / 29 (96.55%) |       27 / 28 (96.43%) |   2 / 2 (100.00%) | Crate documentation/configuration wiring; no direct priority gap identified.                                                             |
| `statistics/event/handler/request_discarded.rs` |    33 / 34 (97.06%) |       33 / 35 (94.29%) |   4 / 4 (100.00%) | Existing behavior coverage is sufficient unless admission analysis identifies a missing observable fact.                                 |
| `statistics/event/handler/request_received.rs`  |    33 / 34 (97.06%) |       33 / 35 (94.29%) |   4 / 4 (100.00%) | Existing behavior coverage is sufficient unless admission analysis identifies a missing observable fact.                                 |
| `statistics/event/listener.rs`                  |  159 / 163 (97.55%) |     206 / 210 (98.10%) | 21 / 21 (100.00%) | Existing listener coverage is strong; lifecycle ownership changes are explicitly deferred.                                               |
| `statistics/event/handler/request_aborted.rs`   |    56 / 57 (98.25%) |       56 / 58 (96.55%) |   6 / 6 (100.00%) | Existing counter mapping is strong; event production remains an admission/overload review candidate.                                     |
| `statistics/event/handler/request_banned.rs`    |    56 / 57 (98.25%) |       56 / 58 (96.55%) |   6 / 6 (100.00%) | Existing counter mapping is strong; event production remains an admission review candidate.                                              |
| `banning/event/listener.rs`                     |  139 / 141 (98.58%) |     174 / 176 (98.86%) | 19 / 19 (100.00%) | Existing listener coverage is strong; shutdown ownership is deferred.                                                                    |
| `handlers/announce.rs`                          |  790 / 800 (98.75%) | 1,149 / 1,164 (98.71%) | 51 / 51 (100.00%) | Handler behavior is already well covered; avoid duplicating `udp-core` business-rule contracts.                                          |
| `statistics/event/handler/response_sent.rs`     |    98 / 99 (98.99%) |     141 / 143 (98.60%) |   6 / 6 (100.00%) | Existing behavior coverage is strong.                                                                                                    |
| `statistics/metrics.rs`                         |  827 / 834 (99.16%) | 1,228 / 1,238 (99.19%) |  86 / 87 (98.85%) | Metrics arithmetic is extensively covered; root integration retains multi-listener aggregation.                                          |
| `handlers/error.rs`                             |  153 / 154 (99.35%) |     144 / 145 (99.31%) | 14 / 14 (100.00%) | Individual error response behavior is strong; dispatch-to-error integration remains the relevant gap.                                    |
| `handlers/scrape.rs`                            |  319 / 321 (99.38%) |     385 / 390 (98.72%) |  27 / 28 (96.43%) | Handler behavior is strong; no percentage-driven expansion proposed.                                                                     |
| `statistics/event/handler/request_accepted.rs`  |  162 / 163 (99.39%) |     165 / 167 (98.80%) | 14 / 14 (100.00%) | Existing behavior coverage is strong.                                                                                                    |
| `server/mod.rs`                                 |  176 / 177 (99.44%) |     289 / 291 (99.31%) | 13 / 13 (100.00%) | Startup and registration-failure cleanup are already covered.                                                                            |
| `testing/environment.rs`                        |  183 / 184 (99.46%) |     197 / 201 (98.01%) | 20 / 20 (100.00%) | Do not expand lifecycle coverage until SI-14, SI-15, and SI-17 define ownership and cancellation.                                        |
| `statistics/repository.rs`                      |  547 / 549 (99.64%) |     771 / 773 (99.74%) |  61 / 62 (98.39%) | Existing repository and concurrency behavior coverage is strong.                                                                         |
| `container.rs`                                  |   19 / 19 (100.00%) |      29 / 29 (100.00%) |   2 / 2 (100.00%) | Indirect coverage exists; direct composition tests are optional and must demonstrate clearer regression value.                           |
| `handlers/connect.rs`                           | 251 / 251 (100.00%) |    285 / 285 (100.00%) | 18 / 18 (100.00%) | Existing connect behavior coverage is strong.                                                                                            |
| `server/processor.rs`                           | 116 / 116 (100.00%) |    157 / 157 (100.00%) | 17 / 17 (100.00%) | Portable direct source-port-zero defensive coverage is intentional; raw transport testing is not proposed.                               |
| `server/spawner.rs`                             |   17 / 17 (100.00%) |      17 / 17 (100.00%) |   2 / 2 (100.00%) | Existing lifecycle coverage is sufficient pending shutdown design.                                                                       |
| `statistics/event/handler/mod.rs`               |   21 / 21 (100.00%) |       50 / 52 (96.15%) |   2 / 2 (100.00%) | Module wiring; no direct priority gap identified.                                                                                        |
| `statistics/mod.rs`                             |   52 / 52 (100.00%) |      60 / 60 (100.00%) |   1 / 1 (100.00%) | Composition is covered; no percentage-driven expansion proposed.                                                                         |
| `statistics/services.rs`                        |   32 / 32 (100.00%) |      27 / 27 (100.00%) |   4 / 4 (100.00%) | Service aggregation behavior is covered.                                                                                                 |

## Prioritized Behavioral Review Queue

1. `server/request_buffer.rs`: establish the current normal-operation capacity, eviction, and
   drop cleanup contract without specifying the future shutdown policy.
2. `event.rs`, `error.rs`, and `handlers/mod.rs`: verify deterministic event/error classification
   and packet dispatch/error conversion where individual handler tests do not cover the boundary.
3. `server/bound_socket.rs`: verify stable port-zero and endpoint metadata behavior while treating
   IPv6/dual-stack availability as platform dependent.
4. `server/launcher.rs`: consider only a deterministic admission/event contract. Do not expand
   receive-loop, cancellation, or task-joining coverage before the #1488 UDP lifecycle subissues
   are approved and implemented.
5. `tests/server/contract.rs`: add a real-loopback test only when it proves a transport behavior
   that the preceding unit seams and existing package/root tests cannot express.

## Boundary and Deferral Decisions

| Test level or behavior         | Decision               | Rationale                                                                                                                                          |
| ------------------------------ | ---------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- |
| Unit tests                     | Selected               | Primary fast, deterministic boundary for the prioritized package-owned seams.                                                                      |
| Package integration            | Selected selectively   | Existing eleven real-loopback contracts are retained. Add only a stable wire contract with unique transport value.                                 |
| Runnable example               | Retained               | `examples/udp_only_public_tracker.rs` remains a consumer-facing smoke target; signal and awaited shutdown behavior belongs to SI-17.               |
| Root integration               | Retained externally    | Multi-listener metrics, shared banning, configuration ordering, and application composition are already correctly covered in root `tests/`.        |
| Container E2E                  | Deferred               | Outer interoperability protection is not the default mechanism for package coverage.                                                               |
| Mutation testing               | Bounded assessment     | Run only after approving a focused plan; record behavior-relevant survivors and do not create a score target or CI gate.                           |
| Property/fuzz testing          | Not currently selected | Protocol parsing/serialization properties belong primarily to `udp-protocol`; no server-orchestration invariant currently justifies a new harness. |
| Source-port-zero UDP transport | Deferred by constraint | Standard sockets cannot send it; existing direct processor coverage is the deepest portable automated boundary.                                    |
| Lifecycle shutdown             | Deferred               | SI-14, SI-15, and SI-17 own cancellation, request-task policy, joining, and standalone-environment lifecycle design.                               |
