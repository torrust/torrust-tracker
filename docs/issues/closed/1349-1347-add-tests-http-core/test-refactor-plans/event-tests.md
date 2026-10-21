---
doc-type: file-test-plan
issue: 1349
package: torrust-tracker-http-core
target-file: packages/http-core/src/event.rs
status: done
semantic-links:
  related-artifacts:
    - packages/http-core/src/event.rs
    - packages/http-core/src/statistics/event/handler.rs
    - docs/issues/closed/1349-1347-add-tests-http-core/coverage-evidence.md
---

# HTTP Core Event File Test Plan

Follow the shared [guidance](README.md). This plan covers only `packages/http-core/src/event.rs`. It reuses
the approach of the [announce](services-announce-tests.md) and [scrape](services-scrape-tests.md) plans.

## Current State

- **Unit-only coverage:** 157 / 187 lines (83.96%), 259 / 294 regions, 15 / 20 functions, measured at the
  S3 baseline (`develop` at the merge of #2586; the file did not change in S2 or S3). It was 174 / 187 at the
  S1 baseline. The difference is not production code: the `announce_events_match` test helper (lines
  216-246) lost its only caller when the announce tests moved to `RecordingSender` in S2, so its 18 lines are
  now uncovered. The totals include the test module.
- **Existing tests:** 7 in the file, in a `pub mod test` module: event equality, the `public_url` label, the
  client IP family for IPv4 and IPv6, and the client IP type for direct IPv4, native IPv6 and an
  IPv4-mapped IPv6 address.
- **Module-owned decisions:** the `Event` and `ConnectionContext` types that every HTTP service publishes
  and the statistics listener consumes; the projection of a `ConnectionContext` to the metric labels
  (`From<ConnectionContext> for LabelSet`), which is the Prometheus label contract of every HTTP tracker
  metric; the client IP family and type derived from the remote client address (not from the server
  binding); the public URL label that appears only when a URL is configured; and the public accessors.
  Client-address resolution is `http-protocol`'s; the label types are `torrust-metrics`'s; the binding is
  `torrust-net-primitives`'s.
- **T1 hypothesis:** probably a small change.

## Current Tests Review

Reviewed against `write-unit-test`. Conclusion: **refactor**.

| Test | Smells found |
| --- | --- |
| `events_should_be_comparable` | Tests two derives (`Clone` and `PartialEq`) with an unnamed behavior; no `// Arrange`, `// Act`, `// Assert`; builds two 10-line events inline. S2 and S3 rely on whole-event equality, which only holds if every part of the context takes part in it, but the test checks one part. |
| `connection_context_labels_should_include_the_configured_public_url_only_when_present` | Two contexts built by hand (about 12 lines); asserts one label pair and its absence; says nothing about the other seven labels. |
| the two `client_address_ip_family_*` tests and the three `client_address_ip_type_*` tests | Five near-identical 8-line bodies differing in one address; each repeats the same instance id, server binding and port; no AAA markers; names do not start with `it_should_`. |
| `announce_events_match` helper and its `pub mod test` module | Dead: no test calls the helper any more (S2), and the module is public only because the announce tests once imported it. Its `updated` field comparison is the reason the old announce tests needed it; the S2 announce tests freeze the clock instead. |

## Coverage and Decision Analysis

| Behavior group | Classification | Decision |
| --- | --- | --- |
| The full label set of a connection context (the `From<ConnectionContext> for LabelSet` impl) | 3A unit | R2a: a table of whole expected label sets written as literals (`server_binding_protocol`, `server_binding_ip`, `server_binding_address_ip_type`, `server_binding_address_ip_family`, `server_binding_port`, `client_address_ip_family`, `client_address_ip_type`, plus `public_url` only when configured). It is the label contract that the statistics handler and every Prometheus consumer depend on, and today only one of the eight labels is asserted. Mutations L1-L3 confirm it. |
| Client IP family and type from the remote client, not the server binding | 3A unit | R1: the five existing tests become two tables. The server binding is IPv4 in every case, so a family or type derived from the binding fails the IPv6 and mapped cases. Adds the `X-Forwarded-For` resolved variant, which carries the same address type. |
| `public_url()` and `configuration_instance_id()` accessors | 3A unit | R2b: `public_url()` returns the URL only when configured; `configuration_instance_id()` returns the id given to `new`. |
| `client_ip_addr()`, `client_port()`, `server_socket_addr()` and `ClientConnectionContext::port()` (lines 80-92 and 125-127, uncovered) | 3A unit, public API with no caller | R2c. No code in the repository calls them: the evidence flagged them as dead-code candidates. They are `pub` on a type of the `pub` module `event`, so removing them is a breaking API change before the crate's first publish; that is a production change and not a test decision. **Decision needed from the maintainer:** pin them (recommended, as for the unused `From` conversions in S2 and S3) or hand the removal off through the findings procedure and write no test for them. |
| `Event` equality | 3A unit | R1: one test that each part of a context (instance id, remote client address, server binding, public URL) makes two contexts unequal, and a clone is equal, so the whole-event assertions of S2 and S3 cannot miss a part. |
| `announce_events_match` and `pub mod test` | test support | R1 removes the helper and makes the module private (`mod tests`). Confirmed unused: `rg` finds no caller; the Criterion benches have their own helpers. |
| Type definitions and `pub mod` aliases (`sender`, `receiver`, `bus`) | wiring | Not tested: aliases with no behavior. |

No property candidate. The label table is a small input-by-state grid that examples cover.

## Test Bootstrap

A tiny in-file builder replaces the repeated construction: `connection(client_ip, bind_ip)` builds a
`ConnectionContext` with a fixed instance id, port and binding protocol, because the cases vary only in the
addresses. It has one responsibility and is not shared outside the file.

## Steps

| ID | Status | Work | Completion evidence |
| --- | --- | --- | --- |
| R1 | DONE | Refactor the 7 existing tests: the in-file builder, AAA markers, `it_should_*` names, the family and type tests as tables, the equality test per part, and remove the dead helper and the `pub mod test`. | Focused tests, `cargo +nightly fmt --all -- --check`, `git diff --check`, prose-first AAA review. Maintainer review of the result. Done together with R2 (disclosed); pending the maintainer's review. |
| R2 | DONE | Add R2a-R2c. | Same validation; the named mutations L1-L4 each make their test fail. All four did, plus seven survivor probes. |
| R3 | DONE | Select an integration increment, or `none`. | `none`: the file is types and pure projections. |
| R4 | DONE | Re-run unit-only coverage; record before and after. | Results and `coverage-evidence.md`. |

## Named Mutations

Each is a temporary hand-made change that its test must catch; it is reverted after the run and recorded in
Results. Lines are those of `event.rs` at the S3 baseline. The workspace denies `unused` and `warnings`: run a
mutation with `RUSTFLAGS="--cap-lints=warn"` only when the plain run fails to compile, and count it only when
its test runs and fails.

| ID | Test | Site | Temporary change |
| --- | --- | --- | --- |
| L1 | R2a | `From<ConnectionContext> for LabelSet`, the `server_binding_port` label | Use the client port instead of the server's. |
| L2 | R2a | the `client_address_ip_family` label | Use the server binding's family instead of the client's. |
| L3 | R2a | the `public_url` label | Upsert it also when no URL is configured (an empty value). |
| L4 | R1 (type table) | `client_address_ip_type` | Treat every IPv6 address as `V4MappedV6`. |

Gates: this plan needs approval first; R1 needs the maintainer's review of its result; the completed file needs
the maintainer's completed-file review before its one signed commit.

## Prose-First AAA Review

The 7 old tests became 29 cases in 10 test functions inside a private `mod tests` (27 cases when the file was
first completed; the independent review added two label cases, see "Independent Review Round"). Each reads as:
a fixed instance id, port and binding protocol (the in-file builders `connection_from` and `connection_between`),
one varying address or part, then one whole-value assertion.

| Test | Behavior it states | Cases | Expected value is |
| --- | --- | --- | --- |
| `it_should_derive_the_client_ip_family_from_the_client_address_not_the_server_binding` | The family label follows the client, never the IPv4 server binding. | direct IPv4, direct IPv6, IPv4-mapped IPv6, `::1` | A literal `IpFamily`. |
| `it_should_derive_the_client_ip_family_from_the_forwarded_address` | An `X-Forwarded-For` address gives the same family. | 1 | A literal `IpFamily`. |
| `it_should_derive_the_client_ip_type_from_the_client_address` | Only an IPv4-mapped IPv6 address is `V4MappedV6`; native, loopback and IPv4-compatible IPv6 are `Plain`. | 5 | A literal `IpType`. |
| `it_should_project_a_connection_context_to_exactly_these_metric_labels` | The full label contract: eight labels, `public_url` only when configured. | IPv4, IPv6, IPv4-mapped client of an IPv4 server, configured public URL, HTTPS server on port 443, server bound to an IPv4-mapped IPv6 address | A whole literal label set built from strings, not from the production label names. |
| `it_should_expose_the_public_url_only_when_it_is_configured` | `public_url()` returns what was set. | configured, absent | The configured string or `None`. |
| `it_should_expose_the_configuration_instance_id_it_was_created_with` | The accessor returns the id given to `new`. | 1 | A literal id that differs from the helper default. |
| `it_should_expose_the_client_address_and_port` | `client_ip_addr`, `client_port` and `ClientConnectionContext::port` follow the resolved address. | socket address with a port, `X-Forwarded-For` without one | Literal address and port, given as case parameters. |
| `it_should_expose_the_server_socket_address` | `server_socket_addr` is the binding's address. | 1 | A literal `SocketAddr`. |
| `it_should_not_consider_two_connection_contexts_equal_when_any_part_differs` | Equality covers the instance id, client address, client port, resolution source, server address and public URL. | 6 | `assert_ne` against the base context. |
| `it_should_consider_a_cloned_event_equal_to_the_original` | `Clone` and `PartialEq` agree on a whole event. | 1 | The original event. |

No test asserts through the production label names: the expected label sets are strings, so a renamed label
fails a test. None mocks or sleeps.

## Results

Unit-only coverage (`cargo llvm-cov -p torrust-tracker-http-core --all-features --lib`), measured on
2026-10-20 at 20:00 UTC. The baseline was re-measured at `develop` after the S3 merge, because that merge
brought in #2414 (listener tests), so the package baseline is not the S3 figure.

| Scope | Lines before | Lines after | Regions before | Regions after | Functions before | Functions after |
| --- | --- | --- | --- | --- | --- | --- |
| `event.rs` | 157 / 187 (83.96%) | 135 / 135 (100.00%) | 259 / 294 | 184 / 184 | 15 / 20 | 22 / 22 |
| Package `http-core` | 2,040 / 2,197 (92.85%) | 2,018 / 2,145 (94.08%) | 2,664 / 2,845 (93.64%) | 2,589 / 2,735 (94.66%) | 244 / 260 (93.85%) | 251 / 262 (95.80%) |

Unit tests: 95 before, 115 after. The package line total falls (2,197 to 2,145) because the dead
`announce_events_match` helper (31 lines in the file, 18 of them counted and uncovered) is removed with its
test module, while the four unused accessors (lines 80-92 and 125-127) are now covered. Production code
before the first `#[cfg(test)]` is byte-identical to `HEAD`. The figures are those of the file's own commit;
the review round below changes the case and test counts but not the coverage of `event.rs`.

Named mutations (each reverted; the production diff was checked empty afterwards):

| ID | Result | Tests that failed |
| --- | --- | --- |
| L1 | killed | 4 label-set cases |
| L2 | killed | 5 (family table, forwarded case and one label case) |
| L3 | killed | 3 label-set cases (the cases with no URL) |
| L4 | killed | 4 (type table and one label case) |

Survivor probes beyond the plan (all killed):

| ID | Temporary change | Killed by |
| --- | --- | --- |
| P1 | `to_ipv4()` instead of `to_ipv4_mapped()` in the type check | the `::1` and IPv4-compatible cases |
| P2 | `ClientConnectionContext::port()` always `None` | the client address and port case with a port |
| P3 | `public_url()` always `None` | the accessor case and the configured-URL label case |
| P5 | `with_public_url` discards its argument | the accessor, the labels and the equality case |
| P6 | `server_socket_addr` returns the client IP with the server port | the server address test |
| P7 | `configuration_instance_id()` returns a constant | the instance id test |

The probe numbering has no P4: the first form of the `with_public_url` probe (P5) did not compile under the
workspace lints, so it was redone in a compiling form before it counted.

### Independent Review Round

The independent Task Reviewer (see `agent-review-reports.md`) found two surviving mutants in the label table
(finding F1) and one test with logic in it (F2). Both are fixed:

| ID | Temporary change | Result before the fix | Result after the fix |
| --- | --- | --- | --- |
| L5 | the `server_binding_protocol` label hard-coded to `http` | survived | killed by the HTTPS case |
| L6 | the `server_binding_address_ip_type` label hard-coded to `plain` | survived | killed by the case with a server bound to `::ffff:127.0.0.1` |
| L7 | the `server_binding_port` label hard-coded to `7070` | survived (the reviewer's INFO) | killed by the HTTPS case on port 443 |

`it_should_expose_the_client_address_and_port` no longer computes its expected address with a `match`: it is a
case parameter. The label table now has 6 cases and the file 29 cases; the package has 117 unit tests after
this round (140 with `container.rs`).

## Approval

Approved by Jose Celano on 2026-10-20 (19:55 UTC): R1, R2a-R2c, R3 `none` and mutations L1-L4. The four accessors with no repository caller are pinned as public API (R2c), not handed off for removal.

## Progress Log

- 2026-10-20 19:53 UTC - GitHub Copilot - Created the plan from `event.rs`, the S3 baseline and the S2 and S3
  plans. No Rust test or production code changed. The plan awaits maintainer approval, with one decision:
  pin the four unused accessors or hand their removal off.
- 2026-10-20 19:55 UTC - Jose Celano - Approved the plan and chose to pin the four accessors. R1 starts.
- 2026-10-20 20:41 UTC - GitHub Copilot - R1-R4 done together (disclosed): 27 cases in 10 test functions,
  mutations L1-L4 and six extra probes killed, `event.rs` at 135 / 135 lines. The AAA review and the results
  are recorded above. The baseline was re-measured because #2414 changed the listener after S3.
- 2026-10-20 20:41 UTC - Jose Celano - Approved R1 and the completed file. The plan is `done`.
- 2026-10-21 07:25 UTC - GitHub Copilot - The independent review found two surviving label mutants (the
  protocol and address-type labels hard-coded) and one test with a `match` that computes the expected value.
  The label table gained an HTTPS case on port 443 and a case with a server bound to `::ffff:127.0.0.1`, and
  the client-address test takes its expected address as a case parameter. The file has 29 cases; see
  "Independent Review Round" above.
