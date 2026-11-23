---
doc-type: decision-record
issue-spec: docs/issues/open/2387-review-tracker-lifecycle-logs/EPIC.md
last-updated-utc: "2026-11-23 16:54"
---

# Decisions of EPIC #2387

## Purpose

This file records what the maintainer decided for the lifecycle logs, the alternatives that were considered, their pros and cons, and the reason for each choice. The decisions were taken on 2026-11-21 and 2026-11-23, in one question-and-answer session between the maintainer (`josecelano`) and Claude Code (Fable 5.2), after reading the fresh log captures in [baseline-lifecycle-logs.md](baseline-lifecycle-logs.md).

Each sub-issue of the EPIC implements one or more decisions and names them by ID (`D1` to `D12`). SI-2 moves the lasting rules into an ADR and into `docs/logging.md`. This file stays as the history of how they were chosen.

A decision here is the starting point of a sub-issue, not a final design. If a sub-issue finds that a decision does not hold, it stops, records the finding and asks the maintainer.

## Rules the maintainer gave for all decisions

These rules came up while answering and apply to every sub-issue:

1. **Prefer structured fields.** A fact that a tool may read is an event field, never only words in the message.
2. **The log maps what happens inside the application.** Do not merge two lines when something can fail between the two moments they report. A crash must leave the last reached state in the log. Merge lines only when they report the same moment, or when nothing can go wrong between them.
3. **Effort is not a reason to choose the smaller design.** When the better design costs more work, do it before 4.0.0 and split it into small sub-issues.
4. **Record the alternatives.** Every decision keeps its alternatives, pros, cons and reason in the specification, and the rules that result are documented where future contributors find them (ADR, documentation, skill, rustdoc).

## Facts found before deciding

### Four addresses, four names

The code has four related values. Today the logs mix them under five field names.

| Value | Where it comes from | Example | Can the port be `0`? |
| --- | --- | --- | --- |
| Configured bind address | `bind_address` in the configuration file | `0.0.0.0:0` | Yes |
| Bound socket address | The operating system, after `bind` | `0.0.0.0:41237` | No |
| Service binding | `ServiceBinding` (crate `torrust-net-primitives`): protocol and bound socket address, as a URL | `udp://0.0.0.0:41237` | No |
| Public URL | `public_url`, optional, in the configuration file | `https://tracker.example.com` | Does not apply |

The health check API returns three of them for every service: `service_binding`, `binding` (the bound socket address) and `public_url`.

`ServiceBinding` prints itself through `Url`. An HTTP binding gets a trailing slash (`http://0.0.0.0:7070/`) and a UDP binding does not (`udp://0.0.0.0:6969`).

### Programs that read the log text

| Reader | What it reads | File |
| --- | --- | --- |
| E2E runner | The service URLs, with a regular expression on `Started on: udp://…` and `Started on: http://…`, on lines selected by the target text (`UDP TRACKER`, `HTTP TRACKER`, `HEALTH CHECK API`) | `src/console/ci/e2e/logs_parser.rs` |
| Native tracker test fixture | The health check address after `Started on: http://`, on lines with `HEALTH CHECK API` | `tests/common/native_tracker/health.rs` |
| Signal tests | Exact shutdown messages and `Waiting for job to finish` | `tests/lifecycle/signals.rs` |
| Shutdown policy tests | `timeout_seconds=N`, the success message and the absence of `Started on:` | `tests/lifecycle/shutdown_policy.rs`, `tests/common/native_tracker/mod.rs` |

Every change to these lines breaks one of these readers in CI, so each sub-issue that changes a line updates its readers in the same pull request.

### Targets in use

Named targets: `UDP TRACKER`, `HTTP TRACKER`, `HEALTH CHECK API`, `UDP CLIENT`, `API`, `EVENTS`, `TRACKER_CORE`, `METRICS` (from the crate `torrust-metrics`), the swarm coordination registry target, and `torrust_tracker` for some process lines. Every other lifecycle line has the Rust module path as target, for example `torrust_tracker_lib::bootstrap::jobs::manager` and `torrust_tracker_axum_server::signals`.

### How a listener starts today

1. The job logs `Starting UDP tracker instance` (HTTP: similar) with the configured `bind_address`, before the socket is bound.
2. The server binds the socket.
3. The server logs `Starting on: <url>` and `Started on: <url>` one after the other, both after the bind. `Starting on:` is therefore already false when it is printed.
4. The server logs `Started UDP tracker` (HTTP: `Started HTTP tracker`).
5. The server registers the service in the `Registar`, which makes it visible to the health check API. Registration happens after the `Started` line.

