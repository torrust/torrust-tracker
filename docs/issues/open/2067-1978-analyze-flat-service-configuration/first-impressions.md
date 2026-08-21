# Preliminary Impressions: Flat Heterogeneous Service Configuration

> **Status:** Provisional snapshot written before the deeper analysis
>
> **Date:** 2026-08-20
>
> **Issue contract:** [ISSUE.md](ISSUE.md)
>
> **Later decision record:** [analysis.md](analysis.md)

This document deliberately records an initial opinion, not a conclusion. Do not rewrite it after
the analysis. Instead, compare its claims with the evidence and final recommendation in
`analysis.md`.

## Initial Recommendation

**Defer implementation.** The proposal is worth analyzing, but I would not currently recommend
creating an implementation issue or scheduling it after v3 solely because a flat list looks
cleaner than role-specific root sections.

The adjacent-tagged `services` representation appears technically plausible with the current
Serde, TOML, and Figment stack. That makes the investigation worthwhile. However, technical
plausibility is not enough for a breaking configuration-schema change: the operational benefit is
not yet demonstrated, while the migration and runtime integration cost is already concrete.

## What Looks Promising

- A single inventory can make a configuration with many listeners easier to scan.
- The structure gives future service kinds one consistent root-level extension point.
- It can model heterogeneous service-specific settings without forcing unrelated configuration
  fields into one shared structure.
- Preserving the existing role-local `ConfigurationInstanceId` ordinal while scanning the list
  appears conceptually possible, avoiding a change to the established runtime identity contract.

## Why I Am Cautious

- The current role-specific configuration is not merely cosmetic. The application builds
  role-specific containers and starts grouped lifecycle phases; UDP listeners require shared
  jobs before their instances start, and the health-check API always starts.
- A flat source order does not remove those runtime distinctions. It introduces a normalization
  step that must produce consistent role-specific views for container construction, job startup,
  registration, metrics, and identity allocation.
- `ConfigurationInstanceId` is explicitly a role plus a role-local index. Using global list
  positions would make unrelated insertions renumber later services and would be a regression.
- Current UDP configuration exposes an important semantic mismatch: the shared `BanService` is
  initialized from one UDP listener's `max_connection_id_errors_per_ip` value. A flat list could
  make this policy less visible without resolving it.
- A v3-to-successor migration cannot recover a meaningful cross-role order, because v3 stores
  independent HTTP and UDP lists rather than one interleaved inventory. Any migration must impose
  a canonical order or require operator intervention.
- The change must retain defaulting, environment overrides, configuration serialization, and
  secret masking. Moving `http_api` into an enum could bypass the current explicit redaction path
  unless it is redesigned and tested with the #1490 work.

## What Would Change My Mind

I would lean toward implementation only if the deeper analysis establishes all of the following:

1. A concrete operator or maintainer workflow is materially improved, beyond aesthetic
   consistency. Examples could include an existing need to manage a mixed service inventory,
   clearer extensibility for planned service kinds, or a documented configuration error that the
   current grouping causes.
2. A focused prototype proves that the selected representation round-trips through TOML, supports
   required Figment defaults and numeric environment overrides, and produces clear validation
   errors for unknown kinds and invalid singleton combinations.
3. A small, explicit normalization model preserves role-local identity allocation and grouped
   startup dependencies without spreading positional translation across containers and jobs.
4. The design resolves or clearly separates shared UDP policy from per-listener configuration.
5. A migration and schema-transition policy is acceptable to operators, including a documented
   canonical ordering rule and compatible secret-redaction behavior.

## Current Confidence

| Question                                                               | Preliminary view                                       |
| ---------------------------------------------------------------------- | ------------------------------------------------------ |
| Is the representation technically feasible?                            | Probably, pending focused Serde/TOML/Figment evidence. |
| Does it provide a demonstrated user-facing benefit today?              | Not yet.                                               |
| Is the implementation likely to stay local to the configuration crate? | No.                                                    |
| Should it block or expand v3 work?                                     | No.                                                    |
| Should maintainers commit to implementing it now?                      | No; defer pending the analysis.                        |

## Reassessment Record

When the deeper analysis finishes, add a new entry below without editing the preceding sections.

| Date | Final outcome | Which initial impressions held, changed, or were disproved? | Link                       |
| ---- | ------------- | ----------------------------------------------------------- | -------------------------- |
| TODO | TODO          | TODO                                                        | [analysis.md](analysis.md) |

## Source Basis for This Snapshot

This initial opinion is based on a narrow source review, not a feasibility prototype:

- `packages/configuration/src/v3_0_0/mod.rs`: role-specific schema, Figment loading/defaulting,
  exact schema-version validation, and explicit `http_api` secret masking.
- `packages/primitives/src/configuration_instance_id.rs` and
  `packages/primitives/src/service_role.rs`: role-qualified, role-local service identity.
- `src/container.rs` and `src/app.rs`: separate HTTP and UDP container lists, role-grouped
  startup, and UDP prerequisite jobs.
- `packages/udp-core/src/container.rs`: shared `BanService` initialization from a UDP
  configuration value.
- `src/bootstrap/app.rs`: masked configuration logging.
