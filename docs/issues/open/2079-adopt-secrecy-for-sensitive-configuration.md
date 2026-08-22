---
doc-type: issue
issue-type: enhancement
status: open
priority: p1
github-issue: 2079
spec-path: docs/issues/open/2079-adopt-secrecy-for-sensitive-configuration.md
branch: "2079-adopt-secrecy-for-sensitive-configuration"
related-pr: null
last-updated-utc: 2026-08-22 00:00
semantic-links:
  skill-links:
    - create-issue
    - handle-secrets
  related-artifacts:
    - .github/skills/dev/rust-code-quality/handle-secrets/SKILL.md
    - docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md
    - packages/configuration/src/v2_0_0/tracker_api.rs
    - packages/configuration/src/v3_0_0/tracker_api.rs
    - docs/issues/open/1490-1978-decompose-database-configuration.md
---

# Issue #2079 - Adopt `secrecy` for sensitive configuration

## Goal

Use the Rust `secrecy` crate consistently for configuration API tokens in both schema versions. This makes secrets explicit in the public type system, redacts them automatically from `Debug` and `Display` output, clears them from memory when dropped, and makes every intentional exposure visible in code review.

## Background

Configuration currently represents API tokens and database credentials as plain `String` values. The application manually clones configuration and calls `mask_secrets()` before selected log output. That remains an important control, but it is easy to bypass through a new debug, display, error, or tracing path and does not let developers audit secret values by type.

The repository's `handle-secrets` skill requires the current stable `secrecy` string-secret type, `SecretString`, for passwords, API tokens, and credentials. The accepted [Torrust Tracker Deployer ADR: Use Secrecy Crate for Sensitive Data Handling](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/decisions/secrecy-crate-for-sensitive-data.md) independently reaches the same decision. It identifies automatic redaction, clearing secrets from memory, a searchable secret inventory, and explicit `expose_secret()` calls as the key benefits. Its rationale supports adopting the crate directly rather than building a custom wrapper.

This is the first of two refactors. It delivers immediate protection for API tokens in the active v2 configuration while establishing the dependency and usage conventions that #1490 consumes. #1490 subsequently decomposes only v3 database configuration and protects its new isolated password field with `SecretString` from the outset.

### Version-specific representation

| Schema version | API tokens                      | Database credentials                                                                                                                                             |
| -------------- | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| v2.0.0         | `HashMap<String, SecretString>` | No change. Network database URLs remain plain strings with the existing `mask_secrets()` behavior because the password is embedded in the legacy representation. |
| v3.0.0         | `HashMap<String, SecretString>` | No change in this issue. #1490 later introduces `ConnectionInfo.password: SecretString`; SQLite paths remain plain strings.                                      |

The TOML schema remains unchanged: users continue writing token values such as `access_tokens.admin = "..."`. Only the Rust public API for access tokens changes.

> **Release gate**: Changing public API-token values from `String` to `SecretString` is semver-breaking for Rust consumers. Do not publish a `torrust-tracker-configuration` release exposing the v3 types until this issue and #1490 are complete. If such a release is already published, schedule the type changes for the next major package version.

## Scope

### In Scope

- Add the current stable `secrecy` crate dependency at the appropriate workspace/package boundary.
- Represent configuration API tokens as `SecretString` in v2 and v3.
- Preserve TOML serialization and deserialization of API tokens without changing the configuration-file surface.
- Review default, example, fixture, and documentation TOML configurations affected by the type migration; retain their existing token syntax and update any Rust-facing examples that require explicit secret construction.
- Retain v2 and current v3 database URL masking; #1490 separately removes only its superseded v3 database redaction after isolating the password.
- Replace selected API-token redaction code paths with type-level protection and expose values only at runtime integration boundaries.
- Add focused tests that assert the current stable crate's exact `SecretBox<str>([REDACTED])` representation and that actual test tokens never appear.
- Audit configuration logging, display, debug, tracing, and error contexts for accidental API-token exposure.
- Update the secret-handling skill and relevant documentation describing the old manual convention.

### Out of Scope

- Changing v2 or v3 TOML field names or configuration file syntax.
- Protecting database credentials, including wrapping legacy v2/v3 database URLs. #1490 introduces and protects only the new isolated v3 password.
- Encrypting configuration files or secrets at rest.
- Introducing a custom wrapper around `secrecy` secret types.
- Applying secret types outside configuration unless an audit finds a direct configuration boundary that requires it.

## Architectural Decisions

