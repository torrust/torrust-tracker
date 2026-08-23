---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 2067
spec-path: docs/issues/open/2067-1978-analyze-flat-service-configuration/ISSUE.md
branch: "2067-analyze-flat-service-configuration"
related-pr: 2082
depends-on: null
blocks: null
last-updated-utc: 2026-08-23
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
    - docs/issues/open/2067-1978-analyze-flat-service-configuration/max-connection-id-errors-per-ip-bug.md
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

Produce a decision-ready analysis covering viable TOML and Rust representations, benefits, costs, compatibility and migration implications, service lifecycle effects, the relationship with `ConfigurationInstanceId`, and a high-level implementation estimate. The output is a recommendation to reject, defer, or create a separate implementation issue. This is an analysis-only task; it must not implement a schema change, a flat-v3 loader, a migration tool, or production runtime changes.

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

This is an analysis-only sub-issue of #1978. If the final recommendation is to implement a configuration-schema change, maintainers must first approve and create a new #1978 sub-issue. That implementation must complete after #1490 and before #1980 so that the selected shape is included in the v3.0.0 configuration release. The analysis itself must not implement the schema, migration tool, or runtime changes.

## Illustrative Configuration Outcome

The following comparison deliberately starts from the v3 configuration schema, not the current v2 runtime configuration shown in `tests/common/configuration.rs`. The v2-to-v3 changes are independently planned under the Configuration Overhaul EPIC and #1980. If approved, this issue's separately scoped implementation would select the final v3 root-level organization before #1980 migrates production consumers; it is not a post-v3 schema change.

Consequently, the two examples use the same service-specific fields, nested structures, and shared `udp_tracker_server` policy. Their only intentional difference is the root-level representation: v3 uses role-specific sections; the illustrative alternative uses a heterogeneous `services` list. The alternative is a design example only, not a selected representation or a commitment to use the exact field names below. This analysis must validate its TOML and Serde feasibility and may recommend rejecting or changing the proposed form.

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

### Alternative: Illustrative Flat Heterogeneous Service Collection

The example uses an **adjacently tagged** representation: every list item has a `kind` discriminator and a nested `configuration` table. It models a Rust `Vec<Service>`, where `Service` is an enum with one variant per service type, and each variant wraps the corresponding v3 role-specific configuration type. This avoids requiring all service variants to share the same fields.

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

TOML attaches each `[services.configuration]` table and its nested tables to the immediately preceding `[[services]]` entry. `udp_tracker_server` remains top-level because it configures policy shared by all UDP listeners rather than one listener instance. The illustrative schema would replace the existing v3 root-level role-specific layout before #1980 migrates production consumers; it does not imply an additional schema version.

In this illustration, declaration order represents the configuration's service inventory only. It must not acquire startup-order semantics: startup remains dependency-driven and role-grouped. A recommended design must define validation for singleton service kinds and clarify whether `ConfigurationInstanceId` continues to use role-local ordinals while scanning this list or adopts global list positions.

## Maintainer Direction

The final decision must remain evidence-led: decide whether the change should be implemented, deferred, or rejected. The following approved direction constrains the analysis but does not predetermine its recommendation:

