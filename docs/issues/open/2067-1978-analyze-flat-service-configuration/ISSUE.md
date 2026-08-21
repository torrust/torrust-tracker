---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 2067
spec-path: docs/issues/open/2067-1978-analyze-flat-service-configuration/ISSUE.md
branch: "2067-analyze-flat-service-configuration"
related-pr: 2068
depends-on: null
blocks: null
last-updated-utc: 2026-08-20 16:51
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .github/skills/dev/planning/create-issue/SKILL.md
    - docs/issues/open/1978-configuration-overhaul-epic/EPIC.md
    - docs/issues/open/1978-configuration-overhaul-epic/configuration-v2-to-v3-migration.md
    - docs/issues/open/1490-1978-decompose-database-config-and-overhaul-secrets.md
    - docs/issues/open/2067-1978-analyze-flat-service-configuration/analysis.md
    - docs/issues/open/2067-1978-analyze-flat-service-configuration/evidence.md
    - docs/issues/open/2067-1978-analyze-flat-service-configuration/first-impressions.md
    - packages/configuration/src/lib.rs
    - packages/configuration/src/v2_0_0/mod.rs
    - packages/configuration/src/v3_0_0/mod.rs
    - packages/configuration/src/v3_0_0/logging.rs
    - packages/primitives/src/configuration_instance_id.rs
    - packages/primitives/src/service_role.rs
    - src/app.rs
    - src/bootstrap/app.rs
    - src/bootstrap/jobs/health_check_api.rs
    - src/container.rs
    - packages/udp-core/src/container.rs
    - tests/common/configuration.rs
---

<!-- skill-link: create-issue -->

# Issue #2067 - Analyze a flat heterogeneous service configuration (sub-issue of #1978)

## Goal

Determine whether a future version of the Torrust Tracker configuration schema can represent all listener/service instances in one ordered, heterogeneous `services` collection instead of separate `http_trackers`, `udp_trackers`, `http_api`, and `health_check_api` sections.

Produce a decision-ready analysis covering viable TOML and Rust representations, benefits, costs, compatibility and migration implications, service lifecycle effects, the relationship with `ConfigurationInstanceId`, and a high-level implementation estimate. The output is a recommendation to reject, defer, or create a separate implementation issue. This is an analysis-only task; it must not implement a schema change, a v4 configuration loader, a migration tool, or production runtime changes.

## Background

The tracker main binary supervises several independently configured listener services in one process. The current configuration organizes them by concrete role:

- `[[http_trackers]]` contains zero or more HTTP tracker listeners.
- `[[udp_trackers]]` contains zero or more UDP tracker listeners.
- `[http_api]` optionally configures the management REST API.
- `[health_check_api]` configures the health-check API.

For example, `tests/common/configuration.rs` contains two HTTP and two UDP trackers, each configured with `bind_address = "0.0.0.0:0"`. Port zero is valid and causes the operating system to choose the final port only after binding. A configured socket address therefore cannot uniquely identify an in-process listener for its full lifecycle. HTTP and UDP may also validly use the same port because they use different transports.

Recent work introduced `ConfigurationInstanceId`, currently composed of a `ServiceRole` and a zero-based ordinal within that role's configuration-entry list. It identifies a running service against the configuration used to start the process without relying on a configured or final socket address. It remains stable when port-zero binding selects a new port after restart, but intentionally changes when the relevant configuration entries are reordered.

During weekly planning, Cameron proposed representing the listener services as a single flat, ordered list of polymorphic service entries. Such a structure could make the configuration mirror the process's service inventory more directly, but it would be a breaking schema design decision with broad effects. In particular, a flat list may alter how an entry relates to `ConfigurationInstanceId`; this issue must analyze that relationship without reopening the already chosen general strategy for service runtime identity.

The current v3 configuration module still uses the existing split structure, while the application remains on the v2 public aliases pending #1980. This analysis must distinguish an immediately feasible schema representation from the proper delivery point in the configuration-overhaul roadmap.

This is a non-blocking research sub-issue of #1978. It may inform a later schema version, but it must not delay the v3.0.0 delivery or expand #1978's implementation scope. Any implementation recommended by this analysis must be tracked in a new issue and scheduled after #1980; the analysis must also account for the #1490 secrets work that #1980 depends on.

