---
doc-type: issue
issue-type: task
status: open
priority: p2
github-issue: 1798
spec-path: docs/issues/open/1798-global-cli-output-contract-adr.md
branch: 1798-global-cli-output-contract-adr
related-pr: null
last-updated-utc: 2026-05-19 14:00
semantic-links:
  skill-links:
    - create-issue
  related-artifacts:
    - docs/adrs/
    - console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md
    - console/tracker-client/docs/contracts/tracker-cli-io-contract.md
---

<!-- skill-link: create-issue -->

# Issue #1798 - Define a Global CLI Output Contract for the Tracker (ADR)

## Goal

Write a repository-wide Architectural Decision Record (ADR) that establishes a single, canonical
command-line output contract for every first-party, operator-facing CLI entrypoint in the
`torrust-tracker` repository, aligning with the approach adopted by `torrust-index`
(ADR-T-010) and reflecting the reality that AI agents are the dominant CLI consumers today.

**This ADR is prescriptive.** The current codebase does not yet comply with the rules it
establishes. Existing binaries will be migrated progressively in a separate follow-up issue.
The ADR must include a migration policy section so the gap between target state and current
state is documented, expected, and not treated as a defect.

## Background

### Existing partial contracts

The tracker already has a local CLI I/O contract, but it is scoped only to
`console/tracker-client`:

