---
semantic-links:
  skill-links:
    - create-adr
  related-artifacts:
    - .claude/skills/create-adr/SKILL.md
    - docs/adrs/20260519000000_define_global_cli_output_contract.md
    - docs/adrs/20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md
    - docs/issues/open/2766-2758-daemon-json-logging/ISSUE.md
    - docs/issues/open/2766-2758-daemon-json-logging/decisions.md
---

<!-- skill-link: create-adr -->

# Write the Daemon Logs as JSON on Stderr by Default

## Scope

Repository-level decision: it applies the global CLI output contract to the `torrust-tracker`
daemon, which is an operator-visible behavior of the whole application, and it changes how the
tracker application, the configuration crate and the tools that read the daemon's logs fit
together. It therefore lives in the root collection, `docs/adrs/`. It follows
[Define the Global CLI Output Contract](20260519000000_define_global_cli_output_contract.md), as
amended by
[Scope the CLI Output Contract and Keep Exit Codes to 0, 1 and 2](20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md).

## Description

The contract classifies the daemon as `no-stdout-result`: nothing on stdout, tracing events as
NDJSON on stderr, every record with a `kind`, exit codes `0`, `1` and `2`. Until this decision the
daemon did none of that:

- the logging was installed after the configuration was loaded and validated, so a failure before
  that point could not be logged and `main` wrote it with `eprintln!`; the records of the
  configuration loading were lost;
- the logs went to stdout, in the `full` text style, with ANSI colour codes, chosen by the
  configuration key `logging.trace_style` (`full`, `pretty`, `compact`, `json`; default `full`);
- argument errors and `--help` were plain text written by `clap`, and a panic was plain text
  written by the Rust runtime.

The key `trace_style` is public and every shipped configuration sets it to `full`. The default
configuration must follow the contract, but the operator must not be forced to use only that
format: a deployment that parses the old text must be able to keep doing so. The tools of this
repository that read the logs (the end-to-end runners, the native test harness, the lifecycle
tests) need a format a program can read.

The maintainer decided the migration detail on 2026-11-26, one question at a time. The record of
the alternatives, with their pros and cons and example output, is
[decisions.md](../issues/open/2766-2758-daemon-json-logging/decisions.md) in the folder of the
issue 2766; this ADR states what was decided.

## Agreement

1. **JSON is the default; the text styles are an opt-in.** `trace_style` stays in the v3
   configuration schema, with the values `json`, `full`, `pretty` and `compact`, and its default
   becomes `json`. The default follows the contract. A text style is the operator's explicit choice
   to leave it for the log records; this ADR records that as an exception of the daemon to the rule
   "no plain text at any verbosity". An unknown value is a configuration error. The shipped
   configurations do not set a text style. The tools of the repository read JSON only.
2. **Every style writes to stderr, and stdout stays empty.** An operator who needs the logs on
   stdout redirects them (`2>&1`) in the shell, the unit file or the container command; container
   runtimes and `journald` collect both streams. There is no configuration key for the stream.
3. **The logging is installed first thing in `main`**, before the arguments are parsed, in JSON at
   level `info`. When the configuration is loaded, its `trace_filter` sets the level and its
   `trace_style` sets the style of the log records. Every record of the process is built by one
   formatter, and the records written before the configuration is known (help, usage errors, the
   "Loading … configuration" records, a configuration failure) are JSON in every style.
4. **A record is flat and has a `kind`.** The keys, in this order, are `timestamp`, `level`,
   `kind`, `target`, `message`, and then the event fields as top-level keys. The kinds are `log`
   (an ordinary `tracing` event), `startup_failure`, `usage_error`, `help` and `panic`. The
   formatter owns the keys `timestamp`, `level`, `kind` and `target`; the key `message` is the
   event's own `message` field. An event field whose name is already in the record is written with
   the prefix `field_`, repeated until the name is free, so a record never has a duplicate key. A
   record has no `spans` list: a span contributes fields only when it carries an identity.
