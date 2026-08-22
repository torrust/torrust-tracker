---
semantic-links:
  skill-links:
    - create-adr
    - handle-secrets
  related-artifacts:
    - .github/skills/dev/rust-code-quality/handle-secrets/SKILL.md
    - packages/configuration/src/lib.rs
    - docs/issues/open/2079-adopt-secrecy-for-sensitive-configuration.md
---

# Adopt `secrecy` for Sensitive Values

## Description

Credentials represented as plain `String` values can be accidentally disclosed by
`Debug`, `Display`, tracing fields, error contexts, snapshots, or operational
configuration output. Manual masking helps at selected output paths but neither
makes the sensitive nature of a value visible in Rust's type system nor protects
new diagnostics by default.

The project needs one durable convention for API tokens, passwords, private keys,
and comparable credentials. The convention must support configuration
serialization without weakening operational-output redaction and must make every
intentional read of a secret easy to audit.

## Agreement

Use the current stable [`secrecy`](https://docs.rs/secrecy/) crate directly for
sensitive in-memory values.

- Use `secrecy::SecretString` for string credentials, including API tokens and
  isolated passwords. Do not create a project wrapper that duplicates `secrecy`.
- Enable the crate's `serde` feature where a secret needs to deserialize from a
  configuration source. Retain the existing external configuration syntax.
- `SecretString` intentionally does not implement `Serialize`. Implement a
  narrow, explicit serializer only at configuration write boundaries that must
  persist a value, and expose a secret only for that serializer's immediate
  operation.
- Keep diagnostics, JSON output, tracing, `Debug`, `Display`, errors, and test
  assertion messages redacted. `SecretString` formats as
  `SecretBox<str>([REDACTED])`; tests must assert that exact representation and
  confirm a unique test secret is absent.
- Call `ExposeSecret::expose_secret()` only at the last runtime boundary that
  consumes the real value, such as comparing an inbound API credential or
  constructing an outbound authentication request. Never expose a secret for
  logging, formatting, error text, or incidental test inspection.
- Preserve manual redaction for existing credential-bearing plain strings until
  they are migrated to an isolated secret field. In particular, legacy database
  URLs retain their masking until their passwords are separated.
- Prefer the latest stable `secrecy` release. Do not pin an obsolete version to
  preserve a former type spelling or debug representation unless a concrete
  compatibility or security constraint is documented.

## Consequences

### Positive

- Sensitive values are explicit in public Rust APIs and are redacted by default
  in common diagnostic formatting paths.
- Intentional secret exposures are searchable and reviewable.
- `SecretString` clears its allocation when dropped.
- Existing configuration TOML remains compatible while operational output keeps
  its redaction policy.

### Negative

- Consumers must explicitly expose a value at legitimate integration boundaries.
- Configuration serialization needs an audited serializer because `SecretString`
  rejects automatic serialization by design.
- Changing a public credential field from `String` to `SecretString` is a
  semver-breaking API change.

## Alternatives Considered

**Continue using plain `String` with manual masking.** Rejected because a new
formatting or tracing path can bypass masking and the type system cannot identify
credentials for reviewers.

**Use a project-specific secret wrapper.** Rejected because `secrecy` provides the
required redaction and memory-clearing behavior, and a wrapper would duplicate its
API and obscure established practices.

**Pin an older `secrecy` release for `Secret<String>`.** Rejected because the
project's dependency-freshness policy requires the latest stable release absent
a documented compatibility or security reason.

## Affected Code

- [`AccessTokens`](../../packages/configuration/src/lib.rs) defines the shared
  configuration credential type.
- The [secret-handling skill](../../.github/skills/dev/rust-code-quality/handle-secrets/SKILL.md)
  provides implementation and review guidance.
- [Issue #2079](../issues/open/2079-adopt-secrecy-for-sensitive-configuration.md)
  applies this decision first to API tokens.

## Date

2026-08-22

## References

- Issue #2079: [Adopt `secrecy` for sensitive configuration](../issues/open/2079-adopt-secrecy-for-sensitive-configuration.md)
- Follow-up issue #1490: [Decompose v3 database configuration](../issues/open/1490-1978-decompose-database-configuration.md)
- [Secrecy crate documentation](https://docs.rs/secrecy/)
- [Torrust Tracker Deployer secrecy ADR](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/decisions/secrecy-crate-for-sensitive-data.md)
