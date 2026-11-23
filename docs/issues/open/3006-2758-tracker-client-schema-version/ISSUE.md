---
schema-version: 1
doc-type: issue
issue-type: task
status: planned
priority: p2
epic: 2758
github-issue: 3006
spec-path: docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md
branch: 3006-tracker-client-schema-version-spec
related-pr: 3008
last-updated-utc: "2026-11-23 16:48"
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - .claude/skills/create-issue/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/issues/open/2758-cli-output-contract-tools-migration/EPIC.md
    - console/tracker-client/docs/contracts/tracker-cli-io-contract.md
---

<!-- skill-link: create-issue -->

# Issue #3006 - `tracker_client`: add a schema version to the results

Parent EPIC: #2758 - Migrate the CLI tools to the global output contract

## Goal

Every result object that `tracker_client` writes on stdout carries a schema version, so a consumer
can detect a breaking change of the output shape instead of failing on it.

## Background

The `tracker_client` I/O contract (`console/tracker-client/docs/contracts/tracker-cli-io-contract.md`)
says, in its Stability and Migration section, that a breaking change introduces a schema version
and a migration note, and its Auditability section lists `schema_version` as a recommended field.

Issue #2768 changed the `check` result from a top-level array to `{"results": [...]}`, a breaking change.
No schema version existed, so it was recorded only as a documented breaking change in the contract
doc and the README. The reviewer da2ce7 (GitHub user `da2ce7`) raised it in round 2 of PR 2956 (audit row F7 of
`docs/pr-reviews/pr-2956-review/PR-REVIEW.md`), and the contract doc now says a version is left to a
later issue of the EPIC. No open sub-issue of #2758 owns it; this issue does.

No `stdout-result-data` tool of the global contract has a version field today. The global ADR
(`docs/adrs/20260519000000_define_global_cli_output_contract.md`) does not define one. Outside the
contract, `contrib/dev-tools/checks/agent-reply-validator` already uses a schema version of 1 for
its own JSON reply files, under the kebab-case key `schema-version` (its format uses kebab-case
keys and rejects unknown fields).

Today `tracker_client` has four render paths and one writer: `output::render_json` (`check`),
`serialize_json` in `unified/http.rs` (`http announce`, `http scrape`), `to_json_string` of the UDP
response DTO (`udp announce`, `udp scrape`) and a plain `serde_json::to_string` in
`checker/monitor/udp.rs` (`check monitor udp`). All of them pass an already rendered string to
`output::write_result(&str)`, so that writer cannot add a field. Both `--format` values write
JSON: `json` is compact and `text` is the same JSON pretty-printed (`OutputFormat::is_pretty` in
`unified/app.rs`), so a result in either format is one JSON value that `jq` reads.

Every result is a JSON object, but two shapes do not take a top-level field as they are:

- `http scrape` writes the scrape response through its own `Serialize` impl
  (`packages/http-protocol/src/v1/responses/scrape/deserialization.rs`), which makes the `files`
  map the top-level object: every key is a 40-hex info-hash (`{"<info-hash>": {...}}`). A
  `schema_version` key there would sit among the data keys, and a consumer that reads each key as
  an info-hash would break; the package test helper `returned_files` in
  `console/tracker-client/tests/tracker_client/scrape.rs` counts the top-level keys as files.
- The UDP response is an externally tagged enum with one top-level key, the variant tag
  (`{"AnnounceIpv4": {...}}`, `{"Scrape": {...}}`). The repository's consumers index it by key
  (`.Scrape.torrent_stats`); none deserializes it as a typed enum, which expects exactly one key.

The HTTP announce result, the `check` result and the monitor summary are objects with named
keys.

## Decisions

The maintainer answered D1 to D3 before the issue was created and D4 and D5 during the review of
PR #3008 (all on 2026-11-23):

- D1, where the rule lives: the global contract ADR
  (`docs/adrs/20260519000000_define_global_cli_output_contract.md`) gets one rule for every
  `stdout-result-data` tool (field name, type, place, when to bump), and this issue applies it to
  `tracker_client`. Other tools adopt it when they next change shape; #2770 records which tools
  have it. A tool-specific rule in the `tracker_client` contract doc was rejected because it could
  diverge from the other tools.
