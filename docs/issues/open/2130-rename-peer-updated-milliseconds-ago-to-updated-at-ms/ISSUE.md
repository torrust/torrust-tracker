---
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: null
github-issue: 2130
spec-path: docs/issues/open/2130-rename-peer-updated-milliseconds-ago-to-updated-at-ms/ISSUE.md
branch: 2130-add-peer-updated-at-ms
related-pr: null
last-updated-utc: 2026-09-02 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/rest-api-protocol/src/v1/context/torrent/resources/peer.rs
    - packages/rest-api-runtime-adapter/src/v1/conversion.rs
    - packages/axum-rest-api-server/src/v1/context/torrent/mod.rs
    - packages/rest-api-client/src/v1/client.rs
    - docs/issues/closed/1930-1669-si-33-rest-api-contract-first-architecture.md
---

# Issue #2130 - Add `peer.updated_at_ms` to v1 REST API

## Goal

Expose an unambiguous `updated_at_ms` peer timestamp in the current v1 REST API while preserving both existing timestamp fields for v1 client compatibility. Mark the misnamed `updated_milliseconds_ago` field as deprecated so clients can migrate before API v2 uses only the corrected name.

## Problem

The v1 peer DTO field `updated_milliseconds_ago` has a misleading name. It behaves as an **absolute Unix timestamp** (milliseconds since epoch) of the peer's last announce, but the `_ago` suffix implies a **relative duration** ("how long ago since the last update").

Both `updated` (deprecated) and `updated_milliseconds_ago` hold the **same value** — they were populated identically in the original implementation and remain identical in the current runtime adapter.

## Evidence

The field was introduced in commit `bc3d246f` (Nov 2022, "feat(api): in torrent endpoint rename field to"). The diff shows:

```diff
+    #[deprecated(since = "2.0.0", note = "please use `updated_milliseconds_ago` instead")]
     pub updated: u128,
+    pub updated_milliseconds_ago: u128,
```

Both populated identically:

```rust
updated: peer.updated.as_millis(),                          // Unix timestamp in ms
updated_milliseconds_ago: peer.updated.as_millis(),          // same value
```

Where `peer.updated` is of type `DurationSinceUnixEpoch` — an absolute Unix timestamp measured in milliseconds.

The original intent was **hypothesis #2**: to add the unit "milliseconds" to the field name so clients know the value is in ms rather than seconds. The `_ago` suffix is a misnomer.

The historic analysis appears in the Follow-up Tasks section of `docs/issues/closed/1930-1669-si-33-rest-api-contract-first-architecture.md`. That closed issue proposed a breaking rename to `updated_milliseconds` plus removal of `updated`. This specification supersedes that proposal with an additive, migration-safe v1 change. API v2, planned separately in EPIC #144, will use only the corrected `updated_at_ms` name.

## Scope

### In Scope

- Add a required `updated_at_ms: u128` field to the v1 `Peer` protocol DTO.
- Serialize and deserialize `updated_at_ms` as an absolute Unix timestamp in milliseconds since epoch.
- Retain and deprecate `updated_milliseconds_ago`; retain the already deprecated `updated` field.
- Populate all three v1 timestamp fields from the same domain timestamp.
- Add independent conversion and raw JSON contract coverage for the additive wire contract.
- Update v1 endpoint documentation and Rust field documentation with the accurate absolute-timestamp semantics and migration direction.

| Field                      | Status                       | Value                | API v2 status |
| -------------------------- | ---------------------------- | -------------------- | ------------- |
| `updated`                  | stays deprecated             | Unix timestamp in ms | removed       |
| `updated_milliseconds_ago` | stays but becomes deprecated | Unix timestamp in ms | removed       |
| `updated_at_ms`            | new                          | Unix timestamp in ms | retained      |

### Out of Scope

