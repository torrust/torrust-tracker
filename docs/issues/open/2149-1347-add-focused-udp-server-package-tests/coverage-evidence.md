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

### Receiver Test-Level Evidence

At commit `45bada2d`, clean separately collected reports show the following for
`packages/udp-server/src/server/receiver.rs`:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Aggregate/global | 55 / 56 (98.21%) | 77 / 79 (97.47%) | 7 / 7 (100.00%) | Broad progress only; it includes all selected package test binaries and test-only code. |
| Unit-only (`--lib`) | 55 / 56 (98.21%) | 77 / 79 (97.47%) | 7 / 7 (100.00%) | Direct queued-loopback test protects the package-owned datagram-to-`RawRequest` adapter. Before the increment, this scope covered 15 / 22 lines (68.18%) and 18 / 31 regions (58.06%). |
| Integration-only (`--test integration`) | 21 / 22 (95.45%) | 29 / 31 (93.55%) | 3 / 3 (100.00%) | Separately confirms real-loopback production-slice execution; it does not substitute for the direct unit contract. |

The reports have different denominators and are not combined. Pending readiness, receive-error,
and stream-termination branches remain at Tokio readiness, platform fault-injection, and #1488
receive-loop lifecycle boundaries; no mock socket abstraction or percentage-only test is selected.

### Banning Event-Handler Test-Level Evidence

At commit `d357db0a`, clean separately collected reports show the following for
`packages/udp-server/src/banning/event/handler.rs`:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Aggregate/global | 81 / 82 (98.78%) | 108 / 110 (98.18%) | 12 / 12 (100.00%) | Broad progress only; it includes all selected package test binaries and test-only code. |
| Unit-only (`--lib`) | 81 / 82 (98.78%) | 108 / 110 (98.18%) | 12 / 12 (100.00%) | Direct tests protect client-IP forwarding and post-update distinct tracked-IP gauge publication as separate focused handler contracts. |
| Integration-only (`--test integration`) | 28 / 29 (96.55%) | 31 / 33 (93.94%) | 4 / 4 (100.00%) | Separately confirms listener and real-loopback production-slice execution; it does not substitute for the direct unit contracts. |

The reports have different denominators and are not combined. Non-cookie event ignoring remains
covered at the listener boundary; repository failure is logging-only collaborator behavior;
threshold/reset/ban policy belongs to `udp-core` `BanService`; event reception and lifecycle belong
to the listener and #1488; and multi-listener/REST behavior belongs to root composition. No
coverage-only test is selected.

### Server States Test-Level Evidence

At commit `408938d1`, clean separately collected reports show the following for
`packages/udp-server/src/server/states.rs`:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Aggregate/global | 72 / 77 (93.51%) | 91 / 102 (89.22%) | 16 / 20 (80.00%) | Broad progress only; it includes all selected package test binaries and test-only code. |
| Unit-only (`--lib`) | 72 / 77 (93.51%) | 91 / 102 (89.22%) | 16 / 20 (80.00%) | Direct tests cover all `await_startup_notification` mappings. Remaining lines are the separate bind/public-start and #1488 lifecycle boundaries. |
| Integration-only (`--test integration`) | 27 / 37 (72.97%) | 16 / 32 (50.00%) | 7 / 11 (63.64%) | Separately exercises real startup/stop paths through package contracts; it does not substitute for focused unit tests. |

The reports have different denominators and are not combined. Remaining unit-only executable lines
are the bind-error conversion in `Server::<Stopped>::start`, halt/task error mappings in
`Server::<Running>::stop`, and the existing test's defensive fallback. Bind failure remains at the
`BoundSocket` and public-start boundary; `stop` remains #1488 lifecycle work; and the fallback is
not behavior to force through a test. No coverage-only socket/task fixture is selected.

### Handler Error Test-Level Evidence

At commit `496128da`, clean separately collected reports show the following for
`packages/udp-server/src/handlers/error.rs`:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Aggregate/global | 176 / 178 (98.88%) | 187 / 189 (98.94%) | 23 / 23 (100.00%) | Broad progress only; includes all selected package test binaries and test-only wrappers. |
| Unit-only (`--lib`) | 155 / 178 (87.08%) | 172 / 189 (91.01%) | 22 / 23 (95.65%) | Direct tests protect supplied and fallback response transaction IDs, error-event request-kind routing, and event public-URL forwarding. |
| Integration-only (`--test integration`) | 89 / 93 (95.70%) | 54 / 61 (88.52%) | 8 / 8 (100.00%) | Existing real-loopback contracts exercise a separate compiled production slice; they do not replace the direct unit contracts. |

The reports have different denominators and are not combined. Residual logging level and
transaction-ID-field paths are diagnostic detail rather than an observable handler contract, so
tracing capture is not selected. Sender-disabled event suppression is already a prerequisite of
the response tests but has no distinct observable output that warrants a collaborator-matrix test.
Protocol error conversion, dispatcher routing, error classification, and statistics/banning
consumption remain owned by `error.rs`, `handlers/mod.rs`, `event.rs`, and their specialized
event handlers/listeners, respectively. No coverage-only test is selected.