- The operator-facing TOML experience is the primary configuration-design concern. Names, explicit structure, readability, and the ability to build a correct configuration without explanatory comments are more important than mirroring internal runtime types.
- Treat the current role-specific TOML layout as the operator baseline. It keeps each service type's fields close together, avoids a per-entry discriminator, and makes a known service type easy to locate. The analysis must independently test this view against the flat-list alternative rather than assuming it is correct.
- Prioritize the common deployment: one public listener of one tracker protocol, normally either a single HTTP tracker or a single UDP tracker. Also evaluate the less common one-listener-per-kind deployment. Do not optimize the primary configuration experience for uncommon multi-instance, mixed-protocol inventories without demonstrated operator value.
- The configuration representation and the internal runtime representation may differ. The analysis must compare retaining role-specific TOML while normalizing it into a polymorphic internal service inventory against exposing a flat polymorphic `services` list in TOML.
- The internal inventory must be evaluated as a possible way to manage running services, handles, jobs, threads, registration, and metrics. It must remain distinct from the broader job collection, which also contains non-listener tasks such as cleanup jobs.
- If a flat `services` TOML collection is selected, declaration order is presentation/configuration order only; startup remains dependency-driven and role-grouped.
- `http_api` and `health_check_api` are singleton kinds: each may occur at most once. `http_api` remains optional. A missing `health_check_api` entry preserves the existing implicit/default health-check behavior. `http_tracker` and `udp_tracker` remain multi-instance kinds.
- If a v2-to-v3 migration needs to materialize a flat collection, use the canonical order HTTP trackers, UDP trackers, HTTP API, then health-check API.
- If implementation is recommended and approved, create a separate #1978 sub-issue after #1490 and before #1980, so the selected configuration model is included in the v3.0.0 release.

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
6. **Identity, Ordering, and Migration**: `ServiceKind` to `ServiceRole` mapping, `ConfigurationInstanceId` behavior, loss of cross-role ordering when materializing a flat v3 collection from the current split layout, and a canonical migration-order rule if implementation is recommended.
7. **Schema Lifecycle, Security, and Compatibility**: v3 loading and transition policy, the #1490 → implementation → #1980 relationship, secret redaction, external configuration consumers, and observability compatibility.
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
- Compare the operator-facing role-specific TOML model plus a normalized internal polymorphic service inventory with a TOML-level heterogeneous `services` collection. Treat configuration UX and internal runtime organization as separate design decisions.
- Identify the required semantic rules that are currently structural, including singleton handling for the REST API and health-check API and the current always-started/defaulted health-check behavior. Define expected behavior for an omitted `services` list, an empty list, no health-check entry, duplicate singleton entries, and UDP entries in private mode.
- Analyze whether `udp_tracker_server` remains a top-level shared support-service configuration or belongs in a flat listener list.
- Inventory configuration values that look per-listener but are consumed through shared runtime services, including `max_connection_id_errors_per_ip`. Recommend whether each must become shared, be validated as consistent, or be redesigned in a separate implementation issue; do not make that runtime change here.
- Analyze service startup and container construction consequences, including whether list order would define startup order or only configuration presentation order. Define the conceptual normalization boundary that assigns IDs once and supplies consistent role-specific views to container construction, job startup, registration, and metrics.
- Analyze the relationship with the existing `ConfigurationInstanceId` contract:
  - preserve its role-qualified, per-role ordinal semantics when scanning a flat list; and
  - describe the consequences of instead using the global list position.