### The statistics-disabled warning

With `tracker_usage_statistics = false`, three statistics listeners (`udp-core`, `udp-server`, `http-core`) log one `WARN` for every event they ignore:

```text
WARN UDP TRACKER: Ignoring UDP server event from an unknown or metrics-disabled listener configuration_instance_id=ConfigurationInstanceId { service_role: UdpTracker, instance_index: 0 }
```

The code takes the same branch for "this listener has statistics disabled" and "this listener is unknown". A UDP request publishes one to four events and an HTTP announce or scrape publishes one, so a tracker that serves traffic with statistics disabled writes `WARN` lines for every request. The capture shows eight of them for the two UDP probes of one health check request.

## D1 - Order between this EPIC and the JSON log format (#2766)

Issue #2766 (EPIC #2758) changes the daemon's log output to JSON lines on stderr. It rewrites the logging setup and the same log parsers this EPIC touches.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. This EPIC first, then #2766 | The content (targets, field names, messages, levels) is fixed once, in the text format that people read while reviewing. #2766 then changes only the format. The parsers move to structured fields before the format changes, so #2766 changes how a field is read, not what is read | #2766 waits. The parsers are edited twice, once in each issue |
| B. #2766 first, then this EPIC | The parsers are rewritten once for JSON | The convention is reviewed in JSON, which is harder to read. #2766 has to keep parsing `Started on:` message text inside JSON, which is the worst mix |
| C. Do both together | One change | Too large to review. Mixes a format decision with content decisions |

**Decision:** A. This EPIC goes first. The specification of #2766 and its row in EPIC #2758 say so.

**Reason:** the convention is independent of the output format, so it can be decided and reviewed first. After SI-11 the readers depend on fields, which survive the move to JSON.

**Consequence:** the convention must not depend on the text format. It speaks about targets, message text, field names and levels only. The one item that needs the formatter (`run_id` on every record, D10) is ordered after #2766.

## D2 - Targets

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Keep the mix of named targets and module paths | No work. Module paths are exact | An operator must know Rust module names to filter. Moving a function changes a filter key. Two styles side by side |
| B. Named targets on every lifecycle line, and one naming style | One vocabulary of a few names that an operator can list and filter on. Stable across refactors | Every lifecycle log call names its target. Renaming existing targets breaks filters and the test readers |
| C. Add named targets only where they are missing; keep today's names | Half the work. No break for existing names | Keeps `UDP TRACKER` (with a space) next to `TRACKER_CORE` (with an underscore), and the ambiguous `API` |

**Decision:** B, as two separate tasks: first add the missing targets (SI-5), then rename the existing ones (SI-6).

**Reason:** the two tasks have different risk. Adding a target breaks nothing. Renaming breaks every reader that matches the old name, so it is reviewed alone.

Details decided:

- **Style:** uppercase with underscores, for example `UDP_TRACKER`. A name without spaces needs no quotes in a filter or in a JSON query, and two targets already use it (`TRACKER_CORE` and `SWARM_COORDINATION_REGISTRY`).
- **New targets:** `JOB_MANAGER` for the job manager; `APPLICATION` for process-level lines (logging initialized, configuration loaded, signal handlers installed, shutting down, stopped, startup failed, and the new lines of D10); one target for each periodic job.
- **`TRACKER` was rejected** as the name for process-level lines: everything in the tracker is "tracker", so the name says nothing. `APPLICATION` names the layer that owns those lines.
- **Shared HTTP shutdown code** (`torrust_tracker_axum_server::signals`) logs under the target of the service it stops, not under its own module path.
- **Renames:** `UDP TRACKER` to `UDP_TRACKER`, `HTTP TRACKER` to `HTTP_TRACKER`, `HEALTH CHECK API` to `HEALTH_CHECK_API`, `UDP CLIENT` to `UDP_CLIENT`, `API` to `REST_API`, `torrust_tracker` to `APPLICATION`.

## D3 - Identity of a listener

### D3a - Field names

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. One field, `service_binding`, on every line after the bind; `bind_address` only before the bind | One name to filter on. It is the value the health check API already calls `service_binding`. The pre-bind line shows the configured value, which is all that exists then | Lines that print only a socket address today change |
| B. One field with the bound socket address (`local_addr`) | Shorter value | Loses the protocol. Two services may not share a port, but an operator still has to know which protocol a port serves |
| C. Both a socket address field and a URL field | Nothing lost | Two fields for one fact; the socket address is inside the URL |

