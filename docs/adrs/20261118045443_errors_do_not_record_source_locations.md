---
semantic-links:
  skill-links:
    - create-adr
    - handle-errors-in-code
  related-artifacts:
    - .claude/skills/create-adr/SKILL.md
    - .claude/skills/handle-errors-in-code/SKILL.md
    - docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md
    - docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md
    - docs/issues/closed/2804-location-caller-track-caller-audit/options.md
---

<!-- skill-link: create-adr -->

# Errors do not record source locations

## Scope

Repository-level. The rule binds every package of the workspace: it decides the shape of public
error types in `primitives`, `configuration`, `tracker-core`, `http-protocol`, `udp-server` and
`axum-server`, the text that the HTTP tracker, the UDP tracker and the REST API send to clients,
and a dependency policy (`torrust-located-error`). The placement ADR
(`20260830124000_place_adrs_by_decision_scope.md`) requires root placement for workspace
conventions, dependency policy and protocol behaviour.

## Description

Many error types in the workspace have a `location: &'static Location<'static>` field filled with
`Location::caller()`, and several wrap their source in `LocatedError` from the standalone crate
`torrust-located-error`. The intent was to tell a developer where an error was created.

The audit of issue #2804 (`docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md`
and `options.md`) found that the recorded location is never the caller's:

