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
last-updated-utc: "2026-10-23 07:58"
semantic-links:
  skill-links:
    - create-issue
    - fix-bug
  related-artifacts:
    - .github/skills/dev/debugging/fix-bug/SKILL.md
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/architecture/events.md
    - docs/adrs/20260727000000_events_are_objective_facts.md
    - docs/adrs/20260727180000_shared_services_across_tracker_instances.md
    - packages/http-core/src/event.rs
    - packages/http-core/src/statistics/event/listener.rs
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
`statistics-metrics-tests.md`). The #1349 spec is archived under `docs/issues/closed/1349-1347-add-tests-http-core/`; it
was under `docs/issues/open/1349-1347-add-tests-http-core/` until PR #2620 archived it.

The four getters of `Metrics` in `packages/http-core/src/statistics/metrics.rs`
(`tcp4_announces_handled`, `tcp4_scrapes_handled`, `tcp6_announces_handled`, `tcp6_scrapes_handled`) are
documented as "Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers" (and IPv6, and
`scrape`). They select the samples on the label `server_binding_address_ip_family` (`inet` or `inet6`), which
`packages/http-core/src/event.rs` sets from the address the server is **bound to** (`packages/http-core/src/statistics/event/handler.rs` passes the connection on), not from the client's address.

Observable consequence: a request whose client address is IPv6 but whose server binding is IPv4 is counted as `tcp4`.
A native IPv6 connection cannot reach an IPv4-only listener, so this mixed-family case is reachable only in
reverse-proxy mode (`[http_trackers.network] on_reverse_proxy = true`). In that mode the client address comes from
`X-Forwarded-For` (`packages/http-protocol/src/v1/services/peer_ip_resolver.rs`), so an IPv6 forwarded address reaches
the handler over an IPv4 listener; the opposite case (an IPv4 forwarded address over an `inet6` listener) is reachable
the same way. A dual-stack `[::]` binding is a different situation: an IPv4 client arrives there as an IPv4-mapped IPv6
address, which `event.rs` classifies as `Inet6` (the decided rule below), so the binding family and the client family
agree and the two meanings do not diverge. UDP behaves the same way: with `ipv6_v6only = false` on Linux a `[::]`
socket delivers an IPv4 client as `::ffff:a.b.c.d`, which `packages/udp-core/src/event.rs` classifies as `Inet6`
(and as `V4MappedV6` by type), and with `ipv6_v6only = true` no IPv4 client reaches that socket at all. So UDP has no case
where the binding family and the client family differ. Scenario M3 records what a dual-stack operator sees; it is not yet
reproduced.
The REST API publishes these values as `tcp4_announces_handled`, `tcp6_announces_handled`, and so on
(`packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`), so the wording and the numbers are public.

The client family is also classified for IPv4-mapped IPv6 addresses (`::ffff:a.b.c.d`). In
`packages/http-core/src/event.rs` such an address is classified as `Inet6` (case `ipv4_mapped_ipv6` of the
client-family test in `event.rs`, and case `ipv6_scrape_of_an_ipv4_mapped_client_with_a_public_url_on_another_port` of
`it_should_count_the_event_in_the_sample_with_exactly_the_connection_labels_and_the_request_kind` in `handler.rs`). Those two
cases pin that result. **Decided (maintainer, option (b)):** a mapped client stays IPv6, and the docs say so. This
decision rests on the maintainer's choice, not on a precedent in code: issue #1375 (commit `d4c43bd3a`) added only the
server-side label `server_binding_address_type` (`plain` | `v4_mapped_v6`, computed from the server's binding) and no
client-side counterpart (`docs/issues/closed/1671-ipv4-ipv6-client-metrics/ISSUE.md`). The client-side label is
`client_address_ip_type`, which came with #1671. A client-based selection that only reads
`client_address_ip_family` therefore still counts a mapped client as `tcp6`, and the docs must describe that
exception.

Constraints from the existing architecture (read before choosing a meaning):

- Events are objective facts (`docs/adrs/20260727000000_events_are_objective_facts.md`). The HTTP core event already
  carries both the server-binding family and the client family, so either meaning is a change of counter selection in
  the statistics code, never of the event schema.
- The counters are application-wide aggregates (`docs/adrs/20260727180000_shared_services_across_tracker_instances.md`,
  `docs/architecture/events.md`): one HTTP core event bus and one repository serve every HTTP listener, and the metrics
  listener already filters events by the configuration instance's `tracker_usage_statistics` policy
  (`packages/http-core/src/statistics/event/listener.rs`). A per-listener split exists only in the labeled metrics.
- The tracker keeps no per-client or per-peer request statistics, because the cardinality would be unbounded. The
  counters can therefore only be per binding or per family (two values), never per client. This rules out per-client
  breakdowns; it does not rule out counting by the client's family, which is a two-value bucket.

The event also carries the client's family (label `client_address_ip_family`, added by the #1671 line of work,
spec `docs/issues/closed/1671-ipv4-ipv6-client-metrics/ISSUE.md`). That spec explains that the server-binding label
describes the socket and is always `inet6` for a dual-stack `[::]` binding, so on such a deployment the `tcp4`
counters read 0 for IPv4 clients and every request counts as `tcp6`. The getters were not moved to the client label.