- Related ADRs: [Adopt `secrecy` for sensitive values](../../adrs/20260822094338_adopt_secrecy_for_sensitive_values.md)
- ADRs created by this issue: `docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md`

## Design Constraints

1. Use the current stable `secrecy::SecretString` type directly. Do not add a custom wrapper with duplicate behavior.
2. Enable the crate's `serde` feature for configuration deserialization. Use a narrow, explicit TOML serialization boundary because `SecretString` intentionally does not serialize automatically; document the intentional exposure.
3. Permit `.expose_secret()` only at the last possible runtime boundary, such as authenticating a request.
4. Never call `.expose_secret()` in logs, tracing instrumentation, `Debug`, `Display`, errors, test assertion messages, or user-visible output.
5. Treat `SecretBox<str>([REDACTED])` as the exact expected debug representation in tests.
6. Keep API-token type changes and #1490's v3 database-password type change in the same release window as v3 publication rather than publishing a short-lived v3 API that must immediately receive another major bump.

## Implementation Plan

| ID  | Status | Task                                            | Notes                                                                             |
| --- | ------ | ----------------------------------------------- | --------------------------------------------------------------------------------- |
| T1  | DONE   | Add and configure `secrecy`                     | Added stable `secrecy` 0.10 with serde support in configuration.                  |
| T2  | DONE   | Define configuration secret aliases/conventions | Added the shared `AccessTokens = HashMap<String, SecretString>` alias and ADR.    |
| T3  | DONE   | Protect v2 API tokens                           | Protected tokens, retained TOML syntax, and updated runtime/test consumers.       |
| T4  | DONE   | Protect v3 API tokens                           | Protected tokens, retained TOML syntax, and left database URLs unchanged.         |
| T5  | DONE   | Preserve database URL masking                   | Retained both v2 and v3 database `mask_secrets()` implementations.                |
| T6  | DONE   | Audit exposure boundaries                       | Exposures are limited to TOML persistence, authentication, and test-client setup. |
| T7  | DONE   | Update policy documentation                     | Updated skill, linked issue specifications, and added an ADR.                     |
| T8  | DONE   | Verify release readiness                        | Targeted, workspace, and full-linter checks pass.                                 |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2079)
- [ ] (Recommended) Spec-only PR merged into `develop` before implementation
- [x] Implementation completed
- [x] Automatic verification completed (`linter all` and relevant tests)
- [x] Manual verification scenarios executed and recorded
- [x] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Issue closed and spec moved to `docs/issues/closed/`

### Progress Log

- 2026-08-21 00:00 UTC - josecelano - Drafted from #1490 as the secret-handling effort.
- 2026-08-21 16:45 UTC - josecelano - Reordered the two refactors: implement this smaller API-token-focused change first. Do not wrap legacy database URLs; #1490 later isolates and protects the v3 database password.
- 2026-08-21 17:00 UTC - Copilot/User - Maintainer approved the draft; created GitHub issue #2079 and moved the specification to open issues.
- 2026-08-22 UTC - User - Confirmed that the configuration crate's v3 API is unreleased. Regression testing is sufficient to prove unchanged TOML syntax, but implementation must review configuration TOML files and update examples affected by the Rust type migration. Do not create a separate spec-only commit or pull request; record implementation discoveries in this spec as needed.
- 2026-08-22 UTC - User - Confirmed that the dependency-freshness policy is authoritative: use the latest stable `secrecy` release. Its direct string-secret type is `SecretString`, which formats as `SecretBox<str>([REDACTED])`; explicit TOML serialization is required while diagnostic JSON remains redacted.
- 2026-08-22 UTC - Copilot/User - Created ADR `20260822094338_adopt_secrecy_for_sensitive_values.md` to establish project-wide `secrecy` conventions, including current-stable dependency selection, narrow serialization boundaries, and explicit runtime exposure rules.
- 2026-08-22 UTC - Copilot - Implemented `SecretString` API tokens in both schemas, audited the four explicit exposure sites, and verified configuration serialization, output redaction, authentication, workspace tests, and all linters.

## Acceptance Criteria

