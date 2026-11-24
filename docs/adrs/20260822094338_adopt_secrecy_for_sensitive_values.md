---
semantic-links:
  skill-links:
    - create-adr
    - handle-secrets
  related-artifacts:
    - .claude/skills/handle-secrets/SKILL.md
    - packages/configuration/src/lib.rs
    - docs/issues/closed/2079-adopt-secrecy-for-sensitive-configuration/ISSUE.md
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
- `SecretString` intentionally does not implement `Serialize`. Separate format
  from disclosure intent: generic serialization and diagnostic output redact for
  every format, while a narrowly named, authorized persistence boundary may
  expose a secret only for its immediate operation. Do not infer disclosure
  intent from TOML, JSON, or another format alone.
- Keep diagnostics, tracing, `Debug`, `Display`, errors, and test assertion
  messages redacted. `SecretString` formats as
  `SecretBox<str>([REDACTED])`; tests must assert that exact representation and
  confirm a unique test secret is absent.
- Call `ExposeSecret::expose_secret()` only at the last runtime boundary that
  consumes the real value, such as comparing an inbound API credential or
  constructing an outbound authentication request. Never expose a secret for
  logging, formatting, error text, or incidental test inspection.
- Preserve manual redaction for existing credential-bearing plain strings until
  they are migrated to an isolated secret field. In particular, legacy database
  URLs retain their masking until their passwords are separated.
- A secret is not only a configuration value (amendment of 2026-11-24, issue #2999). The peer
  authentication key, a REST API key and the admin token are secrets wherever they travel:
  - A peer key is not a `SecretString` (it is a domain type that must be printed to its owner and
    stored), so `Key` has a `Debug` that prints `Key([REDACTED])`, and a type that holds a key as a
    `String` redacts it in its own `Debug`. `Display` and `Serialize` stay only where the key is
    deliberately handed to its owner or persisted.
  - An error never keeps or prints a secret, and a message never repeats a malformed one. It states
    what is wrong with it: the length received, or the rule for its characters, or that it is not
    registered or has expired. Facts about the client's own input are not secrets; the secret is.
  - The BEP 3 clients put the peer key in the URL path (`/announce/<key>`), and a client may put
    the admin token in the query. A server logs a request URI only through the shared redaction
    (`torrust_tracker_axum_server::request_log`), which replaces what follows `/announce/`,
    `/scrape/` and `/key/` and the `token` query value with `***`, in the log event and in the span.
    The token travels in the `Authorization: Bearer` header; a client and the documentation use the
    header, never the query.
  - An error from a library that prints the value it rejected is made safe at the boundary: a
    `figment` configuration error for a secret option is redacted by option path, and a database
    server message for a statement that binds a secret is replaced by its error kind and code (a
    unique violation becomes its own variant). Wrapping a `reqwest` error removes its URL.
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
- The [secret-handling skill](../../.claude/skills/handle-secrets/SKILL.md)
  provides implementation and review guidance.
- [Issue #2079](../issues/closed/2079-adopt-secrecy-for-sensitive-configuration/ISSUE.md)
  applies this decision first to API tokens.

## Date

2026-08-22

## References

- Issue #2079: [Adopt `secrecy` for sensitive configuration](../issues/closed/2079-adopt-secrecy-for-sensitive-configuration/ISSUE.md)
- Follow-up issue #1490: [Decompose v3 database configuration](../issues/closed/1490-1978-decompose-database-configuration/ISSUE.md)
- [Secrecy crate documentation](https://docs.rs/secrecy/)
- [Torrust Tracker Deployer secrecy ADR](https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/decisions/secrecy-crate-for-sensitive-data.md)
