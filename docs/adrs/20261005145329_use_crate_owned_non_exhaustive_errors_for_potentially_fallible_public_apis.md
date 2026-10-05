---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - .github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md
    - "issue #2435"
    - docs/issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md
    - packages/swarm-coordination-registry/src/swarm/registry.rs
    - packages/tracker-core/src/torrent/repository/in_memory.rs
---

<!-- skill-link: create-adr -->

# Use Crate-Owned Non-Exhaustive Errors for Potentially Fallible Public APIs

## Scope

This is a repository-level decision. It sets an API convention for every workspace package,
because each package is published and can be consumed independently of the tracker. The first
application crosses the boundary between `swarm-coordination-registry` and `tracker-core`.

The convention applies mainly to public APIs of independently published crates and to other
boundaries where consumers evolve independently of the implementation. Internal functions whose
callers and implementation change together do not need this forward-compatibility treatment
unless there is another architectural reason to return `Result`.

## Description

The swarm coordination `Registry` returned `Result<_, Error>` from ten methods with
`pub type Error = Infallible;`. No method could fail. Its main consumer,
`InMemoryTorrentRepository`, called `.expect(...)` on every result and documented a `# Panics`
section for a panic that could not happen. Reviewers read that as a reliability problem, and every
new method copied the pattern.

The `Result` was kept on purpose. Implementations change. An operation that cannot fail today
(for example, updating an in-memory registry) may need to return an error tomorrow (for example,
in a distributed, persistent, or size-bounded registry).

`Infallible` was the wrong error type for that intent, for two reasons:

- **It states the wrong contract.** The standard library defines `Infallible` as "the error type
  for errors that can never happen". The intended contract here is different: the current
  implementation cannot fail, but the operation may legitimately gain failure modes as the
  implementation evolves.
- **The error type is part of the public API.** Changing `Result<T, Infallible>` to
  `Result<T, RealError>` changes a public signature. Because the compiler knows `Infallible` has no
  values, consumers can also legitimately write code that breaks when that happens:
  - `let Ok(value) = registry.count_peers().await;` (irrefutable since Rust 1.82)
  - `match error {}`
  - `impl From<Infallible> for MyError`

  Consumers that call `.unwrap()` or `.expect()` keep compiling and silently gain a panic.

## Agreement

**Public API stability is based on the semantics of the abstraction, not on the limitations of its
current implementation.** A public API exposes the failure modes that belong to the abstraction,
not merely the failure modes of its current implementation.

1. **When to return `Result`.** Return `Result<T, Error>` when the operation is conceptually
   fallible, even if the current implementation cannot fail. Return a plain value when failure is
   impossible by construction and is not part of the operation's meaning. For example,
   `fn len(&self) -> usize` stays a plain value, while
   `async fn handle_announcement(...) -> Result<(), Error>` may be fallible even though today's
   in-memory implementation is not. This is not a rule to return `Result` from every public method
   "just in case".
2. **Introduce the crate-owned error type together with `Result`.** The error type is part of the
   initial API contract, even when it has no variants yet:

   ```rust
   #[derive(Debug, Clone)]
   #[non_exhaustive]
   pub enum Error {}
   ```

   It implements `Display` and `std::error::Error` (and is `Send + Sync`). Derive only the traits
   that every future variant can keep: removing a derive later is a breaking change.
3. **`#[non_exhaustive]` keeps today's emptiness out of the contract.** Inside the defining crate
   the enum is uninhabited, so internal code can bind `let Ok(v) = ...`. External consumers cannot
   rely on the type being uninhabited. In particular, they cannot use exhaustive pattern matching
   to prove that `Err` is impossible (verified on 2026-10-05: `let Ok(v) = lib::count();` fails in
   a consumer crate with `E0005: refutable pattern ... Err(_) not covered`). Their code must
   therefore stay valid if variants are added later, and adding a variant is not a breaking change:

   ```rust
   #[derive(Debug, Clone)]
   #[non_exhaustive]
   pub enum Error {
       CapacityExceeded { limit: usize },
       // ...
   }
   ```

   Consumers can still choose to `.unwrap()` the result. The type does not prevent that; this
   convention does (point 4).
4. **Consumers propagate the error** to the layer that can act on it (a protocol error response, an
   HTTP `500`, a logged job failure). They do not `expect` it, and they do not document panics that
   cannot occur.
5. **Document and guard the property.** The error type's rustdoc states that it has no variants yet
   and links to this ADR. A `compile_fail` doctest shows that a downstream crate cannot treat the
   error as uninhabited.

### Relationship to Established Rust Practice

The exact combination of an initially empty `#[non_exhaustive]` error enum and a currently
infallible implementation is a repository convention, not a Rust language requirement or a widely
documented idiom. It combines established Rust API-design practices:

- Public fallible APIs return `Result<T, E>`.
- Public library APIs expose a meaningful error type specific to the crate
  ([C-GOOD-ERR](https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err)).
- `#[non_exhaustive]` lets public enums evolve without breaking downstream crates, and RFC 2008
  names error types as its most common use
  ([RFC 2008](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)).

This repository applies those practices proactively by introducing the crate-owned error type
before the first concrete failure mode exists.

### Alternatives Considered

- **Keep `Infallible`.** Rejected: `Result<T, Infallible>` makes "failure is impossible" part of
  the type contract, and replacing `Infallible` with a real error type later changes the public API.
- **Remove `Result` from operations that cannot fail today.** Rejected: introducing the first real
  failure changes `T` into `Result<T, Error>`, which breaks every consumer of an independently
  published package.
- **Invent a concrete error variant now.** Rejected: a variant that cannot occur gives the error type
  meaning that does not match any real failure mode. The empty enum describes the current
  implementation accurately, and `#[non_exhaustive]` keeps that emptiness from becoming a
  compatibility constraint.
- **Use a generic error type** such as `Box<dyn std::error::Error + Send + Sync>`. Rejected: the
  public API should communicate the crate's own error contract and let future variants be
  represented and matched explicitly.

### Consequences

Positive:

- A public `T` return type never has to change to `Result<T, Error>` later.
- `Result<T, Infallible>` never has to change to `Result<T, Error>` later.
- The crate's error type is part of the API from the beginning.
- Concrete variants can be added without today's absence of variants being part of the contract.
- Downstream error propagation exists before concrete failure modes do.
- The API no longer documents impossible `expect`/`unwrap` panics.

Negative:

- API and propagation plumbing (error variants, response mappings) exist before any failure can
  happen.
- Consumers propagate an error that cannot currently occur.
- Error paths originating in such an enum cannot be exercised at runtime until a variant exists.
- Developers must distinguish operations that are conceptually infallible from operations whose
  current implementation merely happens to be infallible.
- An empty error enum may look unusual to developers unfamiliar with this API-evolution strategy.

## Affected Code

- [`packages/swarm-coordination-registry/src/swarm/registry.rs`](../../packages/swarm-coordination-registry/src/swarm/registry.rs)
- [`packages/tracker-core/src/torrent/repository/in_memory.rs`](../../packages/tracker-core/src/torrent/repository/in_memory.rs)

## Date

2026-10-05

## References

- Issue #2435 and its [specification](../issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md)
- Found during PR #2423 (issue #2406) review
- [RFC 2008: `#[non_exhaustive]`](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)
- [Rust API Guidelines: error types are meaningful and well-behaved (C-GOOD-ERR)](https://rust-lang.github.io/api-guidelines/interoperability.html#c-good-err)
- [The Rust Reference: the `non_exhaustive` attribute](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute)
- [`std::convert::Infallible`](https://doc.rust-lang.org/std/convert/enum.Infallible.html)
