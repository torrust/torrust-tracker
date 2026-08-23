# Analysis Report: Flat Heterogeneous Service Configuration

> **Status:** Complete — recommendation: reject the flat TOML schema change
>
> **Issue contract:** [ISSUE.md](ISSUE.md)
>
> **Evidence ledger:** [evidence.md](evidence.md)

This is the final decision record for the analysis-only issue. It must recommend exactly one
outcome: reject the change, defer the change, or create a separate implementation issue. It must
not describe unapproved production work as implemented.

## Executive Decision

| Field                  | Result                                                                                                                                                                                                                          |
| ---------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Recommendation         | **Reject** a flat heterogeneous `[[services]]` TOML collection for the current v3.0.0 schema.                                                                                                                                   |
| Decision status        | Ready for maintainer review.                                                                                                                                                                                                    |
| Rationale              | The split layout is clearer for the common one-HTTP-or-one-UDP deployment, preserves structural cardinality, and avoids a breaking migration. The flat form supplies no demonstrated operator benefit that offsets those costs. |
| Required prerequisites | None for this rejection. Complete #2079, #1490, and #1980 under their existing plans.                                                                                                                                           |
| Proposed follow-up     | Do not create the proposed configuration-schema implementation issue. Defer any internal normalized listener inventory until a concrete lifecycle consumer cannot use the existing registry and role-specific container views.  |