- `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
  (status: Accepted) — defines JSON default, stdout/stderr channel split, exit codes 0/1/2, and
  NDJSON progress for monitor-style commands.
- `console/tracker-client/docs/contracts/tracker-cli-io-contract.md` — the normative companion
  contract document.

That local contract was deliberately scoped to the tracker-client because it was expected to be
extracted into its own repository. However, other binaries in the tracker repo
(`http_health_check`, `e2e_tests_runner`, `profiling`, `qbittorrent_e2e_runner`, the server
`main` binary) have no governing output contract at all.

### What the Torrust Index decided

`torrust-index` adopted ADR-T-010 ("Global Command-Line Output Contract", decided and
implemented 2026-05-13). Key rules:

1. **Both streams are machine-readable.** Plain human-readable text is not a valid output format
   on either stdout or stderr.
2. **Stdout = result data.** Commands that produce result data emit exactly one JSON object
   followed by a trailing newline. On failure, stdout is empty.
3. **Stderr = diagnostics.** Logs, progress, help, usage errors, and panic records all go to
   stderr as JSON (NDJSON when multiple records arrive over time). `tracing` is the diagnostic
   writer.
4. **TTY refusal.** Commands with stdout result data refuse to run when stdout is attached to a
   terminal. They exit with code 2 and emit a JSON diagnostic on stderr. This rule is
   unconditional — it does not depend on payload sensitivity.
5. **Exit codes.** Baseline: `0` success, `1` runtime/internal failure, `2` usage/TTY/argv
   failure. Command-specific codes may extend this baseline.
6. **Shared Rust infrastructure.** A dedicated package (`packages/index-cli-common`) provides
   the shared scaffolding: JSON clap parser, JSON panic hook, JSON tracing setup, TTY refusal
   helper, stdout emitter, and workspace-level `clippy::print_stdout` / `clippy::print_stderr`
   denials.
7. **Redaction policy.** Secrets (DB URLs with credentials, JWT secrets, API keys, etc.) must
   not appear in JSON diagnostic output.

### The Deployer research

Earlier research in `torrust-tracker-deployer` explored separating user-friendly progress output
from internal tracing logs, with verbosity levels (`-q`, `-v`, `-vv`, `-vvv`). That research
treated JSON as a machine-mode option rather than the default and assumed human operators as the
primary audience. The format assumption is superseded here — JSON is always the format — but the
**concept of user-facing output verbosity levels remains useful** and should not be discarded.

However, two distinct concerns must not be conflated:

- **Internal tracing / logging levels** (`TRACE`, `DEBUG`, `INFO`, `WARN`, `ERROR`) are
  standard, well-defined levels for developer and operations observability. They are controlled
  by `RUST_LOG` or a `--debug` / `--log-level` flag and feed the `tracing` subscriber. These
  levels govern what the application emits about its own internal behaviour and are not
  user-facing output levels.
- **User-facing output verbosity levels** govern how much information a command surfaces to
  its caller (human or agent) about progress, intermediate results, and the final outcome. These
  levels are application-specific and depend on what data is meaningful to expose, whether the
  command produces a single final result or a stream of progress events, etc. They are a
  separate knob from internal log levels.

Both internal tracing records and user-facing progress events can land on stderr and overlap on
the same channel. NDJSON makes this manageable: each line is a self-contained JSON object with a
`kind` or `type` field, so callers can filter by record type regardless of interleaving. Users
can also redirect each concern independently at runtime — for example, send internal tracing to a
log file only while keeping user-facing progress events visible on stderr, or vice versa.

For the ADR, the number of user-facing verbosity levels should be kept to what is practically
useful for the commands in scope. A richer scheme is only worth the extra API surface if the
distinctions genuinely help callers make different decisions based on the level.

The key change from the Deployer approach is that all output at every verbosity level — both
internal tracing and user-facing — is JSON-formatted. There is no parallel plain-text output
path.

### Why this matters now

The primary consumers of CLI output for the tracker project are increasingly AI agents and
automation scripts, not humans reading a terminal in real time. This changes the calculus:

- **JSON should be the default, always.** There is no practical benefit to plain-text output when
  the primary consumer is an agent or script.
- **Clean stdout is critical.** Diagnostic noise mixed into result data breaks automated parsing.
  The separation of result data (stdout) from diagnostics (stderr) must be enforced mechanically,
  not just by convention.
- **User-facing verbosity levels are useful but must not be conflated with internal log levels.**
  Both are independent knobs. Internal tracing log levels (`RUST_LOG`) control observability for
  developers and ops; user-facing verbosity levels control how much progress and result detail a
  command surfaces to its caller. Both can appear on stderr as NDJSON and can be separated by
  record type or redirected independently via configuration. All output at every level must emit
  JSON, not plain text.
- **AI agents reusing terminals need explicit per-command output capture.** When an agent drives
  multiple commands in the same terminal session, terminal buffer sharing causes output to be
  mis-attributed or partially captured. The recommended pattern is per-command file redirection
  (`> .tmp/<cmd>.stdout 2> .tmp/<cmd>.stderr`). Because the contract enforces JSON on both
  channels, the captured files are always well-formed and parseable without ambiguity.

### The TTY refusal question

TTY refusal is the most controversial rule from ADR-T-010. The rule says: if a command has
stdout result data and stdout is attached to a terminal, the command refuses to run and exits 2.
Operators can inspect output by piping to `jq`, `less`, or `cat`.

**Arguments in favour:**

- It enforces the contract mechanically. A developer cannot accidentally run a stdout-producing
  command interactively and see raw JSON scrolling past without realizing the output is not
  captured.
- For AI agents, it prevents them from driving a command in a pseudo-terminal and seeing
  terminal-formatted or partially buffered output that breaks JSON parsing.
- It removes the temptation to add ANSI color codes or human-friendly text to stdout result
  data "just for interactive use". The contract stays clean.
- Example: `http_health_check` emitting `{"status":"healthy","elapsed_ms":12}` — if run in a
  terminal it should refuse and tell the operator to pipe it. `http_health_check | jq .` works
  fine and gives a pretty-printed result.
- Example: a future `tracker-client announce` command that returns a peers list — the JSON output
  is meant for scripts. TTY refusal prevents accidental interactive use and makes the expectation
  explicit.

**Arguments against / open questions:**

- Developer experience friction: running `tracker-client udp announce --url udp://localhost:6969`
  during local debugging is more cumbersome if you must always pipe to `cat`.
