---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - docs/index.md
    - docs/adrs/README.md
    - docs/adrs/20260527175600_keep_protocol_and_domain_types_decoupled.md
---

# ADR Index

| ADR                                                                                     | Date       | Title                                                  | Short Description                                                                                                                                                                                                                          |
| --------------------------------------------------------------------------------------- | ---------- | ------------------------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| [20240227164834](20240227164834_use_plural_for_modules_containing_collections.md)       | 2024-02-27 | Use plural for modules containing collections          | Module names should use plural when they contain multiple types with the same responsibility (e.g. `requests/`, `responses/`).                                                                                                             |
| [20260420200013](20260420200013_adopt_custom_github_copilot_aligned_agent_framework.md) | 2026-04-20 | Adopt a custom, GitHub-Copilot-aligned agent framework | Use AGENTS.md, Agent Skills, and Custom Agent profiles instead of third-party agent frameworks.                                                                                                                                            |
| [20260429000000](20260429000000_keep_database_as_aggregate_supertrait.md)               | 2026-04-29 | Keep `Database` as an aggregate supertrait             | Split the 18-method monolithic `Database` trait into four narrow context traits (`SchemaMigrator`, `TorrentMetricsStore`, `WhitelistStore`, `AuthKeyStore`) while keeping `Database` as an empty aggregate supertrait with a blanket impl. |
| [20260512102000](20260512102000_define_tracker_client_peer_id_convention.md)            | 2026-05-12 | Define tracker-client peer ID convention               | Adopt `-RC3000-` Azureus-style defaults for tracker-client, use a once-per-process randomized production suffix, and keep deterministic `RC` test fixtures without cross-package constant coupling.                                        |
| [20260519000000](20260519000000_define_global_cli_output_contract.md)                   | 2026-05-19 | Define the global CLI output contract                  | All first-party binaries use JSON on stdout (result data) and stderr (NDJSON diagnostics/progress). No plain text. TTY refusal for stdout-result-data commands. Exit codes 0/1/2. Prescriptive; migration is progressive.                  |
| [20260527175600](20260527175600_keep_protocol_and_domain_types_decoupled.md)            | 2026-05-27 | Keep protocol and domain types decoupled               | Keep protocol-local and domain-local value types (for example `NumberOfBytes`) and map at boundaries so HTTP/UDP wire evolution does not force domain-wide refactors and domain changes do not force protocol redesign.                    |
| [20260603000000](20260603000000_keep_unit_tests_inside_container_build.md)              | 2026-06-03 | Keep unit tests inside the container build process     | Unit tests must run inside the Containerfile build (not on the GHA host) because only the container build environment proves the binary works on the actual target infrastructure (Debian trixie, distroless runtime, specific glibc).     |
| [20260617093046](20260617093046_reject_wildcard_external_ip.md)                         | 2026-06-17 | Reject wildcard IPs as invalid `external_ip` values    | Reject `0.0.0.0`/`::` in `external_ip` config at startup, change default to `None`. Fail fast on invalid config.                                                                                                                           |
| [20260620000000](20260620000000_add_ipv6_v6only_config_option.md)                       | 2026-06-20 | Add `ipv6_v6only` config option for separate sockets   | Add `ipv6_v6only` boolean flag to `UdpTracker` and `HttpTracker` configs, defaulting to `false` (dual-stack), so operators can opt into separate IPv4/IPv6 sockets.                                                                        |
| [20260623200526](20260623200526_adopt_contract-first_architecture_for_rest_api.md)      | 2026-06-23 | Adopt a contract-first architecture for the REST API   | Structure the REST API into four layers: protocol contract, application/use-case, runtime adapter, and transport adapter. Enables a future tracker-agnostic REST API standard.                                                             |
| [20260629000000](20260629000000_adopt_independent_package_versioning.md)                     | 2026-06-29 | Adopt independent package versioning                   | All workspace packages version independently. Path dependencies guarantee compatibility, so linked versions are unnecessary. Enables per-package publishing and aligns with EPIC #1669 extraction goals.                                  |

## ADR Lifecycle

An ADR merged into `develop` or `main` is **accepted**. The PR review process is the acceptance
gate — no explicit `- Status: Accepted` or `- Status: Proposed` header is needed or written.

A `- Status:` header appears in an ADR file only for special terminal states, for example:

- `- Status: Superseded by [ADR link]` — this decision has been replaced by a newer ADR.

Additional states (e.g. `Deprecated`) may be introduced as needed.
