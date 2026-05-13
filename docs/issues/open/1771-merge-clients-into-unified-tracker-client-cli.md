---
doc-type: issue
issue-type: feature
status: done
priority: p2
github-issue: 1771
spec-path: docs/issues/open/1771-merge-clients-into-unified-tracker-client-cli.md
branch: "1771-merge-clients-into-unified-tracker-client-cli"
related-pr: 1772
last-updated-utc: 2026-05-13 15:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - console/tracker-client/src/bin/http_tracker_client.rs
    - console/tracker-client/src/bin/udp_tracker_client.rs
    - console/tracker-client/src/bin/tracker_checker.rs
    - packages/tracker-client/
    - console/tracker-client/
    - console/tracker-client/src/console/clients/unified/mod.rs
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

**`unified/` module structure — flat files, no per-action nesting.** The sub-modules
`http.rs`, `udp.rs`, and `check.rs` are kept as flat single files rather than split into
per-action nested directories (e.g. `http/announce.rs`, `http/scrape.rs`). Reasons:

- `unified/` is a migration scaffold planned for cleanup in issue #1775; adding nested
  directories now would introduce churn for code that will be restructured again during that
  cleanup.
- Current file sizes are within the normal single-responsibility range (`http.rs` ~366 lines,
  `udp.rs` ~231 lines, `check.rs` ~199 lines).
- Nesting by subcommand should be revisited when #1775 flattens `unified/` into the final
  module structure.

See: `console/tracker-client/src/console/clients/unified/mod.rs`

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

## Implementation Strategy

**Progressive copy-and-port approach:**

1. The new `tracker_client` binary is built by **copying command handler code** from the old
   binaries into the new unified binary, one command at a time.
2. After each command is copied, it is tested independently in the new binary to verify behavior
   parity with the old implementation.
3. Test code is also ported to use the new binary, ensuring no behavior regression.
4. The old binary code is marked as deprecated and **frozen — never modified, never called from
   new code**. This ensures a clean separation and avoids bugs from dual maintenance.
5. After approximately one year (when the migration is complete and users have migrated), the old
   binaries are deleted in a follow-up issue.

**Key principle:** The old code is a source for copying, not a runtime dependency. The new binary
must contain its own independent implementation of all command logic.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                 | Notes / Expected Output                                                                      |
| --- | ------ | ---------------------------------------------------- | -------------------------------------------------------------------------------------------- |
| T1  | DONE   | Copy HTTP announce/scrape commands to unified binary | New command handlers in `console/tracker-client/src/console/clients/unified/`; tests copied. |
| T2  | DONE   | Copy UDP announce/scrape commands to unified binary  | New command handlers in `console/tracker-client/src/console/clients/unified/`; tests copied. |
| T3  | DONE   | Copy checker command to unified binary               | New command handler in `console/tracker-client/src/console/clients/unified/`; tests copied.  |
| T4  | DONE   | Add deprecation notices to legacy binaries           | Each old binary prints a deprecation warning on startup; no new features added to them.      |
| T5  | DONE   | Update in-repo docs, skills, and CI references       | All in-repo references to old binary names updated or annotated.                             |
| T6  | DONE   | Run manual verification scenarios and validate gates | Execute the local-tracker manual test matrix and record status/evidence for every scenario.  |

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