- Commands with no stdout result data (e.g. the server `main` binary, `e2e_tests_runner`,
  `profiling`) are unaffected — TTY refusal only applies to commands that emit stdout result data.
  Many tracker binaries may fall in the no-stdout-result-data class, which would make the rule
  largely moot for the most commonly interactive binaries.
- Is there a middle ground, e.g. a `--allow-tty` flag? The Index ADR deliberately rejects this
  because it re-introduces the "two modes" complexity. This needs a concrete decision here.

**Decision: adopted.** TTY refusal is adopted as stated, unconditionally, for all commands
that emit stdout result data. The ADR must record this decision with the full rationale above.

### AI agent terminal output capture

A related concern arises specifically when AI agents drive CLI commands. Agents such as GitHub
Copilot reuse a single persistent terminal session across multiple commands to avoid spawning
extra processes. This creates a capture problem:

- The agent may receive **partial output** if the terminal buffer is read before the command
  finishes.
- Output from **multiple commands** may be interleaved in the same buffer, causing the agent to
  attribute the wrong output to the wrong command.
- **User-interleaved input** — a user typing additional commands in the same terminal session —
  is invisible to the agent and silently corrupts the captured output.

The recommended mitigation is for agents to **redirect each command's output to independent
files**, even when commands share the same terminal:

```sh
my-command > .tmp/my-command.stdout 2> .tmp/my-command.stderr
```

The agent then reads the file to obtain the exact, unambiguous output for that command. The
`.tmp/` directory (workspace-local, git-ignored) is the recommended location because:

1. It is inside the workspace, so the user has a well-known, accessible record of every command
   the agent executed and its output — not buried in agent-internal storage.
2. It is git-ignored, so captured output does not accidentally enter version control.
3. It follows the established convention in this repository (see `TORRUST_GIT_HOOKS_LOG_DIR=.tmp`
   in `AGENTS.md`).

Using **two separate files per command** (one for stdout, one for stderr) preserves the
channel split that the output contract depends on. This is only unambiguous because the contract
mandates JSON on both channels — a mixed plain-text/JSON scheme would make file-based capture
unreliable. The ADR should include this as a recommended practice for agents driving tracker
CLI commands.

### Relationship to the tracker-client local ADR

The local tracker-client ADR and contract document are consistent with the direction proposed
here but are narrower in scope. The decision on disposition is:

- The global ADR **supersedes and deprecates** the local tracker-client ADR
  (`20260512080000_define_tracker_cli_io_contract_and_error_handling.md`) and its companion
  contract document. Once the global ADR is accepted, the local ADR is marked as superseded
  and the local contract document becomes a tracker-client–specific supplement (covering only
  rules unique to the tracker-client, such as NDJSON progress events and the tracker vs. app
  error taxonomy).
- When `console/tracker-client` is extracted into its own repository, a copy of the global ADR
  (or a reference to the version in effect at extraction time) is included in the new repo so
  the two can evolve independently from that point forward.
- If the Torrust Org later decides to adopt this as an organisation-wide convention, the global
  ADR can be promoted to an org-level document. Until that decision is made, each repo maintains
  its own copy.

## Scope

### In Scope

- All first-party, operator-facing CLI entrypoints shipped or documented in this repository,
  including:
  - `src/main.rs` — tracker server binary (long-running daemon; expected no-stdout-result class).
  - `src/bin/http_health_check.rs` — expected stdout-result-data class (JSON health status).
  - `src/bin/e2e_tests_runner.rs` and `src/bin/qbittorrent_e2e_runner.rs` — classification TBD.
  - `src/bin/profiling.rs` — likely developer-only; may be out of normative scope.
  - `console/tracker-client/` — all tracker-client subcommands.
- The TTY refusal rule: **adopted as stated** (commands with stdout result data refuse when
  stdout is a TTY; exit 2 with JSON stderr diagnostic).
- A shared Rust CLI infrastructure package (or a decision not to create one and why).
- Workspace-level `clippy::print_stdout` / `clippy::print_stderr` lint guards.
- A redaction policy for JSON diagnostics.
- Relationship to and disposition of the existing tracker-client local ADR
  (`20260512080000_define_tracker_cli_io_contract_and_error_handling.md`) and contract document.