### Response-Sent Handler Test-Level Evidence

At the completed R1 increment, clean separately collected reports show the following for
`packages/udp-server/src/statistics/event/handler/response_sent.rs`:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Aggregate/global | 129 / 130 (99.23%) | 172 / 174 (98.85%) | 8 / 8 (100.00%) | Broad progress only; includes all selected package test binaries and test-only code. |
| Unit-only (`--lib`) | 122 / 130 (93.85%) | 149 / 174 (85.63%) | 8 / 8 (100.00%) | Direct `Ok { Connect }` handler test protects the successful connect processing-average route. Retained tests protect parent-dispatcher IPv4/IPv6 response-total routes. |
| Integration-only (`--test integration`) | 40 / 41 (97.56%) | 91 / 93 (97.85%) | 2 / 2 (100.00%) | Existing real-loopback contracts exercise a separate compiled production slice; they do not replace the direct unit contract. |

The reports have different denominators and are not combined. Error-response no-average behavior
is a negative collaborator/metric assertion and is not selected. Announce/scrape label
representation, metric aggregation/accessors, counter-write failure logging, parent routing, and
listener lifecycle remain owned by `event.rs`, `statistics/metrics.rs`, the repository/logging
boundary, the parent dispatcher, and the listener, respectively. No coverage-only test is
selected.

### Processor Test-Level Evidence

At the completed R1 increment, clean separately collected reports show the following for
`packages/udp-server/src/server/processor.rs`:

| Measurement scope | Lines | Regions | Functions | Interpretation |
| --- | ---: | ---: | ---: | --- |
| Aggregate/global | 122 / 122 (100.00%) | 175 / 175 (100.00%) | 19 / 19 (100.00%) | Broad progress only; includes all selected package test binaries and test-only code. |
| Unit-only (`--lib`) | 109 / 122 (89.34%) | 168 / 175 (96.00%) | 15 / 19 (78.95%) | Direct port-zero tests separately protect IPv4 response suppression, discard-event publication, and valid-connect handler bypass. |
| Integration-only (`--test integration`) | 34 / 34 (100.00%) | 20 / 20 (100.00%) | 7 / 7 (100.00%) | Existing real-loopback contracts exercise a separate compiled production slice; they do not replace the portable direct port-zero unit contracts. |

The reports have different denominators and are not combined. Normal handler/send behavior,
response serialization, socket failures, event consumption, logging, sender absence, launcher
admission, and lifecycle remain owned by handlers, `udp-protocol`, `BoundSocket`/integration,
specialized statistics handlers/listeners, the diagnostic boundary, and #1488, respectively. No
coverage-only or non-portable raw-socket test is selected.

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

Completed file-plan decisions cover request-buffer, event classification, parse-error conversion,
bound socket, handler dispatch, launcher admission, real-loopback contract, error metrics, and
container composition. The following remaining modules require one independently reviewable
unit-test assessment each; their unit-only figures come from the clean `--lib` report at the
container-plan checkpoint and are not replaced by aggregate/global or integration-only coverage.

| Issue task | Module | Unit-only coverage | Required assessment boundary |
| --- | --- | ---: | --- |
| T10 | `server/receiver.rs` | 15/22 lines (68.18%) | Assess a deterministic `Stream::poll_next` socket-adapter contract; defer if stable I/O control requires lifecycle redesign. |
| T11 | `statistics/event/handler/mod.rs` | 19/21 lines (90.48%) | Assess direct event dispatch only for routing gaps not already protected by individual handlers. |
| T12 | `banning/event/handler.rs` | 28/29 lines (96.55%) | Assess direct connection-cookie ban-counter/gauge behavior without listener lifecycle or ban-service internals. |
| T13 | `server/states.rs` | 45/54 lines (83.33%) | Assess deterministic state/registration behavior; retain #1488 ownership of shutdown and task lifecycle. |
| T14 | `handlers/error.rs` | 132/154 lines (85.71%) | Clean existing tests first, then assess response/error-event routing not already owned by adapters or handlers. |
| T15 | `statistics/event/handler/response_sent.rs` | 85/99 lines (85.86%) | Clean existing tests first, then assess one direct result/request-kind metric route. |
| T16 | `server/processor.rs` | 103/116 lines (88.79%) | Clean existing tests first, then assess direct processing behavior without receiver-loop or shutdown expansion. |
| T17 | `server/spawner.rs` | 17/17 lines (100.00%) | Record fully covered thin-wrapper and #1488 lifecycle deferral; do not add percentage-only coverage. |
| T18 | `statistics/mod.rs` | 52/52 lines (100.00%) | Assess metric-description composition ownership; do not add percentage-only coverage. |

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