## Illustrative Configuration Outcome

The following comparison deliberately starts from the v3 configuration schema, not the current v2 runtime configuration shown in `tests/common/configuration.rs`. The v2-to-v3 changes are independently planned under the Configuration Overhaul EPIC and #1980. This issue would be a later, separate breaking schema change built on top of v3: it changes only how v3's already-defined service configurations are organized at the root level.

Consequently, the two examples use the same service-specific fields, nested structures, and shared `udp_tracker_server` policy. Their only intentional difference is the root-level representation: v3 uses role-specific sections; the illustrative successor uses a heterogeneous `services` list. The successor is a design example only, not a selected representation or a commitment to use the exact field names below. This analysis must validate its TOML and Serde feasibility and may recommend rejecting or changing the proposed form.

### Before: v3 Role-Specific Service Sections

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "3.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "{STORAGE_PATH}/sqlite3.db"

[[http_trackers]]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = false
use_ip_from_query_string = false
public_url = "https://tracker.example.com/announce"

[http_trackers.network]
external_ip = "203.0.113.5"
on_reverse_proxy = true
ipv6_v6only = false

[http_trackers.tls_config]
ssl_cert_path = "./storage/tracker/lib/tls/tracker.crt"
ssl_key_path = "./storage/tracker/lib/tls/tracker.key"

[[http_trackers]]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = true
use_ip_from_query_string = true
public_url = "http://tracker.example.com:7070/announce"

[http_trackers.network]
on_reverse_proxy = false
ipv6_v6only = false

[[udp_trackers]]
bind_address = "0.0.0.0:0"
cookie_lifetime = { secs = 120, nanos = 0 }
tracker_usage_statistics = true
max_connection_id_errors_per_ip = 10
public_url = "udp://tracker.example.com:6969"

[udp_trackers.network]
on_reverse_proxy = false
ipv6_v6only = false

[[udp_trackers]]
bind_address = "0.0.0.0:0"
cookie_lifetime = { secs = 60, nanos = 0 }
tracker_usage_statistics = false
max_connection_id_errors_per_ip = 5
public_url = "udp://tracker.example.com:6969"

[http_api]
bind_address = "127.0.0.1:0"
public_url = "https://api.tracker.example.com/"

[http_api.access_tokens]
admin = "MyAccessToken"

[http_api.tls_config]
ssl_cert_path = "./storage/tracker/lib/tls/api.crt"
ssl_key_path = "./storage/tracker/lib/tls/api.key"

[health_check_api]
bind_address = "127.0.0.2:0"

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
connection_id_validation = "strict"
```

### After: Illustrative Flat Heterogeneous Service Collection

The example uses an **adjacently tagged** representation: every list item has a `kind` discriminator and a nested `configuration` table. It models a Rust `Vec<Service>`, where `Service` is an enum with one variant per service type, and each variant wraps the corresponding v3 role-specific configuration type. This avoids requiring all service variants to share the same fields.

```toml
[metadata]
app = "torrust-tracker"
purpose = "configuration"
schema_version = "4.0.0"

[logging]
trace_filter = "info"
trace_style = "full"

[core]
listed = false
private = false

[core.database]
driver = "sqlite3"
path = "{STORAGE_PATH}/sqlite3.db"

[[services]]
kind = "http_tracker"

[services.configuration]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = false
use_ip_from_query_string = false
public_url = "https://tracker.example.com/announce"

[services.configuration.network]
external_ip = "203.0.113.5"
on_reverse_proxy = true
ipv6_v6only = false

[services.configuration.tls_config]
ssl_cert_path = "./storage/tracker/lib/tls/tracker.crt"
ssl_key_path = "./storage/tracker/lib/tls/tracker.key"

[[services]]
kind = "udp_tracker"

[services.configuration]
bind_address = "0.0.0.0:0"
cookie_lifetime = { secs = 120, nanos = 0 }
tracker_usage_statistics = true
max_connection_id_errors_per_ip = 10
public_url = "udp://tracker.example.com:6969"

[services.configuration.network]
on_reverse_proxy = false
ipv6_v6only = false