The behavior is pinned by `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client` in
`metrics.rs`, so a change of behavior makes that test fail and it must be updated with the fix.

## Scope

### In Scope

- Follow the maintainer's sequence: (1) correct the doc comments and REST API docs to describe the current
  server-binding meaning; (2) add tests that describe what the code does; (3) re-evaluate whether that is the wanted
  behavior; (4) only if it is not, change the behavior (select on `client_address_ip_family`, update the pinned test);
  (5) record the final decision as an ADR and in the architecture docs, whether or not the behavior changes.
- Document that a mapped client counts as IPv6 (decided, option (b)).
- Apply the same decision to the UDP counters (`udp4_*`/`udp6_*` in `udp-core` and `udp-server`, documented "from
  IPv4/IPv6 peers" as well). #1348 already recorded this for UDP in its `lessons.md` ("Metrics plan" row) and it was
  never taken up; one decision should cover HTTP and UDP, so this draft is the shared owner.

### Out of Scope

- Renaming public REST API fields (a breaking change; decide separately if needed).

## Architectural Decisions

- Related ADRs: `20260727000000_events_are_objective_facts.md` and `20260727180000_shared_services_across_tracker_instances.md`.
  Issue #1375 (commit `d4c43bd3a`) added the server-side label `server_binding_address_type`; it records no
  client-side decision and is not an ADR.
- ADRs to create: one, recording which label the aggregate `tcp4`/`tcp6` and `udp4`/`udp6` counters read and why, the
  mapped-address rule, and that no per-client statistics exist. Created in T6 whether or not the behavior changes.
- Architecture docs to update: `docs/architecture/events.md` (what the family counters mean). The same page still
  calls the per-instance metrics-policy filter "Proposed" although the HTTP core and UDP server listeners already
  apply it; correct that in the same change.

## Design and Ownership Review

Not applicable.

## Bug-Fix Process

Follow `.github/skills/dev/debugging/fix-bug/SKILL.md`:

- analysis: confirm the label source in `packages/http-core/src/event.rs` (~151–152) and the history of the selection;
- reproduction: in reverse-proxy mode (`on_reverse_proxy = true`), send an announce with an IPv6 `X-Forwarded-For`
  address to the IPv4 HTTP listener and read the REST API statistics. Recorded in
  `manual-verification-evidence.md` section V1 and classified Reproduced. The wrong-or-intended question depends on T4;
  the reproduction does not decide it. The observed outcome is the public REST API statistics, so no temporary test at
  an internal seam was needed for the reproduction;
- regression boundary: the existing unit test in `metrics.rs` plus a handler-level case. First they describe the
  current behavior (characterization); they change only if the behavior changes;
- production change only if the maintainer's re-evaluation (T4) rejects the server-binding meaning; green tests; a
  final like-for-like recheck.

The documentation-first path (T2, T3, T6) changes no production code, so it has no failing test to record. `fix-bug`
does not define that exception. The maintainer accepted it for this draft; the durable approval reference is recorded
in the progress log when the review thread reply is posted. If T4 chooses a behavior change, the full procedure
applies in T5: recorded red run, then fix, then green recheck.

## Regression Test Strategy

The causal seams are, per package, the getters that select on the server-binding label:

- `torrust-tracker-http-core`: `Metrics::tcp{4,6}_{announces,scrapes}_handled` and the label set built in the handler.
- `torrust-tracker-udp-core`: `udp{4,6}_*_handled` in `packages/udp-core/src/statistics/metrics.rs`.
- `torrust-tracker-udp-server`: `udp{4,6}_*` in `packages/udp-server/src/statistics/metrics.rs`. The REST API's
  `udp4_*`/`udp6_*` fields read this one (`packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`).

The smallest deterministic tests are unit tests that feed one event per case and assert the counter the current
meaning names (T3), flipped only if T5 changes the meaning. HTTP cases R1–R3 feed events to the HTTP handler; the UDP
packages get their own cases, listed per package in T3. UDP has no case where the binding and client families differ (a
dual-stack client is a mapped one, so it belongs to R3, and with `ipv6_v6only = true` no IPv4 client reaches the
socket), so its cases pin the mapped result and the plain cases only.

- R1: IPv6 client on an IPv4 binding (the reverse-proxy case reproduced in V1). HTTP only.
- R2: IPv4 client on an IPv6 binding (reverse proxy with an IPv4 forwarded address). HTTP only; UDP has no such case.
- R3: IPv4-mapped IPv6 client (`::ffff:192.0.2.1`), decided to count as IPv6, for HTTP and UDP. `event.rs` classifies it
  as `Inet6`. The existing cases named in Background (`ipv4_mapped_ipv6` and
  `ipv6_scrape_of_an_ipv4_mapped_client_with_a_public_url_on_another_port`) already pin the HTTP result; T3 cites them
  and updates them only if T5 changes the meaning. In `udp-core`, the existing test
  `it_should_report_a_v4_mapped_v6_ip_type_when_the_client_is_ipv4_mapped_ipv6` pins the type only, so T3 adds a case that
  pins the family of a mapped UDP client.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Record the maintainer's decisions so far | Mapped client counts as IPv6 (option (b), the maintainer's decision in this review; #1375 does not decide it). Sequence is docs first, then tests, re-evaluation, optional behavior change, ADR. No per-client statistics exist. |
| T2 | TODO | Correct the doc comments (`metrics.rs` getters, and the `udp-core` and `udp-server` equivalents) and the REST API docs to say the counters follow the server binding, and state the mapped-client rule | Documentation only, no production change. |
| T3 | TODO | Add characterization tests for what the code does today: R1–R3 in `http-core`, and the per-package UDP cases in `udp-core` (`udp{4,6}_*_handled` by binding family) and `udp-server` (`udp{4,6}_*` by binding family) | Test-only; they pass today. Skip a case that an existing test already pins and cite that test. |
| T4 | TODO | Re-evaluate (maintainer): is the server-binding meaning the wanted behavior? | Inputs: V1, M3, and the dashboards' use. Outcome recorded in Open Decisions. If "keep", go to T6. |
| T5 | BLOCKED on T4 | Only if T4 rejects the meaning: red run of the flipped R1–R3 (record `evidence/red-run.txt`), then the fix, then the green recheck with M2 | Three separate tasks, per `fix-bug`. Needs explicit maintainer approval of the production change. |
| T6 | TODO | Write the ADR and update `docs/architecture/events.md` | Records the final decision, whether or not the behavior changes. Also corrects the stale "Proposed" wording on the metrics-policy filter. |

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | Doc comments and REST API docs | Commit after focused validation. |
| T3 | Characterization tests (test-only) | Commit after the tests pass and the pre-commit checks pass. |
| T5 | Red tests, fix and evidence, as three separate commits (only if T4 changes the behavior) | Commit the red run before starting the fix. |
| T6 | ADR and architecture docs | Commit after the final decision is recorded. |

## Pull Request Plan

Open a pull request for each of T2, T3 and T6, so each is small and mergeable alone. T5, if it happens, gets its own
issue and pull requests. This draft's own pull request (#2621) carries only the specification.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/http-core-tcp-family-counters-clients-or-server/ISSUE.md`
- [ ] Spec reviewed and approved by user/maintainer
- [ ] GitHub issue created and issue number added to this spec
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-10-21 18:50 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Reproduced the reverse-proxy mixed-family case (IPv6 `X-Forwarded-For` over the IPv4 listener); `tcp4_announces_handled` 0→1, `tcp6` stayed 0; addressed the three Copilot review threads on the draft - `evidence/`, `manual-verification-evidence.md` V1
- 2026-10-21 18:55 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Revised the draft for the second PR #2621 review (F1–F5): label source moved to `event.rs`; mapped-address options and the dual-stack route (M3, not yet reproduced) added; T2 split into red and T3 fix, with T4 as the green recheck; the documentation-only option moved to Open Decisions; required sections completed
- 2026-10-22 18:07 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Recorded the maintainer's decisions: mapped client counts as IPv6 (option (b), the maintainer's decision; a #1375 precedent was cited here and withdrawn, see the 18:58 UTC entry); docs-first sequence with characterization tests, a re-evaluation and an ADR; no per-client statistics exist. Added the architecture constraints from `events.md` and the shared-services ADR; replanned T1–T6
- 2026-10-22 18:13 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Addressed the second da2ce7 round (review 5605012005, F6–F10): repeated the V1 reproduction and stored the raw responses; per-package UDP cases and scenario M4 added; the mapped-address tests named; V1 time and artifact commit corrected; the Background routes reordered (the first entry's stamp was 18:50, not 18:49)
- 2026-10-22 18:37 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Addressed Copilot's second review (review 5619053291): the evidence commands use explicit repo-relative paths and name the runtime copy of the configuration; the Background path is fully qualified; the development token stays (see `docs/pr-reviews/pr-2621-review/PR-REVIEW.md`)
- 2026-10-22 18:58 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Withdrew the #1375 precedent for the mapped-address rule (review 5619336077, F19): `d4c43bd3a` added only the server-side label `server_binding_address_type`, so option (b) rests on the maintainer's decision
- 2026-10-22 19:02 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Addressed the rest of da2ce7's third round (review 5619336077, F20, F22, F23 and F24): UDP dual-stack cases moved under R3 and M4 reworded (UDP has no case where the binding and client families differ); the reproduction steps split by shell; the 18:15 stamp above corrected to 18:13
- 2026-10-22 19:23 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Corrected the 19:02 entry above (review 5619631773, F28): it listed F21 and F25, which are audit changes made later in the audit commit, as addressed
- 2026-10-22 19:45 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Aligned acceptance criterion AC2 with the regression strategy (Copilot review 5619691908): `http-core` keeps the mixed-family cases, while the UDP packages have plain-family and mapped-client cases only
- 2026-10-23 07:09 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Corrected the review ID named in the 19:23 entry above, which now cites review 5619631773 (the review that raised F28) instead of 5619336077 (F31). The change was edited on 2026-10-22 at about 20:04 UTC, but the signed commit failed on a GPG timeout and was made after a restart, so the entry carries the commit time (F35)
- 2026-10-23 07:12 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Made scenario M3's configuration explicit (Copilot review 5620148855, audit F33): its own configuration with direct-client mode, a dual-stack socket and debug logging
- 2026-10-23 07:58 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Addressed da2ce7's review 5625253400 and Copilot's review 5625254189 (audit F34 to F40): the M3 settings name their configuration tables; the #1349 specification's two locations are noted; the configuration copy check that the steps did not show is dropped from the evidence and its frontmatter stamp is quoted; the F31 entry's stamp is its commit time

## Acceptance Criteria

- [ ] AC1: Doc comments and REST API documentation describe the same meaning of the `tcp4`/`tcp6` and `udp4`/`udp6` counters as the code, including the mapped-client rule (T2).
- [ ] AC2: Tests pin the behavior (T3): R1–R3 in `http-core` for the mixed-family and IPv4-mapped IPv6 requests, and the plain-family and mapped-client cases in `udp-core` and `udp-server`, which have no mixed-family case.
- [ ] AC3: The re-evaluation outcome (T4) is recorded, and the ADR and `docs/architecture/events.md` state the final decision (T6).
- [ ] AC4: Only if T4 changes the behavior: M1 reproduced before the fix, the red run recorded, and M2 re-run after it (T5).
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

The commands below run on the default stable toolchain. The pre-commit and pre-push gates also run the nightly checks.

- `cargo test -p torrust-tracker-http-core`
- `cargo test -p torrust-tracker-udp-core` (UDP core counters, covered by T2 and T3)
- `cargo test -p torrust-tracker-udp-server` (the UDP counters that the REST API publishes, covered by T2 and T3)
- `linter all`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reverse-proxy mixed family before the fix: IPv6 `X-Forwarded-For` over the IPv4 HTTP listener | Build, run with `on_reverse_proxy = true`, announce with `X-Forwarded-For: 2001:db8::1`, read `/api/v1/stats` | Current behavior recorded (`tcp4` incremented, `tcp6` unchanged) | DONE | `manual-verification-evidence.md` section V1 and `evidence/` |
| M2 | Same scenario after a behavior change in T5 (skipped if T4 keeps the current meaning) | Repeat M1 on the fixed build | Counter matches the T4 decision | TODO | `manual-verification-evidence.md` (to be added as V2) |
| M3 | Dual-stack `[::]` binding: an IPv4 client reaches an inet6 socket | Use a configuration of its own, not V1's: `[[http_trackers]] bind_address = "[::]:7070"`, `[http_trackers.network] on_reverse_proxy = false` (the default, so the client address is the socket address, `packages/http-protocol/src/v1/services/peer_ip_resolver.rs`) and `[logging] trace_filter = "debug"` (the counter labels are logged at debug level by `packages/http-core/src/statistics/event/handler.rs`; both HTTP tracker tables reject unknown fields, so `trace_filter` must sit in `[logging]`). Announce over IPv4 to `127.0.0.1`, read `/api/v1/stats` and the labels in the log | Record the observed counters and labels. Both the binding and the mapped client classify as `inet6`, so M3 records what a dual-stack operator sees today; it does not separate the two meanings | TODO | `manual-verification-evidence.md` (to be added as V3) |
| M4 | UDP dual-stack: an IPv4 client reaches an `inet6` UDP socket as a mapped address (UDP has no reverse-proxy mode, and the two meanings agree here) | Set `bind_address = "[::]:6969"` on the UDP tracker (default `ipv6_v6only = false` on Linux), send one announce over IPv4 with `tracker_client`, read `/api/v1/stats` | Record the `udp4_*`/`udp6_*` counters that the REST API reads from `udp-server`. It records what a dual-stack operator sees (the mapped case, counted as `udp6`); it cannot tell the two meanings apart. Not yet reproduced | TODO | `manual-verification-evidence.md` (to be added as V4) |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Create or extend `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The reverse-proxy check is a manual `curl` sequence recorded in the evidence file.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | T2 commit |
| AC2 | TODO | The `http-core` R1–R3 tests and the `udp-core` and `udp-server` cases (T3 commits) |
| AC3 | TODO | T4 outcome in Open Decisions; T6 ADR and `events.md` |
| AC4 | TODO | Only if T4 changes the behavior: M1 (`manual-verification-evidence.md` V1), the red run and M2 |

## Risks and Trade-offs

- Changing the meaning changes the numbers dashboards already read. The sequence therefore documents the current
  behavior first and changes it only after an explicit re-evaluation.
- The characterization tests pin today's behavior. If T4 changes the meaning they must change with it; that is
  intended, and the red run in T5 shows it.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
  issue specification's directory.
- Independent reviewer results go in `agent-review-reports.md` (see `docs/templates/AGENT-REVIEW-REPORTS.md`).

## Open Decisions (maintainer)

Decided:

- Mapped addresses (`::ffff:a.b.c.d`): keep as IPv6 and document it (option (b)). This is the maintainer's decision;
  #1375 added only a server-side label and does not decide it.
- Sequence: documentation first, then characterization tests, then a re-evaluation, then an optional behavior change,
  then the ADR and architecture docs.
- The documentation-first path is accepted as a reduced `fix-bug` process (no red run while production code is
  unchanged). A durable approval reference is recorded in the progress log with the thread reply.
- The development token `MyAccessToken` in committed evidence (`evidence/tracker.toml`,
  `manual-verification-evidence.md`) is accepted. It matches the repository's own e2e configuration, so it is not a
  real secret.

Open:

- T4: is the server-binding meaning the wanted behavior for HTTP and UDP? Decided after T2 and T3, with V1 and M3 as
  inputs.
- Whether V1 is a defect or expected behavior, given that the pinned test
  `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client` encodes the current behavior. T4 answers it.

## References

- Found by: #1349 (EPIC #1347 subissue, merged through PR #2616)
- Pinning test: `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client`
- Mapped-address context: issue #1375 (commit `d4c43bd3a`, the server-side label only); origin #1368; client-side labels and the dual-stack context #1671
- Architecture: `docs/architecture/events.md`, `docs/adrs/20260727000000_events_are_objective_facts.md`,
  `docs/adrs/20260727180000_shared_services_across_tracker_instances.md`