- None of the 38 production `Location::caller()` sites (37 since #2850) is in a `#[track_caller]`
  function. `#[track_caller]` cannot fix any of them either, because they are reached through
  `str::parse`, `try_into`, closures, `async fn` or `async` blocks, where the caller chain ends at a
  standard-library or workspace line that never changes (`options.md`, fact 6). The value is a fixed
  library line.
- 23 `Display` formats (22 since #2850) print that location, so the text that reaches HTTP, UDP and REST clients,
  event payloads and logs contains a source path of the tracker (`options.md`, fact 1).
- `LocatedError` prints its own location in its `Display` (`torrust-located-error` 3.0.0,
  `lib.rs:58`), whatever the wrapped error does.
- The five `#[track_caller]` `From` conversions into `LocatedError` do record a location that is
  correct for their call chain (`inventory.md`, class C), but it is still a path inside the
  implementation.

Issue #384 (2023) reported that clients receive these paths. The answer then was that the
information helps administrators and developers more than it risks. The audit removes that
argument: the location is wrong, so it helps nobody. The maintainer added three objections that
hold even for a correct location:

1. The text lets an attacker infer the running version of the tracker.
2. It couples the user-facing interface to the implementation: moving a file changes a response.
3. It makes messages unstable for consumers that match or store them.

## Agreement

Errors in this workspace must not record or print source locations. The points below are rules:
new and changed code follows them now, and the code that predates this ADR is brought in line by
the sub-issues of EPIC #2912 (see Consequences).

1. **No location field.** Error types must not have a `location` field or call
   `Location::caller()`.
2. **No `torrust-located-error`.** The workspace must not depend on `torrust-located-error`; the
   EPIC removes the existing dependency. A source error is kept as a plain `#[source]` field (or
   `#[from]`), not wrapped in `LocatedError`.
3. **`Display` states what failed and the offending input.** It names the operation and the value
   that was rejected (the parameter, the info-hash), and holds no source path. A message that would
   lose its only context when the location goes gets new wording, or a new field for the input. A
   secret (an authentication key, a token, a password) is never the offending input in the message:
   the message names the parameter and the expected format instead, and the error does not keep the
   secret, as the `handle-secrets` skill requires.
4. **Client text is the error's own message.** Text sent to an HTTP, UDP or REST client is the
   `Display` of the workspace error. A source error from a crate the workspace does not control is
   not formatted into client text, because its text can change, and can contain a location, in a
   release the workspace does not review (for example `torrust_info_hash::ConversionError` prints its
   own location today).
5. **Developer diagnostics go to `tracing`.** When a developer needs to know where something failed,
   the code emits a `tracing` event at that site. The event records file and line on the server only
   and is never sent to a client.
6. **`#[track_caller]` stays allowed in tests.** Test helpers and assertion helpers may use
   `#[track_caller]` so that a failing assertion points at the test line.

```rust
// ✅ The message names what failed and the input. No location.
#[derive(Debug, thiserror::Error)]
pub enum ParseNumWantError {
    #[error("invalid `numwant` value '{value}': expected a non-negative integer")]
    NotANumber { value: String },
}

// ✅ A secret is not kept: the message names the parameter and the expected format.
#[derive(Debug, thiserror::Error)]
pub enum ParseKeyError {
    #[error("invalid authentication key: expected 32 alphanumeric characters, got {length}")]
    InvalidFormat { length: usize },

    #[error("cannot read the key file '{path}'")]
    CannotRead { path: PathBuf, #[source] source: std::io::Error },
}
```

```rust
// ❌ A location that reaches the client and names a fixed library line.
#[derive(Debug, thiserror::Error)]
pub enum ParseKeyError {
    #[error("invalid key format, {location}")]
    InvalidFormat { location: &'static Location<'static> },
}
```

### Alternatives Considered

The four options compared in `options.md`:

1. **`#[track_caller]` chains.** Rejected: it cannot reach the caller at any of the 38 production
   sites (37 since #2850; trait impls reached through `parse` and `try_into`, closures, `async`), and where it works
   it still puts a workspace path in user text.
2. **Capture the location at the call site** (a constructor or macro). Rejected: the same callers are
   trait impls, closures and `async` code, so the captured line is still fixed, and each type needs a
   new constructor.
3. **Keep the field and stop printing it in `Display`.** This was the audit's recommendation, as it
   closes the user-visible exposure with the least churn. Rejected by the maintainer: the field still
   holds a fixed library line, so `Debug` output and logs keep a misleading value, and the
   `LocatedError` wrapper keeps printing its own location.
4. **Remove the field and the wrapper (chosen).** It removes the wrong value everywhere. The semver
   cost is low: the affected crates other than `primitives` and `configuration` have not been
   published yet, and those two already owe a major release (`options.md`, "Semver per published
   crate").

### Consequences

- Positive: client messages no longer reveal source paths or the tracker version, and they stay the
  same when code moves.
- Positive: `Debug` output and logs no longer show a location that looks precise and is wrong.
- Positive: one dependency less (`torrust-located-error`).
- Negative: the developer hint is gone. A developer who needs a location adds a `tracing` event at the
  site, or reads a backtrace.
- Negative: the 24 production variants that carry the field (23 since #2850) change shape. This is a
  breaking change for `primitives` and `configuration`, inside the major releases they already owe, and free for the crates not yet
  published if the change lands before their first publish.
- Messages that relied on the location for their only context need new wording, and in a few cases a
  field that carries the offending input.
- Until the sub-issues of EPIC #2912 are merged, the code still has location fields and
  `LocatedError`. New code follows this ADR; existing uses are removed by SI-2 to SI-6, and SI-6 adds
  a check that fails on `Location::caller()`, `#[track_caller]` or `torrust_located_error` in non-test
  code.

## Date

2026-11-18

## References

- EPIC [#2912](https://github.com/torrust/torrust-tracker/issues/2912), "Remove source locations
  from errors", and its specification
  `docs/issues/open/2912-remove-source-locations-from-errors/EPIC.md` (section "Maintainer
  Decisions").
- Issue [#2913](https://github.com/torrust/torrust-tracker/issues/2913) (SI-1), which records this
  ADR.
- Audit: issue [#2804](https://github.com/torrust/torrust-tracker/issues/2804),
  `docs/issues/closed/2804-location-caller-track-caller-audit/inventory.md` and `options.md`.
- Issue [#384](https://github.com/torrust/torrust-tracker/issues/384), the 2023 report of locations
  in client responses.
- `docs/adrs/20261005145329_return_result_only_for_concretely_fallible_public_apis.md`, the
  `#[non_exhaustive]` policy for public error enums.
- [torrust/torrust-located-error](https://github.com/torrust/torrust-located-error).
