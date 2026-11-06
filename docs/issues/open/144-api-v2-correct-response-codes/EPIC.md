---
schema-version: 1
doc-type: epic
status: planned
epic: null
github-issue: 144
spec-path: docs/issues/open/144-api-v2-correct-response-codes/EPIC.md
epic-owner: josecelano
last-updated-utc: "2026-11-06 12:00"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/drafts/144-make-rest-api-persistence-aware/ISSUE.md
    - docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md
    - docs/adrs/20260901113500_define_completed_download_metric_retention_names.md
    - docs/adrs/20261105132341_define_the_stats_and_metrics_endpoints_and_require_help_and_type_in_prometheus_output.md
    - docs/issues/closed/1938-rest-api-contract-first-migration/EPIC.md
    - docs/issues/closed/2130-rename-peer-updated-milliseconds-ago-to-updated-at-ms/ISSUE.md
---

<!-- skill-link: create-issue -->

# EPIC #144 - API V2: new API with the correct response codes

## Goal

Introduce a new major version of the tracker REST API under the `/v2` prefix with correct HTTP
semantics (status codes, error bodies, resource-oriented endpoints, unambiguous field meanings),
served in parallel with the current `/api/v1` API until the old one is removed in a later major
release.

## Why This Is Needed

GitHub issue #144 was opened in December 2022, before the project kept issue specifications in
the repository. This document is the in-repo specification of that EPIC, so agents and
contributors have the full context without querying GitHub. The text of the GitHub issue and its
comments is summarized here; the issue remains the place for discussion.

Problems with the current (v1) API that v1 cannot fix without breaking clients:

- Responses use almost only `200` and `500` for resource outcomes. Not-found, created and
  no-content cases are not distinguished. (Authentication failures and disabled capabilities
  are already distinguished in v1: `401` and `409`; v2 must preserve that.)