**Decision:** A.

- `service_binding` is the only name for a bound listener. Its value is the URL, not quoted, in one form. It is an event field on every line about a bound listener, never only words in the message.
- `bind_address` appears only on the `Starting` line, before the bind, with the configured value (which may have port `0`).
- `local_addr`, `address` and `bind_addr` are removed.
- `public_url` is added to the `Started` line when it is configured.
- The bound socket address has no field of its own.
- `service_binding` is always logged together with `service_role` and `instance_index`.
- The convention document carries the four-address table above as a glossary.

**Reason:** the three names in use today describe two different values, and a reader cannot tell which. The URL holds the protocol and the real port, so it identifies a listener by itself.

### D3b - Trailing slash

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Keep what `Url` prints | No work | `http://0.0.0.0:7070/` and `udp://0.0.0.0:6969` are two forms of one field. A filter by exact value has to know which protocol adds a slash |
| B. No trailing slash in any log line | One form. A value can be compared as text | Differs from the health check JSON, which keeps the slash |
| C. Trailing slash everywhere | One form | Not what the UDP URL looks like anywhere else |

**Decision:** B, in every log line, request logs included. The health check JSON API keeps its slash: it is a public API and its change belongs to the `torrust-net-primitives` fix below.

### D3c - Where the slash is removed

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Leave the slash | No work | Rejects D3b |
| B. A display helper in this repository, used at every log site, kept for good | Independent of the other crate's release | About 25 log sites call a helper for something the type should do. The cause stays |
| C. The helper now, an issue on `torrust-net-primitives` at once to fix `Display`, and the helper deleted when the fixed crate is used | The logs are right now and the cause is fixed where it is | Two steps, and a temporary helper |
| D. Wait for the crate fix | No temporary code | The EPIC waits on another repository's release |

**Decision:** C. First decided as B to keep the scope small, then reopened under rule 3: the only argument against C was effort.

### D3d - Job names

Job names of services today hold the index and the configured bind address, for example `udp_instance_0_0.0.0.0:6868` and `http_instance_1_0.0.0.0:7171`.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Leave the names | No work. No test changes | A job name holds a configured address, which may be port `0`, so it can differ from the real listener. It says `instance` where the fields say `udp_tracker`. The identity of a service is in the name's text, not in fields |
| B. Name a service job by role and index (`udp_tracker_0`, `http_tracker_1`), and add `service_role` and `instance_index` fields to the job manager lines of service jobs | The name is stable and matches the configuration order. The job manager lines can be joined with the service lines by the same two fields | Test changes. An operator who reads the address from the job name now reads it from the service's `Started` line |

**Decision:** B. First decided as A ("leave job names alone"), then reopened under rule 3.

## D4 - Start lines of a service

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Keep the four lines of today | No work | `Starting on:` is printed after the bind. Two lines print the same URL in the message text. `Started` is printed before the service is registered |
| B. One line per service: `Started` | Short log | If the process dies between the bind and the end of the start, the log shows nothing about how far it got. Rejected by rule 2 |
| C. Three state lines, each printed when its state is reached | Each line is true when printed. A crash leaves the last reached state. All facts are fields | Three lines for each service |

**Decision:** C.

| Line | When | Fields |
| --- | --- | --- |
| `Starting <service>` | Before the bind | `service_role`, `instance_index`, `bind_address`, and the service's settings |
| `Bound <service> socket` | After the bind succeeds | `service_role`, `instance_index`, `service_binding` |
| `Started <service>` | After the registration in the `Registar` succeeds | `service_role`, `instance_index`, `service_binding`, `public_url` when configured |

- `Bound … socket` replaces `Starting on:`, `Started on:` and the old `Started` line.
- `Started` moves to after the registration, because only then can the health check see the service.
- The text `Started on:` is removed. The E2E parser and the test fixture read `service_binding=` from the `Started` line that has the wanted `service_role=`.
- `service_role` and `instance_index` are event fields on these lines, not only span fields, so a reader does not depend on the span being printed.
- The REST API and the health check API follow the same three lines. SI-2's inventory verifies their start sequence, which was not traced line by line in this session.