5. **The records that say why the process stopped are always JSON and ignore `trace_filter`.** The
   kinds `help`, `usage_error`, `startup_failure` and `panic` are not `tracing` events: the same
   formatter code builds them and they are written to stderr directly, so `trace_filter = "off"`
   hides none of them and they are JSON in every style. Only `log` records pass through the
   subscriber and its filter.
6. **A panic is a record.** A panic hook writes one `panic` record. A panic that unwinds out of
   `main` exits `1`, because the contract allows only `0`, `1` and `2`; a panic inside a task does
   not stop the process.
7. **No ANSI colour codes in any style.** A configuration key for colours is postponed until an
   environment shows the need, for example one where a person must read the logs directly.

How the decisions are carried out is in the specification of issue #2766: the setup moves from the
configuration crate to the tracker application (the configuration crate keeps only data and loses
its `tracing-subscriber` dependency); startup failures exit `2` for argument and configuration
errors and `1` for runtime failures; `docs/logging.md` documents the format, the styles, the exit
codes and the changes of 4.0.0.

### Alternatives Considered

- **Always JSON, with `trace_style` accepted and ignored, or removed, or restricted to `json`.**
  The cleanest result for the contract, but it takes away the choice of the operator, which the
  maintainer wants to keep. Rejected for the `trace_style` key; kept for the default.
- **A text format on a terminal and JSON otherwise.** The output would depend on where the process
  runs, which surprises operators and tests, and it breaks the contract. Rejected.
- **A configuration key for the stream, or a stream that follows the style.** It adds
  combinations without adding capability: a redirection already chooses the stream. Rejected.
- **`tracing-subscriber`'s built-in JSON layout (as `tracker_client` uses today).** The values are
  nested under `fields`, span data repeats on every line, there is no `kind`, and the failure and
  panic records would need a second writer in another shape. Rejected for the daemon; moving
  `tracker_client` to the shape above is a follow-up of issue #2766.
- **The failure and panic records follow the configured style.** The text stream would be uniform,
  but no tool could read the reason of an exit, and a configuration failure would still be JSON,
  so failures would look different depending on when they happen. Rejected.

## Consequences

Positive:

- With the default configuration the daemon meets the contract: stdout is empty, every stderr line
  is a JSON object with a `kind`, and the reason a process stopped is always available to a
  supervisor, whatever `trace_filter` and `trace_style` say.
- One formatter builds every record, so a field that every record needs, such as a run identifier,
  is added in one place.
- The configuration crate is data only and no package gains a dependency on the tracker
  application: libraries emit `tracing` events and the program that owns the process decides where
  they go.

Negative:

- The upgrade is visible. A deployment that keeps `trace_style = "full"` still sees its lines move
  from stdout to stderr and lose the colour codes, and the JSON records named above appear among
  them: it is not a text-only stream. A deployment that drops the key gets JSON.
- A text style leaves the contract for the log records. The tools of the repository, and any log
  collector that expects JSON, need `json`.
- A configuration with a text style produces a mixed stream (JSON and text), by design.
- A third-party project that called the removed `logging::setup` of the configuration crate must
  install its own subscriber.

## Affected Code

- `src/bootstrap/logging.rs`: the installation of the logging, the level and the style set from the
  configuration.
- `src/main.rs`, `packages/hung-component-tracker/src/main.rs`, `src/console/profiling.rs`: the
  entry points that install the logging first.
- `packages/configuration/src/v3_0_0/logging.rs`: the data of the logging configuration
  (`trace_filter`, `trace_style`).

## Date

2026-11-27

## References

- Issue #2766: the `torrust-tracker` daemon: JSON logs on stderr and JSON startup failure.
- EPIC #2758: migrate the CLI tools to the global output contract.
- EPIC #2387: the content of the lifecycle log lines, which follows this decision.
- [Define the Global CLI Output Contract](20260519000000_define_global_cli_output_contract.md)
- [Scope the CLI Output Contract and Keep Exit Codes to 0, 1 and 2](20261106102632_scope_the_cli_output_contract_and_keep_exit_codes_to_zero_one_two.md)
