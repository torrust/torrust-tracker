---
doc-type: decision-record
issue-spec: docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
last-updated-utc: "2026-11-26 17:09"
---

# Decisions of issue #2766

## Purpose

This file records the migration detail of the daemon's log output: the alternatives considered, their
pros and cons, and the choice with its reason. The spec (task T1) asks for these decisions before any
code is written. They were prepared on 2026-11-26 by Claude Code (Claude Opus 5.5) in a
question-and-answer session with the maintainer (`josecelano`).

**Status: decided.** The maintainer answered the decisions one at a time on 2026-11-26; each
"Choice" says what was decided and by whom. The examples in this file are illustrative, not captured
from a run; the implementation records real output in `manual-verification-evidence.md`.

## Rules the maintainer gave for these decisions

1. **Choose the cleanest and best long-term design**: least surprise for operators, high
   maintainability, a good user interface.
2. **Breaking changes are allowed** in the logs and in the configuration API (the next release is
   4.0.0).
3. **Effort is not a reason** to choose the smaller design.
4. **Record the alternatives**, with pros and cons, for every decision.
5. **The default follows the CLI output contract; the operator is not forced to.** The tracker's
   default configuration follows the conventions of the contract ADR. An operator can switch to what
   they used before, or to another alternative, when the combination makes sense. A combination that
   does not make sense (for example JSON together with a human layout) is not offered.

## Facts found before deciding

- The daemon installs its `tracing` subscriber in `torrust_tracker_configuration::v3_0_0::logging::setup`
  (`packages/configuration/src/v3_0_0/logging.rs:65`), called from
  `bootstrap::app::initialize_global_services` (`src/bootstrap/app.rs:79`) after the configuration is
  loaded and validated. It uses `with_test_writer()` (stdout) and ANSI, in the `trace_style` chosen
  by the configuration (default `full`; `pretty`, `compact`, `json` also accepted).
- So a configuration failure happens before any subscriber exists: today it is reported only by
  `eprintln!` in `src/main.rs:44`. A failure after logging started is reported twice:
  `tracing::error!(%error, "Tracker startup failed")` (`src/main.rs:32`) and `report_startup_failure`
  (`src/main.rs:33`).
- `packages/configuration/src/logging.rs` is not compiled: `packages/configuration/src/lib.rs` has no
  `mod logging`. `packages/configuration/src/v2_0_0/logging.rs` is compiled but the daemon never calls
  its `setup`.
- Every default config under `share/default/config/` sets `trace_style = "full"`, and so does the
  example in `docs/benchmarking.md`.
- The v3 configuration schema is the one under development for 4.0.0 (develop keeps changing it) and
  already rejects a removed v2 key (`threshold`, test
  `it_should_reject_the_removed_threshold_field`) through `deny_unknown_fields`.
- The ADR of the contract says stderr records "should carry a `kind` field (or equivalent)" and that
  help text, usage errors and panic records go to stderr as JSON
  (`docs/adrs/20260519000000_define_global_cli_output_contract.md`).
- The daemon is not the only caller of the logging setup:
  - `packages/hung-component-tracker/src/main.rs` is a second binary with the same `main` (it calls
    `app::start_with_explicit_config_toml_path` and has its own `report_startup_failure` with
    `eprintln!`); the lifecycle tests run it.
  - `src/console/profiling.rs:202` calls `app::start()` and reports a failure the same way. The
    contract ADR excludes the `profiling` binary.
  - In-process tests call `initialize_global_services` (seven times in `src/app.rs`, and in
    `src/bootstrap/jobs/{tracker_apis,udp_tracker,http_tracker}.rs`).
  - `packages/axum-rest-api-server` calls the configuration crate's `logging::setup` directly
    (`src/testing/environment.rs:279`, behind the `testing` feature, and `src/server.rs:334`, in
    tests). It cannot depend on the tracker application.
- Configuration loading already emits `info!` records
  (`packages/configuration/src/lib.rs:197-224`, "Loading base configuration from …"). No subscriber
  exists yet at that moment, so today these records are lost.
