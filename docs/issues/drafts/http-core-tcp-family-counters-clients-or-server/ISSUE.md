---
schema-version: 1
doc-type: issue
issue-type: bug
status: draft
priority: p3
epic: null
github-issue: null
spec-path: docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/ISSUE.md
branch: "{issue-number}-http-core-tcp-family-counters-clients-or-server"
related-pr: null
last-updated-utc: "2026-10-21 17:04"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - packages/http-core/src/statistics/metrics.rs
    - packages/http-core/src/statistics/event/handler.rs
    - packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs
---

<!-- skill-link: create-issue -->

# Issue #[To be assigned] - HTTP core `tcp4`/`tcp6` counters: documented per client, implemented per server binding

## Goal

Make the documentation, the code and the public REST API statistics agree on what the HTTP tracker's
`tcp4_*` and `tcp6_*` counters count: requests from IPv4 or IPv6 clients, or requests received on an
IPv4 or IPv6 server binding.

## Background

Found while writing tests for `torrust-tracker-http-core` (#1349, slice S5, plan
`statistics-metrics-tests.md`). The #1349 spec is archived under `docs/issues/closed/1349-1347-add-tests-http-core/`.

The four getters of `Metrics` in `packages/http-core/src/statistics/metrics.rs`
(`tcp4_announces_handled`, `tcp4_scrapes_handled`, `tcp6_announces_handled`, `tcp6_scrapes_handled`) are
documented as "Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers" (and IPv6, and
`scrape`). They select the samples on the label `server_binding_address_ip_family` (`inet` or `inet6`), which
`statistics/event/handler.rs` fills from the address the server is **bound to**, not from the client's address.

Observable consequence: an IPv6 client talking to a tracker bound to an IPv4 address is counted as `tcp4`.
The REST API publishes these values as `tcp4_announces_handled`, `tcp6_announces_handled`, and so on
(`packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`), so the wording and the numbers are public.

The event also carries the client's family (label `client_address_ip_family`, added by the #1671 line of work,
spec `docs/issues/closed/1671-ipv4-ipv6-client-metrics/ISSUE.md`). That spec explains that the server-binding label
describes the socket and is always `inet6` for a dual-stack `[::]` binding, so on such a deployment the `tcp4`
counters read 0 for IPv4 clients and every request counts as `tcp6`. The getters were not moved to the client label.

The behavior is pinned by `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client` in
`metrics.rs`, so a change of behavior makes that test fail and it must be updated with the fix.

## Scope

### In Scope

- Decide which meaning is intended (maintainer decision) and make doc comments, REST API documentation and code agree.
- If the meaning stays "server binding": correct the doc comments (and the REST API docs) to say so.
- If the meaning becomes "client": select on `client_address_ip_family` and update the pinned test.
- Apply the same decision to the UDP counters (`udp4_*`/`udp6_*` in `udp-core` and `udp-server`, documented "from
  IPv4/IPv6 peers" as well). #1348 already recorded this for UDP in its `lessons.md` ("Metrics plan" row) and it was
  never taken up; one decision should cover HTTP and UDP, so this draft is the shared owner.

### Out of Scope

- Renaming public REST API fields (a breaking change; decide separately if needed).

## Architectural Decisions

- Related ADRs: none known. A rename or a change of the public statistics' meaning may need one.
- ADRs to create: `None known`

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- analysis: confirm the label source in `handler.rs` and the history of the selection;
- reproduction: send an announce over IPv6 to a tracker bound to IPv4 (or build the event in a unit test) and read
  the REST API statistics;
- regression boundary: the existing unit test in `metrics.rs`, updated, plus a handler-level case;
- production change per the maintainer's decision; green tests; a final like-for-like recheck.

If the decision is "documentation is wrong", the bug-fix process reduces to a documentation correction with the
existing test as its guard.

## Regression Test Strategy

The causal seam is `Metrics::tcp{4,6}_{announces,scrapes}_handled` and the label set built in the handler. The
smallest deterministic test is a unit test that feeds a mixed-family event (IPv6 client, IPv4 binding) and asserts
the intended counter. The current test already does this for the server-binding meaning.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Decide the meaning (maintainer) | Recorded in this spec. |
| T2 | TODO | Align docs and/or code and update the pinned test | One signed commit per independent change. |
| T3 | TODO | Check the REST API documentation and the dual-stack experiment output | The same wording everywhere. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The doc or code change with its test update | Commit after focused validation and required review. |

## Acceptance Criteria

- [ ] AC1: Doc comments, REST API documentation and code describe the same meaning of the `tcp4`/`tcp6` counters.
- [ ] AC2: A unit test pins the chosen behavior for a mixed-family request.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass

## Verification Plan

- `cargo test -p torrust-tracker-http-core`
- `linter all`
- Manual: announce over IPv6 to an IPv4-bound HTTP tracker and read the REST API statistics.

## Risks and Trade-offs

- Changing the meaning changes the numbers dashboards already read; prefer documenting the current behavior unless
  the maintainer wants the per-client meaning.

## References

- Found by: #1349 (EPIC #1347 subissue, merged through PR #2616)
- Pinning test: `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client`
