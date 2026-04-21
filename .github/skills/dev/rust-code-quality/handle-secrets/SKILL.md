---
name: handle-secrets
description: Guide for handling sensitive data (secrets) in this Rust project. NEVER use plain String for API tokens, passwords, or other credentials. Use the secrecy crate's Secret<T> wrapper to prevent accidental exposure through Debug output, logs, and error messages. Call .expose_secret() only when the actual value is needed. Use when working with credentials, API keys, tokens, passwords, or any sensitive configuration. Triggers on "secret", "API token", "password", "credential", "sensitive data", "secrecy", or "expose secret".
metadata:
  author: torrust
  version: "1.0"
---

# Handling Sensitive Data (Secrets)

## Core Rule

**NEVER use plain `String` for sensitive data.** Wrap secrets in `secrecy::Secret<String>`
(or similar) to prevent accidental exposure.

```rust
// ❌ WRONG: secret leaked in Debug output
pub struct ApiConfig {
    pub token: String,
}
println!("{config:?}"); // → ApiConfig { token: "secret_abc123" } — LEAKED!
```

```rust
// ✅ CORRECT: secret redacted in Debug
use secrecy::Secret;
pub struct ApiConfig {
    pub token: Secret<String>,
}
println!("{config:?}"); // → ApiConfig { token: Secret([REDACTED]) }
```

## Using the `secrecy` Crate

Add the dependency:

```toml
[dependencies]
secrecy = { workspace = true }
```

Basic usage:

```rust
use secrecy::{Secret, ExposeSecret};

// Wrap the secret
let token = Secret::new(String::from("my-api-token"));

// Access the value only when truly needed (e.g., making the actual API call)
let token_str: &str = token.expose_secret();
```

## What to Protect

Wrap with `Secret<T>` when the value is:

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

## Checklist

- [ ] No plain `String` fields for tokens, passwords, or private keys
- [ ] `Secret<String>` (or equivalent) used for all sensitive values
- [ ] `.expose_secret()` called only at the last moment
- [ ] No `.expose_secret()` in log statements or error messages
- [ ] No sensitive values in `Display` or `Debug` output
