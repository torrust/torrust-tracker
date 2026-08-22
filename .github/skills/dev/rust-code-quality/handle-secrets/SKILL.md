---
name: handle-secrets
description: Guide for handling sensitive data (secrets) in this Rust project. NEVER use plain String for API tokens, passwords, or other credentials. Use the current stable secrecy crate's direct secret types to prevent accidental exposure through Debug output, logs, and error messages. Call .expose_secret() only when the actual value is needed. Use when working with credentials, API keys, tokens, passwords, or any sensitive configuration. Triggers on "secret", "API token", "password", "credential", "sensitive data", "secrecy", or "expose secret".
metadata:
  author: torrust
  version: "1.2"
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

## Serialization and Test Expectations

- Keep existing configuration-file syntax for secret values unless a deliberate schema change
  is required. `secrecy` with its `serde` feature supports deserializing a TOML string directly
  into `SecretString`.
- `SecretString` deliberately does not serialize automatically. At a configuration persistence
  boundary, use a narrow `serialize_with` function that calls `.expose_secret()` only to emit
  the established TOML schema. Keep diagnostic JSON behind a separate redaction boundary.
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
- [ ] No `.expose_secret()` in log statements or error messages
- [ ] No sensitive values in `Display` or `Debug` output
- [ ] Serialized configuration tests preserve the existing secret syntax
- [ ] Redaction tests assert `SecretBox<str>([REDACTED])` and never print test secret values