- Define a typed `ServiceKind` to `ServiceRole` mapping, including the distinction between the configuration-facing `http_api` kind and the existing `RestApi` runtime role.
- Treat `ConfigurationInstanceId` as an existing constraint. Do **not** explore alternative identifier schemes such as explicit user-provided IDs, socket addresses after binding, or configuration hashes.
- Identify migration, documentation, test, and consumer impacts, including the #1490 → implementation → #1980 dependency/order relationship and v3 release implications. Decide whether the selected v3 model accepts only one final shape or requires an external migration; state that the current split layout cannot express a cross-role service order and define any canonical migration order.
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
- Implementing a flat-v3 schema parser, dual-layout dispatcher, configuration migration tool, normalizer, or production container/job changes.
- Changing secret storage, secret types, or redaction policy; those remain owned by #1490.
- Creating any implementation issue before the final analysis recommendation is reviewed and approved.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                  | Notes / Expected Output                                                                                                                                                                   |
| --- | ------ | ------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | DONE   | Capture the current model             | Recorded the v3/v2 boundary, cardinality/defaulting, startup, identity, shared UDP state, registration, observability, and redaction evidence in `evidence.md#e1-current-state-baseline`. |
| T2  | DONE   | Prototype schema representations      | Added isolated test-only TOML/Serde/Figment experiments. Numeric list overrides fail with the current Figment provider; see `evidence.md#e2-configuration-representation-feasibility`.    |
| T3  | DONE   | Compare configuration representations | Compared split TOML, adjacent, flattened, and externally tagged forms in `analysis.md#candidate-representations`.                                                                         |
| T4  | DONE   | Analyze runtime integration           | Defined the conditional single-normalizer model and preserved dependency-grouped startup in `analysis.md#runtime-and-normalization-model`.                                                |
| T5  | DONE   | Analyze identity compatibility        | Documented role-local ordinal preservation, global-position consequences, and `ServiceKind` mapping in `analysis.md#identity-ordering-and-migration`.                                     |
| T6  | DONE   | Define migration and schema lifecycle | Rejected the schema transition; documented canonical export ordering and #1490/#1980 constraints in `analysis.md#schema-lifecycle-security-and-compatibility`.                            |
| T7  | DONE   | Analyze security and operator impact  | Documented redaction, logging, override, and post-bind compatibility constraints in `analysis.md`.                                                                                        |
| T8  | DONE   | Write the final analysis deliverables | Completed `analysis.md` and `evidence.md` with an analysis-only rejection recommendation.                                                                                                 |
| T9  | DONE   | Run automatic checks                  | `cargo test -p torrust-tracker-configuration` and the mandatory pre-commit gate passed using the installed stable toolchain.                                                              |
| T10 | DONE   | Perform manual review                 | Reviewed candidate presentation, report/evidence links, migration rule, and impact inventory; see M5 and `evidence.md#e5-final-report-review`.                                            |
| T11 | DONE   | Re-review acceptance criteria         | Acceptance criteria reviewed against E1–E5 and the completed validation results.                                                                                                          |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec (#2067)
- [x] Linked as a sub-issue of #1978 in GitHub and in the EPIC specification
- [ ] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before analysis work
- [x] Analysis completed; no production schema change included
- [x] `analysis.md` completed with an explicit recommendation
- [x] `evidence.md` completed with reproducible evidence for each material conclusion
- [x] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [x] Manual verification scenarios executed and recorded (status + evidence)
- [x] Acceptance criteria reviewed after analysis and updated with evidence
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-08-20 UTC - Copilot/User - Drafted an analysis-only sub-issue after weekly planning discussion. The proposed scope evaluates a heterogeneous listener-service list while explicitly retaining the existing `ConfigurationInstanceId` strategy as a constraint.
- 2026-08-20 UTC - Copilot/User - Converted the draft to a folder-style analysis issue. Added the final report and evidence-ledger contract, and expanded the analysis scope around migration, normalization, shared UDP state, defaults, security, and compatibility. This initially described a later schema change; the 2026-08-22 policy superseded that premise.
- 2026-08-20 16:36 UTC - Copilot/User - User approved the draft. Created GitHub Task #2067 and linked it as the thirteenth native sub-issue of #1978 after restoring #2023's missing native parent relationship.
- 2026-08-20 16:44 UTC - Copilot - Renamed the folder to include the parent EPIC number, as required for folder-based subissue specifications.
- 2026-08-20 16:51 UTC - Copilot/User - Opened spec-only PR #2068 against `develop`, linked it as related to #2067, and requested review from @da2ce7 because the proposal originated with Cameron.
- 2026-08-22 UTC - Copilot/User - Clarified that the analysis must decide the final schema v3.0.0 shape before #1980. If approved after the analysis, a new implementation sub-issue must follow #1490 and precede #1980. Added operator-focused configuration UX and the independent internal-normalization alternative as explicit evaluation criteria. Confirmed role-grouped, dependency-driven startup; singleton HTTP API and health-check kinds; implicit default health-check behavior; and the canonical migration order.
- 2026-08-22 UTC - Copilot - Reviewed the updated issue and EPIC roadmap specifications before committing. `git diff --check` passed; the repository `linter` executable was unavailable in this environment.
- 2026-08-22 UTC - Copilot/User - Recorded the operator baseline and deployment priorities: role-specific sections are provisionally clearer because related fields remain together, no discriminator must be read, and roles are easy to locate. The analysis must assess this against a flat list while prioritizing the common single-HTTP-or-single-UDP deployment rather than uncommon multi-instance inventories.
- 2026-08-22 UTC - Copilot - Completed source tracing and isolated TOML/Serde/Figment prototypes. The adjacent, flattened, and externally tagged forms round-trip, but numeric Figment overrides for list entries fail. Drafted the evidence-backed analysis recommending rejection of a flat TOML schema and deferral of any internal normalizer until it has a concrete consumer.
- 2026-08-22 UTC - Copilot - Completed the final manual report review and acceptance-criteria re-review. The configuration package tests and final mandatory pre-commit gate passed all checks, including `linter all` and workspace documentation tests.
- 2026-08-22 UTC - Task Reviewer - Independently reviewed the final analysis. Confirmed the flat-versus-split Figment comparison, evidence traceability, analysis-only scope, and synchronized acceptance verification. Approved the analysis as commit-ready.
- 2026-08-23 UTC - Copilot - Remediated the five open Copilot review findings for PR #2082 locally: reconciled the v3-before-#1980 policy, traced the masked JSON log boundary, added qualitative effort estimates, extended nested-field feasibility coverage, and asserted the concrete Figment error representation. Per user instruction, this local remediation does not commit, push, reply to, or resolve threads.

### PR #2082 Copilot Review Remediation Checklist

| Thread ID               | Finding                               | Local remediation                                                                            | Validation             | Publish             | Reply and resolution |
| ----------------------- | ------------------------------------- | -------------------------------------------------------------------------------------------- | ---------------------- | ------------------- | -------------------- |
| `PRRT_kwDOGp2yqc6beOKU` | Stale post-v3/v3-to-v4 language       | Reconciled this spec and the #1978 EPIC with the approved v3-before-#1980 policy.            | Pre-commit gate passed | Pending commit/push | Pending              |
| `PRRT_kwDOGp2yqc6beOKf` | Missing logged-JSON redaction trace   | Added source trace and enum redaction-before-JSON prototype evidence.                        | Pre-commit gate passed | Pending commit/push | Pending              |
| `PRRT_kwDOGp2yqc6beOKk` | Missing effort estimate               | Added qualitative estimates for the rejected flat TOML and deferred normalizer alternatives. | Pre-commit gate passed | Pending commit/push | Pending              |
| `PRRT_kwDOGp2yqc6beOKr` | Missing nested-field round trip       | Added adjacent-enum round-trip coverage for `network`, `tls_config`, and `access_tokens`.    | Pre-commit gate passed | Pending commit/push | Pending              |
| `PRRT_kwDOGp2yqc6beOKx` | Weak numeric-override error assertion | Both numeric override tests now match Figment `InvalidType(Map, "a sequence")`.              | Pre-commit gate passed | Pending commit/push | Pending              |

## Acceptance Criteria

- [x] AC3: Test-only feasibility experiments and results are recorded in E2.
- [x] AC4: Order semantics and lifecycle constraints are documented in E3.
- [x] AC5: List, singleton, private-mode, and UDP policy behavior is documented in E1–E2.
- [x] AC6: Identity compatibility, mapping, and normalization boundary are documented in E3.
- [x] AC7: Shared UDP behavior and future policy are documented in E1.
- [x] AC8: Lifecycle, dependencies, consumers, and estimate are documented in E4.
- [x] AC9: Redaction and observability constraints are documented in E1 and E4.
- [x] AC10: `analysis.md` gives the explicit rejection rationale.
- [x] AC11: `evidence.md` contains E1–E5.
- [x] AC12: The 2026-08-22 pre-commit gate passed `linter all`.
- [x] AC13: `cargo test -p torrust-tracker-configuration` passed 96 tests.
- [x] AC14: M1–M5 are recorded as complete below.
- [x] AC15: Acceptance criteria were re-reviewed on 2026-08-22.
- [x] AC16: Issue decision artifacts were updated.

## Verification Plan

### Automatic Checks

- `linter all`
- Focused `cargo test` commands for `torrust-tracker-configuration` and any new experiment/fixture modules
- Relevant serialization, Figment loading, and environment-override tests when a candidate representation is exercised
- Pre-push checks (when applicable)

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                         | Command/Steps                                                                                                                                                                                                    | Expected Result                                                                                                                                           | Status | Evidence                                                  |
| --- | -------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------- | ------ | --------------------------------------------------------- |
| M1  | Review current port-zero fixture | Compare `tests/common/configuration.rs` with configuration structs, bootstrap, containers, shared UDP services, registry, and redaction paths.                                                                   | Evidence explains role-local IDs, post-bind identities, shared policy behavior, and current compatibility constraints.                                    | DONE   | `evidence.md#e1-current-state-baseline`                   |
| M2  | Review candidate TOML files      | Parse and serialize interleaved entries for each viable form. Exercise unknown kinds, numeric environment overrides, omitted/empty lists, missing health entries, and duplicate singletons.                      | Each result records syntax, readability, round-trip behavior, defaulting, error quality, and compatibility with nested TLS/network/access-token settings. | DONE   | `evidence.md#e2-configuration-representation-feasibility` |
| M3  | Review normalization plan        | Trace a representative interleaved list through conceptual normalization, role-local ID allocation, container lookup, startup phases, registration, and metrics without changing production code.                | The analysis identifies one consistent normalization boundary and proves whether source list order affects startup or presentation only.                  | DONE   | `evidence.md#e3-runtime-and-identity-model`               |
| M4  | Review migration and transition  | Compare the recommended final-v3 form with the current split layout/default configs, environment overrides, docs, integration fixtures, #1490, and #1980. Define a canonical migration order and loading policy. | The impact inventory, compatibility policy, prerequisites, and implementation estimate are complete; unresolved constraints are explicit.                 | DONE   | `evidence.md#e4-migration-schema-lifecycle-and-security`  |
| M5  | Review final reports             | Check every conclusion in `analysis.md` against the linked record in `evidence.md`; confirm the recommendation does not include implementation work.                                                             | The decision record is complete, traceable, and limited to analysis plus a proposed follow-up scope when warranted.                                       | DONE   | `evidence.md#e5-report-review`                            |

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                            |
| ----- | ---------------------- | ------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `evidence.md#e1-current-state-baseline`                                                                             |
| AC2   | DONE                   | `analysis.md#candidate-representations`, `evidence.md#e2-configuration-representation-feasibility`                  |
| AC3   | DONE                   | `evidence.md#e2-configuration-representation-feasibility`                                                           |
| AC4   | DONE                   | `analysis.md#runtime-and-normalization-model`, `evidence.md#e3-runtime-and-identity-model`                          |
| AC5   | DONE                   | `analysis.md#feasibility-results`, E1–E2                                                                            |
| AC6   | DONE                   | `analysis.md#identity-ordering-and-migration`, `evidence.md#e3-runtime-and-identity-model`                          |
| AC7   | DONE                   | `analysis.md#current-state-baseline`, `evidence.md#e1-current-state-baseline`                                       |
| AC8   | DONE                   | `analysis.md#schema-lifecycle-security-and-compatibility`, `evidence.md#e4-migration-schema-lifecycle-and-security` |
| AC9   | DONE                   | E1 and E4                                                                                                           |
| AC10  | DONE                   | `analysis.md`, `evidence.md#e5-final-report-review`                                                                 |
| AC11  | DONE                   | `evidence.md#e1-current-state-baseline` through `evidence.md#e5-final-report-review`                                |
| AC12  | DONE                   | 2026-08-22 pre-commit gate (`linter all`)                                                                           |
| AC13  | DONE                   | Focused prototype tests (9 passed) and final pre-commit gate                                                        |
| AC14  | DONE                   | M1–M5 and E1–E5                                                                                                     |
| AC15  | DONE                   | 2026-08-22 acceptance review                                                                                        |
| AC16  | DONE                   | `ISSUE.md`, `analysis.md`, and `evidence.md`                                                                        |

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
- **Schema lifecycle ambiguity:** Selecting a different final v3 representation before #1980 would require an explicit transition, compatibility, or migration strategy because a versioned configuration loader accepts one schema shape at a time.
- **Secret exposure:** Nesting API configuration in an enum can bypass current redaction paths unless serialization/logging behavior is explicitly tested and coordinated with #1490.
- **Roadmap integration:** If approved, the configuration change must be a separately scoped sub-issue after #1490 and before #1980. It must resolve its schema shape before #1980 performs the final v3 consumer migration, avoiding a second migration of runtime consumers.

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
