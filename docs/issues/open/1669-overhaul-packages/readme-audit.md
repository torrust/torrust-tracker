# README Audit

Point-in-time audit of README quality across all workspace packages and console
tools. Generated manually on 2026-05-18 as part of SI-01 (baseline analysis).

## Quality scale

| Rating      | Criteria                                                                                       |
| ----------- | ---------------------------------------------------------------------------------------------- |
| **good**    | Meaningful sections (purpose, usage, badges, examples); gives a reader enough to get started.  |
| **minimal** | Title, one-sentence description, and at most a `## Documentation` link; mostly placeholder.    |
| **stub**    | Only heading + one-liner + a `## Documentation` link (~11 lines); essentially a template copy. |

## Workspace packages (`packages/`)

| Package directory                 | Crate name                                        | Lines | Rating  | Notes                                                        |
| --------------------------------- | ------------------------------------------------- | ----- | ------- | ------------------------------------------------------------ |
| `axum-health-check-api-server`    | `torrust-axum-health-check-api-server`            | 49    | minimal | Has purpose and port info; no usage examples                 |
| `axum-http-tracker-server`        | `torrust-axum-http-tracker-server`                | 11    | stub    | Template only                                                |
| `axum-rest-tracker-api-server`    | `torrust-axum-rest-tracker-api-server`            | 11    | stub    | Template only                                                |
| `axum-server`                     | `torrust-axum-server`                             | 11    | stub    | Template only                                                |
| `clock`                           | `torrust-tracker-clock`                           | 11    | stub    | Template only                                                |
| `configuration`                   | `torrust-tracker-configuration`                   | 11    | stub    | Template only                                                |
| `events`                          | `torrust-tracker-events`                          | 11    | stub    | Template only                                                |
| `http-protocol`                   | `bittorrent-http-tracker-protocol`                | 11    | stub    | Template only                                                |
| `http-tracker-core`               | `bittorrent-http-tracker-core`                    | 15    | minimal | Explains when to use vs. when not to; minimal depth          |
| `located-error`                   | `torrust-tracker-located-error`                   | 11    | stub    | Template only                                                |
| `metrics`                         | `torrust-tracker-metrics`                         | 210   | good    | Comprehensive — overview, types, usage, examples             |
| `peer-id`                         | `bittorrent-peer-id`                              | 38    | minimal | Origin story + maintenance note; no usage examples           |
| `primitives`                      | `torrust-tracker-primitives`                      | 11    | stub    | Template only                                                |
| `rest-tracker-api-client`         | `torrust-rest-tracker-api-client`                 | 23    | minimal | Has license section; no usage examples                       |
| `rest-tracker-api-core`           | `torrust-rest-tracker-api-core`                   | 11    | stub    | **Wrong title** — says "BitTorrent UDP Tracker Core library" |
| `server-lib`                      | `torrust-server-lib`                              | 11    | stub    | Template only                                                |
| `swarm-coordination-registry`     | `torrust-tracker-swarm-coordination-registry`     | 22    | minimal | **Wrong title** — says "Torrust Tracker Torrent Repository"  |
| `test-helpers`                    | `torrust-tracker-test-helpers`                    | 11    | stub    | **Wrong title** — says "Torrust Tracker Configuration"       |
| `torrent-repository-benchmarking` | `torrust-tracker-torrent-repository-benchmarking` | 32    | minimal | Has benchmarking section; no run instructions beyond basic   |
| `tracker-client`                  | `bittorrent-tracker-client`                       | 25    | minimal | Has WIP disclaimer; no usage examples                        |
| `tracker-core`                    | `bittorrent-tracker-core`                         | 39    | minimal | Has purpose and context; no usage examples                   |
| `udp-protocol`                    | `bittorrent-udp-tracker-protocol`                 | 38    | minimal | Has purpose section; no usage examples                       |
| `udp-tracker-core`                | `bittorrent-udp-tracker-core`                     | 15    | minimal | Explains when to use; minimal depth                          |
| `udp-tracker-server`              | `torrust-udp-tracker-server`                      | 11    | stub    | Template only                                                |

## Console tools (`console/`)

| Directory        | Crate name                  | Lines | Rating | Notes                                       |
| ---------------- | --------------------------- | ----- | ------ | ------------------------------------------- |
| `tracker-client` | `bittorrent-tracker-client` | 204   | good   | Comprehensive — purpose, commands, examples |

## Community contributions (`contrib/`)

| Directory | Crate name                        | Lines | Rating | Notes                                     |
| --------- | --------------------------------- | ----- | ------ | ----------------------------------------- |
| `bencode` | `torrust-tracker-contrib-bencode` | 5     | stub   | Title + one-liner only; no usage examples |

## Summary

| Rating      | Count |
| ----------- | ----- |
| **good**    | 2     |
| **minimal** | 9     |
| **stub**    | 16    |

Most workspace packages have stub or minimal READMEs — they were likely cloned from a
template without being updated. The three packages with wrong titles need to be corrected:

| Package directory             | Current (wrong) title               | Expected title                                |
| ----------------------------- | ----------------------------------- | --------------------------------------------- |
| `rest-tracker-api-core`       | BitTorrent UDP Tracker Core library | Torrust REST Tracker API Core (or equivalent) |
| `swarm-coordination-registry` | Torrust Tracker Torrent Repository  | Torrust Tracker Swarm Coordination Registry   |
| `test-helpers`                | Torrust Tracker Configuration       | Torrust Tracker Test Helpers (or equivalent)  |

Improving READMEs to at least **minimal** status across all workspace packages is a
low-effort, high-value documentation task that could be bundled into a dedicated subissue.
