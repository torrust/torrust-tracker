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
last-updated-utc: "2026-10-21 18:49"
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

Observable consequence: a request whose client address is IPv6 but whose server binding is IPv4 is counted as `tcp4`.
A native IPv6 connection cannot reach an IPv4-only listener, so this mixed-family case is reachable only in
reverse-proxy mode (`[http_trackers.network] on_reverse_proxy = true`). In that mode the client address comes from
`X-Forwarded-For` (`packages/http-protocol/src/v1/services/peer_ip_resolver.rs`), so an IPv6 forwarded address reaches
the handler over an IPv4 listener.
The REST API publishes these values as `tcp4_announces_handled`, `tcp6_announces_handled`, and so on
(`packages/rest-api-runtime-adapter/src/v1/adapters/stats.rs`), so the wording and the numbers are public.

The client family is also classified for IPv4-mapped IPv6 addresses (`::ffff:a.b.c.d`). In
`packages/http-core/src/event.rs` such an address is classified as `Inet6`, and the handler test `v4_mapped_v6`
pins that result. Any decision about the client meaning must therefore also decide how mapped addresses count.

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
- reproduction: in reverse-proxy mode (`on_reverse_proxy = true`), send an announce with an IPv6 `X-Forwarded-For`
  address to the IPv4 HTTP listener and read the REST API statistics. Recorded in
  `manual-verification-evidence.md` section V1 and classified Reproduced. The wrong-or-intended question depends on T1;
  the reproduction does not decide it. The observed outcome is the public REST API statistics, so no temporary test at
  an internal seam was needed for the reproduction;
- regression boundary: the existing unit test in `metrics.rs`, updated, plus a handler-level case;
- production change per the maintainer's decision; green tests; a final like-for-like recheck.

If the decision is "documentation is wrong", the bug-fix process reduces to a documentation correction with the
existing test as its guard.

## Regression Test Strategy

The causal seam is `Metrics::tcp{4,6}_{announces,scrapes}_handled` and the label set built in the handler. The
smallest deterministic tests are unit tests that feed one event per regression case and assert the counter that the
decision in T1 names:

- R1: IPv6 client on an IPv4 binding (the reverse-proxy case reproduced in V1).
- R2: IPv4 client on an IPv6 binding (for example a dual-stack `[::]` listener).
- R3: IPv4-mapped IPv6 client (`::ffff:192.0.2.1`). `event.rs` classifies it as `Inet6` today, and the handler test
  `v4_mapped_v6` pins that result. T1 must say whether it counts as IPv4 or IPv6; if the decision changes it, update
  `v4_mapped_v6` with it.

Each case covers both the HTTP (`tcp4`/`tcp6`) and the UDP (`udp4`/`udp6`) counters, so the decision is enforced on
both transports.

## Implementation Plan

| ID | Status | Task | Notes / Expected Output |
| --- | --- | --- | --- |
| T1 | TODO | Decide the meaning for HTTP and UDP, including IPv4-mapped IPv6 clients (maintainer) | Options: (A) keep server-binding meaning and correct the docs; (B) client meaning via `client_address_ip_family`, with mapped addresses counted as IPv4 or IPv6. Recorded in this spec before T2. |
| T2 | TODO | Red: add cases R1–R3 that fail against the current code wherever the T1 decision changes behavior | Red before green. Skipped only under the documentation-only exception below. |
| T3 | TODO | Green: align code, doc comments and REST API docs with T1; update the pinned test and `v4_mapped_v6` | One signed commit per independent change. |
| T4 | TODO | Re-run manual scenario M2 after the fix and record it in `manual-verification-evidence.md` | Reverse-proxy case reads the new meaning. |

Documentation-only exception: if T1 keeps the server-binding meaning and the only change is documentation, there is
no production change and no red test. The existing pinned test stays the guard, and V1 records the behavior that the
corrected docs must describe. The evidence file must state this exception explicitly.

## Commit Points

| Task | Coherent change set | Commit policy |
| --- | --- | --- |
| T2 | The red regression cases R1–R3 (failing tests only) | Commit after confirming they fail for the expected reason. |
| T3 | The production or doc change with its test update (green) | Commit after focused validation and required review. |
| T4 | Evidence update only (`manual-verification-evidence.md`) | Commit with the final verification. |

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

## Acceptance Criteria

- [ ] AC1: Doc comments, REST API documentation and code describe the same meaning of the `tcp4`/`tcp6` counters for HTTP and UDP.
- [ ] AC2: Regression cases R1–R3 pin the chosen behavior for mixed-family and IPv4-mapped IPv6 requests on both transports.
- [ ] AC3: Manual scenario M1 is reproduced and recorded before the fix, and M2 is re-run after it.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `cargo test -p torrust-tracker-http-core`
- `cargo test -p torrust-tracker-udp-core` (UDP counters, when T1 covers them)
- `linter all`
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | --- | --- | --- | --- | --- |
| M1 | Reverse-proxy mixed family before the fix: IPv6 `X-Forwarded-For` over the IPv4 HTTP listener | Build, run with `on_reverse_proxy = true`, announce with `X-Forwarded-For: 2001:db8::1`, read `/api/v1/stats` | Current behavior recorded (`tcp4` incremented, `tcp6` unchanged) | DONE | `manual-verification-evidence.md` section V1 and `evidence/` |
| M2 | Same scenario after the T1 change | Repeat M1 on the fixed build | Counter matches the decision in T1 | TODO | `manual-verification-evidence.md` (to be added as V2) |

Notes:

- Manual verification is mandatory even when automated tests pass.
- Create or extend `manual-verification-evidence.md` from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Disposable Verification Scripts

None planned. The reverse-proxy check is a manual `curl` sequence recorded in the evidence file.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO | T1 decision and T3 commit |
| AC2 | TODO | R1–R3 tests (T2 and T3 commits) |
| AC3 | TODO | M1 (`manual-verification-evidence.md` V1) and M2 |

## Risks and Trade-offs

- Changing the meaning changes the numbers dashboards already read; prefer documenting the current behavior unless
  the maintainer wants the per-client meaning.

## Implementation Completion Review

- Retrospective: `Not yet assessed`
- If needed, create `implementation-retrospective.md` from `docs/templates/IMPLEMENTATION-RETROSPECTIVE.md` in this
  issue specification's directory.
- Independent reviewer results go in `agent-review-reports.md` (see `docs/templates/AGENT-REVIEW-REPORTS.md`).

## References

- Found by: #1349 (EPIC #1347 subissue, merged through PR #2616)
- Pinning test: `it_should_count_by_the_family_of_the_server_binding_and_not_of_the_client`