- Removing or renaming either deprecated field in v1.
- Changing the domain type `DurationSinceUnixEpoch` or domain `peer::Peer`.
- Any API v2 contract implementation.
- Retroactively changing the closed #1930 issue specification.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`
- Decision: use an additive v1 field and Rust deprecation instead of a v1 rename/removal. This preserves server-to-existing-client compatibility while enabling API v2 to use only `updated_at_ms`.
- Decision: `updated_at_ms` names the timestamp point-in-time and its unit; it is not a relative duration.
- Compatibility caveat: a newly compiled typed client expects the required `updated_at_ms` field when deserializing a peer from an older server. This version-skew limitation is accepted because the field is required in the new protocol contract.
- ADRs to create: None known. Reassess during implementation if the API versioning policy or cross-package REST contract changes materially.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                           | Notes / Expected Output                                                                                        |
| --- | ------ | ------------------------------ | -------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Extend the v1 peer DTO         | Add and document required `updated_at_ms`; deprecate the misleading legacy field with a migration note.        |
| T2  | TODO   | Map the domain timestamp       | Update `from_domain_peer` so all three v1 timestamp fields equal `DurationSinceUnixEpoch::as_millis()`.        |
| T3  | TODO   | Protect the REST contract      | Add independent conversion assertions and a raw endpoint JSON assertion for all three keys and equal values.   |
| T4  | TODO   | Update consumer-facing docs    | Add `updated_at_ms` and accurate legacy-field semantics to the torrent endpoint example and API documentation. |
| T5  | TODO   | Validate and review acceptance | Execute automatic and mandatory manual checks, then record acceptance evidence.                                |

## Implementation Considerations

| Area               | File(s)                                                                     | Change                                                                                 |
| ------------------ | --------------------------------------------------------------------------- | -------------------------------------------------------------------------------------- |
| Protocol DTO       | `packages/rest-api-protocol/src/v1/context/torrent/resources/peer.rs`       | Add and document `updated_at_ms`; deprecate `updated_milliseconds_ago`.                |
| Runtime adapter    | `packages/rest-api-runtime-adapter/src/v1/conversion.rs`                    | Populate `updated_at_ms` and independently assert the timestamp values.                |
| Axum contract test | `packages/axum-rest-api-server/tests/server/v1/contract/context/torrent.rs` | Assert raw endpoint JSON includes all three timestamp keys with equal values.          |
| API documentation  | `packages/axum-rest-api-server/src/v1/context/torrent/mod.rs`               | Show `updated_at_ms` and correct timestamp semantics in the endpoint example.          |
| REST API client    | `packages/rest-api-client/src/v1/client.rs`                                 | No implementation change expected; confirm typed peer deserialization remains covered. |

The previously proposed Axum and qBittorrent E2E DTO-literal edits are not required: those paths have no inline `Peer` literals or named-field parsing.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`.
- [x] Spec reviewed and approved by user/maintainer.
- [x] GitHub issue #2130 created and issue number added to this spec.
- [x] Spec moved to `docs/issues/open/` using the assigned issue number.
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation.
- [ ] Implementation completed.
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks).
- [ ] Manual verification scenarios executed and recorded (status + evidence).
- [ ] Acceptance criteria reviewed after implementation and updated with evidence.
- [ ] Reviewer validated acceptance criteria and updated checkboxes.
- [ ] Committer verified spec progress is up to date before commit.
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`.

### Progress Log

- 2026-06-24 00:00 UTC - Planning - Initial draft created from the REST API contract-first follow-up analysis - `bc3d246f`, issue #1930.
- 2026-09-02 00:00 UTC - Copilot - Rebased draft on the current issue-spec template; corrected paths and scope after codebase review - draft ready for user review.
- 2026-09-02 00:00 UTC - User - Approved the draft specification and requested a folder-style layout - approval recorded.
- 2026-09-02 00:00 UTC - User - Clarified this is a standalone current-API refactor, not an EPIC #144 subissue; API v2 will use only `updated_at_ms` - scope and relationship updated.
- 2026-09-02 00:00 UTC - Copilot - Created GitHub issue #2130 and moved the approved specification to the open-issues folder - https://github.com/torrust/torrust-tracker/issues/2130.

