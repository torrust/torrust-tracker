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

## ADR Lifecycle

An ADR merged into `develop` or `main` is **accepted**. The PR review process is the acceptance
gate — no explicit `- Status: Accepted` or `- Status: Proposed` header is needed or written.

A `- Status:` header appears in an ADR file only for special terminal states, for example:

- `- Status: Superseded by [ADR link]` — this decision has been replaced by a newer ADR.

Additional states (e.g. `Deprecated`) may be introduced as needed.