The rejection is limited to changing the **operator-facing TOML shape**. It does not prohibit a
future internal service inventory when it is justified independently of the configuration schema.
The existing `Registar<RuntimeServiceMetadata>` already provides an inventory of successfully
started listeners, while the job manager intentionally also contains non-listener work. See
[E1](evidence.md#e1-current-state-baseline) and [E3](evidence.md#e3-runtime-and-identity-model).

## Current-State Baseline

Schema v3 currently has separate root fields: optional `Vec<HttpTracker>` and `Vec<UdpTracker>`,
an optional `HttpApi`, a defaulted `HealthCheckApi`, and defaulted shared
`UdpTrackerServer` policy. Consequently, trackers are $0..N$, the REST API is structurally
$0..1$, and health checking has exactly one effective configuration even when no TOML health
section is supplied. The health listener is always started; a missing REST API is not. [E1](evidence.md#e1-current-state-baseline)

The application currently imports the v2 public aliases. The v3 module is the appropriate
analysis target, but no production v3 consumer migration is valid before #1980. Runtime startup
is role- and dependency-grouped: shared UDP support work precedes UDP listeners, then HTTP
listeners, optional REST API, and unconditional health API. Therefore source declaration order
has no current startup meaning and must not acquire one. [E1](evidence.md#e1-current-state-baseline)

`ConfigurationInstanceId` is the established runtime identity: `(ServiceRole, role-local index)`.
It deliberately excludes both configured and bound addresses, which is required for valid
port-zero listeners. REST and health already register as `RestApi(0)` and `HealthCheckApi(0)`.
Registration instead records the final post-bind `ServiceBinding`, preserving metrics and health
contracts. [E1](evidence.md#e1-current-state-baseline)

The primary baseline defect is unrelated to TOML layout: each `UdpTracker` exposes
`max_connection_id_errors_per_ip`, yet container construction reads only the first UDP entry to
initialize one shared ban service. This is a confirmed configuration-model bug, not merely an
open design choice: a setting consumed by one shared service must be global/shared, or the runtime
must construct genuinely independent per-instance services. The shared-services ADR requires the
former for the ban service. The separately tracked bug record defines the correction boundary;
this analysis does not implement it. [E1](evidence.md#e1-current-state-baseline)

## Candidate Representations

| Representation                                              | TOML and Rust shape                                                                                                                                                             | Advantages                                                                                                                                                                                                     | Costs and decision                                                                                                                                                                                                                                                                    |
| ----------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Current split TOML plus optional internal normalization** | `[[http_trackers]]`, `[[udp_trackers]]`, optional `[http_api]`, defaulted `[health_check_api]`; normalize role-specific views only inside a lifecycle boundary if later needed. | Names and role-specific fields remain adjacent; common single-service files require no type discriminator; singleton cardinality is structural; named nested Figment overrides remain supported; no migration. | Cross-role source order cannot be expressed; numeric overrides of any list entry are unsupported by the current Figment provider. **Recommended.**                                                                                                                                    |
| **Adjacent-tagged list**                                    | `Vec<Service>` with `#[serde(tag = "kind", content = "configuration")]`. Each list item has `kind` plus a nested configuration table.                                           | The most viable flat representation: preserves per-kind typed configuration, TOML order, Serde round trips, and clear unknown-kind rejection.                                                                  | Adds a discriminator and nesting before each service's fields; duplicate singleton rules move to custom validation; omitted health needs normalizer defaulting; numeric list environment overrides fail; a future successor-schema migration invents an order. **Rejected for TOML.** |
| **Internally tagged flattened list**                        | `#[serde(tag = "kind")]` plus `#[serde(flatten)]` wrapped role configuration.                                                                                                   | Removes one TOML nesting level and round-trips.                                                                                                                                                                | Mixes discriminators with fields whose meaning varies by type, makes field discovery less local, and has no compensating benefit for common deployments. **Not recommended.**                                                                                                         |
| **Externally tagged list**                                  | `Vec<ExternallyTaggedService>` such as `[services.http_tracker]`.                                                                                                               | Round-trips and has no explicit discriminator field.                                                                                                                                                           | Adds a role-named wrapper table, duplicates the role grouping at per-item granularity, and is less discoverable than current sections. **Not recommended.**                                                                                                                           |

For an operator with one HTTP or one UDP listener—the expected primary deployment—the split form
has a direct path from service purpose to its fields. A flat list imposes the extra steps “find
the list entry” and “interpret its kind” before fields can be evaluated. Interleaving service
types is only valuable when it represents an operational ordering, but ordering must not control
startup and the current model has no demonstrated operator workflow requiring it. [E2](evidence.md#e2-configuration-representation-feasibility)

## Feasibility Results

Isolated tests using the repository's `toml`, Serde, and Figment versions confirm that adjacent,
flattened, and externally tagged enum forms parse and serialize an interleaved service document.
Adjacent tagging rejects an unknown `kind`. It is therefore technically feasible, but feasibility
does not make it an appropriate operator schema. [E2](evidence.md#e2-configuration-representation-feasibility)

The prototype establishes a provider limitation, not a flat-list regression: Figment's environment
provider merges a numeric path such as `SERVICES__0__CONFIGURATION__BIND_ADDRESS` as a map, not a
sequence item, so extraction fails with `InvalidType(Map, "a sequence")`. The equivalent current
split-list override also fails. Named nested overrides such as `HTTP_API__ACCESS_TOKENS__ADMIN`
remain supported. A flat representation would inherit this existing list-override limitation;
it would need a separate provider solution only if indexed listener overrides become a requirement.
[E2](evidence.md#e2-configuration-representation-feasibility)

For example, an operator may expect this current split-list configuration and override to change
the listener's bind address:

```toml
[[http_trackers]]
bind_address = "127.0.0.1:7070"
```

```text
TORRUST_TRACKER_CONFIG_OVERRIDE_HTTP_TRACKERS__0__BIND_ADDRESS=127.0.0.1:17070
```

Instead, Figment merges the environment path as a table/map and fails because `http_trackers`
must deserialize as a sequence. The adjacent flat-list equivalent fails for the same reason:

```text
TORRUST_TRACKER_CONFIG_OVERRIDE_SERVICES__0__CONFIGURATION__BIND_ADDRESS=127.0.0.1:17070
```

Do not add an alternative canonical configuration layout solely to solve this unproven deployment
need. A map keyed by operator-chosen listener names could make an override path such as
`HTTP_TRACKERS__PUBLIC__BIND_ADDRESS` feasible, but it would replace ordering with naming,
introduce an additional schema and migration decision, and make the common single-listener TOML
less direct. If deployments demonstrate a need for per-listener environment overrides, investigate
that option or a configuration-provider capability in a separate issue. Until then, operators can
provide the complete listener configuration through `TORRUST_TRACKER_CONFIG_TOML` or use a mounted
TOML file. [E2](evidence.md#e2-configuration-representation-feasibility)

An omitted or empty prototype list deserializes as empty. That alone does **not** preserve the
current default health listener: normalization would need to materialize `HealthCheckApi::default`
when no health entry exists. Duplicate `http_api` and `health_check_api` entries also require
explicit semantic diagnostics, whereas the split TOML form makes duplicates structurally
impossible. [E2](evidence.md#e2-configuration-representation-feasibility)

## Runtime and Normalization Model

Do not implement a normalization layer now. If a concrete internal consumer later needs one, it
must be the sole boundary between parsed configuration and runtime assembly. It would scan the
chosen configuration representation once, assign role-local IDs, validate singleton and shared
policy rules, materialize the default health configuration, and expose role-specific ordered views
to existing container and job startup code. No container, job, metrics collector, or registry
consumer should independently translate a list position into a role-local identity. [E3](evidence.md#e3-runtime-and-identity-model)

The normalized output must retain dependency grouping: initialize core and shared UDP services;
start prerequisite UDP event/cleanup jobs before UDP listeners; then HTTP listeners; then optional
REST API and health API. Declaration order is presentation order only. It must preserve
post-bind `ServiceBinding`, `RuntimeServiceMetadata`, registration, and metrics behavior. For
UDP entries in private mode, current semantics are retained: configuration is accepted, startup
skips UDP and logs a warning; this is not a schema validation failure. [E1](evidence.md#e1-current-state-baseline)

## Identity, Ordering, and Migration

If a future typed configuration enum is ever justified, its configuration-facing mapping must be:

| `ServiceKind`      | Runtime role                  |
| ------------------ | ----------------------------- |
| `http_tracker`     | `ServiceRole::HttpTracker`    |
| `udp_tracker`      | `ServiceRole::UdpTracker`     |
| `http_api`         | `ServiceRole::RestApi`        |
| `health_check_api` | `ServiceRole::HealthCheckApi` |

`http_api` intentionally does not expose the runtime serialization name `tracker_rest_api` to
operators. A single scanner can preserve IDs by incrementing a separate ordinal for each mapped
role; the prototype proves this for interleaved HTTP and UDP entries. Using global list positions
would renumber an HTTP entry when an unrelated earlier UDP entry is added, contradicting the
existing identity contract and destabilizing metrics/container lookups. [E3](evidence.md#e3-runtime-and-identity-model)

The present split layout records ordering only within each role; it cannot recover cross-role
order. If an external migration ever has to materialize a flat list, it must document the approved
synthetic order: HTTP trackers, UDP trackers, HTTP API, health-check API. That rule is a
deterministic export convention, not recovery of historical startup or operator order. Since the
flat TOML proposal is rejected, no migration tool or dual loader is proposed. [E4](evidence.md#e4-migration-schema-lifecycle-and-security)

## Schema Lifecycle, Security, and Compatibility

Keep the current v3 role-specific layout and do not introduce a dual layout or migration tool.
The v3 release continues independently through the #2079 secrecy prerequisite, #1490 database
configuration work, and #1980 consumer migration. Any future flat representation would be a
separately approved successor-schema decision after #1980, with its own versioning and migration
plan. [E4](evidence.md#e4-migration-schema-lifecycle-and-security)

The current bootstrap boundary is concrete: `src/bootstrap/app.rs::setup` logs
`configuration.clone().mask_secrets().to_json()` through `tracing::info!`. `Configuration::mask_secrets`
first masks the database and then explicitly descends into root `http_api`; only the resulting clone
is JSON serialized and logged. A hypothetical `Vec<Service>` enum must preserve that exact ordering:
clone the complete configuration, exhaustively traverse every secret-carrying enum variant (currently
the `HttpApi` variant) to mask it, and only then call `to_json` for the log. A test-only enum prototype
confirms that traversal removes an API token from serialized JSON; it must be extended for every future
secret-bearing variant. The #2079 secrecy prerequisite and #1490 database configuration work make
this boundary more important. Retaining the split root prevents new traversal risk while those
planned changes complete. [E1](evidence.md#e1-current-state-baseline) [E2](evidence.md#e2-configuration-representation-feasibility)

## Cost, Risks, and Recommendation

Implementing flat TOML would change at least `packages/configuration` loading/defaulting/
serialization/validation/redaction, default configuration files, migration documentation,
fixtures, configuration consumers, containers, bootstrap jobs, registration/metrics tests, and
environment override behavior. It would also require a later successor-schema migration after
issues #2079, #1490, and #1980 complete. The confirmed UDP shared-policy bug must be fixed independently
rather than preserving first-entry-wins behavior. [E1](evidence.md#e1-current-state-baseline) [E4](evidence.md#e4-migration-schema-lifecycle-and-security)

The estimates are deliberately qualitative because the flat schema is rejected before an approved
implementation design exists. A complete flat TOML delivery is **large, multi-week work**: it spans
public schema and default/migration surfaces, semantic validation and secret-redaction traversal,
and cross-package lifecycle regression coverage after the completed v3 migration.
An internal normalizer that retains split TOML is **medium, multi-day to small multi-week work** if a
concrete consumer justifies it; its size is driven by establishing one ID/default/shared-policy owner
and adapting its consumers, rather than by external migration. Neither estimate authorizes work;
both exclude the separately required correction for the shared UDP error-limit bug. [E1](evidence.md#e1-current-state-baseline) [E3](evidence.md#e3-runtime-and-identity-model) [E4](evidence.md#e4-migration-schema-lifecycle-and-security)

The adjacent enum is feasible, but its only distinct benefit—cross-role presentation order—does
not improve the primary operator workflows and cannot influence lifecycle startup. Its costs are
concrete: less local TOML, semantic singleton/default rules, unsupported indexed overrides,
breaking migration, and redaction changes. **Reject the flat TOML implementation and do not
create a new #1978 implementation sub-issue.**

The remaining opportunity is deliberately deferred, not committed: if a future runtime feature
needs a complete configuration-derived listener inventory beyond the existing registry, create a
separate issue for an internal normalizer while retaining the role-specific TOML model. It must
first define a consumer, shared UDP policy handling, and role-local ID ownership. [E3](evidence.md#e3-runtime-and-identity-model)

## Evidence Index

| Report area                                   | Evidence                                                                                                                                                   |
| --------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Executive decision and current-state baseline | [E1](evidence.md#e1-current-state-baseline), [E3](evidence.md#e3-runtime-and-identity-model), [E4](evidence.md#e4-migration-schema-lifecycle-and-security) |
| Candidate representations and feasibility     | [E2](evidence.md#e2-configuration-representation-feasibility)                                                                                              |
| Runtime/normalization and identity            | [E1](evidence.md#e1-current-state-baseline), [E3](evidence.md#e3-runtime-and-identity-model)                                                               |
| Migration, lifecycle, security, cost          | [E1](evidence.md#e1-current-state-baseline), [E4](evidence.md#e4-migration-schema-lifecycle-and-security)                                                  |