- D2, the field: `schema_version`, an integer, at the top level of every stdout result object
  (`{"schema_version": 1, ...}`), not on stderr diagnostics. It is snake_case like the keys that
  `tracker_client` defines itself (`results`, `udp_trackers`), unlike the kebab-case
  `schema-version` of `agent-reply-validator`.
- D3, the first value: `1`, for the shape after #2768 and this issue. The contract doc will name
  the two changes that `1` marks: the `check` array-to-object change of #2768 and the `http scrape`
  move under `files` of D4.
- D4, the `http scrape` shape (finding F1 of da2ce7's review of PR #3008; the maintainer answered the
  spec author's question in its agent session on 2026-11-23, and the reply on the F1 thread,
  <https://github.com/torrust/torrust-tracker/pull/3008#discussion_r4668650441>, records it): the info-hash map
  moves under a named key, `{"schema_version": 1, "files": {"<info-hash>": {...}}}`, the same key
  as in the bencoded tracker response. The ADR rule adds that a result object never uses data as
  its top-level keys, so the version field cannot collide with data.
- D5, the UDP shape (same question, answer and record as D4): the field goes beside the variant tag,
  `{"schema_version": 1, "Scrape": {...}}`. The tag is a name, not data, and no repository
  consumer reads the result as a typed enum; the risk for an outside typed reader is recorded
  under Risks.

## Scope

### In Scope

- Apply D1 to D3 to every subcommand that writes a result: `http announce`, `http scrape`,
  `udp announce`, `udp scrape` and `check`, each in both `--format` values (`json` compact, `text`
  pretty-printed; both are JSON), and
  the `check monitor udp` summary (always compact JSON).
- Move the `http scrape` result under `files` (D4), in `tracker_client`'s own result type; the
  `Serialize` impl of the protocol package stays as it is.
- Replace the string writer with one function in `unified/output.rs` that takes the result value,
  adds the field, renders it (compact or pretty) and writes it; every result path calls it.
- Update the contract doc (Stability and Migration, Auditability, examples) and the README.
- A short amendment of the global contract ADR with the rule (D1).
- Check every consumer of the `tracker_client` stdout and update any that parses the result, shows
  a whole result sample or compares it whole:
  - the package tests in `console/tracker-client/tests/` (`output_contract.rs`, `tracker_client/`
    including the `returned_files` helper of `scrape.rs`, `tracker_checker/`);
  - the skills that pipe the result to `jq` or show result samples: `use-tracker-client` (its
    whole result samples gain the field, and its `http scrape` samples and `jq` paths move under
    `files`), `run-tracker-locally`, `use-rest-api`, `public-trackers-for-testing`,
    `manual-udp-download-completion-e2e` and `manual-torrent-cleanup-e2e`;
  - the README.
- The e2e checker helper `src/console/ci/e2e/tracker_checker.rs` needs no change: it only logs the
  checker's stdout and never parses it. It stays out of this issue's file set (it is in #2769's).

### Out of Scope

- Versions for other tools (they adopt the rule of D1 when they next change shape).
- Masking the tracker key (separate bug issue of this EPIC).
- Any other change to the output shape than the field and D4.

Files this issue may touch (a sibling issue in the same wave must not touch them):

- `console/tracker-client/**`
- `.claude/skills/{use-tracker-client,run-tracker-locally,use-rest-api,public-trackers-for-testing,manual-udp-download-completion-e2e,manual-torrent-cleanup-e2e}/SKILL.md`
- `docs/adrs/20260519000000_define_global_cli_output_contract.md` (the amendment). #2770 also
  edits that ADR, so this issue must merge before #2770 starts.

## Architectural Decisions

- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`,
  `docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md`.
- ADRs to create: none. The global contract ADR is amended in place with the schema version rule
  (a repository-wide decision, so it stays in root `docs/adrs/`).

## Design and Ownership Review

Not applicable: no child processes, readiness waits or fixtures. One place adds the field: the
result writer in `unified/output.rs`, which takes the result value instead of a rendered string,
so no subcommand can forget it and no subcommand renders its own result any more.

## Bug-Fix Process

Not applicable: this is a contract feature, not a defect.

## Regression Test Strategy

Not applicable: not a bug. Tests are in the Verification Plan.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task | Notes / Expected Output |
| --- | ------ | ---- | ----------------------- |
| T1  | TODO   | ADR rule | The amendment of the global contract ADR with the rule of D1 to D3 and the no-data-keys rule of D4. |
| T2  | TODO   | Tests first | Each result-writing subcommand: a test that the result has `schema_version` equal to `1`, and an `http scrape` test that the entries are under `files`; failing before T3. |
| T3  | TODO   | Add the field in the result writer | `output.rs` takes the value, adds the field, renders and writes; the four render paths call it. Every stdout result carries it; `http scrape` writes `{"files": ...}` (D4) and UDP results keep their tag beside the field (D5). |
| T4  | TODO   | Consumers | The package tests (`returned_files` reads `files`) and the skills named in Scope updated; their commands still work. |
| T5  | TODO   | Docs | Contract doc and README examples show the field, the `files` key of `http scrape` and the bump rule, and name the two changes that `1` marks. |

## Commit Points

| Task | Coherent change set | Commit policy |
| ---- | ------------------- | ------------- |
| T1 | ADR amendment | Commit after focused validation and required review. |
| T2 | Tests | Commit after focused validation and required review. |
| T3 | Result writer | Commit after focused validation and required review. |
| T4 | Tests and skills that consume the result | Commit after focused validation and required review. |
| T5 | Docs | Commit after focused validation and required review. |

Use the `write-unit-test` skill and the design review after each passing test increment. Sign every
commit.

## Progress Tracking

### Workflow Checkpoints

- [x] Folder-style spec drafted in `docs/issues/drafts/2758-tracker-client-schema-version/ISSUE.md` (now `docs/issues/open/3006-2758-tracker-client-schema-version/ISSUE.md`)
- [x] Decisions D1 to D3 answered by the maintainer
- [ ] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] Spec-only PR merged into `develop` before implementation
- [ ] Implementation completed
- [ ] Automatic verification completed (`linter all`, relevant tests, and any pre-push checks)
- [ ] Manual verification scenarios executed and recorded in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria reviewed after implementation and updated with evidence
- [ ] Evidence-based implementation completion review recorded
- [ ] Reviewer validated acceptance criteria and updated checkboxes
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/open/` to `docs/issues/closed/`

### Progress Log

- 2026-11-19 16:16 UTC - Claude Code (Claude Opus 5.5) - Drafted this spec as the EPIC #2758 coordinator from the #2768 leftover (PR 2956 audit row F7).
- 2026-11-23 13:46 UTC - Claude Code (Claude Opus 5.5) - Re-read the draft against `develop` (`ff3600f10`): named the four render paths and the string-only writer, `--format` on every result subcommand, the `check monitor udp` summary, the consumers, and the kebab-case key of `agent-reply-validator`.
- 2026-11-23 14:02 UTC - Claude Code (Claude Opus 5.5) - Recorded the maintainer's answers (D1 global ADR rule, D2 top-level integer `schema_version`, D3 first value `1`) and fitted Scope, ADR, tasks, ACs and risks to them.
- 2026-11-23 14:02 UTC - Claude Code (Claude Opus 5.5) - Created issue #3006 as josecelano-bot, moved the spec to `docs/issues/open/`, branch `3006-tracker-client-schema-version-spec` for the spec-only PR.
- 2026-11-23 14:18 UTC - Claude Code (Claude Opus 5.5) - Opened spec-only PR #3008.
- 2026-11-23 16:10 UTC - Claude Code (Claude Opus 5.5) - Addressed da2ce7's review of PR #3008: recorded the maintainer's answers D4 (`http scrape` entries under `files`) and D5 (UDP field beside the tag) for F1, named the skills and the README among the consumers and in the file set and stated why the e2e helper needs no change (F3), asked for toolchain-qualified evidence (F4), fixed the snake_case claim (F6) and the tense of D3 (F7). F5: the maintainer answered D1 to D3 before the issue was created; the approval of the whole spec, now with D4 and D5, is the review and merge of PR #3008, so that checkpoint stays open until then. F2 (EPIC row and order before #2770) is in the coordinator's PR #3007.
- 2026-11-23 16:18 UTC - Claude Code (Claude Opus 5.5) - Addressed Copilot's review of PR #3008: stated that `--format text` is pretty-printed JSON, so `jq` reads both formats, and named da2ce7 as a GitHub user.
- 2026-11-23 16:34 UTC - Claude Code (Claude Opus 5.5) - Addressed da2ce7's second review of PR #3008: D4 and D5 name where the maintainer answered and the F1 thread reply that records it, and D4, D5 and AC5 now follow D3 and AC4.
- 2026-11-23 16:48 UTC - Claude Code (Claude Opus 5.5) - Addressed Copilot's second review of PR #3008: `agent-reply-validator` "uses" a schema version, not "fixes" one.

## Acceptance Criteria

- [ ] AC1: Every stdout result of `tracker_client` has a top-level integer `schema_version` equal to
  `1`.
- [ ] AC2: The contract doc states when the version is bumped and lists the changes that the
  current value marks (#2768's `check` change and D4).
- [ ] AC3: The global contract ADR states the rule for all `stdout-result-data` tools.
- [ ] AC4: Every consumer of the `tracker_client` stdout in the repository still works.
- [ ] AC5: The `http scrape` result holds its info-hash entries under `files`, and no result uses
  data as a top-level key.
- [ ] `linter all` exits with code `0`
- [ ] Relevant tests pass
- [ ] Manual verification scenarios are executed and documented in issue-local `manual-verification-evidence.md`
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

### Automatic Checks

- `linter all`
- `cargo test -p torrust-tracker-client`
- Pre-push checks

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario | Human-oriented command/steps | Expected Result | Status | Evidence |
| --- | -------- | ---------------------------- | --------------- | ------ | -------- |
| M1 | Announce and scrape | Run `tracker_client http announce ... \| jq .schema_version` against a local tracker, and the UDP and scrape equivalents, in both `--format` values | `1` for each | TODO | `manual-verification-evidence.md` section V1 |
| M2 | `check` and monitor | `tracker_client check ... \| jq .schema_version` in both `--format` values, and a short `check monitor udp` run | `1` | TODO | `manual-verification-evidence.md` section V2 |
| M3 | E2E runner | Run the e2e runner | The checker step passes | TODO | `manual-verification-evidence.md` section V3 |

Manual verification is mandatory even when automated tests pass. Record the actual commands, output
and the toolchain that produced each result (for example the `stable Rust toolchain` for
`cargo test -p torrust-tracker-client`, the `nightly Rust toolchain` for `cargo +nightly fmt`) in
`manual-verification-evidence.md` (from `docs/templates/MANUAL-VERIFICATION-EVIDENCE.md`).

### Acceptance Verification

| AC ID | Status (`TODO`/`DONE`) | Evidence |
| ----- | ---------------------- | -------- |
| AC1 | TODO |  |
| AC2 | TODO |  |
| AC3 | TODO |  |
| AC4 | TODO |  |
| AC5 | TODO |  |

## Risks and Trade-offs

- Adding a field breaks a strict consumer that rejects unknown fields. Neither the contract doc nor
  the ADR says consumers must ignore unknown fields today; the contract doc should say so in this
  issue, and T4 checks the repository's own consumers.
- D4 is a second breaking shape change: a consumer of `http scrape` that reads the info-hash keys
  at the top level must read them under `files`. Version `1` marks it, and the contract doc and
  README name it as a migration note.
- D5 leaves the UDP result with two top-level keys. An outside consumer that deserializes it as an
  externally tagged enum, which expects exactly one key, must drop `schema_version` first. No
  consumer in the repository does.
- The global rule costs an ADR change and orders this issue before #2770; a tool-specific rule
  would have avoided that but could diverge from the other tools.

## Implementation Completion Review

- Retrospective: `Not yet assessed`

## References

- Related issues: #2758 (EPIC), #2768 (the `check` shape change)
- Related PRs: PR 2956 (audit row F7)
- Related ADRs: `docs/adrs/20260519000000_define_global_cli_output_contract.md`