**Reason:** rule 2. Between "socket bound" and "service registered" the receive loop is spawned and the registration can fail, so they are two moments.

## D5 - Spans

Today `#[instrument]` captures every argument with `Debug`, which prints values such as `cancellation_token=CancellationToken { is_cancelled: false }` and `request_drain_deadline=5s` in front of every line of a job. Nested functions add their own spans.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Keep the spans | No work | Internal types in every line. Long lines. The same setting is repeated on every line |
| B. Remove all lifecycle spans | Shortest lines | Lines logged by shared code lose the service identity |
| C. One span per service job with identity only | Every line of a job says which service it belongs to, and nothing else | Settings must be logged somewhere else |

**Decision:** C.

- A lifecycle span carries only `service_role` and `instance_index`. Every other argument is skipped.
- A setting is logged once, as a field of the `Starting` line (for example `connection_id_validation`, `cookie_lifetime_in_secs`, `udp_request_drain_in_secs`).
- A nested span does not repeat the identity. A span without identity carries no fields.
- One span per service job stays (`start_job`). `#[instrument]` is removed from the inner functions (`start_with_cancellation`, `start_v1`) and from `complete_startup`.

**Reason:** a span is context for every line inside it, so it should hold only what identifies the work. A setting is a fact about one moment, the start.

## D6 - Durations

Today the same kind of value appears as `timeout_seconds=25`, `drain_timeout=15s`, `request_drain_deadline=5s` and `in 15s` inside a message.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. `Debug` of `Duration` (`15s`, `1.5s`, `250ms`) | Easy for a person | The unit changes with the value, so a tool must parse it. Not a number in JSON |
| B. A number, with the unit in the field name | A number in text and in JSON. The name says the unit. Matches the configuration keys, which already end in `_in_secs` | Longer names |
| C. A number with a fixed unit for all fields (milliseconds) | One unit | `deadline=25000` is unlike the configuration value `25`, so the operator converts |

**Decision:** B, with four rules.

1. A duration is a number, and its unit is in the field name. Never `Debug` of `Duration`, and never only in the message.
2. The suffix is `_in_<unit>`: `_in_secs`, `_in_ms`.
3. A configured duration is logged under the name of its configuration key, with the section as prefix when the key alone is ambiguous: `shutdown_deadline_in_secs=25`, `http_drain_in_secs`, `udp_request_drain_in_secs`, `cookie_lifetime_in_secs`, `ip_bans_reset_interval_in_secs`.
4. A measured duration is `elapsed_in_ms`, an integer.

Two follow-ups are separate sub-issues because they are outside lifecycle lines:

- Request logs: `latency_ms` becomes `latency_in_ms` (SI-19).
- Configuration keys without a unit (SI-4): `max_peer_timeout` becomes `max_peer_timeout_in_secs`, `inactive_peer_cleanup_interval` becomes `inactive_peer_cleanup_interval_in_secs`, and `cookie_lifetime` of a UDP tracker becomes `cookie_lifetime_in_secs`. The first two are integers of seconds already. `cookie_lifetime` is a `Duration`, written in the configuration as a table with `secs` and `nanos`, so its rename also changes its value to an integer of seconds. This changes the configuration schema.

These three keys are the only exception to rule 3: until SI-4 lands, the log uses the name the key will have (`max_peer_timeout_in_secs`, `inactive_peer_cleanup_interval_in_secs`, `cookie_lifetime_in_secs`), not the key of today. The maintainer approved the first two renames on 2026-11-23. The third was found by the review of PR 3010 and follows the same decision; SI-4's specification pull request asks him to confirm it.

**Reason:** rule 3 lets an operator search the log for the same word that is in the configuration file.

## D7 - Metrics registration and the configuration dump

### D7a - Metrics registration lines

The crate `torrust-metrics` logs one `INFO` line for every metric it describes, 28 in the capture, with `Debug` values such as `description=Some(MetricDescription("…"))`.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Leave them | No work | 28 of about 250 lines of a start are a list an operator does not need. `Debug` wrappers |
| B. Hide them with a filter on the `METRICS` target in the tracker's logging setup | No change in the other crate | Hides a problem instead of fixing it. Also hides any useful line of that target |
| C. Fix the crate: `DEBUG` level, `Display` values, no `Option` wrapper; here, bump the dependency and log one summary line | The cause is fixed for every user of the crate. The start keeps one line that says registration happened | Waits for a release of the other crate |