- [x] AC1: `secrecy::SecretString` is the standard direct type for configuration API tokens in both v2 and v3.
- [x] AC2: v2 and v3 API tokens use `SecretString`; legacy database URL types and masking remain unchanged.
- [x] AC3: Deserializing existing v2 and v3 TOML API-token values remains compatible without syntax changes.
- [x] AC4: Formatting configuration values containing test API tokens produces the exact `SecretBox<str>([REDACTED])` literal and never reveals the actual values.
- [x] AC5: Every `.expose_secret()` call is limited to a runtime integration boundary and absent from logs, tracing, errors, and user-visible output.
- [x] AC6: API-token manual masking is removed or replaced without weakening the CLI JSON output redaction policy; database URL masking remains in place.
- [x] AC7: The secret-handling skill and relevant documentation describe the implemented convention.
- [x] AC8: This issue and #1490 are release-gated before publishing the configuration crate's v3 public API.
- [x] `linter all` exits with code `0`.
- [x] Relevant tests pass.

## Verification Plan

### Automatic Checks

- `cargo test -p torrust-tracker-configuration`
- `cargo test -p torrust-tracker-axum-rest-api-server`
- `cargo test -p torrust-tracker-core`
- `cargo test --workspace`
- `linter all`

### Manual Verification Scenarios

| ID  | Scenario                  | Command/Steps                                                                            | Expected Result                                                                               | Status | Evidence                                                     |
| --- | ------------------------- | ---------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- | ------ | ------------------------------------------------------------ |
| M1  | Verify v2 formatting      | Deserialize v2 TOML containing a unique API token and format the config with `Debug`.    | The token displays exactly as `SecretBox<str>([REDACTED])`; the actual token does not appear. | DONE   | `cargo +stable test -p torrust-tracker-configuration`        |
| M2  | Verify v3 formatting      | Deserialize v3 TOML containing a unique API token and format the config with `Debug`.    | The token displays exactly as `SecretBox<str>([REDACTED])`; the actual token does not appear. | DONE   | `cargo +stable test -p torrust-tracker-configuration`        |
| M3  | Verify runtime access     | Start authenticated API test paths for both configuration versions.                      | Authentication receives the actual token only at the required integration boundary.           | DONE   | `cargo +stable test -p torrust-tracker-axum-rest-api-server` |
| M4  | Verify operational output | Run configuration/startup logging and CLI JSON diagnostic paths with unique test tokens. | No actual token appears in logs, tracing output, errors, or JSON.                             | DONE   | `cargo +stable test -p torrust-tracker-configuration`        |

### Acceptance Verification

| AC ID | Status | Evidence                                                  |
| ----- | ------ | --------------------------------------------------------- |
| AC1   | DONE   | `packages/configuration/src/lib.rs`                       |
| AC2   | DONE   | Configuration package tests and retained database masking |
| AC3   | DONE   | v2/v3 TOML serialization tests                            |
| AC4   | DONE   | v2/v3 redaction tests                                     |
| AC5   | DONE   | Audited `expose_secret()` call sites                      |
| AC6   | DONE   | v2/v3 JSON-redaction tests                                |
| AC7   | DONE   | Secret-handling skill and ADR                             |
| AC8   | DONE   | Release gate retained in #2079 and #1490                  |

## Risks and Trade-offs

- **Public API break**: `SecretString` changes API-token construction, comparison, and access for downstream Rust consumers. Mitigation: complete it with #1490 before the v3 public API is published and document it in the release notes.
- **Serialization opt-in**: Configuration requires intentional serialization/deserialization support for API tokens. Mitigation: use the supported crate mechanism and add regression tests for both schemas.
- **False sense of security**: `SecretString` cannot prevent exposure after an explicit `.expose_secret()`. Mitigation: audit exposures and make them narrowly scoped and reviewable.
- **Manual-redaction scope**: Removing database URL masking would weaken handling of a legacy credential-bearing string. Mitigation: preserve it; #1490 replaces only the v3 representation with an isolated secret password.

## References

- Follow-up: [#1490 — Decompose v3 database configuration](1490-1978-decompose-database-configuration.md).
- Related issue: #1441 (secret leak through tracing).
- Repository policy: [Handle secrets skill](../../../.github/skills/dev/rust-code-quality/handle-secrets/SKILL.md).
- Architecture: [Adopt `secrecy` for sensitive values](../../adrs/20260822094338_adopt_secrecy_for_sensitive_values.md).
- Repository policy: [Global CLI output contract ADR](../../adrs/20260519000000_define_global_cli_output_contract.md).
- External architectural reference: [Torrust Tracker Deployer ADR: Use Secrecy Crate for Sensitive Data Handling](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/decisions/secrecy-crate-for-sensitive-data.md).
- [Secrecy crate documentation](https://docs.rs/secrecy/).
