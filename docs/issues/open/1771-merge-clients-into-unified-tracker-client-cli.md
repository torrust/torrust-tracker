---
doc-type: issue
issue-type: feature
status: in_progress
priority: p2
github-issue: 1771
spec-path: docs/issues/open/1771-merge-clients-into-unified-tracker-client-cli.md
branch: "1771-merge-clients-into-unified-tracker-client-cli"
related-pr: 1772
last-updated-utc: 2026-05-13 10:37
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - console/tracker-client/src/bin/http_tracker_client.rs
    - console/tracker-client/src/bin/udp_tracker_client.rs
    - console/tracker-client/src/bin/tracker_checker.rs
    - packages/tracker-client/
    - console/tracker-client/
---

<!-- skill-link: create-issue -->

# Issue #1771 — Merge all tracker client tools into a single unified `tracker_client` CLI

## Goal

Replace the three separate client binaries (`http_tracker_client`, `udp_tracker_client`,
`tracker_checker`) with a single `tracker_client` binary that supports all their use-cases
under a unified command-line interface.

## Background

Three binaries currently ship with the tracker to support testing and development workflows:

- **`http_tracker_client`** — sends `announce` and `scrape` requests to HTTP trackers, returns
  JSON.
- **`udp_tracker_client`** — sends `announce` and `scrape` requests to UDP trackers, returns
  JSON.
- **`tracker_checker`** — checks whether UDP trackers, HTTP trackers, and health-check endpoints
  are alive and responding correctly.

