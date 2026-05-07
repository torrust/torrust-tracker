# Issue #669 — Overhaul Clients (EPIC)

## Overview

This EPIC tracks the work to overhaul the three client/tool binaries that ship with the Torrust
Tracker: the **UDP Tracker client**, the **HTTP Tracker client**, and the **Tracker Checker**.
The long-term goal is to merge them into a single, polished **Tracker Client** CLI.

- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/669>

## Background

Three console commands were added to aid developers and sysadmins in testing and debugging
trackers:

- **HTTP Tracker Client** — sends `announce` and `scrape` requests to HTTP trackers and returns
  responses as JSON.
- **UDP Tracker Client** — sends `announce` and `scrape` requests to UDP trackers and returns
  responses as JSON.
- **Tracker Checker** — checks whether UDP trackers, HTTP trackers, and health-check endpoints
  are alive and responding correctly.

The initial implementations were quick prototypes: some parts were moved from test code to
production code without full coverage, parameters are hard-coded, and error handling is fragile.
This EPIC systematically improves each tool and eventually unifies them.

## Goals

- [ ] Overhaul the UDP Tracker client (see sub-issues below)
- [ ] Overhaul the HTTP Tracker client (see sub-issues below)
- [ ] Overhaul the Tracker Checker (see sub-issues below)
- [ ] Merge all clients into a single unified Tracker Client CLI

## Pending Sub-Issues

### UDP Tracker Client

| Issue                                                           | Title                                                        | Status |
| --------------------------------------------------------------- | ------------------------------------------------------------ | ------ |
| [#1533](https://github.com/torrust/torrust-tracker/issues/1533) | Add optional parameters with the rest of the announce params | Open   |
| [#671](https://github.com/torrust/torrust-tracker/issues/671)   | Print unrecognized responses                                 | Open   |
| [#1563](https://github.com/torrust/torrust-tracker/issues/1563) | Add option to show response in pretty JSON                   | Open   |

### HTTP Tracker Client

| Issue                                                           | Title                                                        | Status |
| --------------------------------------------------------------- | ------------------------------------------------------------ | ------ |
| [#1532](https://github.com/torrust/torrust-tracker/issues/1532) | Add optional parameters with the rest of the announce params | Open   |
| [#672](https://github.com/torrust/torrust-tracker/issues/672)   | Print unrecognized responses in JSON                         | Open   |
| [#1561](https://github.com/torrust/torrust-tracker/issues/1561) | Duplicate URL suffix `announce` when already in tracker URL  | Open   |
| [#1562](https://github.com/torrust/torrust-tracker/issues/1562) | Add option to show response in pretty JSON                   | Open   |

### Tracker Checker

| Issue                                                           | Title                                                               | Status |
| --------------------------------------------------------------- | ------------------------------------------------------------------- | ------ |
| [#1042](https://github.com/torrust/torrust-tracker/issues/1042) | (HTTP) Improve error message when JSON config is not well-formatted | Open   |
| [#1178](https://github.com/torrust/torrust-tracker/issues/1178) | (UDP) Add command to monitor uptime                                 | Open   |

### Unified Tracker Client

| Issue                                                           | Title                                       | Status |
| --------------------------------------------------------------- | ------------------------------------------- | ------ |
| [#1564](https://github.com/torrust/torrust-tracker/issues/1564) | Change the default `PeerId` used in clients | Open   |

## Already Closed Sub-Issues

### UDP Tracker Client

- [#670](https://github.com/torrust/torrust-tracker/issues/670) — Closed

### Tracker Checker

- [#674](https://github.com/torrust/torrust-tracker/issues/674) — Closed
- [#675](https://github.com/torrust/torrust-tracker/issues/675) — Closed
- [#677](https://github.com/torrust/torrust-tracker/issues/677) — Closed (and its sub-issues #682, #681, #679, #680, #678)
- [#683](https://github.com/torrust/torrust-tracker/issues/683) — Closed
- [#676](https://github.com/torrust/torrust-tracker/issues/676) — Closed
- [#1040](https://github.com/torrust/torrust-tracker/issues/1040) — Closed
- [#767](https://github.com/torrust/torrust-tracker/issues/767) — Closed
- [#673](https://github.com/torrust/torrust-tracker/issues/673) — Closed

## Recommended Implementation Order

The list order in the EPIC is the recommended order of implementation. In broad terms:

1. Add missing announce parameters to both UDP and HTTP clients (#1533, #1532)
2. Fix panics on unrecognized responses in both clients (#671, #672)
3. Fix the HTTP client URL duplication bug (#1561)
4. Add pretty-print JSON output to both clients (#1562, #1563)
5. Fix Tracker Checker error messages (#1042)
6. Add uptime monitoring to Tracker Checker (#1178)
7. Fix the default `PeerId` in all clients (#1564)
8. Merge the three tools into a single unified Tracker Client CLI

## Implementation Specs

Each pending sub-issue has a dedicated spec document in this folder:

- [1532-http-tracker-client-add-optional-announce-params.md](1532-http-tracker-client-add-optional-announce-params.md)
- [1533-udp-tracker-client-add-optional-announce-params.md](1533-udp-tracker-client-add-optional-announce-params.md)
- [671-udp-tracker-client-print-unrecognized-responses.md](671-udp-tracker-client-print-unrecognized-responses.md)
- [672-http-tracker-client-print-unrecognized-responses.md](672-http-tracker-client-print-unrecognized-responses.md)
- [1562-http-tracker-client-add-option-show-response-pretty-json.md](1562-http-tracker-client-add-option-show-response-pretty-json.md)
- [1563-udp-tracker-client-add-option-show-response-pretty-json.md](1563-udp-tracker-client-add-option-show-response-pretty-json.md)

## References

- EPIC issue: <https://github.com/torrust/torrust-tracker/issues/669>
- Discussion: <https://github.com/torrust/torrust-tracker/discussions/660>
- HTTP tracker client source: `console/tracker-client/src/console/clients/http/`
- UDP tracker client source: `console/tracker-client/src/console/clients/udp/`
- Tracker Checker source: `console/tracker-client/src/console/clients/checker/`
- `tracker-client` package: `packages/tracker-client/`
