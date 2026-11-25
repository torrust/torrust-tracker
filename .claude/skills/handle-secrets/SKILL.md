---
name: handle-secrets
description: Keep API tokens, passwords and other credentials in secrecy types, never a plain String; expose_secret only where needed. Use when handling secrets or sensitive configuration.
metadata:
  author: torrust
  version: "1.3"
---

# Handling Sensitive Data (Secrets)

## Core Rule

**NEVER use plain `String` for sensitive data.** Use the current stable
`secrecy::SecretString` type for string secrets to prevent accidental exposure.

```rust
// ❌ WRONG: secret leaked in Debug output
pub struct ApiConfig {
    pub token: String,
}
println!("{config:?}"); // → ApiConfig { token: "secret_abc123" } — LEAKED!
```

```rust
// ✅ CORRECT: secret redacted in Debug
use secrecy::SecretString;
pub struct ApiConfig {
  pub token: SecretString,
}
println!("{config:?}"); // → ApiConfig { token: SecretBox<str>([REDACTED]) }
```

## Using the `secrecy` Crate

Add the dependency:

```toml
[dependencies]
secrecy = { version = "0.10", features = [ "serde" ] }
```

Enable `serde` only when a secret must be read from or written to a serialized
configuration format. This is an intentional opt-in: configuration-file syntax remains
unchanged while the Rust type becomes `SecretString`.

Basic usage:

```rust
use secrecy::{ExposeSecret, SecretString};

// Wrap the secret
let token = SecretString::from("my-api-token");

// Access the value only when truly needed (e.g., making the actual API call)
let token_str: &str = token.expose_secret();
```

## What to Protect

Wrap with `SecretString` (or another appropriate direct `secrecy` type) when the value is:

- API tokens (REST API admin token, external service tokens)
- Passwords (database credentials, service accounts)
- Private keys or certificates

## Secrets That Are Not Configuration

The peer authentication key, a REST API key and the admin token are secrets wherever they travel
(see the amendment in
[the secrecy ADR](../../../docs/adrs/20260822094338_adopt_secrecy_for_sensitive_values.md)).

- **The peer key** is `Key`, not a `SecretString`: it must be printed to its owner and stored.
  `Key`'s `Debug` prints `Key([REDACTED])`; a type that holds a key as a `String` gets a redacting
  `Debug`. Print or serialize a key only where it is deliberately handed to its owner or persisted.
- **Errors** never keep or print a secret, even a malformed one. State the length received or the
  rule for its characters (`expected 32 characters, got 14`), or that it is not registered or has
  expired. Do not add a `key` field to an error variant.
- **Secrets in URLs**: the peer key is a path segment (`/announce/<key>`, `/scrape/<key>`,
  `/api/v1/key/<key>`) and a token may be a `?token=` query value. Log a request URI only through
  `torrust_tracker_axum_server::request_log::redact_uri` (and `RedactedMakeSpan` for the span), which
  replaces them with `***`. Never log `request.uri()` directly.
- **The admin token goes in the `Authorization: Bearer` header**, in clients, scripts and
  documentation. `?token=` is still accepted by the server but it ends up in proxy and browser
  logs. Do not write it in an example. The header only keeps the token out of logged URLs; it
  does not protect it in transit, so do not call it safe without HTTPS.
- **Library errors that print the value they rejected**: a `reqwest` error holds the URL, so wrap
  it with `without_url()`; a `figment` error echoes a value of the wrong type, so a new secret
  option must be added to `SECRET_OPTIONS` in `packages/configuration/src/secret_options.rs`; a
  database server message can hold a written value (MySQL prints a duplicate key), so convert the
  error of a statement that binds a secret with `databases::error::Error::from_statement_that_binds_a_secret`.
- Tests assert the absence of a unique test secret and never print it
  (`tracker-core`'s `assert_holds_no_secret`, `test-helpers`' `capture_logs_while`).

## Rules for `.expose_secret()`

- Call **as late as possible** — only at the point where the value is required
- **Never** call in `log!`, `debug!`, `info!`, `warn!`, `error!` macros
- **Never** call in `Display` or `Debug` implementations
- **Never** include in error messages that may be logged or shown to users

```rust
// ✅ Correct: called at last moment for HTTP header
let response = client
    .get(url)
    .header("Authorization", format!("Bearer {}", token.expose_secret()))
    .send()
    .await?;

// ❌ Wrong: exposed in log
tracing::debug!("Using token: {}", token.expose_secret());
```

## Comparing Secrets

**Never compare a secret with `==`, `!=`, `eq()`, or `Iterator::any` over `==`.** Those may
exit at the first differing byte and leak, through timing, how many leading bytes were
correct. Use `subtle::ConstantTimeEq` on the byte slices and, when checking against several
candidates, evaluate **all** of them and OR the `Choice` results instead of stopping at the
first match:

```rust
use subtle::{Choice, ConstantTimeEq};

let provided = provided.as_bytes();

let matched = configured.iter().fold(Choice::from(0), |acc, secret| {
    acc | secret.expose_secret().as_bytes().ct_eq(provided)
});

bool::from(matched)
```

`subtle` is already a workspace dependency (transitively via `sqlx`); adding it directly to a
crate pulls in no new code. Length still short-circuits in `ct_eq`; if token length must be
hidden, that is a separate design decision (fixed-length tokens or a keyed hash), not a reason
to fall back to `==`.

See `docs/security/analysis/reports/` for disclosed, handled-report examples.

## Serialization and Test Expectations

- Keep existing configuration-file syntax for secret values unless a deliberate schema change
  is required. `secrecy` with its `serde` feature supports deserializing a TOML string directly
  into `SecretString`.
- `SecretString` deliberately does not serialize automatically. Separate serialization format
  from disclosure intent: generic serialization and diagnostic output redact for every format;
  an explicitly named, authorized persistence boundary may call `.expose_secret()` only to emit
  a runnable configuration artifact. Do not infer disclosure intent from TOML, JSON, or another
  format alone.
- Test redaction without exposing the secret. For `SecretString`, assert that `Debug` output
  contains the exact literal `SecretBox<str>([REDACTED])` and does not contain the unique test
  value.
- Do not write assertions, snapshots, test failures, or diagnostics that call
  `.expose_secret()` merely to inspect a value. Restrict exposure tests to the runtime boundary
  that genuinely consumes the secret.
- Do not remove unrelated legacy redaction solely because a new secret field is type-protected;
  credential-bearing strings continue to require their existing masking until migrated.

## Checklist

- [ ] No plain `String` fields for tokens, passwords, or private keys
- [ ] `SecretString` (or an equivalent direct `secrecy` type) used for string secrets
- [ ] `.expose_secret()` called only at the last moment
- [ ] Secrets compared with `subtle::ConstantTimeEq`, never `==`; multiple candidates all evaluated
- [ ] No `.expose_secret()` in log statements or error messages
- [ ] No sensitive values in `Display` or `Debug` output
- [ ] No peer key or token in an error variant, an error message, a log line or a logged URI
- [ ] A new secret configuration option is listed in `SECRET_OPTIONS`
- [ ] Serialized configuration tests preserve the existing secret syntax
- [ ] Redaction tests assert `SecretBox<str>([REDACTED])` and never print test secret values