[[services]]
kind = "http_tracker"

[services.configuration]
bind_address = "0.0.0.0:0"
tracker_usage_statistics = true
use_ip_from_query_string = true
public_url = "http://tracker.example.com:7070/announce"

[services.configuration.network]
on_reverse_proxy = false
ipv6_v6only = false

[[services]]
kind = "udp_tracker"

[services.configuration]
bind_address = "0.0.0.0:0"
cookie_lifetime = { secs = 60, nanos = 0 }
tracker_usage_statistics = false
max_connection_id_errors_per_ip = 5
public_url = "udp://tracker.example.com:6969"

[[services]]
kind = "http_api"

[services.configuration]
bind_address = "127.0.0.1:0"
public_url = "https://api.tracker.example.com/"

[services.configuration.access_tokens]
admin = "MyAccessToken"

[services.configuration.tls_config]
ssl_cert_path = "./storage/tracker/lib/tls/api.crt"
ssl_key_path = "./storage/tracker/lib/tls/api.key"

[[services]]
kind = "health_check_api"

[services.configuration]
bind_address = "127.0.0.2:0"

[udp_tracker_server]
ip_bans_reset_interval_in_secs = 86400
connection_id_validation = "strict"
```

TOML attaches each `[services.configuration]` table and its nested tables to the immediately preceding `[[services]]` entry. `udp_tracker_server` remains top-level because it configures policy shared by all UDP listeners rather than one listener instance. The illustrative schema therefore requires a new schema version beyond the current v3 model; `4.0.0` is only a placeholder, not a release decision.

In this illustration, declaration order represents the configuration's service inventory; the analysis must determine whether it would also carry startup-order semantics. A recommended design must define validation for singleton service kinds and clarify whether `ConfigurationInstanceId` continues to use role-local ordinals while scanning this list or adopts global list positions.

## Analysis Deliverables

This folder-style issue separates the execution contract, the decision record, and the supporting evidence. The analysis must create no production schema or runtime implementation.

| Artifact      | Purpose                                                                                              | Completion Standard                                                                                                                      |
| ------------- | ---------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| `ISSUE.md`    | Scope, tasks, acceptance criteria, progress, and verification contract.                              | Keep it current as work proceeds; do not put the full analysis here.                                                                     |
| `analysis.md` | Final decision-ready report for maintainers.                                                         | Complete every required section, state one recommendation, and identify any follow-up issue(s) or explicit rejection/defer rationale.    |
| `evidence.md` | Reproducible evidence ledger for source tracing, TOML/Serde/Figment experiments, and manual reviews. | Every material conclusion in `analysis.md` links to one or more evidence records with commands, source paths, observations, and results. |

This open issue is stored at `docs/issues/open/2067-1978-analyze-flat-service-configuration/`; `ISSUE.md`, `analysis.md`, `evidence.md`, and `first-impressions.md` remain siblings. Do not create a production implementation branch or production configuration files as part of this analysis.

### Required `analysis.md` Sections

1. **Executive Decision**: recommendation (`reject`, `defer`, or `create implementation issue`), decision status, rationale, prerequisites, and proposed owner/follow-up.
2. **Current-State Baseline**: v3 configuration shape, cardinality/defaulting, startup phases, container/registry behavior, configuration identity, shared UDP state, and secret-redaction boundary.
3. **Candidate Representations**: at least two TOML/Rust shapes, including the adjacent-tagged candidate; operator ergonomics and validation consequences for each.
4. **Feasibility Results**: TOML parsing, Serde serialization round-trip, Figment defaulting and environment overrides, unknown/discriminator errors, and constraints discovered by prototypes.
5. **Runtime and Normalization Model**: recommended single owner for normalization, role-specific views, service startup dependencies, singleton/default behavior, and preservation of existing health/metrics/registration contracts.
6. **Identity, Ordering, and Migration**: `ServiceKind` to `ServiceRole` mapping, `ConfigurationInstanceId` behavior, loss of cross-role ordering during v3-to-v4 migration, and a canonical migration-order rule if implementation is recommended.
7. **Schema Lifecycle, Security, and Compatibility**: v3/v4 loading and transition policy, #1980/#1490 relationship, secret redaction, external configuration consumers, and observability compatibility.
8. **Cost, Risks, and Recommendation**: affected modules, high-level effort, unresolved risks, decision rationale, and exact scope for any follow-up implementation issue.

### Required `evidence.md` Record Format

Each evidence record uses the following fields:

```markdown
## E<N>: <short title>