The domain library code has already been extracted into the `packages/tracker-client` package
(see issue #1067). The remaining step is to unify the three binary entry points into a single
CLI and retire the old per-protocol binaries.

The idea of merging these tools was first proposed in
[discussion #660](https://github.com/torrust/torrust-tracker/discussions/660) and tracked as
the final goal of EPIC [#669](https://github.com/torrust/torrust-tracker/issues/669).

### Design decisions

**CLI shape — Option B: explicit protocol subcommand.** The scope of this issue is a mechanical
port: the three independent binaries are moved into a single `tracker_client` binary with
explicit protocol subcommands. No behaviour changes are introduced beyond the unification itself.

```sh
tracker_client http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
tracker_client udp  announce udp://127.0.0.1:6969  9c38422213e30bff212b30c360d26f9a02136422
tracker_client check -- --config-path ./tracker_checker.json
```

An alternative CLI shape was proposed in discussion #660 by da2ce7: auto-detect the protocol
from the URL scheme (`udp://` → UDP, `http://`/`https://` → HTTP), reducing the required
subcommand depth:

```sh
tracker_client announce udp://127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422
tracker_client scrape  http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
```

This idea is **out of scope here** — the goal of this issue is the simplest possible unification
(a direct port, not a redesign). The auto-detection approach will be reconsidered in a follow-up
issue once the single binary exists and all three use-cases are verified.

Potential future additive UX (follow-up issue, not this one):

```sh
tracker_client announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422
tracker_client announce udp://127.0.0.1:6969  9c38422213e30bff212b30c360d26f9a02136422
tracker_client check -- --config-path ./tracker_checker.json
```

In that model, top-level `announce` and `scrape` would behave as optional convenience commands
that dispatch internally to `http` or `udp` based on URL scheme. Explicit protocol subcommands
would remain supported.

#### CLI shape options: pros and cons

|          | **Option A — URL-scheme auto-detection**                                                                        | **Option B — Explicit protocol subcommand**                                                                                 |
| -------- | --------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| **Pros** | Shorter commands; matches how tracker URLs naturally appear in torrent files and tracker lists                  | Clear code separation per protocol; `--help` reveals all subcommands; error messages are unambiguous                        |
|          | No need to remember whether to type `http` or `udp` before the action                                           | Easier to extend with protocol-specific flags without polluting a shared namespace                                          |
|          | Feels more ergonomic for interactive use                                                                        | Simple mechanical port — minimal risk for this issue                                                                        |
| **Cons** | Requires URL parsing before dispatch; edge cases (e.g. custom ports, missing scheme) must be handled explicitly | More verbose at the command line; users must always specify the protocol even when the URL already carries that information |
|          | Protocol-specific flags can collide in a flat namespace                                                         | Slightly redundant: the URL scheme and the subcommand both encode the protocol                                              |

**Output format — JSON default.** `--format=json` is the default output mode for all
subcommands; `--format=text` produces human-friendly output. The flag must be consistent across
all subcommands.

**Legacy binary strategy — deprecate in-place for approximately one year.** The three old
binaries (`http_tracker_client`, `udp_tracker_client`, `tracker_checker`) are widely referenced
in the Torrust organization website, blog posts, and external documentation. To allow time for
those references to be updated, the old binaries will be kept as-is — no new features will be
added to them — and will print a deprecation warning on startup directing users to
`tracker_client`. They will be removed no earlier than approximately one year after `tracker_client`
is released and documented. The removal milestone should be tracked in a follow-up issue.

**Checker subcommand name — `check`.** Consistent with the verb pattern used by `announce` and
`scrape`, and moves from the old binary noun (`tracker_checker`) to an imperative verb (`check`).

**REST API client:** extending the CLI with a `tracker_client api` subcommand to interact
with the Torrust Tracker management REST API was mentioned in discussion #660. This is out of scope
for this issue but should be kept in mind for the CLI shape.

## Scope

### In Scope

- Define the final CLI interface (command/subcommand hierarchy, argument names, defaults).
- Implement a single `tracker_client` binary entry point in `console/tracker-client/src/bin/`.
- Wire all three existing use-cases (HTTP announce/scrape, UDP announce/scrape, checker) into
  the new CLI.
- Unified `--format=<json|text>` flag shared across all subcommands, with JSON as the default.
- Add deprecation notices to the three legacy binaries (print warning on startup, no new
  features). Track removal (≥ 1 year after release) in a follow-up issue.
- Update in-repo docs and skills that reference the old binary names.

### Out of Scope

- Implementation of missing announce parameters (#1532, #1533) — those are tracked separately.
- REST API console client — deferred to a future issue.
- Top-level `announce`/`scrape` convenience commands that auto-dispatch by URL scheme
  (future additive UX).
- Changes to the `packages/tracker-client` library itself (only the CLI entrypoint is in scope
  unless structural changes are required for the CLI unification).

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status      | Task                                           | Notes / Expected Output                                                                             |
| --- | ----------- | ---------------------------------------------- | --------------------------------------------------------------------------------------------------- |
| T1  | IN_PROGRESS | Implement unified `tracker_client` entry point | New `console/tracker-client/src/bin/tracker_client.rs` with `http`, `udp`, and `check` subcommands. |
| T2  | TODO        | Add unified `--format=<json\|text>` flag       | JSON default; flag works identically across all subcommands.                                        |
| T3  | TODO        | Add deprecation notices to legacy binaries     | Each old binary prints a deprecation warning on startup; no new features added to them.             |
| T4  | TODO        | Update in-repo docs, skills, and CI references | All in-repo references to old binary names updated or annotated.                                    |
| T5  | TODO        | Validate gates and regression                  | `linter all` and relevant tests pass; existing tests ported or replaced.                            |
| T6  | TODO        | Run manual verification scenarios              | Execute the local-tracker manual test matrix and record status/evidence for every scenario.         |

## Manual Verification Plan (Local Tracker)

The refactor must be manually validated against a locally running tracker to ensure no behavior
regression across protocol commands.

### Test Setup

Terminal A (start local tracker):

```sh
mkdir -p ./storage/tracker/etc/
cp ./share/default/config/tracker.development.sqlite3.toml ./storage/tracker/etc/tracker.toml
TORRUST_TRACKER_CONFIG_TOML_PATH="./storage/tracker/etc/tracker.toml" cargo run
```

Terminal B (run client scenarios against local tracker):

Use this sample info hash in all announce/scrape tests:

```text
9c38422213e30bff212b30c360d26f9a02136422
```

### Scenario Matrix and Progress Tracking

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                     | Command                                                                                                                                                                                                  | Expected Result                                                       | Status | Evidence                    |
| --- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ------ | --------------------------- |
| M1  | HTTP announce (JSON default) | `cargo run --bin tracker_client http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422`                                                                                            | Command exits 0 and prints valid JSON announce response               | TODO   | {command output / log path} |
| M2  | HTTP scrape (JSON default)   | `cargo run --bin tracker_client http scrape http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422`                                                                                              | Command exits 0 and prints valid JSON scrape response                 | TODO   | {command output / log path} |
| M3  | UDP announce (JSON default)  | `cargo run --bin tracker_client udp announce udp://127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422`                                                                                              | Command exits 0 and prints valid JSON announce response               | TODO   | {command output / log path} |
| M4  | UDP scrape (JSON default)    | `cargo run --bin tracker_client udp scrape udp://127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422`                                                                                                | Command exits 0 and prints valid JSON scrape response                 | TODO   | {command output / log path} |
| M5  | Checker command              | `TORRUST_CHECKER_CONFIG='{"udp_trackers":["127.0.0.1:6969"],"http_trackers":["http://127.0.0.1:7070"],"health_checks":["http://127.0.0.1:1212/api/health_check"]}' cargo run --bin tracker_client check` | Command exits 0 and reports successful UDP/HTTP/health checks in JSON | TODO   | {command output / log path} |
| M6  | HTTP announce (text format)  | `cargo run --bin tracker_client http announce --format=text http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422`                                                                              | Command exits 0 and prints human-readable response                    | TODO   | {command output / log path} |
| M7  | UDP scrape (text format)     | `cargo run --bin tracker_client udp scrape --format=text udp://127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422`                                                                                  | Command exits 0 and prints human-readable response                    | TODO   | {command output / log path} |

Notes:

- Update the `Status` and `Evidence` columns as each scenario is executed.
- If any scenario fails, capture the failing output and add a short diagnosis entry in the
  progress log before continuing.

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [x] (Optional, recommended for complex issues) Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-05-13 00:00 UTC - Copilot - Created draft spec from discussion #660 and EPIC #669.
- 2026-05-13 10:00 UTC - Copilot - Recorded design decisions: Option B CLI shape, JSON default output, ~1-year deprecation window for legacy binaries, `check` subcommand name.
- 2026-05-13 10:10 UTC - Copilot - Added future additive UX note for top-level `announce`/`scrape` aliases that auto-dispatch by URL scheme; kept out of scope for this issue.
- 2026-05-13 10:20 UTC - Copilot - Added explicit acceptance criterion to prevent scope drift: top-level `announce`/`scrape` auto-dispatch aliases are not part of this issue.
- 2026-05-13 10:30 UTC - Copilot - Added local-tracker manual verification plan with concrete commands and a scenario status matrix.
- 2026-05-13 10:35 UTC - Copilot - Opened GitHub issue #1771 and moved spec from drafts to open.
- 2026-05-13 10:36 UTC - User - Merged upstream spec-only PR #1772 into `develop`.
- 2026-05-13 10:37 UTC - Copilot - Created implementation branch `1771-merge-clients-into-unified-tracker-client-cli` from updated `develop` and started T1.

## Acceptance Criteria

- [ ] AC1: A single `tracker_client` binary exists with `http announce`, `http scrape`,
      `udp announce`, `udp scrape`, and `check` subcommands, all behaving equivalently to the
      current per-protocol binaries.
- [ ] AC2: `--format=json` (default) produces valid JSON on stdout for all subcommands.
- [ ] AC3: `--format=text` produces human-readable output for all subcommands.
- [ ] AC4: Each legacy binary (`http_tracker_client`, `udp_tracker_client`, `tracker_checker`)
      prints a deprecation notice on startup directing users to `tracker_client`; their existing
      behaviour is otherwise unchanged.
- [ ] AC5: A follow-up issue for removing the legacy binaries (no earlier than ~1 year after
      `tracker_client` ships) is linked from this spec or the EPIC.
- [ ] AC6: In-repo docs and skill files that reference old binary names are updated.
- [ ] AC7: Top-level `announce`/`scrape` auto-dispatch aliases are not implemented in this
      issue (kept for follow-up to prevent scope drift).
- [ ] AC8: `linter all` exits with code `0`.
- [ ] AC9: Relevant tests pass.
- [ ] AC10: Manual verification matrix scenarios (M1-M7) are executed against a local tracker,
      with status and evidence recorded for each.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                            |
| ----- | ---------------------- | ------------------------------------------------------------------- |
| AC1   | TODO                   | {test/log/PR link}                                                  |
| AC2   | TODO                   | {test/log/PR link}                                                  |
| AC3   | TODO                   | {test/log/PR link}                                                  |
| AC4   | TODO                   | {test/log/PR link}                                                  |
| AC5   | TODO                   | {follow-up issue link}                                              |
| AC6   | TODO                   | {test/log/PR link}                                                  |
| AC7   | TODO                   | {CLI help/output showing only explicit protocol path in this issue} |
| AC8   | TODO                   | {test/log/PR link}                                                  |
| AC9   | TODO                   | {test/log/PR link}                                                  |
| AC10  | TODO                   | {manual verification matrix with statuses and evidence completed}   |

## Risks and Trade-offs

- **External documentation references**: the old binary names appear in the Torrust website,
  blog posts, and other organization-wide materials that cannot be updated in a single PR.
  Mitigation: keep the legacy binaries alive for approximately one year after `tracker_client`
  ships; add startup deprecation warnings; track removal in a dedicated follow-up issue.
- **Inconsistency across subcommands**: if output format handling is not centralized, each
  subcommand may behave differently.
  Mitigation: implement a shared output formatter before wiring subcommands.
- **Scope creep**: the Tracker Checker has a richer config-file-driven interface; merging
  it may introduce complexity into the shared CLI argument parser.
  Mitigation: keep the checker as a self-contained subcommand; do not restructure its
  internals in this issue.

## References

- Parent EPIC: <https://github.com/torrust/torrust-tracker/issues/669>
- GitHub issue: <https://github.com/torrust/torrust-tracker/issues/1771>
- Spec: [docs/issues/open/669-overhaul-clients.md](../open/669-overhaul-clients.md)
- Original discussion: <https://github.com/torrust/torrust-tracker/discussions/660>
- HTTP Tracker Client source: `console/tracker-client/src/console/clients/http/`
- UDP Tracker Client source: `console/tracker-client/src/console/clients/udp/`
- Tracker Checker source: `console/tracker-client/src/console/clients/checker/`
- `tracker-client` package: `packages/tracker-client/`
- Related: #1532, #1533, #1561, #1562, #1563, #1564