- A recommended practice for AI agents driving CLI commands: per-command output redirection to
  `.tmp/<command>.stdout` and `.tmp/<command>.stderr`.

### Out of Scope

- Developer-only tooling (`contrib/dev-tools/`, benchmarks, examples, tests).
- `build.rs` Cargo protocol output.
- Changes to the tracker-server internal tracing configuration beyond ensuring tracing
  diagnostics go to stderr as JSON.
- Individual command-level contract documents (those remain in the relevant package or
  `console/` subtree).
- Implementation work — this issue is to produce the ADR only. A follow-up issue will cover
  migrating existing binaries to the contract.

## Implementation Plan

Status values: `TODO`, `IN_PROGRESS`, `BLOCKED`, `DONE`.

| ID  | Status | Task                                                                           | Notes / Expected Output                                                                                                                                                                                                                                                       |
| --- | ------ | ------------------------------------------------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| T1  | TODO   | Enumerate and classify all in-scope binaries                                   | A table of binaries with their expected class (stdout-result or no-stdout); base for ADR scope section                                                                                                                                                                        |
| T2  | DONE   | Decide on TTY refusal rule                                                     | Decision: **adopt as stated** (maintainer confirmed 2026-05-19); rationale to be recorded in ADR text (T5)                                                                                                                                                                    |
| T3  | TODO   | Decide on user-facing verbosity level scheme                                   | Define levels for user-facing progress/result output (distinct from internal `RUST_LOG` tracing levels); all levels must emit JSON; document rationale for the chosen set                                                                                                     |
| T4  | TODO   | Decide on shared CLI infrastructure package                                    | Create `packages/tracker-cli-common` (mirroring `index-cli-common`) or use a lighter approach; consider whether extraction plan for tracker-client changes this                                                                                                               |
| T5  | TODO   | Draft the global CLI output contract ADR                                       | File at `docs/adrs/YYYYMMDDHHMMSS_global_cli_output_contract.md`; follow the ADR template; reference this spec and related docs; **must include a migration policy section** stating that current code does not yet comply and migration is progressive via a follow-up issue |
| T6  | TODO   | Mark tracker-client local ADR as superseded; narrow its companion contract doc | Local ADR marked superseded by the global ADR; companion contract doc scoped to tracker-client–only rules (NDJSON progress, tracker vs. app error taxonomy)                                                                                                                   |
| T7  | TODO   | Define workspace lint guard policy                                             | Decide whether to deny `clippy::print_stdout` / `clippy::print_stderr` at workspace level; note interaction with issue #1786 (workspace lints migration)                                                                                                                      |
| T8  | TODO   | Peer-review ADR draft                                                          | At minimum one review pass before accepting; update status to `Accepted`                                                                                                                                                                                                      |
| T9  | TODO   | Add ADR to `docs/adrs/index.md`                                                | Row added to the index table                                                                                                                                                                                                                                                  |

## Progress Tracking

### Workflow Checkpoints

- [x] Spec drafted in `docs/issues/drafts/`
- [x] Spec reviewed and approved by user/maintainer
- [x] GitHub issue created and issue number added to this spec
- [ ] ADR draft written
- [x] TTY refusal decision confirmed by maintainer (adopt as stated, 2026-05-19)
- [ ] TTY refusal decision recorded in ADR
- [ ] Verbosity level scheme decided and recorded in ADR
- [ ] Shared infrastructure decision recorded in ADR
- [ ] Existing tracker-client local ADR marked superseded; companion contract doc narrowed
- [ ] ADR peer-reviewed and status set to `Accepted`
- [ ] ADR added to `docs/adrs/index.md`
- [ ] Committer verified spec progress is up to date before commit
- [ ] Issue closed and spec moved from `docs/issues/drafts/` to `docs/issues/closed/`

### Progress Log