- Errors are not structured. The demo deployment returns `HTTP 500` with the internal text
  `Unhandled rejection: Err { reason: "unauthorized" }` for `GET /` and `GET /health_check`
  (observed 2026-04-09, see torrust/torrust-tracker-demo#13).
- Some endpoints behave like remote commands (`GET /api/whitelist/reload`) instead of
  resource operations.
- The original issue raises a privacy concern: a torrent lookup must not reveal whether a
  private tracker holds a torrent (Insecure Direct Object Reference). This is not a confirmed
  current vulnerability: v1 protects API routes with the auth middleware, so a missing or
  invalid token is rejected before any handler runs. v2 must preserve that non-disclosure and
  keep it when the status codes change (for example `404` for an absent torrent, for any caller
  that is not authorized to know).
- Some fields are ambiguous or deprecated but cannot be removed in v1 (see
  [v1 deprecations carried to v2](#v1-deprecations-carried-to-v2)).

## Constraints

- **v1 stays additive-only.** The contract governance rules of the closed
  [EPIC #1938](../../closed/1938-rest-api-contract-first-migration/EPIC.md#contract-evolution-governance)
  still apply: no field removal or rename in v1; everything breaking goes to v2.
- **Architecture.** v2 is built on the contract-first layering of the
  [contract-first ADR](../../../adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md):
  `rest-api-protocol` → `rest-api-application` → `rest-api-runtime-adapter` →
  `axum-rest-api-server`, plus `rest-api-client`. v2 is a new versioned module next to `v1`, not
  an in-place change of v1.
- **Timing.** No development starts before tracker **4.0.0** is released. Until then the only
  allowed work is specification-only pull requests (this spec, sub-issue specs, ADR drafts) and
  answering questions from other sessions about v2 intent.
- **No v2 contract decision is binding until it is recorded** in a sub-issue spec or an ADR
  merged to `develop`. Items marked _candidate_ below come from the original issue text and need
  a decision before implementation.

## Scope

### In Scope

- A `/v2` API prefix served in parallel with v1.
- Correct HTTP status codes and a structured error body for all v2 endpoints.
- Resource-oriented replacements for the command-style v1 endpoints.
- Explicit completed-download metric semantics (session versus persisted versus unavailable).
- Removal of the v1 fields and metrics that were deprecated for v2.
- A typed v2 representation for I2P peers, defined by an ADR before implementation.
- Updates to `rest-api-protocol`, `rest-api-application`, `rest-api-runtime-adapter`,
  `axum-rest-api-server`, `rest-api-client`, contract tests, user documentation and a migration
  guide.

### Out of Scope

- Removing v1 (a later major release decides that).
- Changing the HTTP or UDP tracker protocols.
- Changing tracker configuration behavior or the persistence schema.
- Extracting REST API packages to standalone repositories (per EPIC #1669 policy).
- Long-term ideas listed in the issue (JSON-LD schema, API version in a header instead of a URL
  prefix, a `POST /api/commands` endpoint). They may be reconsidered by a v2 ADR but are not
  committed scope.

## Inputs Collected From the GitHub Issue and the Repository

### Candidate v2 behavior (from the issue body)

All of these are _candidates_ until a sub-issue or ADR records the decision.

| Area                         | Candidate change                                                                                                            |
| ---------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| Prefix and URLs              | Use `/v2`, for example `/v2/api/stats`; keep the same URLs where possible and change the response codes.                    |
| Errors                       | One error type and body for all rejections; `401` when a token is present but not valid, instead of `token not valid` text. |
| `GET` torrent by info-hash   | `404` when the torrent does not exist, with a valid or an invalid token, so a private tracker does not leak what it holds.  |
| `DELETE` whitelist entry     | `204 No Content` on delete; `404 Not Found` when the torrent is absent or the token is invalid.                             |
| `POST` whitelist entry       | `201 Created` with a `Location` header; requires a new `GET` whitelist entry endpoint (or `PUT` as an idempotent add).      |
| `POST` auth key              | `201 Created`.                                                                                                              |
| `DELETE` auth key            | `204 No Content`.                                                                                                           |
| Reload whitelist / keys      | Use `POST`, `PUT` or `PATCH` (not `GET`); later possibly `POST /reload-whitelist` or a commands resource.                   |
| Root / discovery             | Fix the `500` on `/` and `/health_check`; consider an index listing all APIs (the rqbit `GET /` pattern).                   |
| Documentation                | Consider generating OpenAPI documents (utoipa was suggested for Axum).                                                      |

Disabled capabilities already return `409` in v1 (delivered by #2107). v2 must keep that
distinction: a disabled feature is not an authorization failure and not an accidental `404`.

### v1 deprecations carried to v2

These items are deprecated in v1 and are removed (not merely renamed) in v2:

- Stats and torrent `completed`: replaced by explicit session/persisted/unavailable semantics
  (see [ADR: completed-download metric retention names](../../../adrs/20260901113500_define_completed_download_metric_retention_names.md)
  and the draft sub-issue below). v2 also drops the legacy
  `tracker_core_persistent_torrents_downloads_total` compatibility path.
- Peer `updated` and `updated_milliseconds_ago`: v2 uses only `updated_at_ms`
  (see [#2130](../../closed/2130-rename-peer-updated-milliseconds-ago-to-updated-at-ms/ISSUE.md)).

### Cross-cutting inputs from other work

- **Stats and metrics shape (#1228).** The flat `Stats` response may be regrouped (for example
  torrent, HTTP tracker and UDP tracker sections) and metric names aligned with Prometheus naming
  recommendations. The
  [stats/metrics ADR](../../../adrs/20261105132341_define_the_stats_and_metrics_endpoints_and_require_help_and_type_in_prometheus_output.md)
  states `/stats` and `/metrics` both remain supported and that `/stats` changes only additively
  in v1; any regrouping is therefore a v2 decision.
- **I2P peers (#2491).** The I2P design review requires I2P peers to be returned in separate,
  explicitly typed collections, without overloading `peer_addr` with a Destination. The final
  versioned JSON contract needs an ADR under this EPIC before the I2P implementation exposes
  anything over REST.

## Subissues

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

The EPIC currently has **no created GitHub sub-issues** (the sub-issues API of #144 is empty at
the time of writing). The list below is the proposed breakdown; order and content need the
maintainer's approval before issues are created.

| Order | Issue                                                    | Local Spec                                                             | Status | Notes                                                                                                                                |
| ----- | -------------------------------------------------------- | ---------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------------------------------------------ |
| 1     | #[To be assigned] - Make the REST API persistence-aware  | `docs/issues/drafts/144-make-rest-api-persistence-aware/ISSUE.md`      | TODO   | Draft only, no GitHub issue yet. Defines completed-metric provenance DTOs.                                                           |
| 2     | #[To be assigned] - ADR: v2 contract and error model     | not drafted                                                            | TODO   | Candidate: prefix, error body, status-code policy, versioning, deprecation/migration policy, I2P collections, stats regrouping.      |
| 3     | #[To be assigned] - v2 skeleton and error handling       | not drafted                                                            | TODO   | Depends on 2. Candidate: `/v2` module in all five packages, shared error type, fix `500` on `/` and `/health_check`.                 |
| 4     | #[To be assigned] - v2 health_check, stats, torrent      | not drafted                                                            | TODO   | Depends on 1 and 3.                                                                                                                  |
| 5     | #[To be assigned] - v2 whitelist and auth keys           | not drafted                                                            | TODO   | Depends on 3. Covers `201`, `204`, `404`, `Location` and the reload endpoints.                                                       |
| 6     | #[To be assigned] - v2 client and migration guide        | not drafted                                                            | TODO   | Depends on 4 and 5.                                                                                                                  |
| 7     | #[To be assigned] - v2 I2P peer representation           | not drafted                                                            | TODO   | Depends on the #2491 I2P work and ADR (2).                                                                                           |

## Delivery Strategy

For each subissue implementation in this EPIC, the default completion policy is:

1. Run automatic checks (`linter all`, relevant tests, pre-push checks when applicable).
2. Run manual verification scenarios and record evidence.
3. Re-review acceptance criteria after implementation and update verification evidence.
4. Complete an evidence-based implementation review. Create or update an
   issue-local retrospective for reusable lessons, material design changes, or
   meaningful deviations from the plan; otherwise record why one was unnecessary
   in the issue progress log.

### Manual verification policy

The v2 contracts are still candidates, so this EPIC does not fix concrete commands or expected
results. Instead, every child spec must define, before implementation starts, concrete manual
scenarios with steps, expected results, and a status/evidence field per scenario. Commands,
output and relevant logs are retained in the child's issue-local `manual-verification-evidence.md`.
Across the EPIC, the scenarios must cover at least:

| Scenario area                    | Expected result to define in the child spec                                            | Status | Evidence |
| -------------------------------- | -------------------------------------------------------------------------------------- | ------ | -------- |
| v1 and v2 coexistence            | Both prefixes are served by one process; v1 responses are byte-compatible with before. | TODO   | -        |
| Unauthorized torrent lookups     | Missing/invalid token: same non-disclosing response for an existing and an absent one. | TODO   | -        |
| Authorized absent torrent lookup | `404` with the structured error body.                                                  | TODO   | -        |
| Capability availability          | Disabled whitelist/keys/persistence return the agreed code (`409` in v1), not `404`.   | TODO   | -        |
| Completed-metric availability    | Session, persisted and unavailable values are distinguishable, no numeric sentinel.    | TODO   | -        |

### Phase 0 - Specification (allowed now)

- Outcome: this EPIC spec merged; draft sub-issue specs and the v2 contract ADR drafted and
  reviewed; GitHub sub-issues created only after the maintainer approves the breakdown.
- Exit criteria: ADR merged; every sub-issue in the table has a reviewed local spec.

### Phase 1 - Foundations (after 4.0.0)

- Outcome: v2 module skeleton and common error handling, served in parallel with v1.
- Exit criteria: v2 routes mounted, structured errors in place, v1 behavior unchanged.

### Phase 2 - Endpoints

- Outcome: all v1 contexts re-exposed under v2 with the agreed codes and semantics.
- Exit criteria: contract tests for every v2 endpoint; client updated.

### Phase 3 - Migration and release

- Outcome: migration guide, deprecation notices in v1, release notes.
- Exit criteria: a release containing both v1 and v2; v1 removal scheduled by a separate decision.

### Parallelism and model/effort

How many sub-issue sessions may run at once, and the model and effort for each, are not decided
yet. Sub-issue 1-3 touch the same packages, so they run one at a time. Ask the maintainer before
starting parallel sessions. Do not start any implementation session before 4.0.0 is released.

## Progress Tracking

### Workflow Checkpoints

- [x] Epic issue exists on GitHub (#144, opened 2022-12)
- [ ] Epic spec reviewed and approved by user/maintainer
- [ ] Subissues created and linked in this spec
- [ ] Subissue statuses kept up to date in the `Subissues` table
- [ ] For each implemented subissue: automatic checks completed and recorded
- [ ] For each implemented subissue: manual verification completed and recorded
- [ ] For each implemented subissue: acceptance criteria reviewed post-implementation
- [ ] For each implemented subissue: implementation completion review recorded
- [ ] Epic acceptance criteria reviewed and checked off
- [ ] Epic issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-06 12:00 UTC - EPIC coordinator session (Claude Sonnet 5.5) - Created the in-repo EPIC
  spec from the GitHub issue, its comments and existing repository specs/ADRs. Development
  deferred until after tracker 4.0.0 (maintainer decision). Proposed sub-issue breakdown is
  unapproved.

## Acceptance Criteria

- [ ] All required subissues are created and linked.
- [ ] Implementation order is explicit and justified.
- [ ] Dependencies and blockers are documented and current.
- [ ] Epic status reflects actual state of linked subissues.
- [ ] v1 and v2 are served in parallel and v1 wire format is unchanged.
- [ ] Every v2 endpoint uses the agreed status codes and the structured error body.
- [ ] Deprecated v1 fields and metrics are absent from v2.
- [ ] A migration guide for v1 clients exists.
- [ ] Every completed subissue includes automated verification evidence.
- [ ] Every completed subissue includes manual verification evidence.
- [ ] Every completed subissue includes post-implementation acceptance criteria review.
- [ ] Every completed subissue includes an implementation completion review.
- [ ] Documentation and governance updates are included when required.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   | -        |
| AC2   | TODO                   | -        |

## Risks and Trade-offs

- **Long-lived EPIC.** The issue is nearly four years old and the repository changed a lot
  (contract-first packages, persistence-optional runtime). Mitigation: this spec records the
  current inputs; re-validate every candidate before creating its sub-issue.
- **Maintaining two APIs.** v1 and v2 in parallel double the surface. Mitigation: share the
  application and runtime-adapter layers; only protocol DTOs and transport differ.
- **Scope creep.** The issue lists many ideas (OpenAPI, JSON-LD, commands endpoint). Mitigation:
  only the in-scope list above is committed; anything else needs an ADR.
- **Security change in behavior.** Returning `404` for unauthorized lookups changes what clients
  can diagnose. Mitigation: document it in the migration guide.

## References

- Related issues: #144 (this EPIC), #141, #143, #999, #1228, #1938, #1978, #2107, #2122, #2130,
  #2491
- Related ADRs:
  - `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`
  - `docs/adrs/20260825193119_make_persistence_an_optional_application_composition_capability.md`
  - `docs/adrs/20260901113500_define_completed_download_metric_retention_names.md`
  - `docs/adrs/20261105132341_define_the_stats_and_metrics_endpoints_and_require_help_and_type_in_prometheus_output.md`
- Demo security review: torrust/torrust-tracker-demo#13
