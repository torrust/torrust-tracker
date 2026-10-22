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
last-updated-utc: "2026-10-22 19:30"
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
`statistics-metrics-tests.md`). The #1349 spec is archived under `docs/issues/closed/1349-1347-add-tests-http-core/`.

The four getters of `Metrics` in `packages/http-core/src/statistics/metrics.rs`
(`tcp4_announces_handled`, `tcp4_scrapes_handled`, `tcp6_announces_handled`, `tcp6_scrapes_handled`) are
documented as "Total number of TCP (HTTP tracker) `announce` requests from IPv4 peers" (and IPv6, and
`scrape`). They select the samples on the label `server_binding_address_ip_family` (`inet` or `inet6`), which
`packages/http-core/src/event.rs` sets from the address the server is **bound to** (`statistics/event/handler.rs` passes the connection on), not from the client's address.

Observable consequence: a request whose client address is IPv6 but whose server binding is IPv4 is counted as `tcp4`.
A native IPv6 connection cannot reach an IPv4-only listener, so this mixed-family case is reachable only in
reverse-proxy mode (`[http_trackers.network] on_reverse_proxy = true`). A second route is a dual-stack `[::]` binding,
where IPv4 clients arrive as IPv4-mapped IPv6 addresses (scenario M3, not yet reproduced). In that mode the client address comes from
`X-Forwarded-For` (`packages/http-protocol/src/v1/services/peer_ip_resolver.rs`), so an IPv6 forwarded address reaches
the handler over an IPv4 listener.
The REST API publishes these values as `tcp4_announces_handled`, `tcp6_announces_handled`, and so on
(`packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`), so the wording and the numbers are public.

The client family is also classified for IPv4-mapped IPv6 addresses (`::ffff:a.b.c.d`). In
`packages/http-core/src/event.rs` such an address is classified as `Inet6`, and the handler test `v4_mapped_v6`
pins that result. **Decided (maintainer, option (b)):** a mapped client stays IPv6, and the docs say so. This matches
the earlier decision recorded in issue #1375 (commit `d4c43bd3a`), which kept a mapped client out of the IPv4 count
and exposed it through the separate label `server_binding_address_type` (`plain` | `v4_mapped_v6`). That decision
does not rule explicitly on the `tcp4`/`tcp6` aggregate, so applying it here is an inference. A client-based selection
that only reads `client_address_ip_family` therefore still counts a mapped client as `tcp6`, and the docs must
describe that exception.

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
  Issue #1375 (commit `d4c43bd3a`) is the earlier decision on mapped addresses; it is not an ADR.
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

The causal seam is `Metrics::tcp{4,6}_{announces,scrapes}_handled` and the label set built in the handler. The
smallest deterministic tests are unit tests that feed one event per case and assert the counter the current meaning
names (T3), flipped only if T5 changes the meaning:

- R1: IPv6 client on an IPv4 binding (the reverse-proxy case reproduced in V1).
- R2: IPv4 client on an IPv6 binding (for example a dual-stack `[::]` listener).
- R3: IPv4-mapped IPv6 client (`::ffff:192.0.2.1`), decided to count as IPv6. `event.rs` classifies it as `Inet6`,
  and the handler test `v4_mapped_v6` already pins that result.

Each case covers both the HTTP (`tcp4`/`tcp6`) and the UDP (`udp4`/`udp6`) counters.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | DONE | Record the maintainer's decisions so far | Mapped client counts as IPv6 (option (b), consistent with #1375). Sequence is docs first, then tests, re-evaluation, optional behavior change, ADR. No per-client statistics exist. |
| T2 | TODO | Correct the doc comments (`metrics.rs` getters, and the `udp-core` and `udp-server` equivalents) and the REST API docs to say the counters follow the server binding, and state the mapped-client rule | Documentation only, no production change. |
| T3 | TODO | Add characterization tests R1–R3 for HTTP and UDP that describe what the code does today | Test-only; they pass today. Skip a case that an existing test already pins and cite that test. |
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

- 2026-10-21 18:49 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Reproduced the reverse-proxy mixed-family case (IPv6 `X-Forwarded-For` over the IPv4 listener); `tcp4_announces_handled` 0→1, `tcp6` stayed 0; addressed the three Copilot review threads on the draft - `evidence/`, `manual-verification-evidence.md` V1
- 2026-10-21 18:55 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Revised the draft for the second PR #2621 review (F1–F5): label source moved to `event.rs`; mapped-address options and the dual-stack route (M3, not yet reproduced) added; T2 split into red and T3 fix, with T4 as the green recheck; the documentation-only option moved to Open Decisions; required sections completed
- 2026-10-22 19:30 UTC - AI assistant (GitHub Copilot, Copilot SDK in VS Code) - Recorded the maintainer's decisions: mapped client counts as IPv6 (option (b), #1375 and `d4c43bd3a`); docs-first sequence with characterization tests, a re-evaluation and an ADR; no per-client statistics exist. Added the architecture constraints from `events.md` and the shared-services ADR; replanned T1–T6

## Acceptance Criteria

- [ ] AC1: Doc comments and REST API documentation describe the same meaning of the `tcp4`/`tcp6` and `udp4`/`udp6` counters as the code, including the mapped-client rule (T2).
- [ ] AC2: Tests R1–R3 pin the behavior for mixed-family and IPv4-mapped IPv6 requests on both transports (T3).
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
- `cargo test -p torrust-tracker-udp-core` (UDP counters, covered by T2 and T3)
- `linter all`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reverse-proxy mixed family before the fix: IPv6 `X-Forwarded-For` over the IPv4 HTTP listener | Build, run with `on_reverse_proxy = true`, announce with `X-Forwarded-For: 2001:db8::1`, read `/api/v1/stats` | Current behavior recorded (`tcp4` incremented, `tcp6` unchanged) | DONE | `manual-verification-evidence.md` section V1 and `evidence/` |
| M2 | Same scenario after a behavior change in T5 (skipped if T4 keeps the current meaning) | Repeat M1 on the fixed build | Counter matches the T4 decision | TODO | `manual-verification-evidence.md` (to be added as V2) |
| M3 | Dual-stack `[::]` binding: an IPv4 client reaches an inet6 socket | Set `bind_address = "[::]:7070"` on the HTTP tracker, announce over IPv4 to `127.0.0.1`, read `/api/v1/stats` and the labels in the log | Record the observed counters and labels. Both the binding and the mapped client classify as `inet6`, so M3 records what a dual-stack operator sees today; it does not separate the two meanings | TODO | `manual-verification-evidence.md` (to be added as V3) |

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
| AC2 | TODO | R1–R3 tests (T3 commit) |
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

- Mapped addresses (`::ffff:a.b.c.d`): keep as IPv6 and document it (option (b)), matching #1375.
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
- Mapped-address decision: issue #1375 (commit `d4c43bd3a`); origin #1368; dual-stack context #1671
- Architecture: `docs/architecture/events.md`, `docs/adrs/20260727000000_events_are_objective_facts.md`,
  `docs/adrs/20260727180000_shared_services_across_tracker_instances.md`
