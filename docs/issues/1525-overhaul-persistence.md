# Issue #1525 Implementation Plan (Overhaul Persistence)

## Goal

Redesign the persistence layer progressively so PostgreSQL support can be added safely, with each step independently reviewable and mergeable.

## Scope

- Target issue: https://github.com/torrust/torrust-tracker/issues/1525
- Reference PR: https://github.com/torrust/torrust-tracker/pull/1695
- Review record PR: https://github.com/torrust/torrust-tracker/pull/1700
- Key review comment: https://github.com/torrust/torrust-tracker/pull/1695#pullrequestreview-4127741472
- Reference branch for existing implementation work: `review/pr-1695`

## Context

This EPIC was created in May 2025, almost a year before the current implementation effort. The problems it describes were identified early, and the opening of PR #1695 (PostgreSQL support) is what turned the plan into an active priority — but PostgreSQL is not the only driver.

### Original motivations (from issue #1525)

- **No migrations**: The tracker has no schema migration mechanism. As more tables are planned (e.g. extended metrics from issue #1437), the absence of migrations becomes increasingly risky.
- **Wrong crate for the job**: `r2d2` is a synchronous connection-pool library. It is not clear it is still the best fit; `sqlx` is already used in the Index project and supports async natively. The issue references SeaORM as an alternative worth researching.
- **Adding a new driver is too hard**: The `Database` trait is too wide. Adding PostgreSQL support (issue #462) was confirmed to be tricky with the current `r2d2`-based abstraction — the trait must be split before new backends can be added cleanly.

### Immediate trigger

PR #1695 demonstrates that the PostgreSQL work is feasible, but bundled the entire redesign into one large diff. This plan re-delivers that work incrementally so every step is independently reviewable and mergeable.

### Why now

The PostgreSQL PR created momentum and a concrete reference implementation. Leaving the redesign for later would mean adding more complexity on top of a layer that is already known to be the wrong shape.

## Delivery Strategy

Apply the redesign in small steps that can be merged independently into `develop`.

### Phase 1: Make the change easy

1. Add a DB compatibility matrix across supported database versions.
2. Add an end-to-end test with a real BitTorrent client.
3. Add before/after persistence benchmarking so later changes can be compared against a concrete baseline.
4. Split the persistence traits to reduce coupling.
5. Migrate existing SQL backends to the new async `sqlx` substrate without introducing PostgreSQL yet.
6. Introduce schema migrations and align schema ownership with migrations.
7. Align Rust types with the actual SQL storage model. This step may require schema changes (e.g. widening 32-bit counter columns to 64-bit), so it belongs after migrations are in place.

### Phase 2: Make the easy change

1. Add PostgreSQL as a first-class backend on top of the refactored persistence layer.

## Working Rules

- Treat `review/pr-1695` as a read-only reference branch.
- Do not try to preserve the original PR commit structure.
- Port useful code selectively from the reference branch into clean subissue branches.
- New QA and tooling code should be written in Rust unless there is a strong reason not to.
- Every subissue should produce a PR that is reviewable on its own and safe to merge before PostgreSQL support is complete.

## Reference Implementation

PR #1695 was authored on the fork `josecelano/torrust-tracker`, branch `pr-1684-review`.
The reference implementation lives at:

```text
https://github.com/josecelano/torrust-tracker/tree/pr-1684-review
```

This branch should be treated as a **read-only reference** — a prototype that demonstrates
feasibility. Implementation work is done in dedicated subissue branches cut from `develop`.

### Checking out the reference branch locally

To inspect the reference implementation without affecting your current checkout, clone the
fork into a separate directory:

```bash
git clone --branch pr-1684-review \
    https://github.com/josecelano/torrust-tracker.git \
    /path/to/torrust-tracker-pr-1700
```

Replace `/path/to/torrust-tracker-pr-1700` with any directory outside your main checkout.
You can then browse or search it while working in the main repository.

## Proposed Subissues

### 1) Add DB compatibility matrix

- Spec file: `docs/issues/1525-01-persistence-test-coverage.md`
- Outcome: compatibility matrix exercises SQLite and multiple MySQL versions; PostgreSQL slot
  reserved for subissue 8

### 2) Add qBittorrent end-to-end test

- Spec file: `docs/issues/1525-02-qbittorrent-e2e.md`
- Outcome: one complete seeder/leecher torrent-sharing scenario using real containerized clients
  and docker compose, with SQLite as the backend

### 3) Add persistence benchmarking

- Spec file: `docs/issues/1525-03-persistence-benchmarking.md`
- Outcome: reproducible before/after performance measurements across supported backends

### 4) Split the persistence traits by context

- Spec file: `docs/issues/1525-04-split-persistence-traits.md`
- Outcome: smaller interfaces with lower coupling and clearer responsibilities

### 5) Migrate SQLite and MySQL drivers to async `sqlx`

- Spec file: `docs/issues/1525-05-migrate-sqlite-and-mysql-to-sqlx.md`
- Outcome: shared async persistence substrate without adding PostgreSQL yet

### 6) Introduce schema migrations

- Spec file: `docs/issues/1525-06-introduce-schema-migrations.md`
- Outcome: schema changes become explicit, versioned, and testable

### 7) Align persisted counters and Rust/SQL type boundaries

- Spec file: `docs/issues/1525-07-align-rust-and-db-types.md`
- Outcome: explicit contract for persisted counters and numeric ranges, with any needed schema
  changes delivered through migrations

### 8) Add PostgreSQL driver support

- Spec file: `docs/issues/1525-08-add-postgresql-driver.md`
- Outcome: PostgreSQL support lands on top of the refactored and migration-backed persistence
  layer; PostgreSQL is added to the compatibility matrix (subissue 1) and qBittorrent E2E
  (subissue 2) test harnesses

## PR Strategy

- Current branch for the planning docs: `1525-persistence-plan`
- Merge this planning PR into `develop` first.
- After the planning PR is merged, create one branch per subissue from `develop`.
- Keep the PRs narrow and link them back to this EPIC.

## Acceptance Criteria

- [ ] The EPIC plan is merged into `develop`.
- [ ] Each subissue has its own specification file in `docs/issues/`.
- [ ] The implementation order is explicit and justified.
- [ ] The plan references PR #1695 and PR #1700 as historical context, not as the delivery vehicle.

## References

- Related issue: #1525
- Related PRs: #1695, #1700
- Related discussion: PostgreSQL support request #462