| ID  | Scenario                     | Command                                                                                                                                                                                                  | Expected Result                                                       | Status | Evidence                                                                                          |
| --- | ---------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------- | ------ | ------------------------------------------------------------------------------------------------- |
| M1  | HTTP announce (JSON default) | `cargo run --bin tracker_client http announce http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422`                                                                                            | Command exits 0 and prints valid JSON announce response               | DONE   | Exit 0; output: `{"complete":1,"incomplete":0,"interval":120,"min interval":120,"peers":[]}`      |
| M2  | HTTP scrape (JSON default)   | `cargo run --bin tracker_client http scrape http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422`                                                                                              | Command exits 0 and prints valid JSON scrape response                 | DONE   | Exit 0; output: `{"9c38422213e30bff212b30c360d26f9a02136422":{"complete":1,"downloaded":10,...}}` |
| M3  | UDP announce (JSON default)  | `cargo run --bin tracker_client udp announce udp://127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422`                                                                                              | Command exits 0 and prints valid JSON announce response               | DONE   | Exit 0; output: `{"AnnounceIpv4":{"transaction_id":...,"announce_interval":120,...}}`             |
| M4  | UDP scrape (JSON default)    | `cargo run --bin tracker_client udp scrape udp://127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422`                                                                                                | Command exits 0 and prints valid JSON scrape response                 | DONE   | Exit 0; output: `{"Scrape":{"transaction_id":...,"torrent_stats":[{"seeders":2,...}]}}`           |
| M5  | Checker command              | `TORRUST_CHECKER_CONFIG='{"udp_trackers":["127.0.0.1:6969"],"http_trackers":["http://127.0.0.1:7070"],"health_checks":["http://127.0.0.1:1212/api/health_check"]}' cargo run --bin tracker_client check` | Command exits 0 and reports successful UDP/HTTP/health checks in JSON | DONE   | Exit 0; JSON array with `Udp`, `Health`, `Http` keys all showing `Ok`                             |
| M6  | HTTP announce (text format)  | `cargo run --bin tracker_client http announce --format=text http://127.0.0.1:7070 9c38422213e30bff212b30c360d26f9a02136422`                                                                              | Command exits 0 and prints human-readable response                    | DONE   | Exit 0; pretty-printed JSON with `"complete"`, `"peers"` keys                                     |
| M7  | UDP scrape (text format)     | `cargo run --bin tracker_client udp scrape --format=text udp://127.0.0.1:6969 9c38422213e30bff212b30c360d26f9a02136422`                                                                                  | Command exits 0 and prints human-readable response                    | DONE   | Exit 0; pretty-printed JSON with `"Scrape"`, `"torrent_stats"` keys                               |

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
- [x] Implementation completed (copy-and-port approach, one command at a time)
- [x] Reviewer validated acceptance criteria and updated checkboxes
- [x] Committer verified spec progress is up to date before commit
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
- 2026-05-13 11:00 UTC - User - Clarified implementation strategy: progressive copy-and-port approach (not dispatcher pattern). Old code must be frozen and never called from new code.
- 2026-05-13 11:05 UTC - Copilot - Reset working tree; updated issue spec with new implementation strategy section. Reorganized tasks (T1-T6) to reflect copy-and-port approach with one command at a time. Enhanced acceptance criteria to explicitly require independent implementations and frozen old code.
- 2026-05-13 12:00 UTC - Copilot - T1 DONE: copied HTTP announce/scrape handlers into `console/tracker-client/src/console/clients/unified/http.rs`; independent implementation, not calling old `http/app.rs`.
- 2026-05-13 12:05 UTC - Copilot - T2 DONE: copied UDP announce/scrape handlers into `unified/udp.rs`; independent command handlers.
- 2026-05-13 12:10 UTC - Copilot - T3 DONE: copied checker/monitor handlers into `unified/check.rs`; re-parses args via clap `try_parse_from` with prefix `["tracker_client-check"]`.
- 2026-05-13 12:15 UTC - Copilot - Created `console/tracker-client/src/bin/tracker_client.rs` entrypoint; wired `http`, `udp`, `check` subcommands.
- 2026-05-13 12:20 UTC - Copilot - T4 DONE: added startup deprecation `eprintln!` to all three legacy binaries.
- 2026-05-13 12:25 UTC - Copilot - T5 DONE: updated skill files (public-trackers-for-testing, run-tracker-locally) and feature docs to use `tracker_client http/udp/check` commands.
- 2026-05-13 12:30 UTC - Copilot - Ported `tracker_checker.rs` integration tests to invoke `tracker_client check --`; added new `tracker_client.rs` integration tests for help, HTTP error, UDP error paths.
- 2026-05-13 12:35 UTC - Copilot - Renamed internal module `tracker` → `unified` to better convey migration state; added Rust doc comments explaining deprecation plan.
- 2026-05-13 12:40 UTC - Copilot - All automated gates pass: 46 unit tests, 10 checker integration tests, 3 unified binary integration tests, `linter all` exits 0.
- 2026-05-13 13:00 UTC - Copilot - T6 DONE: ran manual verification matrix M1–M7 against local tracker; all 7 scenarios exit 0 with correct output. Spec updated with evidence.
- 2026-05-13 15:00 UTC - Copilot - Recorded design decision: `unified/` sub-modules kept flat (no per-action nesting); deferred to #1775 cleanup. Cross-referenced `unified/mod.rs` in spec `related-artifacts`.
- 2026-05-13 15:30 UTC - Copilot - Implementation complete. All tasks (T1–T6) DONE, all ACs (AC1–AC13) verified, all manual scenarios (M1–M7) passed. Remaining workflow step: open implementation PR, merge, close GitHub issue #1771, move spec to `docs/issues/closed/`.