- 2026-05-18 00:00 UTC - Copilot (GitHub Copilot) - Spec drafted based on review of tracker-client
  local ADR, Index ADR-T-010, and Deployer UX research docs.
- 2026-05-19 00:00 UTC - Copilot (GitHub Copilot) - Spec updated: TTY refusal marked as pending
  maintainer decision; verbosity levels reframed as useful for both humans and AI agents (JSON
  format only); tracker-client local ADR disposition set to supersede/deprecate.
- 2026-05-19 12:00 UTC - Copilot (GitHub Copilot) - TTY refusal decision confirmed as adopted;
  new background subsection added on AI agent terminal output capture (per-command file
  redirection to `.tmp/`, user-accessible well-known location); related updates to in-scope,
  AC, M scenarios, and "Why this matters now".
- 2026-05-19 13:00 UTC - Copilot (GitHub Copilot) - Clarified that the ADR is prescriptive;
  current code does not yet comply; migration is progressive via a follow-up issue; Goal section
  updated with explicit notice; T5 notes require migration policy section; AC12 and M8 added.
- 2026-05-19 14:00 UTC - Copilot (GitHub Copilot) - Linter passed (fixed British spelling
  "realising" → "realizing"); GitHub issue #1798 created; spec promoted to
  `docs/issues/open/1798-global-cli-output-contract-adr.md`; branch
  `1798-global-cli-output-contract-adr` created.

## Acceptance Criteria

- [ ] AC1: A new ADR file exists at `docs/adrs/YYYYMMDDHHMMSS_global_cli_output_contract.md`.
- [ ] AC2: The ADR states the output class (stdout-result or no-stdout) for every in-scope binary.
- [ ] AC3: The ADR makes a concrete, documented decision on TTY refusal (adopt / reject / caveats).
- [ ] AC4: The ADR defines the user-facing output verbosity level scheme (distinct from internal
      tracing log levels), with rationale for the chosen set of levels.
- [ ] AC5: The ADR makes a concrete decision on a shared CLI infrastructure package.
- [ ] AC6: The ADR defines the redaction policy for JSON diagnostics.
- [ ] AC7: The tracker-client local ADR is marked superseded and the companion contract doc is
      narrowed to tracker-client–specific rules.
- [ ] AC8: The ADR defines the workspace lint guard policy for `print_stdout` / `print_stderr`.
- [ ] AC9: The ADR is added to `docs/adrs/index.md`.
- [ ] AC10: The ADR is status `Accepted` after at least one review pass.
- [ ] AC11: The ADR includes a recommended practice for AI agents driving CLI commands
      (per-command output redirection to `.tmp/<command>.stdout` and `.tmp/<command>.stderr`,
      with rationale tied to the JSON-on-both-channels contract).
- [ ] AC12: The ADR includes a migration policy section that explicitly states the ADR is
      prescriptive, the current codebase does not yet comply, and migration will happen
      progressively via a dedicated follow-up issue.
- [ ] `linter all` exits with code `0`
- [ ] Manual verification scenarios are executed and documented (status + evidence)
- [ ] Acceptance criteria are re-reviewed after implementation and reflect actual behavior
- [ ] Documentation is updated when behavior/workflow changes

## Verification Plan

This issue produces a documentation artifact (an ADR), not runnable code. Verification is
therefore primarily review-based.

### Automatic Checks

- `linter all` — covers markdownlint, cspell, and taplo for the new ADR and this spec.

### Manual Verification Scenarios

Status values: `TODO`, `IN_PROGRESS`, `DONE`, `FAILED`, `BLOCKED`.

