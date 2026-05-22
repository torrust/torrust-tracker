---
name: handle-errors-in-code
description: Guide for error handling in this Rust project. Covers the four principles (clarity, context, actionability, explicit enums over anyhow), the thiserror pattern for structured errors, including what/where/when/why context, writing actionable help text, and avoiding vague errors. Also covers the located-error package for errors with source location. Use when writing error types, handling Results, adding error variants, or reviewing error messages. Triggers on "error handling", "error type", "Result", "thiserror", "anyhow", "error enum", "error message", "handle error", "add error variant", or "located-error".
metadata:
  author: torrust
  version: "1.0"
---

# Handling Errors in Code

## Core Principles

1. **Clarity** — Users immediately understand what went wrong
2. **Context** — Include what/where/when/why
3. **Actionability** — Tell users how to fix it
4. **Explicit enums over `anyhow`** — Prefer structured errors for pattern matching

## Prefer Explicit Enum Errors

```rust
// ✅ Correct: explicit, matchable, clear
#[derive(Debug, thiserror::Error)]
pub enum TrackerError {
    #[error("Torrent '{info_hash}' not found in whitelist")]
    TorrentNotWhitelisted { info_hash: InfoHash },

    #[error("Peer limit exceeded for torrent '{info_hash}': max {limit}")]
    PeerLimitExceeded { info_hash: InfoHash, limit: usize },
}

// ❌ Wrong: opaque, hard to match
return Err(anyhow::anyhow!("Something went wrong"));
return Err("Invalid input".into());
```

## Include Actionable Fix Instructions in Display

When the error is user-facing, add instructions:

```rust
#[error(
    "Configuration file not found at '{path}'.\n\
     Copy the default: cp share/default/config/tracker.toml {path}"
)]
ConfigNotFound { path: PathBuf },
```

## Context Requirements

Each error should answer:

- **What**: What operation was being performed?
- **Where**: Which component, file, or resource?
- **When**: Under what conditions?
- **Why**: What caused the failure?

```rust
// ✅ Good: full context
#[error("UDP socket bind failed for '{addr}': {source}. Is port {port} already in use?")]
SocketBindFailed { addr: SocketAddr, port: u16, source: std::io::Error },

// ❌ Bad: no context
return Err("bind failed".into());
```

## The `located-error` Package

For errors that benefit from source location tracking, use the `located-error` package:

```toml
[dependencies]
torrust-located-error = { version = "3.0.0-develop", path = "../located-error" }
```

```rust
use torrust_located_error::Located;

// Wraps any error with file and line information
let err = Located(my_error).into();
```

## Unwrap and Expect Policy

| Context                | `.unwrap()` | `.expect("msg")`                          | `?` / `Result` |
| ---------------------- | ----------- | ----------------------------------------- | -------------- |
| Production code        | Never       | Only when failure is logically impossible | Default        |
| Tests and doc examples | Acceptable  | Preferred when message adds clarity       | —              |

```rust
// ✅ Production: propagate errors with ?
fn load_config(path: &Path) -> Result<Config, ConfigError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| ConfigError::FileAccess { path: path.to_path_buf(), source: e })?;
    toml::from_str(&content)
        .map_err(|e| ConfigError::InvalidToml { path: path.to_path_buf(), source: e })
}

// ✅ Tests: unwrap() is fine
#[test]
fn it_should_parse_valid_config() {
    let config = Config::parse(VALID_TOML).unwrap();
    assert_eq!(config.http_api.bind_address, "127.0.0.1:1212");
}
```

## Quick Checklist

- [ ] Error type uses `thiserror::Error` derive
- [ ] Error message includes specific context (names, paths, addresses, values)
- [ ] Error message includes fix instructions where possible
- [ ] Prefer `enum` over `Box<dyn Error>` or `anyhow` in library code
- [ ] No vague messages like "invalid input" or "error occurred"
- [ ] No `.unwrap()` in production code (tests and doc examples are fine)
- [ ] Consider `located-error` for diagnostics-rich errors