**Decision:** C. An issue is filed on `torrust-metrics` at once. SI-21 bumps the dependency and adds `Registered metrics counters=<n> gauges=<n>`. The numbers are the real counts at that moment.

### D7b - The configuration dump

The tracker logs `Configuration:` followed by about 110 lines of indented JSON, as one event that spans many lines.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Leave it | Easy to read for a person | One event over 110 lines breaks every line-based tool. In JSON output it becomes one field with escaped newlines |
| B. Remove it | Short log | The operator loses the effective configuration, which answers most support questions |
| C. One line with compact JSON in a field, plus one summary line | One event is one line. The full configuration is still there. The summary answers "what is running" at a glance | The compact line is long |

**Decision:** C.

```text
APPLICATION: Loaded configuration configuration={compact json}
APPLICATION: Enabled services udp_trackers=2 http_trackers=2 rest_api=true health_check_api=true database_driver=sqlite3 private=false listed=false
```

Secrets stay masked as they are today.

## D8 - The statistics-disabled warning

See the fact above.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Leave it | No work | A supported configuration writes `WARN` lines for every request. Real warnings are lost among them. Cost in the hot path |
| B. Lower the line to `DEBUG` | Small change | Still one line per event at `DEBUG`. Still does not tell "disabled" from "unknown", and "unknown" is a bug that should be loud |
| C. Split the two cases and report the disabled state once | The normal case is silent. The bug case is an `ERROR` with fields. The operator still sees, once, that statistics are disabled | Touches three listeners |

**Decision:** C, as a bug fix done early (SI-1).

- Statistics disabled for the listener (`Some(false)`): no log per event (`TRACE` at most).
- Listener unknown (`None`): `ERROR`, with `service_role` and `instance_index` fields, because it means the event came from a listener the policy does not know.
- The disabled state is reported once at startup: `tracker_usage_statistics` on each service's `Starting` line, and counts on the statistics listener's start line (for example `enabled_instances=0 disabled_instances=2`).
- A regression test starts a tracker with statistics disabled, sends a request and asserts that no `WARN` is logged.

**Reason:** a log line per request for a normal state is a defect in production, not a style issue.

## D9 - Levels

Issue #1166 discusses levels for request handling. This decision covers lifecycle lines.

| Level | Meaning for a lifecycle line |
| --- | --- |
| `ERROR` | Something failed and the outcome is wrong: the process exits with a non-zero code, or a configured service is not running |
| `WARN` | The tracker continues correctly, but the operator should know |
| `INFO` | A state change, at most once for each component and transition |
| `DEBUG` | How a transition happened |
| `TRACE` | One line per event |

Alternatives for the rule itself: keep levels as each author chose them (no work; the same kind of event is `WARN` in one place and `INFO` in another), or the table above (one test for each line: what does the operator have to do?). The table was chosen.

**Corollary:** a clean start and a clean stop have no `WARN` and no `ERROR`, except warnings about a deliberate configuration, which the convention lists one by one.

Changes that follow:

- `Job panicked`, `Job reported a failure` and `Job was aborted after the shutdown deadline` go from `WARN` to `ERROR`: the process exits `1` in these cases.
- The lines `Job will be asked to stop after …`, one for each dependency edge of the shutdown plan, go from `INFO` to `DEBUG`: they say how the stop is ordered, not that a state changed.
- `!! Shutting down … !!` becomes a plain sentence with fields.
- **Private mode with UDP trackers configured.** Today the tracker logs a `WARN` (`Could not start UDP trackers while in private mode. UDP is not safe for private trackers!`) and runs without the configured UDP trackers.

  | Alternative | Pros | Cons |
  | --- | --- | --- |
  | A. Keep the `WARN` | No break | A configured service silently does not run. By the table that is an `ERROR`, and an `ERROR` on every start of a "working" tracker is not acceptable either |
  | B. Reject the configuration at startup | The configuration either does what it says or the tracker does not start. No line to argue about | Breaks configurations that rely on the silent skip. Acceptable in 4.0.0 |

  **Decision:** B, in its own sub-issue (SI-3) with its own ADR, because it changes behavior, not only a log line.

## D10 - Information an operator cannot see today

Approved additions:

