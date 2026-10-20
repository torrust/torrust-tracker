---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .github/skills/dev/planning/create-adr/SKILL.md
    - .github/skills/dev/rust-code-quality/handle-errors-in-code/SKILL.md
    - "issue #2435"
    - docs/issues/closed/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md
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
3. **It is a trait or port with an existing or planned backend that can fail.** The database
   driver traits are an example. A "port" is a trait that defines a boundary other
   implementations can back, such as the REST application port traits in the
   [contract-first REST API architecture](20260623200526_adopt_contract-first_architecture_for_rest_api.md).
   Being swappable is not enough: some backend that is real or planned must be able to fail. For
   example, the auth-key and whitelist ports return `Result` because their implementation does
   database I/O (conditions 1 and 2). The stats and torrent query ports return plain values,
   because no backend that can fail exists or is planned; they change to `Result` when one is.

Otherwise, it returns a plain value. If a real failure appears later, the signature changes to
`Result` as a semver-signalled breaking change: a `0.x` minor bump or a major bump after 1.0. The
compiler then lists every caller that must handle the error.

Related rules:

- **Never use `Result<T, Infallible>` or an empty error enum to reserve room for future failures.**
  `Infallible` means "the error type for errors that can never happen". If an operation cannot
  fail, it returns `T`.
- **Do not `expect` a workspace API's `Result` whose error cannot occur.** Change that API to
  return a plain value instead, so the signature states the truth and no false `# Panics` section
  is needed. A call-site `expect` on an API that can fail in general but not for this input, often
  one we do not own (for example serializing plain data with `serde_json`), stays under the
  `handle-errors-in-code` skill's Unwrap and Expect Policy, with the `# Panics` section that
  `clippy::missing_panics_doc` requires.
- **Mark existing public error enums `#[non_exhaustive]` before their first publish.** Enums with
  real variants gain variants as features grow (the reverted first attempt at #2435 had to add one
  to two enums). With
  `#[non_exhaustive]`, adding a variant is not a breaking change. This is the cheap, standard tool;
  RFC 2008 names error types as its most common use. Derive only traits every future variant can
  keep, because removing a derive is a breaking change. The 2026-10-19 amendment below
  makes an exception for enums whose variants select a recovery action.

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
- The rule needs judgement. "Can this fail today, does it do I/O, or does a real or planned backend
  fail?" is
  usually clear, but borderline cases should be decided in review.

### Amendment (2026-10-19): Enums Whose Variants Select a Recovery Action Stay Exhaustive

Applying the rule in issue #2454 showed that `#[non_exhaustive]` is not right for every public
error enum. Marking an enum makes every consumer outside its crate add a wildcard arm. For an
enum that consumers only report or convert (log it, show its message, answer with a generic `500`,
map it to a label), the wildcard is harmless, and a new variant is not a breaking change. For an
enum whose variants select different recovery actions (stop or continue, retry or give up, reply
or drop), the wildcard silently picks an action for a variant that nobody designed for, and no
compiler error tells the consumer to decide. There, the breaking change is the safer signal.

So the rule above applies in this form:

- Mark a public error enum `#[non_exhaustive]` when consumers can handle an unknown variant
  generically.
- Keep it exhaustive when its variants select different recovery actions, and say so in a doc
  comment on the enum, so that a later reader does not add the attribute. Adding a variant to it is
  then a deliberate breaking change that forces every consumer to decide.

This follows the standard library and tokio: `std::sync::mpsc::TryRecvError` and tokio's
`broadcast::error::RecvError` (closed or lagged) are exhaustive control-flow enums, while
`std::io::ErrorKind` is `#[non_exhaustive]`. The maintainer decided this on 2026-10-19 during
issue #2454, which applies the rule to every workspace crate: `torrust_tracker_events::receiver::RecvError`
(closed stops a listener, lagged continues) and `torrust_tracker_udp_protocol::request::RequestParseError`
(sendable replies to the client, unsendable drops) are its two exhaustive enums.

## Affected Code

- [`packages/swarm-coordination-registry/src/swarm/registry.rs`](../../packages/swarm-coordination-registry/src/swarm/registry.rs)
- [`packages/tracker-core/src/torrent/repository/in_memory.rs`](../../packages/tracker-core/src/torrent/repository/in_memory.rs)

## Date

2026-10-05. Amended 2026-10-19 (enums whose variants select a recovery action stay exhaustive).

## References

- Issue #2454 (applies the `#[non_exhaustive]` rule to every workspace crate; source of the amendment)
- Issue #2435 and its [specification](../issues/closed/2435-remove-misleading-panics-in-in-memory-torrent-repository/ISSUE.md)
- Found during PR #2423 (issue #2406) review
- EPIC #1669 (package overhaul and publishing)
- [ADR: adopt independent package versioning](20260629000000_adopt_independent_package_versioning.md)
- [RFC 2008: `#[non_exhaustive]`](https://rust-lang.github.io/rfcs/2008-non-exhaustive.html)
- [`std::convert::Infallible`](https://doc.rust-lang.org/std/convert/enum.Infallible.html)
- [Cargo Book: SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
