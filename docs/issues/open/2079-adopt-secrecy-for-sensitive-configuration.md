---
doc-type: issue
issue-type: enhancement
status: open
priority: p1
github-issue: 2079
spec-path: docs/issues/open/2079-adopt-secrecy-for-sensitive-configuration.md
branch: "2079-adopt-secrecy-for-sensitive-configuration"
related-pr: null
last-updated-utc: 2026-08-21 17:00
semantic-links:
  skill-links:
    - create-issue
    - handle-secrets
  related-artifacts:
    - .github/skills/dev/rust-code-quality/handle-secrets/SKILL.md
    - packages/configuration/src/v2_0_0/tracker_api.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
    - docs/issues/open/1490-1978-decompose-database-configuration.md
---

# Issue #2079 - Adopt `secrecy` for sensitive configuration

## Goal

Use the Rust `secrecy` crate consistently for configuration API tokens in both schema versions. This makes secrets explicit in the public type system, redacts them automatically from `Debug` and `Display` output, clears them from memory when dropped, and makes every intentional exposure visible in code review.

## Background

Configuration currently represents API tokens and database credentials as plain `String` values. The application manually clones configuration and calls `mask_secrets()` before selected log output. That remains an important control, but it is easy to bypass through a new debug, display, error, or tracing path and does not let developers audit secret values by type.

The repository's `handle-secrets` skill already requires `Secret<String>` for passwords, API tokens, and credentials. The accepted [Torrust Tracker Deployer ADR: Use Secrecy Crate for Sensitive Data Handling](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/decisions/secrecy-crate-for-sensitive-data.md) independently reaches the same decision. It identifies automatic redaction, clearing secrets from memory, a searchable secret inventory, and explicit `expose_secret()` calls as the key benefits. Its rationale supports adopting the crate directly rather than building a custom wrapper.

This is the first of two refactors. It delivers immediate protection for API tokens in the active v2 configuration while establishing the dependency and usage conventions that #1490 consumes. #1490 subsequently decomposes only v3 database configuration and protects its new isolated password field with `Secret<String>` from the outset.

### Version-specific representation

| Schema version | API tokens                        | Database credentials                                                                                                                                             |
| -------------- | --------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| v2.0.0         | `HashMap<String, Secret<String>>` | No change. Network database URLs remain plain strings with the existing `mask_secrets()` behavior because the password is embedded in the legacy representation. |
| v3.0.0         | `HashMap<String, Secret<String>>` | No change in this issue. #1490 later introduces `ConnectionInfo.password: Secret<String>`; SQLite paths remain plain strings.                                    |

The TOML schema remains unchanged: users continue writing token values such as `access_tokens.admin = "..."`. Only the Rust public API for access tokens changes.

> **Release gate**: Changing public API-token values from `String` to `Secret<String>` is semver-breaking for Rust consumers. Do not publish a `torrust-tracker-configuration` release exposing the v3 types until this issue and #1490 are complete. If such a release is already published, schedule the type changes for the next major package version.

## Scope

### In Scope

- Add the current stable `secrecy` crate dependency at the appropriate workspace/package boundary.
- Represent configuration API tokens as `Secret<String>` in v2 and v3.
- Preserve TOML serialization and deserialization of API tokens without changing the configuration-file surface.
- Retain v2 and current v3 database URL masking; #1490 separately removes only its superseded v3 database redaction after isolating the password.
- Replace selected API-token redaction code paths with type-level protection and expose values only at runtime integration boundaries.
- Add focused tests that assert the exact `Secret([REDACTED])` representation and that actual test tokens never appear.
- Audit configuration logging, display, debug, tracing, and error contexts for accidental API-token exposure.
- Update the secret-handling skill and relevant documentation describing the old manual convention.

### Out of Scope

- Changing v2 or v3 TOML field names or configuration file syntax.
- Protecting database credentials, including wrapping legacy v2/v3 database URLs. #1490 introduces and protects only the new isolated v3 password.
- Encrypting configuration files or secrets at rest.
- Introducing a custom wrapper around `secrecy::Secret<T>`.
- Applying secret types outside configuration unless an audit finds a direct configuration boundary that requires it.

## Design Constraints

1. Use `secrecy::Secret<String>` directly. Do not add a custom wrapper with duplicate behavior.
2. Enable the crate's `serde` feature and use the crate-supported serialization mechanism required for configuration deserialization and serialization; document the intentional opt-in.
3. Permit `.expose_secret()` only at the last possible runtime boundary, such as authenticating a request.
4. Never call `.expose_secret()` in logs, tracing instrumentation, `Debug`, `Display`, errors, test assertion messages, or user-visible output.
5. Treat `Secret([REDACTED])` as the exact expected debug representation in tests.
6. Keep API-token type changes and #1490's v3 database-password type change in the same release window as v3 publication rather than publishing a short-lived v3 API that must immediately receive another major bump.

## Implementation Plan