1. A first line with the build identity: `APPLICATION: Starting Torrust Tracker version=… git_commit=… build_profile=… pid=… run_id=…`. A build script embeds the commit.
2. `APPLICATION: Tracker started services=6 jobs=16 elapsed_in_ms=58`, after every job has started.
3. `elapsed_in_ms` on every job outcome line, measured from the moment that job was asked to stop.
4. A final line: `APPLICATION: Tracker stopped outcome=graceful signal=SIGTERM jobs_completed=16 jobs_failed=0 jobs_aborted=0 elapsed_in_ms=312 exit_code=0`. On failure it is an `ERROR` with `outcome=incomplete` and the names of the jobs that failed or were aborted.

The numbers in the examples are illustrations.

### Identity of a run

The word **run** means one execution of the tracker process, from start to exit. The word **instance** is not used for this, because in the tracker an instance is one configured service (for example the second HTTP tracker, `instance_index=1`).

The need: when logs of several runs are in one place (a restart loop, two containers, a log collector), tell which lines belong to which run.

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Nothing | No work | After a restart, old and new lines can only be told apart by time |
| B. `pid` on the start line | Free. Matches what the operating system and the process supervisor show | Reused by the operating system. Always `1` in a container, so it does not separate two containers |
| C. `run_id` (a random UUID, like `request_id`) on the start line | Unique across hosts and restarts. The same idea the tracker already uses for requests | On one line only: to find a run's lines the operator still uses the time range after that line |
| D. `run_id` on every record | Any single line can be attributed to its run. One filter gives a whole run | 36 more characters on every text line. As a root span it would print in every span list and is lost in tasks that do not inherit the span, so it belongs in the formatter |
| E. Leave it to the infrastructure (container ID, systemd invocation ID, collector labels) | No tracker code | Each deployment solves it differently or not at all. Native runs and tests have nothing |

**Decision:** B and C now (SI-16), D later (SI-22).

**Reason:** `pid` and `run_id` on the start line cost almost nothing and are useful at once. D is the complete answer, but it must be done in the formatter, and #2766 replaces the formatter; doing D before #2766 would be written twice. In JSON the extra field costs nothing to read. E stays possible and does not replace an identity the tracker itself knows.

## D11 - Where the rules live

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Only in this EPIC's specification | No extra documents | A closed specification is history. Nobody reads it before adding a log line |
| B. An ADR only | Decisions and reasons are kept | An ADR is not a reference; it has no list of lines and fields to look up |
| C. An ADR for the decisions, a reference document for the content, the skill for the workflow, rustdoc at the code, and an automated check | Each kind of knowledge is where its reader looks, and the check keeps the code from drifting | More artifacts to keep aligned. Mitigated by linking instead of copying |

**Decision:** C.

- One new logging ADR in `docs/adrs/` with the decisions and reasons (SI-2).
- A new `docs/logging.md`, linked from `docs/index.md`: glossary, targets, canonical fields, levels, and the catalogue of lifecycle lines (SI-2, extended by each sub-issue that changes a line).
- The skill `structured-runtime-logging` is extended with links, not copies (SI-2).
- Rustdoc on the target constants and on the display helper.
- An automated check (SI-20): a test starts the tracker, stops it, and asserts no `WARN` or `ERROR` on a clean run, no `Debug` patterns such as `{ … }` or `Some(`, and a named target from the list on every lifecycle line.
- The private-mode rejection has its own ADR (SI-3).

## D12 - Compatibility

| Alternative | Pros | Cons |
| --- | --- | --- |
| A. Clean break in 4.0.0 | One vocabulary. No transition code. 4.0.0 is a major release and is close | Operators who parse the old text must update when they upgrade |
| B. A deprecation period: log old and new names together for one release | Parsers keep working for a while | Doubles the lines or fields that this EPIC exists to clean. Needs a later removal, after 4.0.0, which would then be a break inside the 4.x line |
| C. An option to select the old format | No forced change | A second format to test and keep for ever |

**Decision:** A. No deprecation period.

**Reason:** version 4.0.0 is the right moment to break. The cost for operators is paid once, with a document that tells them what changed.

**Changelog:** the project has no general `CHANGELOG` yet; the maintainer plans one after 4.0.0. This EPIC keeps its own list of operator-visible changes: targets renamed, fields renamed or removed, messages changed, lines added or removed, levels changed. It is the section "Changes in 4.0.0" of `docs/logging.md`, created by SI-2. Every sub-issue that changes a line adds its entries in the same pull request. It lives in `docs/logging.md` and not in this folder because this folder moves to `docs/issues/closed/` at the end, where an operator does not look.
