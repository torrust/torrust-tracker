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

# Return `Result` Only for Concretely Fallible Public APIs

## Scope

This is a repository-level decision. It sets an API convention for every workspace package,
because every package will be published on crates.io and consumed independently of the tracker
(EPIC #1669, [independent package versioning](20260629000000_adopt_independent_package_versioning.md)).
The first application is the boundary between `swarm-coordination-registry` and `tracker-core`.

## Description

The swarm coordination `Registry` returned `Result<_, Error>` from ten methods with
`pub type Error = Infallible;`. No method could fail. Its main consumer,
`InMemoryTorrentRepository`, called `.expect(...)` on every result and documented a `# Panics`
section for a panic that could not happen. Reviewers read that as a reliability problem, and every
new method copied the pattern.

The `Result` had been kept on purpose, to absorb future failures without a breaking change. The
first implementation of issue #2435 followed that intent through: it replaced `Infallible` with a
crate-owned, empty `#[non_exhaustive]` error enum and propagated it to every delivery layer. That
work showed the cost of the approach:

- `SwarmRegistry` variants in `AnnounceError` and `ScrapeError`, a REST `StatsError` with a `500`
  path, error handling in the torrent cleanup job, and 37 new `.unwrap()` calls in tests;
- all of it for an error that cannot occur, so none of those paths could be tested;
- and readers once again inferred a failure mode that does not exist, which was the original
  problem.

The deciding question was simple: can counting the swarms (`Registry::len`) ever fail? Nobody could
name a case. The counting methods that returned `Result` had an equally weak case: a different
backend would affect `len()` as much as `count_peers()`, and a backend that different would most
likely be a new abstraction with its own error type anyway.

## Agreement

A public API returns `Result` when at least one of these holds:

1. **A failure can happen today**: I/O, parsing, validation, resource limits.
2. **The operation crosses an I/O boundary**: database, network, filesystem.
3. **It is a trait or port designed for swappable backends**, and a realistic backend can fail.
   The database driver traits are an example.

Otherwise, it returns a plain value. If a real failure appears later, the signature changes to
`Result` as a semver-signalled breaking change: a `0.x` minor bump or a major bump after 1.0. The
compiler then lists every caller that must handle the error.

Related rules:

- **Never use `Result<T, Infallible>` or an empty error enum to reserve room for future failures.**
  `Infallible` means "the error type for errors that can never happen". If an operation cannot
  fail, it returns `T`.
- **Never `expect` or document `# Panics` for failures that cannot occur.** If the code cannot fail,
  the signature should say so.
- **Mark existing public error enums `#[non_exhaustive]` before their first publish.** Enums with
  real variants gain variants as features grow (the reverted first attempt at #2435 had to add one
  to two enums). With
  `#[non_exhaustive]`, adding a variant is not a breaking change. This is the cheap, standard tool;
  RFC 2008 names error types as its most common use. Derive only traits every future variant can
  keep, because removing a derive is a breaking change.

### Why Breaking Changes Are Acceptable

The two options spread their costs differently:

- A speculative `Result` costs every consumer, from the first release and forever: propagation code
  and untestable error paths for an error that does not exist.
- A breaking change costs a one-time migration, only if the failure ever appears, and only for
  callers of the changed method. Cargo treats `0.1` to `0.2` (or `1.x` to `2.0`) as incompatible,
  so consumers adopt the change deliberately rather than being broken silently.

Before a crate reaches 1.0, its public API should be reviewed against this rule method by method.

### Alternatives Considered

- **Keep `Result<T, Infallible>`.** Rejected: it states that failure is impossible while keeping a
  `Result`, so consumers `expect` it, and replacing `Infallible` later changes the public API
  anyway.
- **An empty `#[non_exhaustive]` error enum with full propagation.** Implemented first in #2435,
  then reverted. Consumers cannot treat the type as uninhabited, so later variants are
  non-breaking, but every consumer pays for the plumbing up front for a failure that may never
  exist. See the Description.
- **A generic error type** such as `Box<dyn std::error::Error + Send + Sync>`. Rejected: it carries
  the same speculative cost and also hides which failures are possible.

### Consequences

Positive:

- Signatures state the truth: callers see `Result` only where something can fail.
- No error plumbing, `expect` calls, or `# Panics` sections exist for impossible failures.
- Error paths that do exist correspond to real failures and can be tested.

Negative:

- Introducing the first failure into an operation is a breaking change for its callers.
- The rule needs judgement. "Can this fail today, does it do I/O, or is it a swappable port?" is
  usually clear, but borderline cases should be decided in review.

## Affected Code

- [`packages/swarm-coordination-registry/src/swarm/registry.rs`](../../packages/swarm-coordination-registry/src/swarm/registry.rs)
- [`packages/tracker-core/src/torrent/repository/in_memory.rs`](../../packages/tracker-core/src/torrent/repository/in_memory.rs)

## Date

2026-10-05

## References

- Issue #2435 and its [specification](../issues/open/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md)
- Found during PR #2423 (issue #2406) review
- EPIC #1669 (package overhaul and publishing)
- [ADR: adopt independent package versioning](20260629000000_adopt_independent_package_versioning.md)
- [RFC 2008: `#[non_exhaustive]`](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)
- [`std::convert::Infallible`](https://doc.rust-lang.org/std/convert/enum.Infallible.html)
- [Cargo Book: SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