- **Question**: What decision does this evidence support?
- **Status**: `TODO`, `PASS`, `FAIL`, or `BLOCKED`.
- **Method**: Source paths inspected, test fixture, command, or manual steps.
- **Observation**: Relevant output or source-level fact.
- **Conclusion**: What the observation proves or leaves unresolved.
- **Report Links**: Section(s) in `analysis.md` that use this evidence.
```

For an experiment, preserve the exact TOML input and command in the record. Test-only prototype code may be added only when necessary to establish feasibility; it must not change the public configuration schema or runtime behavior.

## Scope

### In Scope

- Document the current service configuration model, including cardinality, ordering, defaulting, and startup behavior for HTTP trackers, UDP trackers, the REST API, and the health-check API.
- Evaluate whether the current Rust, Serde, TOML, and Figment stack can deserialize and serialize an ordered heterogeneous service list.
- Compare practical TOML/Rust representation options, including at least:
  - an adjacent-tagged enum with a role/kind discriminator and nested per-service configuration;
  - an internally tagged/flattened representation, including whether it requires duplicated fields or custom deserialization;
  - an externally tagged or equivalent representation where relevant.
- Evaluate configuration usability, readability, validation, environment-variable overrides, default configuration generation, and serialization/round-trip behavior for each viable representation.
- Identify the required semantic rules that are currently structural, including singleton handling for the REST API and health-check API and the current always-started/defaulted health-check behavior. Define expected behavior for an omitted `services` list, an empty list, no health-check entry, duplicate singleton entries, and UDP entries in private mode.
- Analyze whether `udp_tracker_server` remains a top-level shared support-service configuration or belongs in a flat listener list.
- Inventory configuration values that look per-listener but are consumed through shared runtime services, including `max_connection_id_errors_per_ip`. Recommend whether each must become shared, be validated as consistent, or be redesigned in a separate implementation issue; do not make that runtime change here.
- Analyze service startup and container construction consequences, including whether list order would define startup order or only configuration presentation order. Define the conceptual normalization boundary that assigns IDs once and supplies consistent role-specific views to container construction, job startup, registration, and metrics.
- Analyze the relationship with the existing `ConfigurationInstanceId` contract:
  - preserve its role-qualified, per-role ordinal semantics when scanning a flat list; and
  - describe the consequences of instead using the global list position.
- Define a typed `ServiceKind` to `ServiceRole` mapping, including the distinction between the configuration-facing `http_api` kind and the existing `RestApi` runtime role.
- Treat `ConfigurationInstanceId` as an existing constraint. Do **not** explore alternative identifier schemes such as explicit user-provided IDs, socket addresses after binding, or configuration hashes.
- Identify migration, documentation, test, and consumer impacts, including the dependency/order relationship with #1980 and schema-versioning implications. Decide whether a future application accepts only the successor schema, dispatches among schema versions, or requires an external migration; state that v3 cannot express a cross-role service order and define any canonical migration order.
- Analyze the effect of moving `HttpApi` inside a service enum on configuration logging, JSON serialization, and redaction of API tokens, including compatibility with #1490's planned secret types.
- Preserve existing post-bind `ServiceBinding`, health-check registration, and metrics behavior as compatibility invariants, even though changing those public contracts is out of scope.
- Provide a high-level implementation estimate, dependency plan, risks, and a recommended next step: reject, defer, or create a separate implementation issue.

### Out of Scope

- Implementing a flat `services` configuration schema.
- Changing the definition of `ConfigurationInstanceId` or evaluating alternate runtime identifier designs.
- Changing service bindings, `ServiceBinding`, metrics behavior, listener protocols, or runtime behavior beyond documenting the potential impact of a schema change.
- Making the REST API or health-check API multi-instance unless the analysis identifies that as a necessary consequence requiring a separately approved decision.
- Replacing the global `udp_tracker_server` policy with per-listener configuration.
- Changing the active v2 runtime configuration or completing #1980.
- Implementing a successor schema parser, dual-version dispatcher, configuration migration tool, normalizer, or production container/job changes.
- Changing secret storage, secret types, or redaction policy; those remain owned by #1490.
- Creating any implementation issue before the final analysis recommendation is reviewed and approved.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Notes / Expected Output                                                                                                                                                                               |
| --- | ------ | ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Capture the current model             | Record v3 shape, cardinality/defaulting, startup phases, identity, role mappings, shared UDP state, service registration, observability contracts, and secret-redaction paths in `evidence.md`.       |
| T2  | TODO   | Prototype schema representations      | Add isolated, non-production TOML/Serde/Figment experiments for viable enum forms, including round-trip behavior, numeric environment overrides, unknown kinds, and default/empty-list cases.         |
| T3  | TODO   | Compare configuration representations | Record readability, ergonomics, validation, environment override, round-trip serialization, and backwards-migration trade-offs for each option in `analysis.md`.                                      |
| T4  | TODO   | Analyze runtime integration           | Define a conceptual single normalization owner, role-specific views, startup dependencies, shared UDP policies, singleton/default behavior, and compatibility invariants without changing production. |
| T5  | TODO   | Analyze identity compatibility        | Compare role-local ordinals with global positions; define `ServiceKind` to `ServiceRole` mapping and show how one normalizer keeps IDs, containers, jobs, and registry metadata aligned.              |
| T6  | TODO   | Define migration and schema lifecycle | Decide v3-to-successor ordering rules, schema loading/transition strategy, v3 compatibility policy, #1980/#1490 prerequisites, and the non-blocking relationship to the v3 EPIC.                      |
| T7  | TODO   | Analyze security and operator impact  | Document redaction, configuration logging/serialization, external configuration consumers, deployment overrides, and post-bind observability compatibility.                                           |
| T8  | TODO   | Write the final analysis deliverables | Complete `analysis.md` and `evidence.md`; ensure every recommendation is traceable to evidence and no production implementation is included.                                                          |
| T9  | TODO   | Run automatic checks                  | Run `linter all` and relevant focused tests for any analysis fixtures or documentation tooling changes.                                                                                               |
| T10 | TODO   | Perform manual review                 | Review candidate TOML, normalizer pseudocode, migration rules, the report/evidence cross-links, and the impact inventory; record evidence.                                                            |
| T11 | TODO   | Re-review acceptance criteria         | Update evidence after the analysis and recommendation are complete.                                                                                                                                   |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2067)
- [x] Linked as a sub-issue of #1978 in GitHub and in the EPIC specification
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before analysis work
- [ ] Analysis completed; no production schema change included
- [ ] `analysis.md` completed with an explicit recommendation
- [ ] `evidence.md` completed with reproducible evidence for each material conclusion
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded (status + evidence)
- [ ] Acceptance criteria reviewed after analysis and updated with evidence
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-08-20 UTC - Copilot/User - Drafted an analysis-only sub-issue after weekly planning discussion. The proposed scope evaluates a heterogeneous listener-service list while explicitly retaining the existing `ConfigurationInstanceId` strategy as a constraint.
- 2026-08-20 UTC - Copilot/User - Converted the draft to a folder-style analysis issue. Added the final report and evidence-ledger contract, clarified that the work is non-blocking research for a potential post-v3 successor, and expanded the analysis scope around migration, normalization, shared UDP state, defaults, security, and compatibility.
- 2026-08-20 16:36 UTC - Copilot/User - User approved the draft. Created GitHub Task #2067 and linked it as the thirteenth native sub-issue of #1978 after restoring #2023's missing native parent relationship.
- 2026-08-20 16:44 UTC - Copilot - Renamed the folder to include the parent EPIC number, as required for folder-based subissue specifications.
- 2026-08-20 16:51 UTC - Copilot/User - Opened spec-only PR #2068 against `develop`, linked it as related to #2067, and requested review from @da2ce7 because the proposal originated with Cameron.

## Acceptance Criteria

- [ ] AC1: The analysis describes the current v2/v3 service configuration, cardinality rules, startup sequence, shared runtime state, secret-redaction boundary, and the role of `ConfigurationInstanceId`, using concrete source references.
- [ ] AC2: At least two viable TOML/Rust representations for a heterogeneous ordered service list are compared, with an explicit recommendation or rejection rationale.
- [ ] AC3: Feasibility is demonstrated or disproved with focused deserialization, serialization, defaulting, and environment-override evidence using the repository's supported configuration stack; no production schema change is made.
- [ ] AC4: The analysis states whether list order controls startup order, configuration presentation order, both, or neither; explains the loss of cross-role order when migrating v3; and identifies the necessary runtime constraints.
- [ ] AC5: The analysis explicitly evaluates omitted/empty lists, duplicate/absence validation for REST API and health-check API entries, private-mode UDP behavior, and the placement of shared `udp_tracker_server` configuration.
- [ ] AC6: The analysis documents the consequences of preserving role-local `ConfigurationInstanceId` ordinals versus using global list positions; defines the `ServiceKind` to `ServiceRole` mapping; and recommends a single normalization boundary consistent with the existing identifier contract.
- [ ] AC7: The analysis inventories shared UDP behavior, including `max_connection_id_errors_per_ip`, and recommends a future policy without changing current runtime behavior.
- [ ] AC8: The analysis identifies schema migration/versioning and transition requirements, #1980/#1490 dependency implications, affected configuration consumers, documentation, defaults, test fixtures, and a high-level implementation estimate.
- [ ] AC9: The analysis documents secret-redaction, configuration logging/serialization, and post-bind health/metrics/registration compatibility constraints.
- [ ] AC10: `analysis.md` contains every required section, makes one explicit recommendation, and identifies a precise follow-up implementation issue or rejection/defer rationale.
- [ ] AC11: `evidence.md` contains reproducible evidence records for every material conclusion in `analysis.md`.
- [ ] AC12: `linter all` exits with code `0` for all changes made by this analysis task.
- [ ] AC13: Relevant focused tests pass for any experiment or analysis fixture added by this task.
- [ ] AC14: Manual verification scenarios are executed and documented with status and evidence.
- [ ] AC15: Acceptance criteria are re-reviewed after analysis and reflect actual evidence.
- [ ] AC16: Documentation is updated when the analysis changes the configuration roadmap or governance artifacts.

## Verification Plan

### Automatic Checks

- `linter all`
- Focused `cargo test` commands for `torrust-tracker-configuration` and any new experiment/fixture modules
- Relevant serialization, Figment loading, and environment-override tests when a candidate representation is exercised
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                         | Command/Steps                                                                                                                                                                                     | Expected Result                                                                                                                                           | Status | Evidence                                                  |
| --- | -------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------- |
| M1  | Review current port-zero fixture | Compare `tests/common/configuration.rs` with configuration structs, bootstrap, containers, shared UDP services, registry, and redaction paths.                                                    | Evidence explains role-local IDs, post-bind identities, shared policy behavior, and current compatibility constraints.                                    | TODO   | `evidence.md#e1-current-state-baseline`                   |
| M2  | Review candidate TOML files      | Parse and serialize interleaved entries for each viable form. Exercise unknown kinds, numeric environment overrides, omitted/empty lists, missing health entries, and duplicate singletons.       | Each result records syntax, readability, round-trip behavior, defaulting, error quality, and compatibility with nested TLS/network/access-token settings. | TODO   | `evidence.md#e2-configuration-representation-feasibility` |
| M3  | Review normalization plan        | Trace a representative interleaved list through conceptual normalization, role-local ID allocation, container lookup, startup phases, registration, and metrics without changing production code. | The analysis identifies one consistent normalization boundary and proves whether source list order affects startup or presentation only.                  | TODO   | `evidence.md#e3-runtime-and-identity-model`               |
| M4  | Review migration and transition  | Compare the recommended form with v3/default configs, environment overrides, docs, integration fixtures, #1980, and #1490. Define a canonical migration order and version-loading policy.         | The impact inventory, compatibility policy, prerequisites, and implementation estimate are complete; unresolved constraints are explicit.                 | TODO   | `evidence.md#e4-migration-schema-lifecycle-and-security`  |
| M5  | Review final reports             | Check every conclusion in `analysis.md` against the linked record in `evidence.md`; confirm the recommendation does not include implementation work.                                              | The decision record is complete, traceable, and limited to analysis plus a proposed follow-up scope when warranted.                                       | TODO   | `evidence.md#e5-report-review`                            |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1   | TODO                   |          |
| AC2   | TODO                   |          |
| AC3   | TODO                   |          |
| AC4   | TODO                   |          |
| AC5   | TODO                   |          |
| AC6   | TODO                   |          |
| AC7   | TODO                   |          |
| AC8   | TODO                   |          |
| AC9   | TODO                   |          |
| AC10  | TODO                   |          |
| AC11  | TODO                   |          |
| AC12  | TODO                   |          |
| AC13  | TODO                   |          |
| AC14  | TODO                   |          |
| AC15  | TODO                   |          |
| AC16  | TODO                   |          |

