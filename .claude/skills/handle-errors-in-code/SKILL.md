---
name: handle-errors-in-code
description: "Write Rust error types: thiserror enums over anyhow, with context and actionable messages, and no source locations. Use when adding error types or handling a Result."
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
- **Where**: Which component, file, or resource? (A runtime resource such as a config file or an address, not a source location.)
- **When**: Under what conditions?
- **Why**: What caused the failure?

```rust
// ✅ Good: full context
#[error("UDP socket bind failed for '{addr}': {source}. Is port {port} already in use?")]
SocketBindFailed { addr: SocketAddr, port: u16, source: std::io::Error },

// ❌ Bad: no context
return Err("bind failed".into());
```

## No Source Locations in Errors

Errors must not record or print source locations. See
[ADR 20261118045443](../../../docs/adrs/20261118045443_errors_do_not_record_source_locations.md).

- No `location` field and no `Location::caller()` in error types.
- No `torrust-located-error`: keep a source error as a plain `#[source]` (or `#[from]`) field.
- `Display` states what failed and the offending input, never a source path.
- Text sent to a client is the error's own message; do not format a source error from a crate the
  workspace does not control into client text.
- For developer diagnostics, emit a `tracing` event at the site; it records file and line on the
  server only.
- `#[track_caller]` is allowed only in test helpers and assertion helpers.

```rust
// ✅ Correct: names what failed and the input
#[derive(Debug, thiserror::Error)]
pub enum KeyError {
    #[error("invalid authentication key '{key}': expected 32 alphanumeric characters")]
    InvalidKeyFormat { key: String },
}
```

```rust
// ❌ Wrong: a location that reaches clients and logs
#[derive(Debug, thiserror::Error)]
pub enum KeyError {
    #[error("invalid key format, {location}")]
    InvalidKeyFormat { location: &'static Location<'static> },
}
```

Existing `location` fields and `LocatedError` uses are being removed by EPIC #2912; do not add new
ones.

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

## When Public APIs Return `Result`

Return `Result` only when an operation can fail today, crosses an I/O boundary, or is a trait or port
with an existing or planned backend that can fail. Otherwise return a plain value; a real failure later is a
semver-signalled breaking change. Never use `Infallible` or an empty error enum as a placeholder, and mark
existing public error enums `#[non_exhaustive]` before first publish, except an enum whose variants select
different recovery actions (stop or continue, reply or drop): keep it exhaustive and say why in a doc comment, so
that a new variant is a deliberate breaking change. See [ADR 20261005145329](../../../docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md).

## Quick Checklist

- [ ] Error type uses `thiserror::Error` derive
- [ ] Error message includes specific context (names, paths, addresses, values)
- [ ] Error message includes fix instructions where possible
- [ ] Prefer `enum` over `Box<dyn Error>` or `anyhow` in library code
- [ ] No vague messages like "invalid input" or "error occurred"
- [ ] No `.unwrap()` in production code (tests and doc examples are fine)
- [ ] No `location` field, no `Location::caller()` and no `torrust-located-error` (see the ADR above)
