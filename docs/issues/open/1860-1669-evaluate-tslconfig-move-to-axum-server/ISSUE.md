---
doc-type: issue
issue-type: task
status: resolved
priority: p3
github-issue: 1860
spec-path: docs/issues/open/1860-1669-evaluate-tslconfig-move-to-axum-server/ISSUE.md
branch: null
related-pr: null
last-updated-utc: 2026-06-03 00:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - packages/axum-server/src/tsl.rs
    - packages/configuration/src/lib.rs
    - docs/issues/open/1669-overhaul-packages/DECISIONS.md
    - docs/issues/open/1669-overhaul-packages/EPIC.md
---

<!-- skill-link: create-issue -->

# Issue #1860 — Evaluate moving `TslConfig` from `torrust-tracker-configuration` into `torrust-tracker-axum-server`

## Goal

Decide whether `TslConfig` should be moved out of `torrust-tracker-configuration`
into `torrust-tracker-axum-server`, where it is its only production consumer.

Record a decision entry in `DECISIONS.md`. Implement the chosen approach if it is
beneficial.

This is **FU-2** from the analysis in issue
[#1856](https://github.com/torrust/torrust-tracker/issues/1856) (DEC-07).

This issue is a subissue of EPIC [#1669](../1669-overhaul-packages/EPIC.md).

## Background

`TslConfig` is currently defined in `torrust-tracker-configuration`
(`packages/configuration/src/v2_0_0/tls.rs`). Its only production consumer is
`torrust-tracker-axum-server` (`packages/axum-server/src/tsl.rs`). No other production
code in the workspace depends on `TslConfig` directly.

This makes `torrust-tracker-axum-server` depend on the full configuration package for a
two-field struct (`ssl_cert_path` and `ssl_key_path`) that has
no relationship to the config file schema or TOML deserialization.

The EPIC.md already flags this as a temporary coupling:

> `TslConfig` remains the temporary tracker-specific dependency: it is a small two-field
> struct with no tracker-specific logic and could be moved to a generic package. Once that
> change lands, the package could move to the `torrust-` group as a generic
> `torrust-axum-server` reusable across the Torrust organisation.

### Options

| Option | Description                                                            | Benefit                                                                    |
| ------ | ---------------------------------------------------------------------- | -------------------------------------------------------------------------- |
| A      | Move `TslConfig` to `torrust-tracker-axum-server`                      | Removes `axum-server`'s config dependency                                  |
| B      | Move `TslConfig` to a new generic location (e.g. `torrust-server-lib`) | Enables `axum-server` → `torrust-axum-server` extraction to org-level repo |
| C      | Keep as-is                                                             | Document why the gain is too small to act on                               |

## Proposed Analysis Steps

### Step 1 — Audit `TslConfig` usage

Confirm all usages of `TslConfig` across the workspace:

```bash
grep -rn "TslConfig" packages/ src/ --include="*.rs"
```

Verify that `torrust-tracker-axum-server` is the only non-test, non-config consumer.

### Step 2 — Evaluate dependency direction impact

- If Option A: check whether `torrust-tracker-configuration` deserializes TLS config from
  `[tls]` in `tracker.toml`. If yes, a re-export or mapping step is needed so deserialization
  still constructs the moved type.
- If Option B: identify whether `torrust-server-lib` is the right home or whether a dedicated
  `torrust-axum-tls` micro-package is warranted.

### Step 3 — Record decision

Add a decision entry (e.g. DEC-08) to `DECISIONS.md`.

### Step 4 — Implement (if Option A or B chosen)

Move the type, update import sites, update Cargo manifests, run tests.

## Acceptance Criteria

- [x] A decision entry is added to `docs/issues/open/1669-overhaul-packages/DECISIONS.md`
      with chosen approach and rationale
- [x] Option C chosen: `TslConfig` stays in `torrust-tracker-configuration`, the
      package boundary stays tracker-scoped, and no new TLS DTO package was added
- [x] All tests pass; no new clippy warnings — not applicable because no code changes
      were required for the selected option

## Out of Scope

- Extracting `torrust-tracker-axum-server` to a standalone repo (tracked separately in EPIC)
- Moving `TrackerPolicy` or `PrivateMode` (FU-1, #1859)
- Changing `EnvContainer::initialize` (FU-3, #1861)

## Layer Impact

Option A removes the edge `axum-server → configuration`. This does not introduce any
forbidden dependency edges per the EPIC layer guardrails. It makes `axum-server` a
pure framework-integration layer with no domain-level config coupling.

## Related

- Parent EPIC: #1669 — [EPIC.md](../1669-overhaul-packages/EPIC.md)
- Decision recorded: DECISIONS.md DEC-08
- Analysis: #1856 — [ISSUE.md](../1856-1669-analyse-configuration-package-coupling/ISSUE.md)
- EPIC note: see "Note on `torrust-tracker-axum-server`" in EPIC.md
- Follow-ups: FU-1 (#1859), FU-3 (#1861)

---

## Codebase Audit (2026-06-03)

### `TslConfig` structural facts

`TslConfig` is defined in `packages/configuration/src/lib.rs`:

```rust
#[serde_as]
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone, Default)]
pub struct TslConfig {
    #[serde(default = "TslConfig::default_ssl_cert_path")]
    pub ssl_cert_path: Utf8PathBuf,
    #[serde(default = "TslConfig::default_ssl_key_path")]
    pub ssl_key_path: Utf8PathBuf,
}
```

It carries `#[derive(Serialize, Deserialize)]` and `#[serde(...)]` on its fields.
It is the TOML deserialization type, embedded in both `HttpApi.tsl_config` and
`HttpTracker.tsl_config`. It therefore **cannot be moved out of `configuration`** without
either (a) keeping a DTO copy there, or (b) making `configuration` import from wherever
the type lands — which would invert a dependency edge.

### Production consumers

| Site                         | File                                                        | Role                                                          |
| ---------------------------- | ----------------------------------------------------------- | ------------------------------------------------------------- |
| `HttpApi.tsl_config`         | `packages/configuration/src/v2_0_0/tracker_api.rs`          | TOML deserialization field                                    |
| `HttpTracker.tsl_config`     | `packages/configuration/src/v2_0_0/http_tracker.rs`         | TOML deserialization field                                    |
| `make_rust_tls`              | `packages/axum-server/src/tsl.rs`                           | Uses `ssl_cert_path` / `ssl_key_path` to build `RustlsConfig` |
| `axum-http-server` (2 sites) | `packages/axum-http-server/src/server.rs`, `environment.rs` | Passes `&tsl_config` to `make_rust_tls`                       |

`make_rust_tls` in `axum-server` is the only **behavioral** consumer.
`HttpApi` and `HttpTracker` are **structural** consumers — they hold the type purely
for deserialization.

### Revised options

The original option table in the spec above was written before confirming that `TslConfig`
carries `Serialize`/`Deserialize` and is embedded in two config structs. The analysis below
supersedes it.

| Option                                                                                                                     | Description                                                                                                                                                                 | Dependency edge change                                                                                                                             | Verdict                                                 |
| -------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------- |
| **A — Move to `axum-server`**                                                                                              | Define `TslConfig` in `axum-server`; `configuration` imports it for deserialization.                                                                                        | `configuration → axum-server` introduced. Delivery layer ← config. **Inverted edge — not viable.**                                                 | ❌ Off the table                                        |
| **B — Move to `server-lib`**                                                                                               | Define `TslConfig` in `torrust-server-lib`; both `configuration` and `axum-server` import from there.                                                                       | `configuration → server-lib`, `axum-server → server-lib`. No forbidden edges. Aligns with EPIC goal of extracting a generic `torrust-axum-server`. | ✅ Architecturally sound                                |
| **C — Keep in `configuration`; change `make_rust_tls` to accept raw paths**                                                | `axum-server` stops importing `TslConfig` at all. Call sites extract `cert` and `key` individually before calling the function. No new type or package needed.              | `axum-server → configuration` removed.                                                                                                             | ✅ Minimal and clean                                    |
| **D — Keep `TslConfig` in `configuration` as TOML DTO; define internal `TlsConfig` in `axum-server`; map at the boundary** | `configuration` owns the TOML DTO; `axum-server` owns its internal type; a one-line mapping converts between them at the call site. Aligns with DEC-06 (map at boundaries). | `axum-server → configuration` removed (no longer needs the type).                                                                                  | ✅ Principled but adds boilerplate for a 2-field struct |

### Design tension

You raised an important concern: exposing inner implementation details through the public
configuration API couples the internals to the public contract. The configuration type is
part of the public TOML schema — changing `TslConfig` would be a schema-breaking change.
If `axum-server` has its own internal `TlsConfig`, the schema DTO and the runtime type
evolve independently and the boundary between "what the user configures" and "what the code
uses" is explicit.

On the other hand, for a 2-field struct with no tracker-specific logic, the DTO and the
internal type are identical in practice. The mapping is trivial, but every change to the
schema still requires updating two types.

### Constraint from the "build-your-own tracker" goal and DEC-09

Issue #1861 (DEC-09, now closed) narrowed `HttpTrackerEnvironment::new` to accept
`(&Arc<Core>, &Arc<HttpTracker>)` instead of `&Arc<Configuration>`. The HTTP-only
example (`packages/axum-http-server/examples/http_only_public_tracker.rs`) now
constructs an `HttpTracker` directly:

```rust
let http_tracker = HttpTracker {
    bind_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 0),
    tsl_config: None,
    tracker_usage_statistics: false,
};
```

This reveals an important constraint: **`TslConfig` is now part of the public API for
building custom trackers**. A user composing an HTTP-only tracker must construct
`HttpTracker`, which includes `tsl_config: Option<TslConfig>`. They will import and
use `TslConfig` from whatever package it lives in.

This changes the shape of the decision in two ways:

1. **Moving `TslConfig` to `axum-server` (Option A) would be worse, not better**, in
   this scenario. A user building a custom HTTP tracker would have to import
   `HttpTracker` from `configuration` _and_ `TslConfig` from `axum-server` — two
   separate packages for one config struct and one of its fields. That is a worse
   developer experience than the current coupling.

2. **The deeper architectural tension is now visible**: `HttpTracker` (defined in
   `configuration`) is used both as the TOML deserialization DTO _and_ as the
   runtime config passed directly to the HTTP tracker service. These two roles are
   conflated. The question you are raising is whether there should be a clean
   separation:
   - **TOML DTO** (public schema contract, owned by `configuration`): what the user
     writes in `tracker.toml` or constructs in application code.
   - **Service config** (internal runtime contract, owned by the service package):
     what the HTTP tracker server actually reads at runtime.

   With a clean separation, a mapping step at the boundary converts the public DTO
   into the internal service config. This is the DEC-06 pattern applied to
   configuration. The cost is the mapping boilerplate and the version synchronization
   discipline between the two representations.

   Without this separation (status quo), the configuration type _is_ the service
   config. Evolution is simpler (one type to change), but changing any field is
   immediately a public schema break.

### The version-synchronization problem

If the service packages define their own internal config types, each change to a
service's runtime behaviour that requires a new config option must be coordinated in
two places:

1. The service package's internal type (to add the field).
2. The global `configuration` package's TOML DTO (to expose it in the schema).

This means the global `configuration` package version must be bumped whenever any
service adds a new config field — even if the change is entirely internal to that
service. The version coupling is not eliminated; it is just made explicit through a
mapping layer. The benefit is that the _shape_ of the coupling is controlled: the
service's internal type can change freely as long as the mapping from the DTO
handles the translation.

---

## Open Questions (awaiting answers)

**Q1 — Is Option A off the table, and does the build-your-own constraint settle it?**

The analysis concludes Option A is not viable for two independent reasons:

1. Embedding `TslConfig` in `HttpApi` and `HttpTracker` for TOML deserialization would
   require `configuration → axum-server`, inverting the layering.
2. With DEC-09 in place, a user building a custom HTTP tracker constructs `HttpTracker`
   directly. If `TslConfig` lived in `axum-server`, they would need to import from both
   `configuration` (for `HttpTracker`) and `axum-server` (for `TslConfig`) — a worse
   experience than today.

Do you agree that Option A is off the table?

> **Answer:**

Yes, I agree.

**Q2 — Should there be a clean DTO / service-config separation (DEC-06 pattern)?**

The core architectural question is whether the configuration type (`HttpTracker`,
`TslConfig`) should be both the TOML DTO _and_ the runtime service config, or whether
the service should own its internal config and map from the DTO at the boundary.

- **No separation (status quo / Path C)**: `HttpTracker` and `TslConfig` from
  `configuration` flow all the way into the service. Simplest, no mapping boilerplate.
  Any change to a field is immediately a schema change. `make_rust_tls` in `axum-server`
  could be changed to accept `(cert: &Utf8PathBuf, key: &Utf8PathBuf)` directly,
  removing the `TslConfig` import from `axum-server` with zero new types.
- **Clean separation (Path D)**: `axum-server` defines its own `TlsConfig`; a mapping
  converts `configuration::TslConfig` → `axum_server::TlsConfig` at the boundary.
  Aligns with DEC-06. Adds a trivial `From` impl for a 2-field struct. Service internals
  can evolve without touching the schema.

For `TslConfig` specifically, there is no functional difference between the two today
(same two fields, same types). The question is whether you want to establish the
DTO-separation pattern now as a precedent, or whether you consider it premature given
the type's simplicity. If we do not establish it here, the inconsistency with DEC-06
is intentional and should be documented in the decision.

> **Answer:**

That looks overengineered for a 2-field struct that is unlikely to change. I see it more like the type `SocketAddr` from the standard library: it is both the deserialization type and the runtime type, and that is fine. If we had a more complex config with more fields and more complex logic, I would be more inclined to separate the DTO from the internal type, but for this case I think it's fine to keep them together.

**Q3 — Does `TslConfig` belong in `server-lib` long-term (Path B)?**

The EPIC mentions extracting `axum-server` as a generic `torrust-axum-server` reusable
across the Torrust organisation. If the extraction is planned soon, moving `TslConfig`
to `server-lib` now would give it a neutral home that neither `configuration` nor
`axum-server` owns. If the extraction is far off, this is premature abstraction.

Should we act on this now or defer? And if we defer, should the decision entry
explicitly flag this as a deferred action so it is not forgotten?

> **Answer:**

Maybe, but for now that package has common functionality used in all Torrust servers, not only HTTP servers. What about "packages/axum-http-server"?

**Q4 — Tests in `axum-server/src/tsl.rs` construct `TslConfig` directly.**

If we go with Path C (`make_rust_tls` accepts raw paths), the tests would build
`Utf8PathBuf` directly with no config type involved. If we go with Path D (internal
`TlsConfig`), the tests would construct the new internal type. Either way the
`axum-server → configuration` dependency is removed — including the dev-dependency.
Is that acceptable, or do you want to keep the tests constructing `TslConfig` from
`configuration` (e.g. as an integration check that the mapping is correct)?

> **Answer:**

It does not make sense not to use it, just to remove the dependency if that abstraction makes sense in the test.

### Preferred choice from the discussion

Based on the current answers, the preferred choice is **Option C** with the current
tracker-scoped package boundary preserved:

- keep `TslConfig` in `torrust-tracker-configuration`
- keep `torrust-tracker-axum-server` as tracker-specific infrastructure rather than a
  generic org-level wrapper
- keep the `HttpTracker` public API self-contained for custom tracker composition
- avoid introducing a new package only for `TslConfig`

That preference keeps the build-your-own tracker story straightforward while avoiding a
new abstraction layer for a two-field config DTO that is already part of the public
tracker configuration contract.