## Risks and Trade-offs

- **Public breaking change:** Replacing top-level role-specific sections requires a new configuration schema version, migration guidance, and coordinated changes in deployment and automation consumers.
- **Configuration ergonomics:** A representation that is easy for Serde to deserialize may be materially harder for operators to read and edit. The final recommendation must value human-maintained TOML as well as implementation simplicity.
- **Implicit rules become validation:** A heterogeneous list no longer makes REST API and health-check API singleton cardinality structural. The schema would require clear semantic validation and error messages.
- **Order semantics can become accidental:** A flat source order must not silently become a startup-order or identity contract. Each meaning must be explicitly chosen and tested.
- **Identity disruption:** Switching `ConfigurationInstanceId` to global list positions would make an unrelated preceding service insertion renumber later services. Retaining role-local ordinals is expected to minimize disruption, but the analysis must confirm the integration consequences.
- **Bootstrap complexity:** Current startup is role-grouped and has UDP support-job prerequisites. A dispatcher that directly follows list order could introduce invalid lifecycle ordering unless it normalizes entries or enforces dependencies.
- **Environment override uncertainty:** Numeric paths for list entries may not work with current Figment override behavior. This must be verified before recommending the schema.
- **Unrecoverable migration order:** V3 stores role-local order but not a cross-role order. A migration cannot reconstruct a desired interleaving; the analysis must recommend a canonical order or require explicit operator reordering.
- **Hidden shared UDP policy:** A field placed on a UDP listener can still configure one shared runtime service. The analysis must expose and resolve that semantic mismatch before a flat list makes ordering effects less visible.
- **Schema lifecycle ambiguity:** A v4 representation requires an explicit transition, compatibility, or migration strategy because a versioned configuration loader accepts one schema shape at a time.
- **Secret exposure:** Nesting API configuration in an enum can bypass current redaction paths unless serialization/logging behavior is explicitly tested and coordinated with #1490.
- **Roadmap conflict:** Implementing the change before #1980 would create parallel v3 schema work while the application still consumes v2 aliases. This analysis is non-blocking; any implementation must be separately scheduled after #1980 and its prerequisites.

## References

- Parent EPIC: [#1978 — Configuration Overhaul](../1978-configuration-overhaul-epic/EPIC.md)
- Current lifecycle identity: `packages/primitives/src/configuration_instance_id.rs`
- Service roles: `packages/primitives/src/service_role.rs`
- Port-zero multi-listener fixture: `tests/common/configuration.rs`
- Current application bootstrap: `src/app.rs`
- Current configuration logging and redaction: `src/bootstrap/app.rs`
- Current instance-container construction: `src/container.rs`
- Shared UDP service construction: `packages/udp-core/src/container.rs`
- Current schema v2: `packages/configuration/src/v2_0_0/mod.rs`
- Candidate schema v3: `packages/configuration/src/v3_0_0/mod.rs`
- Runtime v3 consumer migration: [#1980](../1980-1978-configuration-overhaul-final-cleanup.md)
- Secrets and database follow-up: [#1490](../1490-1978-decompose-database-config-and-overhaul-secrets.md)
- Final decision record: [analysis.md](analysis.md)
- Evidence ledger: [evidence.md](evidence.md)
