---
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/issues/open/1669-overhaul-packages/EPIC.md
    - packages/
---

# README Audit

Point-in-time audit of README quality across the tracker packages. First generated manually on
2026-05-18 as part of SI-01 (baseline analysis); refreshed on 2026-10-06 for issue
[#2446](https://github.com/torrust/torrust-tracker/issues/2446).

The audited set is the 25 tracker packages listed in the EPIC #1669 Package Inventory: the root
`torrust-tracker` crate, `console/tracker-client` and the 23 `packages/*` members. The
`contrib/dev-tools/` workspace members are internal tools and are not audited. Line counts are
`wc -l` of each `README.md`.

## Quality scale

| Rating      | Criteria                                                                                       |
| ----------- | ---------------------------------------------------------------------------------------------- |
| **good**    | Meaningful sections (purpose, usage, badges, examples); gives a reader enough to get started.  |
| **minimal** | Title, one-sentence description, and at most a `## Documentation` link; mostly placeholder.    |
| **stub**    | Only heading + one-liner + a `## Documentation` link (~11 lines); essentially a template copy. |
| **missing** | The package has no `README.md`.                                                                |

## Root crate

| Directory         | Crate name        | Lines | Rating | Notes                                                                 |
| ----------------- | ----------------- | ----- | ------ | --------------------------------------------------------------------- |
| (repository root) | `torrust-tracker` | 318   | good   | Comprehensive: features, demo, roadmap, BEPs, architecture, getting started |

## Workspace packages (`packages/`)

| Package directory                 | Crate name                                        | Lines | Rating  | Notes                                                                                     |
| --------------------------------- | ------------------------------------------------- | ----- | ------- | ----------------------------------------------------------------------------------------- |
| `axum-health-check-api-server`    | `torrust-tracker-axum-health-check-api-server`    | 49    | minimal | Purpose, port and example response; **wrong title** ("Torrust Axum HTTP Tracker")         |
| `axum-http-server`                | `torrust-tracker-axum-http-server`                | 37    | minimal | One-line purpose plus testing and coverage instructions                                  |
| `axum-rest-api-server`            | `torrust-tracker-axum-rest-api-server`            | 11    | stub    | Template only                                                                             |
| `axum-server`                     | `torrust-tracker-axum-server`                     | 29    | minimal | Purpose and design notes; notes still name the old `TslConfig` type                      |
| `configuration`                   | `torrust-tracker-configuration`                   | 13    | stub    | Template plus a link to the v2-to-v3 migration guide                                     |
| `e2e-tools`                       | `torrust-tracker-e2e-tools`                       | 26    | good    | Purpose, binary list and usage examples                                                  |
| `events`                          | `torrust-tracker-events`                          | 11    | stub    | Template only                                                                             |
| `http-core`                       | `torrust-tracker-http-core`                       | 15    | minimal | Explains when to use vs. when not to; minimal depth                                      |
| `http-protocol`                   | `torrust-tracker-http-protocol`                   | 11    | stub    | Template only                                                                             |
| `persistence-benchmark`           | `torrust-tracker-persistence-benchmark`           | 18    | good    | Purpose and usage examples per database driver                                           |
| `primitives`                      | `torrust-tracker-primitives`                      | 11    | stub    | Template only                                                                             |
| `rest-api-application`            | `torrust-tracker-rest-api-application`            | 11    | stub    | Template only                                                                             |
| `rest-api-client`                 | `torrust-tracker-rest-api-client`                 | 23    | minimal | Has license section; no usage examples                                                   |
| `rest-api-protocol`               | `torrust-tracker-rest-api-protocol`               | 11    | stub    | Template only                                                                             |
| `rest-api-runtime-adapter`        | `torrust-tracker-rest-api-runtime-adapter`        | 11    | stub    | Template only                                                                             |
| `swarm-coordination-registry`     | `torrust-tracker-swarm-coordination-registry`     | 22    | minimal | **Wrong title** ("Torrust Tracker Torrent Repository")                                    |
| `test-helpers`                    | `torrust-tracker-test-helpers`                    | 11    | stub    | **Wrong title** ("Torrust Tracker Configuration")                                         |
| `torrent-repository-benchmarking` | `torrust-tracker-torrent-repository-benchmarking` | 32    | minimal | Has a benchmarking section; its `cargo bench -p` command names a package that no longer exists |
| `tracker-client`                  | `torrust-tracker-client-lib`                      | 25    | minimal | Has WIP disclaimer; no usage examples                                                    |
| `tracker-core`                    | `torrust-tracker-core`                            | 39    | minimal | Has purpose and context; no usage examples                                               |
| `udp-core`                        | `torrust-tracker-udp-core`                        | 19    | minimal | Explains when to use; links benchmarking notes and ADRs                                  |
| `udp-protocol`                    | `torrust-tracker-udp-protocol`                    | 38    | minimal | Has purpose and origin sections; no usage examples                                       |
| `udp-server`                      | `torrust-tracker-udp-server`                      | 16    | minimal | Template plus links to the package ADRs                                                  |

## Console tools (`console/`)

| Directory        | Crate name               | Lines | Rating | Notes                                       |
| ---------------- | ------------------------ | ----- | ------ | ------------------------------------------- |
| `tracker-client` | `torrust-tracker-client` | 204   | good   | Comprehensive — purpose, commands, examples |

## Summary

| Rating      | Count |
| ----------- | ----- |
| **good**    | 4     |
| **minimal** | 12    |
| **stub**    | 9     |
| **missing** | 0     |
| **Total**   | 25    |

Since the 2026-05-18 audit:

- Removed rows for packages no longer in the workspace: `clock`, `located-error`, `metrics`,
  `server-lib` (extracted to standalone repositories), `peer-id` and `contrib/bencode` (moved to
  `torrust/torrust-bittorrent`), and `rest-tracker-api-core` (removed by #1938).
- Added rows for the root crate, `e2e-tools`, `persistence-benchmark`, `rest-api-application`,
  `rest-api-protocol` and `rest-api-runtime-adapter`.
- Updated folder and crate names to the SI-11 and SI-29 renames.
- Re-rated `axum-http-server`, `axum-server` and `udp-server` from stub to minimal.
- Corrected the Summary: the 2026-05-18 table (2 good, 9 minimal, 16 stub = 27) did not match its
  26 rows (2, 10, 14).

Most tracker packages still have stub or minimal READMEs. Fourteen of them link
`docs.rs/crate/torrust-tracker/latest` (the root crate's documentation) instead of their own
crate's page. Three titles are wrong:

| Package directory              | Current (wrong) title              | Expected title                              |
| ------------------------------ | ---------------------------------- | ------------------------------------------- |
| `axum-health-check-api-server` | Torrust Axum HTTP Tracker          | Torrust Tracker Health Check API            |
| `swarm-coordination-registry`  | Torrust Tracker Torrent Repository | Torrust Tracker Swarm Coordination Registry |
| `test-helpers`                 | Torrust Tracker Configuration      | Torrust Tracker Test Helpers                |

Fixing these is in scope for the "Update all package READMEs" draft under EPIC #1669.
