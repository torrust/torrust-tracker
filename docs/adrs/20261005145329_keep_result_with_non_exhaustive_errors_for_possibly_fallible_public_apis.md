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

# Keep `Result` With Non-Exhaustive Errors for Possibly Fallible Public APIs

## Scope

This is a repository-level decision. It sets an API convention for every workspace package,
because each package is published and can be consumed independently of the tracker. The first
application crosses the boundary between `swarm-coordination-registry` and `tracker-core`.

## Description

The swarm coordination `Registry` returned `Result<_, Error>` from ten methods with
`pub type Error = Infallible;`. No method could fail. Its main consumer,
`InMemoryTorrentRepository`, called `.expect(...)` on every result and documented a `# Panics`
section for a panic that could not happen. Reviewers read that as a reliability problem, and every
new method copied the pattern.

The `Result` was kept on purpose. Implementations change, and an operation that cannot fail today
(for example, an in-memory registry) may need to return an error tomorrow (for example, a
distributed or size-bounded registry). If the signature already returns `Result`, consumers handle
the error case from the start, and introducing a real failure is not a breaking change.

`Infallible` does not deliver that forward compatibility. The compiler knows it has no values, so
consumers can legitimately write code that breaks as soon as the alias becomes a real type:

- `let Ok(value) = registry.count_peers().await;` (irrefutable since Rust 1.82)
- `match error {}`
- `impl From<Infallible> for MyError`

Consumers that call `.unwrap()` or `.expect()` keep compiling and silently gain a panic.

## Agreement

1. A public package API keeps `Result` on an operation that may plausibly become fallible, even
   when the current implementation cannot fail. Return a plain value only when failure is
   impossible by construction (pure computation, accessors).
2. Such an API uses a crate-owned, uninhabited, non-exhaustive error enum instead of `Infallible`:

   ```rust
   /// No variant exists yet; see the ADR for why `Result` is kept.
   #[derive(Debug, Clone, PartialEq, Eq)]
   #[non_exhaustive]
   pub enum Error {}
   ```

   Inside the defining crate the enum is uninhabited, so internal code can bind `let Ok(v) = ...`.
   Other crates must treat a `#[non_exhaustive]` enum as possibly inhabited, so they are forced to
   handle `Err` (verified on 2026-10-05: `let Ok(v) = lib::count();` fails in a consumer crate with
   `E0005: refutable pattern ... Err(_) not covered`). Adding variants later is a non-breaking change.
3. Consumers propagate the error to the layer that can act on it (a protocol error response, an
   HTTP `500`, a logged job failure). They do not `expect` it, and they do not document panics that
   cannot occur.
4. The error type's rustdoc states that it currently has no variants and links to this ADR. A
   `compile_fail` doctest guards the forward-compatibility property.

### Alternatives Considered

- **Keep `Infallible` and destructure it irrefutably in consumers.** Rejected: it hides the result in
  one consumer and keeps the breaking-change trap described above.
- **Remove `Result` from operations that cannot fail today.** Rejected: introducing the first real
  failure becomes a breaking change for every consumer of an independently published package.
- **Invent a concrete error variant now.** Rejected: it adds a variant that is never constructed.

### Consequences

- Positive: consumers are ready for real failures, and the API no longer documents impossible panics.
- Negative: propagation plumbing (error variants, response mappings) exists before any failure can
  happen, and error paths originating in such an enum cannot be exercised at runtime until a variant
  exists.

## Affected Code

- [`packages/swarm-coordination-registry/src/swarm/registry.rs`](../../packages/swarm-coordination-registry/src/swarm/registry.rs)
- [`packages/tracker-core/src/torrent/repository/in_memory.rs`](../../packages/tracker-core/src/torrent/repository/in_memory.rs)

## Date

2026-10-05

## References

- Issue #2435 and its [specification](../issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md)
- Found during PR #2423 (issue #2406) review
- [The Rust Reference: the `non_exhaustive` attribute](https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute)
