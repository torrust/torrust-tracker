# Evidence Ledger: Flat Heterogeneous Service Configuration

> **Status:** Complete
>
> **Issue contract:** [ISSUE.md](ISSUE.md)
>
> **Decision record:** [analysis.md](analysis.md)

This ledger holds reproducible evidence for the analysis. A record may cite source code, a
test-only prototype, an exact command, or a manual review. It must not claim a production schema
or runtime change was implemented.

## E1: Current-State Baseline

- **Question:** What current configuration, runtime, identity, shared-state, and redaction
  contracts constrain the analysis?
- **Status:** PASS
- **Method:** Reviewed `packages/configuration/src/v3_0_0/mod.rs`, `http_tracker.rs`,
  `udp_tracker.rs`, `tracker_api.rs`, `health_check_api.rs`, and `udp_tracker_server.rs`;
  `packages/configuration/src/lib.rs`; `src/bootstrap/app.rs`, `src/app.rs`, and
  `src/container.rs`; `packages/primitives/src/configuration_instance_id.rs`,
  `service_role.rs`, and `runtime_service_metadata.rs`; `packages/udp-core/src/container.rs`;
  and `tests/common/configuration.rs`.
- **Observation:** V3 has optional HTTP/UDP vectors and HTTP API, but defaulted health and shared
  UDP server sections. Production global aliases still select v2 until #1980. Startup groups
  shared UDP work before UDP instances, then HTTP instances, optional REST, and health. IDs are
  role-local; REST and health use ordinal zero. The registry records final post-bind bindings.
  The shared UDP ban service takes `max_connection_id_errors_per_ip` from only the first configured
  UDP listener. V3 manual `mask_secrets` explicitly descends into the root `http_api`.
- **Conclusion:** The split schema encodes cardinality/defaulting structurally and is distinct from
  the existing role-grouped runtime lifecycle. Any future normalizer needs one ownership point for
  ID allocation, health defaulting, singleton validation, and shared UDP policy. It must retain
  post-bind registration and redaction behavior.
- **Report Links:** `analysis.md` sections "Current-State Baseline" and "Runtime and Normalization Model".

## E2: Configuration Representation Feasibility

- **Question:** Which TOML/Rust enum representations parse, serialize, validate, and support
  required configuration-source behavior?
- **Status:** PASS
- **Method:** Added test-only local types under
  `packages/configuration/src/v3_0_0/mod.rs::tests::flat_service_configuration_prototype`.
  Ran:

  `cargo test -p torrust-tracker-configuration flat_service_configuration_prototype -- --nocapture`

  Adjacent-tagged TOML input:

  ```toml
  [[services]]
  kind = "http_tracker"
  [services.configuration]
  bind_address = "127.0.0.1:17070"

  [[services]]
  kind = "udp_tracker"
  [services.configuration]
  bind_address = "127.0.0.1:16969"
  ```

  Flat-list indexed override input:

  ```text
  TORRUST_TRACKER_CONFIG_OVERRIDE_SERVICES__0__CONFIGURATION__BIND_ADDRESS=127.0.0.1:18080
  ```

  Equivalent split-list indexed override input:

  ```text
  TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_TRACKERS__0__BIND_ADDRESS=127.0.0.1:18080
  ```

- **Observation:** All nine focused tests passed. Adjacent, flattened/internal-tagged, and
  externally tagged forms round-trip through TOML and Serde. Adjacent tagging rejects an unknown
  kind. Omitted and empty lists deserialize as empty; duplicate singleton kinds need semantic
  validation. Both flat and equivalent split-list indexed Figment overrides fail extraction with
  `InvalidType(Map, "a sequence")`; the current named nested HTTP API override remains covered by
  an existing test.
- **Conclusion:** An adjacent enum is technically feasible and shares the current Figment
  limitation for indexed listener overrides. It transfers singleton/default behavior from
  structure to custom normalization/validation. Flattened and external forms are feasible but less
  operator-friendly.
- **Report Links:** `analysis.md` sections "Candidate Representations" and "Feasibility Results".

## E3: Runtime and Identity Model

- **Question:** Can one normalization model preserve role-local IDs, container lookups, startup
  dependencies, registration, and metrics behavior for interleaved services?
- **Status:** PASS
- **Method:** Traced `src/container.rs::{initialize,
initialize_http_tracker_instance_containers,initialize_udp_tracker_instance_containers}` and
  `src/app.rs::{start_jobs,start_udp_tracker_services,start_the_http_instances,start_the_http_api}`.
  Reviewed prototype test `role_local_ids_remain_stable_when_another_role_precedes_a_service`.
- **Observation:** Existing container construction assigns IDs beside per-role containers, and
  jobs retrieve those containers by role-local index. The prototype scans interleaved services and
  yields `UdpTracker(0)`, `HttpTracker(0)`, `UdpTracker(1)`, `HttpTracker(1)`. Startup is grouped
  by dependency rather than declaration order. `Registar` already inventories started listeners;
  the job manager includes both listeners and non-listener jobs.
- **Conclusion:** A flat source list can preserve existing IDs only through one scanner with
  role-specific counters. Global list positions are incompatible. An internal normalizer is
  possible without a flat TOML schema, but no current consumer demonstrates that it is required.
- **Report Links:** `analysis.md` sections "Runtime and Normalization Model" and "Identity, Ordering, and Migration".

## E4: Migration, Schema Lifecycle, and Security

- **Question:** What migration order, schema transition policy, dependency order, and redaction
  constraints would a successor schema require?
- **Status:** PASS
- **Method:** Reviewed `packages/configuration/src/lib.rs`, v3 load/default/version checks,
  `src/bootstrap/app.rs`, #1490 at
  `docs/issues/open/1490-1978-decompose-database-config-and-overhaul-secrets.md`, and #1978 at
  `docs/issues/open/1978-configuration-overhaul-epic/EPIC.md`.
- **Observation:** Current v3 loading accepts a single exact schema version, while production
  consumers remain v2 until #1980. #1490 replaces manual secret masking and must precede #1980.
  The split layout has no cross-role ordering; a flat migration would fabricate the approved
  HTTP, UDP, REST, health order. A flat enum would require new redaction traversal, migration
  guidance, default files, fixture updates, and an override solution.
- **Conclusion:** A dual loader or migration tool adds cost without an operator benefit. Retaining
  the split layout lets #1490 and #1980 proceed without redoing their consumer migration. No
  schema implementation follow-up is warranted.
- **Report Links:** `analysis.md` sections "Identity, Ordering, and Migration" and "Schema Lifecycle, Security, and Compatibility".

## E5: Final Report Review

- **Question:** Does every material recommendation in `analysis.md` have sufficient evidence,
  and does the recommendation remain analysis-only?
- **Status:** PASS
- **Method:** Checked every required `analysis.md` section against E1–E4 and confirmed the
  prototype is restricted to `#[cfg(test)]` test-local types with no production schema/runtime
  behavior changes.
- **Observation:** The report compares three TOML representations plus internal normalization,
  provides reproducible test input and commands, identifies the Figment limit, preserves the
  identity/lifecycle constraints, and issues one recommendation.
- **Conclusion:** The decision record is traceable and remains analysis-only. The recommendation
  is to reject a flat TOML schema change and defer any internal normalizer until a real consumer
  need exists.
- **Report Links:** `analysis.md` section "Executive Decision" and "Cost, Risks, and Recommendation".