| ID  | Status | Task                                            | Notes                                                                                |
| --- | ------ | ----------------------------------------------- | ------------------------------------------------------------------------------------ |
| T1  | TODO   | Add and configure `secrecy`                     | Use the current stable version and required serialization support.                   |
| T2  | TODO   | Define configuration secret aliases/conventions | Keep usage direct and discoverable; document any aliases if they add domain clarity. |
| T3  | TODO   | Protect v2 API tokens                           | Wrap tokens and update authentication, bootstrap, and test consumers.                |
| T4  | TODO   | Protect v3 API tokens                           | Wrap tokens and update v3 consumers without changing the database URL.               |
| T5  | TODO   | Preserve database URL masking                   | Confirm v2 and current v3 database `mask_secrets()` behavior remains unchanged.      |
| T6  | TODO   | Audit exposure boundaries                       | Verify logging, tracing, errors, debug, display, and CLI JSON output.                |
| T7  | TODO   | Update policy documentation                     | Align the secret-handling skill and output-redaction docs with the implementation.   |
| T8  | TODO   | Verify release readiness                        | Run targeted tests, workspace tests, linters, and manual redaction scenarios.        |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2079)
- [ ] (Recommended) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and pre-push checks)
- [ ] Manual verification scenarios executed and recorded
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-08-21 00:00 UTC - josecelano - Drafted from #1490 as the secret-handling effort.
- 2026-08-21 16:45 UTC - josecelano - Reordered the two refactors: implement this smaller API-token-focused change first. Do not wrap legacy database URLs; #1490 later isolates and protects the v3 database password.
- 2026-08-21 17:00 UTC - Copilot/User - Maintainer approved the draft; created GitHub issue #2079 and moved the specification to open issues.

## Acceptance Criteria

- [ ] AC1: `secrecy` is the standard direct type for configuration API tokens in both v2 and v3.
- [ ] AC2: v2 and v3 API tokens use `Secret<String>`; legacy database URL types and masking remain unchanged.
- [ ] AC3: Deserializing existing v2 and v3 TOML API-token values remains compatible without syntax changes.
- [ ] AC4: Formatting configuration values containing test API tokens produces the exact `Secret([REDACTED])` literal and never reveals the actual values.
- [ ] AC5: Every `.expose_secret()` call is limited to a runtime integration boundary and absent from logs, tracing, errors, and user-visible output.
- [ ] AC6: API-token manual masking is removed or replaced without weakening the CLI JSON output redaction policy; database URL masking remains in place.
- [ ] AC7: The secret-handling skill and relevant documentation describe the implemented convention.
- [ ] AC8: This issue and #1490 are merged before publishing the configuration crate's v3 public API.
- [ ] `linter all` exits with code `0`.
- [ ] Relevant tests pass.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-configuration`
- `cargo test -p torrust-tracker-axum-rest-api-server`
- `cargo test -p torrust-tracker-core`
- `cargo test --workspace`
- `linter all`

### Manual Verification Scenarios

| ID  | Scenario                  | Command/Steps                                                                            | Expected Result                                                                       | Status | Evidence |
| --- | ------------------------- | ---------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------- | ------ | -------- |
| M1  | Verify v2 formatting      | Deserialize v2 TOML containing a unique API token and format the config with `Debug`.    | The token displays exactly as `Secret([REDACTED])`; the actual token does not appear. | TODO   |          |
| M2  | Verify v3 formatting      | Deserialize v3 TOML containing a unique API token and format the config with `Debug`.    | The token displays exactly as `Secret([REDACTED])`; the actual token does not appear. | TODO   |          |
| M3  | Verify runtime access     | Start authenticated API test paths for both configuration versions.                      | Authentication receives the actual token only at the required integration boundary.   | TODO   |          |
| M4  | Verify operational output | Run configuration/startup logging and CLI JSON diagnostic paths with unique test tokens. | No actual token appears in logs, tracing output, errors, or JSON.                     | TODO   |          |

### Acceptance Verification

| AC ID | Status | Evidence |
| ----- | ------ | -------- |
| AC1   | TODO   |          |
| AC2   | TODO   |          |
| AC3   | TODO   |          |
| AC4   | TODO   |          |
| AC5   | TODO   |          |
| AC6   | TODO   |          |
| AC7   | TODO   |          |
| AC8   | TODO   |          |

## Risks and Trade-offs

- **Public API break**: `Secret<String>` changes API-token construction, comparison, and access for downstream Rust consumers. Mitigation: complete it with #1490 before the v3 public API is published and document it in the release notes.
- **Serialization opt-in**: Configuration requires intentional serialization/deserialization support for API tokens. Mitigation: use the supported crate mechanism and add regression tests for both schemas.
- **False sense of security**: `Secret<T>` cannot prevent exposure after an explicit `.expose_secret()`. Mitigation: audit exposures and make them narrowly scoped and reviewable.
- **Manual-redaction scope**: Removing database URL masking would weaken handling of a legacy credential-bearing string. Mitigation: preserve it; #1490 replaces only the v3 representation with an isolated secret password.

## References

- Follow-up: [#1490 — Decompose v3 database configuration](1490-1978-decompose-database-configuration.md).
- Related issue: #1441 (secret leak through tracing).
- Repository policy: [Handle secrets skill](../../../.github/skills/dev/rust-code-quality/handle-secrets/SKILL.md).
- Repository policy: [Global CLI output contract ADR](../../adrs/20260519000000_define_global_cli_output_contract.md).
- External architectural reference: [Torrust Tracker Deployer ADR: Use Secrecy Crate for Sensitive Data Handling](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/decisions/secrecy-crate-for-sensitive-data.md).
- [Secrecy crate documentation](https://docs.rs/secrecy/).