| ID  | Scenario                                                            | Command/Steps                                                                      | Expected Result                                                                                              | Status | Evidence |
| --- | ------------------------------------------------------------------- | ---------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ | ------ | -------- |
| M1  | ADR file passes markdownlint                                        | `linter all` or `markdownlint docs/adrs/<new-file>.md`                             | No markdownlint errors                                                                                       | TODO   |          |
| M2  | ADR covers all in-scope binaries                                    | Manual review of the binary classification table against `src/bin/` and `console/` | All binaries classified                                                                                      | TODO   |          |
| M3  | TTY refusal section gives concrete examples                         | Manual review of ADR text                                                          | At least two concrete examples explaining when TTY refusal fires                                             | TODO   |          |
| M4  | Verbosity level scheme is defined and distinguished from log levels | Manual review of ADR text                                                          | User-facing verbosity levels defined separately from `RUST_LOG` tracing levels; all levels produce JSON      | TODO   |          |
| M5  | Tracker-client local ADR marked superseded                          | Open `20260512080000_define_tracker_cli_io_contract_and_error_handling.md`         | Status changed to Superseded; reference to global ADR added                                                  | TODO   |          |
| M6  | ADR added to index                                                  | Check `docs/adrs/index.md`                                                         | New row present with correct date and title                                                                  | TODO   |          |
| M7  | ADR includes agent output capture recommendation                    | Manual review of ADR text                                                          | Per-command redirect to `.tmp/` documented with rationale tied to JSON contract                              | TODO   |          |
| M8  | ADR migration policy section is present                             | Manual review of ADR text                                                          | Section states ADR is prescriptive, current code non-compliant, migration is progressive via follow-up issue | TODO   |          |

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

## Risks and Trade-offs

- **TTY refusal friction vs. enforcement value.** If adopted, developers lose the ability to
  run stdout-producing commands directly in a terminal without piping. The benefit is a
  mechanically enforced contract. Mitigation: document the `| cat` / `| jq` workaround clearly;
  restrict the rule only to the commands that actually emit stdout result data (most tracker
  binaries do not).
- **Shared infrastructure package scope creep.** Creating `packages/tracker-cli-common` is
  useful but adds a new package to maintain. Mitigation: keep the package minimal — only the
  shared scaffolding listed in Index ADR-T-010 (clap handler, panic hook, tracing setup, TTY
  refusal, stdout emitter).
- **Tracker-client extraction timeline.** The local tracker-client ADR is superseded by the
  global ADR, and the tracker-client companion contract doc is narrowed to tracker-client–specific
  rules. When the tracker-client is extracted into its own repository, a copy of the global ADR
  (or a reference to the version in effect at extraction time) travels with it and evolves
  independently from that point. If the Torrust Org later adopts this as an org-wide convention,
  individual repo copies may be retired in favour of the org-level document.
- **Alignment with issue #1786** (workspace lints migration). The workspace lint guards for
  `print_stdout`/`print_stderr` interact with that issue. Mitigation: coordinate tasks; the
  global CLI ADR defines the policy, and #1786 implements it as part of workspace lints.
- **Inconsistency window.** Until individual binaries are migrated (a separate follow-up issue),
  the ADR will be accepted but not yet fully implemented. Mitigation: the ADR should include a
  migration policy (analogous to the tracker-client progressive migration rule) so the gap is
  documented and expected.

## References

- Existing tracker-client local ADR:
  `console/tracker-client/docs/adrs/20260512080000_define_tracker_cli_io_contract_and_error_handling.md`
- Existing tracker-client I/O contract:
  `console/tracker-client/docs/contracts/tracker-cli-io-contract.md`
- Torrust Index ADR-T-010 (the main reference and inspiration):
  <https://github.com/torrust/torrust-index/blob/develop/adr/010-global-command-line-output-contract.md>
- Torrust Tracker Deployer — console output research:
  - <https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/research/UX/console-output-logging-strategy.md>
  - <https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/research/UX/console-stdout-stderr-handling.md>
  - <https://github.com/torrust/torrust-tracker-deployer/blob/main/docs/research/UX/user-output-vs-logging-separation.md>
- Related issue: #1786 (workspace lints migration — interacts with `print_stdout`/`print_stderr` guards)
- ADR template: `docs/templates/ADR.md`
- ADR index: `docs/adrs/index.md`