- `tracker_client` (#2768, closed) already follows the contract and is the precedent in this
  repository: its tracing is `tracing_subscriber::fmt().json().with_writer(std::io::stderr)`
  (`console/tracker-client/src/console/clients/unified/app.rs:90`), which is tracing-subscriber's
  default shape (text under `fields.message`), and its panic hook writes
  `{"kind":"panic","message":…,"location":"file:line"}` directly
  (`console/tracker-client/src/console/clients/unified/output.rs:62`). The e2e runners still use
  the plain `fmt()` (`src/console/ci/e2e/runner.rs:135`, `src/console/ci/qbittorrent_e2e/runner.rs:150`;
  issue #2769).
- Two events pass the log target as a field, `target = crate::UDP_TRACKER_LOG_TARGET`, instead of
  `target:` (`packages/udp-core/src/services/announce.rs:178` and `scrape.rs:140`). In a flat record
  a field called `target` collides with the record's own `target` key.
- `app::Error` (`src/app.rs:52`) has seven variants and `bootstrap::app::Error` four; D5 maps each.
- No profile sets `panic = "abort"`, so a panic hook runs.
- Other files that show `trace_style` or the text log lines:
  `packages/configuration/docs/migrate-v2-to-v3.md`, `docs/benchmarking.md:195`, and the skills
  `manual-torrent-cleanup-e2e`, `manual-http-download-completion-e2e`,
  `manual-udp-download-completion-e2e` and `setup-dev-environment`.
- Sub-issue SI-22 of EPIC #2387 adds `run_id` to every record "in the formatter", and its AC16 expects
  a "Changes in 4.0.0" section in `docs/logging.md`.

The example records below are illustrative: real messages of today, in the shape each alternative
produces. Timestamps are shortened to `…`.

## D1 - The `trace_style` option

| Alternative | Example | Pros | Cons |
| --- | --- | --- | --- |
| A. Always JSON; `trace_style` still accepted, any value other than `json` logs one WARN deprecation record and is ignored | `{"level":"WARN","message":"trace_style = \"full\" is deprecated and ignored; logs are always JSON on stderr"}` | No deployment fails on upgrade. The operator is told once | A key that does nothing stays in the schema and in the code. A warning is easy to miss in a log stream. A second removal later is a second breaking change |
| B. JSON by default; `full`, `pretty`, `compact` kept as an opt-in, on stderr | `2026-11-26T10:00:00Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070` (opt-in) | Readable output for a developer in a terminal | The opt-in breaks the contract ("no plain text at any verbosity"). Two formats to test and support forever. Readers must handle both or refuse one |
| C. Key kept, only `json` accepted; other values are a configuration error | `{"kind":"startup_failure",…,"exit_code":2}` | Loud and explicit | Keeps a key that has exactly one legal value: noise in every config |
| D. Key removed from the v3 schema; a config that still has it fails startup as a configuration error (exit `2`), like the removed `threshold` key | `{"kind":"startup_failure","message":"… unknown field trace_style, expected trace_filter …","exit_code":2}` | The schema says exactly what the daemon does: one format. No dead code. Same treatment as the earlier removed key. The failure names the key at the first start, so nobody runs with a wrong belief | Every deployment whose config sets the key must delete one line before it starts. Mitigated by the release note and the migration step in the docs |
| E. Text when stderr is a terminal, JSON otherwise | — | Nice in a terminal | Breaks the contract; output depends on where it runs, which surprises operators and tests |

**Choice: B** (confirmed by the maintainer, 2026-11-26: "I want the JSON to be the default but I want operators to be
able to set an alternative configuration"). The recommendation was D; the maintainer keeps the
choice of style for operators. What B means exactly:

- `trace_style` stays in the v3 schema with the values `json`, `full`, `pretty` and `compact`. The
  default changes from `full` to `json`. An unknown value is a configuration error (exit `2`), as
  today. The environment override `TORRUST_TRACKER_CONFIG_OVERRIDE_LOGGING__TRACE_STYLE` works as
  for any other key.
- Every style writes to stderr. Stdout is always empty.
- Only `json` meets the output contract. A text style is the operator's explicit choice to leave it
  for the log records; the new ADR records this exception for the daemon, and the docs say that
  tools (the e2e runner, the test harness, a log collector that expects JSON) need `json`.
- **Records that are always JSON, in every style:** everything written before the configuration is
  known (help, usage errors, the "Loading … configuration" records, a configuration failure) and
  the two records that say why the process died (`startup_failure`, `panic`). The style is not
  known yet for the first group, and a supervisor must always be able to parse the second.
  The maintainer confirmed this on 2026-11-26 over the alternative (the two records follow the
  configured style: a uniform text stream, but no tool can parse the exit reason and a configuration
  failure would still be JSON, so failures would look different depending on when they happen).
- The shipped configs under `share/default/config/` and the example in `docs/benchmarking.md` drop
  `trace_style = "full"`, so they get the default.
- The text styles are tracing-subscriber's own formats, unchanged. The record shape of D4 is the
  `json` style.

See "Combinations" below for what each style prints in each situation.

## D2 - When the subscriber is installed

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. As today: after the configuration is loaded and validated; failures before that are written by a separate direct JSON writer in `main` | Small change | Two writers with two shapes on one stream. Everything before the subscriber (argument errors, configuration errors, panics during loading) needs its own code path |
| B. First thing in `main`, before argument parsing, with a level filter that can be changed later (`tracing_subscriber::reload`) at a default of `info`; after the configuration is loaded, the filter is set to `logging.trace_filter` | One formatter writes every stderr record of the process: help, usage errors, configuration errors, startup failures, panics and logs. One shape, one place to add a field (SI-22's `run_id`). The process starts in JSON; when the configuration is loaded, the level and, if a text style is configured, the formatter of log records are replaced (D1). The "Loading … configuration" records, lost today, become visible | Records written before the configuration is read use the default level, so a `trace_filter = "error"` config still lets the few `info` records of configuration loading through. A reload handle has to be carried to where the filter is set |

**Choice: B** (confirmed by the maintainer, 2026-11-26). It removes the "before logging" special case completely and gives every record the
same shape (D4). Consequences found in the code:

- The install function is public in `torrust_tracker_lib`, because `hung-component-tracker` has the
  same `main` and must behave like the daemon (the lifecycle tests read its output), and `profiling`
  calls it too. Both stop using `eprintln!` for the startup failure.
- `initialize_global_services` no longer installs a subscriber; it only sets the level, and does
  nothing when no subscriber was installed (in-process tests). Tests that want log output use
  `torrust_tracker_test_helpers::logging::setup()`, as most already do.

## D3 - Where the logging setup code lives

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Stays in the configuration crate (`v3_0_0::logging::setup`) | No move | A data crate owns process-wide side effects (the global subscriber). Each schema version carries a copy of the setup (v2 already does, and the `logging.rs` that is not compiled is a third copy) |
| B. Moves to the tracker application (`src/bootstrap/logging.rs`, the formatter in its own module); the configuration crate keeps only the data (`trace_filter` and the `TraceStyle` enum). Delete `packages/configuration/src/logging.rs`, which is not compiled, and the `setup`/`tracing_init` helpers of `v2_0_0` and `v3_0_0` | The configuration crate is data only, which is what it is for. One setup, one formatter, in the binary that owns the process. Removes dead code | A breaking API change of `torrust-tracker-configuration` (allowed). `packages/test-helpers` keeps its own plain capture. `packages/axum-rest-api-server` (testing environment and server tests) stops calling the configuration crate's `setup`; its tests call the test helper instead |
| C. A new shared crate for the CLI output contract (formatter, panic hook, JSON help), used by every binary | Reuse: the e2e runners (#2769) need the same JSON tracing | Built for one consumer before the second exists; the ADR says to extract common scaffolding gradually. The right shape is clearer once #2769 needs it |

**Choice: B** (confirmed by the maintainer, 2026-11-26), with the shared crate of alternative C
created later, in the follow-up issue that moves `tracker_client` to the record shape of D4.

The maintainer's condition: packages stay highly cohesive and loosely coupled, and usable on their
own by third-party projects. The check of every use in the workspace:

| User of the configuration crate's logging module | Where | What it uses |
| --- | --- | --- |
| Tracker application | `src/bootstrap/app.rs:79` (production) | `logging::setup` |
| `udp-server` | `src/server/mod.rs:61`, `src/server/launcher.rs:534` (unit tests) | `logging::setup` |
| `axum-http-server` | `src/server.rs:486` (unit tests) | `logging::setup` |
| `axum-rest-api-server` | `src/server.rs:334` (unit tests), `src/testing/environment.rs:279` (`testing` feature) | `logging::setup` |
| `test-helpers` | `src/logging.rs:6`, `src/configuration.rs:11` | The types `TraceStyle` and `Threshold`; it has its own setup |

The tracker application is the only production caller. The effect of B on each package:

| Package | Dependencies after B | Effect |
| --- | --- | --- |
| `configuration` | Loses `tracing-subscriber`, which only the setup code uses | Data and loading only. A third-party user no longer pulls in a logging backend or a function that installs a global subscriber |
| `udp-server`, `axum-http-server`, `axum-rest-api-server` (tests) | Unchanged: their tests call `torrust_tracker_test_helpers::logging::setup()`, already a dev-dependency of the three | No new edge |
| `axum-rest-api-server` (`testing` feature) | Unchanged: the test environment stops installing logging; the test that uses it does | No new edge |
| `test-helpers` | Unchanged: it already depends on `configuration` and `tracing-subscriber` | No new edge |
| Tracker application | Unchanged: it already depends on `tracing-subscriber` | Gains the setup, the formatter and the panic hook |
| `hung-component-tracker`, `e2e-tools` | Unchanged: both already depend on the tracker application | No new edge |

No package gains a dependency on the tracker application, and no package installs a subscriber in
production code: a library emits `tracing` events, and the program that owns the process decides
where they go. One dependency edge is removed and none is added.

Costs:

- The public function `torrust_tracker_configuration::v3_0_0::logging::setup` (and the v2 one)
  disappears. A third-party project that calls it must install its own subscriber. Whether any
  project does (for example the Torrust Index) was not checked.
- From the tracker application the formatter cannot be reused by another package.

Why the shared crate waits:

- The e2e runners of #2769 are in the same crate as the tracker application (`src/console/ci/`,
  `src/bin/`), so they use the formatter from there; they need no new crate.
- `tracker_client` is the only tool that needs a separate crate, because it is its own package. Its
  move to the D4 shape is a follow-up issue, and that issue creates the crate with two real users.
  The daemon needs a level and a format that change after start; the client needs a fixed level, the
  TTY refusal and the stdout result writer. An API designed from the daemon alone would be published
  before the second user's needs are known.
- The formatter and the panic hook are written in modules that depend only on `tracing`,
  `tracing-subscriber` and `serde_json`, so the later move is a file move.
- A reason to create the crate now would be an outside user from the first day, for example the
  Torrust Index adopting the same format soon. The maintainer did not ask for that.

## D4 - The shape of a record

| Alternative | Example | Pros | Cons |
| --- | --- | --- | --- |
| A. Flat; `target` and `level` stand in for `kind` on log records; only non-log records have a `kind` | `{"timestamp":"…","level":"INFO","target":"HTTP TRACKER","message":"Started on: http://0.0.0.0:7070"}` | Short lines | Two kinds of records on one stream, told apart by which keys exist. Relies on "or equivalent" in the ADR |
| B. Flat, with `kind` on every record: `log` for ordinary events, a specific kind for the others (`startup_failure`, `usage_error`, `help`, `panic`) | `{"timestamp":"…","level":"INFO","kind":"log","target":"HTTP TRACKER","message":"Started on: http://0.0.0.0:7070"}` `{"timestamp":"…","level":"INFO","kind":"log","target":"…::jobs::manager","message":"Waiting for job to finish under the shared shutdown deadline","timeout_seconds":25}` `{"timestamp":"…","level":"ERROR","kind":"startup_failure","target":"torrust_tracker","message":"Tracker startup failed","error":"…","exit_code":2}` | Every record of every Torrust tool can be classified the same way (`select(.kind=="startup_failure")`). Event fields are top-level keys, which is what the lifecycle convention of EPIC #2387 reads. Our own formatter: SI-22 adds `run_id` in one place | A constant `"kind":"log"` on most lines (a few bytes). A custom formatter to write and test |
| C. tracing-subscriber's default `json()`, as `tracker_client` does today | `{"timestamp":"…","level":"INFO","fields":{"message":"Started on: …"},"target":"HTTP TRACKER","span":{…},"spans":[…]}` | Almost no code. Same shape as the tool that is already migrated | Values nested under `fields`; span data on every line; no `kind`; no place for `run_id` without a later rewrite. Non-log records (failure, panic) would still be written by a second writer in another shape, as in `tracker_client` |

**Choice: B** (confirmed by the maintainer, 2026-11-26). With D2 every stderr record of the process goes through one formatter, so one shape
with a `kind` discriminator is the simplest contract for a reader. Key order: `timestamp`, `level`,
`kind`, `target`, `message`, then the event fields. Span fields are added only if a span carries
identity (EPIC #2387 convention), never as a `spans` list. No ANSI codes. Consequences:

- **Reserved keys.** `timestamp`, `level`, `kind`, `target` and `message` belong to the formatter.
  An event sets the kind with a `kind` field (that is how `startup_failure` and `panic` are
  written). The formatter writes its own keys first and remembers every key
  it has written in the record. An event field whose name is already in the record (a reserved name,
  or a name an earlier field took) is written with the prefix `field_`, repeated until the name is
  free (`target` becomes `field_target`, or `field_field_target` if the event also has a
  `field_target`), so a record never has a duplicate key and no field is dropped. The two `target = …` events of `udp-core` are a mistake for `target:`;
  issue #3060 (open) already corrects them, so this issue does not touch them.
- **Two shapes in the repository until the others follow.** `tracker_client` keeps
  tracing-subscriber's default shape and its direct `panic` record, and the e2e runners are not
  migrated yet. The ADR names the flat shape as the one for Torrust tools; #2769 uses this
  formatter from the tracker application (see D3), and a follow-up issue moves `tracker_client` to
  it and creates the shared crate. The follow-up is filed
  when the maintainer confirms this decision.

## D5 - Exit codes of a startup failure

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Configuration load, semantic-validation and persistence-requirement errors exit `2`; bind, composition and other runtime failures exit `1`; argument errors stay `2` | Follows the contract (a usage error is `2`). A supervisor can tell "fix the config" from "retry later" | Scripts and restart policies that expect exactly `1` see `2` for configuration errors; the root tests in `invalid_sources.rs` change |
| B. Every startup failure exits `1` | No change for scripts | A documented exception to the contract |

**Choice: A** (already the spec's scope; the maintainer left the choice to the author on 2026-11-26,
with one condition: the binary must behave well both for an operator in a console and when a
container runtime or an orchestrator launches it). How the codes behave there:

| Environment | Effect of exit `1` versus `2` |
| --- | --- |
| Console | `echo $?` tells a configuration mistake (`2`) from a runtime failure (`1`) without reading the record |
| Docker and Podman restart policies (`on-failure`, `always`, `unless-stopped`) | No difference: every non-zero code is a failure, so nothing behaves worse than today. `docker inspect` shows the code |
| Docker Compose | Same as Docker (`restart:` policies); `depends_on` conditions use health, not the exit code |
| Kubernetes `Deployment` or `StatefulSet` | No difference for restarts (`CrashLoopBackOff` on any non-zero code). `kubectl describe pod` shows `Exit Code: 2`, which tells the operator to look at the configuration |
| Kubernetes `Job` (`podFailurePolicy`) | A rule can stop retrying on exit `2` |
| systemd | `RestartPreventExitStatus=2` stops a restart loop on a configuration error, while `Restart=on-failure` keeps retrying a busy port |

Codes `1` and `2` collide with nothing a runtime reserves: `125` to `127` belong to the container
runtime, `128 + n` means killed by signal `n` (`137` for `SIGKILL` or out of memory, `143` for
`SIGTERM`), and Rust's own `101` for a panic in `main` is not used (D6 maps it to `1`). The BSD exit codes (`78` for a
configuration error) were not considered further: the contract ADR fixes `0`, `1` and `2`. A clean
stop after `SIGTERM` or `SIGINT` keeps exit `0`, which `docker stop` and Kubernetes pod termination
rely on. `docs/logging.md` (D7) documents the codes with these examples.

The mapping, variant by variant:

| Error | Exit | Reason |
| --- | --- | --- |
| `app::Error::Setup` with `bootstrap::app::Error::Configuration`, `SemanticValidation` or `PersistenceRequirements` | `2` | The operator must correct the configuration |
| `app::Error::PersistentStatisticsRequirePersistence` | `2` | Same: "Configure `[core.database]` or disable the feature" |
| `app::Error::Setup` with `bootstrap::app::Error::Composition` | `1` | A dependency could not be built at run time |
| `app::Error::InitialPersistenceLoad`, `ServiceStartup` | `1` | Database not available, address in use |
| `app::Error::MissingServiceContainer`, `ComponentRegistration`, `EventFlows` | `1` | Defects in the startup code, not usage errors |

The mapping is a method on the error types (`exit_code()`), with one unit test per variant, so a
new variant cannot be added without choosing its code.

## D6 - Panics

| Alternative | Example | Pros | Cons |
| --- | --- | --- | --- |
| A. In this issue: a panic hook writes one record through the same formatter | `{"timestamp":"…","level":"ERROR","kind":"panic","target":"torrust_tracker","message":"called Option::unwrap() on a None value","location":"src/…:42:9","thread":"main"}` | Closes the last plain-text path; AC2 ("every stderr line is JSON") holds without exceptions. Small, in the same files. Same `kind`, `message` and `location` fields as the `panic` record of `tracker_client` | A little more scope. A backtrace (`RUST_BACKTRACE=1`) goes in one `backtrace` field |
| B. A follow-up issue | — | Smaller PR | AC2 holds only until something panics |
| C. Not done | — | — | Plain text on stderr; breaks the contract |

**Choice: A** (confirmed by the maintainer, 2026-11-26). The hook changes only how a panic is
reported: a panic inside a task still does not stop the process (the job supervisor decides), and a
panic that unwinds out of `main` exits `1` ("could not do its job"), not Rust's `101`, because the
contract allows only `0`, `1` and `2`: `main` catches the unwind of the application future and
returns `1` after the hook has written the record (changed after the review of the spec pull
request; the first version of this decision kept `101`).

## D7 - Where the operator-visible change is documented

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Create `docs/logging.md` with the output format (stream, record shape, `kind` values, a `jq` recipe) and a "Changes in 4.0.0" section (stdout to stderr, `json` as the new default of `trace_style` with the text styles still available, the records that are JSON in every style, no colour codes, exit `2` for configuration errors, migration steps); the 4.0.0 GitHub release notes copy that section | One place in the repository that an operator and the release notes use. EPIC #2387 SI-2 extends the same file (convention, catalogue) and its AC16 expects this section | SI-2's spec must start from an existing file instead of creating it (tell the #2387 coordinator) |
| B. Only the GitHub release notes, written at release time | Nothing to keep in sync | Nothing in the repository until the release; easy to forget; no reference page for the format |

**Requirement from D9 (maintainer):** review every place where the tracker's documentation tells an
operator how to run it or read its logs (`README.md`, `docs/containers.md`, `docs/benchmarking.md`,
`docs/profiling.md`, the crate documentation in `src/lib.rs`, the skills that show log lines) and
make it clear how to get the logs: which stream, how to redirect it, how each style looks.

**Choice: A** (confirmed by the maintainer, 2026-11-26, who added: review the rest of the docs,
because some examples will not work after this change). Examples and descriptions found that this
issue must correct, besides the new page:

| Place | What is there today |
| --- | --- |
| `README.md`, `docs/containers.md`, `docs/profiling.md`, crate docs in `src/lib.rs` (lines 153 to 260) | How to run the tracker; no word on where the logs go. Add the link to `docs/logging.md` and correct any sample output |
| `packages/axum-rest-api-server/src/lib.rs:42-48` | A sample of the start-up output in a text format that is two formats old |
| `docs/benchmarking.md:193-195` | A `[logging]` example with `trace_style = "full"` |
| `packages/configuration/docs/migrate-v2-to-v3.md` | Describes `trace_style` with the default `full` |
| `packages/configuration/src/v3_0_0/mod.rs:220` and `logging.rs` rustdoc | "Default is `Full`", "Redirects log output to stdout" |
| `.claude/skills/setup-dev-environment/SKILL.md:122-124` | Expected start-up lines as text |
| `.claude/skills/manual-http-download-completion-e2e/SKILL.md:54-55`, `manual-udp-download-completion-e2e/SKILL.md:48-49` | Expected `Started on:` lines as text |
| `.claude/skills/manual-torrent-cleanup-e2e/SKILL.md:37-39,78` | `trace_style = "full"` and `torrust-tracker 2>&1 \| tee …` (still works; the lines it then reads change) |
| `.claude/skills/run-tracker-locally/SKILL.md`, `structured-runtime-logging/SKILL.md` | To review: how to run the tracker and how to log |
| `contrib/dev-tools/experiments/dual-stack-sockets/README.md:108-114` | Recorded output of a past experiment: left as history, not a how-to |
| `src/console/ci/e2e/logs_parser.rs` rustdoc | The sample log in the text format |

This repository keeps no `CHANGELOG.md`; the GitHub release is where breaking
changes are recorded, and it copies the section.

## D8 - ANSI colour codes in the text styles

Today the text styles always write colour codes, also into a pipe or a file.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Colours only when stderr is a terminal | The usual convention; clean files and pipes | The output depends on where the process runs |
| B. Never | The same bytes everywhere; nothing to strip in `docker logs`, journald or a file | No colours in a terminal |
| C. Always, as today | No change; `full` looks exactly as before | Escape codes in every log file and collector |
| D. A new key, for example `logging.color = "auto" \| "always" \| "never"`, default `auto`, which could also honour the `NO_COLOR` environment variable | The operator decides. `always` restores the old look; `never` covers a container with a TTY | One more key to document and test. With `trace_style = "json"` it does nothing |

**Choice: B** (maintainer, 2026-11-26). `json` never has colour codes in any case. The maintainer's
reason: the logs are now read by tools and AI agents much more than by people, so colours are not
worth implementation time today. Alternative D is postponed, not rejected: it is added when a real
need is shown, for example an environment where, for privacy or security, a person must read the
logs directly. Until then the colour support that tracing-subscriber already has stays switched off
(`with_ansi(false)`); no new option is built in this issue.

## D9 - The output stream

Today all logs go to stdout. The contract says a daemon writes nothing on stdout.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Always stderr, in every style | One rule; stdout is always empty. An operator who needs the logs on stdout redirects in the shell, the systemd unit or the container command (`2>&1`). `docker logs`, logging drivers and journald capture both streams, so most deployments see no difference | A deployment that redirects only stdout (`torrust-tracker > tracker.log`) gets an empty file until it changes the redirection |
| B. A new key, for example `logging.output = "stderr" \| "stdout"`, default `stderr` | The old behavior can be restored from the configuration alone | A second exception to the contract. Records written before the configuration is loaded still go to stderr, so one run is split across two streams. Four styles times two streams to test and document. No capability that a redirection does not give |
| C. The stream follows the style: `json` on stderr, text styles on stdout | `trace_style = "full"` alone restores the old behavior | A hidden coupling; JSON on stdout or text on stderr is impossible; the run is split across streams |

**Choice: A** (maintainer, 2026-11-26), with a requirement: the issue also reviews the tracker's
documentation so that it is clear to an operator how to get the logs from the tracker (direct run,
redirection to a file, `2>&1`, container, systemd). See D7.

## Combinations

What stderr contains, by configured style and situation. Stdout is empty in every cell. The examples
are illustrative (real messages, shortened timestamps); the text lines are tracing-subscriber's
formats without colour codes (D8).

### Summary

| Situation | `json` (default) | `full` | `pretty` | `compact` | Exit |
| --- | --- | --- | --- | --- | --- |
| `--help` | JSON `help` record | same as `json` | same as `json` | same as `json` | `0` |
| Argument error (`--unknown`, `-c ""`) | JSON `usage_error` record | same as `json` | same as `json` | same as `json` | `2` |
| Configuration error (missing file, bad TOML, unknown `trace_style` value, semantic validation) | JSON `log` records of configuration loading, then one JSON `startup_failure` | same as `json`: the style was never applied | same as `json` | same as `json` | `2` |
| Normal run | JSON only | JSON until the configuration is loaded, then `full` text | JSON, then `pretty` text | JSON, then `compact` text | `0` at a clean stop |
| Runtime startup failure (port in use) | JSON `log` records, then one JSON `startup_failure` | JSON, then text, then one JSON `startup_failure` | same pattern | same pattern | `1` |
| Panic | one JSON `panic` record | one JSON `panic` record among the text | same | same | `1` if the panic unwinds out of `main`; the process continues if a task panicked |
| Tools that read the logs (e2e runner, test harness, `jq`) | Work | Do not work | Do not work | Do not work | |

### `--help` and argument errors (every style)

```text
$ torrust-tracker --help
{"timestamp":"…","level":"INFO","kind":"help","target":"torrust_tracker","message":"Usage: torrust-tracker [OPTIONS]\n\nOptions:\n  -c, --config-toml-path <CONFIG_TOML_PATH>  Path to the TOML configuration file to load\n  -h, --help  Print help"}
exit 0

$ torrust-tracker --unknown
{"timestamp":"…","level":"ERROR","kind":"usage_error","target":"torrust_tracker","message":"unexpected argument '--unknown' found","exit_code":2}
exit 2
```

### Configuration error (every style)

```text
$ torrust-tracker -c /nonexistent.toml
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_configuration","message":"Loading base configuration from explicit configuration file","path":"/nonexistent.toml"}
{"timestamp":"…","level":"ERROR","kind":"startup_failure","target":"torrust_tracker","message":"Tracker startup failed","error":"Tracker setup failed. … Unable to load explicit configuration file `/nonexistent.toml`: No such file or directory (os error 2)","exit_code":2}
exit 2
```

The same happens with `trace_style = "fancy"` (unknown value): the configuration is rejected, so
no style is applied and the failure is JSON.

### Normal run, `json` (default, or `trace_style = "json"`)

```text
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_configuration","message":"Loading base configuration from explicit configuration file","path":"tracker.toml"}
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_lib::bootstrap::logging","message":"Logging initialized","trace_filter":"info","trace_style":"json"}
{"timestamp":"…","level":"INFO","kind":"log","target":"HTTP TRACKER","message":"Started on: http://0.0.0.0:7070"}
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_lib::bootstrap::jobs::manager","message":"Waiting for job to finish under the shared shutdown deadline","timeout_seconds":25}
```

### Normal run, `trace_style = "full"`

```text
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_configuration","message":"Loading base configuration from explicit configuration file","path":"tracker.toml"}
2026-11-26T10:00:00.120000Z  INFO torrust_tracker_lib::bootstrap::logging: Logging initialized trace_filter="info" trace_style="full"
2026-11-26T10:00:00.123456Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070
2026-11-26T10:00:05.000000Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=25
```

The first line is JSON because it is written before the style is known. From "Logging initialized"
on, the lines are the same text as today, without colour codes and on stderr.

### Normal run, `trace_style = "pretty"`

```text
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_configuration","message":"Loading base configuration from explicit configuration file","path":"tracker.toml"}
  2026-11-26T10:00:00.123456Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070

  2026-11-26T10:00:05.000000Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline, timeout_seconds: 25

```

One event takes several lines (a blank line after each, and one more line per enclosing span), so
this style cannot be read line by line.

### Normal run, `trace_style = "compact"`

```text
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_configuration","message":"Loading base configuration from explicit configuration file","path":"tracker.toml"}
2026-11-26T10:00:00.123456Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070
2026-11-26T10:00:05.000000Z  INFO torrust_tracker_lib::bootstrap::jobs::manager: Waiting for job to finish under the shared shutdown deadline timeout_seconds=25
```

It differs from `full` only inside spans: `full` prints the span names and their fields before the
target, `compact` prints the span fields once, after the event's own fields.

### Runtime startup failure with a text style (`full`, port 7070 in use)

```text
{"timestamp":"…","level":"INFO","kind":"log","target":"torrust_tracker_configuration","message":"Loading base configuration from explicit configuration file","path":"tracker.toml"}
2026-11-26T10:00:00.120000Z  INFO torrust_tracker_lib::bootstrap::logging: Logging initialized trace_filter="info" trace_style="full"
2026-11-26T10:00:00.123000Z  INFO HTTP TRACKER: Starting on: http://0.0.0.0:7070
{"timestamp":"…","level":"ERROR","kind":"startup_failure","target":"torrust_tracker","message":"Tracker startup failed","error":"Configured HTTP tracker startup failed. … Address already in use (os error 98)","exit_code":1}
exit 1
```

With `json` every line of this run is JSON. With a text style the stream mixes text lines and JSON
lines; a reader finds the reason of the exit with `grep '"kind":"startup_failure"'`.

### Panic with a text style

```text
2026-11-26T10:00:07.000000Z  INFO HTTP TRACKER: Started on: http://0.0.0.0:7070
{"timestamp":"…","level":"ERROR","kind":"panic","target":"torrust_tracker","message":"called `Option::unwrap()` on a `None` value","location":"src/…:42","thread":"tokio-runtime-worker"}
```

### Consequences to keep in mind

- **A stream with a text style is mixed.** The first records and the two "why it died" records are
  JSON, the rest is text. This is the price of knowing the style only after the configuration is
  read and of keeping the exit reason parseable. The docs say so in the description of
  `trace_style`.
- **`trace_filter` applies in every style** once the configuration is loaded. Before that the level
  is `info`, so the "Loading … configuration" records appear even with `trace_filter = "error"`.
- **Tools need `json`.** The e2e log parser, the native test harness and the SI-20 check of EPIC
  #2387 read JSON only; the configs they use do not set a text style.
- **SI-22 (`run_id` on every record)** adds the field in the JSON formatter. Whether the text styles
  also show it is SI-22's decision; the text formatters are tracing-subscriber's and take the field
  only through a span.
- **The upgrade is still a visible change** for a deployment that keeps `trace_style = "full"`: its
  lines move from stdout to stderr and lose the colour codes, and the few JSON records above appear
  among them. A deployment that drops the key gets JSON.

## Delivery: one issue or several

Raised by the maintainer on 2026-11-26 while confirming D2: installing the logging first could be its
own sub-issue, and this issue may be too large for one pull request. To decide when the spec is
complete, before any code:

- The maintainer's practice: steps of one process that is long are delivered as several pull
  requests of the same issue; tasks that are related but independent become sub-issues of an EPIC.
- This issue is already a sub-issue of EPIC #2758 (row 7).
- The spec's Implementation Plan must say which tasks land in which pull request.

**Proposal in the spec (to be confirmed by the maintainer in the review of the spec pull request):**
one issue, four pull requests, because every part needs the one before: (1) the ADR, the missing
reproduction evidence and the move of the logging setup, with no change of format; (2) the JSON
formatter, stderr, the `json` default and the readers; (3) the startup-failure record, the exit
codes, the argument and help records and the panic hook; (4) the documentation, the manual
verification and the follow-up issues.

## ADR

D1, D2, D4, D6 and D8 together define how a long-running daemon meets the output contract. They become
one ADR in the implementation PR (`create-adr` skill), linked from `docs/logging.md`. This file stays
as the history of how they were chosen.