## Acceptance Criteria

- [x] AC1: A single `tracker_client` binary exists with `http announce`, `http scrape`,
      `udp announce`, `udp scrape`, and `check` subcommands.
- [x] AC2: All command logic is **copied** (not called/dispatched) from the old binaries into
      the new unified binary. The new binary contains its own independent implementation of all
      command handlers.
- [x] AC3: `--format=json` (default) produces valid JSON on stdout for all subcommands.
- [x] AC4: `--format=text` produces human-readable output for all subcommands.
- [x] AC5: Each legacy binary (`http_tracker_client`, `udp_tracker_client`, `tracker_checker`)
      prints a deprecation notice on startup directing users to `tracker_client`. The old code
      is otherwise **unchanged and frozen** — no new functions or modifications are added to
      the old binary implementations.
- [x] AC6: Old binary code is **never called from the new binary**. The old code is source
      material for copying only.
- [x] AC7: Tests for all three command sets are ported to use the new `tracker_client` binary,
      with no behaviour regression versus the old binaries.
- [x] AC8: In-repo docs and skill files that reference old binary names are updated.
- [x] AC9: A follow-up issue for removing the legacy binaries (no earlier than ~1 year after
      `tracker_client` ships) is linked from this spec or the EPIC.
      Follow-up: <https://github.com/torrust/torrust-tracker/issues/1775>
- [x] AC10: Top-level `announce`/`scrape` auto-dispatch aliases are not implemented in this
      issue (kept for follow-up to prevent scope drift).
- [x] AC11: `linter all` exits with code `0`.
- [x] AC12: All tests pass.
- [x] AC13: Manual verification matrix scenarios (M1-M7) are executed against a local tracker,
      with status and evidence recorded for each.

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence                                                                                                                                              |
| ----- | ---------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------- |
| AC1   | DONE                   | `console/tracker-client/src/bin/tracker_client.rs`; `unified/app.rs` defines `Http`, `Udp`, `Check` subcommands                                       |
| AC2   | DONE                   | `unified/http.rs`, `unified/udp.rs`, `unified/check.rs` are independent copies; no calls to `http::app::run`, `udp::app::run`, or `checker::app::run` |
| AC3   | DONE                   | M1–M5 all exit 0 with compact JSON output; `it_should_fail_http_announce_for_invalid_infohash` integration test validates JSON error path             |
| AC4   | DONE                   | M6 (HTTP announce `--format=text`) and M7 (UDP scrape `--format=text`) both exit 0 with pretty-printed JSON                                           |
| AC5   | DONE                   | `src/bin/http_tracker_client.rs`, `udp_tracker_client.rs`, `tracker_checker.rs` each print `eprintln!("warning: ... is deprecated ...")` on startup   |
| AC6   | DONE                   | `unified/` modules only import library helpers (`udp::checker`, `checker::checks`, etc.), never call old `app::run()` functions                       |
| AC7   | DONE                   | `tests/tracker_checker.rs` and submodules ported to `tracker_client_check_bin()` invoking `tracker_client check --`; 13 integration tests pass        |
| AC8   | DONE                   | Skills (`public-trackers-for-testing/SKILL.md`, `run-tracker-locally/SKILL.md`) and `docs/features/json-request-input/README.md` updated              |
| AC9   | DONE                   | Follow-up issue opened: <https://github.com/torrust/torrust-tracker/issues/1775>                                                                      |
| AC10  | DONE                   | `tracker_client --help` shows only `http`, `udp`, `check` subcommands; no top-level `announce`/`scrape` aliases                                       |
| AC11  | DONE                   | `just linter all` exits 0 (markdownlint, yamllint, taplo, cspell, clippy, rustfmt, shellcheck all pass)                                               |
| AC12  | DONE                   | `cargo nextest run` — 46 unit tests + 13 integration tests all pass                                                                                   |
| AC13  | DONE                   | M1–M7 executed against local tracker (`127.0.0.1:7070`/`6969`/`1212`); all exit 0 with correct output (see scenario matrix above)                     |

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