## Acceptance Criteria

- [ ] AC1: The v1 `Peer` DTO serializes and deserializes a required `updated_at_ms: u128` field documented as an absolute Unix timestamp in milliseconds since epoch.
- [ ] AC2: `from_domain_peer` maps `updated_at_ms` from `DurationSinceUnixEpoch::as_millis()`.
- [ ] AC3: Until API v2, `updated`, `updated_milliseconds_ago`, and `updated_at_ms` are all serialized for a returned peer and contain the same value; both legacy fields are deprecated with a migration path to `updated_at_ms`.
- [ ] AC4: The v1 torrent endpoint documentation accurately shows all timestamp fields and their absolute-time semantics.
- [ ] AC5: This issue documents that API v2 is planned to use `updated_at_ms` and omit `updated` and `updated_milliseconds_ago`; implementing API v2 remains out of scope.
- [ ] AC6: `linter all`, relevant tests, and applicable pre-push checks pass.
- [ ] AC7: Manual verification scenarios are executed and documented with status and evidence.
- [ ] AC8: Acceptance criteria are re-reviewed after implementation and reflect observed behavior.

## Verification Plan

Define verification before implementation starts and execute it before closing the issue.

### Automatic Checks

- `cargo test -p torrust-tracker-rest-api-protocol`
- `cargo test -p torrust-tracker-rest-api-runtime-adapter`
- `cargo test -p torrust-tracker-axum-rest-api-server`
- `cargo +nightly doc --no-deps --workspace --all-features`
- `linter all`
- Pre-push checks when the implementation is ready to push.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                             | Command/Steps                                                                                                                   | Expected Result                                                                                                                                         | Status | Evidence                |
| --- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | ----------------------- |
| M1  | Inspect the v1 torrent wire response | Start a local tracker, announce a known peer, then request `GET /api/v1/torrent/{info_hash}` with an authorized `curl` request. | The peer JSON contains integer `updated`, `updated_milliseconds_ago`, and `updated_at_ms` fields, and all values are equal Unix-millisecond timestamps. | TODO   | Pending implementation. |
| M2  | Verify typed-client deserialization  | Deserialize the M1 response through the current `ApiClient`/`Torrent` model.                                                    | Deserialization succeeds and exposes the peer's `updated_at_ms` value.                                                                                  | TODO   | Pending implementation. |
| M3  | Inspect generated API documentation  | Build and inspect the generated Rust documentation for `Peer` and the torrent endpoint.                                         | `updated_at_ms` and the migration-only legacy fields are documented with accurate absolute-time semantics.                                              | TODO   | Pending implementation. |

Notes:

- Manual verification is mandatory even when automated tests pass.
- If a scenario fails, record the failure and diagnosis in the progress log before proceeding.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                |
| ----- | ---------------------- | ----------------------- |
| AC1   | TODO                   | Pending implementation. |
| AC2   | TODO                   | Pending implementation. |
| AC3   | TODO                   | Pending implementation. |
| AC4   | TODO                   | Pending implementation. |
| AC5   | TODO                   | Pending implementation. |
| AC6   | TODO                   | Pending implementation. |
| AC7   | TODO                   | Pending implementation. |
| AC8   | TODO                   | Pending implementation. |

## Risks and Trade-offs

- New typed clients cannot deserialize a peer response from an older server that lacks the required field. Documenting and accepting this version skew keeps the new v1 wire contract explicit.
- Keeping three equivalent response fields temporarily adds payload and maintenance cost. This is intentional migration compatibility until API v2.
- A DTO round-trip test alone can hide a missing serialized field because the server and test share the same type. A raw JSON assertion mitigates this.

## References

- API v2 EPIC: #144
- Historical analysis: #1930
- Related ADR: `docs/adrs/20260623200526_adopt_contract-first_architecture_for_rest_api.md`
