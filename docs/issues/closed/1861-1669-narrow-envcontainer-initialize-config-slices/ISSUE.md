---
doc-type: issue
issue-type: task
status: closed
priority: p3
github-issue: 1861
spec-path: docs/issues/open/1861-1669-narrow-envcontainer-initialize-config-slices/ISSUE.md
branch: 1861-1669-narrow-envcontainer-initialize-config-slices
related-pr: null
last-updated-utc: 2026-06-05 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/tracker-core/src/container.rs
    - packages/udp-server/examples/udp_only_public_tracker.rs
    - packages/axum-http-server/examples/http_only_public_tracker.rs
    - packages/configuration/src/v2_0_0/mod.rs
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---


# Issue #1861 — Revisit `EnvContainer::initialize` to accept narrower config slices

## Goal

Evaluate whether the initialization API of `EnvContainer` (and related server
environment types) should accept narrower config slices (`Arc<Core>`,
`Arc<UdpTracker>`, etc.) instead of `&Arc<Configuration>`.

Record a decision in `DECISIONS.md`. Implement the chosen approach if it is adopted.

This is **FU-3** from the analysis in issue
[#1856](https://github.com/torrust/torrust-tracker/issues/1856) (DEC-07).

This issue is a subissue of EPIC [#1669](../1669-overhaul-packages/EPIC.md).

## Background

Issue #1856 found that the root cause for a UDP-only binary compiling the full
`Configuration` aggregate is not the package structure of the config crate — it is the
`EnvContainer::initialize` / `Environment::new` signature. These functions take
`&Arc<Configuration>`, which means the compiler must resolve and compile `HttpTracker`,
`HttpApi`, `HealthCheckApi`, `TslConfig`, and `AccessTokens` types even when none of those
services run.

Example evidence from the UDP-only Cargo example (`udp_only_public_tracker.rs`):

| Config type      | Compiled | Used                             |
| ---------------- | -------- | -------------------------------- |
| `Core`           | Yes      | Yes                              |
| `UdpTracker`     | Yes      | Yes                              |
| `HttpTracker`    | Yes      | **No** — idle                    |
| `HttpApi`        | Yes      | **No** — idle                    |
| `HealthCheckApi` | Yes      | **No** — idle                    |
| `TslConfig`      | Yes      | **No** — idle                    |
| `AccessTokens`   | Yes      | **No** — idle (private mode off) |

If narrowing is adopted, a UDP-only server environment would be initialized as:

```rust
UdpEnvironment::new(&arc_core, &arc_udp_tracker)
```

instead of:

```rust
UdpEnvironment::new(&arc_configuration)
```

## Proposed Analysis Steps

### Step 1 — Trace `EnvContainer::initialize` call sites

Identify every place in the workspace (binary entry points, integration tests, examples)
that calls `EnvContainer::initialize`, `UdpTrackerEnvironment::new`,
`HttpTrackerEnvironment::new`, and similar constructors that accept `&Arc<Configuration>`.

### Step 2 — Prototype narrow signature (spike)

Introduce a prototype version of `UdpTrackerEnvironment::new` that accepts
`(&Arc<Core>, &Arc<UdpTracker>)`. Confirm that the UDP Cargo example compiles without
pulling in `HttpTracker` config.

### Step 3 — Evaluate full migration cost

Assess how the main binary (`src/bootstrap/`) would provide the narrower slices. Determine
whether a decomposition helper in `torrust-tracker-configuration` (e.g. `Configuration::core()`,
`Configuration::udp_tracker()`) is sufficient or whether a deeper redesign is needed.

### Step 4 — Record decision

Add a decision entry (e.g. DEC-09) to `DECISIONS.md` with the chosen approach.

### Step 5 — Implement (if narrowing is adopted)

Update `EnvContainer::initialize` and all `Environment::new` constructors. Update all
call sites. Confirm the Cargo examples no longer compile idle types.

## Acceptance Criteria

- [x] A decision entry is added to `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
      with chosen approach and rationale (DEC-09)
- [x] If narrowing is adopted: `UdpTrackerEnvironment::new` accepts narrower config types
      and the UDP Cargo example no longer compiles `HttpTracker`/`HttpApi`/etc.
- [x] If narrowing is adopted: `HttpTrackerEnvironment::new` accepts narrower config types
      and the HTTP Cargo example no longer compiles `UdpTracker`/`HealthCheckApi`/etc.
- [x] All tests pass (`cargo test --workspace`); no new clippy warnings
- [x] The Cargo examples still run correctly end-to-end (as verified by the manual test
      results in `docs/issues/open/1856-.../manual-test-results.md`)

## Out of Scope

- Moving `TrackerPolicy`/`PrivateMode` (FU-1, #1859)
- Moving `TslConfig` (FU-2, #1860)
- Full persistence layer redesign (#1525)
- Any changes to the TOML config file format or schema versioning

## Notes

If narrowing requires changes to `src/bootstrap/`, those changes must remain backwards
compatible with the full tracker binary (`cargo run`) and the Docker container startup.

## Related

- Parent EPIC: #1669 — [EPIC.md](../1669-overhaul-packages/EPIC.md)
- Decision to be added: DECISIONS.md DEC-09 (or next available)
- Analysis: #1856 — [ISSUE.md](../1856-1669-analyse-configuration-package-coupling/ISSUE.md)
- UDP example: `packages/udp-server/examples/udp_only_public_tracker.rs`
- HTTP example: `packages/axum-http-server/examples/http_only_public_tracker.rs`
- Follow-ups: FU-1 (#1859), FU-2 (#1860)
